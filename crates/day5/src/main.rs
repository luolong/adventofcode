use std::fmt::{Debug, Formatter};
use std::io::{BufRead, BufReader, Lines};
use std::ops::Range;
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

trait Remap: Copy + Debug {
    /// Remaps this item based on the provided  range map entry
    ///
    /// Should return a copy of the item
    fn remap(&self, entry: &RangeMapEntry) -> Vec<Self>;
}

trait Bounds {
    fn first(&self) -> usize;
    fn last(&self) -> usize;

    fn to_bounds(&self) -> (usize, usize) {
        (self.first(), self.last())
    }

    fn is_before_first(&self, i: usize) -> bool {
        i < self.first()
    }

    fn is_after_last(&self, i: usize) -> bool {
        self.last() < i
    }
}

impl Bounds for RangeMapEntry {
    fn first(&self) -> usize {
        self.source_range_start
    }

    fn last(&self) -> usize {
        self.source_range_start + self.range_length
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
struct SeedRange(usize, usize);

impl From<&[usize]> for SeedRange {
    fn from(value: &[usize]) -> Self {
        let start = value[0];
        let end = start + value[1];
        SeedRange::from(start..end)
    }
}
impl From<Range<usize>> for SeedRange {
    fn from(value: Range<usize>) -> Self {
        let Range { start, end } = value;
        SeedRange(start, end)
    }
}

impl From<SeedRange> for Range<usize> {
    fn from(value: SeedRange) -> Self {
        let SeedRange(start, end) = value;
        start..end
    }
}

impl Remap for usize {
    fn remap(&self, entry: &RangeMapEntry) -> Vec<Self> {
        let RangeMapEntry {
            destination_range_start,
            source_range_start,
            range_length,
        } = *entry;

        if (source_range_start..(source_range_start + range_length)).contains(self) {
            let diff = self.abs_diff(source_range_start);
            return vec![destination_range_start + diff];
        }

        vec![*self]
    }
}

impl Bounds for usize {
    fn first(&self) -> usize {
        *self
    }

    fn last(&self) -> usize {
        *self
    }
}

impl Bounds for SeedRange {
    fn first(&self) -> usize {
        self.0
    }

    fn last(&self) -> usize {
        self.1 - 1
    }
}

impl Remap for SeedRange {
    fn remap(&self, entry: &RangeMapEntry) -> Vec<Self> {
        let RangeMapEntry {
            destination_range_start,
            source_range_start,
            range_length,
        } = *entry;

        let range = Range::from(*self);
        let source_range_end = source_range_start + range_length;

        if range.contains(&source_range_start) || range.contains(&source_range_end) {
            if range.start < source_range_start {
                let tail_len = source_range_start.abs_diff(range.end);
                vec![
                    SeedRange::from(range.start..source_range_start),
                    SeedRange::from(destination_range_start..(destination_range_start + tail_len)),
                ]
            } else if range.end > source_range_end {
                let offset = source_range_start.abs_diff(range.start);
                let head_start = destination_range_start + offset;
                let head_end = destination_range_start + range_length;
                vec![
                    SeedRange::from(head_start..head_end),
                    SeedRange::from(source_range_end..range.end),
                ]
            } else {
                let start_diff = source_range_start.abs_diff(range.start);
                let end_diff = source_range_start.abs_diff(range.end);
                let destination_start = destination_range_start + start_diff;
                let destination_end = destination_range_start + end_diff;
                vec![SeedRange::from(destination_start..destination_end)]
            }
        } else if range.start < source_range_start && range.end > source_range_end {
            vec![
                SeedRange::from(range.start..source_range_start),
                SeedRange::from(destination_range_start..destination_range_start + range_length),
                SeedRange::from(source_range_end..range.end),
            ]
        } else {
            vec![*self]
        }
    }
}

struct Items<T>
where
    T: Sized + Ord + Remap + Debug,
{
    items: Vec<T>,
}

impl<T> Bounds for Items<T>
where
    T: Bounds + Remap + Ord + Debug,
{
    fn first(&self) -> usize {
        self.items[0].first()
    }

    fn last(&self) -> usize {
        let last = self.items.len() - 1;
        self.items[last].last()
    }
}

impl FromStr for Items<usize> {
    type Err = Error;

    fn from_str(line: &str) -> Result<Self, Self::Err> {
        let line = line
            .strip_prefix("seeds: ")
            .context("stripping prefix before seeds")?;

        let mut items: Vec<usize> = line
            .split_ascii_whitespace()
            .map(|s| s.parse::<usize>())
            .try_collect()
            .context("Parsing numbers")?;

        items.sort();
        Ok(Items { items })
    }
}

impl FromStr for Items<SeedRange> {
    type Err = Error;

    fn from_str(line: &str) -> std::result::Result<Self, Self::Err> {
        let line = line
            .strip_prefix("seeds: ")
            .context("Stripping prefix before seed ranges");

        let numbers: Vec<usize> = line
            .unwrap()
            .split_ascii_whitespace()
            .map(|s| s.parse::<usize>())
            .try_collect()
            .context("Parsing numbers")?;

        let mut items: Vec<SeedRange> = numbers.chunks_exact(2).map_into().collect_vec();

        items.sort();
        Ok(Items { items })
    }
}

impl<T> Items<T>
where
    T: Bounds + Remap + Ord + Debug,
{
    pub fn remap_using(&mut self, entry: &RangeMapEntry) -> Result<()> {
        let (first, last) = self.to_bounds();
        let (start, end) = entry.to_bounds();

        if end < first || start > last {
            if cfg!(debug_assertions) {
                if end < first {
                    eprintln!("∅ Out of range: {start}–{end} < {first}");
                }
                if start > last {
                    eprintln!("∅ Out of range: {last} < {start}–{end}");
                }
            }
            return Ok(());
        }

        let sources = Vec::from_iter(self.items.iter().cloned());
        let pp1 = sources.partition_point(|&s| s.first() <= start);
        let pp2 = sources.partition_point(|&s| s.last() < end);
        if pp1 == pp2 {
            if cfg!(debug_assertions) {
                eprintln!("∅ No matching indices: {pp1}..{pp2} ({start}/{first}  {last}/{end})",);
            }
            return Ok(());
        }

        for i in pp1..pp2 {
            let source = sources[i];
            let destinations = source.remap(entry);
            if cfg!(debug_assertions) {
                eprintln!("❱❱ Mapping {source:?} to {destinations:?}");
            }
            (&mut self.items).splice(i..=i, destinations);
        }

        self.items.sort();
        Ok(())
    }
}

fn read_until_header(lines: &mut Lines<Box<dyn BufRead>>, expected_header: &str) -> Result<()> {
    while let Some(line) = lines.next() {
        let Ok(line) = line else {
            bail!("Failed to read a line");
        };

        if line == expected_header {
            if cfg!(debug_assertions) {
                eprintln!("\n{line}");
            }
            return Ok(());
        }
    }

    bail!("Failed to find header: {expected_header}");
}

fn while_entries(
    lines: &mut Lines<Box<dyn BufRead>>,
    header_line: &str,
    mut f: impl FnMut(&RangeMapEntry) -> Result<()>,
) -> Result<()> {
    read_until_header(lines, header_line)
        .with_context(|| format!("Reading header {header_line}"))?;

    while let Some(line) = lines.next() {
        let line = line.context("Reading a line of input")?;
        if line.is_empty() {
            break;
        }

        let entry = line
            .parse::<RangeMapEntry>()
            .context("Parsing the mapping entry")?;
        if cfg!(debug_assertions) {
            eprintln!("❱ {entry:?}");
        }

        f(&entry)?;
    }

    Ok(())
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

    let first_line = seeds.context("Reading first line of input")?;
    let mut seeds: Items<usize> = first_line.parse().context("Parsing seeds")?;
    let mut _seed_ranges: Items<SeedRange> = first_line.parse().context("Parsing seed ranges")?;

    if cfg!(debug_assertions) {
        eprintln!("Seeds: {:?}", seeds.items);
    }

    while_entries(&mut lines, "seed-to-soil map:", |entry| {
        seeds.remap_using(&entry)?;
        Ok(())
    })
    .context("Mapping seeds to soil")?;

    if cfg!(debug_assertions) {
        eprintln!("Soil: {:?}", seeds.items);
    }

    while_entries(&mut lines, "soil-to-fertilizer map:", |entry| {
        seeds.remap_using(&entry)?;
        Ok(())
    })
    .context("Mapping soil to fertilizer")?;

    if cfg!(debug_assertions) {
        eprintln!("Fertilizer: {:?}", seeds.items);
    }

    while_entries(&mut lines, "fertilizer-to-water map:", |entry| {
        seeds.remap_using(&entry)?;
        Ok(())
    })
    .context("Mapping fertilizer to water")?;

    if cfg!(debug_assertions) {
        eprintln!("Water: {:?}", seeds.items);
    }

    while_entries(&mut lines, "water-to-light map:", |entry| {
        seeds.remap_using(&entry)?;
        Ok(())
    })
    .context("Mapping water to light")?;

    if cfg!(debug_assertions) {
        eprintln!("Light: {:?}", seeds.items);
    }

    while_entries(&mut lines, "light-to-temperature map:", |entry| {
        seeds.remap_using(&entry)?;
        Ok(())
    })
    .context("Mapping light to temperature")?;

    if cfg!(debug_assertions) {
        eprintln!("Temperature: {:?}", seeds.items);
    }

    while_entries(&mut lines, "temperature-to-humidity map:", |entry| {
        seeds.remap_using(&entry)?;
        Ok(())
    })
    .context("Mapping temperature to humidity")?;

    if cfg!(debug_assertions) {
        eprintln!("Humidity: {:?}", seeds.items);
    }

    while_entries(&mut lines, "humidity-to-location map:", |entry| {
        seeds.remap_using(&entry)?;
        Ok(())
    })
    .context("Mapping humidity to location")?;

    if cfg!(debug_assertions) {
        eprintln!("Location: {:?}", seeds.items);
    }

    let closest_location = seeds.first();
    println!("Day 5, part 1: {closest_location}");

    Ok(())
}
