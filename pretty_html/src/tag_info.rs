// https://developer.mozilla.org/en-US/docs/Web/HTML/Element

use crate::sieves::SafetySieve;
use parsley::ParsleySieve;

pub struct TagInfo {
    pub namespace: String,
    pub tag: String,
    pub indent_count: usize,
    pub void_path: bool,
    pub preserved_text_path: bool,
    pub banned_path: bool,
}

// tag needs to be aware of sieve
impl TagInfo {
    fn new(&self, sieve: &(impl ParsleySieve + SafetySieve), tag: &str) -> TagInfo {
        let mut namespace = "".to_string();
        if namespace_el(tag) {
            namespace = tag.to_string()
        }

        TagInfo {
            namespace: namespace,
            tag: tag.to_string(),
            indent_count: 0,
            void_path: void_el(tag),
            preserved_text_path: preserve_space_el(tag),
            banned_path: sieve.banned(tag),
        }
    }

    fn from(
        &self,
        sieve: &(impl ParsleySieve + SafetySieve),
        prevTagInfo: &TagInfo,
        tag: &str,
    ) -> TagInfo {
        let mut namespace = prevTagInfo.namespace.clone();
        if namespace_el(tag) {
            namespace = tag.to_string();
        }

        let mut void_path = prevTagInfo.void_path;
        if void_el(tag) {
            void_path = true;
        }

        let mut preserved_text_path = prevTagInfo.preserved_text_path;
        if preserve_space_el(tag) {
            preserved_text_path = true;
        }

        let mut banned_path = prevTagInfo.banned_path;
        if sieve.banned(tag) {
            banned_path = true;
        }

        let mut indent_count = prevTagInfo.indent_count;
        // if tabable_el(tag) {
        //     indent_count += 1;
        // }

        TagInfo {
            namespace: namespace,
            tag: tag.to_string(),
            indent_count: indent_count, // find out if tabable do a +1 effort here
            void_path: void_path,
            preserved_text_path: preserved_text_path,
            banned_path: banned_path,
        }
    }
}

fn preserve_space_el(tag: &str) -> bool {
    return tag == "pre";
}

fn namespace_el(tag: &str) -> bool {
    match tag {
        "html" => true,
        "svg" => true,
        "math" => true,
        _ => false,
    }
}

fn void_el(tag: &str) -> bool {
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
