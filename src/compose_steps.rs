use crate::parse::{get_text_from_step, Step};
use crate::routes::StepKind;
use crate::rulesets::RulesetImpl;
use crate::tag_info::{TagInfo, TextFormat};

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
            StepKind::Text => push_text(results, tag_info_stack, rules, template_str, step),
            StepKind::Attr => push_attr(results, tag_info_stack, template_str, step),
            StepKind::AttrValue => push_attr_value(results, tag_info_stack, template_str, step),
            StepKind::AttrValueUnquoted => {
                push_attr_value_unquoted(results, tag_info_stack, template_str, step)
            }
            _ => {}
        }
    }
}

fn push_text(
    results: &mut String,
    stack: &mut Vec<TagInfo>,
    rules: &dyn RulesetImpl,
    template_str: &str,
    step: &Step,
) {
    let text = get_text_from_step(template_str, step);
    push_text_component(results, stack, rules, text)
}

pub fn push_text_component(
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
        // this should never happen
        _ => return,
    };

    if tag_info.banned_path || tag_info.void_el {
        return;
    }

    if tag_info.preserved_text_path {
        results.push_str(text);
        tag_info.text_format = TextFormat::Inline;
        return;
    }

    // if alt text
    if let Some(_) = rules.get_close_sequence_from_alt_text_tag(&tag_info.tag) {
        add_alt_element_text(results, text, tag_info);
        tag_info.text_format = TextFormat::Inline;
        return;
    }

    // if unformatted
    if !rules.respect_indentation() {
        add_inline_text(results, text, &tag_info);
        tag_info.text_format = TextFormat::Inline;
        return;
    }

    // formatted text
    if TextFormat::Inline == tag_info.text_format {
        results.push(' ');
    }

    if tag_info.inline_el || TextFormat::Inline == tag_info.text_format {
        add_first_line_text(results, text, tag_info);
    } else {
        add_text(results, text, tag_info);
    }

    tag_info.text_format = TextFormat::Inline;
}

fn push_element(
    results: &mut String,
    stack: &mut Vec<TagInfo>,
    rules: &dyn RulesetImpl,
    template_str: &str,
    step: &Step,
) {
    let prev_tag_info = match stack.last_mut() {
        Some(pti) => pti,
        _ => {
            // this never happens
            return;
        }
    };

    let tag = get_text_from_step(template_str, step);
    let tag_info = TagInfo::from(rules, prev_tag_info, tag);

    // banned path
    if tag_info.banned_path {
        stack.push(tag_info);
        return;
    }

    if !rules.respect_indentation()
        && TextFormat::Initial != prev_tag_info.text_format
        && TextFormat::Root != prev_tag_info.text_format
    {
        results.push(' ');
    }

    if rules.respect_indentation() {
        if !tag_info.inline_el {
            if TextFormat::Root != prev_tag_info.text_format {
                results.push('\n');
                results.push_str(&"\t".repeat(prev_tag_info.indent_count));
            }
            prev_tag_info.text_format = TextFormat::Block;
        }

        if tag_info.inline_el {
            if TextFormat::Inline == prev_tag_info.text_format {
                results.push(' ');
            }
            prev_tag_info.text_format = TextFormat::Inline;
        }
    }

    results.push('<');
    results.push_str(tag);

    stack.push(tag_info);
}

