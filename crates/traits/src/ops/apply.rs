/*
    Appellation: apply <module>
    Created At: 2025.12.24:17:20:29
    Contrib: @FL03
*/
/// [`Apply`] establishes a binary operator for applying a given object, typically a function or
/// closure, onto a target object, producing some output.
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
