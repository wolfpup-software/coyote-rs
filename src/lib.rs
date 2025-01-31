mod component_string;
mod compose_steps;
mod components;
mod parse;
mod routes;
mod rulesets;
mod sliding_window;
mod tag_info;
mod template_steps;

pub mod html;
pub mod xml;

pub use crate::components::attr;
pub use crate::components::attr_val;
pub use crate::components::list;
pub use crate::components::text;
pub use crate::components::tmpl;
pub use crate::components::unescaped_text;
pub use crate::components::vlist;
pub use crate::components::Component;

// pub use crate::html;
