use std::fmt;
use std::cmp;
use std::fs::File;
use std::io::{self, BufReader, Lines};
use std::num::ParseIntError;
use std::str::FromStr;
use fmt::Display;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {

    /// Creates new point at the origin (0, 0)
    fn origin() -> Point {
        Point { x: 0, y: 0 }
    }

    /// Returns a manhattan distance of this point from the origin
    fn distance_from_origin(&self) -> u32 {
        let x = self.x.abs() as u32;
        let y = self.y.abs() as u32;
        x + y
    }

}

impl PartialOrd for Point {
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        self.distance_from_origin().partial_cmp(&other.distance_from_origin())
    }
}

impl Ord for Point {
    fn cmp(&self, other: &Self) -> cmp::Ordering {
        self.distance_from_origin().cmp(&other.distance_from_origin())
    }
}

impl Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

#[derive(Debug, Eq, PartialEq)]
struct Segment {
    start: Point,
    end: Point,
}

impl Segment {
    fn new(start: Point, end: Point) -> Segment {
        Segment { start, end }
    }

    fn x(&self) -> Option<i32> {
        if self.start.x == self.end.x {
            Some(self.start.x)
        } else {
            None
        }
    }

    fn y(&self) -> Option<i32> {
        if self.start.y == self.end.y {
            Some(self.start.y)
        } else {
            None
        }
    }


    /// Calculates an intersection point of two segments
    fn intersection(&self, other: &Segment) -> Option<Point> {
        if let Some(x) = self.x() {
            if let Some(y) = other.y() {
                let y1 = cmp::min(self.start.y, self.end.y);
                let y2 = cmp::max(self.start.y, self.end.y);
                let x1 = cmp::min(other.start.x, other.end.x);
                let x2 = cmp::max(other.start.x, other.end.x);
                if (x1..=x2).contains(&x) && (y1..=y2).contains(&y) {
                    return Some(Point { x, y })
                }
            }
        }
        else if let Some(y) = self.y() {
            if let Some(x) = other.x() {
                let x1 = cmp::min(self.start.x, self.end.x);
                let x2 = cmp::max(self.start.x, self.end.x);
                let y1 = cmp::min(other.start.y, other.end.y);
                let y2 = cmp::max(other.start.y, other.end.y);
                if (x1..=x2).contains(&x) && (y1..=y2).contains(&y) {
                    return Some(Point { x, y })
                }
            }
        }

        None
    }

}

impl Point {
    /// Creates new segment starting at current point coordinates following the direction
    /// and mutating this point so that new coordinates are at the end of the given coordinates.
    fn next_segment(&mut self, dir: Direction) -> Segment {
        let start = *self; // Copy of self

        match dir {
            Direction::U(len) => { self.y += len as i32; }
            Direction::D(len) => { self.y -= len as i32; }
            Direction::R(len) => { self.x += len as i32; }
            Direction::L(len) => { self.x -= len as i32; }
        };

        // Return new segment
        Segment::new(start, *self)
    }
}

#[derive(Debug, Eq, PartialEq)]
struct Wire {
    segments: Vec<Segment>,
}

impl Wire {
    /// Creates a wire starting at the origin and following the sequence of directions
    fn from_directions(directions: Vec<Direction>) -> Wire {
        let segments: Vec<Segment> = directions.into_iter().scan(Point::origin(), |point, dir| {
            Some(point.next_segment(dir))
        }).collect();

        Wire { segments }
    }

    fn segments(&self) -> &[Segment] {
        self.segments.as_slice()
    }

    fn closest_intersection(&self, other: &Wire) -> Option<Point> {
        let mut is_first_segment = true;

        self.segments().into_iter().filter_map(|seg1| {
            let slice = if is_first_segment {
                is_first_segment = false;
                // Drop the firstsegment of other wires
                other.segments().split_first().map(|(_, rest)| { rest })
            } else {
                Some(other.segments())
            };

            slice.map(|segments| {
                segments.into_iter()
                        .filter_map(|seg2| { seg1.intersection(seg2)})
                        .min()
            }).flatten()
        }).min()
    }
}

#[derive(Debug, Eq, PartialEq)]
enum Direction {
    R(u32),
    L(u32),
    U(u32),
    D(u32),
}

#[derive(Eq, PartialEq)]
enum ParseDirectionError {
    EmptyStringError,
    InvalidDirectionError(char),
    InvalidLengthError(ParseIntError),
}

