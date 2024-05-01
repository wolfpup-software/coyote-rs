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

pub struct TxmlHtmlResult {
    strs: Vec<String>,
    inj_kinds: Vec<Option<StepKind>>,
}

impl TxmlHtmlResult {
    fn new() -> TxmlHtmlResult {
        TxmlHtmlResult {
            strs: Vec::new(),
            inj_kinds: Vec::new(),
        }
    }
}

pub struct TxmlHtmlBuilder {
    tags: Vec<String>,
    pub results: TxmlHtmlResult,
}

impl TxmlHtmlBuilder {
    fn new() -> TxmlHtmlBuilder {
        TxmlHtmlBuilder {
            tags: Vec::new(),
            results: TxmlHtmlResult::new(),
        }
    }
    fn build(&mut self) -> TxmlHtmlBuilder {
        let mut builder = TxmlHtmlBuilder {
            tags: Vec::new(),
            results: TxmlHtmlResult::new(),
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
                // builder.push_element(get_text_from_step(&template_str, &step));
            }
            StepKind::ElementClosed => {
                // builder.close_element();
            }
            StepKind::VoidElementClosed => {
                // builder.pop_void_element();
            }
            StepKind::Attr => {
                // builder.add_attr(get_text_from_step(&template_str, &step));
            }
            StepKind::AttrValue => {
                // builder.add_attr_value(get_text_from_step(&template_str, &step));
            }
            StepKind::AttrValueUnquoted => {
                // builder.add_attr_value_unquoted(get_text_from_step(&template_str, &step));
            }
            StepKind::Text => {
                // builder.push_text(get_text_from_step(&template_str, &step));
            }
            StepKind::TailTag => {
                // builder.pop_element(get_text_from_step(&template_str, &step));
            }
            // injections
            StepKind::AttrMapInjection => {
                // builder.push_attr_map_injection();
            }
            StepKind::DescendantInjection => {
                // builder.push_descendants_injection();
            }
            StepKind::InjectionSpace => {
                // builder.add_injection_space(get_text_from_step(&template_str, &step));
            }
            StepKind::InjectionConfirmed => {
                // builder.confirm_injection();
            }
            // all other steps silently pass through
            _ => {}
        }
    }

    // fn push_element(&mut self, tag: &str) {
    //     self.tags.push(tag.to_string());
    //     if let Some(last) = self.results.last_mut() {
    //         last.push('<');
    //         last.push_str(tag);
    //     }
    // }
    // fn push_text(&mut self, text: &str) {
    //     if let Some(last) = self.results.last_mut() {
    //         last.push_str(text);
    //     }
    // }
    // fn add_attr(&mut self, attr: &str) {
    //     if let Some(last) = self.results.last_mut() {
    //         last.push(' ');
    //         last.push_str(attr);
    //     }
    // }
    // fn add_attr_value_unquoted(&mut self, value: &str) {
    //     if let Some(last) = self.results.last_mut() {
    //         last.push('=');
    //         last.push_str(value);
    //     }
    // }
    // fn add_attr_value(&mut self, value: &str) {
    //     if let Some(last) = self.results.last_mut() {
    //         last.push('=');
    //         last.push('"');
    //         last.push_str(value);
    //         last.push('"');
    //     }
    // }
    // fn close_element(&mut self) {
    //     if let Some(last) = self.results.last_mut() {
    //         last.push('>');
    //     }
    // }
    // // out of sync error if tags arent the same
    // fn pop_element(&mut self, tag: &str) {
    //     // could check if the same
    //     self.tags.pop();
    //     if let Some(last) = self.results.last_mut() {
    //         last.push_str("</");
    //         last.push_str(tag);
    //         last.push('>');
    //     }
    // }
    // fn pop_void_element(&mut self) {
    //     // if current element is void element
    //     match (self.tags.pop(), self.results.last_mut()) {
    //         (Some(tag), Some(last)) => {
    //             // check tag if is void
    //             last.push('>');
    //             // otherwise
    //         }
    //         _ => (),
    //     }
    // }
    // fn push_attr_map_injection(&mut self) {
    //     self.results.push("".to_string());
    //     self.inj_kinds.push(Some(StepKind::AttrMapInjection));
    // }
    // fn push_descendants_injection(&mut self) {
    //     self.results.push("".to_string());
    //     self.inj_kinds.push(Some(StepKind::DescendantInjection));
    // }
    // fn add_injection_space(&mut self, _space: &str) {}
    // fn confirm_injection(&mut self) {}
}

// safety builder
// https://developer.mozilla.org/en-US/docs/Web/HTML/Element

// build doc
// pub fn build_doc(template: Template<K, I, R>)
