use crate::routes;
use crate::routes::StepKind;
use crate::rulesets::RulesetImpl;
use crate::sliding_window::SlidingWindow;

#[derive(Debug, Eq, Clone, PartialEq)]
pub struct Step {
    pub kind: StepKind,
    pub origin: usize,
    pub target: usize,
}

pub fn parse_str(rules: &dyn RulesetImpl, template_str: &str, intial_kind: StepKind) -> Vec<Step> {
    let mut steps = Vec::from([Step {
        kind: intial_kind.clone(),
        origin: 0,
        target: 0,
    }]);

    let mut tag: &str = "";
    let mut prev_inj_kind = intial_kind;
    let mut sliding_window: Option<SlidingWindow> = None;

    for (index, glyph) in template_str.char_indices() {
        // window-slide through reserved tag
        if let Some(ref mut slider) = sliding_window {
            if !slider.slide(glyph) {
                continue;
            }

            if let Err(_) = add_reserved_element_text(rules, &mut steps, tag, index) {
                return steps;
            };

            sliding_window = None;
            continue;
        }

        // route next step
        let end_step = match steps.last_mut() {
            Some(step) => step,
            _ => return steps,
        };
        let mut curr_kind = match end_step.kind {
            StepKind::InjectionConfirmed => routes::route(glyph, &prev_inj_kind),
            _ => routes::route(glyph, &end_step.kind),
        };
        if curr_kind == end_step.kind {
            continue;
        }

        if is_injection_kind(&curr_kind) {
            prev_inj_kind = end_step.kind.clone();
        }

        // record change
        end_step.target = index;

        if StepKind::Tag == end_step.kind {
            tag = get_text_from_step(template_str, &end_step);
        }

        // two edge cases for comments and alt text
        if rules.tag_is_comment(tag) {
            if let Some(close_seq) = rules.get_close_sequence_from_alt_text_tag(tag) {
                let mut slider = SlidingWindow::new(close_seq);
                slider.slide(glyph);
                sliding_window = Some(slider);
                curr_kind = StepKind::CommentText;
            };
        }

        if let (true, Some(close_seq)) = (
            StepKind::ElementClosed == end_step.kind,
            rules.get_close_sequence_from_alt_text_tag(tag),
        ) {
            let mut slider = SlidingWindow::new(close_seq);
            slider.slide(glyph);
            sliding_window = Some(slider);

            curr_kind = StepKind::AltText;
        }

        steps.push(Step {
            kind: curr_kind,
            origin: index,
            target: index,
        });
    }

    if let Some(step) = steps.last_mut() {
        step.target = template_str.len();
    }

    steps
}

pub fn get_text_from_step<'a>(template_str: &'a str, step: &Step) -> &'a str {
    &template_str[step.origin..step.target]
}

fn is_injection_kind(step_kind: &StepKind) -> bool {
    match step_kind {
        StepKind::AttrMapInjection => true,
        StepKind::DescendantInjection => true,
        _ => false,
    }
}

fn add_reserved_element_text(
    rules: &dyn RulesetImpl,
    steps: &mut Vec<Step>,
    tag: &str,
    index: usize,
) -> Result<(), ()> {
    let step = match steps.last_mut() {
        Some(step) => step,
        _ => return Err(()),
    };

    let closing_sequence = match rules.get_close_sequence_from_alt_text_tag(tag) {
        Some(sequence) => sequence,
        _ => return Ok(()),
    };

    step.target = index - (closing_sequence.len() - 1);
    steps.push(Step {
        kind: StepKind::AltTextCloseSequence,
        origin: index - (closing_sequence.len() - 1),
        target: index - (closing_sequence.len()),
    });

    Ok(())
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    use crate::components::{attr_val, list, text, tmpl, Component};
    use crate::routes::StepKind;
    use crate::rulesets::ServerRules;
    use crate::template_steps::{compose, Results};

    fn woof() -> Component {
        tmpl("<input type=submit value=\"yus -_-\">", [])
    }

    fn woof_woof() -> Component {
        let descendants = list([text("you're a boy kisser aren't you >:3"), woof()]);

        let attributes = list([attr_val("action", "/uwu"), attr_val("method", "post")]);

        tmpl("<form {}>{}</form>", [attributes, descendants])
    }

    #[test]
    fn test_parse_str() {
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
}
