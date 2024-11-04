use parse::{get_text_from_step, parse_str, Step, StepKind};

mod tag_info;
use sieve::SieveImpl;
use tag_info::{DescendantStatus, TagInfo};

pub fn compose(sieve: &dyn SieveImpl, template_str: &str) -> String {
    let mut results = "".to_string();
    let mut stack: Vec<TagInfo> = Vec::new();

    for step in parse_str(sieve, &template_str, StepKind::Initial) {
        match step.kind {
            StepKind::Tag => push_element(&mut results, &mut stack, sieve, template_str, step),
            StepKind::ElementClosed => close_element(&mut results, &mut stack),
            StepKind::EmptyElementClosed => close_empty_element(&mut results, &mut stack),
            StepKind::TailTag => pop_element(&mut results, &mut stack, sieve, template_str, step),
            StepKind::Text => push_text(&mut results, &mut stack, sieve, template_str, step),
            StepKind::Attr => add_attr(&mut results, &mut stack, template_str, step),
            StepKind::AttrValue => add_attr_value(&mut results, &mut stack, template_str, step),
            StepKind::AttrValueUnquoted => {
                add_attr_value_unquoted(&mut results, &mut stack, template_str, step)
            }
            // injections
            StepKind::DescendantInjection => {
                push_injection_kind(&mut results, &mut stack, template_str, step)
            }
            StepKind::InjectionSpace => {
                push_injection_kind(&mut results, &mut stack, template_str, step)
            }
            StepKind::InjectionConfirmed => {
                push_injection_kind(&mut results, &mut stack, template_str, step)
            }
            // alt text
            StepKind::CommentText => push_text(&mut results, &mut stack, sieve, template_str, step),
            StepKind::AltText => push_text(&mut results, &mut stack, sieve, template_str, step),
            StepKind::AltTextCloseSequence => {
                pop_closing_sequence(&mut results, &mut stack, sieve, template_str, step)
            }
            _ => {}
        }
    }

    results
}

fn push_element(
    results: &mut String,
    stack: &mut Vec<TagInfo>,
    sieve: &dyn SieveImpl,
    template_str: &str,
    step: Step,
) {
    let tag = get_text_from_step(template_str, &step);
    let tag_info = match stack.last_mut() {
        Some(prev_tag_info) => TagInfo::from(sieve, &prev_tag_info, tag),
        _ => TagInfo::new(sieve, tag),
    };

    // banned path
    if let (true, Some(prev_tag_info)) = (tag_info.banned_path, stack.last_mut()) {
        prev_tag_info.most_recent_descendant = match sieve.tag_is_inline_el(tag) {
            true => DescendantStatus::InlineElement,
            _ => DescendantStatus::Element,
        };
        stack.push(tag_info);
        return;
    }

    // edge case for start of document
    if sieve.respect_indentation() && results.len() > 0 && tag_info.inline_el {
        // edge case that requires reading from the results to prevent starting with \n
        // not my favorite but works here
        results.push(' ');
    }

    if !sieve.respect_indentation() && tag_info.inline_el && !tag_info.void_el {
        if let Some(prev_tag_info) = stack.last() {
            if prev_tag_info.most_recent_descendant == DescendantStatus::Text {
                results.push(' ');
            }
        }
    }

    if sieve.respect_indentation() && results.len() > 0 && !tag_info.inline_el {
        // edge case that requires reading from the results to prevent starting with \n
        // not my favorite but works here
        results.push('\n');
        results.push_str(&"\t".repeat(tag_info.indent_count));
    }

    if let Some(prev_tag_info) = stack.last_mut() {
        prev_tag_info.most_recent_descendant = match sieve.tag_is_inline_el(tag) {
            true => DescendantStatus::InlineElement,
            _ => DescendantStatus::Element,
        };
    }

    results.push('<');
    results.push_str(tag);

    stack.push(tag_info);
}

fn close_element(results: &mut String, stack: &mut Vec<TagInfo>) {
    let tag_info = match stack.last_mut() {
        Some(prev_tag_info) => prev_tag_info,
        _ => return,
    };

    if !tag_info.banned_path {
        results.push_str(">");
    }

    if tag_info.void_el && "html" == tag_info.namespace {
        stack.pop();
    }
}

fn close_empty_element(results: &mut String, stack: &mut Vec<TagInfo>) {
    let tag_info = match stack.last() {
        Some(curr) => curr,
        _ => return,
    };

    if tag_info.banned_path || tag_info.void_el {
        stack.pop();
        return;
    }

    if "html" != tag_info.namespace {
        results.push_str("/>");
    }

    if !tag_info.void_el && "html" == tag_info.namespace {
        results.push_str("></");
        results.push_str(&tag_info.tag);
    }

    if "html" == tag_info.namespace {
        results.push('>');
    }

    stack.pop();
}

