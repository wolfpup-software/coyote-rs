# html-rs

Create HTML from component functions in Rust!

## Hello, world

```rust
use html::Template;

fn my_component() -> Template {
    html(
        "<p>hai :3</p>",
        Vec::new(),
    )
}
```

## More interesting 

```rust
use txml::{txml, Template};
use txml::Injection::{Attr, AttrVal, Templ, Text, List};

fn my_submit_button() -> Template {
    txml(
        "<input type=submit value=\"yus :3\">".to_string(),
        Vec::new(),
    )
}

fn my_form() -> Template {
    let attributes = Vec::from([
        AttrVal("action".to_string(), "/uwu".to_string()),
        AttrVal("method".to_string(), "post".to_string()),
    ]);

    let descendants = Vec::from([
        Text("you're a boy kisser, aren't you >:3".to_string()),
        Templ(my_submit_button()),
    ]);

    txml("<form {}>{}</form>".to_string(),
        Vec::from([
            List(attributes),
            List(descendants),
        ]),
    )
}
```

## safety

Only templates are allowed html characters

Text will be escaped, all `<a>` becomes `&srb;a&srb;`