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

// ergonomic functions to quickly create componets without the typical rust verbosity
//  (improves readability of component code considerably)
pub fn tmpl<const N: usize>(template_str: &str, injections: [Component; N]) -> Component {
    Component::Tmpl(Template {
        template_str: template_str.to_string(),
        injections: Vec::from(injections),
    })
}

pub fn text(txt: &str) -> Component {
    let escaped = txt.replace("<", "&lt;").replace("&", "&amp;");
    Component::Text(escaped)
}

pub fn unescaped_text(txt: &str) -> Component {
    Component::Text(txt.to_string())
}

pub fn attr(attr_str: &str) -> Component {
    Component::Attr(attr_str.to_string())
}

pub fn attr_val(attr_str: &str, value_txt: &str) -> Component {
    let escaped_value = value_txt.replace("\"", "&quot;").replace("&", "&amp;");
    Component::AttrVal(attr_str.to_string(), escaped_value)
}

pub fn list<const N: usize>(components: [Component; N]) -> Component {
    Component::List(Vec::from(components))
}

pub fn vlist(components: Vec<Component>) -> Component {
    Component::List(components)
}
