use parsley::constants::{
    ATTRIBUTE, ATTRIBUTE_MAP_INJECTION, ATTRIBUTE_VALUE, CLOSE_TAGNAME, DESCENDANT_INJECTION,
    INDEPENDENT_NODE_CLOSED, NODE_CLOSED, TAGNAME, TEXT,
};
use parsley::parse;
use parsley::type_flyweight::NodeStep;
use std::collections::HashMap;
use std::vec;

/*
    handle void elements

    don't add closing slashes (not valid html)

    do not add "on" events to elements,

    this will require knowing parent element and last attribute

    should there be an all or nothing quit attidue? if one wrong thing happens
    than nothing is built
*/

#[derive(Debug)]
pub enum Injection<'a> {
    Text(&'a str),
    Attr(&'a str),
    AttrValue(&'a str, &'a str),
    Template(Template<'a>),
		List(&'a [Injection<'a>]),
}

#[derive(Debug)]
pub struct Template<'a> {
    injections: &'a [Injection<'a>],
    template: &'a str,
}

pub enum StackBits<'a> {
    Template(TemplateBit<'a>),
    Text(&'a str),
}

pub struct TemplateBit<'a> {
    template: &'a Template<'a>,
    iterator: vec::IntoIter<NodeStep<'a>>,
    inj_index: usize,
}

// rules
// no fallback elements, no content: style, script
// skip html listeners "onclick"

pub fn build<'a>(template: &'a Template<'a>) -> String {
    let mut stack = Vec::<StackBits>::new();

    stack.push(StackBits::Template(TemplateBit {
        iterator: parse::parse_str(&template.template).into_iter(),
        template: template,
        inj_index: 0,
    }));

    let mut result = String::from("");
    let mut tab_count = 0;

    while stack.len() != 0 {
        let stack_bit = match stack.pop() {
            Some(n) => n,
            _ => return result,
        };

        match stack_bit {
            StackBits::Text(text) => {
                let text_iterator = text.trim().split("\n");
                for text in text_iterator {
                    result.push_str(&"\t".repeat(tab_count));
                    result.push_str(text.trim());
                    result.push_str("\n");
                }
            }
            StackBits::Template(mut stack_bit) => {
                while let Some(node_step) = stack_bit.iterator.next() {
                    match node_step.kind {
                        TAGNAME => {
                            // check here if tagname allowed
                            // or if last parent was a script?
                            result.push_str(&"\t".repeat(tab_count));
                            result.push_str("<");
                            result.push_str(parse::get_chunk(
                                &stack_bit.template.template,
                                &node_step.vector,
                            ));
                        }
                        NODE_CLOSED => {
                            result.push_str(">\n");
                            tab_count += 1;
                        }
                        INDEPENDENT_NODE_CLOSED => {
                            result.push_str("/>\n");
                            tab_count -= 1;
                        }
                        ATTRIBUTE => {
                            // if attribute is blocked, skip
                            result.push_str(" ");
                            result.push_str(parse::get_chunk(
                                &stack_bit.template.template,
                                &node_step.vector,
                            ));
                        }
                        ATTRIBUTE_VALUE => {
                            // if attribute is blocked, skip
                            result.push_str("=\"");
                            result.push_str(parse::get_chunk(
                                &stack_bit.template.template,
                                &node_step.vector,
                            ));
                            result.push_str("\"");
                        }
                        TEXT => {
                            let text_iterator =
                                parse::get_chunk(&stack_bit.template.template, &node_step.vector)
                                    .trim()
                                    .split("\n");

                            for text in text_iterator {
                                result.push_str(&"\t".repeat(tab_count));
                                result.push_str(text.trim());
                                result.push_str("\n");
                            }
                        }
                        CLOSE_TAGNAME => {
                            tab_count -= 1;
                            result.push_str(&"\t".repeat(tab_count));
                            result.push_str("</");
                            result.push_str(parse::get_chunk(
                                &stack_bit.template.template,
                                &node_step.vector,
                            ));
                            result.push_str(">\n");
                        }
                        ATTRIBUTE_MAP_INJECTION => {
                            let injections = &stack_bit.template.injections[stack_bit.inj_index];
                            stack_bit.inj_index += 1;
                            
                            // check if its an
                            // attr
                            // attrvalue
                            // list of attr, attrValue
														match injections {
															Injection::Attr(text) => {},
															Injection::AttrValue(template) => {},
															Injection::List(attributes) => {},
															_ => {},
														}
													/*
                            for injection in injections {
                                match injection {
                                    Injection::Attr(attr) => {
                                        // if attribute is blocked, skip
                                        result.push_str(" ");
                                        result.push_str(attr);
                                    }
                                    Injection::AttrValue(attr, value) => {
                                        // if attribute is blocked, skip
                                        result.push_str(" ");
                                        result.push_str(attr);
                                        result.push_str("=\"");
                                        result.push_str(value);
                                        result.push_str("\"");
                                    }
                                    _ => continue,
                                }
                            }
                        */
                        }
                        DESCENDANT_INJECTION => {
                            // if parent is SCRIPT or STYLE, skip
                            let injections = &stack_bit.template.injections[stack_bit.inj_index];
                            stack_bit.inj_index += 1;

                            stack.push(StackBits::Template(stack_bit));
														
														// check if:
														//	text
														//	descendant
														//	list of text, descendant
														
														match injections {
															Injection::Text(text) => {},
															Injection::Template(template) => {},
															Injection::List(descendants) => {},
															_ => {},
														}
														/*
                            for injection in injections.iter().rev() {
                                match injection {
                                    Injection::Text(text) => stack.push(StackBits::Text(text)),
                                    Injection::Template(template) => {
                                        stack.push(StackBits::Template(TemplateBit {
                                            iterator: parse::parse_str(&template.template)
                                                .into_iter(),
                                            template: &template,
                                            inj_index: 0,
                                        }))
                                    }
                                    _ => continue,
                                }
                            }
                            */

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

    result
}

//
pub fn html<'a>(template: &'a str, injections: &'a [Injection<'a>]) -> Template<'a> {
    Template {
        template: template,
        injections: injections,
    }
}
