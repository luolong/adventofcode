mod common;

use anyhow::Result;
use std::io::BufRead;

const DEFAULT_FILENAME: &str = "day8.txt";

fn main() -> Result<()> {
    let mut reader = common::get_reader(DEFAULT_FILENAME)?;

    let mut instructions = String::new();
    reader.read_line(&mut instructions)?;

    println!("{instructions}");

    let part1 = 0usize;

    println!("Part 1: {part1}");
    println!("Part 2: (not implemented)");

    Ok(())
}
