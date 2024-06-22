// https://developer.mozilla.org/en-US/docs/Web/HTML/Element

use crate::sieves::Sieve;

#[derive(Debug)]
pub struct TagInfo {
    pub namespace: String,
    pub tag: String,
    pub last_descendant_tag: String,
    pub has_text: bool,
    pub indent_count: usize,
    pub void_el: bool,
    pub preserved_text_el: bool,
    pub indented_el: bool,
    pub void_path: bool,
    pub preserved_text_path: bool,
    pub banned_path: bool,
}

// tag needs to be aware of sieve
impl TagInfo {
    pub fn new(sieve: &impl Sieve, tag: &str) -> TagInfo {
        let mut namespace = "html".to_string();
        if namespace_el(tag) {
            namespace = tag.to_string()
        }

        TagInfo {
            namespace: namespace,
            tag: tag.to_string(),
            last_descendant_tag: "".to_string(),
            has_text: false,
            indent_count: 0,
            void_el: sieve.void_el(tag),
            preserved_text_el: sieve.preserved_text_el(tag),
            indented_el: sieve.indented_el(tag),
            void_path: false,
            preserved_text_path: false,
            banned_path: sieve.banned(tag),
        }
    }

    pub fn from(sieve: &impl Sieve, prev_tag_info: &TagInfo, tag: &str) -> TagInfo {
        // clone, then update values, then return

        let mut namespace = prev_tag_info.namespace.clone();
        if namespace_el(tag) {
            namespace = tag.to_string();
        }

        let mut void_path = prev_tag_info.void_path;
        if sieve.void_el(&prev_tag_info.tag) {
            void_path = true;
        }

        // preserved text happends _after_ tag, so only if prev tag is "pre"
        let mut preserved_text_path = prev_tag_info.preserved_text_path;
        if sieve.preserved_text_el(&prev_tag_info.tag) {
            preserved_text_path = true;
        }

        // immediately ban elements
        let mut banned_path = prev_tag_info.banned_path;
        if sieve.banned(tag) {
            banned_path = true;
        }

        let void_el = sieve.void_el(tag);
        let mut indent_count = prev_tag_info.indent_count;
        if void_el || indented_el(tag) {
            indent_count += 1;
        }

        TagInfo {
            namespace: namespace,
            tag: tag.to_string(),
            last_descendant_tag: "".to_string(),
            has_text: false,
            indent_count: indent_count,
            void_el: void_el,
            preserved_text_el: sieve.preserved_text_el(tag),
            indented_el: sieve.indented_el(tag),
            void_path: void_path,
            preserved_text_path: preserved_text_path,
            banned_path: banned_path,
        }
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

pub fn preserve_space_el(tag: &str) -> bool {
    return tag == "pre";
}

pub fn indented_el(tag: &str) -> bool {
    match tag {
        "a" => false,
        "abbr" => false,
        "b" => false,
        "bdi" => false,
        "bdo" => false,
        "cite" => false,
        "code" => false,
        "data" => false,
        "dfn" => false,
        "em" => false,
        "i" => false,
        "kbd" => false,
        "mark" => false,
        "q" => false,
        "rp" => false,
        "rt" => false,
        "ruby" => false,
        "s" => false,
        "samp" => false,
        "small" => false,
        "span" => false,
        "strong" => false,
        "sub" => false,
        "sup" => false,
        "time" => false,
        "u" => false,
        "var" => false,
        "wbr" => false,
        "area" => false,
        "audio" => false,
        "img" => false,
        "map" => false,
        "track" => false,
        "video" => false,
        "embed" => false,
        "iframe" => false,
        "object" => false,
        "picture" => false,
        "portal" => false,
        "source" => false,
        _ => true,
    }
}
