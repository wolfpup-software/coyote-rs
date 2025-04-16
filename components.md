# Components

`Coyote` creates documents with function components.

## Function Components

Function components are functions that return components!

```rust
use coyote::components::{Component, tmpl};

fn hai() -> Component {
    tmpl("<p>omgawsh hai :3</p>", [])
}
```

## Types of components

`Components` are used to build documents:

| Component | Description | Type |
| --------- | ---- | ----------- |
| Attribute | an element attribute | `attr(name: &str) -> Component` |
| Attribute with value | an element and attribute and value pair | `attr_val(name: &str, value: &str) -> Component` | 
| Text | text with the HTML glyphs `<` and `"` escaped | `text(text_str: &str) -> Component` |
| Unescaped text | dangerously unescaped text | `unescaped_text(text_str: &str) -> Component` |
| List | a list of components | `list(component_list: [Component, ...]) -> Component` |
| Vector List | a vector of components | `vlist(component_vector_list: Vec<Component>) -> Component` |
| Template | a document fragment described by a string template and a list or vector of injections | `tmpl(template_str: &str, injections: [Component, ...]) -> Component` |
| None | the abscence of a component | `Component::None` |

## The template component

The most crtical component, the template component, is most likely the most approachable component syntax.

## Tags, void elements, fragments

`Coyote-rs` supports self-closing tags, void elements, and fragments in templates:

```rs
fn syntax_story() -> Component {
    tmpl("
        <article>
            <>
                <p>no waaaay?</p>
                <custom-element />
                <input type=button value=\"high five! \" />
            </>
        </article>
    ", [])
}
```

However, `coyote-rs` will only output valid and correct HTML5:

```html
<article>
    <p>
        no waaaay?
    </p>
    <custom-element></custom-element>
    <input value=button value="high-five">
</article>
```

This gives developers an expressive template syntax while upholding modern HTML5 standards.

## Injections

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

### Escape the `{` character

To use a `{` in a template without creating a descendant injection, use the html escape charactor for a left bracket.

So ...

```html
helloooo { world }
```

in a template would be:

```rust
tmpl("hellooo, &#123; world }"); 
```

## Nested components

The `list` and `vlist` components reflects the `node -> [node, text, node, ...]` heiarchy of an xml-like document.

The example below creates a form defined by lists of attributes, templates, and text.

```rust
use coyote::{Component, attr_val, list, text, tmpl};

fn submit_button() -> Component {
    tmpl("<input type=submit value=\"yus -_-\">", [])
}

fn form() -> Component {
    let attributes = list([
        attr_val("action", "/uwu"),
        attr_val("method", "post"),
    ]);

    let mut descendant_vec: Vec<Component> = Vec::new();
    descendant_vec.push(text("you're a boy kisser aren't you >:3"));
    descendant_vec.push(submit_button());
    
    let descendants = vlist(descendant_vec);

    tmpl(
        "<form {}>{}</form>",
        [attributes, descendants],
    )
}
```

And the output will something like:

```html
<form action="/uwu" method="post">
    you're a boy kisser aren't you >:3
    <input type=submit value="yus -_-">
</form>
```

## Components as an IMR

Components are not quite HTML or XML.

Components are a kind of (I)ntermediate (R)endering (F)ormat. They are the _potential_ for a document like HTML or XML.

## Render Components

`Components` are not coupled to any particular markup language or environment. This makes `coyote` a particularly expressive way to create custom documents from html and scenes from xml.

## Document builders

Coyote renders components with [document builders](./document_builders.md).
