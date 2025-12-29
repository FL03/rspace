/*
    Appellation: hkt <module>
    Contrib: @FL03
*/

/// The [`Functor`] trait describes an interface for a higher-kinded type that can be mapped 
/// over. The trait is parameterized over a function `F` and a target type `T`, allowing for 
/// granular control over the mapping process itself, relying on associated types like
/// `Cont<U>` to define the resulting container type after the mapping operation and the 
/// `Elem` type to specify the type of elements contained within the functor _**before**_
/// the mapping operation is applied. Moreover, the `apply` method takes ownership of the 
/// functor allowing for distinct implementations for referenced, mutabled, and owned 
/// instances.
pub trait Functor<F, T>
where
    F: FnOnce(Self::Elem) -> T,
{
    type Cont<U>: ?Sized;
    type Elem;

    fn apply(self, f: F) -> Self::Cont<T>;
}

// pub trait Applicative<T>: Functor<T> {
//     fn pure(value: T) -> Self::Cont<T>;
// }

// pub trait Monad<T>: Applicative<T> {
//     fn flat_map<F, U>(&self, f: F) -> Self::Cont<U>
//     where
//         F: Fn(&mut T, &T) -> Self::Cont<U>;
// }

/*
 *************  Implementations  *************
*/
impl<F, X, Y> Functor<F, Y> for Option<X>
where
    F: FnOnce(X) -> Y,
{
    type Cont<U> = Option<U>;
    type Elem = X;

    fn apply(self, f: F) -> Self::Cont<Y> {
        self.map(f)
    }
}

impl<'a, F, X, Y> Functor<F, Y> for &'a Option<X>
where
    for<'b> F: FnMut(&'b X) -> Y,
{
    type Cont<U> = Option<U>;
    type Elem = &'a X;

    fn apply(self, f: F) -> Self::Cont<Y> {
        self.as_ref().map(f)
    }
}

impl<F, X, Y> Functor<F, Y> for Vec<X>
where
    F: FnMut(X) -> Y,
{
    type Cont<U> = core::iter::Map<std::vec::IntoIter<X>, F>;
    type Elem = X;

    fn apply(self, f: F) -> Self::Cont<Y> {
        self.into_iter().map(f)
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_option() {
        fn sample(input: u8) -> f32 {
            input as f32 + 1.25
        }
        assert_eq! {
            Some(42u8).apply(sample), 
            Some(43.25)
        }
    }
}
