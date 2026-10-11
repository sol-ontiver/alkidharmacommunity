/// RSS parsing using quick_xml

use quick_xml::events::{ Event };
use quick_xml::reader::Reader;
use quick_xml::XmlVersion;

use log::{debug, warn};

pub enum RssError {
    UnsupportedVersion,
    NotRssFeed,
    ParseError(String),
}

impl std::fmt::Display for RssError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RssError::UnsupportedVersion => write!(f, "Unsupported RSS version"),
            RssError::NotRssFeed => write!(f, "Not a valid RSS feed"),
            RssError::ParseError(msg) => write!(f, "Parse error: {}", msg),
        }
    }
}

impl std::error::Error for RssError {}

impl std::fmt::Debug for RssError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "RssError: {}", self)
    }
}

#[allow(non_snake_case)]
pub struct TrackData {
    pub title: String, // Title of the item
    pub description: String,
    pub link: String, // Link to the item
    pub author: String, // Email address of the author of the item
    pub category: String, // Includes the item in one or more categories
    pub comments: String, // URL of a page for comments relating to the item
    pub enclosure: String, // Describes a media object that is attached to the item
    pub guid: String, // A string that uniquely identifies the item
    pub pubDate: String, // Indicates when the item was published
    pub source: String, // The RSS channel that the item came from
}

impl std::fmt::Debug for TrackData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "TrackData {{ title: {}, description: {}, link: {} }}", 
            self.title, self.description, self.link)
    }
}

impl std::fmt::Display for TrackData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "TrackData: {} - {}", self.title, self.description)
    }
}

pub struct RSSFeed {
    pub channel_title: String,
    pub channel_description: String,
    pub channel_link: String,
    pub items: Vec<TrackData>,
}

impl std::fmt::Debug for RSSFeed {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "RSSFeed Channel Info: {{ title: {}, description: {}, link: {} }}", self.channel_title, self.channel_description, self.channel_link)
    }
}

impl std::fmt::Display for RSSFeed {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "RSSFeed Channel Info: {{ title: {}, description: {}, link: {} }}", self.channel_title, self.channel_description, self.channel_link)
    }
}

fn parse_channel_item(item_xml: &str) -> Result<TrackData, RssError> {
    let mut reader = Reader::from_str(item_xml);
    reader.config_mut().trim_text(true);
    let mut track = TrackData {
        title: String::new(),
        description: String::new(),
        link: String::new(),
        author: String::new(),
        category: String::new(),
        comments: String::new(),
        enclosure: String::new(),
        guid: String::new(),
        pubDate: String::new(),
        source: String::new(),
    };

    loop {
        match reader.read_event() {
            Ok(Event::Start(ref e)) | Ok(Event::Empty(ref e)) => {
                match e.name().as_ref() {
                    b"title" => {
                        let title_bytes = reader.read_text(e.name()).expect("Failed to read title text");
                        track.title = reader.decoder().decode(&title_bytes).expect("Failed to decode title text").to_string();
                    }
                    b"description" => {
                        let description_bytes = reader.read_text(e.name()).expect("Failed to read description text");
                        track.description = reader.decoder().decode(&description_bytes).expect("Failed to decode description text").to_string();
                    }
                    b"link" => {
                        match reader.read_event(){
                            Ok(Event::CData(ref data_event)) => {
                                track.link = reader.decoder().decode(data_event).expect("Failed to decode CDATA").to_string();
                            }
                            _ => {
                                warn!("Item link is not a CDATA element.");
                                let link_bytes = reader.read_text(e.name()).expect("Failed to read link text");
                                track.link = reader.decoder().decode(&link_bytes).expect("Failed to decode link text").to_string();
                            }
                        }
                    }
                    b"author" => {
                        let author_bytes = reader.read_text(e.name()).expect("Failed to read author text");
                        track.author = reader.decoder().decode(&author_bytes).expect("Failed to decode author text").to_string();
                    }
                    b"category" => {
                        let category_bytes = reader.read_text(e.name()).expect("Failed to read category text");
                        track.category = reader.decoder().decode(&category_bytes).expect("Failed to decode category text").to_string();
                    }
                    b"comments" => {
                        let comments_bytes = reader.read_text(e.name()).expect("Failed to read comments text");
                        track.comments = reader.decoder().decode(&comments_bytes).expect("Failed to decode comments text").to_string();
                    }
                    b"enclosure" => {
                        let mut url = String::new();
                        for attr in e.attributes().flatten() {
                            if attr.key.as_ref() == b"url" {
                                url = reader.decoder().decode(&attr.value).expect("Failed to decode enclosure url").to_string();
                            }
                        }
                        track.enclosure = url;
                    }
                    b"guid" => {
                        let guid_bytes = reader.read_text(e.name()).expect("Failed to read guid text");
                        track.guid = reader.decoder().decode(&guid_bytes).expect("Failed to decode guid text").to_string();
                    }
                    b"pubDate" => {
                        let pub_date_bytes = reader.read_text(e.name()).expect("Failed to read pubDate text");
                        track.pubDate = reader.decoder().decode(&pub_date_bytes).expect("Failed to decode pubDate text").to_string();
                    }
                    b"source" => {
                        let source_bytes = reader.read_text(e.name()).expect("Failed to read source text");
                        track.source = reader.decoder().decode(&source_bytes).expect("Failed to decode source text").to_string();
                    }
                    _ => (),
                }
            }
            Ok(Event::Eof) => break,
            Err(e) => return Err(RssError::ParseError(format!("XML parse error: {}", e))),
            _ => (),
        }
    }
    Ok(track)
}

