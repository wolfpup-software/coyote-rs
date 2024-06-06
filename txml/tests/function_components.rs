use txml::{attr_val, list, text, txml, Component};

// Test will not build if Function Components do not build

fn woof() -> Component {
    txml("<input type=submit value=\"yus -_-\">", [])
}

fn woof_woof() -> Component {
    let descendants = list([text("you're a boy kisser aren't you >:3"), woof()]);

    let attributes = list([attr_val("action", "/uwu"), attr_val("method", "post")]);

    txml("<form {}>{}</form>", [attributes, descendants])
}
