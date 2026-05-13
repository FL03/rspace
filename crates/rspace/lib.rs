/*
    Appellation: rspace <library>
    Created At: 2026.05.13:06:57:09
    Contrib: @FL03
*/
//! Welcome to `rspace`, a crate dedicated to providing a comprehensive suite of tools and 
//! interfaces for working with containers in Rust. This library is designed to be as flexible 
//! as possible leveraging feature flags to enable specific functionality whenever needed, 
//! allowing users to tailor the crate to their specific needs without incurring unnecessary 
//! overhead.
//! 
//! ## Features
//! 
//! The `rspace` crate focuses on defining a generic [`Container`] trait used to establish a 
//! base interface that any *container* type can implement while providing us with an ability 
//! to then define generic representations, operations, and otherwise relating to containers.
//! 
#![allow(
    clippy::missing_errors_doc,
    clippy::missing_safety_doc,
    clippy::module_inception,
    clippy::needless_doctest_main,
)]
#![cfg_attr(not(feature = "std"), no_std)]
#![cfg_attr(all(feature = "alloc", feature = "nightly"), feature(allocator_api))]
// external crates
#[cfg(feature = "alloc")]
extern crate alloc;
// declare external crates as modules
#[doc(inline)]
pub use rspace_traits as traits;
// re-exports
#[doc(inline)]
pub use rspace_core::*;
#[doc(inline)]
pub use rspace_traits::prelude::*;
// prelude
#[doc(hidden)]
pub mod prelude {
    pub use rspace_core::prelude::*;
    pub use rspace_traits::prelude::*;
}
