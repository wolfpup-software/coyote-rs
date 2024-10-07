
## Coyote Html

## Install

```sh
cargo install --git https://github.com/wolf-pup/coyote-rs coyote_html
```

### Hello, world!

The example below creates an html document from a coyote component function.

```rust
use coyote::{Component, tmpl};
use coyote_html::Html

fn hai() -> Component {
    tmpl("<p>omgawsh hai :3</p>", [])
}

fn main() {
    let hello_world = hai();

    let html = Html::new();
    let sieve = Sieve::new();

    let document = html.compose(&sieve, &hello_world); 

    println!("{}", document);
}
```

### Hello, safe world!

The example below creates a _safer_ fragment for client-side renders. 

```rust
use coyote_html::{ClientSieve, pretty_html};

fn main() {
    let hello_world = "
    <article><script>
        console.log(\"do something dangerous\");</script>
    <style>* { color: transparent }</style>
    <p>hai :3
    <p></article>";
    
    println!("{}", pretty_html(ClientSieve::new(), document);
}
```

`Coyote Html` guides template composition with a `sieve`.

The `ClietSieve` rejects elements like `<script>` and `<style>`. It also removes unneccessary spaces.

The output will be:
```html
<article><p>hai :3</p></article>
```

## License

`Coyote-rs` is released under the BSD-3-Clause License
