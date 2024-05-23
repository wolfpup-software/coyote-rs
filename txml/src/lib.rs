use std::collections::HashMap;
use std::vec;

use parsley::{parse_str, Step, StepKind};

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Injection {
    Text(String),
    AttrVal(String, String),
    Tmpl(Template),
    List(Vec<Injection>),
    None,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Template {
    pub template_str: String,
    pub injections: Vec<Injection>,
}

// defacto template function 
pub fn txml<const N: usize>(template_str: &str, injections: [Injection; N]) -> Template {
    Template {
        template_str: template_str.to_string(),
        injections: Vec::from(injections),
    }
}

// ergonomic functions to quickly create Injection Enums
// (makes component code considerably more readable)
pub fn text(txt: &str) -> Injection {
    Injection::Text(txt.to_string())
}

pub fn attr(txt: &str) -> Injection {
    Injection::Text(txt.to_string())
}

pub fn attrVal(txt: &str, text: &str) -> Injection {
    Injection::Text(txt.to_string())
}

pub fn tmpl(template: Template) -> Injection {
    Injection::Tmpl(template)
}

pub fn list<const N: usize>(list: [Injection; N]) -> Injection {
    Injection::List(Vec::from(list))
}

