/*
    Appellation: container <module>
    Created At: 2025.12.29:14:39:20
    Contrib: @FL03
*/
use crate::{Apply, RawSpace};

/// The [`Container`] trait is a higher-kinded trait used to establish an interface for
/// defining containers themselves.
pub trait Container<U>
where
    Self::Cont<U>: RawSpace<Elem = U>,
{
    type Cont<V>: ?Sized;
}

pub trait ContainerMap<T>: Container<T>
where
    Self::Cont<T>: RawSpace<Elem = T>,
{
    fn map<F, X>(&self, f: F) -> Self::Cont<X>
    where
        Self::Cont<T>: Apply<F, Output = Self::Cont<X>>,
        Self::Cont<X>: Sized;
}

/// The [`ContainerIter`] trait extends the [`Container`] trait to provide an interface
/// for obtaining iterators over the elements of the container.
pub trait ContainerIter<U>: Container<U>
where
    Self::Cont<U>: RawSpace<Elem = U>,
{
    type Iter<'a, T>: Iterator<Item = &'a T>
    where
        Self: 'a,
        T: 'a;

    fn iter(&self) -> Self::Iter<'_, U>;
}

