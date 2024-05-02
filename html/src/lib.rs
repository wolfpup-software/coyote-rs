/*
    HTML template requirements:

    handle void elements

    don't add closing slashes (not valid html)

    do not add "on" events to elements,

    this will require knowing parent element and last attribute

    should there be an all or nothing quit attidue? if one wrong thing happens
    than nothing is built
*/

use std::mem;

use parsley::{get_text_from_step, Step, StepKind};
use txml::{Template, TxmlBuilder};

pub enum TemplateKind {
    Html,
    Svg,
}

// this is a type that is used across other types:
//  text -> html
//  nodes -> dom
// Template (K)ind, (C)allback
#[derive(Debug)]
pub enum Injection<'a, K, C> {
    Text(String),
    TextStr(&'a str),
    Attr(String),
    AttrStr(&'a str),
    AttrValue(String, String),
    AttrValueStr(&'a str, &'a str),
    Callback(String, C),
    Template(Template<'a, K, C>),
    List(Vec<Injection<'a, K, C>>),
}

type NonCallback = ();

// Template options

// this is close to a tagged template literal
// debug clone

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct TxmlHtmlBuilder {
    pub tags: Vec<String>,
    pub strs: Vec<String>,
    pub inj_kinds: Vec<Option<StepKind>>,
}

impl TxmlHtmlBuilder {
    pub fn new() -> TxmlHtmlBuilder {
        TxmlHtmlBuilder {
            tags: Vec::new(),
            strs: Vec::from(["".to_string()]),
            inj_kinds: Vec::new(),
        }
    }
    pub fn build(&mut self) -> TxmlHtmlBuilder {
        let mut builder = TxmlHtmlBuilder {
            tags: Vec::new(),
            strs: Vec::from(["".to_string()]),
            inj_kinds: Vec::new(),
        };

        mem::swap(self, &mut builder);

        builder
    }
}

impl TxmlBuilder for TxmlHtmlBuilder {
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

fn push_element(builder: &mut TxmlHtmlBuilder, tag: &str) {
    builder.tags.push(tag.to_string());
    if let Some(last) = builder.strs.last_mut() {
        last.push('<');
        last.push_str(tag);
    }
}

fn close_element(builder: &mut TxmlHtmlBuilder, tag: &str) {
    if let Some(last) = builder.strs.last_mut() {
        last.push_str(">");
    }
}

fn close_void_element(builder: &mut TxmlHtmlBuilder, tag: &str) {
    builder.tags.pop();
    if let Some(last) = builder.strs.last_mut() {
        last.push_str(">");
    }
}

fn pop_element(builder: &mut TxmlHtmlBuilder, tag: &str) {
    builder.tags.pop();
    if let Some(last) = builder.strs.last_mut() {
        last.push_str("</");
        last.push_str(tag);
        last.push_str(">");
    }
}

fn push_text(builder: &mut TxmlHtmlBuilder, text: &str) {
    if let Some(last) = builder.strs.last_mut() {
        last.push_str(text);
    }
}

fn add_attr(builder: &mut TxmlHtmlBuilder, tag: &str) {
    if let Some(last) = builder.strs.last_mut() {
        last.push(' ');
        last.push_str(tag);
    }
}

fn add_attr_value(builder: &mut TxmlHtmlBuilder, tag: &str) {
    if let Some(last) = builder.strs.last_mut() {
        last.push_str("=\"");
        last.push_str(tag);
        last.push('"');
    }
}

fn add_attr_value_unquoted(builder: &mut TxmlHtmlBuilder, tag: &str) {
    if let Some(last) = builder.strs.last_mut() {
        last.push('=');
        last.push_str(tag);
    }
}

// injections
fn push_attr_map_injection(builder: &mut TxmlHtmlBuilder, tag: &str) {
    builder.strs.push("".to_string());
    builder.inj_kinds.push(Some(StepKind::AttrMapInjection));
}

fn push_descendant_injection(builder: &mut TxmlHtmlBuilder, tag: &str) {
    builder.strs.push("".to_string());
    builder.inj_kinds.push(Some(StepKind::DescendantInjection));
}
