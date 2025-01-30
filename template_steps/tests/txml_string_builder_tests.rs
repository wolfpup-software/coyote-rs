use coyote::{attr_val, list, text, tmpl, Component};
use rulesets::ServerRules;

use parse::{Step, StepKind};
use template_steps::{compose, Results};

// Test will not build if Function Components do not build

fn woof() -> Component {
    tmpl("<input type=submit value=\"yus -_-\">", [])
}

fn woof_woof() -> Component {
    let descendants = list([text("you're a boy kisser aren't you >:3"), woof()]);

    let attributes = list([attr_val("action", "/uwu"), attr_val("method", "post")]);

    tmpl("<form {}>{}</form>", [attributes, descendants])
}

#[test]
fn test_txml_builder() {
    let rules = ServerRules::new();

    let template = woof_woof();
    let expected = Results {
        steps: Vec::from([
            Vec::from([
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
                    target: 5,
                },
                Step {
                    kind: StepKind::ElementSpace,
                    origin: 5,
                    target: 6,
                },
            ]),
            Vec::from([
                Step {
                    kind: StepKind::InjectionConfirmed,
                    origin: 7,
                    target: 8,
                },
                Step {
                    kind: StepKind::ElementClosed,
                    origin: 8,
                    target: 9,
                },
            ]),
            Vec::from([
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
                    target: 17,
                },
                Step {
                    kind: StepKind::TailElementClosed,
                    origin: 17,
                    target: 18,
                },
            ]),
        ]),
        injs: Vec::from([
            Step {
                kind: StepKind::AttrMapInjection,
                origin: 6,
                target: 7,
            },
            Step {
                kind: StepKind::DescendantInjection,
                origin: 9,
                target: 10,
            },
        ]),
    };

    if let Component::Tmpl(tmpl) = template {
        let results = compose(&rules, &tmpl.template_str);
        assert_eq!(expected, results);
    }
}
