use parsley::{get_text_from_step, parse_template_str, Step, StepKind};
use static_txml_builder::{TxmlBuilder, TxmlBuilderResults};
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

#[test]
fn test_txml_builder() {
    let template = woof_woof();
    let builder = TxmlBuilder::new();

    if let Component::Tmpl(tmpl) = template {
        let results = builder.build(&tmpl.template_str);
        println!("{:?}", results);
    }
}
