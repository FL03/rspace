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

pub trait ContainerIter<U>: Container<U>
where
    Self::Cont<U>: RawSpace<Elem = U>,
{
    type Iter<'a, T>: Iterator<Item = &'a T>
    where
        Self: 'a,
        T: 'a;
}

/*
 ************* Implementations *************
*/
impl<S, T> ContainerMap<T> for S
where
    S: Container<T, Cont<T> = S>,
    S::Cont<T>: RawSpace<Elem = T>,
{
    fn map<F, X>(&self, f: F) -> Self::Cont<X>
    where
        Self::Cont<T>: Apply<F, Output = Self::Cont<X>>,
        Self::Cont<X>: Sized,
    {
        <Self::Cont<T> as Apply<F>>::apply(self, f)
    }
}

impl<S, T> Container<T> for S
where
    S: RawSpace<Elem = T>,
{
    type Cont<V> = S;
}
