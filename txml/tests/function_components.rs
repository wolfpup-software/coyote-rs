use txml::Injection::{AttrVal, List, Text, Tmpl};
use txml::{Injection, Template, txml};

fn woof() -> Template {
    txml(
        "<input type=submit value=\"yus -_-\">".to_string(),
        Vec::new(),
    )
}

fn woof_woof() -> Template {
    let descendants = List(Vec::from([
        Text("you're a boy kisser aren't you >:3".to_string()),
        Tmpl(woof()),
    ]));

    let attributes = List(Vec::from([
        AttrVal("action".to_string(), "/uwu".to_string()),
        AttrVal("method".to_string(), "post".to_string()),
    ]));

    txml(
        "<form {}>{}</form>".to_string(),
        Vec::from([attributes, descendants]),
    )
}
