/*
    Appellation: get <module>
    Created At: 2025.12.29:15:17:51
    Contrib: @FL03
*/

/// [`Get`] defines an interface for entities that can be accessed by a key; the design is
/// similar to the [`Index`](core::ops::Index) trait in the standard library, however, uses the
/// [`Borrow`](core::borrow::Borrow) trait to allow for more flexible key types.
pub trait Get<Q> {
    type Key;
    type Value: ?Sized;
    /// returns a reference to the element at the specified index.
    fn get(&self, index: Q) -> Option<&Self::Value>
    where
        Self::Key: core::borrow::Borrow<Q>;
}
/// [`GetMut`] defines an interface for entities that can be accessed by a key; the design
/// is similar to the [`IndexMut`](core::ops::IndexMut) trait in the standard library
pub trait GetMut<Q>: Get<Q> {
    /// returns a mutable reference to the element at the specified index.
    fn get_mut(&mut self, index: Q) -> Option<&mut Self::Value>
    where
        Self::Key: core::borrow::Borrow<Q>;
}

/*
 ************* Implementations *************
*/

impl<Q, K, Y, U> Get<Q> for &U
where
    U: Get<Q, Key = K, Value = Y>,
{
    type Key = U::Key;
    type Value = U::Value;

    fn get(&self, index: Q) -> Option<&Y>
    where
        Self::Key: core::borrow::Borrow<Q>,
    {
        (*self).get(index)
    }
}

impl<Q, T> Get<Q> for [T]
where

    Q: core::slice::SliceIndex<[T]>,
{
    type Key = usize;
    type Value = Q::Output;

    fn get(&self, index: Q) -> Option<&Self::Value>
    where
        Self::Key: core::borrow::Borrow<Q>,
    {
        self.as_ref().get(index)
    }
}

#[cfg(feature = "hashbrown")]
impl<Q, K, V, S> Get<Q> for hashbrown::HashMap<K, V, S>
where
    Q: Eq + core::hash::Hash,
    K: Eq + core::hash::Hash,
    S: core::hash::BuildHasher,
{
    type Key = K;
    type Value = V;

    fn get(&self, index: Q) -> Option<&V>
    where
        Self::Key: core::borrow::Borrow<Q>,
    {
        self.get(&index)
    }
}
