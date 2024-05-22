use std::collections::HashMap;
use std::vec;

use parsley::{get_text_from_step, parse_str, Step, StepKind};

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum TemplateKind {
    Html,
    Svg,
    MathML,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Injection {
    Text(String),
    Attr(String),
    AttrValue(String, String),
    Tmpl(Template),
    List(Vec<Injection>),
    None,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Template {
    // or just a &str?
    pub kind: TemplateKind,
    pub template_str: String,
    pub injections: Vec<Injection>,
}

pub trait TxmlBuilder {
    fn push_step(&mut self, template_str: &str, step: Step);
}

pub fn build_template(builder: &mut impl TxmlBuilder, template_str: &str) {
    for step in parsley::parse_str(template_str, StepKind::Initial) {
        builder.push_step(template_str, step);
    }
}
