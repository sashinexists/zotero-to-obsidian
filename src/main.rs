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

    delete_existing_resource_folders();
    create_new_resource_folders();
    


    references.articles.article_list.iter().for_each(|article| {
        fs::write(
            &format!("Articles/{}.md", article.resource_details.id),
            article.to_string(),
        )
        .expect("failed to create article notes");
    });

    references.academic_papers.academic_paper_list.iter().for_each(|academic_paper| {
        fs::write(
            &format!("Academic Papers/{}.md", academic_paper.resource_details.id),
            academic_paper.to_string(),
        )
        .expect("failed to create academic paper notes");
    });

    references.books.book_list.iter().for_each(|book| {
        fs::write(
            &format!("Books/{}.md", book.resource_details.id),
            book.to_string(),
        )
        .expect("failed to create book notes");
    });

    references.ted_talks.ted_talk_list.iter().for_each(|ted_talk| {
        fs::write(
            &format!("TED Talks/{}.md", ted_talk.resource_details.id),
            ted_talk.to_string(),
        )
        .expect("failed to create TED Talk notes.md");
    });

    references.youtube_videos.youtube_video_list.iter().for_each(|youtube_video| {
        fs::write(
            &format!("Youtube Videos/{}.md", youtube_video.resource_details.id),
            youtube_video.to_string(),
        )
        .expect("failed to create Youtube video notes");
    });
}

fn create_new_resource_folders() {
    create_new_resource_folder("Articles");
    create_new_resource_folder("Books");
    create_new_resource_folder("Academic Papers");
    create_new_resource_folder("Youtube Videos");
    create_new_resource_folder("TED Talks");
}

fn create_new_resource_folder(folder_name:&str) {
    fs::create_dir(folder_name).expect(&format!("failed to create {} directory", folder_name));

}

fn delete_existing_resource_folders() -> () {
    delete_existing_resource_folder("Articles");
    delete_existing_resource_folder("Books");
    delete_existing_resource_folder("Academic Papers");
    delete_existing_resource_folder("Youtube Videos");
    delete_existing_resource_folder("TED Talks");
}

fn delete_existing_resource_folder(folder_name:&str) -> () {
    if std::fs::metadata(folder_name).is_ok() && std::fs::metadata(folder_name).unwrap().is_dir() {
        fs::remove_dir_all(folder_name).expect(&format!("Failed to remove {} directory", folder_name));
        println!("Deleted old {} folder and creating a new one", folder_name);
    } else {
        println!("No {} folder found - creating one now", folder_name);
    }
}