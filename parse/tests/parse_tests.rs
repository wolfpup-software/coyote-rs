use parse::{parse_str, Results, Step, StepKind};

use rulesets::ServerRules;

/** DX **/
// this test will fail to build if `clone` or `default formatter` is not available
#[test]
fn confirm_clone_and_debug() {
    let rules = ServerRules::new();

    let template_str: &str = "<fox>{}</fox>";
    let steps = parse_str(&rules, template_str, StepKind::Initial);

    let cloned = steps.clone();
    let _debugged = format!("{:?}", cloned);
}

/** README EXAMPLE **/
#[test]
fn parse_readme_example() {
    let rules = ServerRules::new();

    let template_str: &str = "<fox>{}</fox>";
    let steps = parse_str(&rules, template_str, StepKind::Initial);
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
            kind: StepKind::DescendantInjection,
            origin: 5,
            target: 6,
        },
        Step {
            kind: StepKind::InjectionConfirmed,
            origin: 6,
            target: 7,
        },
        Step {
            kind: StepKind::Element,
            origin: 7,
            target: 8,
        },
        Step {
            kind: StepKind::TailElementSolidus,
            origin: 8,
            target: 9,
        },
        Step {
            kind: StepKind::TailTag,
            origin: 9,
            target: 12,
        },
        Step {
            kind: StepKind::TailElementClosed,
            origin: 12,
            target: 13,
        },
    ]);

    assert_eq!(steps, expected);
}

