use std::path::Path;
use std::fs::File;
use std::io::Read; // this is to use the function .read_to_string()
extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;

mod datatypes;
use datatypes::{Item, ZoteroData};

//use serde_json::Value as JsonValue;

//https://medium.com/@nightraiser/read-and-parse-json-with-rust-day-1-of-codedaily-9feab54b29e8
//https://docs.citationstyles.org/en/stable/specification.html

pub const LIBRARY_PATH:&str = ".library/library.json";

fn main() {

    let json_file_path = Path::new(LIBRARY_PATH);

    let file = File::open(json_file_path).expect("Failed to open file");

    let zotero_data: ZoteroData = serde_json::from_reader(file).expect("failed to parse json");

    let citations:Vec<Item> = zotero_data.items;

    citations.iter().for_each( |citation| {
        println!("{}", citation);
    });

    
}

// now implement display