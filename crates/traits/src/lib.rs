#![crate_name = "rspace_traits"]
//! Various traits used to establish a solid foundation for defining and manipulating
//! containers, spaces, fields, and other related abstractions. The core trait, [`RawSpace`],
//! is a fundamental building block for defining _spaces_ (i.e. containers containing elements
//! of a specific type). The [`Container`] trait builds upon [`RawSpace`] to provide a more
//! robust interface for containers and higher-kinded abstractions.
#![allow(
    clippy::missing_errors_doc,
    clippy::missing_safety_doc,
    clippy::module_inception,
    clippy::needless_doctest_main,
    clippy::should_implement_trait,
    clippy::upper_case_acronyms
)]
#![cfg_attr(not(feature = "std"), no_std)]
#![cfg_attr(feature = "nightly", feature(allocator_api))]
// // compiler check
// #[cfg(not(any(feature = "std", feature = "alloc")))]
// compile_error! { "either the \"std\" or \"alloc\" feature must be enabled" }
// macros
#[macro_use]
pub(crate) mod macros {
    #[macro_use]
    pub mod seal;
}
// external crates
#[cfg(feature = "alloc")]
extern crate alloc;
// modules
pub mod container;
pub mod functor;
pub mod space;
pub mod store;

mod impls {
    mod impl_apply;
    mod impl_container;
    mod impl_map;
    mod impl_space;
    mod impl_store;
}

pub mod ops {
    //! This module provides various operations traits and implementations for musical concepts
    #[doc(inline)]
    pub use self::{apply::*, get::*, map::*};

    mod apply;
    mod get;
    mod map;

    pub(crate) mod prelude {
        pub use super::apply::*;
        pub use super::get::*;
        pub use super::map::*;
    }
}
// re-exports
#[doc(inline)]
pub use self::{container::*, functor::*, ops::prelude::*, space::*, store::*};
// prelude
#[doc(hidden)]
pub mod prelude {
    pub use crate::container::*;
    pub use crate::functor::*;
    pub use crate::ops::prelude::*;
    pub use crate::space::*;
    pub use crate::store::*;
}
