use parsley::{get_text_from_step, parse_str, Step, StepKind};
use txml::Template;

mod tag_info;
use tag_info::TagInfo;

pub mod sieves;
use sieves::Sieve;

pub fn compose(sieve: &impl Sieve, template_str: &str) -> String {
    let mut results = "".to_string();
    let mut stack: Vec<TagInfo> = Vec::new();

    for step in parse_str(sieve, &template_str, StepKind::Initial) {
        match step.kind {
            StepKind::Tag => push_element(&mut results, &mut stack, sieve, template_str, step),
            StepKind::ElementClosed => {
                close_element(&mut results, &mut stack, sieve, template_str, step)
            }
            StepKind::EmptyElementClosed => {
                close_empty_element(&mut results, &mut stack, sieve, template_str, step)
            }
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
    sieve: &impl Sieve,
    template_str: &str,
    step: Step,
) {
    let tag = get_text_from_step(template_str, &step);
    let tag_info = match stack.last() {
        Some(prev_tag_info) => TagInfo::from(sieve, prev_tag_info, tag),
        _ => TagInfo::new(sieve, tag),
    };

    if tag_info.banned_path || tag_info.void_path {
        stack.push(tag_info);
        return;
    }

    if sieve.respect_indentation() && !tag_info.preserved_text_path {
        // EDGE CASE, no \n at start of document
        if stack.len() > 0 {
            results.push('\n');
        }
        results.push_str(&"\t".repeat(tag_info.indent_count));
    }

    results.push('<');
    results.push_str(tag);

    stack.push(tag_info);
}

fn close_element(
    results: &mut String,
    stack: &mut Vec<TagInfo>,
    sieve: &impl Sieve,
    template_str: &str,
    step: Step,
) {
    // cannot close non existant tag
    let stack_len = stack.len();
    let tag_info = match stack.last_mut() {
        Some(prev_tag_info) => prev_tag_info,
        _ => return,
    };

    if !(tag_info.banned_path || tag_info.void_path) {
        results.push_str(">");
    }

    if tag_info.namespace == "html" && tag_info.void_el {
        // EDGE CASE, void elements at start of document
        if sieve.respect_indentation() && stack_len < 2 {
            results.push_str("\n");
        }
        stack.pop();
    }
}

fn close_empty_element(
    results: &mut String,
    stack: &mut Vec<TagInfo>,
    sieve: &impl Sieve,
    template_str: &str,
    step: Step,
) {
    let tag_info = match stack.last() {
        Some(curr) => curr,
        _ => return,
    };

    if tag_info.banned_path || tag_info.void_path {
        stack.pop();
        return;
    }

    if tag_info.namespace != "html" {
        results.push_str("/>");
    } else {
        if !tag_info.void_el {
            results.push_str("></");
            results.push_str(&tag_info.tag);
        }
        results.push('>');
    }

    stack.pop();
}

fn pop_element(
    results: &mut String,
    stack: &mut Vec<TagInfo>,
    sieve: &impl Sieve,
    template_str: &str,
    step: Step,
) {
    let tag_info = match stack.last() {
        Some(curr) => curr,
        _ => return,
    };

    let tag = get_text_from_step(template_str, &step);
    if tag != tag_info.tag {
        return;
    }

    if tag_info.banned_path || tag_info.void_path {
        stack.pop();
        return;
    }

    if tag_info.namespace == "html" && tag_info.void_el {
        results.push('>');
    } else {
        if tag_info.has_text
            && !tag_info.preserved_text_path
            && sieve.respect_indentation()
            && !sieve.preserved_text_el(&tag_info.tag)
        {
            results.push_str("\n");
            results.push_str(&"\t".repeat(tag_info.indent_count));
        }

        results.push_str("</");
        results.push_str(tag);
        results.push('>');
    }

    stack.pop();
}

fn push_text(
    results: &mut String,
    stack: &mut Vec<TagInfo>,
    sieve: &impl Sieve,
    template_str: &str,
    step: Step,
) {
    let text = get_text_from_step(template_str, &step);
    let tag_info = match stack.last_mut() {
        Some(curr) => curr,
        _ => {
            for line in text.trim().split("\n") {
                if line.len() == 0 {
                    continue;
                }
                results.push('\n');
                results.push_str(line.trim());
            }
            return;
        }
    };

    if tag_info.banned_path || tag_info.void_path {
        return;
    }

    tag_info.has_text = true;

    if tag_info.preserved_text_path || sieve.preserved_text_el(&tag_info.tag) {
        results.push_str(text);
        return;
    }

    let trimmed = text.trim();
    for line in trimmed.split("\n") {
        if line.len() == 0 {
            continue;
        }
        results.push('\n');
        results.push_str(&"\t".repeat(tag_info.indent_count + 1));
        results.push_str(line.trim());
    }
}

fn add_attr(results: &mut String, stack: &mut Vec<TagInfo>, template_str: &str, step: Step) {
    let tag_info = match stack.last() {
        Some(curr) => curr,
        _ => return,
    };

    if tag_info.banned_path || tag_info.void_path {
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

    if tag_info.banned_path || tag_info.void_path {
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

    if tag_info.banned_path || tag_info.void_path {
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

    if tag_info.banned_path || tag_info.void_path {
        return;
    }

    let glyph = get_text_from_step(template_str, &step);
    results.push_str(glyph);
}
