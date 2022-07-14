use std::{error, fmt::Display, str::FromStr};

use crate::point::{self, Point};

#[derive(Debug, PartialEq, Eq)]
pub struct Segment(Point, Point);

impl FromStr for Segment {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s.split_once(" -> ")
            .ok_or_else(|| Error::SegmentFormat(s.to_string()))
            .and_then(|(a, b)| {
                let pt_a: Point = a.parse()?;
                let pt_b: Point = b.parse()?;
                Ok(Segment(pt_a, pt_b))
            })
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    SegmentFormat(String),
    PointFormat(point::Error),
}

impl error::Error for Error {}

impl From<point::Error> for Error {
    fn from(cause: point::Error) -> Self {
        Error::PointFormat(cause)
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::SegmentFormat(s) => write!(f, "Invalid segment: {}", s),
            Error::PointFormat(source) => write!(f, "{}", source),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    impl Segment {
        fn of<P>(a: P, b: P) -> Segment
        where
            P: Into<Point>,
        {
            Segment(a.into(), b.into())
        }
    }

    #[test]
    fn it_can_parse_valid_segment() {
        assert_eq!(
            Ok(Segment::of((445, 187), (912, 654))),
            "445,187 -> 912,654".parse()
        );
    }

    #[test]
    fn it_fails_to_parse_partial_segment() {
        assert_eq!(invalid_segment_format(""), parse_segment(""));
        assert_eq!(invalid_segment_format("445,187"), parse_segment("445,187"));
        assert_eq!(invalid_point_format(""), parse_segment(" -> "));
        assert_eq!(invalid_point_format(""), parse_segment("445,187 -> "));
        assert_eq!(invalid_point_format(""), parse_segment(" -> 445,187"));
    }

    fn parse_segment(s: &str) -> Result<Segment, Error> {
        s.parse()
    }

    fn invalid_segment_format(m: &str) -> Result<Segment, Error> {
        Err(Error::SegmentFormat(m.to_string()))
    }

    fn invalid_point_format(m: &str) -> Result<Segment, Error> {
        Err(point::Error::PointFormat(m.to_string()).into())
    }
}
