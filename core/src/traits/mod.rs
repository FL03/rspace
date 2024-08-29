/*
    Appellation: traits <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
//! # Traits
//! 
//! 
#[doc(inline)]
pub use self::space::RawSpace;

pub mod space;

pub(crate) mod prelude {
    pub use super::space::*;
}
