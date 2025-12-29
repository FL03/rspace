/*
    Appellation: point <module>
    Created At: 2025.12.29:14:01:09
    Contrib: @FL03
*/
mod impl_point;
mod impl_point_ext;
mod impl_point_repr;
/// An instance of the [`Point`] implementation containing owned references to the inner values.
pub type PointView<'a, X, Y = X> = Point<&'a X, &'a Y>;
/// An instance of the [`Point`] implementation containing mutable references to the inner
/// values.
pub type PointViewMut<'a, X, Y = X> = Point<&'a mut X, &'a mut Y>;
/// A [`Point`] whose elements are raw pointers to `X` and `Y`
pub type RawPoint<X, Y = X> = Point<*const X, *const Y>;
/// A mutable [`Point`] whose elements are raw pointers to `X` and `Y`
pub type RawPointMut<X, Y = X> = Point<*mut X, *mut Y>;

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
