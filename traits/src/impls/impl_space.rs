/*
    Appellation: impl_raw_space <module>
    Created At: 2025.12.26:19:20:09
    Contrib: @FL03
*/
use crate::space::RawSpace;

macro_rules! impl_raw_space  {
    (impl<Elem = $E:ident> $trait:ident for {$(
        $($cont:ident)::*<$($lt:lifetime,)? $($T:ident),*> $({where $($rest:tt)*})?
    ),* $(,)?}) => {
        $(impl_raw_space! {
            @impl<Elem = $E> $trait for $($cont)::*<$($lt,)? $($T),*> $(where $($rest)*)?
        })*
    };
    (@impl<Elem = $E:ident> $trait:ident for $($cont:ident)::*<$($lt:lifetime,)? $($T:ident),*> $(where $($rest:tt)*)?) => {
        impl<$($lt,)? $($T),*> $trait for $($cont)::*<$($lt,)? $($T),*> $(where $($rest)*)? {
            type Elem = $E;
        }
    };
}

macro_rules! impl_raw_tuple_store {
    (@impl<Elem = $E:ident> $trait:ident for ($($name:ident),+ $(,)?)) => {
        impl<$E> $trait for ($($name),+) {
            type Elem = $E;
        }
    };
    (impl<Elem = $E:ident> $trait:ident for {$(($($name:ident),+)),* $(,)?}) => {
        $(impl_raw_tuple_store! { @impl<Elem = $E> $trait for ($($name),+) } )*
    };
}

impl_raw_tuple_store! {
    impl<Elem = T> RawSpace for {
        (T, T),
        (T, T, T),
        (T, T, T, T),
        (T, T, T, T, T),
        (T, T, T, T, T, T),
        (T, T, T, T, T, T, T),
        (T, T, T, T, T, T, T, T),
        (T, T, T, T, T, T, T, T, T),
        (T, T, T, T, T, T, T, T, T, T),
        (T, T, T, T, T, T, T, T, T, T, T),
        (T, T, T, T, T, T, T, T, T, T, T, T),
    }
}

impl_raw_space! {
    impl<Elem = T> RawSpace for {
        core::option::Option<T>,
        core::cell::Cell<T>,
        core::cell::OnceCell<T>,
        core::cell::RefCell<T>,
        core::cell::UnsafeCell<T>,
        core::ops::Range<T>,
        core::result::Result<T, E>,
    }
}

#[cfg(all(feature = "alloc", not(feature = "nightly")))]
impl_raw_space! {
    impl<Elem = T> RawSpace for {
        alloc::boxed::Box<T>,
        alloc::rc::Rc<T>,
        alloc::sync::Arc<T>,
        alloc::collections::BTreeSet<T>,
        alloc::collections::LinkedList<T>,
        alloc::collections::VecDeque<T>,
        alloc::collections::BinaryHeap<T>,
        alloc::collections::BTreeMap<K, T>,
        alloc::collections::btree_map::Entry<'a, K, T>,
        alloc::vec::Vec<T>,
    }
}

#[cfg(all(feature = "alloc", feature = "nightly"))]
impl_raw_space! {
    impl<Elem = T> RawSpace for {
        alloc::collections::BTreeSet<T, A> { where A: alloc::allocator::Allocator },
        alloc::collections::LinkedList<T, A> { where A: alloc::allocator::Allocator },
        alloc::collections::VecDeque<T, A> { where A: alloc::allocator::Allocator },
        alloc::collections::BinaryHeap<T, A> { where A: alloc::allocator::Allocator },
        alloc::collections::BTreeMap<K, T, A> { where A: alloc::allocator::Allocator },
        alloc::collections::btree_map::Entry<'a, K, T, a> { where A: alloc::allocator::Allocator },
        alloc::vec::Vec<T, A> { where A: alloc::allocator::Allocator },
    }
}

#[cfg(feature = "std")]
impl_raw_space! {
    impl<Elem = T> RawSpace for {
        std::sync::Mutex<T>,
        std::sync::RwLock<T>,
        std::sync::LazyLock<T>,
        std::collections::HashMap<K, T>,
        std::collections::HashSet<T>,

    }
}

#[cfg(feature = "hashbrown")]
impl_raw_space! {
    impl<Elem = T> RawSpace for {
        hashbrown::HashMap<K, T, S>,
        hashbrown::HashSet<T, S>,
    }
}

#[cfg(feature = "complex")]
impl<T> RawSpace for num_complex::Complex<T> {
    type Elem = T;
}

impl<T> RawSpace for [T] {
    type Elem = T;
}

impl<T> RawSpace for &[T] {
    type Elem = T;
}

impl<T> RawSpace for &mut [T] {
    type Elem = T;
}

impl<const N: usize, T> RawSpace for [T; N] {
    type Elem = T;
}
