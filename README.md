# txml-rs

Create `XML | HTML | FRAGMENTS` from component template functions in Rust!

## Components

### Hai :3

```rust
use txml::{txml, Component};

fn hai() -> Component {
    txml("<p>omgawsh hai :3</p>", []);
}
```

### More interesting hai :3

```rust
use txml::{Component, attr_val, list, text, txml};

fn woof() -> Component {
    txml("<input type=submit value=\"yus -_-\">", [])
}

fn woof_woof() -> Component {
    let descendants = list([
        text("you're a boy kisser aren't you >:3"),
        woof(),
    ]);

    let attributes = list([
        attr_val("action", "/uwu"),
        attr_val("method", "post"),
    ]);

    txml(
        "<form {}>{}</form>",
        [attributes, descendants],
    )
}
```