/** NODE TYPES **/
#[test]
fn parse_text() {
    let rules = ServerRules::new();

    let template_str: &str = "hai :3";
    let steps = parse_str(&rules, template_str, StepKind::Initial);
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

#[test]
fn parse_fragment() {
    let rules = ServerRules::new();

    let template_str: &str = "<>";
    let steps = parse_str(&rules, template_str, StepKind::Text);
    let expected: Results = Vec::from([
        Step {
            kind: StepKind::Text,
            origin: 0,
            target: 0,
        },
        Step {
            kind: StepKind::Element,
            origin: 0,
            target: 1,
        },
        Step {
            kind: StepKind::Fragment,
            origin: 1,
            target: 2,
        },
    ]);

    assert_eq!(steps, expected);
}

#[test]
fn parse_close_fragment() {
    let rules = ServerRules::new();

    let template_str: &str = "</>";
    let steps = parse_str(&rules, template_str, StepKind::FragmentClosed);
    let expected: Results = Vec::from([
        Step {
            kind: StepKind::FragmentClosed,
            origin: 0,
            target: 0,
        },
        Step {
            kind: StepKind::Element,
            origin: 0,
            target: 1,
        },
        Step {
            kind: StepKind::TailElementSolidus,
            origin: 1,
            target: 2,
        },
        Step {
            kind: StepKind::FragmentClosed,
            origin: 2,
            target: 3,
        },
    ]);

    assert_eq!(steps, expected);
}

#[test]
fn parse_node() {
    let rules = ServerRules::new();

    let template_str: &str = "<wolf>";
    let steps = parse_str(&rules, template_str, StepKind::TailElementClosed);
    let expected: Results = Vec::from([
        Step {
            kind: StepKind::TailElementClosed,
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
            target: 5,
        },
        Step {
            kind: StepKind::ElementClosed,
            origin: 5,
            target: 6,
        },
    ]);

    assert_eq!(steps, expected);
}

#[test]
fn parse_close_node() {
    let rules = ServerRules::new();

    let template_str: &str = "</wolf>";
    let steps = parse_str(&rules, template_str, StepKind::FragmentClosed);
    let expected: Results = Vec::from([
        Step {
            kind: StepKind::FragmentClosed,
            origin: 0,
            target: 0,
        },
        Step {
            kind: StepKind::Element,
            origin: 0,
            target: 1,
        },
        Step {
            kind: StepKind::TailElementSolidus,
            origin: 1,
            target: 2,
        },
        Step {
            kind: StepKind::TailTag,
            origin: 2,
            target: 6,
        },
        Step {
            kind: StepKind::TailElementClosed,
            origin: 6,
            target: 7,
        },
    ]);

    assert_eq!(steps, expected);
}

#[test]
fn parse_void_node() {
    let rules = ServerRules::new();

    let template_str: &str = "<wolf/>";
    let steps = parse_str(&rules, template_str, StepKind::TailElementClosed);
    let expected: Results = Vec::from([
        Step {
            kind: StepKind::TailElementClosed,
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
            target: 5,
        },
        Step {
            kind: StepKind::EmptyElement,
            origin: 5,
            target: 6,
        },
        Step {
            kind: StepKind::EmptyElementClosed,
            origin: 6,
            target: 7,
        },
    ]);

    assert_eq!(steps, expected);
}

#[test]
fn parse_all_nodes() {
    let rules = ServerRules::new();

    let template_str: &str = "
    	prarie
    	<>
		  	<hare />
		  	<fox>
		  		feel the wind in your fur
		  		feel the grass in your paws
		  	</fox>
    	</>
    	chase the sun
    ";
    let steps = parse_str(&rules, template_str, StepKind::Initial);
    let expected: Results = Vec::from([
        Step {
            kind: StepKind::Initial,
            origin: 0,
            target: 0,
        },
        Step {
            kind: StepKind::Text,
            origin: 0,
            target: 18,
        },
        Step {
            kind: StepKind::Element,
            origin: 18,
            target: 19,
        },
        Step {
            kind: StepKind::Fragment,
            origin: 19,
            target: 20,
        },
        Step {
            kind: StepKind::Text,
            origin: 20,
            target: 26,
        },
        Step {
            kind: StepKind::Element,
            origin: 26,
            target: 27,
        },
        Step {
            kind: StepKind::Tag,
            origin: 27,
            target: 31,
        },
        Step {
            kind: StepKind::ElementSpace,
            origin: 31,
            target: 32,
        },
        Step {
            kind: StepKind::EmptyElement,
            origin: 32,
            target: 33,
        },
        Step {
            kind: StepKind::EmptyElementClosed,
            origin: 33,
            target: 34,
        },
        Step {
            kind: StepKind::Text,
            origin: 34,
            target: 40,
        },
        Step {
            kind: StepKind::Element,
            origin: 40,
            target: 41,
        },
        Step {
            kind: StepKind::Tag,
            origin: 41,
            target: 44,
        },
        Step {
            kind: StepKind::ElementClosed,
            origin: 44,
            target: 45,
        },
        Step {
            kind: StepKind::Text,
            origin: 45,
            target: 117,
        },
        Step {
            kind: StepKind::Element,
            origin: 117,
            target: 118,
        },
        Step {
            kind: StepKind::TailElementSolidus,
            origin: 118,
            target: 119,
        },
        Step {
            kind: StepKind::TailTag,
            origin: 119,
            target: 122,
        },
        Step {
            kind: StepKind::TailElementClosed,
            origin: 122,
            target: 123,
        },
        Step {
            kind: StepKind::Text,
            origin: 123,
            target: 129,
        },
        Step {
            kind: StepKind::Element,
            origin: 129,
            target: 130,
        },
        Step {
            kind: StepKind::TailElementSolidus,
            origin: 130,
            target: 131,
        },
        Step {
            kind: StepKind::FragmentClosed,
            origin: 131,
            target: 132,
        },
        Step {
            kind: StepKind::Text,
            origin: 132,
            target: 156,
        },
    ]);

    assert_eq!(steps, expected);
}

/** ATTRIBUTES **/
#[test]
fn parse_attribute() {
    let rules = ServerRules::new();

    let template_str: &str = "<hello howdy>";
    let steps = parse_str(&rules, template_str, StepKind::Initial);
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
            kind: StepKind::ElementSpace,
            origin: 6,
            target: 7,
        },
        Step {
            kind: StepKind::Attr,
            origin: 7,
            target: 12,
        },
        Step {
            kind: StepKind::ElementClosed,
            origin: 12,
            target: 13,
        },
    ]);

    assert_eq!(steps, expected);
}

