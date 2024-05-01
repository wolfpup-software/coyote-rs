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

fn build_template(builder: &mut impl TxmlBuilder, template_str: &str) {
    for step in parsley::parse_str(template_str, StepKind::Initial) {
        builder.push_step(template_str, step);
        // match step.kind {
        //     // steps
        //     StepKind::Tag => {
        //         builder.push_element(get_text_from_step(&template_str, &step));
        //     }
        //     StepKind::ElementClosed => {
        //         builder.close_element();
        //     }
        //     StepKind::VoidElementClosed => {
        //         builder.pop_void_element();
        //     }
        //     StepKind::Attr => {
        //         builder.add_attr(get_text_from_step(&template_str, &step));
        //     }
        //     StepKind::AttrValue => {
        //         builder.add_attr_value(get_text_from_step(&template_str, &step));
        //     }
        //     StepKind::AttrValueUnquoted => {
        //         builder.add_attr_value_unquoted(get_text_from_step(&template_str, &step));
        //     }
        //     StepKind::Text => {
        //         builder.push_text(get_text_from_step(&template_str, &step));
        //     }
        //     StepKind::TailTag => {
        //         builder.pop_element(get_text_from_step(&template_str, &step));
        //     }
        //     // injections
        //     StepKind::AttrMapInjection => {
        //         builder.push_attr_map_injection();
        //     }
        //     StepKind::DescendantInjection => {
        //         builder.push_descendants_injection();
        //     }
        //     StepKind::InjectionSpace => {
        //         builder.add_injection_space(get_text_from_step(&template_str, &step));
        //     }
        //     StepKind::InjectionConfirmed => {
        //         builder.confirm_injection();
        //     }
        //     // all other steps silently pass through
        //     _ => {}
        // }
    }
}

// Template (K)ind, (I)njection, (R)eturn type
pub trait DocBuilder<K, I, R> {
    // steps
    // utility
    fn build(&mut self, template: Template<K, I>) -> R;
}
