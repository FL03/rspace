/*
    Appellation: impl_map <module>
    Created At: 2026.01.01:21:59:39
    Contrib: @FL03
*/
use crate::ops::map::{MapInto, MapTo};

impl<F, X, Y> MapInto<F, Y> for Option<X>
where
    F: FnOnce(X) -> Y,
{
    type Cont<U> = Option<U>;
    type Elem = X;

    fn map_into(self, f: F) -> Self::Cont<Y> {
        self.map(f)
    }
}

impl<'a, F, X, Y> MapInto<F, Y> for &'a Option<X>
where
    F: FnOnce(&X) -> Y,
{
    type Cont<U> = Option<U>;
    type Elem = &'a X;

    fn map_into(self, f: F) -> Self::Cont<Y> {
        self.as_ref().map(|x| f(x))
    }
}

impl<F, X, Y> MapTo<F, Y> for Option<X>
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
    use alloc::alloc::Allocator;
    use alloc::vec::Vec;

    impl<X, A, F, Y> MapInto<F, Y> for Vec<X, A>
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

    impl<X, A, F, Y> MapTo<F, Y> for Vec<X, A>
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

    impl<F, X, Y> MapInto<F, Y> for Vec<X>
    where
        F: FnMut(X) -> Y,
    {
        type Cont<U> = Vec<U>;
        type Elem = X;

        fn map_into(self, f: F) -> Self::Cont<Y> {
            self.into_iter().map(f).collect()
        }
    }

    impl<F, X, Y> MapTo<F, Y> for Vec<X>
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
