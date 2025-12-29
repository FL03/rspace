/*
    Appellation: impl_point <module>
    Created At: 2025.12.29:14:06:01
    Contrib: @FL03
*/
use super::Point;

impl<X, Y> Point<X, Y> {
    /// Creates a new `Point`.
    pub const fn new(x: X, y: Y) -> Self {
        Point { x, y }
    }
    /// create a new instance from the given 2-tuple
    pub fn from_tuple((x, y): (X, Y)) -> Self {
        Point { x, y }
    }
    /// returns a reference to the x coordinate.
    pub const fn x(&self) -> &X {
        &self.x
    }
    /// returns a mutable reference to the x coordinate.
    pub const fn x_mut(&mut self) -> &mut X {
        &mut self.x
    }
    /// returns a reference to the y coordinate.
    pub const fn y(&self) -> &Y {
        &self.y
    }
    /// returns a mutable reference to the y coordinate.
    pub const fn y_mut(&mut self) -> &mut Y {
        &mut self.y
    }
    /// [`replace`](core::mem::replace) the x value, returning the old value.
    pub const fn replace_x(&mut self, x: X) -> X {
        core::mem::replace(self.x_mut(), x)
    }
    /// [`replace`](core::mem::replace) the y value, returning the old value.
    pub const fn replace_y(&mut self, y: Y) -> Y {
        core::mem::replace(self.y_mut(), y)
    }
    #[inline]
    /// set the x value
    pub fn set_x(&mut self, x: X) {
        self.x = x;
    }
    #[inline]
    /// set the y value
    pub fn set_y(&mut self, y: Y) {
        self.y = y;
    }
    #[inline]
    /// consumes the current instance to create another with the given `x` value.
    pub fn with_x<X2>(self, x: X2) -> Point<X2, Y> {
        Point { x, y: self.y }
    }
    #[inline]
    /// consumes the current instance to create another with the given `y` value.
    pub fn with_y<Y2>(self, y: Y2) -> Point<X, Y2> {
        Point { x: self.x, y }
    }
    /// returns an owned view of the point
    pub const fn as_view(&self) -> Point<&X, &Y> {
        Point {
            x: self.x(),
            y: self.y(),
        }
    }
    /// returns a view of the [`Point`] containing mutable references to the inner values.
    pub const fn as_mut_view(&mut self) -> Point<&mut X, &mut Y> {
        Point {
            x: &mut self.x,
            y: &mut self.y,
        }
    }
    /// returns a 2-tuple containing references to the inner values.
    pub const fn as_tuple(&self) -> (&X, &Y) {
        (self.x(), self.y())
    }
    #[inline]
    /// consumes the caller to convert the object into a 2-tuple.
    pub fn into_tuple(self) -> (X, Y) {
        (self.x, self.y)
    }
}