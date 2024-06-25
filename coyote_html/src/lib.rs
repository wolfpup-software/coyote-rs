pub use html::sieves::{ClientSieve, Sieve, SieveImpl};
pub use txml_string::Builder;

use txml::Component;
use template_string::build_template;
use html::compose as compose_html;


pub fn compose(mut builder: Builder, sieve: &impl SieveImpl, component: &Component) -> (Builder, String) {
    (builder, "".to_string())
}

// create Html with a builder in mind
pub struct Html {}