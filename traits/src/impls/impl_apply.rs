/*
    Appellation: impl_apply <module>
    Created At: 2026.01.01:21:39:28
    Contrib: @FL03
*/
use crate::ops::apply::{Apply, ApplyMut, ApplyOnce};

impl<U, V, F> ApplyOnce<F> for Option<U>
where
    F: FnOnce(U) -> V,
{
    type Output = Option<V>;

    fn apply_once(self, rhs: F) -> Self::Output {
        self.map(rhs)
    }
}

impl<U, V, F> Apply<F> for Option<U>
where
    F: Fn(&U) -> V,
{
    type Output = Option<V>;

    fn apply(&self, rhs: F) -> Self::Output {
        self.as_ref().map(rhs)
    }
}

impl<U, V, F> ApplyMut<F> for Option<U>
where
    F: FnMut(&mut U) -> V,
{
    type Output = Option<V>;

    fn apply_mut(&mut self, rhs: F) -> Self::Output {
        self.as_mut().map(rhs)
    }
}

impl<const N: usize, U, V, F> Apply<F> for [U; N]
where
    F: Fn(&U) -> V,
{
    type Output = [V; N];

    fn apply(&self, rhs: F) -> Self::Output {
        core::array::from_fn(|i| rhs(&self[i]))
    }
}

#[cfg(all(feature = "alloc", feature = "nightly"))]
mod impl_alloc {
    use crate::ops::Apply;
    use alloc::alloc::Allocator;
    use alloc::boxed::Box;
    use alloc::vec::Vec;

    impl<U, V, F, A> Apply<F> for Box<U, A>
    where
        A: Allocator,
        F: Fn(&U) -> V,
    {
        type Output = Box<V>;

        fn apply(&self, rhs: F) -> Self::Output {
            Box::new(rhs(self.as_ref()))
        }
    }

    impl<U, V, F, A> Apply<F> for Vec<U, A>
    where
        A: Allocator,
        F: Fn(&U) -> V,
    {
        type Output = Vec<V, A>;

        fn apply(&self, rhs: F) -> Self::Output {
            self.iter().map(rhs).collect()
        }
    }
}
#[cfg(all(feature = "alloc", not(feature = "nightly")))]
mod impl_alloc {
    use crate::ops::Apply;
    use alloc::boxed::Box;
    use alloc::vec::Vec;

    impl<U, V, F> Apply<F> for Box<U>
    where
        F: Fn(&U) -> V,
    {
        type Output = Box<V>;

        fn apply(&self, rhs: F) -> Self::Output {
            Box::new(rhs(self.as_ref()))
        }
    }

    impl<U, V, F> Apply<F> for Vec<U>
    where
        F: Fn(&U) -> V,
    {
        type Output = Vec<V>;

        fn apply(&self, rhs: F) -> Self::Output {
            self.iter().map(rhs).collect()
        }
    }
}
