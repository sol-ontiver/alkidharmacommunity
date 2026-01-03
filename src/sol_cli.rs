/*
Clap-based cli for various utilities
*/

mod clip;
mod format;
mod note;
pub mod sol_config;

use atty::Stream;
use clap::{Parser, Subcommand};
use crate::{format::FormatCmd, note::NoteCmd};
use log::{info, debug};
use env_logger;

#[derive(Parser)]
#[command(name = "sol-cli")]
#[command(about = "Example CLI")]
#[command(version)]
pub struct SolCliParser {
    /// Increase verbosity
    #[arg(short, long, action = clap::ArgAction::SetTrue)]
    verbose: bool,

    /// Increase verbosity
    #[arg(short, long, action = clap::ArgAction::SetTrue)]
    debug: bool,

    // /// Optional config file
    // #[arg(long)]
    // config: Option<std::path::PathBuf>,

    #[command(subcommand)]
    pub command: SolCliCommand,
}

#[derive(Subcommand)]
pub enum SolCliCommand {
    /// Copy text to clipboard
    Clip {
        /// Text to copy to clipboard
        text: String,
    },
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
    /// Text formatting utilities (alias)
    #[command(subcommand)]
    Fmt(FormatCmd),
    /// Notes utilities (subcommand defaults to write)
    Note(NoteCmd),
}

fn greet(name: String, count: u8) {
    for _ in 0..count {
        println!("Hello {}!", name);
    }
}

fn main() {
    let cli = SolCliParser::parse();

    // Base CLI options
    if atty::is(Stream::Stdout) {
        if cli.debug {
            env_logger::Builder::new()
                .target(env_logger::Target::Stderr)
                .filter_level(log::LevelFilter::Trace)
                .init();
            debug!("Debug mode is on");
        }
        else if cli.verbose {
            env_logger::Builder::new()
                .target(env_logger::Target::Stderr)
                .filter_level(log::LevelFilter::Debug)
                .init();
            info!("Verbose mode is on");
        }
        else {
            env_logger::Builder::new()
                .target(env_logger::Target::Stderr)
                .filter_level(log::LevelFilter::Off)
                .parse_default_env() // allow RUST_LOG to override
                .init();
        }
    }
    else {
        // If not running in a terminal, disable logging so it 
        // doesn't interfere with output, pipe, etc.
        env_logger::Builder::new()
            .target(env_logger::Target::Stderr)
            .filter_level(log::LevelFilter::Off)
            .init();
    }

    // Handle commands and subcommands
    match &cli.command {
        SolCliCommand::Clip { text } => {
            let _ = clip::clip(&text);
        }
        SolCliCommand::Greet { name, count } => {
            greet(name.to_string(), *count);
        }
        SolCliCommand::Format(cmd) => {
            format::handle(cmd);
        }
        SolCliCommand::Fmt(cmd) => {
            format::handle(cmd);
        }
        SolCliCommand::Note(cmd) => {
            note::handle(cmd);
        }
    }
}