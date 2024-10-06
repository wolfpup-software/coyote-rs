use coyote::{attr_val, list, text, tmpl, Component};
use sieve::{Sieve, SieveImpl};
use template_string::{compose as compose_template, BuilderImpl};
use txml_string::{compose as compose_txml, Results};

// Test will not build if Function Components do not build

pub struct TxmlBuilder {}

impl TxmlBuilder {
    fn new() -> TxmlBuilder {
        TxmlBuilder {}
    }
}

impl BuilderImpl for TxmlBuilder {
    fn build(&mut self, sieve: &dyn SieveImpl, template_str: &str) -> Results {
        compose_txml(sieve, template_str)
    }
}

fn woof() -> Component {
    tmpl("<input type=submit value=\"yus -_-\">", [])
}

fn woof_woof() -> Component {
    let descendants = list([text("you're a boy kisser aren't you >:3"), woof()]);

    let attributes = list([attr_val("action", "/uwu"), attr_val("method", "post")]);

    tmpl("<form {}>{}</form>", [attributes, descendants])
}

#[test]
fn test_static_template_builder() {
    let sieve = Sieve::new();

    let mut builder = TxmlBuilder::new();
    let template = woof_woof();
    let _results = compose_template(&mut builder, &sieve, &template);
}
