use crate::parse::{get_text_from_step, Step};
use crate::routes::StepKind;
use crate::rulesets::RulesetImpl;
use crate::tag_info::{DescendantStatus, TagInfo};

// Text,
// Element,
// ElementClosed,
// InlineElement,
// InlineElementClosed,
// Initial,
//

pub fn compose_steps(
    rules: &dyn RulesetImpl,
    results: &mut String,
    tag_info_stack: &mut Vec<TagInfo>,
    template_str: &str,
    steps: &Vec<Step>,
) {
    for step in steps {
        match step.kind {
            StepKind::Tag => push_element(results, tag_info_stack, rules, template_str, step),
            StepKind::ElementClosed => close_element(results, tag_info_stack),
            StepKind::EmptyElementClosed => close_empty_element(results, tag_info_stack),
            StepKind::TailTag => pop_element(results, tag_info_stack, rules, template_str, step),
            StepKind::Text => {
                push_text_component(results, tag_info_stack, rules, template_str, step)
            }
            StepKind::Attr => add_attr(results, tag_info_stack, template_str, step),
            StepKind::AttrValue => add_attr_value(results, tag_info_stack, template_str, step),
            StepKind::AttrValueUnquoted => {
                add_attr_value_unquoted(results, tag_info_stack, template_str, step)
            }
            StepKind::CommentText => {
                push_text_component(results, tag_info_stack, rules, template_str, step)
            }
            StepKind::AltText => {
                push_text_component(results, tag_info_stack, rules, template_str, step)
            }
            StepKind::AltTextCloseSequence => {
                pop_closing_sequence(results, tag_info_stack, rules, template_str, step)
            }
            _ => {}
        }
    }
}

fn push_text_component(
    results: &mut String,
    stack: &mut Vec<TagInfo>,
    rules: &dyn RulesetImpl,
    template_str: &str,
    step: &Step,
) {
    let text = get_text_from_step(template_str, step);
    push_text(results, stack, rules, text)
}

