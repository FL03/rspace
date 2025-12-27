/*
    Appellation: key_value <module>
    Created At: 2025.12.26:17:44:22
    Contrib: @FL03
*/
//! this module defines traits for key-value stores and their entries
//!

/// [`StoreEntry`] establishes a common interface for entries within a key-value store.
pub trait StoreEntry<'a> {
    type Key;
    type Value;
}
/// The [`RawStore`] trait is a marker trait for key-value store containers.
pub trait RawStore<K, V> {}
/// The [`Store`] trait is used to define a key-value store container.
pub trait Store<K, V>: RawStore<K, V> {
    type Entry<'a>: StoreEntry<'a, Key = K, Value = V>
    where
        Self: 'a;
}

/*
 ************* Implementations *************
*/
