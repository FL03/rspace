/*
    Appellation: impl_map <module>
    Created At: 2026.01.01:21:59:39
    Contrib: @FL03
*/
use crate::ops::{MapInto, MapTo};

impl<U, V, F> MapInto<F, V> for Option<U>
where
    F: FnOnce(U) -> V,
{
    type Cont<T> = Option<T>;
    type Elem = U;

    fn apply(self, f: F) -> Self::Cont<V> {
        self.map(f)
    }
}

impl<'a, F, X, Y> MapInto<F, Y> for &'a Option<X>
where
    F: FnOnce(&X) -> Y,
{
    type Cont<U> = Option<U>;
    type Elem = &'a X;

    fn apply(self, f: F) -> Self::Cont<Y> {
        self.as_ref().map(f)
    }
}

impl<'a, U, V, F> MapTo<F, V> for Option<&'a U>
where
    for<'b> F: FnOnce(&'b U) -> V,
{
    type Cont<T> = Option<T>;
    type Elem = &'a U;

    fn apply(&self, f: F) -> Self::Cont<V> {
        self.map(f)
    }
}

#[cfg(feature = "ndarray")]
mod impl_ndarray {
    use super::{MapInto, MapTo};
    use ndarray::{Array, ArrayBase, Data, Dimension};

    impl<A, B, S, D, F> MapInto<F, B> for ArrayBase<S, D, A>
    where
        A: Clone,
        D: Dimension,
        S: Data<Elem = A>,
        F: Fn(A) -> B,
    {
        type Cont<V> = Array<V, D>;
        type Elem = A;

        fn apply(self, f: F) -> Self::Cont<B> {
            self.mapv(f)
        }
    }

    impl<A, B, S, D, F> MapTo<F, B> for ArrayBase<S, D, A>
    where
        A: Clone,
        D: Dimension,
        S: Data<Elem = A>,
        F: Fn(A) -> B,
    {
        type Cont<V> = Array<V, D>;
        type Elem = A;

        fn apply(&self, f: F) -> Self::Cont<B> {
            self.mapv(f)
        }
    }
}

#[cfg(all(feature = "alloc", feature = "nightly"))]
mod impl_alloc {
    use super::{MapInto, MapTo};
    use alloc::alloc::Allocator;
    use alloc::vec::Vec;

    impl<F, X, Y, A> MapInto<F, Y> for Vec<X, A>
    where
        A: Allocator,
        F: FnMut(X) -> Y,
        Vec<Y, A>: FromIterator<Y>,
    {
        type Cont<_T> = Vec<_T, A>;
        type Elem = X;

        fn apply(self, f: F) -> Self::Cont<Y> {
            self.into_iter().map(f).collect()
        }
    }

    impl<'a, F, X, Y, A> MapTo<F, Y> for &'a Vec<X, A>
    where
        A: Allocator,
        F: FnMut(&X) -> Y,
        Vec<Y, A>: FromIterator<Y>,
    {
        type Cont<_T> = Vec<_T, A>;
        type Elem = &'a X;

        fn apply(&self, f: F) -> Self::Cont<Y> {
            self.iter().map(f).collect()
        }
    }
}

#[cfg(all(feature = "alloc", not(feature = "nightly")))]
mod impl_alloc {
    use super::{MapInto, MapTo};
    use alloc::vec::Vec;

    impl<F, X, Y> MapInto<F, Y> for Vec<X>
    where
        F: FnMut(X) -> Y,
        Vec<Y>: FromIterator<Y>,
    {
        type Cont<_U> = Vec<_U>;
        type Elem = X;

        fn apply(self, f: F) -> Self::Cont<Y> {
            self.into_iter().map(f).collect()
        }
    }

    impl<'a, U, V, F> MapTo<F, V> for &'a Vec<U>
    where
        F: FnMut(&U) -> V,
        Vec<V>: FromIterator<V>,
    {
        type Cont<_T> = Vec<_T>;
        type Elem = &'a U;

        fn apply(&self, f: F) -> Self::Cont<V> {
            self.iter().map(f).collect()
        }
    }
}
