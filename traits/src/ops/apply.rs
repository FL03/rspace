/*
    Appellation: apply <module>
    Created At: 2025.12.24:17:20:29
    Contrib: @FL03
*/
/// [`Apply`] defines an interface for objects capable of _applying_ the given function onto
/// itself or its elements to produce some output.
pub trait Apply<Rhs> {
    type Output;

    fn apply(&self, rhs: Rhs) -> Self::Output;
}
/// The [`ApplyOnce`] trait defines an interface for objects capable of consuming themselves
/// to apply the given function onto themselves or their elements to produce some output.
pub trait ApplyOnce<Rhs> {
    type Output;

    fn apply_once(self, rhs: Rhs) -> Self::Output;
}
/// The [`ApplyMut`] trait defines an interface for objects capable of mutably borrowing themselves
/// to apply the given function onto themselves or their elements to produce some output.
pub trait ApplyMut<Rhs> {
    type Output;

    fn apply_mut(&mut self, rhs: Rhs) -> Self::Output;
}

pub trait TryApply<Rhs> {
    type Output;
    type Error;

    fn try_apply(&self, rhs: Rhs) -> Result<Self::Output, Self::Error>;
}

pub trait TryApplyOnce<Rhs> {
    type Output;
    type Error;

    fn try_apply_once(self, rhs: Rhs) -> Result<Self::Output, Self::Error>;
}

pub trait TryApplyMut<Rhs> {
    type Output;
    type Error;

    fn try_apply_mut(&mut self, rhs: Rhs) -> Result<Self::Output, Self::Error>;
}

/*
 ************* Implementations *************
*/
impl<A, X, Y> TryApply<X> for A
where
    A: Apply<X, Output = Y>,
{
    type Output = A::Output;
    type Error = core::convert::Infallible;

    fn try_apply(&self, rhs: X) -> Result<Self::Output, Self::Error> {
        Ok(self.apply(rhs))
    }
}

impl<A, X, Y> TryApplyOnce<X> for A
where
    A: ApplyOnce<X, Output = Y>,
{
    type Output = A::Output;
    type Error = core::convert::Infallible;

    fn try_apply_once(self, rhs: X) -> Result<Self::Output, Self::Error> {
        Ok(self.apply_once(rhs))
    }
}

impl<A, X, Y> TryApplyMut<X> for A
where
    A: ApplyMut<X, Output = Y>,
{
    type Output = A::Output;
    type Error = core::convert::Infallible;

    fn try_apply_mut(&mut self, rhs: X) -> Result<Self::Output, Self::Error> {
        Ok(self.apply_mut(rhs))
    }
}

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
    use super::Apply;
    use alloc::allocator::Allocator;
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
    use super::Apply;
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
