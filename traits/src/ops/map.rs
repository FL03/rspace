/*
    Appellation: map <module>
    Created At: 2026.01.01:21:31:12
    Contrib: @FL03
*/
/// The [`MapTo`] trait defines an interface for containers able to _map_ or apply a given
/// function onto a reference to each of its constituting elements, producing a new container
/// with the results of those function applications. The construction of the trait itself
/// allows implementors to have granular controls over exactly which type of function and
/// output is being produced, relying on associated type parameters to define the container
/// and its _current_ element type.  
pub trait MapTo<T, F> {
    type Cont<U>;
    type Elem;

    fn map_to(&self, f: F) -> Self::Cont<T>;
}
/// [`MapInto`] describes a consuming interface for containers that enables the mapping of a
/// given function onto each element of the container. While the trait definition constrains
/// the generic function parameters, `F` to a `FnOnce` closure, implementors coulds choose
/// to strengthen this constraint to `FnMut` or `Fn` as needed.
pub trait MapInto<T, F>
where
    F: FnOnce(Self::Elem) -> T,
{
    type Cont<U>;
    type Elem;

    fn map_into(self, f: F) -> Self::Cont<T>;
}

/*
 ************* Implementations *************
*/
impl<F, X, Y> MapInto<Y, F> for Option<X>
where
    F: FnOnce(X) -> Y,
{
    type Cont<U> = Option<U>;
    type Elem = X;

    fn map_into(self, f: F) -> Self::Cont<Y> {
        self.map(f)
    }
}

impl<F, X, Y> MapTo<Y, F> for Option<X>
where
    F: FnOnce(&X) -> Y,
{
    type Cont<U> = Option<U>;
    type Elem = X;

    fn map_to(&self, f: F) -> Self::Cont<Y> {
        self.as_ref().map(f)
    }
}

#[cfg(all(feature = "alloc", feature = "nightly"))]
mod impl_alloc {
    use crate::ops::map::{MapInto, MapTo};
    use alloc::allocator::Allocator;
    use alloc::vec::Vec;

    impl<X, A, F, Y> MapInto<Y, F> for Vec<X, A>
    where
        A: Allocator,
        F: FnMut(X) -> Y,
    {
        type Cont<U> = Vec<U, A>;
        type Elem = X;

        fn map_into(self, f: F) -> Self::Cont<Y> {
            self.into_iter().map(f).collect()
        }
    }

    impl<X, A, F, Y> MapTo<Y, F> for Vec<X, A>
    where
        A: Allocator,
        F: FnMut(&X) -> Y,
    {
        type Cont<U> = Vec<U, A>;
        type Elem = X;

        fn map_to(&self, f: F) -> Self::Cont<Y> {
            self.iter().map(f).collect()
        }
    }
}

#[cfg(all(feature = "alloc", not(feature = "nightly")))]
mod impl_alloc {
    use crate::ops::map::{MapInto, MapTo};
    use alloc::vec::Vec;

    impl<F, X, Y> MapInto<Y, F> for Vec<X>
    where
        F: FnMut(X) -> Y,
    {
        type Cont<U> = Vec<U>;
        type Elem = X;

        fn map_into(self, f: F) -> Self::Cont<Y> {
            self.into_iter().map(f).collect()
        }
    }

    impl<F, X, Y> MapTo<Y, F> for Vec<X>
    where
        F: FnMut(&X) -> Y,
    {
        type Cont<U> = Vec<U>;
        type Elem = X;

        fn map_to(&self, f: F) -> Self::Cont<Y> {
            self.iter().map(f).collect()
        }
    }
}
