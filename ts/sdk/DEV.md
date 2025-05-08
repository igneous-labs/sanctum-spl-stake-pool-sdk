# Dev Notes

## Prefer `Box<[T]>`/`Box<str>` to `Vec<T>`/`String`

We usually dont need the resizeable feature of the latter.

The Box types should be smaller since they dont store capacity.

However, it seems like there are cases where using the latter actually results in smaller binary sizes. TODO: investigate this.

## `*UserArgs` uses `Box<str>` instead of `&str`

Because `wasm-bindgen` does not support lifetimes in functions yet.

## `&str` is the only type with `&` that can be used as `wasm_bindgen` function params

Even `Option<&str>` is unsupported.

## core `#[derive]`s are OK

Seems like `wasm-pack` does a prety good job of dead-code elimination for unused derives. Removing `#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]` from everything only decreased wasm bin size by 3 bytes.
