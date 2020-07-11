use std::fmt::Debug;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

#[derive(Debug)]
struct Module {
    mass: i64
}

impl Module {
    
    fn new(mass: i64) -> Module {
        Module { mass: mass }
    }
    
    fn from_line(input: std::result::Result<std::string::String, std::io::Error>) -> Module {
      let mass = input.unwrap().parse::<i64>().unwrap();
      Module { mass: mass }
    }

    /// # Calculate fuel required for the module.
    ///
    /// Fuel required to launch a given module is based on its mass.
    /// Specifically, to find the fuel required for a module,
    /// take its mass, divide by three, round down, and subtract 2.
    ///
    /// # Example
    /// ```
    /// assert_eq!(Module::new(12).get_fuel_required(), 2);
    /// assert_eq!(Module::new(14).get_fuel_required(), 2);
    /// assert_eq!(Module::new(1969).get_fuel_required(), 654);
    /// assert_eq!(Module::new(100756).get_fuel_required(), 33583);
    /// ```

    fn get_fuel_required(&self) -> i64 {
        (&self.mass / 3) - 2
    }
}

fn day1() {
  if let Ok(lines) = read_lines("./day1.txt") {
    let total_mass = lines.map(Module::from_line)
         .map(|module| { module.get_fuel_required() })
         .fold(0, |total, mass| { total + mass });
     println!("Total mass of all modules is {}", total_mass);
  }
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn main() {
    day1()
}
