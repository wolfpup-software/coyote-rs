#[derive(Debug, Eq, Clone, PartialEq)]
pub enum StepKind {
    AttrQuoteClosed,
    AttrQuote,
    AttrMapInjection,
    AttrSetter,
    AttrValue,
    AttrValueUnquoted,
    Attr,
    TailElementClosed,
    TailElementSolidus,
    TailElementSpace,
    TailTag,
    DescendantInjection,
    FragmentClosed,
    Fragment,
    EmptyElementClosed,
    EmptyElement,
    Initial,
    InjectionConfirmed,
    InjectionSpace,
    ElementClosed,
    ElementSpace,
    Element,
    Tag,
    Text,
    // needed for comments and scripts
    AltText,
}

// Names based roughly on:
// https://html.spec.whatwg.org/multipage/parsing.html

pub fn route(glyph: char, prev_kind: &StepKind) -> StepKind {
    match prev_kind {
        StepKind::Attr => get_kind_from_attribute(glyph),
        StepKind::AttrMapInjection => get_kind_from_injection(glyph),
        StepKind::AttrQuote => get_kind_from_attribute_quote(glyph),
        StepKind::AttrQuoteClosed => get_kind_from_attribute_quote_closed(glyph),
        StepKind::AttrSetter => get_kind_from_attribute_setter(glyph),
        StepKind::AttrValue => get_kind_from_attribute_quote(glyph),
        StepKind::AttrValueUnquoted => get_kind_from_attribute_value_unquoted(glyph),
        StepKind::DescendantInjection => get_kind_from_injection(glyph),
        StepKind::Element => get_kind_from_element(glyph),
        StepKind::ElementSpace => get_kind_from_element_space(glyph),
        StepKind::EmptyElement => get_kind_from_empty_element(glyph),
        StepKind::InjectionSpace => get_kind_from_injection(glyph),
        StepKind::Tag => get_kind_from_tag(glyph),
        StepKind::TailElementSolidus => get_kind_from_tail_element_solidus(glyph),
        StepKind::TailElementSpace => get_kind_from_tail_element_space(glyph),
        StepKind::TailTag => get_kind_from_tail_tag(glyph),
        _ => get_kind_from_initial(glyph),
    }
}

fn get_kind_from_attribute(glyph: char) -> StepKind {
    if glyph.is_whitespace() {
        return StepKind::ElementSpace;
    }

    match glyph {
        '=' => StepKind::AttrSetter,
        '>' => StepKind::ElementClosed,
        '/' => StepKind::EmptyElement,
        '{' => StepKind::AttrMapInjection,
        _ => StepKind::Attr,
    }
}

fn get_kind_from_injection(glyph: char) -> StepKind {
    match glyph {
        '}' => StepKind::InjectionConfirmed,
        _ => StepKind::InjectionSpace,
    }
}

fn get_kind_from_attribute_quote(glyph: char) -> StepKind {
    match glyph {
        '"' => StepKind::AttrQuoteClosed,
        _ => StepKind::AttrValue,
    }
}

fn get_kind_from_attribute_quote_closed(glyph: char) -> StepKind {
    match glyph {
        '>' => StepKind::ElementClosed,
        '/' => StepKind::EmptyElement,
        _ => StepKind::ElementSpace,
    }
}

fn get_kind_from_attribute_setter(glyph: char) -> StepKind {
    if glyph.is_whitespace() {
        return StepKind::AttrSetter;
    }

    match glyph {
        '"' => StepKind::AttrQuote,
        _ => StepKind::AttrValueUnquoted,
    }
}

fn get_kind_from_attribute_value_unquoted(glyph: char) -> StepKind {
    if glyph.is_whitespace() {
        return StepKind::ElementSpace;
    }

    match glyph {
        '>' => StepKind::ElementClosed,
        _ => StepKind::AttrValueUnquoted,
    }
}

fn get_kind_from_element(glyph: char) -> StepKind {
    if glyph.is_whitespace() {
        return StepKind::Element;
    }

    match glyph {
        '>' => StepKind::Fragment,
        '/' => StepKind::TailElementSolidus,
        _ => StepKind::Tag,
    }
}

fn get_kind_from_element_space(glyph: char) -> StepKind {
    if glyph.is_whitespace() {
        return StepKind::ElementSpace;
    }

    match glyph {
        '>' => StepKind::ElementClosed,
        '/' => StepKind::EmptyElement,
        '{' => StepKind::AttrMapInjection,
        _ => StepKind::Attr,
    }
}

fn get_kind_from_empty_element(glyph: char) -> StepKind {
    match glyph {
        '>' => StepKind::EmptyElementClosed,
        _ => StepKind::EmptyElement,
    }
}

fn get_kind_from_tag(glyph: char) -> StepKind {
    if glyph.is_whitespace() {
        return StepKind::ElementSpace;
    }

    match glyph {
        '>' => StepKind::ElementClosed,
        '/' => StepKind::EmptyElement,
        _ => StepKind::Tag,
    }
}

fn get_kind_from_tail_element_solidus(glyph: char) -> StepKind {
    if glyph.is_whitespace() {
        return StepKind::TailElementSolidus;
    }

    match glyph {
        '>' => StepKind::FragmentClosed,
        _ => StepKind::TailTag,
    }
}

fn get_kind_from_tail_tag(glyph: char) -> StepKind {
    if glyph.is_whitespace() {
        return StepKind::TailElementSpace;
    }

    match glyph {
        '>' => StepKind::TailElementClosed,
        _ => StepKind::TailTag,
    }
}

fn get_kind_from_tail_element_space(glyph: char) -> StepKind {
    match glyph {
        '>' => StepKind::TailElementClosed,
        _ => StepKind::TailElementSpace,
    }
}

fn get_kind_from_initial(glyph: char) -> StepKind {
    match glyph {
        '<' => StepKind::Element,
        '{' => StepKind::DescendantInjection,
        _ => StepKind::Text,
    }
}
