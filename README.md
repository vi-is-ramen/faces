# faces

[![Crates.io](https://img.shields.io/crates/v/faces.svg)](https://crates.io/crates/faces)
[![Docs.rs](https://docs.rs/faces/badge.svg)](https://docs.rs/faces)
[![License](https://img.shields.io/badge/license-MIT)](https://github.com/vi-is-ramen/faces#license)
[![License](https://img.shields.io/badge/Apache--2.0-blue.svg)](https://github.com/vi-is-ramen/faces#license)

**Faces** provides primitive types and a collection of ready‑to‑use traits that help
unify interfaces between different crates and software components. It makes it
easier to write adapters between two otherwise incompatible APIs.

> The name reflects the idea of a “face” – a common interface that different
> systems can present to each other.

## Where to learn more

This crate is intentionally designed to be **infinitely extended**, so it doesn't
document every single trait or type here. Instead, please refer to the dedicated
book article that explains the concepts and usage in depth:

&rarr; **[Faces – The Book](https://vi-is-ramen.github.io/book/en/my-crates/faces.html)**

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
faces = "0.1"
```

Then, in your code, import the traits you need:

```rust
use faces::*;

// Example: Use the conversion traits
let pfn = PageFrameNumber::new(42);
let addr: PhysicalAddress = pfn.into();
```

## Features

* **`std`** – enabled by default. Disable it for `no_std` environments.
* **`serde`** – adds `Serialize` / `Deserialize` derives for address types.
* **`log`** – re‑exports the `log::Log` trait.

## Contributing

Pull requests and any kind of contribution are very welcome! If you have a
useful trait that others might benefit from, feel free to open a PR.

## License

This crate is dual‑licensed under the **MIT** license or the **Apache 2.0**
license, at your option.
