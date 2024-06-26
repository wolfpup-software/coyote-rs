use parse::{get_text_from_step, parse_template_str, Step, StepKind};

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct BuilderResults {
    pub strs: Vec<String>,
    pub injs: Vec<StepKind>,
}

impl BuilderResults {
    pub fn new() -> BuilderResults {
        BuilderResults {
            strs: Vec::from(["".to_string()]),
            injs: Vec::new(),
        }
    }
}

pub struct Builder {}

impl Builder {
    pub fn new() -> Builder {
        Builder {}
    }

    pub fn build(&self, template_str: &str) -> BuilderResults {
        // check for already built results
        let mut results = BuilderResults::new();

        for step in parse_template_str(template_str, StepKind::Initial) {
            push_step(&mut results, template_str, step);
        }

        results
    }
}

fn push_step(results: &mut BuilderResults, template_str: &str, step: Step) {
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

fn push_element(results: &mut BuilderResults, template_str: &str, step: Step) {
    let tag = get_text_from_step(template_str, &step);

    if let Some(last) = results.strs.last_mut() {
        last.push('<');
        last.push_str(tag);
    }
}

fn add_attr(results: &mut BuilderResults, template_str: &str, step: Step) {
    let attr = get_text_from_step(template_str, &step);

    if let Some(last) = results.strs.last_mut() {
        last.push(' ');
        last.push_str(attr);
    }
}

fn add_attr_value_unquoted(results: &mut BuilderResults, template_str: &str, step: Step) {
    let attr_val = get_text_from_step(template_str, &step);

    if let Some(last) = results.strs.last_mut() {
        last.push('=');
        last.push_str(attr_val);
    }
}

fn add_attr_value(results: &mut BuilderResults, template_str: &str, step: Step) {
    let attr_val = get_text_from_step(template_str, &step);

    if let Some(last) = results.strs.last_mut() {
        last.push_str("=\"");
        last.push_str(attr_val);
        last.push('"');
    }
}

fn close_element(results: &mut BuilderResults) {
    if let Some(last) = results.strs.last_mut() {
        last.push_str(">");
    }
}

fn empty_void_element(results: &mut BuilderResults) {
    if let Some(last) = results.strs.last_mut() {
        last.push_str("/>");
    }
}

fn pop_element(results: &mut BuilderResults, template_str: &str, step: Step) {
    let tag = get_text_from_step(template_str, &step);
    if let Some(last) = results.strs.last_mut() {
        last.push_str("</");
        last.push_str(tag);
        last.push_str(">");
    }
}

fn push_text(results: &mut BuilderResults, template_str: &str, step: Step) {
    let text = get_text_from_step(template_str, &step);
    if let Some(last) = results.strs.last_mut() {
        last.push_str(text);
    }
}

fn push_attr_map_injection(results: &mut BuilderResults) {
    results.strs.push("".to_string());
    results.injs.push(StepKind::AttrMapInjection);
}

fn push_descendant_injection(results: &mut BuilderResults) {
    results.strs.push("".to_string());
    results.injs.push(StepKind::DescendantInjection);
}
