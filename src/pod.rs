///Podcatching utility

use std::error::Error;

use clap::{Parser, Subcommand};
use log::{debug, info, warn};

use crate::rss::{parse_from_string, RSSFeed};

#[derive(Parser)]
pub struct PodCmd {
    #[arg(short, long, default_value_t = 25)]
    pub titles: usize,

    #[command(subcommand)]
    pub subcommand: Option<PodSubCmd>,
}

#[derive(Subcommand)]
pub enum PodSubCmd {
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
    }
}

fn read_from_url(url: &str, max_tracks: usize) -> Result<RSSFeed, Box<dyn Error>> {
    debug!("Fetching feed from {}", url);
    let response = reqwest::blocking::get(url)?.text()?;
    debug!("Parsing feed at {}", url);
    let feed = parse_from_string(&response, max_tracks)?;
    debug!("Parsed feed: {:?}", feed);
    Ok(feed)
}

/// Scan a given url or list of urls.
fn scan(url: Option<String>) {
    let mut feed_urls = Vec::new();
    if let Some(url) = url {
        debug!("Scanning feed at {}", url);
        feed_urls.push(url);
    } else {
        debug!("Scanning all configured feeds...");
        warn!("No URL provided, scanning all feeds is not implemented yet.");
        return;
    }

    for feed_url in feed_urls {
        debug!("Fetching feed from {}", feed_url);
        let _feed = read_from_url(&feed_url, 25).expect("Failed to get feed");
        debug!("Parsed feed: {:?}", _feed);

        info!("Fetched feed {} from {}", _feed.channel_title, feed_url);
    }
}

fn list(url: String) {
    let feed = read_from_url(&url, 25).expect(&format!("Failed to get feed from url [{}]", url));
    info!("Feed: {}", feed.channel_title);

    for item in feed.items {
        info!("{}:{} ({}) - [{}]", item.guid, item.title, item.pubDate, item.link);
    }
}

pub fn handle(cmd: &PodCmd) {
    match cmd.subcommand {
        Some(PodSubCmd::Scan { ref url }) => {
            scan(url.clone());
        }
        Some(PodSubCmd::List { ref url }) => {
            list(url.clone());
        }
        None => {
            warn!("No subcommand provided");
        }
    }
}
