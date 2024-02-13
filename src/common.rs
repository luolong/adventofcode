use anyhow::{Context, Result};
use atty::Stream;
use std::{
    env, fs,
    io::{self, BufRead, BufReader},
};

pub fn get_reader(default_filename: &str) -> Result<Box<dyn BufRead>> {
    let filename = env::args()
        .nth(1)
        .unwrap_or_else(|| default_filename.to_string());

    if filename == "-" && atty::is(Stream::Stdin) {
        Ok(Box::new(BufReader::new(io::stdin())))
    } else {
        let file = fs::File::open(&filename).with_context(|| format!("Opening {filename}"))?;
        Ok(Box::new(BufReader::new(file)))
    }
}
