/*
    Appellation: transform <module>
    Created At: 2025.12.24:14:22:11
    Contrib: @FL03
*/
/// The [`Transform`] trait defines an interface for objects capable of being transformed by 
/// a given function, operation, or otherwise to produce some output.
pub trait Transform<Rhs> {
    type Output;

    fn transform(&self, rhs: Rhs) -> Self::Output;
}
/// [`TransformOnce`] defines an interface for objects capable of consuming themselves
/// to be transformed by a given function, operation, or otherwise to produce some output.
pub trait TransformOnce<Rhs> {
    type Output;

    fn transform_once(self, rhs: Rhs) -> Self::Output;
}
/// The [`TransformInplace`] trait defines an interface for objects capable of mutably borrowing
/// themselves to be transformed by a given function, operation, or otherwise.
pub trait TransformInplace<Rhs> {
    fn transform_inplace(&mut self, rhs: Rhs);
}

pub trait TryTransform<Rhs> {
    type Output;
    type Error;

    fn try_transform(&self, rhs: Rhs) -> Result<Self::Output, Self::Error>;
}

/*
 ************* Implementations *************
*/

impl<X, Y, Q> TryTransform<X> for Q
where
    Q: Transform<X, Output = Y>,
{
    type Output = Q::Output;
    type Error = core::convert::Infallible;

    fn try_transform(&self, rhs: X) -> Result<Self::Output, Self::Error> {
        Ok(self.transform(rhs))
    }
}
