use parsley::{get_text_from_step, Step, StepKind};
use txml::{Template, TxmlBuilder};

pub struct PrettyHtmlConfig {
    max_text_width: usize,
}

pub struct PretyHtmlBuilder {
    tab_count: usize,
    text_width: usize,
    results: String,
}

impl PretyHtmlBuilder {
    fn new() -> PretyHtmlBuilder {
        PretyHtmlBuilder {
            tab_count: 0,
            text_width: 0,
            results: "".to_string(),
        }
    }
}

impl TxmlBuilder for PretyHtmlBuilder {
    fn push_step(&mut self, template_str: &str, step: Step) {
        match step.kind {
            // steps
            StepKind::Tag => {
                // builder.push_element(get_text_from_step(&template_str, &step));
            }
            StepKind::ElementClosed => {
                // builder.close_element();
            }
            StepKind::VoidElementClosed => {
                // builder.pop_void_element();
            }
            StepKind::Attr => {
                // builder.add_attr(get_text_from_step(&template_str, &step));
            }
            StepKind::AttrValue => {
                // builder.add_attr_value(get_text_from_step(&template_str, &step));
            }
            StepKind::AttrValueUnquoted => {
                // builder.add_attr_value_unquoted(get_text_from_step(&template_str, &step));
            }
            StepKind::Text => {
                // builder.push_text(get_text_from_step(&template_str, &step));
            }
            StepKind::TailTag => {
                // builder.pop_element(get_text_from_step(&template_str, &step));
            }
            // injections
            StepKind::AttrMapInjection => {
                // builder.push_attr_map_injection();
            }
            StepKind::DescendantInjection => {
                // builder.push_descendants_injection();
            }
            StepKind::InjectionSpace => {
                // builder.add_injection_space(get_text_from_step(&template_str, &step));
            }
            StepKind::InjectionConfirmed => {
                // builder.confirm_injection();
            }
            // all other steps silently pass through
            _ => {}
        }
    }

    // fn push_element(&mut self, tag: &str) {
    //     self.results.push('<');
    //     self.results.push_str(tag);
    // }
    // fn push_text(&mut self, text: &str) {
    //     self.results.push_str(text);
    // }
    // fn add_attr(&mut self, attr: &str) {
    //     self.results.push(' ');
    //     self.results.push_str(attr);
    // }
    // fn add_attr_value_unquoted(&mut self, value: &str) {
    //     self.results.push('=');
    //     self.results.push_str(value);
    // }
    // fn add_attr_value(&mut self, value: &str) {
    //     self.results.push('=');
    //     self.results.push('"');
    //     self.results.push_str(value);
    //     self.results.push('"');
    // }
    // fn close_element(&mut self) {
    //     self.results.push('>');
    // }
    // // out of sync error if tags arent the same
    // fn pop_element(&mut self, tag: &str) {
    //     // could check if the same
    //     self.results.push_str("</");
    //     self.results.push_str(tag);
    //     self.results.push('>');
    // }
    // fn pop_void_element(&mut self) {
    //     // if current element is void element
    //     // check tag if is void
    //     self.results.push('>');
    // }
    // fn push_attr_map_injection(&mut self) {
    //     self.results.push('{');
    // }
    // fn push_descendants_injection(&mut self) {
    //     self.results.push('{');
    // }
    // fn add_injection_space(&mut self, space: &str) {
    //     self.results.push_str(space);
    // }
    // fn confirm_injection(&mut self) {
    //     self.results.push('}');
    // }
}
