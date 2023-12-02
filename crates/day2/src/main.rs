use std::{env, fs, io};
use std::cmp::max;
use std::fmt::{Display, Formatter};
use std::io::{BufRead, BufReader};
use std::iter::zip;
use std::ops::AddAssign;
use std::str::FromStr;

use anyhow::{anyhow, Context, Result};
use atty::Stream;

const DEFAULT_FILENAME: &str = "day2.txt";

#[derive(Default, Debug)]
struct Set(u32, u32, u32);

impl AddAssign for Set {
    fn add_assign(&mut self, rhs: Self) {
        if rhs.0 > 0 {
            self.0 += rhs.0
        }
        if rhs.1 > 0 {
            self.1 += rhs.1
        }
        if rhs.2 > 0 {
            self.2 += rhs.2
        }
    }
}

impl FromStr for Set {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut set: Set = Default::default();

        let unparsed_set = s.trim()
            .split_terminator(',')
            .map(|s| s.trim().to_string())
            .collect::<Vec<String>>();

        for cubes in unparsed_set {
            let (count, color) = cubes
                .split_once(' ')
                .with_context(|| format!("Parsing '{cubes}' as set"))?;

            let count: u32 = count
                .parse()
                .with_context(|| format!("Not a number: {}", count))?;

            set += match color {
                "red" => Ok(Set(count, 0, 0)),
                "green" => Ok(Set(0, count, 0)),
                "blue" => Ok(Set(0, 0, count)),
                _ => Err(anyhow!("Unrecognized color: '{color}'")),
            }?;
        }
        Ok(set)
    }
}

impl Display for Set {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let Set(red, green, blue) = self;
        let mut cubes =
            zip([red, green, blue], ["red", "green", "blue"]).filter(|(count, _)| **count > 0);

        if let Some((count, color)) = cubes.next() {
            write!(f, "{count} {color}")?;
        }

        while let Some((count, color)) = cubes.next() {
            write!(f, ", {count} {color}")?;
        }

        Ok(())
    }
}

#[derive(Debug)]
struct Game {
    id: u32,
    sets: Vec<Set>,
}

impl FromStr for Game {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let (name, rest) = s.split_once(':').context("Parsing game name")?;
        let name = name.trim().to_string();

        let mut sets: Vec<Set> = Vec::with_capacity(3);
        let segments = rest.split_terminator(';');

        for unparsed_set in segments {
            let set: Set = unparsed_set.parse()
                .with_context(|| format!("Parsing game subset: {unparsed_set}"))?;

            sets.push(set);
        }

        let id: u32 = name
            .rsplit_once(' ')
            .context("Parsing game id")?
            .1
            .parse()?;

        Ok(Game { id, sets })
    }
}

impl Display for Game {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let Game { id, sets } = self;
        write!(f, "Game {id}: ")?;

        let mut sets = sets.iter();
        if let Some(set) = sets.next() {
            write!(f, "{set}")?;
        }

        while let Some(set) = sets.next() {
            write!(f, "; {set}")?;
        }

        Ok(())
    }
}

fn main() -> Result<()> {
    let filename = env::args()
        .skip(1)
        .next()
        .unwrap_or_else(|| DEFAULT_FILENAME.to_string());

    let reader: Box<dyn BufRead> = if filename == "-" && atty::is(Stream::Stdin) {
        Box::new(BufReader::new(io::stdin()))
    } else {
        Box::new(BufReader::new(
            fs::File::open(&filename).with_context(|| format!("Reading from {filename:?}"))?,
        ))
    };

    let mut part1 = 0u32;
    let mut part2 = 0u32;
    for line in reader.lines() {
        let line = line.context("reading line of input")?;
        if line.is_empty() {
            continue;
        }

        let game: Game = line.parse()?;
        let max = game.sets.iter().fold(Set::default(), |a, b| {
            Set(max(a.0, b.0), max(a.1, b.1), max(a.2, b.2))
        });

        let Set(red, green, blue) = max;
        if red <= 12 && green <= 13 && blue <= 14 {
            part1 += game.id;
        }

        let power = red * green * blue;
        part2 += power;
    }

    println!("Day2, Part 1: {part1}");
    println!("Day2, Part 2: {part2}");

    Ok(())
}
