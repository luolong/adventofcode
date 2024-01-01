use anyhow::Result;
use std::io::BufRead;

const DEFAULT_FILENAME: &str = "day1.txt";

fn parse_calibration_value(s: &str) -> u32 {
    let digits: Vec<char> = s
        .char_indices()
        .filter_map(|(_, c)| "0123456789".find(c).map(|_| c))
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
            if "0123456789".find(c).is_some() {
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

fn main() -> Result<()> {
    let reader = common::get_reader(DEFAULT_FILENAME)?;

    let mut part1: u32 = 0;
    let mut part2: u32 = 0;
    for line in reader.lines().map_while(Result::ok) {
        if !line.is_empty() {
            part1 += parse_calibration_value(&line);
            part2 += parse_correct_calibration_value(&line);
        }
    }

    println!("Part 1: {part1}");
    println!("Part 2: {part2}");

    Ok(())
}
