use parsley::constants::{
    ATTRIBUTE, ATTRIBUTE_MAP_INJECTION, ATTRIBUTE_VALUE, CLOSE_TAGNAME, DESCENDANT_INJECTION,
    INDEPENDENT_NODE_CLOSED, NODE_CLOSED, TAGNAME, TEXT,
};
use parsley::parse;
use parsley::type_flyweight::Step;
use std::vec;

/*
    HTML template requirements:

    handle void elements

    don't add closing slashes (not valid html)

    do not add "on" events to elements,

    this will require knowing parent element and last attribute

    should there be an all or nothing quit attidue? if one wrong thing happens
    than nothing is built
*/

// where I is an Injection Enum
struct StaticHtmlBuilder {
    result: String,
    tab_count: usize,
}

impl StaticHtmlBuilder {
		// steps
		fn push_node() {}
		fn add_attr() {}
		fn add_attr_value() {}
		fn add_text() {}
		fn pop_node() {}
		
		// injections
    fn add_attr_map() {}
    fn add_descendants() {}
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

pub fn build<'a, T>(template: &'a Template<'a, T>) -> () {
    let mut stack = Vec::<StackBits<T>>::from([
		  StackBits::Template(TemplateBit {
		      iterator: parse::parse_str(&template.template_str, "INITIAL").into_iter(),
		      template: template,
		      inj_index: 0,
		  })
    ]);

    while stack.len() != 0 {
        let stack_bit = match stack.pop() {
            Some(n) => n,
            _ => return,
        };

        match stack_bit {
            StackBits::Text(text) => {

            }
            StackBits::Template(mut stack_bit) => {
                while let Some(node_step) = stack_bit.iterator.next() {
                    match node_step.kind {
                        TAGNAME => {

                        }
                        NODE_CLOSED => {

                        }
                        INDEPENDENT_NODE_CLOSED => {

                        }
                        ATTRIBUTE => {

                        }
                        ATTRIBUTE_VALUE => {

                        }
                        TEXT => {

                        }
                        CLOSE_TAGNAME => {

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
