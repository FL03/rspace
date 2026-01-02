/*
    Appellation: container <module>
    Created At: 2025.12.29:14:39:20
    Contrib: @FL03
*/
use crate::RawSpace;

/// The [`Container`] trait is a higher-kinded trait used to establish an interface for
/// defining containers themselves.
pub trait Container<U>
where
    Self::Cont<U>: RawSpace<Elem = U>,
{
    type Cont<V>: ?Sized;
}

pub trait ContainerIndex<T, Idx>: Container<T>
where
    Self::Cont<T>: RawSpace<Elem = T> + core::ops::Index<Idx, Output = T>,
{
    type Output;

    fn get(&self, index: Idx) -> Option<&Self::Output>;
}

/// The [`ContainerIter`] trait extends the [`Container`] trait to provide an interface
/// for obtaining iterators over the elements of the container.
pub trait ContainerIter<T>: Container<T>
where
    Self::Cont<T>: RawSpace<Elem = T>,
{
    type Iter<'a, U>: Iterator<Item = &'a U>
    where
        Self: 'a,
        U: 'a;

    fn iter(&self) -> Self::Iter<'_, T>;
}

pub trait ContainerIterMut<T>: Container<T>
where
    Self::Cont<T>: RawSpace<Elem = T>,
{
    type Iter<'a, U>: Iterator<Item = &'a mut U>
    where
        Self: 'a,
        U: 'a;

    fn iter_mut(&mut self) -> Self::Iter<'_, T>;
}

impl<T> ContainerIter<T> for [T] {
    type Iter<'a, U>
        = core::slice::Iter<'a, U>
    where
        Self: 'a,
        U: 'a;

    fn iter(&self) -> Self::Iter<'_, T> {
        <[T]>::iter(self)
    }
}

impl<T> ContainerIterMut<T> for [T] {
    type Iter<'a, U>
        = core::slice::IterMut<'a, U>
    where
        Self: 'a,
        U: 'a;

    fn iter_mut(&mut self) -> Self::Iter<'_, T> {
        <[T]>::iter_mut(self)
    }
}

impl<const N: usize, T> ContainerIter<T> for [T; N] {
    type Iter<'a, U>
        = core::slice::Iter<'a, U>
    where
        Self: 'a,
        U: 'a;

    fn iter(&self) -> Self::Iter<'_, T> {
        <[T]>::iter(self)
    }
}

impl<const N: usize, T> ContainerIterMut<T> for [T; N] {
    type Iter<'a, U>
        = core::slice::IterMut<'a, U>
    where
        Self: 'a,
        U: 'a;

    fn iter_mut(&mut self) -> Self::Iter<'_, T> {
        <[T]>::iter_mut(self)
    }
}

impl<T> ContainerIter<T> for &[T] {
    type Iter<'a, U>
        = core::slice::Iter<'a, U>
    where
        Self: 'a,
        U: 'a;

    fn iter(&self) -> Self::Iter<'_, T> {
        <[T]>::iter(self)
    }
}

impl<T> ContainerIterMut<T> for &mut [T] {
    type Iter<'a, U>
        = core::slice::IterMut<'a, U>
    where
        Self: 'a,
        U: 'a;

    fn iter_mut(&mut self) -> Self::Iter<'_, T> {
        <[T]>::iter_mut(self)
    }
}
