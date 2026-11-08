# Loomer

A cross-platform desktop GUI framework for Rust.

> **Status:** pre-release. The project is at `0.0.0`; APIs are still being designed and may change
> without notice.

## Crates

| Crate                                                                                    | Role                                                         |
|------------------------------------------------------------------------------------------|--------------------------------------------------------------|
| [`loomer`](https://github.com/emanyzwww/Loomer/tree/dev/loomer)                          | Facade — the only crate applications should depend on        |
| [`loomer-runtime`](https://github.com/emanyzwww/Loomer/tree/dev/crates/loomer-runtime)   | Application runtime: window lifecycle and the event loop     |
| [`loomer-platform`](https://github.com/emanyzwww/Loomer/tree/dev/crates/loomer-platform) | Platform layer: `winit`-backed windows and event translation |
| [`loomer-layout`](https://github.com/emanyzwww/Loomer/tree/dev/crates/loomer-layout)     | Layout engine (work in progress)                             |
| [`loomer-render`](https://github.com/emanyzwww/Loomer/tree/dev/crates/loomer-render)     | Rendering layer (work in progress)                           |
| [`loomer-core`](https://github.com/emanyzwww/Loomer/tree/dev/crates/loomer-core)         | Core data types: colors, geometry, elements, window options  |

The layered crates are implementation details of the framework and may be restructured without
notice. Application code depends on `loomer` only.

## Usage

```rust
use loomer::prelude::*;

fn main() -> loomer::Result<()> {
  loomer::run(WindowOptions::new("Hello").size(800.0, 600.0))
}
```

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](https://github.com/emanyzwww/Loomer/blob/dev/LICENSE-APACHE))
- MIT license ([LICENSE-MIT](https://github.com/emanyzwww/Loomer/blob/dev/LICENSE-MIT))

at your option.

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in
this project by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without
any additional terms or conditions.
