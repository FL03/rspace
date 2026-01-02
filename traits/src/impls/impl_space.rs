/*
    Appellation: impl_raw_space <module>
    Created At: 2025.12.26:19:20:09
    Contrib: @FL03
*/
use crate::{RawSpace, RawSpaceMut, RawSpaceRef, SliceSpace, SliceSpaceMut};

macro_rules! impl_scalar_space  {
    (impl $trait:ident for {$($T:ty),* $(,)?}) => {
        $(impl_scalar_space! { @impl $trait for $T })*
    };
    (@impl $trait:ident for $T:ty) => {
        impl $crate::$trait for $T {
            type Elem = $T;

        }
    };
}

macro_rules! impl_raw_space  {
    (impl<Elem = $E:ident> $trait:ident for {$(
        $($cont:ident)::*<$($lt:lifetime,)? $($T:ident),*> $({where $($rest:tt)*})?
    ),* $(,)?}) => {
        $(impl_raw_space! {
            @impl<Elem = $E> $trait for $($cont)::*<$($lt,)? $($T),*> $(where $($rest)*)?
        })*
    };
    (@impl<Elem = $E:ident> $trait:ident for $($cont:ident)::*<$($lt:lifetime,)? $($T:ident),*> $(where $($rest:tt)*)?) => {
        impl<$($lt,)? $($T),*> $crate::$trait for $($cont)::*<$($lt,)? $($T),*> $(where $($rest)*)? {
            type Elem = $E;
        }
    };
}

macro_rules! impl_raw_tuple_store {
    (@impl<Elem = $E:ident> $trait:ident for ($($name:ident),+ $(,)?)) => {
        impl<$E> $crate::$trait for ($($name),+) {
            type Elem = $E;
        }
    };
    (impl<Elem = $E:ident> $trait:ident for {$(($($name:ident),+)),* $(,)?}) => {
        $(impl_raw_tuple_store! { @impl<Elem = $E> $trait for ($($name),+) } )*
    };
}

impl_scalar_space! {
    impl RawSpace for {
        i8, i16, i32, i64, i128, isize,
        u8, u16, u32, u64, u128, usize,
        f32, f64,
        bool, char
    }
}
#[cfg(feature = "alloc")]
impl_scalar_space! {
    impl RawSpace for {
        alloc::string::String
    }
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
        alloc::collections::BTreeSet<T, A> { where A: Clone + alloc::alloc::Allocator },
        alloc::collections::LinkedList<T, A> { where A: alloc::alloc::Allocator },
        alloc::collections::VecDeque<T, A> { where A: alloc::alloc::Allocator },
        alloc::collections::BinaryHeap<T, A> { where A: alloc::alloc::Allocator },
        alloc::collections::BTreeMap<K, T, A> { where A: Clone + alloc::alloc::Allocator },
        alloc::collections::btree_map::Entry<'a, K, T, A> { where A: Clone + alloc::alloc::Allocator },
        alloc::vec::Vec<T, A> { where A: alloc::alloc::Allocator },
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

impl<const N: usize, T> RawSpaceRef for [T; N] {
    fn as_ptr(&self) -> *const Self::Elem {
        <[T]>::as_ptr(self)
    }
}

impl<const N: usize, T> RawSpaceMut for [T; N] {
    fn as_ptr_mut(&mut self) -> *mut Self::Elem {
        <[T]>::as_mut_ptr(self)
    }
}

impl<const N: usize, T> SliceSpace for [T; N] {
    fn as_slice(&self) -> &[Self::Elem] {
        self
    }
}

impl<const N: usize, T> SliceSpaceMut for [T; N] {
    fn as_mut_slice(&mut self) -> &mut [Self::Elem] {
        self
    }
}

impl<T> RawSpaceRef for [T] {
    fn as_ptr(&self) -> *const Self::Elem {
        <[T]>::as_ptr(self)
    }
}

impl<T> RawSpaceMut for [T] {
    fn as_ptr_mut(&mut self) -> *mut Self::Elem {
        <[T]>::as_mut_ptr(self)
    }
}

impl<T> SliceSpace for [T] {
    fn as_slice(&self) -> &[Self::Elem] {
        self
    }
}

impl<T> SliceSpaceMut for [T] {
    fn as_mut_slice(&mut self) -> &mut [Self::Elem] {
        self
    }
}

impl<T> RawSpaceRef for &[T] {
    fn as_ptr(&self) -> *const Self::Elem {
        <[T]>::as_ptr(self)
    }
}

impl<T> SliceSpace for &[T] {
    fn as_slice(&self) -> &[Self::Elem] {
        self
    }
}

impl<T> RawSpaceRef for &mut [T] {
    fn as_ptr(&self) -> *const Self::Elem {
        <[T]>::as_ptr(self)
    }
}

impl<T> RawSpaceMut for &mut [T] {
    fn as_ptr_mut(&mut self) -> *mut Self::Elem {
        <[T]>::as_mut_ptr(self)
    }
}

impl<T> SliceSpace for &mut [T] {
    fn as_slice(&self) -> &[Self::Elem] {
        self
    }
}

impl<T> SliceSpaceMut for &mut [T] {
    fn as_mut_slice(&mut self) -> &mut [Self::Elem] {
        self
    }
}

#[cfg(all(feature = "alloc", feature = "nightly"))]
mod impl_alloc {
    use crate::space::*;
    use alloc::alloc::Allocator;
    use alloc::vec::Vec;

    impl<T, A> RawSpaceRef for Vec<T, A>
    where
        A: Allocator,
    {
        fn as_ptr(&self) -> *const Self::Elem {
            Vec::as_ptr(self)
        }
    }

    impl<T, A> RawSpaceMut for Vec<T, A>
    where
        A: Allocator,
    {
        fn as_ptr_mut(&mut self) -> *mut Self::Elem {
            Vec::as_mut_ptr(self)
        }
    }

    impl<T, A> SliceSpace for Vec<T, A>
    where
        A: Allocator,
    {
        fn as_slice(&self) -> &[Self::Elem] {
            self.as_slice()
        }
    }

    impl<T, A> SliceSpaceMut for Vec<T, A>
    where
        A: Allocator,
    {
        fn as_mut_slice(&mut self) -> &mut [Self::Elem] {
            self.as_mut_slice()
        }
    }
}

#[cfg(all(feature = "alloc", not(feature = "nightly")))]
mod impl_alloc {
    use crate::space::*;
    use alloc::vec::Vec;

    impl<T> RawSpaceRef for Vec<T> {
        fn as_ptr(&self) -> *const Self::Elem {
            Vec::as_ptr(self)
        }
    }

    impl<T> RawSpaceMut for Vec<T> {
        fn as_ptr_mut(&mut self) -> *mut Self::Elem {
            Vec::as_mut_ptr(self)
        }
    }

    impl<T> SliceSpace for Vec<T> {
        fn as_slice(&self) -> &[Self::Elem] {
            self.as_slice()
        }
    }

    impl<T> SliceSpaceMut for Vec<T> {
        fn as_mut_slice(&mut self) -> &mut [Self::Elem] {
            self.as_mut_slice()
        }
    }
}