fn pop_element(
    results: &mut String,
    stack: &mut Vec<TagInfo>,
    sieve: &dyn SieveImpl,
    template_str: &str,
    step: Step,
) {
    let tag = get_text_from_step(template_str, &step);

    let tag_info = match stack.last() {
        Some(curr) => curr,
        _ => return,
    };

    if tag != tag_info.tag {
        return;
    }

    if tag_info.banned_path {
        stack.pop();
        return;
    }

    if tag_info.void_el && "html" == tag_info.namespace {
        results.push('>');
        stack.pop();
        if let Some(prev_tag_info) = stack.last_mut() {
            prev_tag_info.most_recent_descendant = DescendantStatus::ElementClosed;
        }
        return;
    }

    if sieve.respect_indentation()
        && !tag_info.inline_el
        && !tag_info.preserved_text_path
        && DescendantStatus::Initial != tag_info.most_recent_descendant
    {
        results.push_str("\n");
        results.push_str(&"\t".repeat(tag_info.indent_count));
    }

    results.push_str("</");
    results.push_str(tag);
    results.push('>');

    stack.pop();

    if let Some(prev_tag_info) = stack.last_mut() {
        prev_tag_info.most_recent_descendant = match sieve.tag_is_inline_el(tag) {
            true => DescendantStatus::InlineElementClosed,
            _ => DescendantStatus::ElementClosed,
        };
    }
}

fn push_text(
    results: &mut String,
    stack: &mut Vec<TagInfo>,
    sieve: &dyn SieveImpl,
    template_str: &str,
    step: Step,
) {
    let text = get_text_from_step(template_str, &step);
    let tag_info = match stack.last_mut() {
        Some(curr) => curr,
        // text is first node
        _ => {
            for line in text.trim().split("\n") {
                let trimmed = line.trim();
                if trimmed.len() == 0 {
                    continue;
                }

                results.push('\n');
                results.push_str(trimmed);
            }
            return;
        }
    };

    if tag_info.banned_path || tag_info.void_el {
        return;
    }

    if tag_info.preserved_text_path {
        tag_info.most_recent_descendant = DescendantStatus::Text;
        results.push_str(text);
        return;
    }

    // if alt text
    if let Some(_) = sieve.get_close_sequence_from_alt_text_tag(&tag_info.tag) {
        let common_index = get_most_common_space_index(text);

        for line in text.split("\n") {
            if line.len() == get_index_of_first_char(line) {
                continue;
            }

            results.push('\n');
            results.push_str(&"\t".repeat(tag_info.indent_count + 1));
            results.push_str(line[common_index..].trim_end());
        }

        tag_info.most_recent_descendant = DescendantStatus::Text;
        return;
    }

    // move this to add_text functions
    let mut texts: Vec<&str> = Vec::new();
    for line in text.split("\n") {
        let trimmed = line.trim();
        if trimmed.len() == 0 {
            continue;
        }

        texts.push(trimmed);
    }

    if texts.len() == 0 {
        return;
    }

    match (
        sieve.respect_indentation(),
        &tag_info.most_recent_descendant,
    ) {
        (true, DescendantStatus::InlineElement) => {
            add_inline_element_text(results, texts);
        }
        (true, DescendantStatus::InlineElementClosed) => {
            add_inline_element_closed_text(results, texts, tag_info)
        }
        (true, DescendantStatus::Initial) => match tag_info.inline_el {
            true => add_inline_element_text(results, texts),
            _ => add_text(results, texts, tag_info),
        },
        (true, _) => add_text(results, texts, tag_info),
        (false, DescendantStatus::InlineElementClosed) => {
            add_unpretty_inline_element_closed_text(results, texts)
        }
        (false, DescendantStatus::Text) => add_inline_element_closed_text(results, texts, tag_info),
        (false, _) => add_inline_element_text(results, texts),
    }

    tag_info.most_recent_descendant = DescendantStatus::Text;
}

fn add_inline_element_text(results: &mut String, texts: Vec<&str>) {
    let mut text_itr = texts.iter();

    if let Some(line) = text_itr.next() {
        results.push_str(line);
    }

    while let Some(line) = text_itr.next() {
        results.push(' ');
        results.push_str(line);
    }
}

fn add_inline_element_closed_text(results: &mut String, texts: Vec<&str>, tag_info: &TagInfo) {
    let mut text_itr = texts.iter();

    if let Some(line) = text_itr.next() {
        results.push(' ');
        results.push_str(line);
    }

    while let Some(line) = text_itr.next() {
        results.push('\n');
        results.push_str(&"\t".repeat(tag_info.indent_count + 1));
        results.push_str(line);
    }
}

