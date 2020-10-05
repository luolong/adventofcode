mod intcode;
use intcode::{Intcode, Int, IntcodeError, Run};

trait RunExperiment: Run<Err = IntcodeError> {
    fn run_with(&mut self, noun: i32, verb: i32) -> Result<(), Self::Err>;
}

impl RunExperiment for Intcode {
    fn run_with(&mut self, noun: i32, verb: i32) -> Result<(), Self::Err> {
        self[1] = Int::from(noun);
        self[2] = Int::from(verb);
        self.run()?;
        Ok(())
    }
}

pub fn day2(input: String) -> Result<(), String> {
    let prototype: Intcode = input.parse().map_err(|err| {
        format!("Failed to parse input: {:?}", err)
    })?;

    for noun in 0..100 {
        for verb in 0..100 {
            let mut experiment = prototype.clone();
            experiment.run_with(noun, verb).map_err(|err| {
                format!("Failed to run the experiment: {:?}", err)
            })?;

            if experiment[0] == Int::from(19690720) {
                println!("(100 * {} + {}) = {}", noun, verb, 100 * noun + verb);
            }
        }
    }

    Ok(())
}
