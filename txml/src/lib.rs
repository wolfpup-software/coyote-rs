use std::collections::HashMap;
use std::vec;

// Components are injected into templates
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Component {
    Text(String),
    AttrVal(String, String),
    Tmpl(Template),
    List(Vec<Component>),
    None,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Template {
    pub template_str: String,
    pub injections: Vec<Component>,
}

// defacto template function
pub fn txml<const N: usize>(template_str: &str, injections: [Component; N]) -> Component {
    Component::Tmpl(Template {
        template_str: template_str.to_string(),
        injections: Vec::from(injections),
    })
}

// ergonomic functions to quickly create Injection Enums
// (makes component code considerably more readable)

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
