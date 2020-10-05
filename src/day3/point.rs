use std::cmp::Ordering;
use std::ops::AddAssign;
use crate::day3::direction::Vector;
use std::fmt::{self, Display, Formatter};

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub struct Point(i32, i32);

impl From<(i32, i32)> for Point {
    fn from(t: (i32, i32)) -> Self { Point(t.0, t.1) }
}

impl Point {
    pub fn origin() -> Point { Point(0, 0) }
}

impl Default for Point {
    fn default() -> Self { Point::origin() }
}

impl Display for Point {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.0, self.1)
    }
}

impl Point {
    pub fn x(&self) -> i32 { self.0 }
    pub fn y(&self) -> i32 { self.1 }
}

impl Point {
    /// Calculates manhattan distance of this point from the origin
    pub fn distance_from_origin(&self) -> usize {
        (self.0.abs() as usize) + (self.1.abs() as usize)
    }
}

impl PartialOrd for Point {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Point {
    fn cmp(&self, other: &Self) -> Ordering {
        self.distance_from_origin().cmp(&other.distance_from_origin())
    }
}

impl <V> AddAssign<V> for Point where V: Vector {
    fn add_assign(&mut self, rhs: V) {
        self.0 += rhs.x();
        self.1 += rhs.y();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_tuple() {
        let pt = Point::from((7, 13));
        assert_eq!(pt.x(), 7);
        assert_eq!(pt.y(), 13);
    }

    #[test]
    fn from_origin() {
        assert_eq!(Point::origin(), Point::from((0, 0)));
    }

    #[test]
    fn default_is_origin() {
        assert_eq!(Point::default(), Point::origin());
    }

    #[test]
    fn distance_from_origin() {
        let pt = Point::from((17, 19));
        assert_eq!(pt.distance_from_origin(), 17 + 19);
    }

    impl Vector for (i32, i32) {
        fn x(&self) -> i32 { self.0 }
        fn y(&self) -> i32 { self.1 }
    }

    #[test]
    fn add_assign_vector() {
        let mut pt = Point::from((23, 29));
        pt += (31, 37);
        assert_eq!(pt.x(), 23 + 31);
        assert_eq!(pt.y(), 29 + 37);
    }

}