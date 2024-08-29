/*
    Appellation: tuples <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
#![allow(unused_macros)]

macro_rules! tuple {
    (@impl $($elem:ident),* $(,)?) => {
        ($($elem,)*)
    };
    ($($elem:ident),* $(,)?) => {
        tuple!(@impl $($elem),*);
    };
    ($(($($elem:ident),* $(,)?)),+ $(,)?) => {
        $(
            tuple!(@impl $($elem),+);
        )+
    };
}
