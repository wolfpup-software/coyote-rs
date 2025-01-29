pub use html::compose as pretty_html;
pub use rulesets::{ClientRules, RulesetImpl, ServerRules};

use component_string::{compose as build_component, BuilderImpl};
use coyote::Component;
use template_string::{compose, Results as TemplateResults};

// The folowing should never change,
// it's a handshake between coyote and devs.
//
// pub trait ComposerImpl {
//     fn build(&mut self, component: &Component) -> String;
// }
//
// HOWEVER
// ComposerImpl is required as an import when ARCd.
// and i don't like that >:(
//
// For now, this is where the contract ends betweeen coyote_html and devs.
// And that should be enough.
//

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
        // start new stack
        // let results = "".to_string;
        // let tag_info_stack = Vec::from([tag_info])
        let template = build_component(&mut self.builder, &self.rules, component);
        pretty_html(&self.rules, &template)
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

    pub fn build(&mut self, component: &Component) -> String {
        let template = build_component(&mut self.builder, &self.rules, component);
        pretty_html(&self.rules, &template)
    }
}
