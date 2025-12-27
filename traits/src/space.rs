/*
    Appellation: space <module>
    Created At: 2025.12.26:14:12:46
    Contrib: @FL03
*/
/// The [`RawSpace`] trait is used to define a base interface for all containers whose elements
/// are of **one** specific type.
pub trait RawSpace {
    /// The type of elements associated with the container
    type Elem;
}
/// [`RawSpaceMut`] is a trait that provides various mutable methods for accessing elements.
pub trait RawSpaceMut: RawSpace {}
/// [`RawSpaceRef`] is a trait that provides various read-only methods for accessing elements.
pub trait RawSpaceRef: RawSpace {}

/*
 ************* Implementations *************
*/

impl<C, T> RawSpace for &C
where
    C: RawSpace<Elem = T>,
{
    type Elem = C::Elem;
}

impl<C, T> RawSpace for &mut C
where
    C: RawSpace<Elem = T>,
{
    type Elem = C::Elem;
}

macro_rules! impl_raw_space  {
    (impl<Elem = $elem:ident> $trait:ident for {$(
        $($cont:ident)::*<$($lt:lifetime,)? $($T:ident),*> $({where $($rest:tt)*})?
    ),* $(,)?}) => {
        $(impl_raw_space! {
            @impl<Elem = $elem> $trait for $($cont)::*<$($lt,)? $($T),*> $(where $($rest)*)?
        })*
    };
    (@impl<Elem = $elem:ident> $trait:ident for $($cont:ident)::*<$($lt:lifetime,)? $($T:ident),*> $(where $($rest:tt)*)?) => {
        impl<$($lt,)? $($T),*> $trait for $($cont)::*<$($lt,)? $($T),*> $(where $($rest)*)? {
            type Elem = $elem;
        }
    };
}

macro_rules! impl_raw_tuple_store {
    (@impl<$T:ident> $trait:ident for ($($name:ident),+ $(,)?)) => {
        impl<$T> $trait for ($($name),+) {
            type Elem = $T;
        }
    };
    (impl<$T:ident> $trait:ident for {$(($($name:ident),+)),* $(,)?}) => {
        $(impl_raw_tuple_store! { @impl<$T> $trait for ($($name),+) } )*
    };
}

impl_raw_tuple_store! {
    impl<T> RawSpace for {
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
