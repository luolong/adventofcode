mod common;

use std::io::BufRead;

use crate::Char::{Blank, Digit, Symbol};
use anyhow::{bail, Result};
use itertools::izip;

const DEFAULT_FILENAME: &str = "day3.txt";

enum Char {
    Digit(char),
    Symbol(char),
    Blank,
}

impl From<char> for Char {
    fn from(value: char) -> Self {
        match value {
            '.' => Blank,
            _ if value.is_ascii_digit() => Digit(value),
            _ => Symbol(value),
        }
    }
}

fn main() -> Result<()> {
    let reader = common::get_reader(DEFAULT_FILENAME)?;

    let mut original_lines: Vec<String> = reader.lines().map_while(Result::ok).collect();

    let line_length = original_lines
        .first()
        .map(|s| s.chars().count())
        .unwrap_or_default();
    let blank_line = ".".repeat(line_length);

    let mut lines: Vec<String> = Vec::with_capacity(original_lines.len() + 2);
    lines.push(blank_line.to_string());
    lines.append(&mut original_lines);
    lines.push(blank_line.to_string());

    let mut part1 = 0u32;
    let mut part2 = 0u32;
    let lines = lines.as_slice();
    for window in lines.windows(3) {
        let [previous, current, next] = window else {
            bail!("Expected window of three lines")
        };

        eprintln!("\n{}", "=".repeat(line_length + 2));
        if !current.contains(|c: char| !c.is_ascii_digit() && c != '.') {
            eprintln!("Skipping:\n {} \n→{}←\n {} ", previous, current, next);
            continue;
        }

        let slice = izip!(
            previous.char_indices(),
            current.char_indices(),
            next.char_indices()
        );
        let sl = slice.map(|((i, a), (_, b), (_, c))| (i, [a, b, c]));

        let mut start: [Option<usize>; 3] = [None; 3];
        let mut symbol: Option<usize> = None;
        let mut gear: Option<usize> = None;

        let mut numbers: Vec<u32> = Vec::with_capacity(6);

        for (i, ccc) in sl {
            eprintln!("{} ↓", " ".repeat(i));
            let i0 = start[0].unwrap_or(i);
            eprintln!(
                "{}[{}]{}",
                previous.get(..i0).unwrap_or_default(),
                previous.get(i0..(i + 1)).unwrap_or_default(),
                previous.get((i + 1)..).unwrap_or_default()
            );
            let i0 = start[1].unwrap_or(i);
            eprintln!(
                "{}[{}]{}",
                current.get(..i0).unwrap_or_default(),
                current.get(i0..(i + 1)).unwrap_or_default(),
                current.get((i + 1)..).unwrap_or_default()
            );
            let i0 = start[2].unwrap_or(i);
            eprintln!(
                "{}[{}]{}",
                next.get(..i0).unwrap_or_default(),
                next.get(i0..(i + 1)).unwrap_or_default(),
                next.get((i + 1)..).unwrap_or_default()
            );
            eprintln!("{} ↑", " ".repeat(i));

            let mut res: [(Option<u32>, bool, bool); 3] = Default::default();
            for n in 0..3 {
                let (number, s, g) = match Char::from(ccc[n]) {
                    Digit(_) => {
                        if start[n].is_none() {
                            start[n] = Some(i);
                        }
                        if i + 1 == line_length {
                            let num = parse_number(&window[n], start[n], i + 1);
                            (num, false, false)
                        } else {
                            (None, false, false)
                        }
                    }
                    Symbol(c) => {
                        let num = parse_number(&window[n], start[n], i);
                        (num, true, c == '*')
                    }
                    Blank => {
                        let num = parse_number(&window[n], start[n], i);
                        (num, false, false)
                    }
                };

                eprint!("{} number: {number:?}", ["⎧", "⎬", "⎩"][n]);
                if s {
                    eprint!("; symbol");
                }
                if g {
                    eprint!("; gear");
                }
                eprintln!();

                res[n] = (number, s, g);
            }

            let (_, symbol1, gear1) = res[1];
            if symbol1 {
                symbol = Some(i);
            }

            if gear1 {
                gear = Some(i);
            }

            let numbers_with_start = izip!(res.map(|(num, _, _)| num), start)
                .filter_map(|(num, start)| num.and_then(|num| start.map(|start| (start, num))))
                .collect::<Vec<(usize, u32)>>();

            if let Some(symbol) = symbol {
                for (start, num) in numbers_with_start.iter() {
                    eprintln!("❯❯ start: {start}; symbol: {symbol}, number: {num}");
                    if *start <= symbol || start.abs_diff(symbol) < 2 {
                        part1 += *num;
                    }
                }
            }

            if let Some(gear) = gear {
                for (start, num) in numbers_with_start.iter() {
                    eprintln!("❯❯ start: {start}; gear: {gear}, number: {num}");
                    if *start <= gear || start.abs_diff(gear) < 2 {
                        numbers.push(*num);
                    }
                }
            }

            eprintln!("❯ symbol: {symbol:?}; gear: {gear:?}, numbers: {numbers:?}");
            eprintln!("❯ part1 = {part1}");
            eprintln!("❯ part2 = {part2}");
            if (['.', '.', '.'] == ccc || i + 1 == line_length) && !numbers.is_empty() {
                if numbers.len() > 1 {
                    let gear_ratio: u32 = numbers.iter().product();
                    eprint!("❯ part2: {part2} + {}", gear_ratio);
                    part2 += gear_ratio;
                    eprintln!(" = {part2}");
                }
                numbers.clear();
            }

            for n in 0..3 {
                if res[n].0.is_some() {
                    start[n] = None
                }
            }

            eprintln!("\n{}\n", "~".repeat(line_length + 2));
        }
        eprintln!("{}\n", "=".repeat(line_length + 2));
    }

    println!("Day3, Part1: {part1}");
    println!("Day3, Part2: {part2}");
    Ok(())
}

fn parse_number(line: &str, start: Option<usize>, end: usize) -> Option<u32> {
    let num = start
        .and_then(|it| line.get(it..end))
        .and_then(|s| s.parse::<u32>().ok());
    num
}
