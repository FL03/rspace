/*
    Appellation: core <library>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
//! this modules defines the core pimitives and utilities for `rspace`
#![allow(
    unused_features,
    clippy::missing_safety_doc,
    clippy::module_inception,
    clippy::needless_doctest_main,
    clippy::upper_case_acronyms
)]
#![cfg_attr(not(feature = "std"), no_std)]
#![cfg_attr(all(feature = "nightly", feature = "alloc"), feature(allocator_api))]
// external crates
#[cfg(feature = "alloc")]
extern crate alloc;
// macros
#[macro_use]
pub(crate) mod macros {
    #[macro_use]
    pub mod seal;
}
// modules
pub mod error;
pub mod point;
// re-exports
#[doc(inline)]
pub use self::{
    error::{Error, Result},
    point::*,
};
// prelude
#[doc(hidden)]
pub mod prelude {
    pub use crate::point::*;
}
