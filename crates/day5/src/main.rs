use std::fmt::{Debug, Formatter};
use std::io::{BufRead, BufReader, Lines};
use std::str::FromStr;
use std::{env, fs, io, usize};

use anyhow::{bail, Context, Error, Result};
use atty::Stream;
use itertools::Itertools;

const DEFAULT_FILENAME: &str = "day5.txt";

struct RangeMapEntry {
    source_range_start: usize,
    destination_range_start: usize,
    range_length: usize,
}

impl RangeMapEntry {
    pub fn get(&self, key: usize) -> Option<usize> {
        let RangeMapEntry {
            source_range_start,
            destination_range_start,
            range_length,
        } = *self;

        let source_range_start_end = source_range_start + range_length;
        if (source_range_start..source_range_start_end).contains(&key) {
            let delta = source_range_start.abs_diff(key);
            Some(destination_range_start + delta)
        } else {
            None
        }
    }
}

impl FromStr for RangeMapEntry {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (destination_range_start, source_range_start, range_length) = s
            .split_ascii_whitespace()
            .filter_map(|s| s.parse::<usize>().ok())
            .collect_tuple()
            .with_context(|| format!("Expected three numbers, but got \"{s}\""))?;

        Ok(RangeMapEntry {
            destination_range_start,
            source_range_start,
            range_length,
        })
    }
}
impl Debug for RangeMapEntry {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let RangeMapEntry {
            destination_range_start,
            source_range_start,
            range_length,
        } = self;

        let source_range_end = source_range_start + range_length;
        let destination_range_end = destination_range_start + range_length;
        let delta = (self.destination_range_start as i128) - (self.source_range_start as i128);

        write!(
            f,
            "{source_range_start}..{source_range_end} → {destination_range_start}..{destination_range_end} (∆{delta})"
        )
    }
}

fn read_until_header(lines: &mut Lines<Box<dyn BufRead>>, expected_header: &str) -> Result<()> {
    while let Some(line) = lines.next() {
        let Ok(line) = line else {
            bail!("Failed to read a line");
        };

        if line == expected_header {
            //eprintln!("\n{line}");
            return Ok(());
        }
    }

    bail!("Failed to find header: {expected_header}");
}

fn read_and_map_values(
    lines: &mut Lines<Box<dyn BufRead>>,
    sources: Vec<usize>,
) -> Result<Vec<usize>> {
    let mut destinations = sources.clone();
    while let Some(line) = lines.next() {
        let line = line.context("Reading a line of input")?;
        if line.is_empty() {
            break;
        }

        let entry = line
            .parse::<RangeMapEntry>()
            .context("Parsing the mapping entry")?;
        //eprintln!("❱ {entry:?}");

        let start = entry.source_range_start;
        let end = start + entry.range_length;

        let (&first, &last) = (sources.first().unwrap(), sources.last().unwrap());
        if end < first && start > last {
            //eprintln!("❱∅ Out of range: {entry:?} <=> [{first}, {last}]",);
            continue;
        }

        let pp1 = sources.partition_point(|&s| s <= start);
        let pp2 = sources.partition_point(|&s| s < end);
        if pp1 == pp2 {
            //eprintln!("❱∅ No matching indices: {pp1}..{pp2} ({start}/{first}  {last}/{end})",);
            continue;
        }

        for i in pp1..pp2 {
            let source = sources[i];
            if let Some(destination) = entry.get(source) {
                //eprintln!("❱❱ {source} → {destination} ({entry:?})",);
                destinations[i] = destination;
            }
        }
    }
    destinations.sort();
    Ok(destinations)
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
            fs::File::open(&filename).with_context(|| format!("Opening file {filename:?}"))?,
        ))
    };

    let mut lines = reader.lines();
    let Some(seeds) = lines.next() else {
        bail!("Expected list of seeds");
    };

    let seeds = seeds.context("Reading first line of input")?;
    let seeds = seeds
        .strip_prefix("seeds: ")
        .with_context(|| format!("Stripping prefix from line «{seeds}»"))?;
    let seeds: Result<Vec<usize>> = seeds
        .split_whitespace()
        .map(|s| {
            s.parse::<usize>()
                .with_context(|| format!("Parsing seed number: {s}"))
        })
        .collect::<Result<Vec<usize>>>()
        .context("Parsing seed numbers");
    let mut seeds = seeds.unwrap();
    seeds.sort();

    /*    let mut debug: Vec<String> = Vec::with_capacity(seeds.len());
        seeds
            .iter()
            .map(|seed| format!("Seed {seed:?}"))
            .for_each(|s| debug.push(s));
    */
    read_until_header(&mut lines, "seed-to-soil map:")?;
    let soils = read_and_map_values(&mut lines, seeds) //
        .context("Parsing seeds-to-soil map")?;

    /*    debug
            .iter_mut()
            .zip(soils.iter())
            .for_each(|(dbg, soil)| dbg.push_str(&format!(", soil {soil}")));
    */
    read_until_header(&mut lines, "soil-to-fertilizer map:")?;
    let fertilizers = read_and_map_values(&mut lines, soils) //
        .context("Parsing soil-to-fertilizer map")?;

    /*    debug
            .iter_mut()
            .zip(fertilizers.iter())
            .for_each(|(dbg, fertilizer)| dbg.push_str(&format!(", fertilizer {fertilizer}")));
    */
    read_until_header(&mut lines, "fertilizer-to-water map:")?;
    let water = read_and_map_values(&mut lines, fertilizers) //
        .context("Parsing fertilizer-to-water map")?;

    /*    debug
            .iter_mut()
            .zip(water.iter())
            .for_each(|(dbg, water)| dbg.push_str(&format!(", water {water}")));
    */
    read_until_header(&mut lines, "water-to-light map:")?;
    let light = read_and_map_values(&mut lines, water) //
        .context("Parsing water-to-light map")?;

    /*    debug
            .iter_mut()
            .zip(light.iter())
            .for_each(|(dbg, light)| dbg.push_str(&format!(", light {light}")));
    */
    read_until_header(&mut lines, "light-to-temperature map:")?;
    let temperatures = read_and_map_values(&mut lines, light) //
        .context("Parsing light-to-temperature map")?;

    /*    debug
            .iter_mut()
            .zip(temperatures.iter())
            .for_each(|(dbg, temperature)| dbg.push_str(&format!(", temperature {temperature}")));
    */
    read_until_header(&mut lines, "temperature-to-humidity map:")?;
    let humidity = read_and_map_values(&mut lines, temperatures) //
        .context("Parsing temperature-to-humidity map")?;

    /*    debug
            .iter_mut()
            .zip(humidity.iter())
            .for_each(|(dbg, humidity)| dbg.push_str(&format!(", humidity {humidity}")));
    */
    read_until_header(&mut lines, "humidity-to-location map:")?;
    let locations = read_and_map_values(&mut lines, humidity) //
        .context("Parsing humidity-to-location map")?;

    /*    debug
            .iter_mut()
            .zip(locations.iter())
            .for_each(|(dbg, location)| dbg.push_str(&format!(", location {location}.")));
    */
    /*    for dbg in debug {
            eprintln!("* {dbg}");
        }
    */
    let closest_location = locations[0];
    println!("Day 5, part 1: {closest_location}");

    Ok(())
}
