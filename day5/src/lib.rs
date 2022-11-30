mod point;
mod segment;

use std::collections::BTreeSet;

use common::problem::Problem;
use common::{read_lines, Solution};
use segment::Segment;

pub struct Day5;

struct Part1;

impl Part1 {
    pub(crate) fn from<L: Iterator<Item = String>>(lines: L) -> impl Iterator<Item = Segment> {
        lines.filter_map(|l| {
            l.parse::<Segment>()
                .ok()
                .filter(|s| s.is_vertical() || s.is_horizontal())
        })
    }
}

struct Part2;

impl Part2 {
    pub(crate) fn from<L: Iterator<Item = String>>(lines: L) -> impl Iterator<Item = Segment> {
        lines.filter_map(|l| l.parse::<Segment>().ok())
    }
}

fn count_overlaps(slice: &[Segment]) -> usize {
    let mut points = BTreeSet::new();
    let mut segments = slice;
    while let Some((first, rest)) = segments.split_first() {
        points.extend(
            rest.into_iter()
                .filter_map(|s| s.intersection(first))
                .flat_map(|s| s),
        );
        segments = rest;
    }

    points.len()
}

impl Solution for Day5 {
    type Result = usize;
    type Err = Problem;

    fn part1(input: &std::path::Path) -> Result<Self::Result, Self::Err> {
        let lines = read_lines(input)?;
        let segments: Vec<Segment> = Part1::from(lines).collect();
        Ok(count_overlaps(segments.as_slice()))
    }

    fn part2(input: &std::path::Path) -> Result<Self::Result, Self::Err> {
        let lines = read_lines(input)?;
        let segments: Vec<Segment> = Part2::from(lines).collect();
        Ok(count_overlaps(segments.as_slice()))
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    macro_rules! segment {
        ($x1:literal,$y1:literal -> $x2:literal, $y2:literal) => {
            Segment::from((($x1, $y1), ($x2, $y2)))
        };
    }

    const PUZZLE_INPUT: &str = "
        0,9 -> 5,9
        8,0 -> 0,8
        9,4 -> 3,4
        2,2 -> 2,1
        7,0 -> 7,4
        6,4 -> 2,0
        0,9 -> 2,9
        3,4 -> 1,4
        0,0 -> 8,8
        5,5 -> 8,2
    ";

    #[test]
    fn sample_part_1_segments() {
        let lines = PUZZLE_INPUT.lines().map(|s| s.trim()).map(String::from);
        assert_eq!(
            vec![
                segment!(0,9 -> 5,9),
                segment!(9,4 -> 3,4),
                segment!(2,2 -> 2,1),
                segment!(7,0 -> 7,4),
                segment!(0,9 -> 2,9),
                segment!(3,4 -> 1,4),
            ],
            Part1::from(lines).collect::<Vec<Segment>>()
        );
    }

    #[test]
    #[ignore = "reasons"]
    fn sample_solution_to_part_1() {
        let lines = PUZZLE_INPUT.lines().map(|s| s.trim()).map(String::from);
        let segments: Vec<Segment> = Part1::from(lines).collect();

        assert_eq!(5, count_overlaps(segments.as_slice()));
    }

    #[test]
    fn sample_part_2_segments() {
        let lines = PUZZLE_INPUT.lines().map(|s| s.trim()).map(String::from);
        assert_eq!(
            vec![
                segment!(0,9 -> 5,9),
                segment!(8,0 -> 0,8),
                segment!(9,4 -> 3,4),
                segment!(2,2 -> 2,1),
                segment!(7,0 -> 7,4),
                segment!(6,4 -> 2,0),
                segment!(0,9 -> 2,9),
                segment!(3,4 -> 1,4),
                segment!(0,0 -> 8,8),
                segment!(5,5 -> 8,2),
            ],
            Part2::from(lines).collect::<Vec<Segment>>()
        );
    }

    #[test]
    #[ignore = "reasons"]
    fn sample_solution_to_part_2() {
        let lines = PUZZLE_INPUT.lines().map(|s| s.trim()).map(String::from);
        let segments: Vec<Segment> = Part2::from(lines).collect();
        assert_eq!(12, count_overlaps(segments.as_slice()));
    }
}
