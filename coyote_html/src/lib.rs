pub use html::compose as pretty_html;
pub use rulesets::{ClientRules, RulesetImpl, ServerRules};

use component_string::{compose as build_component, BuilderImpl};
use coyote::Component;
use template_string::{compose, Results as TemplateResults};

pub struct Builder {
    // place to cache txml results
}

impl Builder {
    pub fn new() -> Builder {
        Builder {}
    }
}

impl BuilderImpl for Builder {
    fn build(&mut self, ruleset: &dyn RulesetImpl, template_str: &str) -> TemplateResults {
        // chance to cache templates here
        compose(ruleset, template_str)
    }
}

pub struct Html {
    pub builder: Builder,
}

impl Html {
    pub fn new() -> Html {
        Html {
            builder: Builder::new(),
        }
    }

    pub fn from_builder(builder: Builder) -> Html {
        Html { builder: builder }
    }

    pub fn build(&mut self, ruleset: &dyn RulesetImpl, component: &Component) -> String {
        let template = build_component(&mut self.builder, ruleset, component);
        pretty_html(ruleset, &template)
    }
}