pub fn push_text(
    results: &mut String,
    stack: &mut Vec<TagInfo>,
    rules: &dyn RulesetImpl,
    text: &str,
) {
    if all_spaces(text) {
        return;
    }

    let tag_info = match stack.last_mut() {
        Some(curr) => curr,
        // text is first node
        _ => {
            for line in text.split("\n") {
                if all_spaces(line) {
                    continue;
                }

                results.push('\n');
                results.push_str(line.trim());
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
    if let Some(_) = rules.get_close_sequence_from_alt_text_tag(&tag_info.tag) {
        let common_index = get_most_common_space_index(text);

        for line in text.split("\n") {
            if all_spaces(line) {
                continue;
            }

            results.push('\n');
            results.push_str(&"\t".repeat(tag_info.indent_count + 1));
            results.push_str(line[common_index..].trim_end());
        }

        tag_info.most_recent_descendant = DescendantStatus::Text;
        return;
    }

    // pass arguments to other functions
    // indents()
    // no_indents()

    // better logic is
    // respect indentation?
    //   most_recent_descendant_status *
    match (
        rules.respect_indentation(),
        &tag_info.most_recent_descendant,
    ) {
        (true, DescendantStatus::InlineElement) => {
            add_inline_element_text(results, text);
        }
        // (true, DescendantStatus::InlineElement) => {
        //     add_inline_element_closed_text(results, text, tag_info)
        // }
        (true, DescendantStatus::Initial) => match tag_info.inline_el {
            true => add_inline_element_text(results, text),
            _ => add_text(results, text, tag_info),
        },
        // (true, _) => add_text(results, text, tag_info),
        (false, DescendantStatus::InlineElement) => {
            add_unpretty_inline_element_closed_text(results, text)
        }
        (false, DescendantStatus::Text) => add_inline_element_closed_text(results, text, tag_info),
        (true, _) => add_text(results, text, tag_info),
        // (false, _) => add_inline_element_text(results, text),
        (_, _) => add_inline_element_text(results, text),
    }

    tag_info.most_recent_descendant = DescendantStatus::Text;
}

fn push_element(
    results: &mut String,
    stack: &mut Vec<TagInfo>,
    rules: &dyn RulesetImpl,
    template_str: &str,
    step: &Step,
) {
    let tag = get_text_from_step(template_str, step);

    match rules.tag_is_inline_el(tag) {
        true => update_most_recent_descendant_status(stack, DescendantStatus::InlineElement),
        _ => update_most_recent_descendant_status(stack, DescendantStatus::Element),
    }

    let tag_info = match stack.last_mut() {
        Some(prev_tag_info) => TagInfo::from(rules, &prev_tag_info, tag),
        _ => TagInfo::new(rules, tag),
    };

    // banned path
    if tag_info.banned_path {
        // if let Some(prev_tag_info) = stack.last_mut() {
        //     prev_tag_info.most_recent_descendant = match rules.tag_is_inline_el(tag) {
        //         true => DescendantStatus::InlineElement,
        //         _ => DescendantStatus::Element,
        //     };
        // };

        stack.push(tag_info);
        return;
    }

    // edge case for start of document
    if rules.respect_indentation() && results.len() > 0 {
        if !tag_info.inline_el && DescendantStatus::Initial != tag_info.most_recent_descendant {
            // true => results.push(' '),
            // _ => {
            println!("start of document!");
            results.push('\n');
            results.push_str(&"\t".repeat(tag_info.indent_count));
            // }
        }
    }

    if let Some(prev_tag_info) = stack.last_mut() {
        if !rules.respect_indentation()
            && prev_tag_info.most_recent_descendant == DescendantStatus::Text
        {
            results.push(' ');
        }
    }

    results.push('<');
    results.push_str(tag);

    stack.push(tag_info);
}

//  I THINK THERES A PROBLEM HERE
//      when i close an element, i should add "element closed " to most recent descendant

// close element keeps on the stack unless inline element
fn close_element(results: &mut String, stack: &mut Vec<TagInfo>) {
    let tag_info = match stack.last_mut() {
        Some(prev_tag_info) => prev_tag_info,
        _ => return,
    };

    if !tag_info.banned_path {
        results.push_str(">");
    }

    let is_inline_el = tag_info.inline_el;
    if tag_info.void_el && "html" == tag_info.namespace {
        stack.pop();
        match is_inline_el {
            true => update_most_recent_descendant_status(stack, DescendantStatus::InlineElement),
            _ => update_most_recent_descendant_status(stack, DescendantStatus::Element),
        }
    }

    // match is_inline_el {
    //     true => update_most_recent_descendant_status(stack, DescendantStatus::InlineElement),
    //     _ => update_most_recent_descendant_status(stack, DescendantStatus::Element),
    // }
}

// most recent descendant
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
        stack.pop();
        return;
    }

    if !tag_info.void_el {
        results.push_str("></");
        results.push_str(&tag_info.tag);
    }

    results.push('>');

    let is_inline_el = tag_info.inline_el;
    stack.pop();

    match is_inline_el {
        true => update_most_recent_descendant_status(stack, DescendantStatus::InlineElementClosed),
        _ => update_most_recent_descendant_status(stack, DescendantStatus::Element),
    }
}

// most recent descendant
fn pop_element(
    results: &mut String,
    stack: &mut Vec<TagInfo>,
    rules: &dyn RulesetImpl,
    template_str: &str,
    step: &Step,
) {
    let tag = get_text_from_step(template_str, step);

    let tag_info = match stack.last() {
        Some(curr) => curr,
        _ => return,
    };

    if tag != tag_info.tag {
        return;
    }

    if tag_info.banned_path {
        stack.pop();

        match rules.tag_is_inline_el(tag) {
            true => {
                update_most_recent_descendant_status(stack, DescendantStatus::InlineElementClosed)
            }
            _ => update_most_recent_descendant_status(stack, DescendantStatus::Element),
        }
        return;
    }

    if tag_info.void_el && "html" == tag_info.namespace {
        results.push('>');
        stack.pop();
        update_most_recent_descendant_status(stack, DescendantStatus::Element);
        return;
    }

    if rules.respect_indentation()
        && !tag_info.inline_el
        && !tag_info.preserved_text_path
        && DescendantStatus::Initial != tag_info.most_recent_descendant
    {
        println!("popping element!@");
        println!("descendant_status: {:?}", tag_info.most_recent_descendant);
        results.push_str("\n");
        results.push_str(&"\t".repeat(tag_info.indent_count));
    }

    results.push_str("</");
    results.push_str(tag);
    results.push('>');

    stack.pop();

    match rules.tag_is_inline_el(tag) {
        true => update_most_recent_descendant_status(stack, DescendantStatus::InlineElementClosed),
        _ => update_most_recent_descendant_status(stack, DescendantStatus::Element),
    }
}

fn all_spaces(line: &str) -> bool {
    line.len() == get_index_of_first_char(line)
}

fn add_inline_element_text(results: &mut String, text: &str) {
    let mut text_iter = text.split("\n");
    let mut found = false;

    while let Some(line) = text_iter.next() {
        if all_spaces(line) {
            continue;
        }

        if found {
            results.push(' ');
        }

        results.push_str(line.trim());
        found = true;
    }
}

fn add_inline_element_closed_text(results: &mut String, text: &str, tag_info: &TagInfo) {
    let mut text_iter = text.split("\n");

    if let Some(line) = text_iter.next() {
        if !all_spaces(line) {
            results.push(' ');
            results.push_str(line.trim());
        }
    }

    while let Some(line) = text_iter.next() {
        if !all_spaces(line) {
            results.push('\n');
            results.push_str(&"\t".repeat(tag_info.indent_count + 1));
            results.push_str(line.trim());
        }
    }
}

fn add_unpretty_inline_element_closed_text(results: &mut String, text: &str) {
    for line in text.split("\n") {
        if !all_spaces(line) {
            results.push(' ');
            results.push_str(line.trim());
        }
    }
}

// result, text, indent count (so i can use others)
fn add_text(results: &mut String, text: &str, tag_info: &TagInfo) {
    for line in text.split("\n") {
        if !all_spaces(line) {
            results.push('\n');
            results.push_str(&"\t".repeat(tag_info.indent_count + 1));
            results.push_str(line.trim());
        }
    }
}

fn add_attr(results: &mut String, stack: &mut Vec<TagInfo>, template_str: &str, step: &Step) {
    let tag_info = match stack.last() {
        Some(curr) => curr,
        _ => return,
    };

    if tag_info.banned_path {
        return;
    }

    let attr = get_text_from_step(template_str, step);
    results.push(' ');
    results.push_str(attr);
}

fn add_attr_value(results: &mut String, stack: &mut Vec<TagInfo>, template_str: &str, step: &Step) {
    let tag_info = match stack.last() {
        Some(curr) => curr,
        _ => return,
    };

    if tag_info.banned_path {
        return;
    }

    let val = get_text_from_step(template_str, step);
    results.push_str("=\"");
    results.push_str(val);
    results.push('"');
}

fn add_attr_value_unquoted(
    results: &mut String,
    stack: &mut Vec<TagInfo>,
    template_str: &str,
    step: &Step,
) {
    let tag_info = match stack.last() {
        Some(curr) => curr,
        _ => return,
    };

    if tag_info.banned_path {
        return;
    }

    let val = get_text_from_step(template_str, step);
    results.push('=');
    results.push_str(val);
}

// set most recent descendant
fn pop_closing_sequence(
    results: &mut String,
    stack: &mut Vec<TagInfo>,
    rules: &dyn RulesetImpl,
    template_str: &str,
    step: &Step,
) {
    // need to get second to last element and then say this was a block element or an inline element
    let closing_sequence = get_text_from_step(template_str, step);

    let tag = match rules.get_alt_text_tag_from_close_sequence(closing_sequence) {
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

    if rules.respect_indentation()
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

fn update_most_recent_descendant_status(
    stack: &mut Vec<TagInfo>,
    descendant_status: DescendantStatus,
) {
    if let Some(prev_tag_info) = stack.last_mut() {
        prev_tag_info.most_recent_descendant = descendant_status;
        return;
    }
    println!("no tag_info in stack!")
}

fn get_index_of_first_char(text: &str) -> usize {
    for (index, glyph) in text.char_indices() {
        if !glyph.is_whitespace() {
            return index;
        }
    }

    text.len()
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

fn get_most_common_space_index_between_two_strings(source: &str, target: &str) -> usize {
    let mut source_chars = source.char_indices();
    let mut target_chars = target.chars();

    let mut prev_index = 0;
    while let (Some((src_index, src_chr)), Some(tgt_chr)) =
        (source_chars.next(), target_chars.next())
    {
        if src_chr != tgt_chr || !src_chr.is_whitespace() || !tgt_chr.is_whitespace() {
            return src_index;
        }
        prev_index = src_index;
    }

    prev_index
}
