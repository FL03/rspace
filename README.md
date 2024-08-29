# rspace

[![crates.io](https://img.shields.io/crates/v/rspace.svg)](https://crates.io/crates/rspace)
[![docs.rs](https://docs.rs/rspace/badge.svg)](https://docs.rs/rspace)
[![license](https://img.shields.io/crates/l/rspace.svg)](https://crates.io/crates/rspace)

[![clippy](https://github.com/FL03/rspace/actions/workflows/clippy.yml/badge.svg)](https://github.com/FL03/rspace/actions/workflows/clippy.yml)
[![rust](https://github.com/FL03/rspace/actions/workflows/rust.yml/badge.svg)](https://github.com/FL03/rspace/actions/workflows/rust.yml)

***

_**The library is currently in the early stages of development and is not yet ready for production use.**_

rspace implements generic fields in support of so-called rulial space.

## Features

- [x] Feature 1

## Getting Started

### Building from the source

Start by cloning the repository

```bash
git clone https://github.com/FL03/rspace.git
cd rspace
```

#### _Building the project_

```bash
cargo build --all-features -r -v --workspace
```

#### _Running tests_

```bash
cargo test --all-features -r -v --workspace
```

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies.rspace]
features = []
version = "0.1.0"
```

### Examples

#### _Basic Usage_

```rust
    extern crate rspace;

    fn main() -> Result<(), Box<dyn std::error::Error>> {
        tracing_subscriber::fmt::init();
        tracing::info!("Welcome to {name}", name = rspace);


        Ok(())
    }
```

## Contributing

Pull requests are welcome. For major changes, please open an issue first
to discuss what you would like to change.

Please make sure to update tests as appropriate.
