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
use coyote::{Component, tmpl};

fn hai() -> Component {
    tmpl("<p>omgawsh hai :3</p>", [])
}
```

### Types of components

`Components` are atomic pieces used to build a template:

| Component | Type | Description |
| --------- | ---- | ----------- |
| Attribute | `attr(&str) -> Component`| element attribute |
| Attribute with value | `attr_val(&str, &str) -> Component` | element attribute and value |
| Safer escaped text | `text(&str) -> Component` | text with glyphs like `<>` escaped as HTML
| Unsafe unescaped text | `unescaped_text(&str) -> Component` | dangerously unescaped text |
| Template | `tmpl(&str, [Component, ...]) -> Component` | a string template describing a document fragment |
| List | `list([Component, ...]) -> Component` | a list of components |
| Vector | `vlist(Vec::from([Component, ...])) -> Component` | a vector list of components |
| None | `Component::None` | a component signifying the abscence of a component |

## Template Syntax

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

`Injections` enable component nesting and attribute assignments.

Likewise, there are only two valid _injections_ in a `tmpl` component:
- attributes
- descendants

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

Any other instance of `{}` will not be considered an injection.

## Nested components

The `list` component reflects the `node -> [node, text, node, ...]` heiarchy of an xml document.

The example below shows a form defined by lists of attributes, templates, and text.

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

## Renders

`Components` are not coupled to any particular markup language or output environment. This makes `coyote` an expressive way to create custom documents and object scenes from xml.

Currently Coyote ships with support for [html](../coyote_html/README.md).
