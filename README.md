# Coyote-rs

Create `HTML` with function components in Rust.

There are no dependencies. There are no macros.

## Install

```sh
cargo install --git https://github.com/wolf-pup/coyote-rs coyote
```

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
use coyote_html::{Html, Sieve}

fn main() {
    let hello_world = hai();

    let html = Html::new();
    let sieve = Sieve::new();

    let document = html.compose(&sieve, &hello_world); 

    println!("{}", document);
}
```

## UPDATE

What if i just said hey it's all strings? we are going from string to strings.

## License

`Coyote-rs` is released under the BSD 3-Clause License.
