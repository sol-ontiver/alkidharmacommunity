/*
Text formatting utility
*/

use clap::{Subcommand};

#[derive(Subcommand)]
pub enum FormatCmd {
    /// Wrap text to a specified line length
    Wrap {
        /// Input text to wrap
        text: String,

        /// Max line length
        #[arg(short, long, default_value_t = 100)]
        count: u8,

        /// Newline character
        #[arg(short, long, default_value_t = String::from("\n"))]
        newline: String,

        /// Ignore word boundaries
        #[arg(short, long, action = clap::ArgAction::SetTrue)]
        ignore_words: bool,
    },
}

/// Wrap text by words. Lines will be split when the line length exceeds `count`.
fn wrap_words(text: &str, count: usize, newline: &str) -> String {
    let mut this_line = String::new();
    let mut lines = Vec::new();
    for word in text.split_whitespace() {
        if this_line.len() < count {
            this_line.push_str(word);
            this_line.push(' ');
        } else {
            lines.push(this_line.trim_end().to_string());
            this_line = String::new();
            this_line.push_str(word);
            this_line.push(' ');
        }
    }
    if !this_line.is_empty() {
        lines.push(this_line.trim_end().to_string());
    }
    lines.join(newline)
}

pub fn handle(cmd: &FormatCmd) {
    match cmd {
        FormatCmd::Wrap { text, count, newline, ignore_words } => {
            let chars = text
                .chars()
                .collect::<Vec<char>>();

            let wrapped = if *ignore_words {
                chars
                    .chunks(*count as usize)
                    .map(|chunk| chunk.iter().collect::<String>())
                    .collect::<Vec<String>>()
                    .join(&newline)
            } else {
                wrap_words(&text, *count as usize, &newline)
            };
            println!("{}", wrapped);
        }
    }
}