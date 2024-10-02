mod routes;
mod sliding_window;

use sliding_window::SlidingWindow;

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
    AltTextCloseSequence, // edge case
    CommentText,          // edge case
}

pub trait SieveImpl {
    fn alt_text(&self, tag: &str) -> bool;
    fn get_close_sequence_from_alt_text_tag(&self, tag: &str) -> Option<&str>;
}

#[derive(Debug, Eq, Clone, PartialEq)]
pub struct Step {
    pub kind: StepKind,
    pub origin: usize,
    pub target: usize,
}

pub type Results = Vec<Step>;

pub fn parse_template_str(template_str: &str, intial_kind: StepKind) -> Results {
    let mut steps = Vec::from([Step {
        kind: intial_kind.clone(),
        origin: 0,
        target: 0,
    }]);

    let mut prev_inj_kind = intial_kind;

    for (index, glyph) in template_str.char_indices() {
        let front_step = match steps.last_mut() {
            Some(step) => step,
            _ => return steps,
        };

        let curr_kind = match front_step.kind {
            StepKind::InjectionConfirmed => routes::route(glyph, &prev_inj_kind),
            _ => routes::route(glyph, &front_step.kind),
        };

        if is_injection_kind(&curr_kind) {
            prev_inj_kind = front_step.kind.clone();
        }

        if curr_kind == front_step.kind {
            continue;
        }

        front_step.target = index;
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

pub fn parse_str(sieve: &impl SieveImpl, template_str: &str, intial_kind: StepKind) -> Results {
    let mut steps = Vec::from([Step {
        kind: intial_kind.clone(),
        origin: 0,
        target: 0,
    }]);

    let mut tag: &str = "";
    let mut sliding_window: Option<SlidingWindow> = None;

    for (index, glyph) in template_str.char_indices() {
        // slide through reserved tag
        if let Some(ref mut slider) = sliding_window {
            if !slider.slide(glyph) {
                continue;
            }

            if let Err(_) = add_reserved_element_text(sieve, &mut steps, tag, index) {
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

        // if tag is comment
        let mut curr_kind = routes::route(glyph, &front_step.kind);
        if tag == "!--" {
            curr_kind = StepKind::CommentText
        }
        if is_injection_kind(&curr_kind) {
            continue;
        }
        if curr_kind == front_step.kind {
            continue;
        }

        front_step.target = index;
        if front_step.kind == StepKind::Tag {
            tag = get_text_from_step(template_str, &front_step);
        }

        // two edge cases for comments
        if front_step.kind == StepKind::Tag && tag == "!--" {
            if let Some(close_seq) = sieve.get_close_sequence_from_alt_text_tag(tag) {
                let mut slider = SlidingWindow::new(close_seq);
                slider.slide(glyph);
                sliding_window = Some(slider);

                curr_kind = StepKind::Text;
            };
        }

        if let (true, Some(close_seq)) = (
            front_step.kind == StepKind::ElementClosed,
            sieve.get_close_sequence_from_alt_text_tag(tag),
        ) {
            let mut slider = SlidingWindow::new(close_seq);
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

fn is_injection_kind(step_kind: &StepKind) -> bool {
    match step_kind {
        StepKind::AttrMapInjection => true,
        StepKind::DescendantInjection => true,
        _ => false,
    }
}

fn add_reserved_element_text(
    sieve: &impl SieveImpl,
    steps: &mut Vec<Step>,
    tag: &str,
    index: usize,
) -> Result<(), ()> {
    let step = match steps.last_mut() {
        Some(step) => step,
        _ => return Err(()),
    };

    let closing_sequence = match sieve.get_close_sequence_from_alt_text_tag(tag) {
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
