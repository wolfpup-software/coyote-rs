// https://developer.mozilla.org/en-US/docs/Web/HTML/Element

use rulesets::RulesetImpl;

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum DescendantStatus {
    Text,
    Element,
    ElementClosed,
    InlineElement,
    InlineElementClosed,
    Initial,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct TagInfo {
    pub namespace: String,
    pub tag: String,
    pub most_recent_descendant: DescendantStatus,
    pub indent_count: usize,
    pub void_el: bool,
    pub inline_el: bool,
    pub preserved_text_path: bool,
    pub banned_path: bool,
}

impl TagInfo {
    pub fn new(sieve: &dyn RulesetImpl, tag: &str) -> TagInfo {
        let mut namespace = "html".to_string();
        if sieve.tag_is_namespace_el(tag) {
            namespace = tag.to_string()
        }

        TagInfo {
            namespace: namespace,
            tag: tag.to_string(),
            most_recent_descendant: DescendantStatus::Initial,
            indent_count: 0,
            void_el: sieve.tag_is_void_el(tag),
            inline_el: sieve.tag_is_inline_el(tag),
            preserved_text_path: sieve.tag_is_preserved_text_el(tag),
            banned_path: sieve.tag_is_banned_el(tag),
        }
    }

    pub fn from(sieve: &dyn RulesetImpl, prev_tag_info: &TagInfo, tag: &str) -> TagInfo {
        // clone, then update values, then return
        let mut tag_info = prev_tag_info.clone();

        if sieve.tag_is_namespace_el(tag) {
            tag_info.namespace = tag.to_string();
        }

        if sieve.tag_is_preserved_text_el(&prev_tag_info.tag) {
            tag_info.preserved_text_path = true;
        }

        if sieve.tag_is_banned_el(tag) {
            tag_info.banned_path = true;
        }

        if !sieve.tag_is_void_el(&prev_tag_info.tag) && !sieve.tag_is_inline_el(tag) {
            tag_info.indent_count += 1;
        }

        tag_info.void_el = sieve.tag_is_void_el(&tag);
        tag_info.tag = tag.to_string();
        tag_info.most_recent_descendant = DescendantStatus::Initial;
        tag_info.inline_el = sieve.tag_is_inline_el(tag);

        tag_info
    }
}
