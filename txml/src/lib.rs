use std::collections::HashMap;
use std::vec;

use parsley::{parse_str, Step, StepKind};

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Injection {
    Text(String),
    Attr(String),
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

pub trait TxmlBuilder {
    fn push_step(&mut self, template_str: &str, step: Step);
}

pub fn txml(template_str: String, injections: Vec<Injection>) -> Template {
    Template {
        template_str: template_str,
        injections: injections,
    }
}
