use std::fs::File;
use std::path::Path;
extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;
extern crate chrono;

mod resource_types;
mod json_parser;
use resource_types::{Book, Books, Article, Articles, YoutubeVideo, YoutubeVideos};
use json_parser::{Item, ZoteroData};

use crate::resource_types::{New, ResourceList};

//use serde_json::Value as JsonValue;

//https://medium.com/@nightraiser/read-and-parse-json-with-rust-day-1-of-codedaily-9feab54b29e8
//https://docs.citationstyles.org/en/stable/specification.html

pub const LIBRARY_PATH: &str = ".library/library.json";

fn main() {
    let json_file_path = Path::new(LIBRARY_PATH);

    let file = File::open(json_file_path).expect("Failed to open file");

    let zotero_data: ZoteroData = serde_json::from_reader(file).expect("failed to parse json");

    let mut youtubeVideos:YoutubeVideos = YoutubeVideos{youtube_video_list:Vec::<YoutubeVideo>::new()};

    let citations: Vec<Item> = zotero_data.items;
    citations.iter().for_each(|citation| {
        if citation.item_type == "videoRecording" && citation.library_catalog.is_some() && citation.library_catalog.as_ref().unwrap() == "YouTube"  {
            let youtube_video:YoutubeVideo = YoutubeVideo::new(citation).expect(&format!("failed to parse article: {}",&citation.id));
            youtubeVideos.add(youtube_video);
        }
    });
    println!("{}", youtubeVideos.print());
}

// now implement display
