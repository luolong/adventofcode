use std::path::PathBuf;

use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "advent", about = "Advent of Code 2021 solutions")]
pub enum Cli {
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
pub enum Selection {
  #[structopt(name = "day", about = "Run solution for a specific day")]
  Day {
    #[structopt(name = "day", help = "Which day's solution to run")]
    day: u8
  },

  #[structopt(name = "all", about = "Run all solutions")]
  All,
}
