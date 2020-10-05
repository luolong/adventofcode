use std::fmt;
use std::num::ParseIntError;
use std::str::FromStr;

use crate::day3::direction::Direction::{D, L, R, U};

#[derive(Debug, Eq, PartialEq)]
pub(crate) enum Direction {
    R(u32),
    L(u32),
    U(u32),
    D(u32),
}

pub trait Vector {
    fn x(&self) -> i32;
    fn y(&self) -> i32;
}

#[derive(Eq, PartialEq)]
pub(crate) enum ParseDirectionError {
    Empty,
    InvalidDirection(char),
    InvalidLength(ParseIntError),
}

impl FromStr for Direction {
    type Err = ParseDirectionError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut chars = s.chars();

        let dir = chars.next().ok_or(ParseDirectionError::Empty)?;

        let chars = chars.as_str();
        let len = chars
            .parse()
            .map_err(|e| ParseDirectionError::InvalidLength(e))?;

        match dir {
            'R' => Ok(Direction::R(len)),
            'L' => Ok(Direction::L(len)),
            'U' => Ok(Direction::U(len)),
            'D' => Ok(Direction::D(len)),

            _ => Err(ParseDirectionError::InvalidDirection(dir)),
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
            ParseDirectionError::Empty => {
                write!(f, "Expected direction but got nothing!")
            }
            ParseDirectionError::InvalidDirection(dir) => {
                write!(f, "Expected direction but got: {}", dir)
            }
            ParseDirectionError::InvalidLength(err) => {
                write!(f, "Failed to parse length: {}", err)
            }
        }
    }
}

impl Vector for Direction {
    fn x(&self) -> i32 {
        match self {
            R(x) => *x as i32,
            L(x) => -(*x as i32),
            _ => 0,
        }
    }

    fn y(&self) -> i32 {
        match self {
            U(y) => *y as i32,
            D(y) => -(*y as i32),
            _ => 0
        }
    }
}

#[cfg(test)]
mod tests {
    use std::num::ParseIntError;

    use super::{
        Direction::{self, *}, ParseDirectionError::{Empty, InvalidDirection, InvalidLength},
        Vector,
    };

    fn parse_error(input: &str) -> ParseIntError {
        input.parse::<u32>().err().unwrap()
    }

    #[test]
    fn it_can_parse_directions() {
        assert_eq!("U7".parse::<Direction>(), Ok(Direction::U(7)));
        assert_eq!("R6".parse::<Direction>(), Ok(Direction::R(6)));
        assert_eq!("L4".parse::<Direction>(), Ok(Direction::L(4)));
        assert_eq!("D4".parse::<Direction>(), Ok(Direction::D(4)));

        assert_eq!("".parse::<Direction>(),            Err(Empty));
        assert_eq!("G20".parse::<Direction>(),         Err(InvalidDirection('G')));
        assert_eq!("D".parse::<Direction>(),           Err(InvalidLength(parse_error(""))));
        assert_eq!("Do".parse::<Direction>(),          Err(InvalidLength(parse_error("o"))));
        assert_eq!("D4294967296".parse::<Direction>(), Err(InvalidLength(parse_error("4294967296"))));
        assert_eq!("D-2".parse::<Direction>(),         Err(InvalidLength(parse_error("-2"))));
    }

    #[test]
    fn it_is_also_a_vector() {
        let d = D(1);
        assert_eq!((d.x(), d.y()), (0, -1));

        let u = U(2);
        assert_eq!((u.x(), u.y()), (0, 2));

        let r = R(3);
        assert_eq!((r.x(), r.y()), (3, 0));

        let l = L(5);
        assert_eq!((l.x(), l.y()), (-5, 0));
    }
}