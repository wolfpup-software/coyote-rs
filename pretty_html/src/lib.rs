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

    // add decision sieve here:
    //
    // self, html_str, sieve
    pub fn build(&self, sieve: &impl Sieve, html_str: &str) -> String {
        // check for already built results
        let mut results = "".to_string();
        let mut stack: Vec<TagInfo> = Vec::new();

        // parse string with reserved words
        for step in parse_str(sieve, &html_str, StepKind::Initial) {
            push_step(&mut results, &mut stack, sieve, &html_str, step);
        }

        results
    }
}

fn push_step(
    results: &mut String,
    stack: &mut Vec<TagInfo>,
    sieve: &impl Sieve,
    template_str: &str,
    step: Step,
) {
    match step.kind {
        // tags
        StepKind::Tag => push_element(results, stack, sieve, template_str, step),
        StepKind::ElementClosed => close_element(results, stack, sieve, template_str, step),
        StepKind::EmptyElementClosed => {
            close_empty_element(results, stack, sieve, template_str, step)
        }
        StepKind::TailTag => pop_element(results, stack, sieve, template_str, step),
        // text
        StepKind::Text => push_text(results, sieve, template_str, step),
        // attributes
        StepKind::Attr => add_attr(results, template_str, step),
        StepKind::AttrValue => add_attr_value(results, template_str, step),
        StepKind::AttrValueUnquoted => add_attr_value_unquoted(results, template_str, step),
        // injections
        StepKind::DescendantInjection => push_injection_kind(results, template_str, step),
        StepKind::InjectionSpace => push_injection_kind(results, template_str, step),
        StepKind::InjectionConfirmed => push_injection_kind(results, template_str, step),
        // all other steps silently pass through
        _ => {}
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

    if sieve.respect_indentation() {
        // add indendation
    }

    results.push_str("<");
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
    results.push_str(">");

    let tag_info = match stack.last() {
        Some(prev_tag_info) => prev_tag_info,
        _ => return,
    };
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
    let tag = get_text_from_step(template_str, &step);
    let tag_info = match stack.last() {
        Some(curr) => curr,
        _ => return,
    };

    if tag != tag_info.tag {
        return;
    }

    if sieve.respect_indentation() {
        // add indendation
    }

    // mathml and svg have empty elements, html has void
    match tag_info.namespace == "html" {
        r#false => results.push_str("/"),
        _ => {
            if !void_el(&tag_info.tag) {
                results.push_str("></");
                results.push_str(tag);
            }
        }
    }
    results.push_str(">");

    stack.pop();
}

fn pop_element(
    results: &mut String,
    stack: &mut Vec<TagInfo>,
    sieve: &impl Sieve,
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
    if sieve.respect_indentation() {
        // add indendation
    }
    let tag = get_text_from_step(template_str, &step);

    results.push_str("</");
    results.push_str(tag);
    results.push_str(">");
}

fn push_text(results: &mut String, sieve: &impl Sieve, template_str: &str, step: Step) {
    if sieve.respect_indentation() {
        // add indendation
    }

    // respect indentation?
    let text = get_text_from_step(template_str, &step);
    results.push_str(text);
}

fn add_attr(results: &mut String, template_str: &str, step: Step) {
    let attr = get_text_from_step(template_str, &step);
    results.push(' ');
    results.push_str(attr);
}

fn add_attr_value(results: &mut String, template_str: &str, step: Step) {
    let val = get_text_from_step(template_str, &step);
    results.push_str("=\"");
    results.push_str(val);
    results.push('"');
}

fn add_attr_value_unquoted(results: &mut String, template_str: &str, step: Step) {
    let val = get_text_from_step(template_str, &step);
    results.push('=');
    results.push_str(val);
}

// injections
fn push_injection_kind(results: &mut String, template_str: &str, step: Step) {
    let glyph = get_text_from_step(template_str, &step);
    results.push_str(glyph);
}
