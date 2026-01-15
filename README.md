# rspace

[![crates.io](https://img.shields.io/crates/v/rspace?style=for-the-badge&logo=rust)](https://crates.io/crates/rspace)
[![docs.rs](https://img.shields.io/docsrs/rspace?style=for-the-badge&logo=docs.rs)](https://docs.rs/rspace)
[![GitHub License](https://img.shields.io/github/license/FL03/rspace?style=for-the-badge&logo=github)](LICENSE)

***

rspace is a library dedicated to providing robust abstractions for creating and working with containers (or spaces) within Rust. It aims to offer a flexible solution for managing collections of items, with a focus on safety, efficiency, and ease of use.

## Features

- [x] `RawSpace` - The core abstraction for defining spaces.
- [x] `Container<T>` - A higher-kinded trait establishing a common interface for all container types.

## Getting Started

To get started using `rspace`, you can run the following command to add it to your project:

```bash
cargo add rspace
```

or, you can manually add it to your `Cargo.toml`:

```toml
[dependencies.rspace]
features = []
version = "0.0.1"
```

### Basic Usage

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
