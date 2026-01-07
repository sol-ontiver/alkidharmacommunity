/// Notes utility. Make and view notes.

use clap::{Args, Parser, Subcommand};
use log::{debug, info, warn};
use crate::{sol_config::get_app_dir};
use chrono::Local;
use std::fs::OpenOptions;
use std::io::Write;
use std::fs;
// use std::io::{Write, stdout};

/// Notes are stored in this directory within the app directory
const NOTES_DIR: &str = "notes";

/***********************************************
    * CLI Structures
***********************************************/
#[derive(Args, Debug)]
pub struct NoteArgs {
    /// Directly append text to a note
    #[arg(short, long, default_value_t = String::new())]
    append: String,

    /// Name of the note (optional)
    #[arg(short, long)]
    name: Option<String>,
}

#[derive(Parser)]
pub struct NoteCmd {
    #[command(subcommand)]
    pub subcommand: Option<NoteSubCmd>,

    #[command(flatten)]
    pub args: NoteArgs,
}

#[derive(Subcommand)]
pub enum NoteSubCmd {
    /// View a note
    View(NoteArgs),

    Merge {
        /// Name of the note to merge from
        note_to_merge: String,
        /// Optional name for note to merge into
        into: Option<String>, 
        /// Preserve the original note after merging (deletes by default)
        #[arg(long, action = clap::ArgAction::SetTrue)]
        preserve_original: bool,
    }
}

/***********************************************
    * Note Structures
***********************************************/
#[derive(Debug)]
pub struct NoteConfig {
    pub editor: String,
}
impl std::fmt::Display for NoteConfig {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "NoteConfig {{ ")?;
        write!(f, "Editor: {}", self.editor)?;
        write!(f, " }}")
    }
}

/***********************************************
    * Private Functions
***********************************************/
fn get_notes_dir() -> std::path::PathBuf {
    let app_dir = get_app_dir();
    let notes_path = app_dir.join(NOTES_DIR);
    if !notes_path.exists() {
        std::fs::create_dir_all(&notes_path).expect("Could not create notes directory");
    }
    notes_path
}

fn get_note_path(name: &Option<String>, notes_dir: &std::path::PathBuf) -> String {
    let note_name: String;
    match name {
        Some(name) => {
            note_name = name.to_owned();
        }
        None => {
            let now = Local::now();
            note_name = now.format("%Y_%b").to_string();
        }
    }

    format!("{}/{}.md", notes_dir.to_str().unwrap(), note_name)
}

fn merge_notes(from: &str, into: &str, preserve: &bool) -> std::io::Result<()> {
    use std::fs;

    let from_content = fs::read_to_string(from)?;
    let mut into_file = fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(into)?;

    writeln!(into_file, "\n{}", from_content)?;
    if !preserve {
        fs::remove_file(from)?;
    }

    Ok(())
}

fn write_line(note_path: &str, content: &str) {
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(note_path)
        .expect("Could not open or create note file");

    writeln!(file, "{}", content).expect("Could not write to note file");
}


/***********************************************
    * Public Functions
***********************************************/
pub fn handle(cmd: &NoteCmd) {
    let notes_path = get_notes_dir();
    info!("Notes directory: {:?}", notes_path);

    let note_cfg = NoteConfig {
        editor: String::from("nano"),
    };
    info!("Note configuration: {}", note_cfg);

    match &cmd.subcommand {
        Some(NoteSubCmd::View(args)) => {
            warn!("View note command is not yet implemented. Args: {:?}", args);
        }
        Some(NoteSubCmd::Merge { note_to_merge, into, preserve_original }) => {
            debug!("Note to merge: {}", note_to_merge);
            let note_path = get_note_path(into, &notes_path);
            debug!("Merging into note: {}", note_path);
            let _ = merge_notes(&note_to_merge, &note_path, &preserve_original);
            info!("Merged note {} into {}", note_to_merge, note_path);
        }
        None => {
            debug!("Writing note. Args: {:?}", cmd.args);
            debug!("Note: {:?}", get_note_path(&cmd.args.name, &notes_path));
            if !cmd.args.append.is_empty() {
                let note_path = get_note_path(&cmd.args.name, &notes_path);
                write_line(&note_path, &cmd.args.append);
                info!("Appended to note: {}", note_path);
            } else {
                warn!("No content to append. Use --append to add text to the note directly.");
                // println!("{}", get_note_path(&cmd.args, &notes_path) );
                // stdout().flush().unwrap();
            }
        }
    }
}