use coyote::{attr_val, list, text, tmpl, Component};
use sieve::Sieve;

use parse::StepKind;
use template_string::{compose, Results};

// Test will not build if Function Components do not build

fn woof() -> Component {
    tmpl("<input type=submit value=\"yus -_-\">", [])
}

fn woof_woof() -> Component {
    let descendants = list([text("you're a boy kisser aren't you >:3"), woof()]);

    let attributes = list([attr_val("action", "/uwu"), attr_val("method", "post")]);

    tmpl("<form {}>{}</form>", [attributes, descendants])
}

#[test]
fn test_txml_builder() {
    let sieve = Sieve::new();

    let template = woof_woof();
    let expected = Results {
        strs: Vec::from(["<form ".to_string(), ">".to_string(), "</form>".to_string()]),
        injs: Vec::from([StepKind::AttrMapInjection, StepKind::DescendantInjection]),
    };

    if let Component::Tmpl(tmpl) = template {
        let results = compose(&sieve, &tmpl.template_str);
        assert_eq!(expected, results);
    }
}
