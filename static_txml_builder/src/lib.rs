use parsley::{get_text_from_step, parse_str, Step, StepKind};
use txml::Template;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct HtmlBuilderResults {
    pub strs: Vec<String>,
    pub injs: Vec<StepKind>,
}

impl HtmlBuilderResults {
    pub fn new() -> HtmlBuilderResults {
        HtmlBuilderResults {
            strs: Vec::from(["".to_string()]),
            injs: Vec::new(),
        }
    }
}

pub struct HtmlBuilder {}

impl HtmlBuilder {
    pub fn new() -> HtmlBuilder {
        HtmlBuilder {}
    }

    pub fn build(&mut self, template: &Template) -> HtmlBuilderResults {
        let mut results = HtmlBuilderResults::new();

        for step in parse_str(&template.template_str, StepKind::Initial) {
            push_step(&mut results, &template.template_str, step);
        }

        results
    }
}

fn push_step(results: &mut HtmlBuilderResults, template_str: &str, step: Step) {
    match step.kind {
        // steps
        StepKind::Tag => {
            push_element(results, get_text_from_step(template_str, &step));
        }
        StepKind::Attr => {
            add_attr(results, get_text_from_step(template_str, &step));
        }
        StepKind::AttrValueUnquoted => {
            add_attr_value_unquoted(results, get_text_from_step(template_str, &step));
        }
        StepKind::AttrValue => {
            add_attr_value(results, get_text_from_step(template_str, &step));
        }
        StepKind::ElementClosed => {
            close_element(results);
        }
        StepKind::VoidElementClosed => {
            close_void_element(results);
        }
        StepKind::VoidElementClosed => {
            close_void_element(results);
        }
        StepKind::TailTag => {
            pop_element(results, get_text_from_step(template_str, &step));
        }
        StepKind::Text => {
            push_text(results, get_text_from_step(template_str, &step));
        }
        // injections
        StepKind::AttrMapInjection => {
            push_attr_map_injection(results);
        }
        StepKind::DescendantInjection => {
            push_descendant_injection(results);
        }
        _ => {}
    }
}

// template strs
fn push_element(builder: &mut HtmlBuilderResults, tag: &str) {
    if let Some(last) = builder.strs.last_mut() {
        last.push('<');
        last.push_str(tag);
    }
}

fn add_attr(builder: &mut HtmlBuilderResults, attr: &str) {
    if let Some(last) = builder.strs.last_mut() {
        last.push(' ');
        last.push_str(attr);
    }
}

fn add_attr_value_unquoted(builder: &mut HtmlBuilderResults, attr_val: &str) {
    if let Some(last) = builder.strs.last_mut() {
        last.push('=');
        last.push_str(attr_val);
    }
}

fn add_attr_value(builder: &mut HtmlBuilderResults, attr_val: &str) {
    if let Some(last) = builder.strs.last_mut() {
        last.push_str("=\"");
        last.push_str(attr_val);
        last.push('"');
    }
}

fn close_element(builder: &mut HtmlBuilderResults) {
    if let Some(last) = builder.strs.last_mut() {
        last.push_str(">");
    }
}

fn close_void_element(builder: &mut HtmlBuilderResults) {
    if let Some(last) = builder.strs.last_mut() {
        last.push_str("/>");
    }
}

fn pop_element(builder: &mut HtmlBuilderResults, tag: &str) {
    if let Some(last) = builder.strs.last_mut() {
        last.push_str("</");
        last.push_str(tag);
        last.push_str(">");
    }
}

fn push_text(builder: &mut HtmlBuilderResults, text: &str) {
    if let Some(last) = builder.strs.last_mut() {
        last.push_str(text);
    }
}

// injections
fn push_attr_map_injection(builder: &mut HtmlBuilderResults) {
    builder.strs.push("".to_string());
    builder.injs.push(StepKind::AttrMapInjection);
}

fn push_descendant_injection(builder: &mut HtmlBuilderResults) {
    builder.strs.push("".to_string());
    builder.injs.push(StepKind::DescendantInjection);
}
