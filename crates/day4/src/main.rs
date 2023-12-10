use std::cmp::min;
use std::collections::HashSet;
use std::io::{BufRead, BufReader};
use std::{env, fs, io};

use anyhow::{bail, Context, Result};
use atty::Stream;

const DEFAULT_FILENAME: &str = "day4.txt";

fn main() -> Result<()> {
    let filename = env::args()
        .skip(1)
        .next()
        .unwrap_or_else(|| DEFAULT_FILENAME.to_string());

    let reader: Box<dyn BufRead> = if filename == "-" && atty::is(Stream::Stdin) {
        Box::new(BufReader::new(io::stdin()))
    } else {
        Box::new(BufReader::new(
            fs::File::open(&filename).with_context(|| format!("Opening file {filename:?}"))?,
        ))
    };

    let mut part1: u32 = 0;

    let mut lines: Vec<String> = reader.lines().filter_map(|line| line.ok()).collect();
    let mut multipliers: Vec<usize> = Vec::with_capacity(lines.len());
    multipliers.resize(lines.len(), 1usize);

    let mut index: usize = 0;
    while index < lines.len() {
        let line = lines
            .get(index)
            .with_context(|| format!("Reading line {index}"))?;
        let multiplier = multipliers[index];
        index += 1;

        let Some((name, line)) = line.split_once(": ") else {
            bail!("Parse card name");
        };

        let Some((winning_numbers, numbers_you_have)) = line.split_once(" | ") else {
            bail!("Winnings vs numbers I have");
        };

        let winning_numbers: HashSet<u32> = winning_numbers
            .split_whitespace()
            .filter_map(|s| s.parse::<u32>().ok())
            .collect();

        let numbers_you_have: HashSet<u32> = numbers_you_have
            .split_whitespace()
            .filter_map(|s| s.parse::<u32>().ok())
            .collect();

        let matching_numbers = winning_numbers.intersection(&numbers_you_have).count();
        eprintln!("❯ {name}: Found {matching_numbers} numbers matching the winning numbers");

        if matching_numbers > 0 {
            for i in index..min(index + matching_numbers, lines.len()) {
                multipliers[i] += multiplier;
            }

            let matching_numbers = matching_numbers as u32;
            let score = 2u32.pow(matching_numbers - 1);
            part1 += score;
        }
    }

    let part2 = multipliers.iter().sum::<usize>();
    println!("Day4, part1: {part1}");
    println!("Day4, part2: {part2}");

    Ok(())
}
