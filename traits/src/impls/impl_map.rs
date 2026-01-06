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

impl<'a, U, V, F> MapTo<F, V> for &'a Option<U>
where
    F: FnOnce(&U) -> V,
{
    type Cont<T> = Option<T>;
    type Elem = &'a U;

    fn apply(&self, f: F) -> Self::Cont<V> {
        self.as_ref().map(f)
    }
}

#[cfg(all(feature = "alloc", feature = "nightly"))]
mod impl_alloc {
    use crate::ops::map::{MapInto, MapTo};
    use alloc::alloc::Allocator;
    use alloc::vec::Vec;

    impl<U, V, A, F> MapInto<F, V> for Vec<U, A>
    where
        A: Allocator,
        F: FnMut(U) -> V,
        Vec<V, A>: FromIterator<V>,
    {
        type Cont<_T> = Vec<_T, A>;
        type Elem = U;

        fn apply(self, f: F) -> Self::Cont<V> {
            self.into_iter().map(f).collect()
        }
    }

    impl<'a, U, V, A, F> MapTo<F, V> for &'a Vec<U, A>
    where
        A: Allocator,
        F: FnMut(&U) -> V,
    {
        type Cont<_T> = Vec<_T, A>;
        type Elem = &'a U;

        fn apply(&self, f: F) -> Self::Cont<V> {
            self.iter().map(f).collect()
        }
    }
}

#[cfg(all(feature = "alloc", not(feature = "nightly")))]
mod impl_alloc {
    use crate::ops::{MapInto, MapTo};
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
    {
        type Cont<_T> = Vec<_T>;
        type Elem = &'a U;

        fn apply(&self, f: F) -> Self::Cont<V> {
            self.iter().map(f).collect()
        }
    }
}
