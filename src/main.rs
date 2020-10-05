use std::{fs::File, io, io::BufRead, path::Path, vec::Vec};

mod day1;
use day1::day1;

mod day2;
use day2::day2;

mod day3;
use day3::day3;

fn run(command: &str) -> Result<(), String> {
    match command {
        "day1" => {
            let input = read_lines("day1.txt")
                .map_err(|err| format!("Failed to load day 1 input: {}", err))?;
            day1(input)
        }
        "day2" => {
            let input = read_line("day2.txt")
                .map_err(|err| format!("Failed to load day 2 input: {}", err))?;
            day2(input)
        }
        "day3" => {
            let input = read_lines("day3.txt")
                .map_err(|err| format!("Failed to load day 3 input: {}", err))?;
            day3(input)
        }
        _ => Err(format!("Unrecognized command: {}", command)),
    }
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn read_line<P>(filename: P) -> io::Result<String>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    let mut line = String::new();
    io::BufReader::new(file).read_line(&mut line)?;
    Ok(line)
}

fn main() -> Result<(), String> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() > 1 {
        run(&args[1])
    } else {
        println!("No command?");
        Ok(())
    }
}
