/*
    Appellation: rspace <library>
    Contrib: Joe McCain III <jo3mccain@icloud.com>
*/
//! # rspace
//!
//! rspace implements generic fields in support of so-called rulial space.
#![cfg_attr(not(feature = "std"), no_std)]
#![crate_name = "rspace"]

#[cfg(feature = "alloc")]
extern crate alloc;

pub use rspace_core::*;

#[macro_use]
pub(crate) mod macros;
#[macro_use]
pub(crate) mod seal;

pub mod prelude {
    pub use rspace_core::prelude::*;
}
