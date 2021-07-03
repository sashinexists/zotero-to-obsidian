use std::fs;
use std::fs::File;
use std::path::Path;
extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;
extern crate chrono;

mod json_parser;
mod resource_types;
use json_parser::ZoteroData;
use resource_types::{ReferenceList, References};

//https://medium.com/@nightraiser/read-and-parse-json-with-rust-day-1-of-codedaily-9feab54b29e8
//https://docs.citationstyles.org/en/stable/specification.html

pub const LIBRARY_PATH: &str = ".library/library.json";

fn main() {
    let json_file_path = Path::new(LIBRARY_PATH);

    let file = File::open(json_file_path).expect("Failed to open file");

    let zotero_data: ZoteroData = serde_json::from_reader(file).expect("failed to parse json");

    let mut references = References::new();

    references.populate(&zotero_data.items);

    //deletes existing Resource folder if it exists
    if std::fs::metadata("Resources").is_ok() && std::fs::metadata("Resources").unwrap().is_dir() {
        fs::remove_dir_all("Resources").expect("failed to removed directory");
        println!("Deleted old Resources folder and creating a new one");
    } else {
        println!("No Resources folder found - creating one now");
    }

    fs::create_dir("Resources").expect("failed to create resource directory");
    fs::create_dir("Resources/Articles").expect("failed to create Articles directory");
    fs::create_dir("Resources/Academic Papers")
        .expect("failed to create AcademicPapers directory");
    fs::create_dir("Resources/Books").expect("failed to create Books directory");
    fs::create_dir("Resources/TED Talks").expect("failed to create TEDTalks directory");
    fs::create_dir("Resources/Youtube Videos")
        .expect("failed to create YoutubeVideos directory");

    references.articles.article_list.iter().for_each(|article| {
        fs::write(
            &format!("Resources/Articles/{}.md", article.resource_details.id),
            article.to_string(),
        )
        .expect("failed to create article notes");
    });

    references.academic_papers.academic_paper_list.iter().for_each(|academic_paper| {
        fs::write(
            &format!("Resources/Academic Papers/{}.md", academic_paper.resource_details.id),
            academic_paper.to_string(),
        )
        .expect("failed to create academic paper notes");
    });

    references.books.book_list.iter().for_each(|book| {
        fs::write(
            &format!("Resources/Books/{}.md", book.resource_details.id),
            book.to_string(),
        )
        .expect("failed to create book notes");
    });

    references.ted_talks.ted_talk_list.iter().for_each(|ted_talk| {
        fs::write(
            &format!("Resources/TED Talks/{}.md", ted_talk.resource_details.id),
            ted_talk.to_string(),
        )
        .expect("failed to create TED Talk notes.md");
    });

    references.youtube_videos.youtube_video_list.iter().for_each(|youtube_video| {
        fs::write(
            &format!("Resources/Youtube Videos/{}.md", youtube_video.resource_details.id),
            youtube_video.to_string(),
        )
        .expect("failed to create Youtube video notes");
    });
}