#[test]
fn parse_multiple_attributes() {
    let rules = ServerRules::new();

    let template_str: &str = "<hello howdy look up occasionally>";

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
            kind: StepKind::ElementSpace,
            origin: 6,
            target: 7,
        },
        Step {
            kind: StepKind::Attr,
            origin: 7,
            target: 12,
        },
        Step {
            kind: StepKind::ElementSpace,
            origin: 12,
            target: 13,
        },
        Step {
            kind: StepKind::Attr,
            origin: 13,
            target: 17,
        },
        Step {
            kind: StepKind::ElementSpace,
            origin: 17,
            target: 18,
        },
        Step {
            kind: StepKind::Attr,
            origin: 18,
            target: 20,
        },
        Step {
            kind: StepKind::ElementSpace,
            origin: 20,
            target: 21,
        },
        Step {
            kind: StepKind::Attr,
            origin: 21,
            target: 33,
        },
        Step {
            kind: StepKind::ElementClosed,
            origin: 33,
            target: 34,
        },
    ]);

    let steps = parse_str(&rules, template_str, StepKind::Initial);
    assert_eq!(steps, expected);
}

#[test]
fn parse_attribute_declaration() {
    let rules = ServerRules::new();

    let template_str: &str = "<hello red=\"blue\">";
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
            kind: StepKind::ElementSpace,
            origin: 6,
            target: 7,
        },
        Step {
            kind: StepKind::Attr,
            origin: 7,
            target: 10,
        },
        Step {
            kind: StepKind::AttrSetter,
            origin: 10,
            target: 11,
        },
        Step {
            kind: StepKind::AttrQuote,
            origin: 11,
            target: 12,
        },
        Step {
            kind: StepKind::AttrValue,
            origin: 12,
            target: 16,
        },
        Step {
            kind: StepKind::AttrQuoteClosed,
            origin: 16,
            target: 17,
        },
        Step {
            kind: StepKind::ElementClosed,
            origin: 17,
            target: 18,
        },
    ]);

    let steps = parse_str(&rules, template_str, StepKind::Initial);
    assert_eq!(steps, expected);
}

#[test]
fn parse_attribute_value_unquoted() {
    let rules = ServerRules::new();

    let template_str: &str = "<hello red=blue>";
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
            kind: StepKind::ElementSpace,
            origin: 6,
            target: 7,
        },
        Step {
            kind: StepKind::Attr,
            origin: 7,
            target: 10,
        },
        Step {
            kind: StepKind::AttrSetter,
            origin: 10,
            target: 11,
        },
        Step {
            kind: StepKind::AttrValueUnquoted,
            origin: 11,
            target: 15,
        },
        Step {
            kind: StepKind::ElementClosed,
            origin: 15,
            target: 16,
        },
    ]);

    let steps = parse_str(&rules, template_str, StepKind::Initial);
    assert_eq!(steps, expected);
}

#[test]
fn parse_multiple_attribute_value_unquoted() {
    let rules = ServerRules::new();

    let template_str: &str = "<hello red=blue hia=:3 herro=!!!>";
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
            kind: StepKind::ElementSpace,
            origin: 6,
            target: 7,
        },
        Step {
            kind: StepKind::Attr,
            origin: 7,
            target: 10,
        },
        Step {
            kind: StepKind::AttrSetter,
            origin: 10,
            target: 11,
        },
        Step {
            kind: StepKind::AttrValueUnquoted,
            origin: 11,
            target: 15,
        },
        Step {
            kind: StepKind::ElementSpace,
            origin: 15,
            target: 16,
        },
        Step {
            kind: StepKind::Attr,
            origin: 16,
            target: 19,
        },
        Step {
            kind: StepKind::AttrSetter,
            origin: 19,
            target: 20,
        },
        Step {
            kind: StepKind::AttrValueUnquoted,
            origin: 20,
            target: 22,
        },
        Step {
            kind: StepKind::ElementSpace,
            origin: 22,
            target: 23,
        },
        Step {
            kind: StepKind::Attr,
            origin: 23,
            target: 28,
        },
        Step {
            kind: StepKind::AttrSetter,
            origin: 28,
            target: 29,
        },
        Step {
            kind: StepKind::AttrValueUnquoted,
            origin: 29,
            target: 32,
        },
        Step {
            kind: StepKind::ElementClosed,
            origin: 32,
            target: 33,
        },
    ]);

    let steps = parse_str(&rules, template_str, StepKind::Initial);
    assert_eq!(steps, expected);
}