fn parse_channel_from_string(channels_xml: &str, feed: &mut RSSFeed, max_tracks: usize) -> Result<(), RssError> {
    let mut reader = Reader::from_str(channels_xml);
    reader.config_mut().trim_text(true);

    loop {
        match reader.read_event() {
            Ok(Event::Start(ref e)) => {
                match e.name().as_ref() {
                    b"title" => {
                        let title_bytes = reader.read_text(e.name()).expect("Failed to read title text");
                        feed.channel_title = reader.decoder().decode(&title_bytes).expect("Failed to decode title text").to_string();
                    }
                    b"description" => {
                        let description_bytes = reader.read_text(e.name()).expect("Failed to read description text");
                        feed.channel_description = reader.decoder().decode(&description_bytes).expect("Failed to decode description text").to_string();
                    }
                    b"link" => {
                        let link_bytes = reader.read_text(e.name()).expect("Failed to read link text");
                        feed.channel_link = reader.decoder().decode(&link_bytes).expect("Failed to decode link text").to_string();
                    }
                    b"item" => {
                        let item_xml = reader.read_text(e.name()).expect("Failed to read item text");
                        let item_decoded = reader.decoder().decode(&item_xml).expect("Failed to decode item XML").to_string();
                        let track = parse_channel_item(&item_decoded)?;
                        feed.items.push(track);
                        if feed.items.len() >= max_tracks {
                            debug!("Reached max track limit of {}, stopping parse", max_tracks);
                            break;
                        }
                    }
                    _ => (),
                }
            }
            Ok(Event::Eof) => break,
            Err(e) => return Err(RssError::ParseError(format!("XML parse error: {}", e))),
            _ => (),
        }
    }

    Ok(())
}

pub fn parse_from_string(xml_string: &str, max_tracks: usize) -> Result<RSSFeed, RssError> {
    let mut reader = Reader::from_str(&xml_string);
    let mut xml_version: Option<XmlVersion> = None;
    let mut feed = RSSFeed {
        channel_title: String::new(),
        channel_description: String::new(),
        channel_link: String::new(),
        items: Vec::new(),
    };

    reader.config_mut().trim_text(true);
    loop {
        match reader.read_event().unwrap() {
            Event::Decl(ref e) => {
                xml_version = Some(e.xml_version().unwrap());
            }
            Event::Start(ref e) => {
                if xml_version.is_none() {
                    warn!("XML declaration not found, assuming xml 1.0");
                    xml_version = Some(XmlVersion::Explicit1_0);
                }
                match e.name().as_ref() {
                    b"rss" => {
                        // Parse RSS feed attributes
                        match e.attributes().find(|a| 
                            a.as_ref()
                            .unwrap()
                            .key.as_ref() == b"version") {
                            Some(Ok(attr)) 
                            if attr
                                .normalized_value(xml_version.unwrap())
                                .as_deref()
                                .unwrap() == "2.0" => {
                                debug!("Parsing RSS 2.0 feed");
                            }
                            _ => {
                                warn!("Unsupported RSS version, expected 2.0");
                                return Err(RssError::UnsupportedVersion);
                            }
                        }
                    }
                    b"channel" => {
                        let text = reader.read_text(e.name()).expect("Failed to read channel text");
                        let channel_xml = reader.decoder().decode(&text).expect("Failed to decode channel XML");
                        parse_channel_from_string(&channel_xml, &mut feed, max_tracks)?;
                    }
                    _ => {
                        break;
                    }
                }
            }
            Event::PI(e) => {
                debug!("Processing instruction: {:?}", e);
            }
            Event::End(ref e) => {
                debug!("End element: {:?}", e);
            }
            Event::Eof => break,
            e => {
                warn!("Unexpected XML event: {:?}", e);
            }
        }
    }

    Ok(feed)
}