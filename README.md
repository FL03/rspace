# rspace

[![crates.io](https://img.shields.io/crates/v/rspace.svg)](https://crates.io/crates/rspace)
[![docs.rs](https://docs.rs/rspace/badge.svg)](https://docs.rs/rspace)
[![license](https://img.shields.io/crates/l/rspace.svg)](https://crates.io/crates/rspace)

[![clippy](https://github.com/FL03/rspace/actions/workflows/clippy.yml/badge.svg)](https://github.com/FL03/rspace/actions/workflows/clippy.yml)
[![rust](https://github.com/FL03/rspace/actions/workflows/rust.yml/badge.svg)](https://github.com/FL03/rspace/actions/workflows/rust.yml)

***

_**The library is currently in the early stages of development and is not yet ready for production use.**_

rspace is a library dedicated to providing robust abstractions for creating and working with containers (or spaces) within Rust. It aims to offer a flexible solution for managing collections of items, with a focus on safety, efficiency, and ease of use.

## Features

- [x] `RawSpace` - The core abstraction for defining spaces.
- [x] `Container<T>` - A higher-kinded trait for defining containers.
- [x] `Store<T>` - A trait for defining key-value stores.

## Getting Started

Add this to your `Cargo.toml`:

```toml
[dependencies.rspace]
features = []
version = "0.0.1"
```

### Examples

#### _Basic Usage_

```rust
    extern crate rspace;

    fn main() -> Result<(), Box<dyn std::error::Error>> {
        tracing_subscriber::fmt()
            .with_max_level(tracing::Level::INFO)
            .init();
        tracing::info! { "Welcome to {name}", name = "rspace" }

        Ok(())
    }
```

## Contributing

Pull requests are welcome. For major changes, please open an issue first
to discuss what you would like to change.

Please make sure to update tests as appropriate.
