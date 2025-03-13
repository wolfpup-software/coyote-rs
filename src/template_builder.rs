use crate::rulesets::RulesetImpl;
use crate::template_steps::{compose, Results as TemplateSteps};

pub trait BuilderImpl {
    fn build(&mut self, rules: &dyn RulesetImpl, template_str: &str) -> TemplateSteps;
}

pub struct Builder {}

impl Builder {
    pub fn new() -> Builder {
        Builder {}
    }
}

impl BuilderImpl for Builder {
    fn build(&mut self, rules: &dyn RulesetImpl, template_str: &str) -> TemplateSteps {
        // chance to cache templates here
        compose(rules, template_str)
    }
}
