use clap::{command, Command};

fn main() {
    let matches = command!()
        .subcommand(Command::new("test").about("Just trying out some things"))
        .get_matches();

    if let Some(_test) = matches.subcommand_matches("test") {
        println!("Testing it so!");
    }
}
