# Coyote-rs

(MORE README)

## HTML

### Fragments

The example below creates a little form. 

```rust
use coyote::{Component, Html, attr_val, list, text, tmpl};

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
    let form = woof_form();

    let html = Html::new();
    // let html = Html::from_builder(builder);

    let document = html.compose(&sieve, &form);
    println!("{}", document);
}
```

And the output will be:
```html
<form action="/uwu" method="post">
    you're a boy kisser aren't you >:3"
    <input type=submit value="yus -_-">
</form>
```

## Syntax and grammars

There is significant overlap between HTML and XML and JSX but there are also non-trival differences:
* empty elements in XML can have self closing tags `<element />`
* Self closing tags are not valid html
* HTML void elements are empty elements without self closing tags
* There are only a handful of void elements in HTML
* All non void elements elements must have closing tags `<element></element>`
* HTML and XML do not support JXS fragments `</>`.

`Coyote` doesn't care.

How a component is _composed_ depends on what `builder` and `sieve` is used.

```rs
    tmpl("
        <form>
            <p>hai?</p>
            <input type=submit />
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

## Runtime steps

The `tmpl()` function generates nested `Components`. 

How templates are _composed_ depends on the type of `builder` and the rules of a `sieve`.

```html
<form>
    <p>hai?</p>
    <input type=submit>
</form>
<p>hai!</p>
<ul>
    <li>1</li>
    <li>2</li>
    <li>3</li>
</ul>
```

## HTML
```
SafetySieve {
    banned_el(string): bool
    preserved_text_el(string): bool
    text_descendants_only(string): bool
}
```
### Sieves

A `sieve` defines how `coyote` reacts to component build steps. It can _ban_ elements and direct behavior based on tag names. This makes component functions useful across multiple environments.

Consider a server and client. A server will generate entire documents for a someone's first visit but lazily load document fragments on certain interactions. The initial document will most likely include inline and external CSS and JS.

However, lazily loading CSS and JS while fetching document fragments can generate side-effects outside of document structure. That's bad!

So a `sieve` lets a developer re-use components with CSS and JS removed from a template when required.

The provided `HtmlSieve` does exactly that.

```rust
use coyote::{Component, tmpl};
use coyote_html::{Html, HtmlSieve};

fn hai() -> Component {
    tmpl("<p>omgawsh hai :3</p>", [])
}

fn main() {
    let hello_world = hai();

    let html = Html::new();
    let sieve = HtmlSieve::new();
    
    let document = html.compose(&sieve, &hello_world);

    println!("{}", document);
}
```

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


## License

`Coyote-rs` is released under the BSD-3-Clause License
