/*
    HTML template requirements:

    handle void elements

    don't add closing slashes (not valid html)

    do not add "on" events to elements,

    this will require knowing parent element and last attribute

    should there be an all or nothing quit attidue? if one wrong thing happens
    than nothing is built
*/

use txml::{StackBit, Template};

#[derive(Debug)]
pub enum Injection<'a, E> {
    Text(&'a str),
    Attr(&'a str),
    AttrValue(&'a str, &'a str),
    Callback(&'a str, E),
    Template(Template<'a, E>),
    List(Vec<Injection<'a, E>>),
}

type NonCallback = ();

struct StaticHtmlBuilder<'a> {
    result: String,
    tab_count: usize,
    stack: Vec<StackBit<'a, Injection<'a, NonCallback>>>,
}

impl<'a> StaticHtmlBuilder<'_> {
    // steps
    fn push_node() {}
    fn add_attr() {}
    fn add_attr_value() {}
    fn add_text() {}
    fn pop_node() {}

    // injections
    fn add_attr_map() {}
    fn get_descendants() {}
}

// Injections could be entirely external to the "builder"

// where E is for event callbacks

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
