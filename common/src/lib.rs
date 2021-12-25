use std::fs::File;
use std::io;
use std::io::BufRead;
use std::path::{Path};

pub use problem;
// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
pub fn read_lines<P>(filename: P) -> io::Result<impl Iterator<Item=String>>
where P: AsRef<Path> {
  let file = File::open(filename)?;
  Ok(io::BufReader::new(file).lines().map(|l| l.unwrap()))
}

/// Common trait for all solution implementations
pub trait Solution {
  /// Type of the result. Typically this is some sort of number
  type Result: Sized;

  /// Type of the error.
  type Err;

  /// Calculate solution to part 1 of the puzzle
  fn part1(input: &Path) -> Result<Self::Result, Self::Err>;

  /// Calculate solution to part 2 of the puzzle
  fn part2(input: &Path) -> Result<Self::Result, Self::Err>;
}
