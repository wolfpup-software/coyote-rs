mod component_string;
mod compose_steps;
mod coyote;
mod parse;
mod routes;
mod rulesets;
mod sliding_window;
mod tag_info;
mod template_steps;

mod html;
mod xml;

pub use crate::coyote::attr;
pub use crate::coyote::attr_val;
pub use crate::coyote::list;
pub use crate::coyote::text;
pub use crate::coyote::tmpl;
pub use crate::coyote::unescaped_text;
pub use crate::coyote::vlist;
pub use crate::coyote::Component;

pub use crate::html::{ClientHtml, Html};
pub use crate::xml::Xml;

// pub use crate::html;
