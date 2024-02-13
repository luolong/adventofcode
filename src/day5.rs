mod common;

use std::cmp::{max, min};
use std::fmt::{Debug, Display, Formatter};
use std::io::{BufRead, Lines};
use std::ops::Range;
use std::str::FromStr;
use std::usize;

use anyhow::{bail, Context, Error, Result};
use itertools::Itertools;

const DEFAULT_FILENAME: &str = "day5.txt";

#[derive(Debug)]
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
impl Display for RangeMapEntry {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let RangeMapEntry {
            destination_range_start,
            source_range_start,
            range_length,
        } = self;

        let source_range_end = source_range_start + range_length - 1;
        let destination_range_end = destination_range_start + range_length - 1;
        let delta = (self.destination_range_start as i128) - (self.source_range_start as i128);

        write!(
            f,
            "{source_range_start}–{source_range_end} → {destination_range_start}-{destination_range_end} (∆{delta})"
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
struct Seed(usize);

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
struct SeedRange(usize, usize);

impl Display for SeedRange {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let SeedRange(start, end) = self;
        write!(f, "{start}–{}", end - 1)
    }
}

impl Display for Seed {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

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

impl Remap for Seed {
    fn remap(&self, entry: &RangeMapEntry) -> Vec<Self> {
        let RangeMapEntry {
            destination_range_start,
            source_range_start,
            range_length,
        } = *entry;

        if (source_range_start..(source_range_start + range_length)).contains(&self.0) {
            let diff = self.0.abs_diff(source_range_start);
            return vec![Seed(destination_range_start + diff)];
        }

        vec![self.clone()]
    }
}

impl Bounds for Seed {
    fn first(&self) -> usize {
        self.0
    }

