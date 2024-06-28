pub use html::compose as pretty_html;
pub use html::sieves::{ClientSieve, Sieve, SieveImpl};

use coyote::Component;
use template_string::{compose as compose_template, BuilderImpl};
use txml_string::{compose as compose_txml, Results as TxmlResults};

// Builder without caching
pub struct Builder {}

impl Builder {
    pub fn new() -> Builder {
        Builder {}
    }
}

impl BuilderImpl for Builder {
    fn build(&mut self, template_str: &str) -> TxmlResults {
        // chance to cache templates here
        compose_txml(template_str)
    }
}

// create Html with a builder in mind
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

    pub fn build(&mut self, component: &Component) -> String {
        compose_template(&mut self.builder, component)
    }
}
