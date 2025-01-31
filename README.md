# Coyote-rs

Create `HTML` with function components in Rust.

There are no dependencies. There are no macros.

## Install

```sh
cargo install --git https://github.com/wolf-pup/coyote-rs coyote
```

## Components

Create documents with coyote [components](./components.md).

```rust
use coyote::components::{Component, tmpl};

fn hai() -> Component {
    tmpl("<p>hai :3</p>", [])
}
```

## Html

Render components as `html` with document [builders](./document_builders.md).

```rust
use coyote::components::{Component, Html, tmpl};
use coyote::html::Html;

fn hai() -> Component {
    tmpl("<p>hai :3</p>", [])
}

fn main() {
    let hello_world = hai();

    let html = Html::new();
    let document = html.compose(&hello_world); 

    println!("{}", document);
}
```

## License

`Coyote-rs` is released under the BSD 3-Clause License.
