# Coyote-rs

Create `HTML` with function components in Rust .

There are no dependencies. There are no macros.

## Components

Create documents with `coyote` [components](./coyote/README.md).

```rust
use coyote::{Component, tmpl};

fn hai() -> Component {
    tmpl("<p>hai :3</p>", [])
}
```

## Html

Render components as [html](./coyote_html/README.md) with `coyote_html`.

```rust
use coyote::{Component, tmpl};
use coyote_html::{Html, Sieve, pretty_html}

fn hai() -> Component {
    tmpl("<p>hai :3</p>", [])
}

fn main() {
    let hello_world = hai();

    let html = Html::new()
    let document = html.compose(&hello_world);

    println!("{}", pretty_html(document, Sieve::new());
}
```

## License

`Coyote-rs` is released under the BSD 3-Clause License.
