use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

/// # Calculate fuel required for the for the mass.
///
/// Fuel required to launch a given module is based on its mass.
/// Specifically, to find the fuel required for a module,
/// take its mass, divide by three, round down, and subtract 2.
pub fn get_fuel_for_mass(mass: i64) -> i64 {
  (mass / 3) - 2
}

/// # Calculate total fuel required for the module
///
/// So, for each module mass, calculate its fuel and add it to the total. 
/// Then, treat the fuel amount you just calculated as the input mass and repeat the process, 
/// continuing until a fuel requirement is zero or negative.
pub fn get_fuel_for_module(module_mass: i64) -> i64 {
  let mut total_fuel: i64 = 0;
  let fuel_for_module = get_fuel_for_mass(module_mass);
  for fuel_mass in additional_fuel(fuel_for_module) {
    total_fuel += fuel_mass
  }
  total_fuel
}

fn additional_fuel(mass: i64) -> impl std::iter::Iterator<Item = i64> {
  let mut fuel_mass = mass;
  std::iter::from_fn(move || {
    let result;
    if fuel_mass > 0 {
      result = Some(fuel_mass);
      fuel_mass = get_fuel_for_mass(fuel_mass);
    } else {
      result = None;
    }
    result
  })
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

pub fn day1<P>(filename: P) where P: AsRef<Path> {
  if let Ok(lines) = read_lines(filename) {
    let mut total_fuel: i64 =  0;
    for line in lines {
      let fuel_mass = line.unwrap().parse().unwrap();
      total_fuel += get_fuel_for_module(fuel_mass);
    }

    println!("Total fuel: {}", total_fuel);
  }
}


#[cfg(test)]
mod test {
  use super::*;
      
  #[test]
  fn it_calculates_fuel_for_mass() {
    assert_eq!(get_fuel_for_mass(12), 2);
    assert_eq!(get_fuel_for_mass(14), 2);
    assert_eq!(get_fuel_for_mass(1969), 654);
    assert_eq!(get_fuel_for_mass(100756), 33583);
  }
  
  #[test]
  fn it_calculates_fuel_for_module() {
    assert_eq!(get_fuel_for_module(14), 2);
    assert_eq!(get_fuel_for_module(1969), 966);
    assert_eq!(get_fuel_for_module(100756), 50346);
  }
}
