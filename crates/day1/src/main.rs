use std::io::{BufRead, BufReader};
use std::{env, fs, io};

use atty::Stream;

const DEFAULT_FILENAME: &str = "day1.txt";

fn parse_calibration_value(s: &str) -> u32 {
    let digits: Vec<char> = s
        .char_indices()
        .filter_map(|(_, c)| match "0123456789".find(c) {
            Some(_) => Some(c),
            None => None,
        })
        .collect();

    if digits.is_empty() {
        0
    } else {
        let first_digit = digits.first().unwrap();
        let last_digit = digits.last().unwrap();

        format!("{first_digit}{last_digit}").parse().unwrap()
    }
}

fn parse_correct_calibration_value(s: &str) -> u32 {
    let digits: Vec<char> = s
        .char_indices()
        .filter_map(|(i, c)| {
            if let Some(_) = "0123456789".find(c) {
                Some(c)
            } else {
                match c {
                    'o' if s[i..].starts_with("one") => Some('1'),
                    't' if s[i..].starts_with("two") => Some('2'),
                    't' if s[i..].starts_with("three") => Some('3'),
                    'f' if s[i..].starts_with("four") => Some('4'),
                    'f' if s[i..].starts_with("five") => Some('5'),
                    's' if s[i..].starts_with("six") => Some('6'),
                    's' if s[i..].starts_with("seven") => Some('7'),
                    'e' if s[i..].starts_with("eight") => Some('8'),
                    'n' if s[i..].starts_with("nine") => Some('9'),
                    _ => None,
                }
            }
        })
        .collect();

    let first_digit = digits.first().unwrap();
    let last_digit = digits.last().unwrap();

    format!("{first_digit}{last_digit}").parse().unwrap()
}

fn main() {
    let filename = env::args()
        .skip(1)
        .next()
        .unwrap_or_else(|| DEFAULT_FILENAME.to_string());
    let reader: Box<dyn BufRead> = if filename == "-" && atty::is(Stream::Stdin) {
        Box::new(BufReader::new(io::stdin()))
    } else {
        Box::new(BufReader::new(fs::File::open(filename).unwrap()))
    };

    let mut part1: u32 = 0;
    let mut part2: u32 = 0;
    for line in reader.lines() {
        if let Ok(l) = line {
            if !l.is_empty() {
                part1 += parse_calibration_value(&l);
                part2 += parse_correct_calibration_value(&l);
            }
        }
    }

    println!("Part 1: {part1}");
    println!("Part 2: {part2}");
}
