use std::fs;
use std::process::Command;
use log::{debug, info, warn};

use clap::{Parser, Subcommand};

use crate::{progress::ProgressBar};

const DEFAULT_INPUT: &str = "/dev/sr0";

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
        #[arg(short, long, default_value = DEFAULT_INPUT)]
        input: String,
    },
    /// Rip multiple titles from a DVD
    Multi {
        target_dir: String,
        series_name: String,
        season: u32,
        start_title: u32,
        end_title: u32,

        #[arg(short, long, default_value = None)]
        start_episode: Option<u32>,
        #[arg(short, long, default_value = DEFAULT_INPUT)]
        input: String,
    },
    /// Rip a single title from a DVD
    Single {
        #[arg(short, long, default_value = DEFAULT_INPUT)]
        input: String,
        #[arg(short, long, default_value_t = 1)]
        title: u32,
        target_dir: String,
        output_file: String,
    },
}

fn last_episode_num(target_dir: &str, series_name: &str, season: u32) -> u32 {
    let season_dir = format!("{}/{}/season_{:02}", target_dir, series_name, season);
    if let Ok(entries) = fs::read_dir(&season_dir) {
        entries.filter_map(|entry| {
            entry.ok().and_then(|e| {
                e.file_name().to_str().and_then(|name| {
                    name.split('_').nth(1)
                        .and_then(|part| part.split('.').next())
                        .and_then(|num_str| num_str[1..].parse::<u32>().ok())
                })
            })
        }).max().unwrap_or(0)
    } else {
        0
    }
}

fn rip_title_range(
    input: &str, 
    target_dir: &str, 
    series_name: &str,
    season: u32,
    start_episode: Option<u32>,
    start_title: u32, 
    end_title: u32, 
) {
    if end_title < start_title {
        warn!("End title must be greater than or equal to start title");
        return;
    }
    let num_titles = end_title - start_title + 1;

    // Create output directory if it doesn't exist
    let target_dir = target_dir.trim_end_matches('/');
    let output_dir= format!("{}/{}/season_{:02}", target_dir, series_name, season);
    fs::create_dir_all(&output_dir).expect("Failed to create target directory");

    let mut episode_num = start_episode.unwrap_or_else(|| last_episode_num(target_dir, series_name, season) + 1);

    info!("Series: {}, Season: {}, Procesing {} titls starting with episode {}", 
        series_name, season, num_titles, episode_num);
    info!("Saving to {}", output_dir);

    let mut progress = ProgressBar::new(&0, &num_titles);
    for title in start_title..=end_title {
        let output_file = format!(
            "{}/{}_S{:02}E{:02}.mp4",
            output_dir, series_name, season, episode_num
        );

        debug!("Processing title {} -> {}", title, output_file);

        let status = Command::new("HandBrakeCLI")
            .args([
                "-i", &input,
                "-t", &title.to_string(),
                "-o", &output_file,
                "--preset=Fast 1080p30",
                "--ipod-atom",
            ])
            .status()
            .expect("Failed to execute HandBrakeCLI");

        if !status.success() {
            warn!("Error processing title {}", title);
        }

        episode_num += 1;
        progress.update(&(title - start_title + 1));
    }

    info!("Done ripping {} titles to {}", num_titles, output_dir);
}

fn rip_single(
    input: &str,
    title_number: u32,
    target_dir: &str,
    output_name: &str,
) {
    let mut filename = output_name.trim().trim_end_matches("/").to_string();
    if filename.is_empty() {
        warn!("Output file name cannot be empty");
        return;
    }
    else if !filename.ends_with(".mkv") && !filename.ends_with(".mp4") {
        filename = format!("{}.mkv", filename);
    }
    fs::create_dir_all(&target_dir).expect("Failed to create target directory");

    let target_file = format!("{}/{}", target_dir.trim_end_matches('/'), filename);
    let status = Command::new("HandBrakeCLI")
        .args([
            "-i", &input,
            "-t", &title_number.to_string(),
            "-o", &target_file,
            "--preset=Fast 1080p30",
            "--ipod-atom",
        ])
        .status()
        .expect("Failed to execute HandBrakeCLI");

    if !status.success() {
        warn!("Error processing title {}", title_number);
    }

    info!("Done ripping title {} to {}", title_number, target_file);
}

fn scan(input: &str) {
    let output = Command::new("lsdvd")
        .arg(&input)
        .output()
        .expect("Failed to execute lsdvd");

    let stdout = String::from_utf8_lossy(&output.stdout);
    println!("{}", stdout);
}

pub fn handle(cmd: &DvdCmd) {
    match &cmd.subcommand {
        Some(DvdSubCmd::Scan { input }) => scan(input.as_str()),
        Some(DvdSubCmd::Multi { input, target_dir, start_title, end_title, series_name, season, start_episode }) => 
            rip_title_range(input, target_dir, series_name, *season, *start_episode, *start_title, *end_title),
        Some(DvdSubCmd::Single { input, title, target_dir, output_file  }) => rip_single(input, *title, target_dir, output_file),
        None => warn!("No subcommand provided"),
    }
}