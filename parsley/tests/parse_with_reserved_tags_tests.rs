use parsley::{parse_str, ParsleySieve, Results, Step, StepKind};

pub struct TestSieve {}

impl TestSieve {
    fn new() -> TestSieve {
        TestSieve {}
    }
}

impl ParsleySieve for TestSieve {
    fn alt_text(&self, tag: &str) -> bool {
        match tag {
            "script" => true,
            "style" => true,
            _ => false,
        }
    }
}

#[test]
fn confirm_clone_and_debug() {
    const template_str: &str = "<fox>{}</fox>";
    let sieve = TestSieve {};

    let steps = parse_str(&sieve, template_str, StepKind::Initial);
    let cloned = steps.clone();
    let debugged = format!("{:?}", cloned);
}

/** NODE TYPES **/
#[test]
fn parse_text() {
    const template_str: &str = "hai :3";
    let sieve = TestSieve {};

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
    const template_str: &str = "<style>.fox{color: auburn;}</style>";
    let sieve = TestSieve {};

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
            target: 6,
        },
        Step {
            kind: StepKind::ElementClosed,
            origin: 6,
            target: 7,
        },
        Step {
            kind: StepKind::Text,
            origin: 7,
            target: 26,
        },
        Step {
            kind: StepKind::Element,
            origin: 27,
            target: 28,
        },
        Step {
            kind: StepKind::TailElementSolidus,
            origin: 28,
            target: 29,
        },
        Step {
            kind: StepKind::TailTag,
            origin: 29,
            target: 34,
        },
        Step {
            kind: StepKind::TailElementClosed,
            origin: 34,
            target: 35,
        },
    ]);

    assert_eq!(steps, expected);
}

#[test]
fn parse_nested_reserved_tag() {
    const template_str: &str = "<fox><style>.fox{color: auburn;}</style></fox>";
    let sieve = TestSieve {};

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
            kind: StepKind::Text,
            origin: 12,
            target: 31,
        },
        Step {
            kind: StepKind::Element,
            origin: 32,
            target: 33,
        },
        Step {
            kind: StepKind::TailElementSolidus,
            origin: 33,
            target: 34,
        },
        Step {
            kind: StepKind::TailTag,
            origin: 34,
            target: 39,
        },
        Step {
            kind: StepKind::TailElementClosed,
            origin: 39,
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
    ]);

    assert_eq!(steps, expected);
}

#[test]
fn parse_multiple_sieve() {
    const template_str: &str =
        "<style>.fox{color: auburn;}</style><script>console.log('hai :3')</script>";
    let sieve = TestSieve {};

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
            target: 6,
        },
        Step {
            kind: StepKind::ElementClosed,
            origin: 6,
            target: 7,
        },
        Step {
            kind: StepKind::Text,
            origin: 7,
            target: 26,
        },
        Step {
            kind: StepKind::Element,
            origin: 27,
            target: 28,
        },
        Step {
            kind: StepKind::TailElementSolidus,
            origin: 28,
            target: 29,
        },
        Step {
            kind: StepKind::TailTag,
            origin: 29,
            target: 34,
        },
        Step {
            kind: StepKind::TailElementClosed,
            origin: 34,
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
            kind: StepKind::Text,
            origin: 43,
            target: 63,
        },
        Step {
            kind: StepKind::Element,
            origin: 64,
            target: 65,
        },
        Step {
            kind: StepKind::TailElementSolidus,
            origin: 65,
            target: 66,
        },
        Step {
            kind: StepKind::TailTag,
            origin: 66,
            target: 72,
        },
        Step {
            kind: StepKind::TailElementClosed,
            origin: 72,
            target: 73,
        },
    ]);

    assert_eq!(steps, expected);
}

#[test]
fn cannot_parse_nested_sieve() {
    const template_str: &str =
        "<script><style>.fox{color: auburn;}</style>console.log('hai :3')</script>";
    let sieve = TestSieve {};

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
            target: 7,
        },
        Step {
            kind: StepKind::ElementClosed,
            origin: 7,
            target: 8,
        },
        Step {
            kind: StepKind::Text,
            origin: 8,
            target: 63,
        },
        Step {
            kind: StepKind::Element,
            origin: 64,
            target: 65,
        },
        Step {
            kind: StepKind::TailElementSolidus,
            origin: 65,
            target: 66,
        },
        Step {
            kind: StepKind::TailTag,
            origin: 66,
            target: 72,
        },
        Step {
            kind: StepKind::TailElementClosed,
            origin: 72,
            target: 73,
        },
    ]);

    assert_eq!(steps, expected);
}
