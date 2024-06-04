// https://developer.mozilla.org/en-US/docs/Web/HTML/Element

use parsley::ParsleySieve;

pub trait SafetySieve {
    fn banned_path(&self, tag: &str) -> bool;
    fn no_descendants(&self, tag: &str) -> bool;
    // fn text_only(&self, tag: &str) -> bool;
}

pub struct HtmlServerSieve {}

impl ParsleySieve for HtmlServerSieve {
    fn text_only(&self, tag: &str) -> bool {
        match tag {
            "script" => true,
            "sylte" => true,
            _ => false,
        }
    }
}

impl SafetySieve for HtmlServerSieve {
    fn banned_path(&self, tag: &str) -> bool {
        false
    }
    fn no_descendants(&self, tag: &str) -> bool {
        false
    }
    // fn text_only(&self, tag: &str) -> bool {
    //     match tag {
    //         "style" => true,
    //         "script" => true,
    //         _ => false,
    //     }
    // }
}

pub struct TagInfo {
    pub namespace: String,
    pub tag: String,
    pub void_element_path: bool,
    pub preserved_text_path: bool,
    // banned path
    // no descendants path
    // text descendants only path
}

// tag needs to be aware of sieve
impl TagInfo {
    fn new(tag: &str) -> TagInfo {
        let mut namespace = "".to_string();
        if namespace_el(tag) {
            namespace = tag.to_string()
        }

        TagInfo {
            namespace: namespace,
            tag: tag.to_string(),
            void_element_path: void_el(tag),
            preserved_text_path: preserve_space_el(tag),
        }
    }

    fn from(prevTagInfo: &TagInfo, tag: &str) -> TagInfo {
        let mut namespace = prevTagInfo.namespace.clone();
        if namespace_el(tag) {
            namespace = tag.to_string();
        }

        let mut void_element_path = void_el(tag);
        if prevTagInfo.void_element_path {
            void_element_path = true;
        }

        let mut preserved_text_path = void_el(tag);
        if prevTagInfo.preserved_text_path {
            preserved_text_path = true;
        }

        TagInfo {
            namespace: namespace,
            tag: tag.to_string(),
            void_element_path: void_element_path,
            preserved_text_path: preserved_text_path,
            // banned path
            // no descendants path
            // text descendants only path
        }
    }
}

// struct ServerSieve {}
// impl SafetySieve for ServerSieve {
//     fn banned_element(tag: &str) -> bool {
//         return false;
//     }
//      no descendants
//      text descendants only
//      banned
//      ?? tabs? yes for readable no for filesize
// }

// struct ClientSieve {}
// impl SafetySieve for ClientSieve {
//     fn banned_element(tag: &str) -> bool {
//         match tag {
//             "script" => true,
//             "style" => true,
//             _ => false,
//         }
//     }
//     fn deprecated_element(tag: &str) -> bool {
//         deprecated_el(tag)
//     }
//     fn namespace_element(tag: &str) -> bool {
//         namespace_el(tag)
//     }
//     fn void_element(tag: &str) -> bool {
//         void_el(tag)
//     }
//     fn comment_element(tag: &str) -> bool {
//         comment_el(tag)
//     }
//     fn el_with_alt_text(tag: &str) -> bool {
//         match tag {
//             "script" => true,
//             "style" => true,
//             _ => false,
//         }
//     }
//     fn element_with_preserved_space(tag: &str) {
//         preserve_space_el(tag)
//     }
// }

// struct WebComponentSieve {}
// impl SafetySieve for WebComponentSieve {
//     fn banned_element(tag: &str) -> bool {
//         match tag {
//             "script" => true,
//             _ => false,
//         }
//     }
//     fn deprecated_element(tag: &str) -> bool {
//         deprecated_el(tag)
//     }
//     fn namespace_element(tag: &str) -> bool {
//         namespace_el(tag)
//     }
//     fn void_element(tag: &str) -> bool {
//         void_el(tag)
//     }
//     fn comment_element(tag: &str) -> bool {
//         comment_el(tag)
//     }
//     fn el_with_alt_text(tag: &str) -> bool {
//         el_with_alt_text(tag)
//     }
//     fn element_with_preserved_space(tag: &str) {
//         preserve_space_el(tag)
//     }
// }

fn el_with_alt_text(tag: &str) -> bool {
    match tag {
        "script" => true,
        "style" => true,
        _ => false,
    }
}

fn valid_el(tag: &str) -> bool {
    // len greater than 0
    // starts with alpha numberic
    // no spaces
    true
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

fn preserve_space_el(tag: &str) -> bool {
    match tag {
        "pre" => true,
        _ => false,
    }
}

fn block_el(tag: &str) -> bool {
    match tag {
        "!DOCTYPE" => true,
        "base" => true,
        "link" => true,
        "meta" => true,
        "noscript" => true,
        "script" => true,
        "style" => true,
        "title" => true,
        "header" => true,
        "footer" => true,
        "article" => true,
        "aside" => true,
        "nav" => true,
        "section" => true,
        "div" => true,
        "h1" => true,
        "h2" => true,
        "h3" => true,
        "h4" => true,
        "h5" => true,
        "h6" => true,
        "hgroup" => true,
        "p" => true,
        "form" => true,
        "fieldset" => true,
        "button" => true,
        "input" => true,
        "label" => true,
        "meter" => true,
        "object" => true,
        "output" => true,
        "progress" => true,
        "select" => true,
        "textarea" => true,
        "ul" => true,
        "ol" => true,
        "li" => true,
        "img" => true,
        "video" => true,
        "audio" => true,
        "template" => true,
        "iframe" => true,
        _ => false,
    }
}
