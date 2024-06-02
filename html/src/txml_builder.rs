use parsley::{get_text_from_step, parse_str, Step, StepKind};
use txml::Template;

// // use these injection details
// // for static html, we at least need to know the last tag element

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct HtmlBuilderResults {
    pub strs: Vec<String>,
    pub injs: Vec<StepKind>,
}

impl HtmlBuilderResults {
    pub fn new() -> HtmlBuilderResults {
        HtmlBuilderResults {
            strs: Vec::from(["".to_string()]),
            injs: Vec::new(),
        }
    }
}

pub struct HtmlBuilder {
    results: HtmlBuilderResults,
}

impl HtmlBuilder {
    pub fn new() -> HtmlBuilder {
        HtmlBuilder {
            results: HtmlBuilderResults::new(),
        }
    }

    pub fn build(&mut self, template: Template) -> HtmlBuilderResults {
        let mut results = HtmlBuilderResults::new();

        for step in parse_str(&template.template_str, StepKind::Initial) {
            push_step(&mut results, &template.template_str, step);
        }

        results
    }
}

fn push_step(results: &mut HtmlBuilderResults, template_str: &str, step: Step) {
    match step.kind {
        // // steps
        // StepKind::Tag => {
        //     push_element(self, get_text_from_step(template_str, &step));
        // }
        // StepKind::ElementClosed => {
        //     close_element(self, get_text_from_step(template_str, &step));
        // }
        // StepKind::VoidElementClosed => {
        //     close_void_element(self, get_text_from_step(template_str, &step));
        // }
        // StepKind::Attr => {
        //     add_attr(self, get_text_from_step(template_str, &step));
        // }
        // StepKind::AttrValue => {
        //     add_attr_value(self, get_text_from_step(template_str, &step));
        // }
        // StepKind::AttrValueUnquoted => {
        //     add_attr_value_unquoted(self, get_text_from_step(template_str, &step));
        // }
        // StepKind::Text => {
        //     push_text(self, get_text_from_step(template_str, &step));
        // }
        // StepKind::TailTag => {
        //     pop_element(self, get_text_from_step(template_str, &step));
        // }
        // // injections
        // StepKind::AttrMapInjection => {
        //     push_attr_map_injection(self, get_text_from_step(template_str, &step));
        // }
        // StepKind::DescendantInjection => {
        //     push_descendant_injection(self, get_text_from_step(template_str, &step));
        // }
        // all other steps silently pass through
        _ => {}
    }
}

// push step
// then get string
// fn push_element(builder: &mut HtmlBuilder, tag: &str) {
//     builder.tags.push(tag.to_string());
//     if let Some(last) = builder.strs.last_mut() {
//         last.push('<');
//         last.push_str(tag);
//     }
// }

// fn close_element(builder: &mut HtmlBuilder, tag: &str) {
//     if let Some(last) = builder.strs.last_mut() {
//         last.push_str(">");
//     }
// }

// fn close_void_element(builder: &mut HtmlBuilder, tag: &str) {
//     builder.tags.pop();
//     if let Some(last) = builder.strs.last_mut() {
//         last.push_str(">");
//     }
// }

// fn pop_element(builder: &mut HtmlBuilder, tag: &str) {
//     builder.tags.pop();
//     if let Some(last) = builder.strs.last_mut() {
//         last.push_str("</");
//         last.push_str(tag);
//         last.push_str(">");
//     }
// }

// fn push_text(builder: &mut HtmlBuilder, text: &str) {
//     if let Some(last) = builder.strs.last_mut() {
//         last.push_str(text);
//     }
// }

// fn add_attr(builder: &mut HtmlBuilder, tag: &str) {
//     if let Some(last) = builder.strs.last_mut() {
//         last.push(' ');
//         last.push_str(tag);
//     }
// }

// fn add_attr_value(builder: &mut HtmlBuilder, tag: &str) {
//     if let Some(last) = builder.strs.last_mut() {
//         last.push_str("=\"");
//         last.push_str(tag);
//         last.push('"');
//     }
// }

// fn add_attr_value_unquoted(builder: &mut HtmlBuilder, tag: &str) {
//     if let Some(last) = builder.strs.last_mut() {
//         last.push('=');
//         last.push_str(tag);
//     }
// }

// // injections
// // keep track of tag step to understand if attribute needs to be sanitized
// fn push_attr_map_injection(builder: &mut HtmlBuilder, tag: &str) {
//     builder.strs.push("".to_string());
//     builder.inj_details.push(Some(StepKind::AttrMapInjection));
// }

// // keep track of tag step to decide if attribute needs to be sanitized later
// //  as in, will style and script text injections preserve "<"
// //  else replace <
// //
// //
// fn push_descendant_injection(builder: &mut HtmlBuilder, tag: &str) {
//     builder.strs.push("".to_string());
//     builder.inj_details.push(Some(StepKind::DescendantInjection));
// }
