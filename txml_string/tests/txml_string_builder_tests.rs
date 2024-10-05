use coyote::{attr_val, list, text, tmpl, Component};
use parse::SieveImpl;

use txml_string::compose;

// Test will not build if Function Components do not build

pub struct TestSieve {}

impl TestSieve {
    fn new() -> TestSieve {
        TestSieve {}
    }
}

impl SieveImpl for TestSieve {
    fn is_comment(&self, tag: &str) -> bool {
        tag == "!--"
    }

    fn get_close_sequence_from_alt_text_tag(&self, tag: &str) -> Option<&str> {
        match tag {
            "script" => Some("</script>"),
            "style" => Some("</style>"),
            "!--" => Some("-->"),
            _ => None,
        }
    }

    fn get_tag_from_close_sequence(&self, tag: &str) -> Option<&str> {
        match tag {
            "</script>" => Some("script"),
            "</style>" => Some("style"),
            "-->" => Some("!--"),
            _ => None,
        }
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
fn test_txml_builder() {
    let sieve = TestSieve::new();

    let template = woof_woof();

    if let Component::Tmpl(tmpl) = template {
        let _results = compose(&sieve, &tmpl.template_str);
    }
}
