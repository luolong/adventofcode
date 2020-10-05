use std::ops::{IndexMut, Index, Add, Mul, Deref};
use std::num::{ParseIntError, TryFromIntError};
use std::convert::TryInto;
use crate::day2::intcode::IntcodeError::{InvalidIntcode, UnrecognizedInstruction, InvalidInput, InvalidTarget};
use std::str::FromStr;
use std::fmt::Debug;

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub struct Int(isize);

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Intcode {
    memory: Vec<Int>,
    pointer: usize,
}

#[derive(Debug, Eq, PartialEq)]
pub enum IntcodeError {
    InvalidIntcode(ParseIntError),
    InvalidInput { pos: usize, err: TryFromIntError },
    InvalidTarget { err: TryFromIntError },
    UnrecognizedInstruction(Int),
}

impl Deref for Int {
    type Target = isize;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl FromStr for Int {
    type Err = IntcodeError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s.parse::<isize>()
            .map(|i| { Int(i) })
            .map_err(|err| { InvalidIntcode(err) })
    }
}

impl From<i32> for Int {
    fn from(v: i32) -> Self {
        Int(v as isize)
    }
}

impl Int {
    fn to_position(self) -> Result<usize, TryFromIntError> {
        self.0.try_into()
    }
}

impl Add for Int {
    type Output = Int;

    fn add(self, rhs: Self) -> Self::Output {
        Int(self.0 + rhs.0)
    }
}

impl Mul for Int {
    type Output = Int;

    fn mul(self, rhs: Self) -> Self::Output {
        Int(self.0 * rhs.0)
    }
}



impl FromStr for Intcode {
    type Err = IntcodeError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let state: Vec<Int> = input.trim_end()
            .split(",")
            .map(|number| number.parse::<Int>())
            .collect::<Result<Vec<Int>, IntcodeError>>()?;

        Ok(Intcode::new(state))
    }
}

impl Intcode {
    pub fn new(memory: Vec<Int>) -> Intcode {
        Intcode { memory, pointer: 0 }
    }
}

impl Intcode {
    fn peek(&mut self) -> Int {
        self[self.pointer]
    }

    /// Evaluates current instruction, returning size of the instruction.
    fn eval(&mut self) -> Result<usize, IntcodeError> {
        let instruction = self.peek();
        match instruction.0 {
            1 => self.eval_add(),
            2 => self.eval_mul(),
            99 => Ok(0),

            _ => Err(UnrecognizedInstruction(instruction)),
        }
    }

    /// evaluates current instruction as ADD, returninig 4 as the size value
    fn eval_add(&mut self) -> Result<usize, IntcodeError> {
        let instruction = self.pointer;

        let p1 = self[instruction + 1].to_position().map_err(|err| {
            InvalidInput { pos: 1, err } }
        )?;

        let p2 = self[instruction + 2].to_position().map_err(|err| {
            InvalidInput { pos: 2, err }
        })?;

        let target = self[instruction + 3].to_position().map_err(|err| {
            InvalidTarget { err }
        })?;

        self[target] = self[p1] + self[p2];

        Ok(4)
    }

    fn eval_mul(&mut self) -> Result<usize, IntcodeError> {
        let instruction = self.pointer;

        let p1 = self[instruction + 1].to_position().map_err(|err| {
            InvalidInput { pos: 1, err }
        })?;

        let p2 = self[instruction + 2].to_position().map_err(|err| {
            InvalidInput { pos: 1, err }
        })?;
        let target = self[instruction + 3].to_position().map_err(|err| {
            InvalidTarget { err }
        })?;

        self[target] = self[p1] * self[p2];

        Ok(4)
    }
}

impl Index<usize> for Intcode {
    type Output = Int;

    fn index(&self, address: usize) -> &Self::Output {
        &self.memory[address]
    }
}

impl IndexMut<usize> for Intcode {
    fn index_mut(&mut self, address: usize) -> &mut Self::Output {
        &mut self.memory[address]
    }
}

pub trait Run
    where Self::Err: Debug
{
    type Err;
    fn run(&mut self) -> Result<&mut Self, Self::Err>;
}

impl Run for Intcode {
    type Err = IntcodeError;

    /// Runs the Intcode machine from the current cursor position
    /// until it finishes or halts execution.
    fn run(&mut self) -> Result<&mut Self, Self::Err> {
        loop {
            let result = self.eval();
            match result {
                Ok(len) => {
                    if len == 0 {
                        // Halt instruction (99) will not move the instruction pointer.
                        // Halt the execution immediately with an Ok result
                        return Ok(self);
                    }

                    // Othrwise increase current cursor position by size of the
                    // evaluated instruction and return
                    self.pointer += len;
                    continue;
                }
                Err(message) => {
                    return Err(message);
                }
            }
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    impl Intcode {
        fn with_position(&mut self, address: usize) -> &mut Self {
            self.pointer = address;
            self
        }
    }

    impl From<Vec<isize>> for Intcode {
        fn from(state: Vec<isize>) -> Self {
            Intcode::new(state.iter().map(|i| { Int(*i) }).collect())
        }
    }

    #[test]
    fn it_can_execute_intcode() {
        assert_eq!(
            Intcode::from(vec![1, 9, 10, 3, 2, 3, 11, 0, 99, 30, 40, 50]).run(),
            Ok(Intcode::from(vec![3500, 9, 10, 70, 2, 3, 11, 0, 99, 30, 40, 50]).with_position(8))
        );
    }
}
