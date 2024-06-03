# Txml

Template XML

## abstract

This is a bundle of `enums`, `structs` and helper `functions` to help create `components`.

`Function Components` are functions that return components:

```rust
use txml::{txml, Component};

fn woof() -> Component {
    txml("<p>hai :3</p>", [])
}
```

There isn't much to test.

Nested functions create nested templates.

Generally, other devs have warned against "trees" and "nodes" in rust but 

It'd be wonderful to output a linear array of components, but I don't know how else to represent the function calls without:
- doing some serious macro magic
- very opinionated lazy static
- passing a ubiquitous c-like `ctx` object to every function

```rust
use txml::{txml, Component};

fn woof(ctx) -> Component {
    ctx.txml("<p>hai :3</p>", [])
}
```