use txml::{Component, Template};

mod txml_builder;

struct TemplateBit {
    pub inj_index: usize,
}

pub enum StackBit<'a> {
    Tmpl(&'a Component, TemplateBit),
    Cmpnt(&'a Component),
    None,
}

fn getStackable(component: &Component) -> StackBit {
    match component {
        Component::Text(text) => StackBit::Cmpnt(component),
        Component::List(list) => StackBit::Cmpnt(component),
        Component::Tmpl(tmpl) => {
            return StackBit::Tmpl(component, TemplateBit{inj_index: 0})
        }
        _ => return StackBit::None,
    }
}

fn build_template(component: Component) -> String {
    let mut templ_str = "".to_string();

    let mut stack: Vec<StackBit> = Vec::from([getStackable(&component)]);
    while let Some(stack_bit) = stack.pop() {
        // //new outer match grabing component or template
        match stack_bit {
            StackBit::Tmpl(tmpl, mut bit) => {
                // figure out injection strategy
                bit.inj_index += 1;
            },
            StackBit::Cmpnt(cmpnt) => {
                match cmpnt {
                    // break lists into smaller chuncks
                    Component::List(list) => {
                        for cmpnt in list.iter().rev() {
                            stack.push(getStackable(cmpnt));
                        }
                    },
                    Component::Text(text) => templ_str.push_str(text),
                    _ => {},
                }
            },
            _ => {}
        }
    }

    templ_str
}
