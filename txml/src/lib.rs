// Components are injected into templates
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Component {
    Text(String),
    Attr(String),
    AttrVal(String, String),
    Tmpl(Template),
    List(Vec<Component>),
    None,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Template {
    pub template_str: String,
    pub injections: Vec<Component>,
}

// defacto template function
pub fn txml<const N: usize>(template_str: &str, injections: [Component; N]) -> Component {
    Component::Tmpl(Template {
        template_str: template_str.to_string(),
        injections: Vec::from(injections),
    })
}

// ergonomic functions to quickly create Component Enums
// (considerably improves readability of component code)
pub fn text(txt: &str) -> Component {
    let escaped = txt.replace("<", "&lt;").replace("&", "&amp;");
    Component::Text(escaped)
}

pub fn unescaped_text(txt: String) -> Component {
    Component::Text(txt)
}

pub fn attr(attr_str: &str) -> Component {
    let escaped_attr = attr_str
        .replace("<", "")
        .replace(">", "")
        .replace("&", "")
        .replace("\"", "")
        .replace("'", "");

    Component::Attr(escaped_attr)
}

pub fn attr_val(attr_str: &str, value_txt: &str) -> Component {
    let escaped_attr = attr_str
        .replace("<", "")
        .replace(">", "")
        .replace("&", "")
        .replace("\"", "")
        .replace("'", "");

    let escaped_value = value_txt.replace("\"", "&quot;").replace("&", "&amp;");
    Component::AttrVal(escaped_attr, escaped_value)
}

pub fn tmpl(template: Template) -> Component {
    Component::Tmpl(template)
}

pub fn list<const N: usize>(components: [Component; N]) -> Component {
    Component::List(Vec::from(components))
}

pub fn vlist(components: Vec<Component>) -> Component {
    Component::List(components)
}
