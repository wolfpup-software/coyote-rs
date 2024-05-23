use std::mem;

use parsley::{get_text_from_step, Step, StepKind};

// use these injection details
// for static html, we at least need to know the last tag element
type InjDetails = (Step, StepKind)

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct HtmlBuilder {
    pub tags: Option<Step>,
    pub strs: Vec<String>,
    pub inj_details: Vec<Option<StepKind>>,
}

impl HtmlBuilder {
    pub fn new() -> HtmlBuilder {
        HtmlBuilder {
            tags: None,
            strs: Vec::from(["".to_string()]),
            inj_details: Vec::new(),
        }
    }
    
    pub fn build(&mut self) -> HtmlBuilder {
        // go through steps, get html 
        // get the cacheable chonk associated with this
        let mut builder = HtmlBuilder {
            tags: None,
            strs: Vec::from(["".to_string()]),
            inj_details: Vec::new(),
        };

        mem::swap(self, &mut builder);

        builder
    }

    fn push_step(&mut self, template_str: &str, step: Step) {
        match step.kind {
            // steps
            StepKind::Tag => {
                push_element(self, get_text_from_step(template_str, &step));
            }
            StepKind::ElementClosed => {
                close_element(self, get_text_from_step(template_str, &step));
            }
            StepKind::VoidElementClosed => {
                close_void_element(self, get_text_from_step(template_str, &step));
            }
            StepKind::Attr => {
                add_attr(self, get_text_from_step(template_str, &step));
            }
            StepKind::AttrValue => {
                add_attr_value(self, get_text_from_step(template_str, &step));
            }
            StepKind::AttrValueUnquoted => {
                add_attr_value_unquoted(self, get_text_from_step(template_str, &step));
            }
            StepKind::Text => {
                push_text(self, get_text_from_step(template_str, &step));
            }
            StepKind::TailTag => {
                pop_element(self, get_text_from_step(template_str, &step));
            }
            // injections
            StepKind::AttrMapInjection => {
                push_attr_map_injection(self, get_text_from_step(template_str, &step));
            }
            StepKind::DescendantInjection => {
                push_descendant_injection(self, get_text_from_step(template_str, &step));
            }
            // all other steps silently pass through
            _ => {}
        }
    }
}

// push step
// then get string
fn push_element(builder: &mut HtmlBuilder, tag: &str) {
    builder.tags.push(tag.to_string());
    if let Some(last) = builder.strs.last_mut() {
        last.push('<');
        last.push_str(tag);
    }
}

fn close_element(builder: &mut HtmlBuilder, tag: &str) {
    if let Some(last) = builder.strs.last_mut() {
        last.push_str(">");
    }
}

fn close_void_element(builder: &mut HtmlBuilder, tag: &str) {
    builder.tags.pop();
    if let Some(last) = builder.strs.last_mut() {
        last.push_str(">");
    }
}

fn pop_element(builder: &mut HtmlBuilder, tag: &str) {
    builder.tags.pop();
    if let Some(last) = builder.strs.last_mut() {
        last.push_str("</");
        last.push_str(tag);
        last.push_str(">");
    }
}

fn push_text(builder: &mut HtmlBuilder, text: &str) {
    if let Some(last) = builder.strs.last_mut() {
        last.push_str(text);
    }
}

fn add_attr(builder: &mut HtmlBuilder, tag: &str) {
    if let Some(last) = builder.strs.last_mut() {
        last.push(' ');
        last.push_str(tag);
    }
}

fn add_attr_value(builder: &mut HtmlBuilder, tag: &str) {
    if let Some(last) = builder.strs.last_mut() {
        last.push_str("=\"");
        last.push_str(tag);
        last.push('"');
    }
}

fn add_attr_value_unquoted(builder: &mut HtmlBuilder, tag: &str) {
    if let Some(last) = builder.strs.last_mut() {
        last.push('=');
        last.push_str(tag);
    }
}

// injections
// keep track of tag step to understand if attribute needs to be sanitized
fn push_attr_map_injection(builder: &mut HtmlBuilder, tag: &str) {
    builder.strs.push("".to_string());
    builder.inj_details.push(Some(StepKind::AttrMapInjection));
}

// keep track of tag step to decide if attribute needs to be sanitized later
//  as in, will style and script text injections preserve "<"
//  else replace <
//
//  
fn push_descendant_injection(builder: &mut HtmlBuilder, tag: &str) {
    builder.strs.push("".to_string());
    builder.inj_details.push(Some(StepKind::DescendantInjection));
}
