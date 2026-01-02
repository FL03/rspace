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
impl<U, V, F> Functor<F, V> for Option<U>
where
    F: FnOnce(U) -> V,
{
    type Cont<T> = Option<T>;
    type Elem = U;

    fn apply(self, f: F) -> Self::Cont<V> {
        self.map(f)
    }
}

impl<'a, U, V, F> Functor<F, V> for &'a Option<U>
where
    F: FnOnce(&U) -> V,
{
    type Cont<T> = Option<T>;
    type Elem = &'a U;

    fn apply(self, f: F) -> Self::Cont<V> {
        self.as_ref().map(|x| f(x))
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