#[test]
fn parse_multiple_attribute_declaration() {
    let rules = ServerRules::new();

    let template_str: &str = "<hello red=\"blue\" orange=\"purple\" green=\"pink\">";
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
            kind: StepKind::ElementSpace,
            origin: 6,
            target: 7,
        },
        Step {
            kind: StepKind::Attr,
            origin: 7,
            target: 10,
        },
        Step {
            kind: StepKind::AttrSetter,
            origin: 10,
            target: 11,
        },
        Step {
            kind: StepKind::AttrQuote,
            origin: 11,
            target: 12,
        },
        Step {
            kind: StepKind::AttrValue,
            origin: 12,
            target: 16,
        },
        Step {
            kind: StepKind::AttrQuoteClosed,
            origin: 16,
            target: 17,
        },
        Step {
            kind: StepKind::ElementSpace,
            origin: 17,
            target: 18,
        },
        Step {
            kind: StepKind::Attr,
            origin: 18,
            target: 24,
        },
        Step {
            kind: StepKind::AttrSetter,
            origin: 24,
            target: 25,
        },
        Step {
            kind: StepKind::AttrQuote,
            origin: 25,
            target: 26,
        },
        Step {
            kind: StepKind::AttrValue,
            origin: 26,
            target: 32,
        },
        Step {
            kind: StepKind::AttrQuoteClosed,
            origin: 32,
            target: 33,
        },
        Step {
            kind: StepKind::ElementSpace,
            origin: 33,
            target: 34,
        },
        Step {
            kind: StepKind::Attr,
            origin: 34,
            target: 39,
        },
        Step {
            kind: StepKind::AttrSetter,
            origin: 39,
            target: 40,
        },
        Step {
            kind: StepKind::AttrQuote,
            origin: 40,
            target: 41,
        },
        Step {
            kind: StepKind::AttrValue,
            origin: 41,
            target: 45,
        },
        Step {
            kind: StepKind::AttrQuoteClosed,
            origin: 45,
            target: 46,
        },
        Step {
            kind: StepKind::ElementClosed,
            origin: 46,
            target: 47,
        },
    ]);

    let steps = parse_str(&rules, template_str, StepKind::Initial);
    assert_eq!(steps, expected);
}

#[test]
fn parse_all_declarations() {
    let rules = ServerRules::new();

    let template_str: &str = "<hello red=\"blue\" wolf orange=\"purple\" tiger crane>";
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
            kind: StepKind::ElementSpace,
            origin: 6,
            target: 7,
        },
        Step {
            kind: StepKind::Attr,
            origin: 7,
            target: 10,
        },
        Step {
            kind: StepKind::AttrSetter,
            origin: 10,
            target: 11,
        },
        Step {
            kind: StepKind::AttrQuote,
            origin: 11,
            target: 12,
        },
        Step {
            kind: StepKind::AttrValue,
            origin: 12,
            target: 16,
        },
        Step {
            kind: StepKind::AttrQuoteClosed,
            origin: 16,
            target: 17,
        },
        Step {
            kind: StepKind::ElementSpace,
            origin: 17,
            target: 18,
        },
        Step {
            kind: StepKind::Attr,
            origin: 18,
            target: 22,
        },
        Step {
            kind: StepKind::ElementSpace,
            origin: 22,
            target: 23,
        },
        Step {
            kind: StepKind::Attr,
            origin: 23,
            target: 29,
        },
        Step {
            kind: StepKind::AttrSetter,
            origin: 29,
            target: 30,
        },
        Step {
            kind: StepKind::AttrQuote,
            origin: 30,
            target: 31,
        },
        Step {
            kind: StepKind::AttrValue,
            origin: 31,
            target: 37,
        },
        Step {
            kind: StepKind::AttrQuoteClosed,
            origin: 37,
            target: 38,
        },
        Step {
            kind: StepKind::ElementSpace,
            origin: 38,
            target: 39,
        },
        Step {
            kind: StepKind::Attr,
            origin: 39,
            target: 44,
        },
        Step {
            kind: StepKind::ElementSpace,
            origin: 44,
            target: 45,
        },
        Step {
            kind: StepKind::Attr,
            origin: 45,
            target: 50,
        },
        Step {
            kind: StepKind::ElementClosed,
            origin: 50,
            target: 51,
        },
    ]);

    let steps = parse_str(&rules, template_str, StepKind::Initial);
    assert_eq!(steps, expected);
}

