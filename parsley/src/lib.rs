mod routes;

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
    VoidElementClosed,
    VoidElement,
    Initial,
    InjectionConfirmed,
    InjectionSpace,
    ElementClosed,
    ElementSpace,
    Element,
    Tag,
    Text,
}

#[derive(Debug, Eq, Clone, PartialEq)]
pub struct Step {
    pub kind: StepKind,
    pub origin: usize,
    pub target: usize,
}

pub type Results = Vec<Step>;

pub fn get_text_from_step<'a>(template_str: &'a str, step: &Step) -> &'a str {
    &template_str[step.origin..step.target]
}

pub fn parse_str(template_str: &str, intial_kind: StepKind) -> Results {
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

fn is_injection_kind<'a>(step_kind: &'a StepKind) -> bool {
    match step_kind {
        StepKind::AttrMapInjection => true,
        StepKind::DescendantInjection => true,
        _ => false,
    }
}

/*
Common enough to have elements with specialized text.
In html. script style tags
*/

struct SlidingWindow {
    target: Vec<char>,
    index: usize,
}

impl SlidingWindow {
    fn new(target_str: &str) -> SlidingWindow {
        let mut glyphs = Vec::from(['<', '/']);
        glyphs.append(&mut target_str.chars().collect());
        SlidingWindow {
            target: glyphs,
            index: 1,
        }
    }

    fn slide(&mut self, glyph: char) -> bool {
        if self.index > self.target.len() {
            self.index = 0;
        }

        if self.target[self.index - 1] != glyph {
            self.index = 0;
        }

        self.index += 1;
        self.index > self.target.len()
    }
}

fn is_reserved_tag(reserved_tags: &Vec<String>, tag: &str) -> bool {
    for word in reserved_tags {
        if tag == word {
            return true;
        }
    }
    false
}

pub fn parse_str_with_reserved_tags(
    template_str: &str,
    reserved_tags: &Vec<String>,
    intial_kind: StepKind,
) -> Results {
    let mut steps = Vec::from([Step {
        kind: intial_kind.clone(),
        origin: 0,
        target: 0,
    }]);

    let mut prev_inj_kind = intial_kind;
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

        let mut curr_kind = match front_step.kind {
            StepKind::InjectionConfirmed => routes::route(glyph, &prev_inj_kind),
            _ => routes::route(glyph, &front_step.kind),
        };

        if is_injection_kind(&curr_kind) {
            prev_inj_kind = front_step.kind.clone();
        }

        // continue if no changes
        if curr_kind == front_step.kind {
            continue;
        }

        front_step.target = index;
        if front_step.kind == StepKind::Tag {
            tag_step = get_text_from_step(template_str, &front_step);
        }

        // create sliding_window on reserved tags
        if front_step.kind == StepKind::ElementClosed && is_reserved_tag(reserved_tags, tag_step) {
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

pub fn add_reserved_element_text(
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