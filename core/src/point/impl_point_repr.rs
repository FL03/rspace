/*
    Appellation: impl_point_repr <module>
    Created At: 2025.12.29:14:05:09
    Contrib: @FL03
*/
use super::Point;

impl<'a, X, Y> Point<&'a X, &'a Y> {
    /// returns an new instance of the [`Point`] with cloned values
    pub fn cloned(&self) -> Point<X, Y>
    where
        X: Clone,
        Y: Clone,
    {
        Point {
            x: self.x.clone(),
            y: self.y.clone(),
        }
    }
    /// return a new instance of the [`Point`] with copied values
    pub const fn copied(&self) -> Point<X, Y>
    where
        X: Copy,
        Y: Copy,
    {
        Point {
            x: *self.x,
            y: *self.y,
        }
    }
}

impl<'a, X, Y> Point<&'a mut X, &'a mut Y> {
    /// returns an new instance of the [`Point`] with cloned values
    pub fn cloned(&self) -> Point<X, Y>
    where
        X: Clone,
        Y: Clone,
    {
        Point {
            x: self.x.clone(),
            y: self.y.clone(),
        }
    }
    /// return a new instance of the [`Point`] with copied values
    pub const fn copied(&self) -> Point<X, Y>
    where
        X: Copy,
        Y: Copy,
    {
        Point {
            x: *self.x,
            y: *self.y,
        }
    }
}