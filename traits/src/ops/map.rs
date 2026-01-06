/*
    Appellation: map <module>
    Created At: 2026.01.01:21:31:12
    Contrib: @FL03
*/
/// [`MapInto`] defines an interface for containers capable of applying a given function onto
/// each of their elements and consuming the container in the process, producing a new
/// container containing the captured results of each invocation. The trait's design allows
/// implementors to specify the exact function signature and output type, utilizing associated
/// type parameters to define the container and its _current_ element type.
pub trait MapInto<F, T>
where
    F: FnOnce(Self::Elem) -> T,
{
    type Cont<_T>: ?Sized;
    /// the current type of element associated with the contained
    type Elem;

    fn apply(self, f: F) -> Self::Cont<T>;
}

/// The [`MapTo`] trait is similar to [`MapInto`], but it operates on references to the
/// container rather than consuming it. This allows for mapping functions over the elements
/// of a container while retaining ownership of the original container.  
pub trait MapTo<F, X>
where
    F: FnOnce(Self::Elem) -> X,
{
    type Cont<T>: ?Sized;
    type Elem;

    fn apply(&self, f: F) -> Self::Cont<X>;
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_map_into_on_option() {
        use super::MapInto;
        fn sample_once(input: u8) -> f32 {
            input as f32 + 1.25
        }
        let exp = Some(43.25f32);
        assert_eq! { Some(42u8).apply(sample_once), exp }
    }

    #[test]
    fn test_map_to_on_option() {
        use super::MapTo;
        fn sample_ref(input: &u8) -> f32 {
            *input as f32 + 1.25
        }
        let exp = Some(43.25f32);
        assert_eq! { Some(&42u8).apply(sample_ref), exp }
    }
}
