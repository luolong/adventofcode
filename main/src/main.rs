use std::path::PathBuf;

use quicli::prelude::*;
use simplelog::{ColorChoice, Config, TerminalMode, TermLogger};
use simplelog::paris::Logger;
use structopt::StructOpt;

use crate::log::LevelFilter;

#[derive(Debug, StructOpt)]
#[structopt(name = "advent", about = "Advent of Code 2021 solutions")]
enum Cli {
  #[structopt(name = "fetch", about = "Fetch personal input for your AoC puzzle")]
  Fetch {
    #[structopt(long = "token", short = "t")]
    token: Option<String>,
  },

  #[structopt(name = "run", about = "Run the solution for your AoC puzzle")]
  Run {
    #[structopt(long = "input", short = "i", help = "Path to the AoC puzzle input", parse(from_os_str))]
    input: Option<PathBuf>,

    #[structopt(subcommand)]
    puzzle: Selection,
  },
}

#[derive(Debug, StructOpt)]
enum Selection {
  #[structopt(name = "day", about = "Run solution for a specific day")]
  Day {
    #[structopt(name = "day", help = "Which day's solution to run")]
    day: u8
  },

  #[structopt(name = "all", about = "Run all solutions")]
  All,
}

fn main() {
  let args: Cli = Cli::from_args();
  TermLogger::init(LevelFilter::Info, Config::default(), TerminalMode::Mixed, ColorChoice::Auto).unwrap();

  match args {
    Cli::Fetch { token } => {
      if let Some(t) = token {
        info!("Fetching Advent of Code 2021 inputs using token {}", t)
      } else {
        info!("Fetching Advent of Code 2021 inputs...")
      }
    }
    Cli::Run { input, puzzle } => {
      let suffix = input.map_or("...".to_string(), |path| {
        format!("using {:?}", path.as_path())
      });

      match puzzle {
        Selection::All => {
          info!("Solving ALL Advent of Code 2021 puzzles {}", suffix)
        },
        Selection::Day { day } => {
          info!("Solving puzzle {} of Advent of Code 2021 {}", day, suffix)
        }
      }

    }
  }
}
