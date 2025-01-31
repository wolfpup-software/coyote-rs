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

use crate::component_string::{compose, Builder};
use crate::components::Component;
use crate::rulesets::{ClientRules, ServerRules};

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
        compose(&mut self.builder, &self.rules, component)
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
        compose(&mut self.builder, &self.rules, component)
    }
}
