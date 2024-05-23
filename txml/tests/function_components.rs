use txml::{attrVal, list, text, tmpl};
use txml::{Injection, Template, txml};

fn woof() -> Template {
    txml(
        "<input type=submit value=\"yus -_-\">",
        [],
    )
}

fn woof_woof() -> Template {
    let descendants = list([
        text("you're a boy kisser aren't you >:3"),
        tmpl(woof()),
    ]);

    let attributes = list([
        attrVal("action", "/uwu"),
        attrVal("method", "post"),
    ]);

    txml(
        "<form {}>{}</form>",
        [attributes, descendants],
    )
}
