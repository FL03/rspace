/*
    Appellation: rspace-core <library>
    Contrib: Joe McCain III <jo3mccain@icloud.com>
*/
//! # Core Modules
//! 
//! ## Configuration Space $(C)$
//!
//! Configuration space $C$, or c-space, is the set of all possible configurations $q$ of a 
//! system where $q \in Q$.
#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(feature = "alloc")]
extern crate alloc;

#[macro_use]
pub(crate) mod macros;
#[macro_use]
pub(crate) mod seal;

pub mod elem;
pub mod traits;
pub mod types;

pub mod prelude {
    pub use super::elem::prelude::*;
    pub use crate::traits::prelude::*;
    pub use crate::types::prelude::*;
}
