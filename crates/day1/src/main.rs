use std::{env, fs, io};
use std::io::{BufRead, BufReader};

use atty::Stream;

const DEFAULT_FILENAME: &str = "day1.txt";

fn parse_calibration_value(s: &str) -> u32 {
    let digits: Vec<&str> = s.matches(|c: char| c.is_ascii_digit()).collect();

    let first_digit = digits.first().unwrap();
    let last_digit = digits.last().unwrap();

    format!("{first_digit}{last_digit}").parse().unwrap()
}

fn main() {
    let filename = env::args().skip(1).next().unwrap_or_else(|| DEFAULT_FILENAME.to_string());
    let reader: Box<dyn BufRead> = if filename == "-" && atty::is(Stream::Stdin) {
        Box::new(BufReader::new(io::stdin()))
    } else {
        Box::new(BufReader::new(fs::File::open(filename).unwrap()))
    };

    let mut result: u32 = 0;
    for line in reader.lines() {
        if let Ok(l) = line {
            result += parse_calibration_value(&l);
        }
    }

    println!("{result}");
}
