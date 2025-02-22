/*
    INTERMEDIATE RENDER FORMAT

    Templates are converted to an array of template steps[][] and and injection steps[].

    Coyote is focused on text / strings
*/

use crate::parse::{parse_str, Step};
use crate::routes::StepKind;
use crate::rulesets::RulesetImpl;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Results {
    pub steps: Vec<Vec<Step>>,
    pub injs: Vec<Step>,
}

impl Results {
    pub fn new() -> Results {
        Results {
            steps: Vec::from([Vec::new()]),
            injs: Vec::new(),
        }
    }
}

pub fn compose(ruleset: &dyn RulesetImpl, template_str: &str) -> Results {
    let mut results = Results::new();

    for step in parse_str(ruleset, template_str, StepKind::Initial) {
        match step.kind {
            StepKind::AttrMapInjection => push_injection(&mut results, step),
            StepKind::DescendantInjection => push_injection(&mut results, step),
            _ => push_step(&mut results, step),
        }
    }

    results
}

fn push_step(results: &mut Results, step: Step) {
    if let Some(last) = results.steps.last_mut() {
        last.push(step);
    }
}

fn push_injection(results: &mut Results, step: Step) {
    results.steps.push(Vec::new());
    results.injs.push(step);
}
