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

pub struct TxmlHtmlBuilder {
    tags: Vec<String>,
    strs: Vec<String>,
    inj_kinds: Vec<Option<StepKind>>,
}

impl TxmlHtmlBuilder {
    fn new() -> TxmlHtmlBuilder {
        TxmlHtmlBuilder {
            tags: Vec::new(),
            strs: Vec::new(),
            inj_kinds: Vec::new(),
        }
    }
    fn build(&mut self) -> TxmlHtmlBuilder {
        let mut builder = TxmlHtmlBuilder {
            tags: Vec::new(),
            strs: Vec::new(),
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
            StepKind::InjectionSpace => {
                push_injection_space(self, get_text_from_step(template_str, &step));
            }
            StepKind::InjectionConfirmed => {
                push_injection_confirmed(self, get_text_from_step(template_str, &step));
            }
            // all other steps silently pass through
            _ => {}
        }
    }
}

fn push_element(builder: &mut TxmlHtmlBuilder, tag: &str) {
    builder.tags.push(tag.to_string());

    // builder.results.push_str(&"\t".repeat(builder.tab_count));
    // builder.results.push('<');
    // builder.results.push_str(tag);
}

fn close_element(builder: &mut TxmlHtmlBuilder, tag: &str) {
    // builder.results.push_str(">\n");
}

fn close_void_element(builder: &mut TxmlHtmlBuilder, tag: &str) {
    builder.tags.pop();

    // builder.results.push_str(">\n");
}

fn pop_element(builder: &mut TxmlHtmlBuilder, tag: &str) {
    builder.tags.pop();

    // builder.results.push_str(&"\t".repeat(builder.tab_count));
    // builder.results.push_str("</");
    // builder.results.push_str(tag);
    // builder.results.push_str(">\n");
}

fn push_text(builder: &mut TxmlHtmlBuilder, text: &str) {
    // let space = "\t".repeat(builder.tab_count);
    // let mut split_text = text.trim().split('\n');
    // while let Some(line) = split_text.next() {
    //     builder.results.push_str(&space);
    //     builder.results.push_str(line.trim());
    // }
    // builder.results.push('\n');
}

fn add_attr(builder: &mut TxmlHtmlBuilder, tag: &str) {
    // builder.results.push(' ');
    // builder.results.push_str(tag);
}

fn add_attr_value(builder: &mut TxmlHtmlBuilder, tag: &str) {
    // builder.results.push_str("=\"");
    // builder.results.push_str(tag);
    // builder.results.push('"');
}

fn add_attr_value_unquoted(builder: &mut TxmlHtmlBuilder, tag: &str) {
    // builder.results.push('=');
    // builder.results.push_str(tag);
}

// injections
// all the same
fn push_attr_map_injection(builder: &mut TxmlHtmlBuilder, tag: &str) {
    // builder.results.push_str(tag);
}

fn push_descendant_injection(builder: &mut TxmlHtmlBuilder, tag: &str) {
    // builder.results.push_str(tag);
}

fn push_injection_space(builder: &mut TxmlHtmlBuilder, tag: &str) {
    // builder.results.push_str(tag);
}

fn push_injection_confirmed(builder: &mut TxmlHtmlBuilder, tag: &str) {
    // builder.results.push_str(tag);
}

// safety builder
// https://developer.mozilla.org/en-US/docs/Web/HTML/Element

// build doc
// pub fn build_doc(template: Template<K, I, R>)
