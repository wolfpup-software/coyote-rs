// own module

// just lists html saftey sieve options and return valuse

pub enum ElementType {
    dangerous,
    no_descendants,
    void_element,
    element,
}

pub trait SafetySieve {
    // fn get_element_type(&self, tag: &str) -> ElementType;
    fn is_void_element(&self, tag: &str) -> bool;
    fn cannot_have_descendants(&self, tag: &str) -> bool;
    fn must_preserve_spacew(&self, tag: &str) -> bool;
}

fn is_html_element(tag: &str) -> bool {
    match tag {
        "html" => true,
        "!DOCTYPE" => true,
        "base" => true,
        "head" => true,
        "link" => true,
        "meta" => true,
        "style" => true,
        "title" => true,
        _ => false,
    }
}

fn is_html_void_element(tag: &str) -> bool {
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

fn is_element_without_descendants(tag: &str) -> bool {
    match tag {
        "!DOCTYPE" => true,
        "style" => true,
        "title" => true,
        _ => false,
    }
}

// https://developer.mozilla.org/en-US/docs/Web/API/Element#events
fn is_banned_attribute(tag: &str) -> bool {
    match tag {
        "onclick" => true,
        "onpointerdown" => true,
        _ => false,
    }
}
