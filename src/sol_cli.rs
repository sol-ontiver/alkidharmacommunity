use clap::{Parser, Subcommand};
use crate::{format::FormatCmd};

#[derive(Parser)]
#[command(name = "sol-cli")]
#[command(about = "Example CLI")]
#[command(version)]
pub struct SolCliParser {
    #[command(subcommand)]
    pub command: SolCliCommand,
}

#[derive(Subcommand)]
pub enum SolCliCommand {
    /// Greet a person
    Greet {
        /// Name of the person to greet
        #[arg(short, long, default_value_t = String::from("World"))]
        name: String,

        /// Number of times to greet
        #[arg(short, long, default_value_t = 1)]
        count: u8,
    },
    /// Text formatting utilities
    #[command(subcommand)]
    Format(FormatCmd),
    // Text formatting utilities (alias)
    #[command(subcommand)]
    Fmt(FormatCmd),
}
