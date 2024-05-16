# concat_identifiers!

At the moment, nightly has a macro called `concat_idents!` that does the same thing. However, it's not stable, and will never be stable (as it is now), because it is useless.

This crate is just like that.

Long story short, I wanted C's paste (`##`) operator to make some fancy generated functions, and this is not it, because nested macros are not expanded depth-first.

## Usage

./tests/main.rs
---

```rust
fn what_function() -> &'static str {
    return "This function";
}

assert_eq!(concat_identifiers!(what, _function)(), "This function");
```

Thats kinda all I could think of. I encourage you to play around with it and see if this macro serves any purpose at all.

Either way. It's my first time making a procedural macro, so it's pretty neat.