/** INJECTIONS **/
#[test]
fn parse_descendant_injection() {
    let rules = ServerRules::new();

    let template_str: &str = "<hello>{}</hello>";
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
            kind: StepKind::DescendantInjection,
            origin: 7,
            target: 8,
        },
        Step {
            kind: StepKind::InjectionConfirmed,
            origin: 8,
            target: 9,
        },
        Step {
            kind: StepKind::Element,
            origin: 9,
            target: 10,
        },
        Step {
            kind: StepKind::TailElementSolidus,
            origin: 10,
            target: 11,
        },
        Step {
            kind: StepKind::TailTag,
            origin: 11,
            target: 16,
        },
        Step {
            kind: StepKind::TailElementClosed,
            origin: 16,
            target: 17,
        },
    ]);

    let steps = parse_str(&rules, template_str, StepKind::Initial);
    assert_eq!(steps, expected);
}

#[test]
fn parse_multiple_descendant_injections() {
    let rules = ServerRules::new();

    let template_str: &str = "{}<hello>{}</hello>{}";
    let expected: Results = Vec::from([
        Step {
            kind: StepKind::Initial,
            origin: 0,
            target: 0,
        },
        Step {
            kind: StepKind::DescendantInjection,
            origin: 0,
            target: 1,
        },
        Step {
            kind: StepKind::InjectionConfirmed,
            origin: 1,
            target: 2,
        },
        Step {
            kind: StepKind::Element,
            origin: 2,
            target: 3,
        },
        Step {
            kind: StepKind::Tag,
            origin: 3,
            target: 8,
        },
        Step {
            kind: StepKind::ElementClosed,
            origin: 8,
            target: 9,
        },
        Step {
            kind: StepKind::DescendantInjection,
            origin: 9,
            target: 10,
        },
        Step {
            kind: StepKind::InjectionConfirmed,
            origin: 10,
            target: 11,
        },
        Step {
            kind: StepKind::Element,
            origin: 11,
            target: 12,
        },
        Step {
            kind: StepKind::TailElementSolidus,
            origin: 12,
            target: 13,
        },
        Step {
            kind: StepKind::TailTag,
            origin: 13,
            target: 18,
        },
        Step {
            kind: StepKind::TailElementClosed,
            origin: 18,
            target: 19,
        },
        Step {
            kind: StepKind::DescendantInjection,
            origin: 19,
            target: 20,
        },
        Step {
            kind: StepKind::InjectionConfirmed,
            origin: 20,
            target: 21,
        },
    ]);

    let steps = parse_str(&rules, template_str, StepKind::Initial);
    assert_eq!(steps, expected);
}

