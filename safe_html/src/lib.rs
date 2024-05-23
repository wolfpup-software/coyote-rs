// own module

// just lists html saftey sieve options and return valuse

// https://developer.mozilla.org/en-US/docs/Web/HTML/Element

use txml::{DocBuilder, Template, TxmlBuilder};
// can have match selection
pub struct SettingsBuff {}
impl SettingsBuff {
    fn void_element() {}
    fn banned_element() {}
    fn preserved_text_element() {}
    fn without_descendants() {}
}
pub enum ElementType {
    dangerous,
    no_descendants,
    void_element,
    element,
}

struct HtmlSieve {}

impl SafetySieve for HtmlSieve {
    fn is_void_element(tag: &str) -> bool {
        match tag {
            "!DOCTYPE" => true,
            "!--" => true,
            "area" => true,
            "base" => true,
            "br" => true,
            "col" => true,
            "embed" => true,
            "hr" => true,
            "img" => true,
            "input" => true,
            "link" => true,
            "meta" => true,
            "param" => true,
            "source" => true,
            "track" => true,
            "wbr" => true,
            _ => false,
        }
    }

    fn descendants_allowed() {
        match tag {
            "script" => false,
            "style" => false,
            _ => true,
        }
    }

    fn descendants_text_allowed() {
        match tag {
            "script" => false,
            "style" => false,
            _ => true,
        }
    }

    fn must_preserve_space(tag: &str) {
        match tag {
            "pre" => true,
            _ => false,
        }
    }
}
