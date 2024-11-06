
## Coyote Html

## Install

```sh
cargo install --git https://github.com/wolf-pup/coyote-rs coyote
cargo install --git https://github.com/wolf-pup/coyote-rs coyote_html
```

### Hello, world!

The example below creates an html document from a coyote component function.

```rust
use coyote::{Component, tmpl};
use coyote_html::{Html, ServerRules}

fn hai() -> Component {
    tmpl("<p>omgawsh hai :3</p>", [])
}

fn main() {
    let hello_world = hai();

    let html = Html::new();
    let rules = ServerRules::new();

    let document = html.compose(&rules, &hello_world); 

    println!("{}", document);
}
```

### Hello, safe world!

The example below creates a _safer_ fragment for client-side renders using `ClientRules`. 

```rust
use coyote_html::{Html, ClientRules};

fn dangerous_hai() -> Component {
    tmpl("<p>omgawsh hai :3</p>", [])
}


fn main() {
    let hello_world = hai();

    let html = Html::new();    
    let rules = ClientRules::new();

    let document = html.compose(&rules, &hello_world); 
    
    println!("{}", document);
}
```

`Coyote Html` guides template composition with `rulesets`.

The `ClientRules` ruleset rejects elements like `<script>` and `<style>` and removes unneccessary spaces.

The output will be:
```html
<article><p>hai :3</p></article>
```

## License

`Coyote-rs` is released under the BSD-3-Clause License
