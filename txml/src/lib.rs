use std::collections::HashMap;
use std::vec;

use parsley::{parse_str, Step, StepKind};

#[derive(Debug, Clone, Eq, PartialEq)]
// Components are injected into templates
pub enum Component {
    Text(String),
    AttrVal(String, String),
    Tmpl(Template),
    List(Vec<Component>),
    None,
}

// give a more romantic type name for composition
// "template" is the string, not what is being returned
// Component is just for names and readability in codebases

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Template {
    pub template_str: String,
    pub injections: Vec<Component>,
}

// ergonomic functions to quickly create Injection Enums
// (makes component code considerably more readable)

// defacto template function
pub fn txml<const N: usize>(template_str: &str, injections: [Component; N]) -> Component {
    Component::Tmpl(Template {
        template_str: template_str.to_string(),
        injections: Vec::from(injections),
    })
}

pub fn text(txt: &str) -> Component {
    Component::Text(txt.to_string())
}

pub fn attr(txt: &str) -> Component {
    Component::Text(txt.to_string())
}

pub fn attrVal(txt: &str, text: &str) -> Component {
    Component::Text(txt.to_string())
}

pub fn tmpl(template: Template) -> Component {
    Component::Tmpl(template)
}

pub fn list<const N: usize>(list: [Component; N]) -> Component {
    Component::List(Vec::from(list))
}

pub fn vecList<const N: usize>(list: Vec<Component>) -> Component {
    Component::List(list)
}
