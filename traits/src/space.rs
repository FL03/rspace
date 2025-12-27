/*
    Appellation: space <module>
    Created At: 2025.12.26:14:12:46
    Contrib: @FL03
*/
//! This module defines the [`RawSpace`] trait alongside other interfaces for immutable and
//! mutable access to the inner elements of a container.

/// The [`RawSpace`] trait is used to define a base interface for all containers whose elements
/// are of **one** specific type.
pub trait RawSpace {
    /// The type of elements associated with the container
    type Elem;
}

/// [`RawSpaceMut`] is a trait that provides various mutable methods for accessing elements.
pub trait RawSpaceMut: RawSpace {}

/// [`RawSpaceRef`] is a trait that provides various read-only methods for accessing elements.
pub trait RawSpaceRef: RawSpace {}

/*
 ************* Implementations *************
*/
impl<C, T> RawSpace for &C
where
    C: RawSpace<Elem = T>,
{
    type Elem = C::Elem;
}

impl<C, T> RawSpace for &mut C
where
    C: RawSpace<Elem = T>,
{
    type Elem = C::Elem;
}
