use parsley::StepKind;
use txml::{Template, TxmlBuilder};

pub struct PretyHtmlBuilder {
    results: String,
}

impl PretyHtmlBuilder {
    fn new() -> PretyHtmlBuilder {
        PretyHtmlBuilder {
            results: "".to_string(),
        }
    }
}

impl TxmlBuilder for PretyHtmlBuilder {
    fn push_element(&mut self, tag: &str) {
        self.results.push('<');
        self.results.push_str(tag);
    }
    fn push_text(&mut self, text: &str) {
        self.results.push_str(text);
    }
    fn add_attr(&mut self, attr: &str) {
        self.results.push(' ');
        self.results.push_str(attr);
    }
    fn add_attr_value_unquoted(&mut self, value: &str) {
        self.results.push('=');
        self.results.push_str(value);
    }
    fn add_attr_value(&mut self, value: &str) {
        self.results.push('=');
        self.results.push('"');
        self.results.push_str(value);
        self.results.push('"');
    }
    fn close_element(&mut self) {
        self.results.push('>');
    }
    // out of sync error if tags arent the same
    fn pop_element(&mut self, tag: &str) {
        // could check if the same
        self.results.push_str("</");
        self.results.push_str(tag);
        self.results.push('>');
    }
    fn pop_void_element(&mut self) {
        // if current element is void element
        // check tag if is void
        self.results.push('>');
    }
    fn push_attr_map_injection(&mut self) {
        self.results.push('{');
    }
    fn push_descendants_injection(&mut self) {
        self.results.push('{');
    }
    fn add_injection_space(&mut self, space: &str) {
        self.results.push_str(space);
    }
    fn confirm_injection(&mut self) {
        self.results.push('}');
    }
}