    fn last(&self) -> usize {
        self.0
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

type Change<T> = (Range<usize>, Vec<T>);

trait Compact {
    fn compact(self) -> Self;
}

impl<T> Compact for Vec<Change<T>>
where
    T: Sized + Ord + Remap + Debug,
{
    fn compact(self) -> Self {
        self.iter()
            .fold(
                Vec::with_capacity(self.len()),
                |mut acc: Vec<Change<T>>, change: &Change<T>| {
                    if let Some((prange, pitems)) = acc.pop() {
                        let (nrange, nitems) = change.clone();
                        if prange.end == nrange.start {
                            let range = prange.start..nrange.end;
                            let mut items = Vec::with_capacity(pitems.len() + nitems.len());
                            items.append(&mut pitems.clone());
                            items.append(&mut nitems.clone());
                            acc.push((range, items));
                            return acc;
                        } else {
                            acc.append(&mut vec![(prange, pitems), (nrange, nitems)]);
                        }
                    } else {
                        acc.push(change.clone())
                    }
                    acc
                },
            )
            .to_vec()
    }
}

impl Compact for Vec<SeedRange> {
    fn compact(self) -> Self {
        self.iter().unique().fold(
            Vec::with_capacity(self.len()),
            |mut acc: Vec<SeedRange>, &next| {
                if let Some(prev) = acc.pop() {
                    if prev.first() < next.last() && next.first() < prev.last() {
                        acc.push(SeedRange(
                            min(prev.first(), next.first()),
                            max(prev.last(), next.last()),
                        ));
                    } else {
                        acc.append(&mut vec![prev, next]);
                    }
                } else {
                    acc.push(next)
                }
                acc
            },
        )
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

impl<T> Display for Items<T>
where
    T: Display + Remap + Ord,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "[")?;
        let mut iter = self.items.iter();
        if let Some(first) = iter.next() {
            write!(f, "{first}")?;
        }
        while let Some(next) = iter.next() {
            write!(f, " {next}")?;
        }
        write!(f, "]")
    }
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

impl FromStr for Items<Seed> {
    type Err = Error;

    fn from_str(line: &str) -> Result<Self, Self::Err> {
        let line = line
            .strip_prefix("seeds: ")
            .context("stripping prefix before seeds")?;

        let mut items: Vec<Seed> = line
            .split_ascii_whitespace()
            .map(|s| s.parse::<usize>())
            .map_ok(|s| Seed(s))
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

        items.sort_by_key(|r| r.first());
        Ok(Items { items })
    }
}

impl<T> Items<T>
where
    T: Bounds + Remap + Ord + Debug + Display,
{
    pub fn remap_using(&self, entry: &RangeMapEntry) -> Result<Option<Change<T>>> {
        let (first, last) = self.to_bounds();
        let (start, end) = entry.to_bounds();

        let sources = &self.items;
        if end < first || start > last {
            /*
            if cfg!(debug_assertions) {
                if end < first {
                    eprintln!("❱❱❱∅ Out of range: {start}–{end} < {first}");
                }
                if start > last {
                    eprintln!("❱❱❱∅ Out of range: {last} < {start}–{end}");
                }
            }
            */
            return Ok(None);
        }

        let pp1 = sources.partition_point(|&s| s.last() < start);
        let pp2 = sources.partition_point(|&s| s.first() <= end);
        if cfg!(debug_assertions) {
            eprint!("❱❱❱ 〖{pp1}..{pp2}〗: ");
            eprint!("{:?}｟", &sources[..pp1]);
            eprint!("｠{:?}", &sources[pp1..]);
            eprintln!();

            let p = format!("❱❱❱ 〖{pp1}..{pp2}〗: {:?}｟", &sources[..pp1]);
            if let Some(len) = p.chars().try_len().ok() {
                if pp1 < pp2 {
                    eprintln!("{}{}", " ".repeat(len), "─".repeat(pp1.abs_diff(pp2)))
                } else {
                    eprintln!("{}╱╲", " ".repeat(len - 1));
                }
            }
        }

        if pp1 == pp2 {
            /*
            if cfg!(debug_assertions) {
                eprintln!("❱❱❱∅ Index {pp1} does not match any elements",);
            }
            */
            return Ok(None);
        }

        let mut dest: Vec<T> = Vec::with_capacity(3 * pp2.abs_diff(pp1));
        for i in pp1..pp2 {
            let source = sources[i];
            let mut dst = source.remap(entry);
            /*
            if cfg!(debug_assertions) {
                eprintln!("❱❱❱ Mapping {source:?} to {dst:?}");
            }
            */
            dest.append(&mut dst);
        }

        /*
        if cfg!(debug_assertions) {
            if !dest.is_empty() {
                eprintln!("❱❱❱ Replacing {:?} with {:?}", &sources[pp1..pp2], &dest);
            }
        }
        */

        Ok(Some((pp1..pp2, dest)))
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

fn remap_with_entries(
    lines: &mut Lines<Box<dyn BufRead>>,
    header_line: &str,
    part1: &mut Items<Seed>,
    part2: &mut Items<SeedRange>,
) -> Result<()> {
    read_until_header(lines, header_line)
        .with_context(|| format!("Reading header {header_line}"))?;

    let mut p1_changes: Vec<(Range<usize>, Vec<Seed>)> = Vec::with_capacity(part1.items.len());
    let mut p2_changes: Vec<(Range<usize>, Vec<SeedRange>)> = Vec::with_capacity(part2.items.len());

    while let Some(line) = lines.next() {
        let line = line.context("Reading a line of input")?;
        if line.is_empty() {
            break;
        }

        let entry = line
            .parse::<RangeMapEntry>()
            .context("Parsing the mapping entry")?;

        if cfg!(debug_assertions) {
            eprintln!("❱ {entry}");
        }

        if let Some(p1) = part1.remap_using(&entry)? {
            p1_changes.push(p1);
        }
        if let Some(p2) = part2.remap_using(&entry)? {
            p2_changes.push(p2);
        }
    }

    if !p1_changes.is_empty() {
        p1_changes.sort_by_key(|(rng, _)| rng.start);
        let mut p1_changes = p1_changes.compact();
        p1_changes.reverse(); // Make sure we don't need to muck around with indices

        for (range, new_items) in p1_changes {
            let (start, end) = (range.start, range.end);
            part1.items.splice(start..end, new_items);
        }

        part1.items.sort();
    }

    if !p2_changes.is_empty() {
        p2_changes.sort_by_key(|(rng, _)| rng.start);
        let mut p2_changes = p2_changes.compact();
        p2_changes.reverse(); // Make sure we don't need to muck around with indices

        for (range, new_items) in p2_changes {
            let (start, end) = (range.start, range.end);
            part2.items.splice(start..end, new_items);
        }

        part2.items.sort();
        let compacted = part2.items.clone().compact();
        part2.items = compacted;
    }

    Ok(())
}

fn main() -> Result<()> {
    let reader = common::get_reader(DEFAULT_FILENAME)?;

    let mut lines = reader.lines();
    let Some(seeds) = lines.next() else {
        bail!("Expected list of seeds");
    };

    let first_line = seeds.context("Reading first line of input")?;
    let mut seeds: Items<Seed> = first_line.parse().context("Parsing seeds")?;
    let mut seed_ranges: Items<SeedRange> = first_line.parse().context("Parsing seed ranges")?;

    if cfg!(debug_assertions) {
        eprintln!("Seeds1:       {seeds}");
        eprintln!("Seeds2:       {seed_ranges}");
    }

    remap_with_entries(
        &mut lines,
        "seed-to-soil map:",
        &mut seeds,
        &mut seed_ranges,
    )
    .context("Mapping seeds to soil")?;

    if cfg!(debug_assertions) {
        eprintln!("Soil1:        {seeds}");
        eprintln!("Soil2:        {seed_ranges}");
    }

    remap_with_entries(
        &mut lines,
        "soil-to-fertilizer map:",
        &mut seeds,
        &mut seed_ranges,
    )
    .context("Mapping soil to fertilizer")?;

    if cfg!(debug_assertions) {
        eprintln!("Fertilizer1:  {seeds}");
        eprintln!("Fertilizer2:  {seed_ranges}");
    }

    remap_with_entries(
        &mut lines,
        "fertilizer-to-water map:",
        &mut seeds,
        &mut seed_ranges,
    )
    .context("Mapping fertilizer to water")?;

    if cfg!(debug_assertions) {
        eprintln!("Water1:       {seeds}");
        eprintln!("Water2:       {seed_ranges}");
    }

    remap_with_entries(
        &mut lines,
        "water-to-light map:",
        &mut seeds,
        &mut seed_ranges,
    )
    .context("Mapping water to light")?;

    if cfg!(debug_assertions) {
        eprintln!("Light1:       {seeds}",);
        eprintln!("Light2:       {seed_ranges}");
    }

    remap_with_entries(
        &mut lines,
        "light-to-temperature map:",
        &mut seeds,
        &mut seed_ranges,
    )
    .context("Mapping light to temperature")?;

    if cfg!(debug_assertions) {
        eprintln!("Temperature1: {seeds}");
        eprintln!("Temperature2: {seed_ranges}");
    }

    remap_with_entries(
        &mut lines,
        "temperature-to-humidity map:",
        &mut seeds,
        &mut seed_ranges,
    )
    .context("Mapping temperature to humidity")?;

    if cfg!(debug_assertions) {
        eprintln!("Humidity1:    {seeds}");
        eprintln!("Humidity2:    {seed_ranges}");
    }

    remap_with_entries(
        &mut lines,
        "humidity-to-location map:",
        &mut seeds,
        &mut seed_ranges,
    )
    .context("Mapping humidity to location")?;

    if cfg!(debug_assertions) {
        eprintln!("Location1:    {seeds}");
        eprintln!("Location2:    {seed_ranges}");
    }

    let closest_location1 = seeds.first();
    let closest_location2 = seed_ranges.first();
    println!("Day 5, part 1: {closest_location1}");
    println!("Day 5, part 2: {closest_location2}");

    Ok(())
}
