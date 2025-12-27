/*
    Appellation: core <library>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
//! this core components of the contained crate
#![allow(
    clippy::missing_safety_doc,
    clippy::module_inception,
    clippy::needless_doctest_main,
    clippy::upper_case_acronyms
)]
#![cfg_attr(not(feature = "std"), no_std)]
#![cfg_attr(feature = "nightly", feature(allocator_api))]
// compiler checks
#[cfg(not(any(feature = "std", feature = "alloc")))]
compile_error! {
    "Either the 'std' or 'alloc' feature must be enabled."
}
// external crates
#[cfg(feature = "alloc")]
extern crate alloc;
#[doc(inline)]
pub use rspace_traits as traits;
// macros
#[macro_use]
pub(crate) mod macros {
    #[macro_use]
    pub mod seal;
}
// modules
pub mod error;
// re-exports
#[doc(inline)]
pub use self::{
    error::{Error, Result},
    traits::prelude::*,
};
// prelude
#[doc(hidden)]
pub mod prelude {
    pub use rspace_traits::prelude::*;
}
