use std::path::Path;

use problem::{FailedToIter, Problem, ProblemWhile};

use common::{read_lines, Solution};

#[derive(Default)]
pub struct Day1;

impl Solution for Day1 {
  type Result = usize;
  type Err = Problem;

  fn part1(input: &Path) -> Result<Self::Result, Self::Err> {
    let lines = read_lines(input)?;
    let measurements = lines.map(|line|
      line.parse::<u32>().problem_while("parsing a measurement")
    ).or_failed_to("read input");

    let measurements = measurements.collect();
    let result = count_increases(measurements);
    Ok(result)
  }

  fn part2(input: &Path) -> Result<Self::Result, Self::Err> {
    let lines = read_lines(input)?;
    let measurements = lines.map(|line|
      line.parse::<u32>().problem_while("parsing a measurement")
    ).or_failed_to("read input");

    let measurements = measurements.collect();
    let result = count_sliding_increases(measurements);
    Ok(result)
  }
}

fn count_increases(measurements: Vec<u32>) -> usize {
  let result: (usize, Option<u32>) = measurements.into_iter().fold((0usize, None), |state, item| {
    if let Some(prev) = state.1 {
      let count = if item > prev { state.0 + 1 } else { state.0 };
      (count, Some(item))
    } else {
      (state.0, Some(item))
    }
  });

  result.0
}

fn count_sliding_increases(measurements: Vec<u32>) -> usize {
  let vec: Vec<u32> = measurements.windows(3).into_iter()
    .map(|window| { window.into_iter().sum() })
    .collect();
  count_increases(vec)
}

#[cfg(test)]
mod tests {
  use crate::count_increases;
  use crate::count_sliding_increases;

  #[test]
  fn part_1_works() {
    let result = count_increases(vec![
      199,
      200,
      208,
      210,
      200,
      207,
      240,
      269,
      260,
      263,
    ]);
    assert_eq!(result, 7);
  }

  # [test]
  fn part_2_works() {
    let result = count_sliding_increases(vec![
      199,
      200,
      208,
      210,
      200,
      207,
      240,
      269,
      260,
      263,
    ]);
    assert_eq!(result, 5);
  }
}
