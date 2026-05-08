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
/// [`ScalarSpace`] defins a type of space that consists of a single element. This trait is
/// useful in that it generally allows for a tensor-like treatment of scalar values within
/// more complex mathematical structures.
pub trait ScalarSpace: RawSpace<Elem = Self> {
    private! {}
}
/// [`RawSpaceRef`] is a trait that provides various read-only methods for accessing elements.
pub trait RawSpaceRef: RawSpace {
    fn as_ptr(&self) -> *const Self::Elem;
}
/// [`RawSpaceMut`] is a trait that provides various mutable methods for accessing elements.
pub trait RawSpaceMut: RawSpace {
    /// returns a mutable pointer to the element currently within scope
    fn as_ptr_mut(&mut self) -> *mut Self::Elem;
}

/// [`SliceSpace`] is used to define sequential collections, spaces, or containers that can be
/// viewed as slices.
pub trait SliceSpace: RawSpaceRef {
    fn as_slice(&self) -> &[Self::Elem];

    fn len(&self) -> usize {
        self.as_slice().len()
    }

    fn is_empty(&self) -> bool {
        self.len() == 0
    }
}
/// [`SliceSpaceMut`] is used to define sequential collections, spaces, or containers that can be
pub trait SliceSpaceMut: SliceSpace + RawSpaceMut {
    /// returns a mutable slice of the elements
    fn as_mut_slice(&mut self) -> &mut [Self::Elem];
}

/*
 ************* Implementations *************
*/

impl<T> ScalarSpace for T
where
    T: RawSpace<Elem = Self>,
{
    seal! {}
}

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

impl<U, T> RawSpaceRef for &U
where
    U: RawSpaceRef<Elem = T>,
{
    fn as_ptr(&self) -> *const Self::Elem {
        U::as_ptr(*self)
    }
}

impl<U, T> RawSpaceRef for &mut U
where
    U: RawSpaceRef<Elem = T>,
{
    fn as_ptr(&self) -> *const Self::Elem {
        U::as_ptr(*self)
    }
}
