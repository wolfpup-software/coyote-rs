use coyote::{attr_val, list, text, tmpl, Component};

// Test will not build if Function Components do not build

fn woof() -> Component {
    tmpl("<input type=submit value=\"yus -_-\">", [])
}

fn woof_woof() -> Component {
    let descendants = list([text("you're a boy kisser aren't you >:3"), woof()]);

    let attributes = list([attr_val("action", "/uwu"), attr_val("method", "post")]);

    tmpl("<form {}>{}</form>", [attributes, descendants])
}

// use coyote::html::{Builder, Sieve, compose};
// use coyote::{Component, Html, attr_val, list, text, tmpl};

// fn hai() -> Component {
//     tmpl("<p>omgawsh hai :3</p>", [])
// }

// fn woof_form() -> Component {
//     let attributes = list([
//         attr_val("action", "/uwu"),
//         attr_val("method", "post"),
//     ]);

//     let descendants = list([
//         text("you're a boy kisser aren't you >:3"),
//         woof(),
//     ]);

//     tmpl(
//         "<form {}>{}</form>",
//         [attributes, descendants],
//     )
// }

// fn test_omgawsh() {
//     let template: String = compose(&Sieve::new(), Builder::new(), &hai());
//     println!("{}", template);
// }

// fn test_woof_form() {
//     let template: String = compose(&Sieve::new(), Builder::new(), &woof_form());
//     println!("{}", template);
// }