fn add_unpretty_inline_element_closed_text(results: &mut String, texts: Vec<&str>) {
    let mut text_itr = texts.iter();

    if let Some(line) = text_itr.next() {
        results.push(' ');
        results.push_str(line);
    }

    while let Some(line) = text_itr.next() {
        results.push(' ');
        results.push_str(line);
    }
}

fn add_text(results: &mut String, texts: Vec<&str>, tag_info: &TagInfo) {
    for line in texts {
        results.push('\n');
        results.push_str(&"\t".repeat(tag_info.indent_count + 1));
        results.push_str(line);
    }
}

fn add_attr(results: &mut String, stack: &mut Vec<TagInfo>, template_str: &str, step: Step) {
    let tag_info = match stack.last() {
        Some(curr) => curr,
        _ => return,
    };

    if tag_info.banned_path {
        return;
    }

    let attr = get_text_from_step(template_str, &step);
    results.push(' ');
    results.push_str(attr);
}

fn add_attr_value(results: &mut String, stack: &mut Vec<TagInfo>, template_str: &str, step: Step) {
    let tag_info = match stack.last() {
        Some(curr) => curr,
        _ => return,
    };

    if tag_info.banned_path {
        return;
    }

    let val = get_text_from_step(template_str, &step);
    results.push_str("=\"");
    results.push_str(val);
    results.push('"');
}

fn add_attr_value_unquoted(
    results: &mut String,
    stack: &mut Vec<TagInfo>,
    template_str: &str,
    step: Step,
) {
    let tag_info = match stack.last() {
        Some(curr) => curr,
        _ => return,
    };

    if tag_info.banned_path {
        return;
    }

    let val = get_text_from_step(template_str, &step);
    results.push('=');
    results.push_str(val);
}

fn push_injection_kind(
    results: &mut String,
    stack: &mut Vec<TagInfo>,
    template_str: &str,
    step: Step,
) {
    let tag_info = match stack.last() {
        Some(curr) => curr,
        _ => return,
    };

    if tag_info.banned_path {
        return;
    }

    let glyph = get_text_from_step(template_str, &step);
    results.push_str(glyph);
}

fn pop_closing_sequence(
    results: &mut String,
    stack: &mut Vec<TagInfo>,
    sieve: &dyn SieveImpl,
    template_str: &str,
    step: Step,
) {
    // need to get second to last element and then say this was a block element or an inline element
    let closing_sequence = get_text_from_step(template_str, &step);

    let tag = match sieve.get_tag_from_close_sequence(closing_sequence) {
        Some(t) => t,
        _ => return,
    };

    let tag_info = match stack.last() {
        Some(curr) => curr,
        _ => return,
    };

    if tag != tag_info.tag {
        return;
    }

    if tag_info.banned_path {
        stack.pop();
        return;
    }

    if sieve.respect_indentation()
        && !tag_info.inline_el
        && !tag_info.preserved_text_path
        && DescendantStatus::Initial != tag_info.most_recent_descendant
    {
        results.push_str("\n");
        results.push_str(&"\t".repeat(tag_info.indent_count));
    }

    results.push_str(closing_sequence);

    stack.pop();
}

fn get_most_common_space_index(text: &str) -> usize {
    let mut prev_space_index = text.len();
    let mut space_index = text.len();
    let mut prev_line = "";

    let mut texts = text.split("\n");

    if let Some(line) = texts.next() {
        prev_line = line;
    }

    while let Some(line) = texts.next() {
        let first_char = get_index_of_first_char(line);
        if line.len() == first_char {
            continue;
        }

        space_index = get_most_common_space_index_between_two_strings(prev_line, line);
        if space_index < prev_space_index {
            prev_space_index = space_index
        }

        prev_line = line;
    }

    space_index
}

fn get_index_of_first_char(text: &str) -> usize {
    for (index, glyph) in text.char_indices() {
        if !glyph.is_whitespace() {
            return index;
        }
    }

    text.len()
}

fn get_most_common_space_index_between_two_strings(source: &str, target: &str) -> usize {
    let mut source_chars = source.char_indices();
    let mut target_chars = target.char_indices();

    let mut prev_index = 0;
    while let (Some((src_index, src_chr)), Some((_, tgt_chr))) =
        (source_chars.next(), target_chars.next())
    {
        if !src_chr.is_whitespace() || !tgt_chr.is_whitespace() || src_chr != tgt_chr {
            return src_index;
        }
        prev_index = src_index;
    }

    prev_index
}
