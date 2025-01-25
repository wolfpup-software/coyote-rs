use parse::{get_text_from_step, parse_str, Step, StepKind};

use rulesets::RulesetImpl;

/*
    INTERMEDIATE RENDER FORMAT

    Templates are converted to an array of content[] and injections[].

    Coyote is focused on text / strings
*/

// Could just keep the step kind.
// builds will involve the string
// caches will involve the string?
// 
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Results {
    pub strs: Vec<String>,
    pub injs: Vec<StepKind>,
}

impl Results {
    pub fn new() -> Results {
        Results {
            strs: Vec::from(["".to_string()]),
            injs: Vec::new(),
        }
    }
}

pub fn compose(ruleset: &dyn RulesetImpl, template_str: &str) -> Results {
    let mut results = Results::new();

    for step in parse_str(ruleset, template_str, StepKind::Initial) {
        match step.kind {
            StepKind::AttrMapInjection => push_attr_map_injection(&mut results),
            StepKind::DescendantInjection => push_descendant_injection(&mut results),
            StepKind::InjectionSpace => {}
            StepKind::InjectionConfirmed => {}
            _ => push_text(&mut results, template_str, step),
        }
    }

    results
}

fn push_text(results: &mut Results, template_str: &str, step: Step) {
    let text = get_text_from_step(template_str, &step);
    if let Some(last) = results.strs.last_mut() {
        last.push_str(text);
    }
}

fn push_attr_map_injection(results: &mut Results) {
    results.strs.push("".to_string());
    results.injs.push(StepKind::AttrMapInjection);
}

fn push_descendant_injection(results: &mut Results) {
    results.strs.push("".to_string());
    results.injs.push(StepKind::DescendantInjection);
}
