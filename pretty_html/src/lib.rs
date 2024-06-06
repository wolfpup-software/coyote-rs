use parsley::{get_text_from_step, parse_str, Step, StepKind};
use txml::Template;

mod tag_info;
use tag_info::{void_el, TagInfo};

mod sieves;
use sieves::Sieve;

struct PrettyHtmlBuilder {}

impl PrettyHtmlBuilder {
    pub fn new() -> PrettyHtmlBuilder {
        PrettyHtmlBuilder {}
    }

    pub fn build(&self, sieve: &impl Sieve, template_str: &str) -> String {
        // check for already built results
        let mut results = "".to_string();
        let mut stack: Vec<TagInfo> = Vec::new();

        for step in parse_str(sieve, &template_str, StepKind::Initial) {
            match step.kind {
                // tags
                StepKind::Tag => push_element(&mut results, &mut stack, sieve, template_str, step),
                StepKind::ElementClosed => {
                    close_element(&mut results, &mut stack, sieve, template_str, step)
                }
                StepKind::EmptyElementClosed => {
                    close_empty_element(&mut results, &mut stack, sieve, template_str, step)
                }
                StepKind::TailTag => {
                    pop_element(&mut results, &mut stack, sieve, template_str, step)
                }
                // text
                StepKind::Text => push_text(&mut results, &mut stack, sieve, template_str, step),
                // attributes
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
                // all other steps silently pass through
                _ => {}
            }
        }

        results
    }
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

    if !(tag_info.banned_path || tag_info.void_path) {
        if sieve.respect_indentation() && !tag_info.preserved_text_path {
            results.push('\n');
            results.push_str(&" ".repeat(tag_info.indent_count))
        }
        results.push('<');
        results.push_str(tag);
    }

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
    let tag_info = match stack.last_mut() {
        Some(prev_tag_info) => prev_tag_info,
        _ => return,
    };

    if !(tag_info.banned_path || tag_info.void_path) {
        results.push_str(">");
    }

    if tag_info.namespace == "html" && void_el(&tag_info.tag) {
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

    if !(tag_info.banned_path || tag_info.void_path) {
        if tag_info.namespace != "html" {
            results.push_str("/>");
        } else {
            if !void_el(&tag_info.tag) {
                results.push_str("></");
                results.push_str(&tag_info.tag);
            }
            results.push('>');
        }
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

    // if tags don't align, skip
    let tag = get_text_from_step(template_str, &step);
    if tag != tag_info.tag {
        return;
    }

    if !(tag_info.banned_path || tag_info.void_path) {
        if tag_info.namespace == "html" && void_el(tag) {
            results.push('>');
        } else {
            if sieve.respect_indentation() && !tag_info.preserved_text_path {
                results.push('\n');
                results.push_str(&" ".repeat(tag_info.indent_count))
            }
            results.push_str("</");
            results.push_str(tag);
            results.push('>')
        }
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

    // add text if no stack
    let tag_info = match stack.last() {
        Some(curr) => curr,
        _ => {
            return {
                for line in text.trim().split("\n") {
                    results.push_str(line.trim());
                    results.push('\n');
                }
            }
        }
    };

    if !(tag_info.banned_path || tag_info.void_path) {
        if !sieve.respect_indentation() || tag_info.preserved_text_path {
            results.push_str(text);
        } else {
            results.push('\n');
            for line in text.trim().split("\n") {
                results.push_str(&" ".repeat(tag_info.indent_count));
                results.push_str(line.trim());
                results.push('\n');
            }
        }
    }
}

fn add_attr(results: &mut String, stack: &mut Vec<TagInfo>, template_str: &str, step: Step) {
    let tag_info = match stack.last() {
        Some(curr) => curr,
        _ => return,
    };

    if !(tag_info.banned_path || tag_info.void_path) {
        let attr = get_text_from_step(template_str, &step);
        results.push(' ');
        results.push_str(attr);
    }
}

fn add_attr_value(results: &mut String, stack: &mut Vec<TagInfo>, template_str: &str, step: Step) {
    let tag_info = match stack.last() {
        Some(curr) => curr,
        _ => return,
    };

    if !(tag_info.banned_path || tag_info.void_path) {
        let val = get_text_from_step(template_str, &step);
        results.push_str("=\"");
        results.push_str(val);
        results.push('"');
    }
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

    if !(tag_info.banned_path || tag_info.void_path) {
        let val = get_text_from_step(template_str, &step);
        results.push('=');
        results.push_str(val);
    }
}

// injections
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

    if !(tag_info.banned_path || tag_info.void_path) {
        let glyph = get_text_from_step(template_str, &step);
        results.push_str(glyph);
    }
}
