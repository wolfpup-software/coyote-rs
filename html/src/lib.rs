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

use parsley::StepKind;
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
    pub results: Vec<String>,
    pub inj_kinds: Vec<Option<StepKind>>,
}

impl TxmlHtmlBuilder {
    fn new() -> TxmlHtmlBuilder {
        TxmlHtmlBuilder {
            tags: Vec::new(),
            results: Vec::from(["".to_string()]),
            inj_kinds: Vec::new(),
        }
    }
    fn build(&mut self) -> TxmlHtmlBuilder {
        let mut builder = TxmlHtmlBuilder {
            tags: Vec::new(),
            results: Vec::from(["".to_string()]),
            inj_kinds: Vec::new(),
        };

        mem::swap(self, &mut builder);

        builder
    }
}

impl TxmlBuilder for TxmlHtmlBuilder {
    fn push_element(&mut self, tag: &str) {
        self.tags.push(tag.to_string());
        if let Some(last) = self.results.last_mut() {
            last.push('<');
            last.push_str(tag);
        }
    }
    fn push_text(&mut self, text: &str) {
        if let Some(last) = self.results.last_mut() {
            last.push_str(text);
        }
    }
    fn add_attr(&mut self, attr: &str) {
        if let Some(last) = self.results.last_mut() {
            last.push(' ');
            last.push_str(attr);
        }
    }
    fn add_attr_value_unquoted(&mut self, value: &str) {
        if let Some(last) = self.results.last_mut() {
            last.push('=');
            last.push_str(value);
        }
    }
    fn add_attr_value(&mut self, value: &str) {
        if let Some(last) = self.results.last_mut() {
            last.push('=');
            last.push('"');
            last.push_str(value);
            last.push('"');
        }
    }
    fn close_element(&mut self) {
        if let Some(last) = self.results.last_mut() {
            last.push('>');
        }
    }
    // out of sync error if tags arent the same
    fn pop_element(&mut self, tag: &str) {
        // could check if the same
        self.tags.pop();
        if let Some(last) = self.results.last_mut() {
            last.push_str("</");
            last.push_str(tag);
            last.push('>');
        }
    }
    fn pop_void_element(&mut self) {
        // if current element is void element
        match (self.tags.pop(), self.results.last_mut()) {
            (Some(tag), Some(last)) => {
                // check tag if is void
                last.push('>');
                // otherwise
            }
            _ => (),
        }
    }
    // injections
    fn push_attr_map_injection(&mut self) {
        self.results.push("".to_string());
        self.inj_kinds.push(Some(StepKind::AttrMapInjection));
    }
    fn push_descendants_injection(&mut self) {
        self.results.push("".to_string());
        self.inj_kinds.push(Some(StepKind::DescendantInjection));
    }
    // utility
}

/*
first build everything
then add option to skip

match tag {
    "script" => self.current_element.push(tag.to_string()),
    "style" => self.current_element.push(tag.to_string()),
    _ => (),
};
*/

// pre --> do not to tabs or spaces

/*
    couple options:
        no formatting, only "pre" elements, []string returned
        format, save pieces, split across arrays,
        no formatting, format the entire document after

        either way two steps, all valid xml-ish stuff

        the main complication: this is unique to a static document
            otherwise this would be a kind of tree or component structure

        two passes,
            one gives us a kind of minimal html
            other process can "prettify" an outputed document

            both can use parsley

    // can be defined as neccessary
    SafetySieve {
        is valid element, has hyphens or is apart of list
        can have descendants ? (tag)
        is unsafe element (tag)
        valid attribute (tag, attr)
    }

    pub enum ElementType {
        dangerous,
        no_descendants,
        void_element,
        element,
    }

    pub trait SafetySieve {
        fn get_element_type(&self, tag: &str) -> ElementType;
        fn is_void_element(&self, tag: &str) -> bool;
        fn cannot_have_descendants(&self, tag: &str) -> bool;
        fn must_preserve_spacew(&self, tag: &str) -> bool;
    }

*/

// safety builder
// https://developer.mozilla.org/en-US/docs/Web/HTML/Element
fn is_html_element(tag: &str) -> bool {
    match tag {
        "html" => true,
        "!DOCTYPE" => true,
        "base" => true,
        "head" => true,
        "link" => true,
        "meta" => true,
        "style" => true,
        "title" => true,
        _ => false,
    }
}

fn is_html_void_element(tag: &str) -> bool {
    match tag {
        "!DOCTYPE" => true,
        "area" => true,
        "base" => true,
        "br" => true,
        "col" => true,
        "embed" => true,
        "hr" => true,
        "img" => true,
        "input" => true,
        "link" => true,
        "meta" => true,
        "param" => true,
        "source" => true,
        "track" => true,
        "wbr" => true,
        _ => false,
    }
}

fn is_element_without_descendants(tag: &str) -> bool {
    match tag {
        "!DOCTYPE" => true,
        "style" => true,
        "title" => true,
        _ => false,
    }
}

// https://developer.mozilla.org/en-US/docs/Web/API/Element#events
fn is_banned_attribute(tag: &str) -> bool {
    match tag {
        "onclick" => true,
        "onpointerdown" => true,
        _ => false,
    }
}

// build doc
// pub fn build_doc(template: Template<K, I, R>)
