/*
Text formatting utility
*/

use clap::{Subcommand};

#[derive(Subcommand)]
pub enum FormatCmd {
    Wrap {
        /// Input text
        #[arg(short, long)]
        text: String,

        // Max line length
        // #[arg(short, long, default_value_t = 1)]
        // count: u8,
    },
}


pub fn handle(cmd: &FormatCmd) {
    match cmd {
        FormatCmd::Wrap { text } => {
            println!("Wrapping text: {}", text);
        }
    }
}