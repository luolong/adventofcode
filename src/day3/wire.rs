use crate::day3::direction::{Direction, ParseDirectionError};
use crate::day3::point::Point;
use crate::day3::wire::ParseWireError::{Empty, InvalidDirection};
use std::cmp::{max, min};
use std::ops::{Deref, RangeInclusive};
use std::str::FromStr;
use std::fmt::{Display, Formatter};
use std::fmt;

#[derive(Debug, Eq, PartialEq)]
pub(crate) struct Wire(Vec<Point>);

#[derive(Debug, Eq, PartialEq)]
pub(crate) enum ParseWireError {
    Empty,
    InvalidDirection(ParseDirectionError),
}

impl FromStr for Wire {
    type Err = ParseWireError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let input = s.trim();
        if input.is_empty() {
            Err(Empty)
        } else {
            let dirs = parse_directions(input).map_err(|err| InvalidDirection(err))?;

            let mut pt = Point::origin();
            let mut points = Vec::with_capacity(dirs.len() + 1);
            points.push(pt);
            for dir in dirs {
                pt += dir;
                points.push(pt);
            }

            Ok(Wire(points))
        }
    }
}

impl Deref for Wire {
    type Target = Vec<Point>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Display for Wire {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        if let Some((first, rest)) = self.split_first() {
            write!(f, "{}", first)?;
            for pt in rest {
                write!(f, "->{}", pt)?;
            }
        }
        Ok(())
    }
}

impl Wire {
    pub(crate) fn len(&self) -> usize {
        self.0.len()
    }
}

impl Wire {
    pub(crate) fn intersections(&self, other: &Wire) -> Vec<(usize, Point)> {
        let maxlen = max(self.len(), other.len());
        let mut intersections = Vec::with_capacity(maxlen);

        let mut d1: usize = 0;
        for s1 in self.windows(2) {
            let mut d2: usize = 0;
            for s2 in other.windows(2) {
                if d1 > 0 || d2 > 0 {
                    if let Some(pt) = s1.intersection(&s2) {
                        let dpt1 = s1.distance_to(&pt).unwrap_or(0);
                        let dpt2 = s2.distance_to(&pt).unwrap_or(0);
                        let distance = d1 + d2 + dpt1 + dpt2;
                        intersections.push((distance, pt));
                    }
                }

                d2 += s2.segment_len().unwrap_or(0);
            }
            d1 += s1.segment_len().unwrap_or(0);
        }
        intersections
    }
}

pub trait Segment {
    fn start(&self) -> &Point;
    fn end(&self) -> &Point;

    fn is_horizontal(&self) -> bool;
    fn is_vertical(&self) -> bool;

    fn x(&self) -> Option<i32>;
    fn y(&self) -> Option<i32>;

    fn x_range(&self) -> RangeInclusive<i32>;
    fn y_range(&self) -> RangeInclusive<i32>;

    fn segment_len(&self) -> Option<usize>;

    fn distance_to(&self, pt: &Point) -> Option<usize>;

    fn intersection(&self, other: &dyn Segment) -> Option<Point>;
}

impl Segment for &[Point] {
    fn start(&self) -> &Point {
        &self[0]
    }
    fn end(&self) -> &Point {
        &self[1]
    }

    fn is_horizontal(&self) -> bool {
        self.start().y() == self.end().y()
    }

    fn is_vertical(&self) -> bool {
        self.start().x() == self.end().x()
    }

    fn x(&self) -> Option<i32> {
        if self.is_vertical() {
            Some(self.start().x())
        } else {
            None
        }
    }

    fn y(&self) -> Option<i32> {
        if self.is_horizontal() {
            Some(self.start().y())
        } else {
            None
        }
    }

    fn x_range(&self) -> RangeInclusive<i32> {
        self.start().x()..=self.end().x()
    }

    fn y_range(&self) -> RangeInclusive<i32> {
        self.start().y()..=self.end().y()
    }

    fn segment_len(&self) -> Option<usize> {
        let (start, end) = (self.start(), self.end());

        if self.is_vertical() {
            let ymax = max(start.y(), end.y());
            let ymin = min(start.y(), end.y());
            Some((ymax - ymin) as usize)
        } else if self.is_horizontal() {
            let xmax = max(start.x(), end.x());
            let xmin = min(start.x(), end.x());
            Some((xmax - xmin) as usize)
        } else {
            None
        }
    }

    fn distance_to(&self, pt: &Point) -> Option<usize> {
        if self.start().x() == pt.x() {
            Some((pt.y() - self.start().y()).abs() as usize)
        } else if self.start().y() == pt.y() {
            Some((pt.x() - self.start().x()).abs() as usize)
        } else {
            None
        }
    }

    fn intersection(&self, other: &dyn Segment) -> Option<Point> {
        if let Some(x) = self.x() {
            if let Some(y) = other.y() {
                return intersection_of(
                    x, other.x_range(),
                    y, self.y_range()
                );
            }
        } else if let Some(y) = self.y() {
            if let Some(x) = other.x() {
                return intersection_of(
                    x, self.x_range(),
                    y, other.y_range()
                );
            }
        }
        None
    }
}

fn intersection_of(x: i32, segment_x_range: RangeInclusive<i32>, y: i32, segment_y_range: RangeInclusive<i32>) -> Option<Point> {
    let xmin = *min(segment_x_range.start(), segment_x_range.end());
    let xmax = *max(segment_x_range.start(), segment_x_range.end());
    let ymin = *min(segment_y_range.start(), segment_y_range.end());
    let ymax = *max(segment_y_range.start(), segment_y_range.end());
    if (xmin..=xmax).contains(&x) && (ymin..=ymax).contains(&y) {
        return Some(Point::from((x, y)));
    }
    None
}

fn parse_directions(line: &str) -> Result<Vec<Direction>, ParseDirectionError> {
    line.split(',').map(|s| Direction::from_str(s)).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_can_parse_directions() {
        assert_eq!(
            "U1,R2,D3,L5".parse(),
            Ok(Wire(vec![
                Point::origin(),
                Point::from((0, 1)),
                Point::from((2, 1)),
                Point::from((2, -2)),
                Point::from((-3, -2)),
            ]))
        );
        assert_eq!("".parse::<Wire>(), Err(Empty))
    }
}
