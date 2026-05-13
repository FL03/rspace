# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Commands

```bash
# Format
cargo fmt --all

# Lint (treat warnings as errors)
cargo clippy --workspace --features full -- -D warnings

# Check (fast type-check, no codegen)
cargo check --lib --locked --workspace --features full

# Build
cargo build --release --lib --locked --workspace --features full

# Test
cargo test --release --locked --workspace --features full

# Run a single test
cargo test --package rspace-traits --features full -- <test_name>

# Benchmark
cargo bench --all-features --verbose --workspace

# Docs (private items included)
cargo doc --locked --workspace --no-deps --document-private-items

# Sprint cleanup (fmt + lint + fix + build + doc by default)
./scripts/cargo.sh
./scripts/cargo.sh --test          # also run tests
./scripts/cargo.sh --all           # everything including clean
```

Feature combinations to verify when touching feature-gated code:

```bash
cargo check -p rspace-traits --no-default-features
cargo check -p rspace-traits --no-default-features --features alloc
cargo check -p rspace-traits --features full
```

## Architecture

This is a 3-crate workspace publishing a single `rspace` crate. The dependency flow is strictly one direction:

```
rspace  →  rspace-core  →  rspace-traits
```

### `crates/traits` (`rspace-traits`)

The foundational layer — `no_std` compatible, no internal dependencies beyond `num-traits` and `paste`. Defines every core trait:

- **`RawSpace`** — the root trait: associates an `Elem` type with a container. Blanket-implemented on primitives, slices, std/alloc/hashbrown/ndarray collections (all cfg-gated).
- **`Container<U>`** / `ContainerIter<T>` / `ContainerIterMut<T>` — higher-kinded trait shape using GATs (`type Cont<V>`, `type Iter<'a, U>`).
- **`RawStore<K, V>`** / `Store<K, V>` — key-value store abstraction.
- **`ops`** module — composable operator traits: `Apply`/`ApplyOnce`/`ApplyMut` and their `Try*` variants; `MapInto`/`MapTo`; `Get`.
- **`Functor<F, T>`** — deprecated in favor of `MapInto`/`MapTo`.

The `mod impls` subdirectory is **private** and contains all std/alloc/external-type implementations. Nothing in `mod impls` is exported; it only provides impls for the public traits.

### `crates/core` (`rspace-core`)

Concrete implementations built on `rspace-traits`. Exports:

- **`Point<X, Y>`** — generic 2D coordinate type with `#[repr(C)]`. Comes with type aliases: `PointView<'a, X, Y>`, `PointViewMut<'a, X, Y>`, `RawPoint<X, Y>`, `RawPointMut<X, Y>`. Methods split across three `mod impl_point*` files.
- **`Error`** / **`Result`** — crate error type via `thiserror`.

### `crates/rspace` (`rspace`)

Thin facade crate — the published library. Re-exports everything from `rspace-core` and `rspace-traits::prelude`. Also exposes `rspace::traits` as a module alias for `rspace_traits`.

## Key Patterns

### Sealed traits

Both `rspace-traits` and `rspace-core` define a `pub(crate) mod macros::seal` module with `private!{}` and `seal!{}` macros. Use them to prevent downstream implementations:

```rust
pub trait ScalarSpace: RawSpace<Elem = Self> {
    private! {}  // prevents external impl
}

impl<T> ScalarSpace for T where T: RawSpace<Elem = Self> {
    seal! {}     // satisfies the private method
}
```

### Feature flag tiers

All crates follow the `no_std` / `alloc` / `std` ladder. `default = ["std"]`, `full` adds `complex`, `json`, `rand`, `serde`. When adding impls for external types, gate the impl block with the matching feature flag and propagate from `rspace` → `rspace-core` → `rspace-traits`.

### `mod impls` convention

Trait implementations for std/external types live in a private `mod impls { ... }` block (never exported). Each submodule is named `impl_<trait_name>.rs`. This keeps `src/lib.rs` clean and avoids mixing trait definitions with their std impls.

### Edition & MSRV

Edition 2024, MSRV 1.95.0 (set in `clippy.toml` and `Cargo.toml`). The `nightly` feature enables `allocator_api` and `hashbrown/nightly`.

### Profile notes

`dev` profile uses `opt-level = 2` and `lto = "thin"` (faster dev iteration at near-release speed). `release` uses `opt-level = "z"` (size-optimized) and `lto = "fat"`. Both use `panic = "abort"`.
