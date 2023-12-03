use std::{env, fs, io};
use std::io::{BufRead, BufReader};

use anyhow::{bail, Context, Result};
use atty::Stream;

const DEFAULT_FILENAME: &str = "day3.txt";

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

    let mut original_lines: Vec<String> = reader.lines().filter_map(|l| l.ok()).collect();
    let original_lines_count = original_lines.len();

    let line_length = original_lines.first().map(|s| s.chars().count()).unwrap_or_default();
    let blank_line = ".".repeat(line_length);

    let mut lines: Vec<String> = Vec::with_capacity(original_lines_count + 2);
    lines.push(blank_line.to_string());
    lines.append(&mut original_lines);
    lines.push(blank_line.to_string());

    let mut part1 = 0u32;
    let lines = lines.as_slice();
    for slice in lines.windows(3) {
        let [previous, current, next] = slice else { bail!("Expected window of three lines") };
        let mut previous = previous.as_str();
        let mut current = current.as_str();
        let mut next = next.as_str();

        let mut symbol: bool = false;

        while !current.is_empty() {
            let number = if current.starts_with(|c: char| c.is_ascii_digit()) {
                let n: String = current.chars().take_while(|c| c.is_ascii_digit()).collect();
                Some(n)
            } else {
                None
            };

            if let Some(n) = &number {
                let (a_prefix, a_rest) = previous.split_at(n.len());
                previous = a_rest;

                (_, current) = current.split_at(n.len());

                let (c_prefix, c_rest) = next.split_at(n.len());
                next = c_rest;

                symbol |= [a_prefix, c_prefix].join("")
                    .contains(|c: char| !(c.is_ascii_digit() || c == '.'));

                if symbol {
                    eprintln!("Found a part number: {n}");
                    part1 += n.parse::<u32>()?;
                    symbol = a_prefix.ends_with(|c: char| !(c.is_ascii_digit() || c == '.'))
                          || c_prefix.ends_with(|c: char| !(c.is_ascii_digit() || c == '.'));
                }
            }

            if current.is_empty() {
                break;
            }

            let (a, a_rest) = previous.split_at(1);
            let (b, b_rest) = current.split_at(1);
            let (c, c_rest) = next.split_at(1);

            previous = a_rest;
            current = b_rest;
            next = c_rest;

            symbol = [a, b, c].join("")
                .contains(|c: char| !(c.is_ascii_digit() || c == '.'));

            if let Some(n) = &number {
                if symbol {
                    //eprintln!("Found a part number: {n}");
                    part1 += n.parse::<u32>()?;
                }
            }
        }
    }


    println!("Day3, Part1: {part1}");
    Ok(())
}
