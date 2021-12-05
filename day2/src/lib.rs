use std::path::Path;
use std::str::FromStr;

use problem::{FailedTo, FailedToIter, OkOrProblem, Problem, problem, ProblemWhile};

use common::{read_lines, Solution};

use crate::Instruction::{Down, Forward, Up};

#[derive(Debug)]
enum Instruction {
  Forward(u32),
  Down(u32),
  Up(u32),
}

impl FromStr for Instruction {
  type Err = Problem;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    let split= s.split_once(' ')
      .ok_or_problem("Invalid instruction:")?;

    let amount: u32 = split.1.parse()?;

    match split.0 {
      "forward" => Ok(Forward(amount)),
      "down" => Ok(Down(amount)),
      "up" => Ok(Up(amount)),
      _ => problem!("Unrecognized instruction: {}", split.0),
    }
  }
}

#[derive(Debug, Default)]
struct Position(u32, u32, u32);

impl Position {

  fn product(&self) -> u32 {
    self.0 * self.1
  }
}

impl Position {

  fn forward(&mut self, amount: u32) {
    self.0 += amount
  }

  fn down(&mut self, amount: u32) {
    self.1 += amount;
  }

  fn up(&mut self, amount: u32) {
    self.1 -= amount
  }

  fn apply(&mut self, cmd: Instruction) {
    match cmd {
      Forward(amount) => self.forward(amount),
      Down(amount) => self.down(amount),
      Up(amount) => self.up(amount),
    }
  }

  fn apply_with_aim(&mut self, cmd: Instruction) {
    match cmd {
      Forward(forward) => {
        self.forward(forward);
        if self.2 > 0 {
          self.down(self.2 * forward)
        }
      }
      Down(aim) => self.2 += aim,
      Up(aim) => self.2 -= aim,
    }
  }

}

fn interpret_movement_instructions(movements: Vec<Instruction>) -> u32 {
  let mut position = Position::default();
  movements.into_iter().for_each(|mov| { position.apply(mov) });

  position.product()
}

fn interpret_movement_instructions_with_aim(movements: Vec<Instruction>) -> u32 {
  let mut position = Position::default();
  movements.into_iter().for_each(|mov| { position.apply_with_aim(mov) });

  position.product()
}

pub struct Day2;

impl Solution for Day2 {
  type Result = u32;
  type Err = Problem;

  fn part1(input: &Path) -> Result<Self::Result, Self::Err> {
    let lines = read_lines(input)
      .or_failed_to("read input file");

    let instructions = lines.map(|l|
      l.problem_while("reading an instruction")
        .and_then(|line| line.parse::<Instruction>())
    ).or_failed_to("read input");

    let instructions: Vec<Instruction> = instructions.collect();
    let result = interpret_movement_instructions(instructions);
    Ok(result)
  }

  fn part2(input: &Path) -> Result<Self::Result, Self::Err> {
    let lines = read_lines(input)
      .or_failed_to("read input file");

    let instructions = lines.map(|l|
      l.problem_while("reading an instruction")
        .and_then(|line| line.parse::<Instruction>())
    ).or_failed_to("read input");

    let instructions: Vec<Instruction> = instructions.collect();
    let result = interpret_movement_instructions_with_aim(instructions);
    Ok(result)
  }
}

#[cfg(test)]
mod tests {
  use crate::{interpret_movement_instructions, Down, Forward, Up, interpret_movement_instructions_with_aim};

  #[test]
  fn part1_works() {
    let result = interpret_movement_instructions(vec![
      Forward(5),
      Down(5),
      Forward(8),
      Up(3),
      Down(8),
      Forward(2),
    ]);
    assert_eq!(result, 150);
  }

  #[test]
  fn part2_works() {
    let result = interpret_movement_instructions_with_aim(vec![
      Forward(5),
      Down(5),
      Forward(8),
      Up(3),
      Down(8),
      Forward(2),
    ]);
    assert_eq!(result, 900);
  }
}
