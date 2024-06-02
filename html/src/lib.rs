use txml::{Component, Template};

mod txml_builder;

use txml_builder::{HtmlBuilder, HtmlBuilderResults};

struct TemplateBit {
    pub inj_index: usize,
}

pub enum StackBit<'a> {
    Tmpl(HtmlBuilderResults, TemplateBit),
    Cmpnt(&'a Component),
    None,
}

fn getStackable<'a>(
    mut builder: HtmlBuilder,
    component: &'a Component,
) -> (HtmlBuilder, StackBit<'a>) {
    match component {
        Component::Text(text) => (builder, StackBit::Cmpnt(component)),
        Component::List(list) => (builder, StackBit::Cmpnt(component)),
        Component::Tmpl(tmpl) => {
            let results = builder.build(tmpl);
            return (builder, StackBit::Tmpl(results, TemplateBit{inj_index: 0}));
        }
        _ => return (builder, StackBit::None),
    }
}

fn build_template(component: Component) -> String {
    let mut builder = HtmlBuilder::new();
    let mut templ_str = "".to_string();

    let stack_bit;
    (builder, stack_bit) = getStackable(builder, &component);
    let mut stack: Vec<StackBit> = Vec::from([stack_bit]);
    
    while let Some(stack_bit) = stack.pop() {
        match stack_bit {
            StackBit::Tmpl(results, mut bit) => {
                let tmpl_str = results.strs.get(bit.inj_index);
                let inj_kind = results.injs.get(bit.inj_index);

                bit.inj_index += 1;
            },
            StackBit::Cmpnt(cmpnt) => {
                match cmpnt {
                    // break lists into smaller chuncks
                    Component::List(list) => {
                        for cmpnt in list.iter().rev() {
                            let bit;
                            (builder, bit) = getStackable(builder, cmpnt);
                            stack.push(bit);
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
