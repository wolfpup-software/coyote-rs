use txml::{Component, Template};

mod txml_builder;

// need to render the template
// then iterate through the built chunks

struct TemplateBit<'a> {
    pub template: &'a Template,
    pub inj_index: usize,
}

pub enum StackBit<'a> {
    Tmpl(TemplateBit<'a>),
    Cmpnt(&'a Component),
    None,
}

fn getStackable(component: &Component) -> StackBit {
    match component {
        Component::Text(text) => StackBit::Cmpnt(component),
        Component::List(list) => StackBit::Cmpnt(component),
        Component::Tmpl(tmpl) => {
            return StackBit::Tmpl(TemplateBit {
                template: tmpl,
                inj_index: 0,
            })
        }
        _ => return StackBit::None,
    }
}

// fn build_template(component: Component) -> String {
//     let mut templ_str = "".to_string();

//     let mut stack: Vec<Option<StackBit>> = Vec::from([getStackable(&component)]);
//     while let Some(frame_opt) = stack.pop() {
//         let mut stack_bit = match frame_opt {
//             Some(frame) => frame,
//             _ => continue,
//         };

// new outer match grabing component or template

//         match stack_bit.component {
//             // break lists into smaller chuncks
//             Component::List(list) => {
//                 for cmpnt in list.iter().rev() {
//                     stack.push(getStackable(cmpnt));
//                 }
//                 continue;
//             },
//             Component::Text(text) => templ_str.push_str(text),
//             // if template
//             _ => {},
//         }
//     }

//     templ_str
// }

// fn add_text(templ: &mut String, text: &str) {
//     templ.push_str(text);
// }

// // fn add_attr(templ: &mut String, attr: &str) {
// //     templ.push_str(" ");
// //     templ.push_str(attr);
// // }

// // fn add_attr_val(templ: &mut String, attr: &str, value: &str) {
// //     templ.push_str(" ");
// //     templ.push_str(attr);
// //     templ.push_str("=\"");
// //     templ.push_str(value);
// //     templ.push_str("\"");
// // }
