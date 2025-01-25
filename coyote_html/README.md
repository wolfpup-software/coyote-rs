# Coyote Html

## Install

```sh
cargo install --git https://github.com/wolf-pup/coyote-rs coyote
cargo install --git https://github.com/wolf-pup/coyote-rs coyote_html
```

### Hello, world!

The example below creates an html document from a coyote component function.

```rust
use coyote::{Component, tmpl};
use coyote_html::Html;

fn hai() -> Component {
    tmpl("<p>omgawsh hai :3</p>", [])
}

fn main() {
    let hello_world = hai();

    let html = Html::new();
    let document = html.build(&hello_world); 

    println!("{}", document);
}
```

The output will be:
```html
<p>hai :3</p>
```

### Hello, safer world!

The example below creates a _safer_ fragment for client-side renders using `ClientRules`. 

```rust
use coyote::{Component, tmpl};
use coyote_html::ClientHtml;

fn malicious_component() -> Component {
    tmpl("
        <link rel=stylesheet href=a_dangerous_stylesheet.css>
        <style>
            * { color: blue; }
        </style>
        <script>
            console.log('a malicious script! grrr rawr');
        </script>
        <p>omgawsh hai :3</p>
    ", [])
}

fn hai() -> Component {
    tmpl(
        "{}<p>omgawsh hai :3</p>",
        [malicious_component()],
    )
}

fn main() {
    let hello_world = hai();

    let safer_html = ClientHtml::new();    
    let document = safer_html.compose(&hello_world); 
    
    println!("{}", document);
}
```

The output will be:
```html
<p>hai :3</p>
```

`Coyote Html` guides template composition with `rulesets`.

`ClientRules` rejects elements like `<script>` and `<style>` and removes unneccessary spaces.

## License

`Coyote-rs` is released under the BSD-3-Clause License
