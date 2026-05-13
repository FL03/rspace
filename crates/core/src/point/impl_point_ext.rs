/*
    Appellation: impl_point_ext <module>
    Created At: 2025.12.29:14:05:12
    Contrib: @FL03
*/
use super::Point;

impl<X, Y> From<(X, Y)> for Point<X, Y> {
    fn from((x, y): (X, Y)) -> Self {
        Point { x, y }
    }
}

impl<X, Y> From<Point<X, Y>> for (X, Y) {
    fn from(point: Point<X, Y>) -> Self {
        point.into_tuple()
    }
}

impl<X> From<[X; 2]> for Point<X, X> {
    fn from([x, y]: [X; 2]) -> Self {
        Point { x, y }
    }
}

impl<X> From<Point<X, X>> for [X; 2] {
    fn from(point: Point<X, X>) -> Self {
        [point.x, point.y]
    }
}

impl<X, Y> PartialEq<(X, Y)> for Point<X, Y>
where
    X: PartialEq,
    Y: PartialEq,
{
    fn eq(&self, (x, y): &(X, Y)) -> bool {
        &self.x == x && &self.y == y
    }
}

impl<X, Y> PartialEq<Point<X, Y>> for (X, Y)
where
    X: PartialEq,
    Y: PartialEq,
{
    fn eq(&self, point: &Point<X, Y>) -> bool {
        let (x, y) = self;
        x == &point.x && y == &point.y
    }
}
