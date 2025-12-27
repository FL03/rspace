/*
    Appellation: rspace <library>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
//! `rspace` works to establish a solid foundation for handling and defining containers, space,
//! and fields within Rust.
#![allow(
    clippy::missing_errors_doc,
    clippy::missing_safety_doc,
    clippy::module_inception,
    clippy::needless_doctest_main,
    clippy::should_implement_trait,
    clippy::upper_case_acronyms
)]
#![cfg_attr(not(feature = "std"), no_std)]
// compiler check
#[cfg(not(any(feature = "std", feature = "alloc")))]
compile_error! { "either the \"std\" or \"alloc\" feature must be enabled" }
// external crates
#[cfg(any(feature = "alloc", feature = "std"))]
extern crate alloc;
// re-exports
pub use rspace_core::*;
#[cfg(feature = "derive")]
pub use rspace_derive::*;
#[cfg(feature = "macros")]
pub use rspace_macros::*;
// prelude
#[doc(hidden)]
pub mod prelude {
    pub use rspace_core::prelude::*;
    #[cfg(feature = "derive")]
    pub use rspace_derive::*;
    #[cfg(feature = "macros")]
    pub use rspace_macros::*;
}
