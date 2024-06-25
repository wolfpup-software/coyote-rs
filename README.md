# Coyote-rs

Create `XML | HTML | FRAGMENTS` in Rust with component functions!

## Components

`Coyote` creates xml-like documents from function components.

```rust
use coyote::{Component, tmpl};

fn hai() -> Component {
    tmpl("<p>omgawsh hai :3</p>", [])
}
```

Componets are the atomic pieces required to build xml-like templates.
- `attr` -> attribute
- `attr_val` -> attribute with value
- `text` -> safer escaped text
- `unescaped_text` -> unsafe unescaped text
- `tmpl` -> a string template describing an xml-like document fragment 
- `list` -> a list of components

They can be nested, placed in a `list`, it's a nested structure that roughly reflects the `node -> [node, text, node, ...]` structure of an xml document. 

```rust
use coyote::{Component, attr_val, list, text, tmpl};

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
```

## HTML

### Hello, world!

The example below creates an html string template from a coyote component function.

```rust
use coyote::{Component, tmpl};
use coyote_html::{Html, Sieve}

fn hai() -> Component {
    tmpl("<p>omgawsh hai :3</p>", [])
}

fn main() {
    let hello_world = hai();

    let sieve = Sieve::new();
    let html = Html::new();

    let document = html.compose(&hello_world, &sieve);
    println!("{}", document);
}
```

And the output will be:
```html
<p>omgawsh hai :3</p>
```

## License

`Coyote-rs` is released under the BSD-3-Clause License
