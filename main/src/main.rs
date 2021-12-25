use std::cmp::min;
use std::path::{Path, PathBuf};

use chrono::{Datelike, Local};
use problem::logged::OkOrLog;
use quicli::prelude::*;
use simplelog::{ColorChoice, Config, TerminalMode, TermLogger};
use structopt::StructOpt;

use common::Solution;
use day1::Day1;
use day2::Day2;
use day3::Day3;
use day4::Day4;

use crate::cli::{Cli, Selection};
use crate::fetch::AOC_YEAR;
use crate::log::LevelFilter;

mod cli;
mod fetch;

fn main() {
  let args: Cli = Cli::from_args();
  TermLogger::init(LevelFilter::Info,
                   Config::default(),
                   TerminalMode::Mixed,
                   ColorChoice::Auto)
    .unwrap();

  match args {
    Cli::Fetch { token } => {
      let max_day = calculate_maxday();
      fetch::fetch_inputs(token, 1..=max_day)
    }

    Cli::Run { input, puzzle } => {
      match puzzle {
        Selection::All => {
          info!("Solving ALL Advent of Code 2021 puzzles");
          let basepath = input.unwrap_or_else(|| {
            PathBuf::from("./")
          });

          if !basepath.is_dir() {
            panic!("Not a directory: {:?}", basepath.as_path());
          }

          let max_day = calculate_maxday();
          for day in 1..=max_day {
            let filename = format!("input{:02}.txt", day);
            let input = basepath.join(filename);
            solve(day, input.as_path())
          }
        },
        Selection::Day { day } => {
          let input = input.unwrap_or_else(|| {
              let s = format!("./input{:02}.txt", day);
              PathBuf::from(s)
            });

          solve(day as u32, input.as_path());
        }
      }

    }
  }
}

fn solve(day: u32, input: &Path) {
  match day {
    1 => {
      if let Some(part1) = Day1::part1(input).ok_or_log_error() {
        info!("Day {}, Part 1: {:?}", day, part1);
      }
      if let Some(part2) = Day1::part2(input).ok_or_log_error() {
        info!("Day {}, Part 2: {:?}", day, part2);
      }
    }

    2 => {
      if let Some(part1) = Day2::part1(input).ok_or_log_error() {
        info!("Day {}, Part 1: {:?}", day, part1);
      }
      if let Some(part2) = Day2::part2(input).ok_or_log_error() {
        info!("Day {}, Part 2: {:?}", day, part2);
      }
    }

    3 => {
      if let Some(part1) = Day3::part1(input).ok_or_log_error() {
        info!("Day {}, Part 1: {:?}", day, part1);
      }
      if let Some(part2) = Day3::part2(input).ok_or_log_error() {
        info!("Day {}, Part 2: {:?}", day, part2);
      }
    }

    4 => {
      if let Some(part1) = Day4::part1(input).ok_or_log_error() {
        info!("Day {}, Part 1: {:?}", day, part1);
      }
      if let Some(part2) = Day4::part2(input).ok_or_log_error() {
        info!("Day {}, Part 2: {:?}", day, part2);
      }
    }

    _ => {
      warn!("Solution to day {} puzzle not implemented!", day);
    }
  };
}

fn calculate_maxday() -> u32 {
  let d = Local::now();
  let max_day = if d.year() == AOC_YEAR && d.month() == 12 {
    min(d.day(), 25)
  } else {
    25
  };
  max_day
}
