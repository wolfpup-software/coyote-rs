use parsley::constants::{
    ATTRIBUTE, ATTRIBUTE_MAP_INJECTION, ATTRIBUTE_VALUE, CLOSE_TAGNAME, DESCENDANT_INJECTION,
    INDEPENDENT_NODE_CLOSED, NODE_CLOSED, TAGNAME, TEXT,
};
use parsley::parse;
use parsley::type_flyweight::Step;
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
    fn add_descendants(&self, injection: T) -> ();
}

#[derive(Debug)]
pub struct Template<'a, I> {
    pub kind: &'a str,
    pub injections: Vec<I>,
    pub template_str: &'a str,
}

pub enum StackBits<'a, I> {
    Template(TemplateBit<'a, I>),
    Text(&'a str),
}

pub struct TemplateBit<'a, I> {
    template: &'a Template<'a, I>,
    iterator: vec::IntoIter<Step<'a>>,
    inj_index: usize,
}

pub fn build<'a, T>(builder: impl TxmlBuilder<'a, T>, template: &'a Template<'a, T>) -> () {
    let mut stack = Vec::<StackBits<T>>::from([StackBits::Template(TemplateBit {
        iterator: parse::parse_str(&template.template_str, "INITIAL").into_iter(),
        template: template,
        inj_index: 0,
    })]);

    while stack.len() != 0 {
        let stack_bit = match stack.pop() {
            Some(n) => n,
            _ => return,
        };

        match stack_bit {
            StackBits::Text(text) => builder.push_text(text),
            StackBits::Template(mut stack_bit) => {
                while let Some(node_step) = stack_bit.iterator.next() {
                    match node_step.kind {
                        TAGNAME => {
                            builder.push_node(parse::get_chunk(
                                &stack_bit.template.template_str,
                                &node_step.vector,
                            ));
                        }
                        NODE_CLOSED => {
                            builder.pop_node(parse::get_chunk(
                                &stack_bit.template.template_str,
                                &node_step.vector,
                            ));
                        }
                        INDEPENDENT_NODE_CLOSED => {
                            builder.pop_independent_node();
                        }
                        ATTRIBUTE => {
                            builder.add_attr(parse::get_chunk(
                                &stack_bit.template.template_str,
                                &node_step.vector,
                            ));
                        }
                        ATTRIBUTE_VALUE => {
                            builder.add_attr_value(parse::get_chunk(
                                &stack_bit.template.template_str,
                                &node_step.vector,
                            ));
                        }
                        TEXT => {
                            builder.push_text(parse::get_chunk(
                                &stack_bit.template.template_str,
                                &node_step.vector,
                            ));
                        }
                        CLOSE_TAGNAME => {
                            builder.pop_node(parse::get_chunk(
                                &stack_bit.template.template_str,
                                &node_step.vector,
                            ));
                        }
                        ATTRIBUTE_MAP_INJECTION => {
                            let injections = &stack_bit.template.injections[stack_bit.inj_index];
                            stack_bit.inj_index += 1;
                            let injections = &stack_bit.template.injections[stack_bit.inj_index];
                            stack_bit.inj_index += 1;

                            //stack.push(StackBits::Template(stack_bit));
                        }
                        DESCENDANT_INJECTION => {
                            // if parent is SCRIPT or STYLE, skip
                            let injections = &stack_bit.template.injections[stack_bit.inj_index];
                            stack_bit.inj_index += 1;

                            stack.push(StackBits::Template(stack_bit));
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
