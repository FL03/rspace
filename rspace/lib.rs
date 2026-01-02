#![crate_name = "rspace"]
#![crate_type = "lib"]
//! `rspace` is a fundmanetal crate looking to support containers via a set of well established
//! interfaces, traits, primitives, and other useful tools. The crate is working towards
//! becoming `no_std` compatible, seeking to support embedded and other resource constrained
//! environments. Additionally, the crate is extensively feature-gated to ensure it remains as
//! lightweight and modular as possible.
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
// declare external crates as modules
#[doc(inline)]
pub use rspace_traits as traits;
// re-exports
#[doc(inline)]
pub use rspace_core::*;
#[doc(inline)]
pub use rspace_traits::*;
// prelude
#[doc(hidden)]
pub mod prelude {
    pub use rspace_core::prelude::*;
    pub use rspace_traits::prelude::*;
}
