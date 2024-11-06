use coyote::Component;
use parse::StepKind;
use sieve::SieveImpl;
use template_string::Results;

struct TemplateBit {
    pub inj_index: usize,
}

enum StackBit<'a> {
    Tmpl(&'a Component, Results, TemplateBit),
    Cmpnt(&'a Component),
    None,
}

pub trait BuilderImpl {
    fn build(&mut self, sieve: &dyn SieveImpl, template_str: &str) -> Results;
}

pub fn compose(
    builder: &mut dyn BuilderImpl,
    sieve: &dyn SieveImpl,
    component: &Component,
) -> String {
    let mut templ_str = "".to_string();

    let sbit = get_stack_bit_from_component(builder, sieve, component);

    let mut stack: Vec<StackBit> = Vec::from([sbit]);
    while let Some(mut stack_bit) = stack.pop() {
        match stack_bit {
            // text or list
            StackBit::Cmpnt(cmpnt) => match cmpnt {
                Component::Text(text) => templ_str.push_str(text),
                Component::List(list) => {
                    for cmpnt in list.iter().rev() {
                        let bit = get_stack_bit_from_component(builder, sieve, cmpnt);
                        stack.push(bit);
                    }
                    continue;
                }
                _ => {}
            },
            StackBit::Tmpl(component, ref results, ref mut bit) => {
                let index = bit.inj_index;
                bit.inj_index += 1;

                // add template
                if let Some(chunk) = results.strs.get(index) {
                    templ_str.push_str(chunk);
                }
                // add injection
                if let Component::Tmpl(template) = component {
                    // if there is an injection
                    if let (Some(inj_kind), Some(inj)) =
                        (results.injs.get(index), template.injections.get(index))
                    {
                        match inj_kind {
                            // add attribute injections to template
                            StepKind::AttrMapInjection => {
                                templ_str = add_attr_inj(templ_str, inj);
                            }
                            // queue descendant injections to queue
                            StepKind::DescendantInjection => {
                                // push template back and bail early
                                stack.push(stack_bit);

                                let bit = get_stack_bit_from_component(builder, sieve, inj);
                                stack.push(bit);
                                continue;
                            }
                            _ => {}
                        }
                    }

                    // don't forget the last part of the templates!
                    if index < results.strs.len() {
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
    sieve: &dyn SieveImpl,
    component: &'a Component,
) -> StackBit<'a> {
    match component {
        Component::Text(_text) => StackBit::Cmpnt(component),
        Component::List(_list) => StackBit::Cmpnt(component),
        Component::Tmpl(tmpl) => {
            let txml_literal = builder.build(sieve, &tmpl.template_str);
            StackBit::Tmpl(component, txml_literal, TemplateBit { inj_index: 0 })
        }
        _ => StackBit::None,
    }
}

fn add_attr_inj(mut template_str: String, component: &Component) -> String {
    match component {
        Component::Attr(attr) => add_attr(template_str, attr),
        Component::AttrVal(attr, val) => add_attr_val(template_str, attr, val),
        Component::List(attr_list) => {
            for cmpnt in attr_list {
                match cmpnt {
                    Component::Attr(attr) => {
                        template_str = add_attr(template_str, &attr);
                    }
                    Component::AttrVal(attr, val) => {
                        template_str = add_attr_val(template_str, &attr, &val);
                    }
                    _ => {}
                }
            }
            template_str
        }
        _ => template_str,
    }
}

fn add_attr(mut templ_str: String, attr: &str) -> String {
    templ_str.push_str(" ");
    templ_str.push_str(attr);

    templ_str
}

fn add_attr_val(mut templ_str: String, attr: &str, val: &str) -> String {
    templ_str.push_str(" ");
    templ_str.push_str(attr);
    templ_str.push_str("=\"");
    templ_str.push_str(val);
    templ_str.push_str("\"");

    templ_str
}
