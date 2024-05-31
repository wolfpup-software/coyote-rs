use txml::Component;

mod txml_builder;

// define stack

// template_bit
//  -> compoment template
//  -> injection index


fn build_template(component: Component) -> String {
    let templ_str = "".to_string();

    // stack across template
    // templates -> template builds

    // list -> 
    match sdf {
        Component::Template(tmpl) => {},
        Component::Text(text) => add_text(&mut templ_str, &text),
        Component::Attr(attr) => add_attr(&mut templ_str, &text),
        Component::AttrVal(attr, val) => add_attr(&mut templ_str, &text),
        Component::List(list) => {
            // get a list of stackable elements
            // reverse them
            // add +1 to injection index
            // add  to stack
            
        },
    }

    templ
}

fn add_text(&mut templ: String, text: &str) {
    templ.push_str(text);
}

fn add_attr(&mut templ: String, attr: &str) {
    templ.push_str();
    templ.push_str(attr);
}

fn add_attr_val(&mut templ: String, attr: &str, value: &str) {
    templ.push_str();
    templ.push_str(attr);
    templ.push_str("=\"");
    templ.push_str(value);
    templ.push_str("\"")
}
