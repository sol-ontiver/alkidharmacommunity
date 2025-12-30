/*
Clap-based cli for various utilities
*/

mod sol_cli;
mod format;

use crate::sol_cli::{SolCliParser, SolCliCommand};

use clap::Parser;

fn greet(name: String, count: u8) {
    for _ in 0..count {
        println!("Hello {}!", name);
    }
}

fn main() {
    let cli = SolCliParser::parse();

    match &cli.command {
        SolCliCommand::Greet { name, count } => {
            greet(name.to_string(), *count);
        }
        SolCliCommand::Format(cmd) => {
            format::handle(cmd);
        }
        SolCliCommand::Fmt(cmd) => {
            format::handle(cmd);
        }
    }
}