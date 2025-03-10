use crate::component_string::compose_string;
use crate::components::Component;
use crate::rulesets::{ClientRules, ServerRules, XmlRules};
use crate::template_builder::Builder;

pub struct Html {
    rules: ServerRules,
    builder: Builder,
}

impl Html {
    pub fn new() -> Html {
        Html {
            rules: ServerRules::new(),
            builder: Builder::new(),
        }
    }

    pub fn build(&mut self, component: &Component) -> Result<String, String> {
        compose_string(&mut self.builder, &self.rules, component)
    }
}

// CLIENT HTML
// safer without styles, scripts, or links
pub struct ClientHtml {
    rules: ClientRules,
    builder: Builder,
}

impl ClientHtml {
    pub fn new() -> ClientHtml {
        ClientHtml {
            rules: ClientRules::new(),
            builder: Builder::new(),
        }
    }

    pub fn build(&mut self, component: &Component) -> Result<String, String> {
        compose_string(&mut self.builder, &self.rules, component)
    }
}

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

    pub fn build(&mut self, component: &Component) -> Result<String, String> {
        compose_string(&mut self.builder, &self.rules, component)
    }
}
