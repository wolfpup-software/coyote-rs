use parsley::{get_text_from_step, parse_str, Step, StepKind};
use std::vec;

pub trait TxmlBuilder<'a, T> {
    // steps
    fn push_node(&self, tag: &'a str) -> ();
    fn push_text(&self, text: &'a str) -> ();
    fn add_attr(&self, attr: &'a str) -> ();
    fn add_attr_value(&self, value: &'a str) -> ();
    fn pop_node(&self, tag: &'a str) -> ();
    fn pop_independent_node(&self) -> ();
    // injections
    fn add_attr_map(&self, injection: T) -> ();
    fn get_descendants(&self, injection: T) -> Vec<StackBit<'a, T>>;
}

#[derive(Debug)]
pub struct Template<'a, I> {
    pub kind: &'a str,
    pub injections: Vec<I>,
    pub template_str: &'a str,
}

pub enum StackBit<'a, I> {
    Template(TemplateBit<'a, I>),
    Text(&'a str),
}

pub struct TemplateBit<'a, I> {
    template: Template<'a, I>,
    iterator: vec::IntoIter<Step>,
    inj_index: usize,
}

pub fn get_stack_bit_from_template<'a, T>(template: Template<'a, T>) -> StackBit<'a, T> {
    StackBit::Template(TemplateBit {
        iterator: parse_str(template.template_str, StepKind::Initial).into_iter(),
        template: template,
        inj_index: 0,
    })
}

pub fn build<'a, T>(builder: &mut impl TxmlBuilder<'a, T>, template: Template<'a, T>) -> () {
    let mut stack = Vec::<StackBit<'a, T>>::from([get_stack_bit_from_template(template)]);

    while stack.len() != 0 {
        let stack_bit = match stack.pop() {
            Some(n) => n,
            _ => return,
        };

        match stack_bit {
            StackBit::Text(text) => builder.push_text(text),
            StackBit::Template(mut stack_bit) => {
                // build template immediately into Vec::<String>;
                // all steps are performed UP UNTIL a descendant injection
                // this can be cached and performed
                //
                // instead of iterator maybe a vec of descendant indexes
                // templates
                //
                while let Some(node_step) = stack_bit.iterator.next() {
                    match node_step.kind {
                        // steps
                        StepKind::Tagname => {
                            builder.push_node(get_text_from_step(
                                &stack_bit.template.template_str,
                                &node_step,
                            ));
                        }
                        StepKind::NodeClosed => {
                            builder.pop_node(get_text_from_step(
                                &stack_bit.template.template_str,
                                &node_step,
                            ));
                        }
                        StepKind::IndependentNodeClosed => {
                            builder.pop_independent_node();
                        }
                        StepKind::Attr => {
                            builder.add_attr(get_text_from_step(
                                &stack_bit.template.template_str,
                                &node_step,
                            ));
                        }
                        StepKind::AttrValue => {
                            builder.add_attr_value(get_text_from_step(
                                &stack_bit.template.template_str,
                                &node_step,
                            ));
                        }
                        StepKind::Text => {
                            builder.push_text(get_text_from_step(
                                &stack_bit.template.template_str,
                                &node_step,
                            ));
                        }
                        StepKind::CloseTagname => {
                            builder.pop_node(get_text_from_step(
                                &stack_bit.template.template_str,
                                &node_step,
                            ));
                        }
                        // injections
                        StepKind::AttrMapInjection => {
                            let injection = stack_bit.template.injections.pop();
                            if let Some(inj) = injection {
                                builder.add_attr_map(inj);
                            };
                        }
                        StepKind::DescendantInjection => {
                            let injection = stack_bit.template.injections.pop();
                            stack.push(StackBit::Template(stack_bit));

                            // descendants must be in reversed order from
                            if let Some(inj) = injection {
                                stack.append(&mut builder.get_descendants(inj));
                            };
                            // skip to the top of the stack after descendant injection
                            break;
                        }
                        // all other steps silently pass through
                        _ => {}
                    }
                }
            }
        }
    }
}
