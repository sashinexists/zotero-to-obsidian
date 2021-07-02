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

    references.print();

    fs::create_dir("Resource").expect("failed to create resource directory");
    fs::create_dir("Resource/Articles").expect("failed to create Resource/Articles directory");
    fs::create_dir("Resource/AcademicPapers")
        .expect("failed to create Resource/AcademicPapers directory");
    fs::create_dir("Resource/Books").expect("failed to create Resource/Books directory");
    fs::create_dir("Resource/TEDTalks").expect("failed to create Resource/TEDTalks directory");
    fs::create_dir("Resource/YoutubeVideos")
        .expect("failed to create resource/YoutubeVideos directory");

    references.articles.article_list.iter().for_each(|article| {
        fs::write(
            &format!("Resource/Articles/{}", article.resource_details.id),
            article.to_string(),
        )
        .expect("failed to create article notes");
    });

    references.academic_papers.academic_paper_list.iter().for_each(|academic_paper| {
        fs::write(
            &format!("Resource/AcademicPapers/{}", academic_paper.resource_details.id),
            academic_paper.to_string(),
        )
        .expect("failed to create academic paper notes");
    });

    references.books.book_list.iter().for_each(|book| {
        fs::write(
            &format!("Resource/Books/{}", book.resource_details.id),
            book.to_string(),
        )
        .expect("failed to create book notes");
    });

    references.ted_talks.ted_talk_list.iter().for_each(|ted_talk| {
        fs::write(
            &format!("Resource/TEDTalks/{}", ted_talk.resource_details.id),
            ted_talk.to_string(),
        )
        .expect("failed to create TED Talk notes");
    });

    references.youtube_videos.youtube_video_list.iter().for_each(|youtube_video| {
        fs::write(
            &format!("Resource/YoutubeVideos/{}", youtube_video.resource_details.id),
            youtube_video.to_string(),
        )
        .expect("failed to create Youtube video notes");
    });
}
