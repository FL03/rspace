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
