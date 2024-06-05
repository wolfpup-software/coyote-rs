use parsley::{Results, Step, StepKind};

mod routes;
mod sliding_window;

use sliding_window::SlidingWindow;

pub trait ParsleySieve {
    fn alt_text(&self, tag: &str) -> bool;
}

pub fn parse_str_with_reserved_tags(
    sieve: &impl ParsleySieve,
    template_str: &str,
    intial_kind: StepKind,
) -> Results {
    let mut steps = Vec::from([Step {
        kind: intial_kind.clone(),
        origin: 0,
        target: 0,
    }]);

    let mut tag_step: &str = "";
    let mut sliding_window: Option<SlidingWindow> = None;

    for (index, glyph) in template_str.char_indices() {
        // slide through reserved tag
        if let Some(ref mut slider) = sliding_window {
            if !slider.slide(glyph) {
                continue;
            }

            if let Err(_) = add_reserved_element_text(&mut steps, tag_step, index) {
                return steps;
            };

            sliding_window = None;
            continue;
        }

        // add steps
        let front_step = match steps.last_mut() {
            Some(step) => step,
            _ => return steps,
        };

        let mut curr_kind = routes::route(glyph, &front_step.kind);

        // continue if no changes
        if curr_kind == front_step.kind {
            continue;
        }

        front_step.target = index;
        if front_step.kind == StepKind::Tag {
            tag_step = get_text_from_step(template_str, &front_step);
        }

        // create sliding_window on reserved tags
        if front_step.kind == StepKind::ElementClosed && sieve.alt_text(tag_step) {
            let mut slider = SlidingWindow::new(tag_step);
            slider.slide(glyph);
            sliding_window = Some(slider);

            curr_kind = StepKind::Text;
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

fn add_reserved_element_text(
    steps: &mut Vec<Step>,
    tag_step: &str,
    index: usize,
) -> Result<(), ()> {
    let step = match steps.last_mut() {
        Some(step) => step,
        _ => return Err(()),
    };

    step.target = index - (tag_step.len() + 2);
    steps.push(Step {
        kind: StepKind::Element,
        origin: index - (tag_step.len() + 1),
        target: index - (tag_step.len()),
    });
    steps.push(Step {
        kind: StepKind::TailElementSolidus,
        origin: index - (tag_step.len()),
        target: index - tag_step.len() + 1,
    });
    steps.push(Step {
        kind: StepKind::TailTag,
        origin: index - tag_step.len() + 1,
        target: index + 1,
    });

    Ok(())
}
