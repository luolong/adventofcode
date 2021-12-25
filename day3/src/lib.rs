use std::path::Path;

use problem::Problem;

use common::{read_lines, Solution};

pub struct Day3;

impl Solution for Day3 {
  type Result = usize;
  type Err = Problem;

  fn part1(input: &Path) -> Result<Self::Result, Self::Err> {
    let lines = read_lines(input)?;
    let input: Vec<String> = lines.collect();
    Ok(calculate_power_consumption(&input))
  }

  fn part2(input: &Path) -> Result<Self::Result, Self::Err> {
    let lines = read_lines(input)?;
    let input: Vec<String> = lines.collect();
    Ok(caclulate_life_support_rating(&input))
  }
}

fn calculate_power_consumption(input: &[String]) -> usize {
  let gamma_rate = calculate_gamma_rate(&input);
  let epsilon_rate = calculate_epsilon_rate(&input);
  gamma_rate * epsilon_rate
}

fn caclulate_life_support_rating(input: &[String]) -> usize {
  let oxy_gen = calculate_oxygen_generator_rating(&input);
  let co2_scr = calculate_co2_scrubber_rating(&input);
  oxy_gen * co2_scr
}

fn calculate_gamma_rate(report: &[String]) -> usize {
  let ones = convert_to_ones_and_zeroes(report);
  let half = ones.len() / 2;
  decode_based_on_bit_counts(ones, |count| count > half)
}

fn calculate_epsilon_rate(report: &[String]) -> usize {
  let ones = convert_to_ones_and_zeroes(report);
  let half = ones.len() / 2;
  decode_based_on_bit_counts(ones, |count| count <= half)
}

fn calculate_oxygen_generator_rating(report: &[String]) -> usize {
  let mut ones = convert_to_ones_and_zeroes(report);
  reduce_to_single_element(&mut ones, |count, total| {
    if count + count >= total { 1 } else { 0 }
  });
  ones.first().and_then(convert_to_uize).unwrap_or(0)
}

fn calculate_co2_scrubber_rating(report: &[String]) -> usize {
  let mut ones = convert_to_ones_and_zeroes(report);
  reduce_to_single_element(&mut ones, |count, total| {
    if count + count >= total { 0 } else { 1 }
  });
  ones.first().and_then(convert_to_uize).unwrap_or(0)
}

fn convert_to_uize(vec: &Vec<usize>) -> Option<usize> {
  vec.clone().into_iter().reduce(|acc, bit| (acc << 1) | bit)
}

fn reduce_to_single_element<F>(mut ones: &mut Vec<Vec<usize>>, f: F)
  where F: Fn(usize, usize) -> usize
{
  if let Some(len) = ones.into_iter().map(|v| v.len()).max() {
    let mut index = 0;
    while ones.len() > 1 && index < len {
      let total = ones.len();
      retain_based_on_bit_count_at(&mut ones, index, |count| f(count, total));
      index += 1;
    }
  }
}

fn decode_based_on_bit_counts<F>(ones: Vec<Vec<usize>>, f: F) -> usize
where
  F: Fn(usize) -> bool
{

  if let Some((first, rest)) = ones.split_first() {
    let counts = count_ones(first, rest);

    let result: usize = counts.into_iter().fold(0usize, |acc, count| {
      if f(count) { (acc << 1) | 1 } else { acc << 1 }
    });

    result
  } else {
    0
  }
}

fn retain_based_on_bit_count_at<F>(input: &mut Vec<Vec<usize>>, index: usize, f: F)
  where F: Fn(usize) -> usize
{
  if let Some((first, rest)) = input.split_first() {
    let retain: usize = f(count_ones(first, rest)[index]);
    input.retain(|vec| vec[index] == retain);
  }
}

fn count_ones(first: &Vec<usize>, rest: &[Vec<usize>]) -> Vec<usize> {
  let counts = rest.into_iter().fold(first.to_vec(), |acc, elem| {
    Vec::from_iter(acc.iter().zip(elem).map(|(a, b)| a + b))
  });
  counts
}

fn convert_to_ones_and_zeroes(report: &[String]) -> Vec<Vec<usize>> {
  let ones: Vec<Vec<usize>> = report.into_iter().map(|s| {
    s.chars().map(|c| { if c == '1' { 1 } else { 0 } }).collect::<Vec<usize>>()
  }).collect();
  ones
}

#[cfg(test)]
mod tests {
  use crate::{caclulate_life_support_rating, calculate_co2_scrubber_rating, calculate_epsilon_rate, calculate_gamma_rate, calculate_oxygen_generator_rating, calculate_power_consumption};

  const INPUT: [&str;12] = [
    "00100",
    "11110",
    "10110",
    "10111",
    "10101",
    "01111",
    "00111",
    "11100",
    "10000",
    "11001",
    "00010",
    "01010",
  ];

  #[test]
  fn it_calculates_correct_power_consumption() {
    let result = calculate_power_consumption(&INPUT.map(|s| s.to_string()));
    assert_eq!(result, 198);
  }

  #[test]
  fn it_calculates_correct_gamma_rate() {
    let result = calculate_gamma_rate(&INPUT.map(|s| s.to_string()));
    assert_eq!(result, 22);
  }

  #[test]
  fn it_calculates_correct_epsilon_rate() {
    let result = calculate_epsilon_rate(&INPUT.map(|s| s.to_string()));
    assert_eq!(result, 9);
  }

  #[test]
  fn it_calculates_correct_life_support_rating() {
    let result = caclulate_life_support_rating(&INPUT.map(|s| s.to_string()));
    assert_eq!(result, 230);
  }

  #[test]
  fn it_calculates_correct_oxygen_generator_rating() {
    let result = calculate_oxygen_generator_rating(&INPUT.map(|s| s.to_string()));
    assert_eq!(result, 23);
  }

  #[test]
  fn it_calculates_correct_co2_scrubber_rating() {
    let result = calculate_co2_scrubber_rating(&INPUT.map(|s| s.to_string()));
    assert_eq!(result, 10);
  }
}