impl FromStr for Direction {
    type Err = ParseDirectionError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut chars = s.chars();

        let dir = chars.next()
            .ok_or(ParseDirectionError::EmptyStringError)?;

        let chars = chars.as_str();
        let len = chars.parse()
            .map_err(|e| ParseDirectionError::InvalidLengthError(e))?;

        match dir {
            'R' => Ok(Direction::R(len)),
            'L' => Ok(Direction::L(len)),
            'U' => Ok(Direction::U(len)),
            'D' => Ok(Direction::D(len)),

            _ => Err(ParseDirectionError::InvalidDirectionError(dir)),
        }
    }
}

impl fmt::Debug for ParseDirectionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self)
    }
}

impl fmt::Display for ParseDirectionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ParseDirectionError::EmptyStringError => {
                write!(f, "Expected direction but got nothing!")
            }
            ParseDirectionError::InvalidDirectionError(dir) => {
                write!(f, "Expected direction but got: {}", dir)
            }
            ParseDirectionError::InvalidLengthError(err) => {
                write!(f, "Failed to parse length: {}", err)
            }
        }
    }
}

pub fn day3(lines: Lines<BufReader<File>>) -> Result<(), String> {
    println!("Day 3 coming up real soon...");

    let lines = lines.collect::<io::Result<Vec<String>>>().map_err(|err| {
        format!("Failed to read a line: {}", err)
    })?;

    let wires = lines.into_iter().map(|line| {
        line.split(",")
            .map(|dir| dir.parse::<Direction>())
            .collect::<Result<Vec<Direction>, ParseDirectionError>>()
            .and_then(|directions| { Ok(Wire::from_directions(directions)) })
            .map_err(|err| { format!("{}", err) })
    }).collect::<Result<Vec<Wire>, String>>()?;


    let mut rest = wires.as_slice();
    let mut closest = None;
    while let Some((first, others)) = rest.split_first() {
        let candidate = others.into_iter()
            .filter_map(|wire| { first.closest_intersection(wire) })
            .min();

        closest = match (closest, candidate) {
            (p1, None) => p1,
            (None, p2) => p2,
            (Some(p1), Some(p2)) => Some(cmp::min(p1, p2)),
        };

        rest = others;
    }

    match closest {
        Some(point) => {
            println!("The closest wire intersect at {} with manhattan distance of {}", point, point.distance_from_origin());
        },
        None => {
            println!("All wires are neatly organized! None of them cross");
        },
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_can_parse_directions() {
        assert_eq!(Ok(Direction::U(7)), "U7".parse::<Direction>());
        assert_eq!(Ok(Direction::R(6)), "R6".parse::<Direction>());
        assert_eq!(Ok(Direction::L(4)), "L4".parse::<Direction>());
        assert_eq!(Ok(Direction::D(4)), "D4".parse::<Direction>());

        assert_eq!(
            Result::Err(ParseDirectionError::EmptyStringError),
            "".parse::<Direction>()
        );
        assert_eq!(
            Result::Err(ParseDirectionError::InvalidDirectionError('G')),
            "G20".parse::<Direction>()
        );
        assert_eq!(
            Result::Err(ParseDirectionError::EmptyStringError),
            "".parse::<Direction>()
        );
    }

    impl Point {
        fn new(x: i32, y: i32) -> Point {
            Point { x, y }
        }
    }

    #[test]
    fn it_can_build_segment() {
        let mut point = Point::origin();
        assert_eq!(
            Segment { start: Point{ x: 0, y: 0 }, end: Point { x: 0, y: 0 } },
            point.next_segment(Direction::U(7))
        );
        assert_eq!(Point { x: 0, y: 7}, point);
    }

    #[test]
    fn it_can_follow_directions() {
        assert_eq!(
            Wire {
                segments: vec![
                    Segment::new(Point::new(0, 0), Point::new(8, 0)),
                    Segment::new(Point::new(8, 0), Point::new(8, 5)),
                    Segment::new(Point::new(8, 5), Point::new(3, 5)),
                    Segment::new(Point::new(3, 5), Point::new(3, 2)),
                ]
            },
            Wire::from_directions(vec![
                Direction::R(8),
                Direction::U(5),
                Direction::L(5),
                Direction::D(3)
            ])
        )
    }
}
