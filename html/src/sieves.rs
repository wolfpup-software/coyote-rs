use coyote;
use parse;

pub trait SieveImpl: parse::SieveImpl + coyote::SieveImpl {}

pub struct Sieve {}

impl Sieve {
    pub fn new() -> Sieve {
        Sieve {}
    }
}

impl SieveImpl for Sieve {}

impl parse::SieveImpl for Sieve {
    fn is_comment(&self, tag: &str) -> bool {
        tag == "!--"
    }

    fn get_close_sequence_from_alt_text_tag(&self, tag: &str) -> Option<&str> {
        match tag {
            "script" => Some("</script>"),
            "style" => Some("</style>"),
            "!--" => Some("-->"),
            _ => None,
        }
    }

    fn get_tag_from_close_sequence(&self, tag: &str) -> Option<&str> {
        match tag {
            "</script>" => Some("script"),
            "</style>" => Some("style"),
            "-->" => Some("!--"),
            _ => None,
        }
    }
}

impl coyote::SieveImpl for Sieve {
    fn respect_indentation(&self) -> bool {
        true
    }
    fn banned_el(&self, _tag: &str) -> bool {
        false
    }
    fn void_el(&self, tag: &str) -> bool {
        is_void_el(tag)
    }
    fn namespace_el(&self, tag: &str) -> bool {
        is_namespace_el(tag)
    }
    fn preserved_text_el(&self, tag: &str) -> bool {
        is_preserved_text_el(tag)
    }
    fn inline_el(&self, tag: &str) -> bool {
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

impl parse::SieveImpl for ClientSieve {
    fn is_comment(&self, tag: &str) -> bool {
        tag == "!--"
    }

    fn get_close_sequence_from_alt_text_tag(&self, tag: &str) -> Option<&str> {
        match tag {
            "script" => Some("</script>"),
            "style" => Some("</style>"),
            "!--" => Some("-->"),
            _ => None,
        }
    }

    fn get_tag_from_close_sequence(&self, tag: &str) -> Option<&str> {
        match tag {
            "</script>" => Some("script"),
            "</style>" => Some("style"),
            "-->" => Some("!--"),
            _ => None,
        }
    }
}

impl coyote::SieveImpl for ClientSieve {
    fn respect_indentation(&self) -> bool {
        false
    }
    fn banned_el(&self, tag: &str) -> bool {
        match tag {
            "script" => true,
            "style" => true,
            _ => false,
        }
    }
    fn void_el(&self, tag: &str) -> bool {
        is_void_el(tag)
    }
    fn namespace_el(&self, tag: &str) -> bool {
        is_namespace_el(tag)
    }
    fn preserved_text_el(&self, tag: &str) -> bool {
        is_preserved_text_el(tag)
    }
    fn inline_el(&self, tag: &str) -> bool {
        if tag == "a" {
            return true;
        }

        is_inline_el(tag)
    }
}

fn is_void_el(tag: &str) -> bool {
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

fn is_namespace_el(tag: &str) -> bool {
    match tag {
        "html" => true,
        "svg" => true,
        "math" => true,
        _ => false,
    }
}

pub fn is_preserved_text_el(tag: &str) -> bool {
    return tag == "pre";
}

pub fn is_inline_el(tag: &str) -> bool {
    match tag {
        "abbr" => true,
        "b" => true,
        "bdi" => true,
        "bdo" => true,
        "cite" => true,
        "code" => true,
        "data" => true,
        "dfn" => true,
        "em" => true,
        "i" => true,
        "kbd" => true,
        "mark" => true,
        "q" => true,
        "rp" => true,
        "rt" => true,
        "ruby" => true,
        "s" => true,
        "samp" => true,
        "small" => true,
        "span" => true,
        "strong" => true,
        "sub" => true,
        "sup" => true,
        "time" => true,
        "u" => true,
        "var" => true,
        "wbr" => true,
        "area" => true,
        "audio" => true,
        "img" => true,
        "map" => true,
        "track" => true,
        "video" => true,
        "embed" => true,
        "iframe" => true,
        "object" => true,
        "picture" => true,
        "portal" => true,
        "source" => true,
        _ => false,
    }
}
