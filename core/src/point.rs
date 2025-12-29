/*
    Appellation: point <module>
    Created At: 2025.12.29:14:01:09
    Contrib: @FL03
*/
mod impl_point;
mod impl_point_ext;
mod impl_point_repr;

/// The [`Point`] implementation is designed a generic, 2-dimensional point object used to
/// define coordinates, vectors, or positions in a 2D space.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[repr(C)]
pub struct Point<X, Y = X> {
    #[cfg_attr(feature = "serde", serde(alias = "lhs", alias = "a"))]
    pub x: X,
    #[cfg_attr(feature = "serde", serde(alias = "rhs", alias = "b"))]
    pub y: Y,
}
