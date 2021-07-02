use std::fs::File;
use std::path::Path;
extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;
extern crate chrono;

mod json_parser;
mod resource_types;
use json_parser::{Item, ZoteroData};
use resource_types::{AcademicPaper, AcademicPapers, Article, Articles, Book, Books, ReferenceList, References, TEDTalk, TEDTalks, YoutubeVideo, YoutubeVideos};

use crate::resource_types::{New, ResourceList};

//use serde_json::Value as JsonValue;

//https://medium.com/@nightraiser/read-and-parse-json-with-rust-day-1-of-codedaily-9feab54b29e8
//https://docs.citationstyles.org/en/stable/specification.html

pub const LIBRARY_PATH: &str = ".library/library.json";

fn main() {
    let json_file_path = Path::new(LIBRARY_PATH);

    let file = File::open(json_file_path).expect("Failed to open file");

    let zotero_data: ZoteroData = serde_json::from_reader(file).expect("failed to parse json");

    let mut references = References {
        articles: Articles {
            article_list: Vec::<Article>::new(),
        },
        books: Books {
            book_list: Vec::<Book>::new(),
        },
        academic_papers: AcademicPapers {
            academic_paper_list: Vec::<AcademicPaper>::new(),
        },
        youtube_videos: YoutubeVideos {
            youtube_video_list: Vec::<YoutubeVideo>::new(),
        },
        ted_talks: TEDTalks {
            ted_talk_list: Vec::<TEDTalk>::new(),
        },
    };

    references.populate(&zotero_data.items);
    
    references.print();

    /*
    let mut academic_papers: AcademicPapers = AcademicPapers {
        academic_paper_list: Vec::<AcademicPaper>::new(),
    };



    let citations: Vec<Item> = zotero_data.items;
    citations.iter().for_each(|citation| {
        if citation.item_type == "journalArticle"
            && citation.doi.is_some()
        {
            let academic_paper: AcademicPaper = AcademicPaper::new(citation)
                .expect(&format!("failed to parse Academic Paper: {}", &citation.id));
            academic_papers.add(academic_paper);
        }
    });
    println!("{}", academic_papers.print());*/
}

// now implement display
