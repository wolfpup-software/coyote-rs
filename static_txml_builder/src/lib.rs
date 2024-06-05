use parsley::{get_text_from_step, parse_str, Step, StepKind};
use txml::Template;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct TxmlBuilderResults {
    pub strs: Vec<String>,
    pub injs: Vec<StepKind>,
}

impl TxmlBuilderResults {
    pub fn new() -> TxmlBuilderResults {
        TxmlBuilderResults {
            strs: Vec::from(["".to_string()]),
            injs: Vec::new(),
        }
    }
}

pub struct TxmlBuilder {}

impl TxmlBuilder {
    pub fn new() -> TxmlBuilder {
        TxmlBuilder {}
    }

    pub fn build(&self, template: &Template) -> TxmlBuilderResults {
        // check for already built results
        let mut results = TxmlBuilderResults::new();

        for step in parse_str(&template.template_str, StepKind::Initial) {
            push_step(&mut results, &template.template_str, step);
        }

        results
    }
}

fn push_step(results: &mut TxmlBuilderResults, template_str: &str, step: Step) {
    match step.kind {
        // steps
        StepKind::Tag => push_element(results, template_str, step),
        StepKind::Attr => add_attr(results, template_str, step),
        StepKind::AttrValueUnquoted => add_attr_value_unquoted(results, template_str, step),
        StepKind::AttrValue => add_attr_value(results, template_str, step),
        StepKind::ElementClosed => close_element(results),
        StepKind::EmptyElementClosed => empty_void_element(results),
        StepKind::TailTag => pop_element(results, template_str, step),
        StepKind::Text => push_text(results, template_str, step),
        // injections
        StepKind::AttrMapInjection => push_attr_map_injection(results),
        StepKind::DescendantInjection => push_descendant_injection(results),
        _ => {}
    }
}

// template strs
fn push_element(results: &mut TxmlBuilderResults, template_str: &str, step: Step) {
    let tag = get_text_from_step(template_str, &step);

    if let Some(last) = results.strs.last_mut() {
        last.push('<');
        last.push_str(tag);
    }
}

fn add_attr(results: &mut TxmlBuilderResults, template_str: &str, step: Step) {
    let attr = get_text_from_step(template_str, &step);

    if let Some(last) = results.strs.last_mut() {
        last.push(' ');
        last.push_str(attr);
    }
}

fn add_attr_value_unquoted(results: &mut TxmlBuilderResults, template_str: &str, step: Step) {
    let attr_val = get_text_from_step(template_str, &step);

    if let Some(last) = results.strs.last_mut() {
        last.push('=');
        last.push_str(attr_val);
    }
}

fn add_attr_value(results: &mut TxmlBuilderResults, template_str: &str, step: Step) {
    let attr_val = get_text_from_step(template_str, &step);

    if let Some(last) = results.strs.last_mut() {
        last.push_str("=\"");
        last.push_str(attr_val);
        last.push('"');
    }
}

fn close_element(results: &mut TxmlBuilderResults) {
    if let Some(last) = results.strs.last_mut() {
        last.push_str(">");
    }
}

fn empty_void_element(results: &mut TxmlBuilderResults) {
    if let Some(last) = results.strs.last_mut() {
        last.push_str("/>");
    }
}

fn pop_element(results: &mut TxmlBuilderResults, template_str: &str, step: Step) {
    let tag = get_text_from_step(template_str, &step);
    if let Some(last) = results.strs.last_mut() {
        last.push_str("</");
        last.push_str(tag);
        last.push_str(">");
    }
}

fn push_text(results: &mut TxmlBuilderResults, template_str: &str, step: Step) {
    let text = get_text_from_step(template_str, &step);
    if let Some(last) = results.strs.last_mut() {
        last.push_str(text);
    }
}

// injections
fn push_attr_map_injection(results: &mut TxmlBuilderResults) {
    results.strs.push("".to_string());
    results.injs.push(StepKind::AttrMapInjection);
}

fn push_descendant_injection(results: &mut TxmlBuilderResults) {
    results.strs.push("".to_string());
    results.injs.push(StepKind::DescendantInjection);
}
