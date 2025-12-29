/*
    Appellation: impl_container <module>
    Created At: 2025.12.26:19:32:27
    Contrib: @FL03
*/
use crate::container::{Container, ContainerMap};
use crate::space::RawSpace;
use crate::ops::Apply;

impl<S, T> Container<T> for S
where
    S: RawSpace<Elem = T>,
{
    type Cont<V> = S;
}

impl<S, T> ContainerMap<T> for S
where
    S: Container<T, Cont<T> = S>,
    S::Cont<T>: RawSpace<Elem = T>,
{
    fn map<F, X>(&self, f: F) -> Self::Cont<X>
    where
        Self::Cont<T>: Apply<F, Output = Self::Cont<X>>,
        Self::Cont<X>: Sized,
    {
        <Self::Cont<T> as Apply<F>>::apply(self, f)
    }
}

impl<T> Container<T> for [T] {
    type Cont<U> = [U];
}

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