use txml::Injection::{AttrValue, List, Text, Tmpl};
use txml::{Injection, Template, TemplateKind};

fn lil_xml(template_str: String, injections: Vec<Injection>) -> Template {
    Template {
        kind: TemplateKind::Html,
        template_str: template_str,
        injections: injections,
    }
}

fn woof() -> Template {
    lil_xml("<text>hai :3</text>".to_string(), Vec::new())
}

fn woof_woof() -> Template {
    let descendants = List(Vec::from([
        Tmpl(woof()),
        Text("you're a boy kisser aren't you >:3".to_string()),
        Tmpl(lil_xml(
            "<input type=submit value=\"yus -_-\">".to_string(),
            Vec::new(),
        )),
    ]));

    let attributes = List(Vec::from([
        AttrValue("action".to_string(), "/uwu".to_string()),
        AttrValue("method".to_string(), "post".to_string()),
    ]));

    lil_xml(
        "<form {}>{}</form>".to_string(),
        Vec::from([attributes, descendants]),
    )
}
