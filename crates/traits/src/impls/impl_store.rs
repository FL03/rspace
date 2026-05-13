/*
    Appellation: impl_store <module>
    Created At: 2025.12.26:19:48:49
    Contrib: @FL03
*/
#[cfg(all(feature = "alloc", feature = "nightly"))]
mod impl_alloc {
    use crate::store::{RawStore, Store, StoreEntry};
    use alloc::alloc::Allocator;
    use alloc::collections::btree_map::{self, BTreeMap};
    use alloc::vec::Vec;

    impl<'a, K, V, A> StoreEntry<'a> for btree_map::Entry<'a, K, V, A>
    where
        A: Allocator + Clone,
    {
        type Key = K;
        type Value = V;
    }

    impl<K, V, A> RawStore<K, V> for BTreeMap<K, V, A> where A: Allocator + Clone {}

    impl<K, V, A> Store<K, V> for BTreeMap<K, V, A>
    where
        A: Allocator + Clone,
    {
        type Entry<'a>
            = btree_map::Entry<'a, K, V, A>
        where
            Self: 'a;
    }

    impl<T, A> RawStore<usize, T> for Vec<T, A> where A: Allocator {}
}

#[cfg(all(feature = "alloc", not(feature = "nightly")))]
mod impl_alloc {
    use crate::store::{RawStore, Store, StoreEntry};
    use alloc::collections::btree_map::{self, BTreeMap};
    use alloc::vec::Vec;

    impl<'a, K, V> StoreEntry<'a> for btree_map::Entry<'a, K, V> {
        type Key = K;
        type Value = V;
    }

    impl<K, V> RawStore<K, V> for BTreeMap<K, V> {}

    impl<K, V> Store<K, V> for BTreeMap<K, V> {
        type Entry<'a>
            = btree_map::Entry<'a, K, V>
        where
            Self: 'a;
    }

    impl<T> RawStore<usize, T> for Vec<T> {}
}

#[cfg(feature = "hashbrown")]
mod impl_hashbrown {
    use crate::store::{RawStore, Store, StoreEntry};
    use hashbrown::hash_map::{self, HashMap};

    impl<'a, K, V, S> StoreEntry<'a> for hash_map::Entry<'a, K, V, S> {
        type Key = K;
        type Value = V;
    }

    impl<K, V, S> RawStore<K, V> for HashMap<K, V, S> {}

    impl<K, V, S> Store<K, V> for HashMap<K, V, S> {
        type Entry<'a>
            = hash_map::Entry<'a, K, V, S>
        where
            Self: 'a;
    }
}

#[cfg(feature = "std")]
mod impl_std {
    use crate::store::{RawStore, Store, StoreEntry};
    use std::collections::hash_map::{self, HashMap};

    impl<'a, K, V> StoreEntry<'a> for hash_map::Entry<'a, K, V> {
        type Key = K;
        type Value = V;
    }

    impl<K, V> RawStore<K, V> for HashMap<K, V> {}

    impl<K, V> Store<K, V> for HashMap<K, V> {
        type Entry<'a>
            = hash_map::Entry<'a, K, V>
        where
            Self: 'a;
    }
}
