use parse::{get_text_from_step, parse_str, Step, StepKind};

mod tag_info;
use tag_info::{DescendantStatus, TagInfo};

pub mod sieves;
use sieves::SieveImpl;

pub fn compose(sieve: &impl SieveImpl, template_str: &str) -> String {
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
            StepKind::DescendantInjection => {
                push_injection_kind(&mut results, &mut stack, template_str, step)
            }
            StepKind::InjectionSpace => {
                push_injection_kind(&mut results, &mut stack, template_str, step)
            }
            StepKind::InjectionConfirmed => {
                push_injection_kind(&mut results, &mut stack, template_str, step)
            }
            _ => {}
        }
    }

    results
}

fn push_element(
    results: &mut String,
    stack: &mut Vec<TagInfo>,
    sieve: &impl SieveImpl,
    template_str: &str,
    step: Step,
) {
    let tag = get_text_from_step(template_str, &step);
    let mut tag_info = match stack.last_mut() {
        Some(mut prev_tag_info) => TagInfo::from(sieve, &prev_tag_info, tag),
        _ => TagInfo::new(sieve, tag),
    };

    if tag_info.banned_path {
        if let Some(mut prev_tag_info) = stack.last_mut() {
            prev_tag_info.most_recent_descendant = match sieve.inline_el(tag) {
                true => DescendantStatus::InlineElement,
                _ => DescendantStatus::Element,
            };
        }
        stack.push(tag_info);
        return;
    }

    if !sieve.respect_indentation() && tag_info.inline_el && !tag_info.void_el {
        if let Some(prev_tag_info) = stack.last() {
            if prev_tag_info.most_recent_descendant == DescendantStatus::Text {
                results.push(' ');
            }
        }
    }

    if sieve.respect_indentation() && !tag_info.inline_el && results.len() > 0 {
        // edge case that requires reading from the results to prevent starting with \n
        // not my favorite but works here
        results.push('\n');
        results.push_str(&"\t".repeat(tag_info.indent_count));
    }

    if sieve.respect_indentation() && tag_info.inline_el && results.len() > 0 {
        // edge case that requires reading from the results to prevent starting with \n
        // not my favorite but works here
        results.push(' ');
    }

    if let Some(mut prev_tag_info) = stack.last_mut() {
        prev_tag_info.most_recent_descendant = match sieve.inline_el(tag) {
            true => DescendantStatus::InlineElement,
            _ => DescendantStatus::Element,
        };
    }

    results.push('<');
    results.push_str(tag);

    stack.push(tag_info);
}

fn close_element(results: &mut String, stack: &mut Vec<TagInfo>) {
    // cannot close non existant tag
    let tag_info = match stack.last_mut() {
        Some(prev_tag_info) => prev_tag_info,
        _ => return,
    };

    if !tag_info.banned_path {
        results.push_str(">");
    }

    if tag_info.namespace == "html" && tag_info.void_el {
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

    if tag_info.namespace != "html" {
        results.push_str("/>");
    }

    if tag_info.namespace == "html" && !tag_info.void_el {
        results.push_str("></");
        results.push_str(&tag_info.tag);
    }

    // svg and mathml elements can self close
    if tag_info.namespace == "html" {
        results.push('>');
    }

    stack.pop();
}

fn pop_element(
    results: &mut String,
    stack: &mut Vec<TagInfo>,
    sieve: &impl SieveImpl,
    template_str: &str,
    step: Step,
) {
    // need to get second to last element and then say this was a block element or an inline element
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

    if tag_info.namespace == "html" && tag_info.void_el {
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
        && tag_info.most_recent_descendant != DescendantStatus::Initial
    {
        results.push_str("\n");
        results.push_str(&"\t".repeat(tag_info.indent_count));
    }

    results.push_str("</");
    results.push_str(tag);
    results.push('>');

    stack.pop();

    if let Some(mut prev_tag_info) = stack.last_mut() {
        prev_tag_info.most_recent_descendant = match sieve.inline_el(tag) {
            true => DescendantStatus::InlineElementClosed,
            _ => DescendantStatus::ElementClosed,
        };
    }
}

fn get_prev_element(stack: &mut Vec<TagInfo>) {
    let prv_idx = stack.len() - 2;
    match stack.get_mut(prv_idx) {
        Some(el) => {}
        _ => {}
    };
}

fn push_text(
    results: &mut String,
    stack: &mut Vec<TagInfo>,
    sieve: &impl SieveImpl,
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

    // if alternative like styles or scripts
    if sieve.alt_text(&tag_info.tag) {
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

    if sieve.respect_indentation() {
        println!("text sieve indentation {:?}", tag_info);

        match tag_info.most_recent_descendant {
            DescendantStatus::Element => add_element_text(results, texts, tag_info),
            DescendantStatus::ElementClosed => add_element_closed_text(results, texts, tag_info),
            DescendantStatus::InlineElement => {
                add_inline_element_text(results, texts, tag_info);
            }
            DescendantStatus::InlineElementClosed => {
                add_inline_element_closed_text(results, texts, tag_info)
            }
            DescendantStatus::Text => add_text(results, texts, tag_info),
            DescendantStatus::Initial => {
                if tag_info.inline_el {
                    add_inline_element_text(results, texts, tag_info);
                } else {
                    add_element_text(results, texts, tag_info);
                }
            }
        }
    } else {
        match tag_info.most_recent_descendant {
            DescendantStatus::Element => add_inline_element_text(results, texts, tag_info),
            DescendantStatus::ElementClosed => add_inline_element_text(results, texts, tag_info),
            DescendantStatus::InlineElement => add_inline_element_text(results, texts, tag_info),
            DescendantStatus::InlineElementClosed => {
                add_unpretty_inline_element_closed_text(results, texts, tag_info)
            }
            DescendantStatus::Text => add_inline_element_closed_text(results, texts, tag_info),
            DescendantStatus::Initial => add_inline_element_text(results, texts, tag_info),
        }
    }

    tag_info.most_recent_descendant = DescendantStatus::Text;
}

fn add_element_text(results: &mut String, texts: Vec<&str>, tag_info: &TagInfo) {
    for line in texts {
        results.push('\n');
        results.push_str(&"\t".repeat(&tag_info.indent_count + 1));
        results.push_str(line);
    }
}

fn add_element_closed_text(results: &mut String, texts: Vec<&str>, tag_info: &TagInfo) {
    for line in texts {
        results.push('\n');
        results.push_str(&"\t".repeat(tag_info.indent_count));
        results.push_str(line);
    }
}

fn add_inline_element_text(results: &mut String, texts: Vec<&str>, tag_info: &TagInfo) {
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

fn add_unpretty_inline_element_closed_text(
    results: &mut String,
    texts: Vec<&str>,
    tag_info: &TagInfo,
) {
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
        results.push_str(&"\t".repeat(&tag_info.indent_count + 1));
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

fn get_most_common_space_index(text: &str) -> usize {
    let mut space_index = 0;

    let mut prev_space;
    let mut curr_space = "";

    for line in text.split("\n") {
        prev_space = curr_space;

        let curr_index = get_index_of_first_char(line);
        if curr_index == line.len() {
            continue;
        }

        curr_space = line;
        if space_index == curr_index {
            continue;
        }

        space_index = get_most_common_space_index_between_two_strings(prev_space, curr_space);
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
