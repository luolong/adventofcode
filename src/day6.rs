mod common;

use std::{io::BufRead, iter::zip, str::FromStr};

use anyhow::{bail, Context, Ok, Result};
use itertools::{self, Itertools};

const DEFAULT_FILENAME: &str = "day6.txt";

type Time = u64;

type Distance = u64;

fn parse<T>(line: &str) -> Result<Vec<T>>
where
    T: FromStr + Sized,
{
    line.trim_start()
        .split_whitespace()
        .map(|s| {
            s.parse::<T>()
                .ok()
                .with_context(|| format!("Failed to parse: {s}"))
        })
        .try_collect()
}

fn brute_force(time: Time, distance: Distance) -> Vec<Time> {
    let (a, _) = (0..=time).partition(|n| {
        let d = n * (time - n);
        d > distance
    });

    a
}

pub fn main() -> Result<()> {
    let reader = common::get_reader(DEFAULT_FILENAME)?;

    let mut time: Time = Default::default();
    let mut distance: Distance = Default::default();

    let mut times: Vec<Time> = Vec::new();
    let mut distances: Vec<Distance> = Vec::new();

    for line in reader
        .lines()
        .map_while(Result::ok)
        .filter(|l| !l.is_empty())
    {
        let (head, tail) = line.split_once(": ").context("Reading line")?;
        match head {
            "Time" => {
                times.append(&mut parse(tail)?);
                time = tail.split_whitespace().join("").parse()?;
            }
            "Distance" => {
                distances.append(&mut parse(tail)?);
                distance = tail.split_whitespace().join("").parse()?;
            }
            _ => bail!("Invalid line headimg: {head}"),
        }
    }

    let part1: usize = zip(times, distances)
        .map(|(time, distance)| brute_force(time, distance))
        .map(|variants| variants.len())
        .product();

    println!("Day 6, Part 1: {part1}");

    let part2: usize = brute_force(time, distance).len();
    println!("Day 6, Part 2: {part2}");

    Ok(())
}
