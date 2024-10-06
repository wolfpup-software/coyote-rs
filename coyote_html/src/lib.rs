pub use html::compose as pretty_html;
pub use sieve::{ClientSieve, Sieve, SieveImpl};

use coyote::Component;
use template_string::{compose as compose_template, BuilderImpl};
use txml_string::{compose as compose_txml, Results as TxmlResults};

pub struct Builder {
    // place to cache txml results
}

impl Builder {
    pub fn new() -> Builder {
        Builder {}
    }
}

impl BuilderImpl for Builder {
    fn build(&mut self, sieve: &dyn SieveImpl, template_str: &str) -> TxmlResults {
        // chance to cache templates here
        compose_txml(sieve, template_str)
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

    pub fn build(&mut self, sieve: &dyn SieveImpl, component: &Component) -> String {
        compose_template(&mut self.builder, sieve, component)
        // just go over with sieve again
    }
}
