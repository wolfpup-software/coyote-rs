use parse::{get_text_from_step, parse_template_str, Step, StepKind};

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Results {
    pub strs: Vec<String>,
    pub injs: Vec<StepKind>,
}

impl Results {
    pub fn new() -> Results {
        Results {
            strs: Vec::from(["".to_string()]),
            injs: Vec::new(),
        }
    }
}

// this needs to be a function
// this is what is cached by a parent scope or context
pub fn compose(template_str: &str) -> Results {
    // check for already built results
    let mut results = Results::new();

    for step in parse_template_str(template_str, StepKind::Initial) {
        match step.kind {
            // steps
            StepKind::Tag => push_element(&mut results, template_str, step),
            StepKind::Attr => add_attr(&mut results, template_str, step),
            StepKind::AttrValueUnquoted => {
                add_attr_value_unquoted(&mut results, template_str, step)
            }
            StepKind::AttrValue => add_attr_value(&mut results, template_str, step),
            StepKind::ElementClosed => close_element(&mut results),
            StepKind::EmptyElementClosed => empty_void_element(&mut results),
            StepKind::TailTag => pop_element(&mut results, template_str, step),
            StepKind::Text => push_text(&mut results, template_str, step),
            // injections
            StepKind::AttrMapInjection => push_attr_map_injection(&mut results),
            StepKind::DescendantInjection => push_descendant_injection(&mut results),
            _ => {}
        }
    }

    results
}

fn push_element(results: &mut Results, template_str: &str, step: Step) {
    let tag = get_text_from_step(template_str, &step);

    if let Some(last) = results.strs.last_mut() {
        last.push('<');
        last.push_str(tag);
    }
}

fn add_attr(results: &mut Results, template_str: &str, step: Step) {
    let attr = get_text_from_step(template_str, &step);

    if let Some(last) = results.strs.last_mut() {
        last.push(' ');
        last.push_str(attr);
    }
}

fn add_attr_value_unquoted(results: &mut Results, template_str: &str, step: Step) {
    let attr_val = get_text_from_step(template_str, &step);

    if let Some(last) = results.strs.last_mut() {
        last.push('=');
        last.push_str(attr_val);
    }
}

fn add_attr_value(results: &mut Results, template_str: &str, step: Step) {
    let attr_val = get_text_from_step(template_str, &step);

    if let Some(last) = results.strs.last_mut() {
        last.push_str("=\"");
        last.push_str(attr_val);
        last.push('"');
    }
}

fn close_element(results: &mut Results) {
    if let Some(last) = results.strs.last_mut() {
        last.push_str(">");
    }
}

fn empty_void_element(results: &mut Results) {
    if let Some(last) = results.strs.last_mut() {
        last.push_str("/>");
    }
}

fn pop_element(results: &mut Results, template_str: &str, step: Step) {
    let tag = get_text_from_step(template_str, &step);
    if let Some(last) = results.strs.last_mut() {
        last.push_str("</");
        last.push_str(tag);
        last.push_str(">");
    }
}

fn push_text(results: &mut Results, template_str: &str, step: Step) {
    let text = get_text_from_step(template_str, &step);
    if let Some(last) = results.strs.last_mut() {
        last.push_str(text);
    }
}

fn push_attr_map_injection(results: &mut Results) {
    results.strs.push("".to_string());
    results.injs.push(StepKind::AttrMapInjection);
}

fn push_descendant_injection(results: &mut Results) {
    results.strs.push("".to_string());
    results.injs.push(StepKind::DescendantInjection);
}
