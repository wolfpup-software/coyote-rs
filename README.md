# Coyote-rs

Create `HTML` documents with function components in Rust.

There are no dependencies. There are no macros.

## Install

```sh
cargo install --git https://github.com/w-lfpup/coyote-rs
```

## Components

Create documents with coyote [components](./components.md).

```rust
use coyote::{Component, tmpl};

fn hai() -> Component {
    tmpl("<p>hai :3</p>", [])
}
```

## Html

Render components as `html` with [document builders](./document_builders.md).

```rust
use coyote::{Component, Html, tmpl};

fn hai() -> Component {
    tmpl("<p>hai :3</p>", [])
}

fn main() {
    let hello_world = hai();
    
    let html = Html::new();

    if let Ok(document) = html.build(&hello_world) {
        println!("{}", document);
    }; 
}
```

The output will be:
```html
<p>hai :3</p>
```

## License

`Coyote-rs` is released under the BSD 3-Clause License.
