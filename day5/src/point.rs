use std::error;
use std::ops::Sub;
use std::{fmt::Display, str::FromStr};

extern crate core;

/// Alias for a dimension unit (i32)
pub type Dim = i32;

/// A single point on the virtual canvas
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub struct Point([Dim; 2]);

impl Point {
    pub(crate) fn x(&self) -> Dim {
        self.0[0]
    }

    pub(crate) fn y(&self) -> Dim {
        self.0[1]
    }
}

/// Error when point can not be parsed from a string
impl FromStr for Point {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s.split_once(",")
            .and_then(|(a, b)| {
                let dim_a: Dim = a.parse().ok()?;
                let dim_b: Dim = b.parse().ok()?;
                Some(Point([dim_a, dim_b]))
            })
            .ok_or_else(|| Error::PointFormat(s.to_string()))
    }
}

impl Display for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let [a, b] = self.0;
        write!(f, "{},{}", a, b)
    }
}

impl From<(Dim, Dim)> for Point {
    fn from(tuple: (Dim, Dim)) -> Self {
        let (a, b) = tuple;
        Point([a, b])
    }
}

impl From<Point> for (Dim, Dim) {
    fn from(pt: Point) -> Self {
        let [x, y] = pt.0;
        (x, y)
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    PointFormat(String),
}

impl error::Error for Error {}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::PointFormat(s) => write!(f, "Invalid point: ({})", s),
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn it_can_parse_point() {
        assert_eq!(Ok(Point([445, 187])), "445,187".parse());
    }

    #[test]
    fn it_fails_to_parse_partial_point() {
        assert_eq!(not_a_point(""), parse_as_point(""));
        assert_eq!(not_a_point(","), parse_as_point(","));
        assert_eq!(not_a_point("445,"), parse_as_point("445,"));
        assert_eq!(not_a_point(",187"), parse_as_point(",187"));
        assert_eq!(not_a_point("foo,187"), parse_as_point("foo,187"));
        assert_eq!(not_a_point("445,bar"), parse_as_point("445,bar"));
        assert_eq!(not_a_point("445 187"), parse_as_point("445 187"));
        assert_eq!(not_a_point("445"), parse_as_point("445"));
    }

    fn not_a_point(m: &str) -> Result<Point, Error> {
        Err(Error::PointFormat(m.to_string()))
    }

    fn parse_as_point(s: &str) -> Result<Point, Error> {
        s.parse()
    }
}
