pub trait ParseSieveImpl {
    fn is_comment(&self, tag: &str) -> bool;
    fn get_close_sequence_from_alt_text_tag(&self, tag: &str) -> Option<&str>;
    fn get_tag_from_close_sequence(&self, tag: &str) -> Option<&str>;
}

pub trait CoyoteSieveImpl {
    fn respect_indentation(&self) -> bool;
    fn is_banned_el(&self, tag: &str) -> bool;
    fn is_void_el(&self, tag: &str) -> bool;
    fn is_namespace_el(&self, tag: &str) -> bool;
    fn is_preserved_text_el(&self, tag: &str) -> bool;
    fn is_inline_el(&self, tag: &str) -> bool;
}

pub trait SieveImpl {
    // parse
    fn is_comment(&self, tag: &str) -> bool;
    fn get_close_sequence_from_alt_text_tag(&self, tag: &str) -> Option<&str>;
    fn get_tag_from_close_sequence(&self, tag: &str) -> Option<&str>;
    // coyote
    fn respect_indentation(&self) -> bool;
    fn is_banned_el(&self, tag: &str) -> bool;
    fn is_void_el(&self, tag: &str) -> bool;
    fn is_namespace_el(&self, tag: &str) -> bool;
    fn is_preserved_text_el(&self, tag: &str) -> bool;
    fn is_inline_el(&self, tag: &str) -> bool;
}

pub struct Sieve {}

impl Sieve {
    pub fn new() -> Sieve {
        Sieve {}
    }
}

impl SieveImpl for Sieve {}

impl ParseSieveImpl for Sieve {
    fn is_comment(&self, tag: &str) -> bool {
        tag == "!--"
    }

    fn get_close_sequence_from_alt_text_tag(&self, tag: &str) -> Option<&str> {
        get_close_sequence_from_alt_text_tag(tag);
    }

    fn get_tag_from_close_sequence(&self, tag: &str) -> Option<&str> {
        get_tag_from_close_sequence(tag);
    }

    fn respect_indentation(&self) -> bool {
        true
    }

    fn is_banned_el(&self, _tag: &str) -> bool {
        false
    }

    fn is_void_el(&self, tag: &str) -> bool {
        is_void_el(tag)
    }

    fn is_namespace_el(&self, tag: &str) -> bool {
        is_namespace_el(tag)
    }

    fn is_preserved_text_el(&self, tag: &str) -> bool {
        is_preserved_text_el(tag)
    }

    fn is_inline_el(&self, tag: &str) -> bool {
        is_inline_el(tag)
    }
}

pub struct ClientSieve {}

impl ClientSieve {
    pub fn new() -> ClientSieve {
        ClientSieve {}
    }
}

impl SieveImpl for ClientSieve {}

impl ParseSieveImpl for ClientSieve {
    fn is_comment(&self, tag: &str) -> bool {
        tag == "!--"
    }

    fn get_close_sequence_from_alt_text_tag(&self, tag: &str) -> Option<&str> {
        get_close_sequence_from_alt_text_tag(tag);
    }

    fn get_tag_from_close_sequence(&self, tag: &str) -> Option<&str> {
        get_tag_from_close_sequence(tag);
    }

    fn respect_indentation(&self) -> bool {
        false
    }

    fn is_banned_el(&self, tag: &str) -> bool {
        match tag {
            "!--" => true,
            "script" => true,
            "style" => true,
            _ => false,
        }
    }

    fn is_void_el(&self, tag: &str) -> bool {
        is_void_el(tag)
    }

    fn is_namespace_el(&self, tag: &str) -> bool {
        is_namespace_el(tag)
    }

    fn is_preserved_text_el(&self, tag: &str) -> bool {
        is_preserved_text_el(tag)
    }

    fn is_inline_el(&self, tag: &str) -> bool {
        // is it?
        if tag == "a" {
            return true;
        }

        is_inline_el(tag)
    }
}

fn get_close_sequence_from_alt_text_tag(tag: &str) -> Option<&str> {
    match tag {
        "!--" => Some("-->"),
        "script" => Some("</script>"),
        "style" => Some("</style>"),
        _ => None,
    }
}

fn get_tag_from_close_sequence(tag: &str) -> Option<&str> {
    match tag {
        "-->" => Some("!--"),
        "</script>" => Some("script"),
        "</style>" => Some("style"),
        _ => None,
    }
}

fn is_void_el(tag: &str) -> bool {
    match tag {
        "!--" => true,
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
    return tag == "pre";
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