fn close_element(results: &mut String, stack: &mut Vec<TagInfo>) {
    let tag_info = match stack.last() {
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
    let tag_info = match stack.pop() {
        Some(curr) => curr,
        _ => return,
    };

    if tag_info.banned_path {
        return;
    }

    if "html" != tag_info.namespace {
        results.push_str("/>");
    } else {
        if !tag_info.void_el {
            results.push_str("></");
            results.push_str(&tag_info.tag);
        }

        results.push('>');
    }
}

fn pop_element(
    results: &mut String,
    stack: &mut Vec<TagInfo>,
    rules: &dyn RulesetImpl,
    template_str: &str,
    step: &Step,
) {
    let tag_info = match stack.pop() {
        Some(ti) => ti,
        _ => {
            // never happens
            return;
        }
    };

    if tag_info.banned_path {
        return;
    }

    let mut tag = get_text_from_step(template_str, step);
    if let Some(close_tag) = rules.get_alt_text_tag_from_close_sequence(tag) {
        tag = close_tag;
    }

    if tag != tag_info.tag {
        return;
    }

    if tag_info.void_el && "html" == tag_info.namespace {
        results.push('>');
        return;
    }

    let prev_tag_info = match stack.last() {
        Some(curr) => curr,
        _ => return,
    };

    if rules.respect_indentation()
        && !tag_info.inline_el
        && !tag_info.preserved_text_path
        && TextFormat::Initial != tag_info.text_format
    {
        results.push('\n');
        results.push_str(&"\t".repeat(prev_tag_info.indent_count));
    }

    if let Some(close_seq) = rules.get_close_sequence_from_alt_text_tag(tag) {
        results.push_str(close_seq);
        results.push('>');
        return;
    }

    results.push_str("</");
    results.push_str(tag);
    results.push('>');
}

fn all_spaces(line: &str) -> bool {
    line.len() == get_index_of_first_char(line)
}

fn add_alt_element_text(results: &mut String, text: &str, tag_info: &TagInfo) {
    let common_index = get_most_common_space_index(text);

    for line in text.split("\n") {
        if all_spaces(line) {
            continue;
        }

        results.push('\n');
        results.push_str(&"\t".repeat(tag_info.indent_count));
        results.push_str(line[common_index..].trim_end());
    }
}

fn add_first_line_text(results: &mut String, text: &str, tag_info: &TagInfo) {
    let mut text_iter = text.split("\n");

    while let Some(line) = text_iter.next() {
        if !all_spaces(line) {
            results.push_str(line.trim());
            break;
        }
    }

    while let Some(line) = text_iter.next() {
        if all_spaces(line) {
            continue;
        }

        results.push('\n');
        results.push_str(&"\t".repeat(tag_info.indent_count));
        results.push_str(line.trim());
    }
}

fn add_inline_text(results: &mut String, text: &str, tag_info: &TagInfo) {
    let mut text_iter = text.split("\n");

    while let Some(line) = text_iter.next() {
        if all_spaces(line) {
            continue;
        }

        if TextFormat::Root != tag_info.text_format && TextFormat::Initial != tag_info.text_format {
            results.push(' ');
        }

        results.push_str(line.trim());
        break;
    }

    while let Some(line) = text_iter.next() {
        if !all_spaces(line) {
            results.push(' ');
            results.push_str(line.trim());
        }
    }
}

fn add_text(results: &mut String, text: &str, tag_info: &TagInfo) {
    let mut text_iter = text.split("\n");

    while let Some(line) = text_iter.next() {
        if all_spaces(line) {
            continue;
        }

        if TextFormat::Root != tag_info.text_format {
            results.push('\n');
        }

        results.push_str(&"\t".repeat(tag_info.indent_count));
        results.push_str(line.trim());
        break;
    }

    while let Some(line) = text_iter.next() {
        if all_spaces(line) {
            continue;
        }

        results.push('\n');
        results.push_str(&"\t".repeat(tag_info.indent_count));
        results.push_str(line.trim());
    }
}

fn push_attr(results: &mut String, stack: &mut Vec<TagInfo>, template_str: &str, step: &Step) {
    let attr = get_text_from_step(template_str, step);
    push_attr_component(results, stack, attr)
}

pub fn push_attr_component(results: &mut String, stack: &mut Vec<TagInfo>, attr: &str) {
    let tag_info = match stack.last() {
        Some(curr) => curr,
        _ => return,
    };

    if tag_info.banned_path {
        return;
    }

    results.push(' ');
    results.push_str(attr.trim());
}

fn push_attr_value(
    results: &mut String,
    stack: &mut Vec<TagInfo>,
    template_str: &str,
    step: &Step,
) {
    let val = get_text_from_step(template_str, step);
    push_attr_value_component(results, stack, val)
}

pub fn push_attr_value_component(results: &mut String, stack: &mut Vec<TagInfo>, val: &str) {
    let tag_info = match stack.last() {
        Some(curr) => curr,
        _ => return,
    };

    if tag_info.banned_path {
        return;
    }

    results.push_str("=\"");
    results.push_str(val.trim());
    results.push('"');
}

fn push_attr_value_unquoted(
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

fn get_index_of_first_char(text: &str) -> usize {
    for (index, glyph) in text.char_indices() {
        if !glyph.is_whitespace() {
            return index;
        }
    }

    text.len()
}

fn get_most_common_space_index(text: &str) -> usize {
    let mut space_index = text.len();
    let mut prev_line = "";

    let mut texts = text.split("\n");

    while let Some(line) = texts.next() {
        if all_spaces(line) {
            continue;
        };

        space_index = get_index_of_first_char(line);
        prev_line = line;
        break;
    }

    while let Some(line) = texts.next() {
        if all_spaces(line) {
            continue;
        }

        let curr_index = get_most_common_space_index_between_two_strings(prev_line, line);
        if curr_index < space_index {
            space_index = curr_index
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
        if src_chr == tgt_chr && src_chr.is_whitespace() {
            prev_index = src_index;
            continue;
        }

        return src_index;
    }

    prev_index
}
