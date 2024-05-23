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
// (great spot to escape characters)

pub fn text(txt: &str) -> Component {
    let escaped = txt.replace("<", "&lt;").replace("&", "&amp;");

    Component::Text(escaped)
}

pub fn unescaped_text(txt: &str) -> Component {
    Component::Text(txt.to_string())
}

pub fn attr(txt: &str) -> Component {
    let escaped_attr = txt
        .replace("<", "")
        .replace(">", "")
        .replace("&", "")
        .replace("\"", "")
        .replace("'", "");

    Component::Text(escaped_attr)
}

pub fn attrVal(txt: &str, text: &str) -> Component {
    let escaped_attr = txt
        .replace("<", "")
        .replace(">", "")
        .replace("&", "")
        .replace("\"", "")
        .replace("'", "");

    let escaped_value = text.replace("\"", "&quot;").replace("&", "&amp;");
    Component::AttrVal(escaped_attr, escaped_value)
}

pub fn tmpl(template: Template) -> Component {
    Component::Tmpl(template)
}

pub fn list<const N: usize>(components: [Component; N]) -> Component {
    Component::List(Vec::from(components))
}

pub fn vecList<const N: usize>(components: Vec<Component>) -> Component {
    Component::List(components)
}
