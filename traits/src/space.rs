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
    /// The type of elements associated with the space
    type Elem;
}

/// [`RawSpaceMut`] is a trait that provides various mutable methods for accessing elements.
pub trait RawSpaceMut: RawSpace {}

/// [`RawSpaceRef`] is a trait that provides various read-only methods for accessing elements.
pub trait RawSpaceRef: RawSpace {}
/// [`SliceSpace`] is used to define sequential collections, spaces, or containers that can be 
/// viewed as slices.
pub trait SliceSpace: RawSpace {
    fn as_slice(&self) -> &[Self::Elem];
    /// returns a raw pointer to the underlying elements
    fn as_ptr(&self) -> *const Self::Elem {
        self.as_slice().as_ptr()
    }

    fn len(&self) -> usize {
        self.as_slice().len()
    }
}

pub trait SliceSpaceMut: SliceSpace {
    fn as_mut_slice(&mut self) -> &mut [Self::Elem];

    fn as_mut_ptr(&mut self) -> *mut Self::Elem {
        self.as_mut_slice().as_mut_ptr()
    }
}

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
