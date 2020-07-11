mod day1;
use day1::*;
use std::vec::Vec;

fn run(command: &str) -> Result<(), String> {
  match command {
    "day1" => Ok(day1("day1.txt")),
    _ => Err(format!("Unrecognized command: {}", command))
  }
}

fn main() -> Result<(), String> {
  let args: Vec<String> = std::env::args().collect();
  if args.len() > 1 {
    run(&args[1])    
  } else {
    println!("No command?");
    Ok(())
  }
}
