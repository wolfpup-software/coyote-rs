// own module

// just lists html saftey sieve options and return valuse

// https://developer.mozilla.org/en-US/docs/Web/HTML/Element

use txml::{DocBuilder, Template, TxmlBuilder};
// can have match selection
pub struct SettingsBuff {}
impl SafetySieve {
    fn deprecated_element() {}
    fn namesapce_custom_element() {}
    fn namespace_element() {}
    fn preserved_text_element() {}
    fn void_element() {}
    fn without_descendants() {}
}

struct ServerSieve {}
impl SafetySieve for ServerSieve {
    fn banned_element(tag: &str) -> bool {
        return false;
    }
    fn deprecated_element(tag: &str) -> bool {
        deprecated_el(tag)
    }
    fn namespace_element(tag: &str) -> bool {
        namespace_el(tag)
    }
    fn void_element(tag: &str) -> bool {
        void_el(tag)
    }
    fn comment_element(tag: &str) -> bool {
        comment_el(tag)
    }
    fn element_with_alt_text_only(tag: &str) -> bool {
        match tag {
            "script" => true,
            "style" => true,
            _ => false,
        }
    }
    fn element_with_preserved_space(tag: &str) {
        preserve_space_el(tag)
    }
}

struct ClientSieve {}
impl SafetySieve for ClientSieve {
    fn banned_element(tag: &str) -> bool {
        match tag {
            "script" => true,
            "style" => true,
            _ => false,
        }
    }
    fn deprecated_element(tag: &str) -> bool {
        deprecated_el(tag)
    }
    fn namespace_element(tag: &str) -> bool {
        namespace_el(tag)
    }
    fn void_element(tag: &str) -> bool {
        void_el(tag)
    }
    fn comment_element(tag: &str) -> bool {
        comment_el(tag)
    }
    fn element_with_alt_text_only(tag: &str) -> bool {
        match tag {
            "script" => true,
            "style" => true,
            _ => false,
        }
    }
    fn element_with_preserved_space(tag: &str) {
        preserve_space_el(tag)
    }
}

struct WebComponentSieve {}
impl SafetySieve for WebComponentSieve {
    fn banned_element(tag: &str) -> bool {
        match tag {
            "script" => true,
            _ => false,
        }
    }
    fn deprecated_element(tag: &str) -> bool {
        deprecated_el(tag)
    }
    fn namespace_element(tag: &str) -> bool {
        namespace_el(tag)
    }
    fn void_element(tag: &str) -> bool {
        void_el(tag)
    }
    fn comment_element(tag: &str) -> bool {
        comment_el(tag)
    }
    fn element_with_alt_text_only(tag: &str) -> bool {
        match tag {
            "script" => true,
            "style" => true,
            _ => false,
        }
    }
    fn element_with_preserved_space(tag: &str) {
        preserve_space_el(tag)
    }
}

fn valid_el(tag: &str) -> bool {
    // len greater than 0
    // starts with alpha numberic
    // no spaces
    if let Some(index) = tag.find(" ") {
        return false
    }
    // has a hyphen after the first character

    
    // custom element
    if let Some(index) = tag.find("-") {
        if index === 0 { return false }
    }
}

fn comment_el(tag: &str) -> bool {
    match tag {
        "!--" => true,
        _ => false,
    }
}

fn namespace_el(tag: &str) -> bool {
    match tag {
        "html" => true,
        "svg" => true,
        "math" => true,
        _ => false,
    }
}

fn deprecated_el(tag: &str) -> bool {
    match tag {
        "acronym" => true,
        "big" => true,
        "center" => true,
        "content" => true,
        "dir" => true,
        "font" => true,
        "frame" => true,
        "frameset" => true,
        "image" => true,
        "marquee" => true,
        "menuitem" => true,
        "nobr" => true,
        "noembed" => true,
        "noframes" => true,
        "param" => true,
        "plaintext" => true,
        "rb" => true,
        "rtc" => true,
        "shadow" => true,
        "strike" => true,
        "tt" => true,
        "xmp" => true,
        _ => false,
    }
}

fn void_el(tag: &str) -> bool {
    match tag {
        "!DOCTYPE" => true,
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

fn preserve_space_el(tag: &str) {
    match tag {
        "pre" => true,
        _ => false,
    }
}
