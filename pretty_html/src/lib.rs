use parsley::{get_text_from_step, parse_str_with_reserved_tags, ParsleySieve, Step, StepKind};
use txml::Template;

mod tag_info;
use tag_info::TagInfo;

mod sieves;
use sieves::Sieve;

// struct PrettyHtmlBuilderResults
//  results
//  stack
//  sieve

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
        for step in parse_str_with_reserved_tags(sieve, &html_str, StepKind::Initial) {
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
        // steps
        StepKind::Tag => push_element(results, template_str, step),
        StepKind::ElementClosed => close_element(results, template_str, step),
        StepKind::EmptyElementClosed => close_empty_element(results, template_str, step),
        StepKind::Text => push_text(results, template_str, step),
        StepKind::TailTag => pop_element(results, template_str, step),

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

fn push_element(results: &mut String, template_str: &str, step: Step) {
    let tag = get_text_from_step(template_str, &step);

    results.push_str("<");
    results.push_str(tag);
}

fn close_element(results: &mut String, template_str: &str, step: Step) {
    results.push_str(">");
}

fn close_empty_element(results: &mut String, template_str: &str, step: Step) {
    // if void
    results.push_str(">");
    // if not void and html namespace do </tag> closing
}

fn pop_element(results: &mut String, template_str: &str, step: Step) {
    // if void element, don't add
    //
    // if tag does not match current el?
    let tag = get_text_from_step(template_str, &step);

    results.push_str("</");
    results.push_str(tag);
    results.push_str(">");
}

fn push_text(results: &mut String, template_str: &str, step: Step) {
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

// // injections
fn push_injection_kind(results: &mut String, template_str: &str, step: Step) {
    let glyph = get_text_from_step(template_str, &step);
    results.push_str(glyph);
}
