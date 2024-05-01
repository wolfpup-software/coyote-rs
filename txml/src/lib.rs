use std::collections::HashMap;
use std::vec;

use parsley::{get_text_from_step, parse_str, Step, StepKind};

// template -> iterpretaion -> document

// This is returnd by functional components
#[derive(Debug)]
pub struct Template<'a, K, I> {
    pub kind: K,
    pub injections: Vec<I>,
    pub template_str: &'a str,
}

// Intermediate (R)etrun Type, a "chunk" or "node"
pub trait TxmlBuilder {
    fn push_step(&mut self, template_str: &str, step: Step);
}

pub fn build_template(builder: &mut impl TxmlBuilder, template_str: &str) {
    for step in parsley::parse_str(template_str, StepKind::Initial) {
        builder.push_step(template_str, step);
    }
}

// Template (K)ind, (I)njection, (R)eturn type
pub trait DocBuilder<K, I, R> {
    // steps
    // utility
    fn build(&mut self, template: Template<K, I>) -> R;
}
