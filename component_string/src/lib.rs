use coyote::Component;
use parse::StepKind;
use rulesets::RulesetImpl;
use template_steps::{compose as compose_steps, Results as TemplateSteps};

mod compose;
mod tag_info;

use crate::compose::compose_steps as compose_by_steps;
use crate::compose::push_text_logic;
use crate::tag_info::TagInfo;

struct TemplateBit {
    pub inj_index: usize,
}

enum StackBit<'a> {
    Tmpl(&'a Component, TemplateSteps, TemplateBit),
    Cmpnt(&'a Component),
    None,
}

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
        compose_steps(rules, template_str)
    }
}

pub fn compose(
    builder: &mut dyn BuilderImpl,
    rules: &dyn RulesetImpl,
    component: &Component,
) -> String {
    let mut templ_str = "".to_string();

    let sbit = get_stack_bit_from_component(builder, rules, component);

    let mut results = "".to_string();
    let mut tag_info_stack: Vec<TagInfo> = Vec::new();

    let mut stack: Vec<StackBit> = Vec::from([sbit]);
    while let Some(mut stack_bit) = stack.pop() {
        match stack_bit {
            // text or list
            StackBit::Cmpnt(cmpnt) => match cmpnt {
                Component::Text(text) => {
                    push_text_logic(&mut results, &mut tag_info_stack, rules, text);
                }
                Component::List(list) => {
                    for cmpnt in list.iter().rev() {
                        let bit = get_stack_bit_from_component(builder, rules, cmpnt);
                        stack.push(bit);
                    }
                    continue;
                }
                _ => {}
            },
            StackBit::Tmpl(component, ref results, ref mut bit) => {
                let index = bit.inj_index;
                bit.inj_index += 1;

                // [TODO]
                // verify results
                //
                // second step is text | node_open | descendant_injection
                // last step is Text | node closed | independed_node_closed

                if let Component::Tmpl(template) = component {
                    // add current template chunk 
                    if let Some(chunk) = results.steps.get(index) {
                        compose_by_steps(
                            rules,
                            &mut templ_str,
                            &mut tag_info_stack,
                            &template.template_str,
                            chunk,
                        );
                    }

                    // if there is an injection
                    if let (Some(inj_step), Some(inj)) =
                        (results.injs.get(index), template.injections.get(index))
                    {
                        match inj_step.kind {
                            // add attribute injections to template
                            StepKind::AttrMapInjection => {
                                add_attr_inj(&mut templ_str, inj);
                            }
                            // add descendant injections to the stack
                            StepKind::DescendantInjection => {
                                // push template back and bail early
                                stack.push(stack_bit);

                                let bit = get_stack_bit_from_component(builder, rules, inj);
                                stack.push(bit);

                                continue;
                            }
                            _ => {}
                        }
                    }

                    // don't forget the last part of the templates!
                    if index < results.steps.len() {
                        stack.push(stack_bit);
                    }
                }
            }
            _ => {}
        }
    }

    templ_str
}

fn get_stack_bit_from_component<'a>(
    builder: &mut dyn BuilderImpl,
    rules: &dyn RulesetImpl,
    component: &'a Component,
) -> StackBit<'a> {
    match component {
        Component::Text(_text) => StackBit::Cmpnt(component),
        Component::List(_list) => StackBit::Cmpnt(component),
        Component::Tmpl(tmpl) => {
            let template_steps = builder.build(rules, &tmpl.template_str);
            StackBit::Tmpl(component, template_steps, TemplateBit { inj_index: 0 })
        }
        _ => StackBit::None,
    }
}

fn add_attr_inj(template_str: &mut String, component: &Component) {
    match component {
        Component::Attr(attr) => add_attr(template_str, attr),
        Component::AttrVal(attr, val) => add_attr_val(template_str, attr, val),
        Component::List(attr_list) => {
            for cmpnt in attr_list {
                match cmpnt {
                    Component::Attr(attr) => {
                        add_attr(template_str, &attr);
                    }
                    Component::AttrVal(attr, val) => {
                        add_attr_val(template_str, &attr, &val);
                    }
                    _ => {}
                }
            }
        }
        _ => {}
    }
}

fn add_attr(templ_str: &mut String, attr: &str) {
    templ_str.push_str(" ");
    templ_str.push_str(attr);
}

fn add_attr_val(templ_str: &mut String, attr: &str, val: &str) {
    templ_str.push_str(" ");
    templ_str.push_str(attr);
    templ_str.push_str("=\"");
    templ_str.push_str(val);
    templ_str.push_str("\"");
}
