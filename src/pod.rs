///Podcatching utility
use std::error::Error;
use std::fs;
use std::io;
use std::path::PathBuf;
use std::process::exit;

use clap::{Parser, Subcommand};
use log::{debug, error, info, warn};
use serde;

use crate::rss::{RSSFeed, parse_from_string};
use crate::sol_config::{BaseConfig, Config, get_default_downloads_dir, load_config};

const APP_NAME: &str = "pod";
const DEFAULT_SCAN_MAX_TRACKS: usize = 25;

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct PodcastInfo {
    pub name: String,
    pub description: String,
    pub url: String,
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
struct PodConfig {
    #[serde(flatten)]
    pub base: BaseConfig,
    pub save_dir: PathBuf,
    pub tracked_podcasts: Vec<PodcastInfo>,
}

impl Default for PodConfig {
    fn default() -> Self {
        Self {
            base: BaseConfig {
                app: APP_NAME.to_string(),
                log_level: "info".to_string(),
            },
            save_dir: get_default_downloads_dir(),
            tracked_podcasts: Vec::new(),
        }
    }
}

impl Config for PodConfig {
    fn app_name(&self) -> &str {
        APP_NAME
    }
}

impl PodConfig {
    fn add_tracked_url(&mut self, url: String) {
        for p in &self.tracked_podcasts {
            if *p.url == url {
                debug!("URL {} is already tracked.", p.url);
                return;
            }
        }

        let feed = scan(url.clone(), Some(1));
        if feed.items.len() < 1 {
            error!("Feed is invalid or empty.");
            return;
        }
        let podcast: PodcastInfo = PodcastInfo {
            name: feed.channel_title,
            description: feed.channel_description,
            url: url,
        };

        self.tracked_podcasts.push(podcast);
        match self.save() {
            Err(..) => {
                error!("Failed to save updated config!");
                exit(-1);
            }
            Ok(..) => {}
        }
    }
}

#[derive(Parser)]
pub struct PodCmd {
    #[arg(short, long, default_value_t = 25)]
    pub titles: usize,

    #[command(subcommand)]
    pub subcommand: Option<PodSubCmd>,
}

#[derive(Subcommand)]
pub enum PodSubCmd {
    /// Add a feed to tracked feeds
    AddFeed {
        /// RSS feed URL
        url: String,
    },

    /// Scan for new podcast episodes and info
    Scan {
        /// Optional URL to scan. If not provided, will scan all configured feeds.
        #[arg(short, long)]
        url: String,
    },

    /// List episodes from a feed URL
    List {
        /// URL of the feed to list episodes from
        url: String,
    },

    /// List tracked feeds in config file
    TrackedFeeds {
        /// Optional - print the description of the podcast
        #[arg(short, long, action = clap::ArgAction::SetTrue)]
        print_descriptions: Option<bool>,
    },

    /// Pull the latest episode
    PullLatest {
        /// URL of the feed to pull from
        url: String,
        /// The directory to save to
        target_dir: String,
    },
}

fn read_from_url(url: &str, max_tracks: usize) -> Result<RSSFeed, Box<dyn Error>> {
    debug!("Fetching feed from {}", url);
    let response = reqwest::blocking::get(url)?.text()?;
    debug!("Parsing feed at {}", url);
    let feed = parse_from_string(&response, max_tracks)?;
    debug!("Parsed feed: {:?}", feed);
    Ok(feed)
}

/// Add feed to list of RSS feeds to track
fn add_feed(cfg: &mut PodConfig, url: &String) {
    cfg.add_tracked_url(url.to_string());
}

/// Scan a given url for RSS Feed info
fn scan(url: String, tracks: Option<usize>) -> RSSFeed {
    let max_tracks: usize;
    if let Some(tracks) = tracks {
        max_tracks = tracks;
    } else {
        max_tracks = DEFAULT_SCAN_MAX_TRACKS;
    }

    debug!("Fetching feed from {}", url);
    let fetched_feed = read_from_url(&url, max_tracks).expect("Failed to get feed");
    debug!("Parsed feed: {:?}", fetched_feed);
    info!("Fetched feed {} from {}", fetched_feed.channel_title, url);

    fetched_feed
}

fn list(url: String) {
    let feed = read_from_url(&url, 25).expect(&format!("Failed to get feed from url [{}]", url));
    info!("Feed: {}", feed.channel_title);

    for item in feed.items {
        info!(
            "{}:{} ({}) - [{}]",
            item.guid, item.title, item.pubDate, item.link
        );
    }
}

fn list_tracked_feeds(cfg: &PodConfig, show_descriptions: bool) {
    for pod in &cfg.tracked_podcasts {
        println!("* {} (URL: {})", pod.name, pod.url);
        if show_descriptions {
            println!("\n{}", pod.description,);
        }
    }
}

fn pull_latest(url: String, target_dir: String) {
    let feed = read_from_url(&url, 1).expect(&format!("Failed to get feed from url [{}]", url));
    fs::create_dir_all(&target_dir).expect("Failed to create target directory");
    let item = &feed.items[0];
    let media_url = if item.enclosure.is_empty() {
        &item.link
    } else {
        &item.enclosure
    };
    let mut response = reqwest::blocking::get(media_url).expect("Rss Feed GET failed");
    let output_file = format!("{}/{}.mp3", target_dir, &item.title);
    let mut file_out = fs::File::create(output_file).expect("File creation failed");
    io::copy(&mut response, &mut file_out).expect("failed to copy content");
}

pub fn handle(cmd: &PodCmd) {
    let mut cfg: PodConfig = load_config(APP_NAME).expect("Failed to load app config!");
    match cmd.subcommand {
        Some(PodSubCmd::AddFeed { ref url }) => {
            add_feed(&mut cfg, &url);
        }
        Some(PodSubCmd::Scan { ref url }) => {
            let _ = scan(url.clone(), None);
        }
        Some(PodSubCmd::List { ref url }) => {
            list(url.clone());
        }
        Some(PodSubCmd::TrackedFeeds {
            ref print_descriptions,
        }) => match print_descriptions {
            Some(desc) => {
                list_tracked_feeds(&cfg, *desc);
            }
            None => {
                list_tracked_feeds(&cfg, false);
            }
        },
        Some(PodSubCmd::PullLatest {
            ref url,
            ref target_dir,
        }) => {
            pull_latest(url.clone(), target_dir.clone());
        }
        None => {
            warn!("No subcommand provided");
        }
    }
}
