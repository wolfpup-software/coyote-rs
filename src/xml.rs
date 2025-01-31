use crate::component_string::{compose_string, Builder};
use crate::components::Component;
use crate::rulesets::XmlRules;

pub struct Xml {
    rules: XmlRules,
    builder: Builder,
}

impl Xml {
    pub fn new() -> Xml {
        Xml {
            rules: XmlRules::new(),
            builder: Builder::new(),
        }
    }

    pub fn build(&mut self, component: &Component) -> String {
        compose_string(&mut self.builder, &self.rules, component)
    }
}