#[test]
fn parse_attribute_injection() {
    let rules = ServerRules::new();

    let template_str: &str = "<hello {}>";
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
            kind: StepKind::ElementSpace,
            origin: 6,
            target: 7,
        },
        Step {
            kind: StepKind::AttrMapInjection,
            origin: 7,
            target: 8,
        },
        Step {
            kind: StepKind::InjectionConfirmed,
            origin: 8,
            target: 9,
        },
        Step {
            kind: StepKind::ElementClosed,
            origin: 9,
            target: 10,
        },
    ]);

    let steps = parse_str(&rules, template_str, StepKind::Initial);
    assert_eq!(steps, expected);
}

#[test]
fn parse_multiple_attribute_injections() {
    let rules = ServerRules::new();

    let template_str: &str = "<hello {} {} world {}>";
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
            kind: StepKind::ElementSpace,
            origin: 6,
            target: 7,
        },
        Step {
            kind: StepKind::AttrMapInjection,
            origin: 7,
            target: 8,
        },
        Step {
            kind: StepKind::InjectionConfirmed,
            origin: 8,
            target: 9,
        },
        Step {
            kind: StepKind::ElementSpace,
            origin: 9,
            target: 10,
        },
        Step {
            kind: StepKind::AttrMapInjection,
            origin: 10,
            target: 11,
        },
        Step {
            kind: StepKind::InjectionConfirmed,
            origin: 11,
            target: 12,
        },
        Step {
            kind: StepKind::ElementSpace,
            origin: 12,
            target: 13,
        },
        Step {
            kind: StepKind::Attr,
            origin: 13,
            target: 18,
        },
        Step {
            kind: StepKind::ElementSpace,
            origin: 18,
            target: 19,
        },
        Step {
            kind: StepKind::AttrMapInjection,
            origin: 19,
            target: 20,
        },
        Step {
            kind: StepKind::InjectionConfirmed,
            origin: 20,
            target: 21,
        },
        Step {
            kind: StepKind::ElementClosed,
            origin: 21,
            target: 22,
        },
    ]);

    let steps = parse_str(&rules, template_str, StepKind::Initial);
    assert_eq!(steps, expected);
}

#[test]
fn parse_all_injections() {
    let rules = ServerRules::new();

    let template_str: &str = "
    	<hello world {} {} howdy>
    		what's good!
    		{}
    		how's it?
    		{}
			</hello>
    ";

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
        Step {
            kind: StepKind::Element,
            origin: 6,
            target: 7,
        },
        Step {
            kind: StepKind::Tag,
            origin: 7,
            target: 12,
        },
        Step {
            kind: StepKind::ElementSpace,
            origin: 12,
            target: 13,
        },
        Step {
            kind: StepKind::Attr,
            origin: 13,
            target: 18,
        },
        Step {
            kind: StepKind::ElementSpace,
            origin: 18,
            target: 19,
        },
        Step {
            kind: StepKind::AttrMapInjection,
            origin: 19,
            target: 20,
        },
        Step {
            kind: StepKind::InjectionConfirmed,
            origin: 20,
            target: 21,
        },
        Step {
            kind: StepKind::ElementSpace,
            origin: 21,
            target: 22,
        },
        Step {
            kind: StepKind::AttrMapInjection,
            origin: 22,
            target: 23,
        },
        Step {
            kind: StepKind::InjectionConfirmed,
            origin: 23,
            target: 24,
        },
        Step {
            kind: StepKind::ElementSpace,
            origin: 24,
            target: 25,
        },
        Step {
            kind: StepKind::Attr,
            origin: 25,
            target: 30,
        },
        Step {
            kind: StepKind::ElementClosed,
            origin: 30,
            target: 31,
        },
        Step {
            kind: StepKind::Text,
            origin: 31,
            target: 57,
        },
        Step {
            kind: StepKind::DescendantInjection,
            origin: 57,
            target: 58,
        },
        Step {
            kind: StepKind::InjectionConfirmed,
            origin: 58,
            target: 59,
        },
        Step {
            kind: StepKind::Text,
            origin: 59,
            target: 82,
        },
        Step {
            kind: StepKind::DescendantInjection,
            origin: 82,
            target: 83,
        },
        Step {
            kind: StepKind::InjectionConfirmed,
            origin: 83,
            target: 84,
        },
        Step {
            kind: StepKind::Text,
            origin: 84,
            target: 88,
        },
        Step {
            kind: StepKind::Element,
            origin: 88,
            target: 89,
        },
        Step {
            kind: StepKind::TailElementSolidus,
            origin: 89,
            target: 90,
        },
        Step {
            kind: StepKind::TailTag,
            origin: 90,
            target: 95,
        },
        Step {
            kind: StepKind::TailElementClosed,
            origin: 95,
            target: 96,
        },
        Step {
            kind: StepKind::Text,
            origin: 96,
            target: 101,
        },
    ]);

    let steps = parse_str(&rules, template_str, StepKind::Initial);
    assert_eq!(steps, expected);
}

