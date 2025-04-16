// Test will not build if function components do not build

use coyote::{attr_val, list, text, tmpl, vlist, Component, Html};

fn submit_button() -> Component {
    tmpl("<input type=submit value=\"yus -_-\">", [])
}

fn form() -> Component {
    let attributes = list([attr_val("action", "/uwu"), attr_val("method", "post")]);

    let mut descendant_vec: Vec<Component> = Vec::new();
    descendant_vec.push(text("you're a boy kisser aren't you >:3"));
    descendant_vec.push(submit_button());

    let descendants = vlist(descendant_vec);

    tmpl("<form {}>{}</form>", [attributes, descendants])
}

#[test]
fn coyote_api() {
    let template = form();

    let expected = "<form action=\"/uwu\" method=\"post\">\n\tyou're a boy kisser aren't you >:3\n\t<input type=submit value=\"yus -_-\">\n</form>";

    let mut html = Html::new();
    let results = html.build(&template);

    assert_eq!(Ok(expected.to_string()), results);
}
