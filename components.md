# Coyote-rs

Create `HTML` with component functions in Rust!

## Install

```sh
cargo install --git https://github.com/wolf-pup/coyote-rs coyote
```

## Components

`Coyote` creates documents from function components.

### Function Components

Function components are functions that return components!

```rust
use coyote::components::{Component, tmpl};

fn hai() -> Component {
    tmpl("<p>omgawsh hai :3</p>", [])
}
```

### Types of components

`Components` are used to build documents:

| Component | Description | Type |
| --------- | ---- | ----------- |
| Attribute | an element attribute | `attr(attribute_name: &str) -> Component` |
| Attribute with value | an element and attribute and value pair | `attr_val(&str, &str) -> Component` | 
| Text | text with escaped HTML glyphs like `<` of `{`| `text(&str) -> Component` |
| Unescaped text | dangerously unescaped text | `unescaped_text(&str) -> Component` |
| List | a list of components | `list([Component, ...]) -> Component` |
| Vector | a vector of components | `vlist(Vec::from([Component, ...])) -> Component` |
| Template | a document fragment described by a string template and a list or vector of injections | `tmpl(&str, [Component, ...]) -> Component` |
| None | the abscence of a component | `Component::None` |

## Templates

### Tags, void elements, fragments

`Coyote-rs` supports self-closing tags, void elements, and fragments.

```rs
fn syntax_story() -> Component {
    tmpl("
        <article>
            <header>cool story!</header>
            <>
                <p>bro what else happened?</p>
                <p>no waaaay?</p>
            </>
            <footer>end of the story!</footer>
        </article>
    ", [])
}
```

### Injections

`Injections` create nested templates and attribute assignments.

There are only two valid _injections_ in a `tmpl` component:
- attributes
- descendants

Likewise there are only two valid injection locations in a `tmpl` component:

```rs
fn injection_story() -> Component {
    let attribute = attr("uwu");
    let descendant = text("hai! :3")

    tmpl("
        <article {}>
            {}
        </article>
    ", [attribute, descendant])
}
```

Any other instance of `{}` in a template component will not be considered an injection.

## Nested components

The `list` component reflects the `node -> [node, text, node, ...]` heiarchy of an xml-like document.

The example below creates a form defined by lists of attributes, templates, and text.

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

## Components as an IMR

Components are not HTML or XML.

Components are a kind of (I)ntermediate (R)endering (F)ormat. They are the _potential_ for a document like HTML or XML.

## Renders

`Components` are not coupled to any particular markup language or output environment. This makes `coyote` an expressive way to create custom documents and object scenes from xml.

### HTML

Coyote supports [html](../document_builders.md).
