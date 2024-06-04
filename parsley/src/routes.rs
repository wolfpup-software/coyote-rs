use crate::StepKind;

// Names based roughly on:
// https://html.spec.whatwg.org/multipage/parsing.html

pub fn route(glyph: char, prev_kind: &StepKind) -> StepKind {
    match prev_kind {
        StepKind::Element => get_state_from_element(glyph),
        StepKind::TailElementSolidus => get_state_from_tail_element_slash(glyph),
        StepKind::Tag => get_state_from_tagname(glyph),
        StepKind::TailTag => get_state_from_close_tagname(glyph),
        StepKind::TailElementSpace => get_state_from_tail_element_space(glyph),
        StepKind::VoidElement => get_state_from_void_element(glyph),
        StepKind::ElementSpace => get_state_from_element_space(glyph),
        StepKind::Attr => get_state_from_attribute(glyph),
        StepKind::AttrSetter => get_state_from_attribute_setter(glyph),
        StepKind::AttrQuote => get_state_from_attribute_quote(glyph),
        StepKind::AttrValue => get_state_from_attribute_quote(glyph),
        StepKind::AttrValueUnquoted => get_state_from_attribute_value_unquoted(glyph),
        StepKind::AttrQuoteClosed => get_state_from_attribute_quote_closed(glyph),
        StepKind::AttrMapInjection => get_state_from_injection_found(glyph),
        StepKind::DescendantInjection => get_state_from_injection_found(glyph),
        StepKind::InjectionSpace => get_state_from_injection_found(glyph),
        _ => get_state_from_initial(glyph),
    }
}

fn get_state_from_initial(glyph: char) -> StepKind {
    match glyph {
        '<' => StepKind::Element,
        '{' => StepKind::DescendantInjection,
        _ => StepKind::Text,
    }
}

fn get_state_from_element(glyph: char) -> StepKind {
    if glyph.is_whitespace() {
        return StepKind::Element;
    }

    match glyph {
        '/' => StepKind::TailElementSolidus,
        '>' => StepKind::Fragment,
        _ => StepKind::Tag,
    }
}

fn get_state_from_tagname(glyph: char) -> StepKind {
    if glyph.is_whitespace() {
        return StepKind::ElementSpace;
    }

    match glyph {
        '>' => StepKind::ElementClosed,
        '/' => StepKind::VoidElement,
        _ => StepKind::Tag,
    }
}

fn get_state_from_tail_element_slash(glyph: char) -> StepKind {
    if glyph.is_whitespace() {
        return StepKind::TailElementSolidus;
    }

    match glyph {
        '>' => StepKind::FragmentClosed,
        _ => StepKind::TailTag,
    }
}

fn get_state_from_close_tagname(glyph: char) -> StepKind {
    if glyph.is_whitespace() {
        return StepKind::TailElementSpace;
    }

    match glyph {
        '>' => StepKind::TailElementClosed,
        _ => StepKind::TailTag,
    }
}

fn get_state_from_tail_element_space(glyph: char) -> StepKind {
    match glyph {
        '>' => StepKind::TailElementClosed,
        _ => StepKind::TailElementSpace,
    }
}

pub fn get_state_from_void_element(glyph: char) -> StepKind {
    match glyph {
        '>' => StepKind::VoidElementClosed,
        _ => StepKind::VoidElement,
    }
}

fn get_state_from_element_space(glyph: char) -> StepKind {
    if glyph.is_whitespace() {
        return StepKind::ElementSpace;
    }

    match glyph {
        '>' => StepKind::ElementClosed,
        '/' => StepKind::VoidElement,
        '{' => StepKind::AttrMapInjection,
        _ => StepKind::Attr,
    }
}

fn get_state_from_attribute(glyph: char) -> StepKind {
    if glyph.is_whitespace() {
        return StepKind::ElementSpace;
    }

    match glyph {
        '=' => StepKind::AttrSetter,
        '>' => StepKind::ElementClosed,
        '/' => StepKind::VoidElement,
        '{' => StepKind::AttrMapInjection,
        _ => StepKind::Attr,
    }
}

fn get_state_from_attribute_setter(glyph: char) -> StepKind {
    if glyph.is_whitespace() {
        return StepKind::AttrSetter;
    }

    match glyph {
        '"' => StepKind::AttrQuote,
        _ => StepKind::AttrValueUnquoted,
    }
}

fn get_state_from_attribute_value_unquoted(glyph: char) -> StepKind {
    if glyph.is_whitespace() {
        return StepKind::ElementSpace;
    }

    match glyph {
        '>' => StepKind::ElementClosed,
        _ => StepKind::AttrValueUnquoted,
    }
}

fn get_state_from_attribute_quote(glyph: char) -> StepKind {
    match glyph {
        '"' => StepKind::AttrQuoteClosed,
        _ => StepKind::AttrValue,
    }
}

fn get_state_from_attribute_quote_closed(glyph: char) -> StepKind {
    match glyph {
        '>' => StepKind::ElementClosed,
        '/' => StepKind::VoidElement,
        _ => StepKind::ElementSpace,
    }
}

fn get_state_from_injection_found(glyph: char) -> StepKind {
    match glyph {
        '}' => StepKind::InjectionConfirmed,
        _ => StepKind::InjectionSpace,
    }
}