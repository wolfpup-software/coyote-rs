use parse::{parse_str, Results, Step, StepKind};

use sieve::Sieve;

#[test]
fn confirm_clone_and_debug() {
    let template_str: &str = "<fox>{}</fox>";
    let sieve = Sieve::new();

    let steps = parse_str(&sieve, template_str, StepKind::Initial);
    let cloned = steps.clone();
    let _debugged = format!("{:?}", cloned);
}

/** NODE TYPES **/
#[test]
fn parse_text() {
    let template_str: &str = "hai :3";
    let sieve = Sieve::new();

    let steps = parse_str(&sieve, template_str, StepKind::Initial);
    let expected: Results = Vec::from([
        Step {
            kind: StepKind::Initial,
            origin: 0,
            target: 0,
        },
        Step {
            kind: StepKind::Text,
            origin: 0,
            target: 6,
        },
    ]);

    assert_eq!(steps, expected);
}

/** RESERVED TAGS **/
#[test]
fn parse_reserved_tag() {
    let template_str: &str = "<style>.fox{color: auburn;}</style>";
    let sieve = Sieve::new();

    let steps = parse_str(&sieve, template_str, StepKind::Initial);
    let expected = [
        Step {
            kind: StepKind::Initial,
            origin: 0,
            target: 0,
        },
        Step {
            kind: StepKind::Element,
            origin: 0,
            target: 1,
        },
        Step {
            kind: StepKind::Tag,
            origin: 1,
            target: 6,
        },
        Step {
            kind: StepKind::ElementClosed,
            origin: 6,
            target: 7,
        },
        Step {
            kind: StepKind::AltText,
            origin: 7,
            target: 27,
        },
        Step {
            kind: StepKind::AltTextCloseSequence,
            origin: 27,
            target: 35,
        },
    ];

    assert_eq!(steps, expected);
}

#[test]
fn parse_reserved_tag_comment() {
    let template_str: &str = "<!-- imma pup! bork! -->";
    let sieve = Sieve::new();

    let steps = parse_str(&sieve, template_str, StepKind::Initial);
    let expected: Results = Vec::from([
        Step {
            kind: StepKind::Initial,
            origin: 0,
            target: 0,
        },
        Step {
            kind: StepKind::Element,
            origin: 0,
            target: 1,
        },
        Step {
            kind: StepKind::Tag,
            origin: 1,
            target: 4,
        },
        Step {
            kind: StepKind::CommentText,
            origin: 4,
            target: 21,
        },
        Step {
            kind: StepKind::AltTextCloseSequence,
            origin: 21,
            target: 24,
        },
    ]);

    assert_eq!(steps, expected);
}

#[test]
fn parse_nested_reserved_tag() {
    let template_str: &str = "<fox><style>.fox{color: auburn;}</style></fox>";
    let sieve = Sieve::new();

    let steps = parse_str(&sieve, template_str, StepKind::Initial);

    let expected = [
        Step {
            kind: StepKind::Initial,
            origin: 0,
            target: 0,
        },
        Step {
            kind: StepKind::Element,
            origin: 0,
            target: 1,
        },
        Step {
            kind: StepKind::Tag,
            origin: 1,
            target: 4,
        },
        Step {
            kind: StepKind::ElementClosed,
            origin: 4,
            target: 5,
        },
        Step {
            kind: StepKind::Element,
            origin: 5,
            target: 6,
        },
        Step {
            kind: StepKind::Tag,
            origin: 6,
            target: 11,
        },
        Step {
            kind: StepKind::ElementClosed,
            origin: 11,
            target: 12,
        },
        Step {
            kind: StepKind::AltText,
            origin: 12,
            target: 32,
        },
        Step {
            kind: StepKind::AltTextCloseSequence,
            origin: 32,
            target: 40,
        },
        Step {
            kind: StepKind::Element,
            origin: 40,
            target: 41,
        },
        Step {
            kind: StepKind::TailElementSolidus,
            origin: 41,
            target: 42,
        },
        Step {
            kind: StepKind::TailTag,
            origin: 42,
            target: 45,
        },
        Step {
            kind: StepKind::TailElementClosed,
            origin: 45,
            target: 46,
        },
    ];

    assert_eq!(steps, expected);
}

#[test]
fn parse_multiple_sieve() {
    let template_str: &str =
        "<style>.fox{color: auburn;}</style><script>console.log('hai :3')</script>";
    let sieve = Sieve::new();

    let steps = parse_str(&sieve, template_str, StepKind::Initial);

    let expected = [
        Step {
            kind: StepKind::Initial,
            origin: 0,
            target: 0,
        },
        Step {
            kind: StepKind::Element,
            origin: 0,
            target: 1,
        },
        Step {
            kind: StepKind::Tag,
            origin: 1,
            target: 6,
        },
        Step {
            kind: StepKind::ElementClosed,
            origin: 6,
            target: 7,
        },
        Step {
            kind: StepKind::AltText,
            origin: 7,
            target: 27,
        },
        Step {
            kind: StepKind::AltTextCloseSequence,
            origin: 27,
            target: 35,
        },
        Step {
            kind: StepKind::Element,
            origin: 35,
            target: 36,
        },
        Step {
            kind: StepKind::Tag,
            origin: 36,
            target: 42,
        },
        Step {
            kind: StepKind::ElementClosed,
            origin: 42,
            target: 43,
        },
        Step {
            kind: StepKind::AltText,
            origin: 43,
            target: 64,
        },
        Step {
            kind: StepKind::AltTextCloseSequence,
            origin: 64,
            target: 73,
        },
    ];

    assert_eq!(steps, expected);
}

#[test]
fn cannot_parse_nested_sieve() {
    let template_str: &str =
        "<script><style>.fox{color: auburn;}</style>console.log('hai :3')</script>";
    let sieve = Sieve::new();

    let steps = parse_str(&sieve, template_str, StepKind::Initial);

    let expected = [
        Step {
            kind: StepKind::Initial,
            origin: 0,
            target: 0,
        },
        Step {
            kind: StepKind::Element,
            origin: 0,
            target: 1,
        },
        Step {
            kind: StepKind::Tag,
            origin: 1,
            target: 7,
        },
        Step {
            kind: StepKind::ElementClosed,
            origin: 7,
            target: 8,
        },
        Step {
            kind: StepKind::AltText,
            origin: 8,
            target: 64,
        },
        Step {
            kind: StepKind::AltTextCloseSequence,
            origin: 64,
            target: 73,
        },
    ];

    assert_eq!(steps, expected);
}