/** RELIABLE CHAOS **/
#[test]
fn parse_a_mangled_mess() {
    let rules = ServerRules::new();

    let template_str: &str = "
			<			moon phase=				text_that_is_very_bad_and_does_not_belong
	\"waxing gibbous\"					/					><
clouds           big            opacity=\"0.9\"
>
";
    let steps = parse_str(&rules, template_str, StepKind::Initial);
    let expected: Results = Vec::from([
        Step {
            kind: StepKind::Initial,
            origin: 0,
            target: 0,
        },
        Step {
            kind: StepKind::Text,
            origin: 0,
            target: 4,
        },
        Step {
            kind: StepKind::Element,
            origin: 4,
            target: 8,
        },
        Step {
            kind: StepKind::Tag,
            origin: 8,
            target: 12,
        },
        Step {
            kind: StepKind::ElementSpace,
            origin: 12,
            target: 13,
        },
        Step {
            kind: StepKind::Attr,
            origin: 13,
            target: 18,
        },
        Step {
            kind: StepKind::AttrSetter,
            origin: 18,
            target: 23,
        },
        Step {
            kind: StepKind::AttrValueUnquoted,
            origin: 23,
            target: 64,
        },
        Step {
            kind: StepKind::ElementSpace,
            origin: 64,
            target: 66,
        },
        Step {
            kind: StepKind::Attr,
            origin: 66,
            target: 73,
        },
        Step {
            kind: StepKind::ElementSpace,
            origin: 73,
            target: 74,
        },
        Step {
            kind: StepKind::Attr,
            origin: 74,
            target: 82,
        },
        Step {
            kind: StepKind::ElementSpace,
            origin: 82,
            target: 87,
        },
        Step {
            kind: StepKind::EmptyElement,
            origin: 87,
            target: 93,
        },
        Step {
            kind: StepKind::EmptyElementClosed,
            origin: 93,
            target: 94,
        },
        Step {
            kind: StepKind::Element,
            origin: 94,
            target: 96,
        },
        Step {
            kind: StepKind::Tag,
            origin: 96,
            target: 102,
        },
        Step {
            kind: StepKind::ElementSpace,
            origin: 102,
            target: 113,
        },
        Step {
            kind: StepKind::Attr,
            origin: 113,
            target: 116,
        },
        Step {
            kind: StepKind::ElementSpace,
            origin: 116,
            target: 128,
        },
        Step {
            kind: StepKind::Attr,
            origin: 128,
            target: 135,
        },
        Step {
            kind: StepKind::AttrSetter,
            origin: 135,
            target: 136,
        },
        Step {
            kind: StepKind::AttrQuote,
            origin: 136,
            target: 137,
        },
        Step {
            kind: StepKind::AttrValue,
            origin: 137,
            target: 140,
        },
        Step {
            kind: StepKind::AttrQuoteClosed,
            origin: 140,
            target: 141,
        },
        Step {
            kind: StepKind::ElementSpace,
            origin: 141,
            target: 142,
        },
        Step {
            kind: StepKind::ElementClosed,
            origin: 142,
            target: 143,
        },
        Step {
            kind: StepKind::Text,
            origin: 143,
            target: 144,
        },
    ]);
    assert_eq!(steps, expected);
}
