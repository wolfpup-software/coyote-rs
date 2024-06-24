# Coyote-rs

(MORE README)

## Components

### More interesting hai :3

```rust
use coyote::html::{compose, Builder, Sieve};
use coyote::{attr_val, list, text, tmpl, Component};

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
    let html: String = compose(Sieve, Builder, woof_form());
    println!("{}", html);
}
```

And the output will be:
```html
<form action="/uwu" method="post">
    you're a boy kisser aren't you >:3"
    <input type=submit value="yus -_-">
</form>
```

## Sieves

A `sieve` defines how `coyote` reacts to build steps recieved from `parsley`.

The api for a `sieve` is defined as the following:

```
SafetySieve {
    respect_indentation(): bool
    banned_el(string): bool
    void_el(string): bool
    namespace_el(string): bool
    preserved_text_el(string): bool
    inline_el(string): bool
}
```

And in rust:

```rs
pub trait SafetySieve {
    fn respect_indentation(&self) -> bool;
    fn banned_el(&self, tag: &str) -> bool;
    fn void_el(&self, tag: &str) -> bool;
    fn namespace_el(&self, tag: &str) -> bool;
    fn preserved_text_el(&self, tag: &str) -> bool;
    fn inline_el(&self, tag: &str) -> bool;
}
```

These handful of functions help `tmxl` understand how to interpret elements in a template.

SafetySieve
* respect_indentation -> add indentation to template output
* banned_el -> skip this element and their descendants
* void_el -> dont add self closing tags or closing tags to this element 
* namespace_el -> establish a new namespace for a branch of elements (ie: MathML or SVG)
* preserved_text_el -> start a branch of elements without formating
* inline_el -> add element and ignore indentation

## Syntax and grammars

`Coyote` is built to be flexible. There's overlap between HTML and XML (and conveniences from JSX) but there are non-trival differences:
* empty elements in XML can have self closing tags `<element />`
* HTML does not have self closing tags `<element>`
* neither has JXS fragments `</>`.

`Coyote` supports all three.

```rs
    tmpl("
        <form>
            <p>hai?</>
            <input type=\"submit\" />
        </form>
        <>
            <p>hai!</p>
            <ul>
                <li>1</li>
                <li>2</li>
                <li>3</li>
            </ul>
        </>",
        [],
    )
```

The `tmpl()` function generates a nested Component structure. Whether empty elements or void elements are _composed_ depends on the rules of a `sieve`.

## License

`Coyote-rs` is released under the BSD-3-Clause License
