use std::env;
use std::fs;
use std::process::Command;
use clap::{Parser, Subcommand};

#[derive(Parser)]
pub struct DvdCmd {
    #[command(subcommand)]
    pub subcommand: Option<DvdSubCmd>,

    // #[command(flatten)]
    // pub args: NoteArgs,
}

#[derive(Subcommand)]
pub enum DvdSubCmd {
    /// Scan a DVD for titles and durations
    Scan {
        input: String,
    },
    // /// Rip multiple titles from a DVD
    // Multi {
    //     target_dir: String,
    //     start_title: u32,
    //     end_title: u32,
    //     base_name: String,
    // },
    // /// Rip a single title from a DVD
    // Single {
    //     title: u32,
    //     output_file: String,
    // },
}

// pub fn rip_multiple() {
//     let args: Vec<String> = env::args().collect();

//     if args.len() != 5 {
//         eprintln!("Usage: {} <target_dir> <start_title> <end_title> <base_name>", args[0]);
//         std::process::exit(1);
//     }

//     let target_dir = &args[1];
//     let start_title: u32 = args[2].parse().expect("Invalid start_title");
//     let end_title: u32 = args[3].parse().expect("Invalid end_title");
//     let base_name = &args[4];

//     // Create output directory if it doesn't exist
//     fs::create_dir_all(target_dir).expect("Failed to create target directory");

//     let mut counter = 1;

//     for title in start_title..=end_title {
//         let output_file = format!(
//             "{}/{}_{:02}.mp4",
//             target_dir, base_name, counter
//         );

//         println!("Processing title {} -> {}", title, output_file);

//         let status = Command::new("HandBrakeCLI")
//             .args([
//                 "-i", "/dev/dvd",
//                 "-t", &title.to_string(),
//                 "-o", &output_file,
//                 "--preset=Fast 1080p30",
//             ])
//             .status()
//             .expect("Failed to execute HandBrakeCLI");

//         if !status.success() {
//             eprintln!("Error processing title {}", title);
//         }

//         counter += 1;
//     }

//     println!("Done!");
// }

fn scan(input: String) {
    let output = Command::new("HandBrakeCLI")
        .args(["-i", &input, "--scan", "-t", "0"])
        .output()
        .expect("Failed to execute HandBrakeCLI");

    if !output.status.success() {
        eprintln!("Error scanning DVD");
        std::process::exit(1);
    }

    let stdout = String::from_utf8_lossy(&output.stdout);
    println!("{}", stdout);
}

pub fn handle(cmd: &DvdCmd) {
    match &cmd.subcommand {
        Some(DvdSubCmd::Scan { input }) => scan(input.clone()),
        // Some(DvdSubCmd::Multi { target_dir, start_title, end_title, base_name }) => rip_multiple(),
        // Some(DvdSubCmd::Single { title, output_file }) => rip_single(),
        None => eprintln!("No subcommand provided"),
    }
}