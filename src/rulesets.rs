pub trait RulesetImpl {
    fn get_initial_namespace(&self) -> &str;
    // parse
    fn tag_is_comment(&self, tag: &str) -> bool;
    fn get_close_sequence_from_alt_text_tag(&self, tag: &str) -> Option<&str>;
    fn get_alt_text_tag_from_close_sequence(&self, tag: &str) -> Option<&str>;
    // coyote
    fn respect_indentation(&self) -> bool;
    fn tag_is_banned_el(&self, tag: &str) -> bool;
    fn tag_is_void_el(&self, tag: &str) -> bool;
    fn tag_is_namespace_el(&self, tag: &str) -> bool;
    fn tag_is_preserved_text_el(&self, tag: &str) -> bool;
    fn tag_is_inline_el(&self, tag: &str) -> bool;
}

pub struct ServerRules {}

impl ServerRules {
    pub fn new() -> ServerRules {
        ServerRules {}
    }
}

impl RulesetImpl for ServerRules {
    fn get_initial_namespace(&self) -> &str {
        "html"
    }

    fn tag_is_comment(&self, tag: &str) -> bool {
        tag == "!--"
    }

    fn get_close_sequence_from_alt_text_tag(&self, tag: &str) -> Option<&str> {
        match tag {
            "!--" => Some("--"),
            "script" => Some("</script"),
            "style" => Some("</style"),
            _ => None,
        }
    }

    fn get_alt_text_tag_from_close_sequence(&self, tag: &str) -> Option<&str> {
        match tag {
            "--" => Some("!--"),
            "</script" => Some("script"),
            "</style" => Some("style"),
            _ => None,
        }
    }

    fn respect_indentation(&self) -> bool {
        true
    }

    fn tag_is_banned_el(&self, tag: &str) -> bool {
        is_banned_el(tag)
    }

    fn tag_is_void_el(&self, tag: &str) -> bool {
        is_void_el(tag)
    }

    fn tag_is_namespace_el(&self, tag: &str) -> bool {
        is_namespace_el(tag)
    }

    fn tag_is_preserved_text_el(&self, tag: &str) -> bool {
        is_preserved_text_el(tag)
    }

    fn tag_is_inline_el(&self, tag: &str) -> bool {
        is_inline_el(tag)
    }
}

pub struct ClientRules {}

impl ClientRules {
    pub fn new() -> ClientRules {
        ClientRules {}
    }
}

impl RulesetImpl for ClientRules {
    fn get_initial_namespace(&self) -> &str {
        "html"
    }

    fn tag_is_comment(&self, tag: &str) -> bool {
        tag == "!--"
    }

    fn get_close_sequence_from_alt_text_tag(&self, tag: &str) -> Option<&str> {
        match tag {
            "!--" => Some("--"),
            "script" => Some("</script"),
            "style" => Some("</style"),
            _ => None,
        }
    }

    fn get_alt_text_tag_from_close_sequence(&self, tag: &str) -> Option<&str> {
        match tag {
            "--" => Some("!--"),
            "</script" => Some("script"),
            "</style" => Some("style"),
            _ => None,
        }
    }

    fn respect_indentation(&self) -> bool {
        false
    }

    fn tag_is_banned_el(&self, tag: &str) -> bool {
        match tag {
            "!--" => true,
            "link" => true,
            "script" => true,
            "style" => true,
            _ => is_banned_el(tag),
        }
    }

    fn tag_is_void_el(&self, tag: &str) -> bool {
        is_void_el(tag)
    }

    fn tag_is_namespace_el(&self, tag: &str) -> bool {
        is_namespace_el(tag)
    }

    fn tag_is_preserved_text_el(&self, tag: &str) -> bool {
        is_preserved_text_el(tag)
    }

    fn tag_is_inline_el(&self, tag: &str) -> bool {
        // is it?
        match tag {
            "a" => true,
            _ => is_inline_el(tag),
        }
    }
}

pub struct XmlRules {}

impl XmlRules {
    pub fn new() -> XmlRules {
        XmlRules {}
    }
}

impl RulesetImpl for XmlRules {
    fn get_initial_namespace(&self) -> &str {
        "xml"
    }

    fn tag_is_comment(&self, tag: &str) -> bool {
        tag == "!--"
    }

    fn get_close_sequence_from_alt_text_tag(&self, tag: &str) -> Option<&str> {
        match tag {
            "!--" => Some("-->"),
            "![CDATA[" => Some("]]>"),
            _ => None,
        }
    }

    fn get_alt_text_tag_from_close_sequence(&self, tag: &str) -> Option<&str> {
        match tag {
            "-->" => Some("!--"),
            "]]>" => Some("!CDATA[["),
            _ => None,
        }
    }

    fn respect_indentation(&self) -> bool {
        true
    }

    fn tag_is_banned_el(&self, _tag: &str) -> bool {
        false
    }

    fn tag_is_void_el(&self, _tag: &str) -> bool {
        false
    }

    fn tag_is_namespace_el(&self, _tag: &str) -> bool {
        false
    }

    fn tag_is_preserved_text_el(&self, tag: &str) -> bool {
        "!CDATA[[" == tag
    }

    fn tag_is_inline_el(&self, _tag: &str) -> bool {
        false
    }
}

// deprecated elements
fn is_banned_el(tag: &str) -> bool {
    match tag {
        "acronym" => true,
        "big" => true,
        "center" => true,
        "content" => true,
        "dir" => true,
        "font" => true,
        "frame" => true,
        "framset" => true,
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

fn is_void_el(tag: &str) -> bool {
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

fn is_namespace_el(tag: &str) -> bool {
    match tag {
        "html" => true,
        "math" => true,
        "svg" => true,
        _ => false,
    }
}

pub fn is_preserved_text_el(tag: &str) -> bool {
    return "pre" == tag;
}

pub fn is_inline_el(tag: &str) -> bool {
    match tag {
        "abbr" => true,
        "area" => true,
        "audio" => true,
        "b" => true,
        "bdi" => true,
        "bdo" => true,
        "cite" => true,
        "code" => true,
        "data" => true,
        "dfn" => true,
        "em" => true,
        "embed" => true,
        "i" => true,
        "iframe" => true,
        "img" => true,
        "kbd" => true,
        "map" => true,
        "mark" => true,
        "object" => true,
        "picture" => true,
        "portal" => true,
        "q" => true,
        "rp" => true,
        "rt" => true,
        "ruby" => true,
        "s" => true,
        "samp" => true,
        "small" => true,
        "source" => true,
        "span" => true,
        "strong" => true,
        "sub" => true,
        "sup" => true,
        "time" => true,
        "track" => true,
        "u" => true,
        "var" => true,
        "video" => true,
        "wbr" => true,
        _ => false,
    }
}
