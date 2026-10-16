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

#[derive(Debug, serde::Deserialize, serde::Serialize)]
struct PodConfig {
    #[serde(flatten)]
    pub base: BaseConfig,
    pub save_dir: PathBuf,
    pub tracked_urls: Vec<String>,
}

impl Default for PodConfig {
    fn default() -> Self {
        Self {
            base: BaseConfig {
                app: APP_NAME.to_string(),
                log_level: "info".to_string(),
            },
            save_dir: get_default_downloads_dir(),
            tracked_urls: Vec::new(),
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
        for u in &self.tracked_urls {
            if *u == url {
                debug!("URL {} is already tracked.", u);
                return;
            }
        }

        if scan(Some(url.clone()), Some(1)) == 0 {
            debug!(
                "Couldn't find valid RSS feed at {}. Not adding to tracked list",
                url
            );
            return;
        }

        self.tracked_urls.push(url);
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
        url: Option<String>,
    },

    /// List episodes from a feed URL
    List {
        /// URL of the feed to list episodes from
        url: String,
    },

    /// List tracked feeds in config file
    TrackedFeeds {},

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

/// Scan a given url or list of urls.
fn scan(url: Option<String>, tracks: Option<usize>) -> usize {
    let mut feed_urls = Vec::new();
    if let Some(url) = url {
        debug!("Scanning feed at {}", url);
        feed_urls.push(url);
    } else {
        debug!("Scanning all configured feeds...");
        warn!("No URL provided, scanning all feeds is not implemented yet.");
        return 0;
    }

    let max_tracks: usize;
    if let Some(tracks) = tracks {
        max_tracks = tracks;
    } else {
        max_tracks = 25;
    }

    let mut num_tracks_scanned: usize = 0;
    for feed_url in feed_urls {
        debug!("Fetching feed from {}", feed_url);
        let feed = read_from_url(&feed_url, max_tracks).expect("Failed to get feed");
        debug!("Parsed feed: {:?}", feed);
        info!("Fetched feed {} from {}", feed.channel_title, feed_url);
        num_tracks_scanned += feed.items.len();
    }

    num_tracks_scanned
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

fn list_tracked_feeds(cfg: &PodConfig) {
    for url in &cfg.tracked_urls {
        println!("{}", url);
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
        Some(PodSubCmd::TrackedFeeds {}) => {
            list_tracked_feeds(&cfg);
        }
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
