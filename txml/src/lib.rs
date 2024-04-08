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

struct StaticHtmlBuilder<'a> {
    result: String,
    tab_count: usize,
    stack: Vec<StackBits<'a, Injection<'a, ()>>>,
}

impl<'a> StaticHtmlBuilder<'_> {
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

// Injections could be entirely external to the "builder"

#[derive(Debug)]
// where I is an Injection Enum
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
// fhtml
//
// rules
// no fallback elements, no content: style, script
// skip html listeners "onclick"

pub fn build<'a, T>(template: &'a Template<'a, T>) -> String {
    let mut stack = Vec::<StackBits<T>>::new();

    stack.push(StackBits::Template(TemplateBit {
        iterator: parse::parse_str(&template.template_str).into_iter(),
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
            		// builder.add_text(text);
                let text_iterator = text.trim().split("\n");
                for text in text_iterator {
                    add_text(&mut result, tab_count, text);
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
                                &stack_bit.template.template_str,
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
                                &stack_bit.template.template_str,
                                &node_step.vector,
                            ));
                        }
                        ATTRIBUTE_VALUE => {
                            // if attribute is blocked, skip
                            result.push_str("=\"");
                            result.push_str(parse::get_chunk(
                                &stack_bit.template.template_str,
                                &node_step.vector,
                            ));
                            result.push_str("\"");
                        }
                        TEXT => {
                            let text_iterator = parse::get_chunk(
                                &stack_bit.template.template_str,
                                &node_step.vector,
                            )
                            .trim()
                            .split("\n");

                            for text in text_iterator {
                                add_text(&mut result, tab_count, text);
                            }
                        }
                        CLOSE_TAGNAME => {
                            tab_count -= 1;
                            result.push_str(&"\t".repeat(tab_count));
                            result.push_str("</");
                            result.push_str(parse::get_chunk(
                                &stack_bit.template.template_str,
                                &node_step.vector,
                            ));
                            result.push_str(">\n");
                        }
                        ATTRIBUTE_MAP_INJECTION => {
                            let injections = &stack_bit.template.injections[stack_bit.inj_index];
                            stack_bit.inj_index += 1;

														/*
                            match injections {
                                Injection::Attr(attr) => {
                                    add_attr(&mut result, attr);
                                }
                                Injection::AttrValue(attr, value) => {
                                    add_attr_value(&mut result, attr, value);
                                }
                                Injection::List(attributes) => {
                                    for injection in attributes.iter() {
                                        match injection {
                                            Injection::Attr(attr) => {
                                                add_attr(&mut result, attr);
                                            }
                                            Injection::AttrValue(attr, value) => {
                                                add_attr_value(&mut result, attr, value);
                                            }
                                            _ => continue,
                                        }
                                    }
                                }
                                _ => {}
                            }
                            */
                        }
                        DESCENDANT_INJECTION => {
                            // if parent is SCRIPT or STYLE, skip
                            let injections = &stack_bit.template.injections[stack_bit.inj_index];
                            stack_bit.inj_index += 1;

                            stack.push(StackBits::Template(stack_bit));

														/*
                            match injections {
                                Injection::Text(text) => stack.push(StackBits::Text(text)),
                                Injection::Template(template) => {
                                    stack.push(StackBits::Template(TemplateBit {
                                        iterator: parse::parse_str(&template.template_str)
                                            .into_iter(),
                                        template: &template,
                                        inj_index: 0,
                                    }))
                                }
                                Injection::List(descendants) => {
                                    for injection in descendants.iter().rev() {
                                        match injection {
                                            Injection::Text(text) => {
                                                stack.push(StackBits::Text(text))
                                            }
                                            Injection::Template(template) => {
                                                stack.push(StackBits::Template(TemplateBit {
                                                    iterator: parse::parse_str(
                                                        &template.template_str,
                                                    )
                                                    .into_iter(),
                                                    template: &template,
                                                    inj_index: 0,
                                                }))
                                            }
                                            _ => {}
                                        }
                                    }
                                }
                                _ => {}
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

fn add_text(result: &mut String, tab_count: usize, text: &str) -> () {
    result.push_str(&"\t".repeat(tab_count));
    result.push_str(text.trim());
    result.push_str("\n");
}

fn add_attr(result: &mut String, attr: &str) -> () {
    result.push_str(" ");
    result.push_str(attr);
}

fn add_attr_value(result: &mut String, attr: &str, value: &str) -> () {
    result.push_str(" ");
    result.push_str(attr);
    result.push_str("=\"");
    result.push_str(value);
    result.push_str("\"");
}

//
pub fn html<'a, T>(template_str: &'a str, injections: Vec<T>) -> Template<'a, T> {
    Template {
        kind: "html",
        template_str: template_str,
        injections: injections,
    }
}

