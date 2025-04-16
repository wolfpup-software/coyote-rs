use crate::components::Component;
use crate::compose_steps::{
    compose_steps, push_attr_component, push_attr_value_component, push_text_component,
};
use crate::routes::StepKind;
use crate::rulesets::RulesetImpl;
use crate::tag_info::TagInfo;
use crate::template_builder::BuilderImpl;
use crate::template_steps::Results as TemplateSteps;

#[derive(Debug)]
struct TemplateBit {
    pub inj_index: usize,
    pub stack_depth: isize,
}

enum StackBit<'a> {
    Tmpl(&'a Component, TemplateSteps, TemplateBit),
    Cmpnt(&'a Component),
    None,
}

/*
    TODO:

    `compose_string` might have too many lines of code (100+).

    This might simply be a nexus of scope and complexity.

    Current refinement attempts resulted in functions with 7 arguments.
    So ... not the best solution ever.

    At least the challenge is isolated to this function.
*/
pub fn compose_string(
    builder: &mut dyn BuilderImpl,
    rules: &dyn RulesetImpl,
    component: &Component,
) -> Result<String, String> {
    let mut template_results = "".to_string();

    let mut tag_info_stack: Vec<TagInfo> = Vec::from([TagInfo::new(rules, ":root")]);
    let mut component_stack: Vec<StackBit> = Vec::from([get_bit_from_component_stack(
        &mut tag_info_stack,
        builder,
        rules,
        component,
    )]);

    while let Some(mut cmpnt_bit) = component_stack.pop() {
        match cmpnt_bit {
            // text or list
            StackBit::Cmpnt(cmpnt) => match cmpnt {
                Component::Text(text) => {
                    push_text_component(&mut template_results, &mut tag_info_stack, rules, text);
                }
                Component::List(list) => {
                    for cmpnt in list.iter().rev() {
                        let bit = get_bit_from_component_stack(
                            &mut tag_info_stack,
                            builder,
                            rules,
                            cmpnt,
                        );
                        component_stack.push(bit);
                    }
                }
                _ => {}
            },
            // template chunk and possible injection
            StackBit::Tmpl(cmpnt, ref template, ref mut bit) => {
                let index = bit.inj_index;
                bit.inj_index += 1;

                let tmpl_cmpnt = match cmpnt {
                    Component::Tmpl(cmpnt) => cmpnt,
                    _ => continue,
                };

                // add current template chunk
                if let Some(chunk) = template.steps.get(index) {
                    compose_steps(
                        rules,
                        &mut template_results,
                        &mut tag_info_stack,
                        &tmpl_cmpnt.template_str,
                        chunk,
                    );
                } else {
                    // is this balanced?
                    // if tag_info_stack.len() != bit.stack_depth {
                    //     println!("double oooooohhh");
                    // }
                    println!("stack depth: {} {}", tag_info_stack.len(), bit.stack_depth);
                    if bit.stack_depth != tag_info_stack.len() as isize {
                        println!("oooooooooh");
                        return Err(
                            "Coyote Err: the following template component is imbalanced:\n{:?}"
                                .to_string()
                                + tmpl_cmpnt.template_str,
                        );
                    }
                }

                // add injections
                if let (Some(inj_step), Some(inj)) =
                    (template.injs.get(index), tmpl_cmpnt.injections.get(index))
                {
                    match inj_step.kind {
                        StepKind::AttrMapInjection => {
                            add_attr_inj(&mut tag_info_stack, &mut template_results, inj);
                        }
                        // push template back and bail early
                        StepKind::DescendantInjection => {
                            component_stack.push(cmpnt_bit);

                            let bit = get_bit_from_component_stack(
                                &mut tag_info_stack,
                                builder,
                                rules,
                                inj,
                            );
                            component_stack.push(bit);

                            continue;
                        }
                        _ => {}
                    }
                }

                if index < template.steps.len() {
                    component_stack.push(cmpnt_bit);
                }
            }
            _ => {}
        }
    }

    Ok(template_results)
}

fn get_bit_from_component_stack<'a>(
    stack: &mut Vec<TagInfo>,
    builder: &mut dyn BuilderImpl,
    rules: &dyn RulesetImpl,
    cmpnt: &'a Component,
) -> StackBit<'a> {
    match cmpnt {
        Component::Text(_) => StackBit::Cmpnt(cmpnt),
        Component::List(_) => StackBit::Cmpnt(cmpnt),
        Component::Tmpl(tmpl) => {
            let template_steps = builder.build(rules, &tmpl.template_str);
            StackBit::Tmpl(
                cmpnt,
                template_steps,
                TemplateBit {
                    inj_index: 0,
                    stack_depth: stack.len() as isize,
                },
            )
        }
        _ => StackBit::None,
    }
}

fn add_attr_inj(stack: &mut Vec<TagInfo>, template_str: &mut String, cmpnt: &Component) {
    match cmpnt {
        Component::Attr(attr) => push_attr_component(template_str, stack, attr),
        Component::AttrVal(attr, val) => {
            push_attr_component(template_str, stack, attr);
            push_attr_value_component(template_str, stack, val);
        }
        Component::List(attr_list) => {
            for cmpnt in attr_list {
                match cmpnt {
                    Component::Attr(attr) => {
                        push_attr_component(template_str, stack, attr);
                    }
                    Component::AttrVal(attr, val) => {
                        push_attr_component(template_str, stack, attr);
                        push_attr_value_component(template_str, stack, val);
                    }
                    _ => {}
                }
            }
        }
        _ => {}
    }
}
