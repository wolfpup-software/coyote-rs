pub use html::sieves::{ClientSieve, Sieve, SieveImpl};
pub use txml_string::Builder;

use coyote::Component;
use html::compose as compose_html;
use template_string::build_template;

pub fn compose(
    mut builder: Builder,
    sieve: &impl SieveImpl,
    component: &Component,
) -> (Builder, String) {
    let template;
    (builder, template) = build_template(builder, component);
    let results = compose_html(sieve, &template);

    (builder, results)
}

// create Html with a builder in mind
pub struct Html {
    pub builder: TemplateBuilder,
}

impl Html {
    fn new() -> Html {
        Html {
            builder: TemplateBuilder::new(),
        }
    }

    fn from_builder(builder: TemplateBuilder) -> Html {
        Html { builder: builder }
    }

    fn build(&mut self, component: &Component, sieve: &impl SieveImpl) -> String {
        let template = build_template(self.builder, component);
        compose_html(sieve, &template)
    }
}
