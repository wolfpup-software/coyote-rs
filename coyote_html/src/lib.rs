pub use html::compose as pretty_html;
pub use rulesets::{ClientRules, RulesetImpl, ServerRules};

use component_string::{compose as build_component, BuilderImpl};
use coyote::Component;
use template_string::{compose, Results as TemplateResults};

struct Builder {}

impl Builder {
    pub fn new() -> Builder {
        Builder {}
    }
}

impl BuilderImpl for Builder {
    fn build(&mut self, rules: &dyn RulesetImpl, template_str: &str) -> TemplateResults {
        // chance to cache templates here
        compose(rules, template_str)
    }
}

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

    pub fn build(&mut self, component: &Component) -> String {
        let template = build_component(&mut self.builder, &self.rules, component);
        pretty_html(&self.rules, &template)
    }
}

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

    pub fn build(&mut self, component: &Component) -> String {
        let template = build_component(&mut self.builder, &self.rules, component);
        pretty_html(&self.rules, &template)
    }
}
