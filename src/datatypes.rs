use chrono::{Date, TimeZone};
use std::fs::File;

use std::collections::HashMap;
use std::fmt::{self, Display};
use std::fmt::write;
use std::io::Read;

use regex::{Captures, Regex};

use super::json_parser::{Creator, Item, Note, Tag};

trait IndexableStruct {
    fn get_field(&self, field: &str) -> Option<String>;
}

pub struct Book {
    pub id: String,
    pub isbn: String,
    pub title: String,
    pub short_title: String,
    pub authors: Vec<Creator>,
    pub publish_date: String,
    pub tags: Vec<Tag>,
    pub notes: Vec<Note>,
    pub zotero_cloud_link: String,
    pub zotero_local_link: String,
}

impl fmt::Display for Book {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut template_file =
            File::open("Resource/Book.md").expect("Failed to open book template file");
        let mut book_template = String::new();

        template_file
            .read_to_string(&mut book_template)
            .expect("failed to parse book template file");

        write!(
            f,
            r##"{}"##,
            book_template
                .replace("{{title}}", &self.short_title)
                .replace("{{publish_date}}", &self.publish_date)
                .replace("{{citekey}}", &self.id)
                .replace("{{full_title}}", &self.title)
                .replace("{{zotero_local_link}}", &self.zotero_local_link)
                .replace("{{zotero_cloud_link}}", &self.zotero_cloud_link)
        )
    }
}

pub fn createBookFromItem(item: &Item) -> Book {
    Book {
        id: item.id.clone(),
        isbn: item.isbn.clone().unwrap(),
        title: item.title.clone().unwrap(),
        short_title: item.short_title.clone().unwrap(),
        authors: item.creators.clone(),
        publish_date: item.published_date.clone().unwrap(),
        tags: item.tags.clone(),
        notes: item.notes.clone(),
        zotero_cloud_link: item.uri.clone().unwrap(),
        zotero_local_link: item.select.clone().unwrap(),
    }
}

pub struct TEDTalk {}

pub struct Article {}

pub struct AcademicPaper {}

#[derive(Serialize, Deserialize, Debug)]

pub struct YoutubeVideo {
    #[serde(rename(serialize = "URL", deserialize = "URL"))]
    pub url: String,
    #[serde(rename(serialize = "abstract", deserialize = "abstract"))]
    pub item_abstract: String,
    //pub accessed: String,
    pub title: String,
    pub id: String,
    pub language: String,
    pub source: String,
    #[serde(rename(serialize = "type", deserialize = "type"))]
    pub item_type: String,
}
/*
impl fmt::Display for YoutubeVideo {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "---\nYOUTUBE VIDEO\ntitle: {}\nauthor: {}\nurl: {}\nid: {}\n---\n",
            self.title, self.author[0], self.url, self.id
        )
    }
}*/
