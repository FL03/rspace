/*
    Appellation: store <module>
    Created At: 2025.12.26:14:12:46
    Contrib: @FL03
*/
use crate::space::RawSpace;

/// The [`Container`] trait is a higher-kinded trait used to establish an interface for
/// defining containers themselves.
pub trait Container<U>
where
    Self::Cont<U>: RawSpace<Elem = U>,
{
    type Cont<V>: ?Sized;
}

pub trait ContainerIter<U>: Container<U>
where
    Self::Cont<U>: RawSpace<Elem = U>,
{
    type Iter<'a, T>: Iterator<Item = &'a T>
    where
        Self: 'a,
        T: 'a;
}

/*
 ************* Implementations *************
*/
#[allow(unused_macros)]
macro_rules! container {
    (@impl $trait:ident<$T:ident> for $($cont:ident)::*<$($U:ident),+ $(,)?>) => {
        paste::paste! {
            impl<$($U),+> $trait<$T> for $($cont)::*<$($U),+> {
                type Cont<[<_ $T>]> = $($cont)::*<[<_ $T>]>;
            }

        }
    };
    (impl $trait:ident<$T:ident> for {$($($cont:ident)::*<$($U:ident),+ $(,)?>),* $(,)?}) => {
        $(container!{ @impl $trait<$T> for $($cont)::*<$($U),+>})*
    };
}
