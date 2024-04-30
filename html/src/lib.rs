/*
    HTML template requirements:

    handle void elements

    don't add closing slashes (not valid html)

    do not add "on" events to elements,

    this will require knowing parent element and last attribute

    should there be an all or nothing quit attidue? if one wrong thing happens
    than nothing is built
*/

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
    Attr(String),
    AttrValue(String, String),
    TextStr(&'a str),
    AttrStr(&'a str),
    AttrValueStr(&'a str, &'a str),
    Callback(String, C),
    Template(Template<'a, K, C>),
    List(Vec<Injection<'a, K, C>>),
}

type NonCallback = ();

// Template options

pub struct TxmlHtmlBuilder {
    current_element: Vec<String>,
    results: Vec<String>,
    inj_kinds: Vec<Option<StepKind>>,
}
impl TxmlHtmlBuilder {
    fn new() -> TxmlHtmlBuilder {
        TxmlHtmlBuilder {
            current_element: Vec::new(),
            results: Vec::from(["".to_string()]),
            inj_kinds: Vec::new(),
        }
    }
}

impl TxmlBuilder<TxmlHtmlBuilder> for TxmlHtmlBuilder {
    fn push_element(&mut self, tag: &str) {
        match tag {
            "script" => self.current_element.push(tag.to_string()),
            "style" => self.current_element.push(tag.to_string()),
            _ => (),
        };
        self.current_element.push(tag.to_string());
        if let Some(last) = self.results.last_mut() {
            last.push('<');
            last.push_str(tag);
        }
    }
    fn push_text(&mut self, text: &str) {}
    fn add_attr(&mut self, attr: &str) {
        if let Some(last) = self.results.last_mut() {
            last.push(' ');
            last.push_str(attr);
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
        // if void element pop
        //
    }
    fn pop_element(&mut self, tag: &str) {
        match tag {
            "script" => self.current_element.pop(),
            "style" => self.current_element.pop(),
            _ => None,
        };
        if let Some(last) = self.results.last_mut() {
            last.push_str("</");
            last.push_str(tag);
            last.push('>');
        }
    }
    fn pop_void_element(&mut self) {
        if let Some(last) = self.results.last_mut() {
            last.push('>');
        }
    }
    // injections
    fn push_attr_map_injection(&mut self) {
        self.inj_kinds.push(Some(StepKind::AttrMapInjection))
    }
    fn push_descendants_injection(&mut self) {
        if let Some(last) = self.current_element.last() {
            match last.as_str() {
                "script" => self.inj_kinds.push(None),
                "style" => self.inj_kinds.push(None),
                _ => self.inj_kinds.push(Some(StepKind::DescendantInjection)),
            }
        }
    }
    // utility
    fn build(&mut self) -> TxmlHtmlBuilder {
        TxmlHtmlBuilder {
            current_element: Vec::new(),
            results: Vec::new(),
            inj_kinds: Vec::new(),
        }
    }
}

fn is_html_void_element(tag: &str) -> bool {
    match tag {
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

/*


// this should be separated. the Injection should be provided by the caller
pub struct StaticHtmlBuilder<'a> {
    result: String,
    tab_count: usize,
    stack: Vec<StackBit<'a, Injection<'a, NonCallback>>>,
}

impl<'a> StaticHtmlBuilder<'_> {
    // eventually this is the cache step ::new(1024) max build steps
    pub fn new() -> StaticHtmlBuilder<'a> {
        StaticHtmlBuilder {
            result: "".to_string(),
            tab_count: 0,
            stack: Vec::new(),
        }
    }

    pub fn build(&self) -> String {
        self.result.clone()
    }

    pub fn reset(mut self) {
        self = StaticHtmlBuilder {
            result: "".to_string(),
            tab_count: 0,
            stack: Vec::new(),
        };
    }
}

// tale of two builders
// TemplateBuilder for caching -> { text: Vec(), descendants: [] }
// StaticHtmlBuilder for the actual page page

impl<'a> TxmlBuilder<'a, Injection<'a, NonCallback>> for StaticHtmlBuilder<'_> {
    // steps
    fn push_node(&self, tag: &'a str) {}
    fn add_attr(&self, attr: &'a str) {}
    fn add_attr_value(&self, value: &'a str) {}
    fn push_text(&self, text: &'a str) {}
    fn pop_node(&self, tag: &'a str) {}
    fn pop_independent_node(&self) {}

    // injections
    fn add_attr_map(&self, injections: Injection<'a, NonCallback>) {}
    fn get_descendants(
        &self,
        injections: Injection<'a, NonCallback>,
    ) -> Vec<StackBit<'a, Injection<'a, NonCallback>>> {
        //
        Vec::new()
    }
}

// Injections could be entirely external to the "builder"

// where E is for event callbacks

fn is_html_void_element(tag: &str) -> bool {
    match tag {
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

fn add_close_tagname(result: &mut String, tab_count: usize, text: &str) -> () {
    // tab_count -= 1;
    result.push_str(&"\t".repeat(tab_count));
    result.push_str("</");
    result.push_str(text);
    result.push_str(">\n");
}

fn add_independent_node(result: &mut String, tab_count: usize, text: &str) -> () {
    result.push_str("/>\n");
    // tab_count -= 1;
}

fn add_node_closed(result: &mut String, tab_count: usize, text: &str) -> () {
    result.push_str(">\n");
    // tab_count += 1;
}

fn add_tag(result: &mut String, tab_count: usize, text: &str) -> () {
    result.push_str(&"\t".repeat(tab_count));
    result.push_str("<");
    result.push_str(text);
}

fn add_text(result: &mut String, tab_count: usize, text: &str) -> () {
    result.push_str(&"\t".repeat(tab_count));
    result.push_str(text.trim());
    result.push_str("\n");
}

fn add_attr(result: &mut String, attr: &str) -> () {
    result.push_str(" ");
    result.push_str(attr);
}

fn add_attr_value(result: &mut String, attr: &str, value: &str) -> () {
    result.push_str(" ");
    result.push_str(attr);
    result.push_str("=\"");
    result.push_str(value);
    result.push_str("\"");
}

//
pub fn html<'a, T>(template_str: &'a str, injections: Vec<T>) -> Template<'a, T> {
    Template {
        kind: "html",
        template_str: template_str,
        injections: injections,
    }
}
*/
