/*
    Appellation: key_value <module>
    Created At: 2025.12.26:17:44:22
    Contrib: @FL03
*/
//! this module defines traits for key-value stores and their entries
//!

/// [`KeyValueEntry`] establishes a common interface for entries within a key-value store.
pub trait StoreEntry<'a> {
    type Key;
    type Value;
}

pub trait RawStore<K, V> {}

/// The [`Store`] trait is used to define a key-value store container.
pub trait Store<K, V> {
    type Entry<'a>: StoreEntry<'a, Key = K, Value = V>
    where
        Self: 'a;
}

/*
 ************* Implementations *************
*/

#[cfg(feature = "alloc")]
mod impl_alloc {
    use super::{StoreEntry, Store};
    use alloc::collections::btree_map::{self, BTreeMap};

    impl<'a, K, V> StoreEntry<'a> for btree_map::Entry<'a, K, V> {
        type Key = K;
        type Value = V;
    }

    impl<K, V> Store<K, V> for BTreeMap<K, V> {
        type Entry<'a>
            = btree_map::Entry<'a, K, V>
        where
            Self: 'a;
    }
}

#[cfg(feature = "hashbrown")]
mod impl_hashbrown {
    use super::{StoreEntry, Store};
    use hashbrown::hash_map::{self, HashMap};

    impl<'a, K, V, S> StoreEntry<'a> for hash_map::Entry<'a, K, V, S> {
        type Key = K;
        type Value = V;
    }

    impl<K, V, S> Store<K, V> for HashMap<K, V, S> {
        type Entry<'a>
            = hash_map::Entry<'a, K, V, S>
        where
            Self: 'a;
    }
}

#[cfg(feature = "std")]
mod impl_std {
    use super::{StoreEntry, Store};
    use std::collections::hash_map::{self, HashMap};

    impl<'a, K, V> StoreEntry<'a> for hash_map::Entry<'a, K, V> {
        type Key = K;
        type Value = V;
    }

    impl<K, V> Store<K, V> for HashMap<K, V> {
        type Entry<'a>
            = hash_map::Entry<'a, K, V>
        where
            Self: 'a;
    }
}
