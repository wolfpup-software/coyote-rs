use coyote::{attr_val, list, text, tmpl, Component};
use coyote_html::{Builder, Html, ServerRules};

use std::sync::Arc;
use std::sync::Mutex;

fn woof() -> Component {
    tmpl("<input type=submit value=\"yus -_-\">", [])
}

fn woof_woof() -> Component {
    let descendants = list([text("you're a boy kisser aren't you >:3"), woof()]);

    let attributes = list([attr_val("action", "/uwu"), attr_val("method", "post")]);

    tmpl("<form {}>{}</form>", [attributes, descendants])
}

#[test]
fn test_coyote_html_with_arc_and_mutex() {
    let html = Html::from_builder(Builder::new());
    let arc = Arc::new(Mutex::new(html));
    let html_clone = arc.clone();

    let rules = ServerRules::new();
    let woof_form = woof_woof();
    if let Ok(mut html_mutex) = html_clone.lock() {
        let _results = html_mutex.build(&rules, &woof_form);
    };
}
