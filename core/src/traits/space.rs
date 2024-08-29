/*
    Appellation: space <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use crate::elem::RawPoint;

/// [RawSpace] describes
pub unsafe trait RawSpace<T> {
    type Elem: RawPoint<Elem = T>;
}

/// [Space] describes a space.
pub trait Space<T>: RawSpace<T> {}
