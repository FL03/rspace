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
pub trait MapTo<F, X>
where
    F: FnOnce(Self::Elem) -> X,
{
    type Cont<T>: ?Sized;
    type Elem;

    fn apply(&self, f: F) -> Self::Cont<X>;
}
/// [`MapInto`] describes a consuming interface for containers that enables the mapping of a
/// given function onto each element of the container. While the trait definition constrains
/// the generic function parameters, `F` to a `FnOnce` closure, implementors coulds choose
/// to strengthen this constraint to `FnMut` or `Fn` as needed.
pub trait MapInto<F, X>
where
    F: FnOnce(Self::Elem) -> X,
{
    type Cont<U>: ?Sized;
    /// the current type of element associated with the contained
    type Elem;

    fn apply(self, f: F) -> Self::Cont<X>;
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
            Some(43.25f32)
        }
    }
}
