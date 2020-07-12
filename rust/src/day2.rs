use std::{
    convert::TryInto,
    ops::{Index, IndexMut},
};

#[derive(Debug, Eq, PartialEq)]
pub struct Intcode {
    state: Vec<isize>,
    cursor: usize,
}

impl Intcode {
    pub fn from_string(input: String) -> Result<Intcode, String> {
        println!("Input: [{}]", input);
        
        let numbers: Vec<&str> = input.trim_end().split(",").collect();
        let mut state = Vec::with_capacity(numbers.len());
        for number in numbers {
            let integer = number.parse().map_err(|e| {
                format!("Could not read intcode state: {}", e)
            })?;
            state.push(integer);
        }
        
        Ok(Intcode::new(state))
    }
    
    pub fn new(state: Vec<isize>) -> Intcode {
        Intcode {
            state: state,
            cursor: 0,
        }
    }

    /// Runs the Intcode machine from the current cursor position
    /// until it finishes or halts execution.
    pub fn run(&mut self) -> Result<&mut Self, String> {
        loop {
            let result = self.eval_next();
            match result {
                Ok(Some(_)) => continue,
                Ok(None) => {
                    return Ok(self);
                }
                Err(message) => {
                    return Err(message);
                }
            }
        }
    }

    fn next_value(&mut self) -> isize {
        let cursor = self.cursor;
        self.cursor += 1;
        self[cursor]
    }

    /// Evaluates next opcode, returning new cursor position
    fn eval_next(&mut self) -> Result<Option<&mut Self>, String> {
        let opcode = self.next_value();
        match opcode {
            1 => self.eval_add().map(|it| Some(it)),
            2 => self.eval_mul().map(|it| Some(it)),
            99 => Ok(None),
            _ => Err(format!("Unrecognized opcode: {}", opcode)),
        }
    }

    fn eval_add(&mut self) -> Result<&mut Self, String> {
        let p1 = self.next_value();
        let p1 = to_usize(p1).ok_or(format!("Invalid first input position: {}", p1))?;

        let p2 = self.next_value();
        let p2 = to_usize(p2).ok_or(format!("Invalid second input position: {}", p2))?;

        let target = self.next_value();
        let target = to_usize(target).ok_or(format!("Invalid target position: {}", target))?;

        self[target] = self[p1] + self[p2];

        Ok(self)
    }

    fn eval_mul(&mut self) -> Result<&mut Self, String> {
        let p1 = self.next_value();
        let p1 = to_usize(p1).ok_or(format!("Invalid first input position: {}", p1))?;

        let p2 = self.next_value();
        let p2 = to_usize(p2).ok_or(format!("Invalid second input position: {}", p2))?;

        let target = self.next_value();
        let target = to_usize(target).ok_or(format!("Invalid target position: {}", target))?;

        self[target] = self[p1] * self[p2];

        Ok(self)
    }
}

impl Index<usize> for Intcode {
    type Output = isize;

    fn index(&self, position: usize) -> &Self::Output {
        &self.state[position]
    }
}

impl IndexMut<usize> for Intcode {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.state[index]
    }
}

fn to_usize(value: isize) -> Option<usize> {
    value.try_into().ok()
}

pub fn day2(input: String) -> Result<(), String> {
    let mut intcode = Intcode::from_string(input)?;
    intcode[1] = 12;
    intcode[2] = 2;
    intcode.run()?;
    
    println!("Value at position 0 is {}", intcode[0]);
    Ok(())
}

#[cfg(test)]
mod test {
    use super::Intcode;

    impl Intcode {
        fn with_position(&mut self, position: usize) -> &mut Self {
            self.cursor = position;
            self
        }        
    }

    #[test]
    fn it_can_execute_intcode() {
        assert_eq!(
            Intcode::new(vec![1, 9, 10, 3, 2, 3, 11, 0, 99, 30, 40, 50]).run(),
            Ok(Intcode::new(vec![3500, 9, 10, 70, 2, 3, 11, 0, 99, 30, 40, 50]).with_position(9))
        );
    }
}
