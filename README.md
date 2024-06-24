# Coyote-rs

Create `XML | HTML | FRAGMENTS` in Rust with component functions!

## About

`Coyote` creates xml-like documents from function components.

## Hello, world!

### HTML

The example below creates an html string template.

```rust
use coyote::{Component, tmpl};
use coyote::html::{Builder, Sieve, compose};

fn hai() -> Component {
    tmpl("<p>omgawsh hai :3</p>", [])
}

fn main() {
    let template = compose(Sieve, Builder, hai());
    println!("{}", template);
}
```

And the output will be:
```html
<p>omgawsh hai :3</p>
```

## Fragments

### HTML

The example below creates a little form. 

```rust
use coyote::{Component, attr_val, list, text, tmpl};
use coyote::html::{Builder, Sieve, compose};

fn woof() -> Component {
    tmpl("<input type=submit value=\"yus -_-\">", [])
}

fn woof_form() -> Component {
    let attributes = list([
        attr_val("action", "/uwu"),
        attr_val("method", "post"),
    ]);

    let descendants = list([
        text("you're a boy kisser aren't you >:3"),
        woof(),
    ]);

    tmpl(
        "<form {}>{}</form>",
        [attributes, descendants],
    )
}

fn main() {
    let template: String = compose(Sieve, Builder, woof_form());
    println!("{}", template);
}
```

And the output will be:
```html
<form action="/uwu" method="post">
    you're a boy kisser aren't you >:3"
    <input type=submit value="yus -_-">
</form>
```

## License

`Coyote-rs` is released under the BSD-3-Clause License
