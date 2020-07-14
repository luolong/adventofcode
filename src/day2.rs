use std::{
    convert::TryInto,
    ops::{Index, IndexMut},
};

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Intcode {
    memory: Vec<isize>,
    pointer: usize,
}


impl Intcode {
    pub fn from_string(input: String) -> Result<Intcode, String> {        
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
    
    pub fn new(memory: Vec<isize>) -> Intcode {
        Intcode { memory, pointer: 0 }
    }

    /// Runs the Intcode machine from the current cursor position
    /// until it finishes or halts execution.
    pub fn run(&mut self) -> Result<&mut Self, String> {
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
                    continue
                },
                Err(message) => {
                    return Err(message);
                }
            }
        }
    }

    fn peek(&mut self) -> isize {
        self[self.pointer]
    }

    /// Evaluates current instruction, returning size of the instruction.
    fn eval(&mut self) -> Result<usize, String> {
        let instruction = self.peek();
        match instruction {
            1 => self.eval_add(),
            2 => self.eval_mul(),
            99 => Ok(0),
            
            _ => Err(format!("Unrecognized instruction: {}", instruction)),
        }
    }

    /// evaluates current instruction as ADD, returninig 4 as the size value
    fn eval_add(&mut self) -> Result<usize, String> {
        let instruction = self.pointer;
        
        let p1 = self[instruction + 1];
        let p1 = to_usize(p1).ok_or(format!("Invalid first input position: {}", p1))?;

        let p2 = self[instruction + 2];
        let p2 = to_usize(p2).ok_or(format!("Invalid second input position: {}", p2))?;

        let target = self[instruction + 3];
        let target = to_usize(target).ok_or(format!("Invalid target position: {}", target))?;

        self[target] = self[p1] + self[p2];

        Ok(4)
    }

    fn eval_mul(&mut self) -> Result<usize, String> {
        let instruction = self.pointer;

        let p1 = self[instruction + 1];
        let p2 = self[instruction + 2];
        let target = self[instruction + 3];

        let p1 = to_usize(p1).ok_or(format!("Invalid first input position: {}", p1))?;
        let p2 = to_usize(p2).ok_or(format!("Invalid second input position: {}", p2))?;
        let target = to_usize(target).ok_or(format!("Invalid target position: {}", target))?;

        self[target] = self[p1] * self[p2];

        Ok(4)
    }
}

impl Index<usize> for Intcode {
    type Output = isize;

    fn index(&self, address: usize) -> &Self::Output {
        &self.memory[address]
    }
}

impl IndexMut<usize> for Intcode {
    fn index_mut(&mut self, address: usize) -> &mut Self::Output {
        &mut self.memory[address]
    }
}

fn to_usize(value: isize) -> Option<usize> {
    value.try_into().ok()
}

pub fn day2(input: String) -> Result<(), String> {
    let prototype = Intcode::from_string(input)?;
    
    for noun in 0..100 {
        for verb in 0..100 {
            let mut experiment = prototype.clone();
            experiment[1] = noun;
            experiment[2] = verb;
            experiment.run()?;
            if experiment[0] == 19690720 {
                println!("(100 * {} + {}) = {}", noun, verb, 100 * noun + verb);
            }
        }
    }
    
    Ok(())
}

#[cfg(test)]
mod test {
    use super::Intcode;

    impl Intcode {
        fn with_position(&mut self, address: usize) -> &mut Self {
            self.pointer = address;
            self
        }        
    }

    #[test]
    fn it_can_execute_intcode() {
        assert_eq!(
            Intcode::new(vec![1, 9, 10, 3, 2, 3, 11, 0, 99, 30, 40, 50]).run(),
            Ok(Intcode::new(vec![3500, 9, 10, 70, 2, 3, 11, 0, 99, 30, 40, 50]).with_position(8))
        );
    }
}
