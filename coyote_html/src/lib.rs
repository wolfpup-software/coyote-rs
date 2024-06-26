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
    (builder, "".to_string())
}

// create Html with a builder in mind
pub struct Html {}
