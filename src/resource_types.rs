use super::json_parser::{Creator, Item, Note, Tag};
use std::fmt::{self, Display};
use std::fs::File;
use std::io::Read;


pub trait New<T> {
    fn new(item: &Item) -> Option<T>;
}
pub struct Resource {
    pub id: String,
    pub full_title: String,
    pub short_title: String,
    pub publish_date: String,
    pub tags: Vec<Tag>,
    pub notes: Vec<Note>,
    pub zotero_cloud_link: String,
    pub zotero_local_link: String,
    pub creators: Vec<Creator>
}

pub struct Book {
    pub resource_details: Resource,
    pub isbn_13: String,
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
                .replace("{{short_title}}", &self.resource_details.short_title)
                .replace("{{publish_date}}", &self.resource_details.publish_date)
                .replace("{{id}}", &self.resource_details.id)
                .replace("{{full_title}}", &self.resource_details.full_title)
                .replace("{{isbn_13}}", &self.isbn_13)
                .replace(
                    "{{zotero_local_link}}",
                    &self.resource_details.zotero_local_link
                )
                .replace(
                    "{{zotero_cloud_link}}",
                    &self.resource_details.zotero_cloud_link
                )
                .replace("{{authors}}", &display_authors(&self.resource_details.creators))
                .replace("{{tags}}", &display_tags(&self.resource_details.tags))
                .replace("{{notes}}", &display_notes(&self.resource_details.notes))
                
        )
    }
}

impl New<Book> for Book {
    fn new(item: &Item) -> Option<Book> {
        if item.isbn.is_some() {
            Some(Book {
                resource_details: 
                Resource {
                    id: item.id.clone(),
                    full_title: item.title.clone().expect("Failed to find book title"),
                    short_title: item.short_title.clone().expect("Failed to find book short_title"),
                    publish_date: item.published_date.clone().expect("Failed to find book's publish_date"),
                    tags: item.tags.clone(),
                    notes: item.notes.clone(),
                    zotero_cloud_link: item.uri.clone().expect("Failed to get book's zotero cloud link"),
                    zotero_local_link: item.select.clone().expect("Failed to get book's zotero local link"),
                    creators: item.creators.clone()
                },
                isbn_13: item.isbn.clone().expect("failed to get ISBN"),
            })
        }
        else {
            None
        }     
    }
}

pub struct Article {
    pub resource_details: Resource,
    pub url: String,
}

pub struct AcademicPaper {
    pub resource_details: Resource,
    pub doi: String,
    pub journal: String,
}

pub struct YoutubeVideo {
    pub resource_details: Resource,
    pub url_query_string: String,
    pub channel: String,
}

pub struct TEDTalk {
    pub resource_details: Resource,
    pub url: String,
    pub speaker: Creator,
}

pub struct Person {
    //later
}

pub struct Quote {
    //later
}

fn display_authors(authors:&Vec<Creator>) -> String {
    let mut output = "".to_string();
    authors.iter().for_each(|author| {
        if author.name.is_some() {
            output.push_str(&format!("\n- [[{}]],", author.name.clone().expect("failed to get first name for author of {}")));
        } else if author.first_name.is_some() && author.last_name.is_some() {
            output.push_str(&format!("\n- [[{} {}]],", author.first_name.clone().expect(&format!("failed to get first name for author of {}", &author.creator_type.clone().expect("failed to get creator even"))), author.last_name.clone().expect("failed to get last name")))
        } else {
            output.push_str("Failed to get creator's name!")
        }
    });
    output.pop();
    output
}

fn display_tags(tags:&Vec<Tag>) -> String {
    let mut output = "".to_string();
    tags.iter().for_each(|tag| {
        output.push_str(&format!("\n- #{},", tag.tag));
    });
    output.pop();
    output
}

fn display_notes(notes:&Vec<Note>) -> String {
    let mut output = "".to_string();
    notes.iter().for_each(|note| {
        output.push_str(&format!("{}\n---\n,", note.content));
    });
    output.pop();
    output
}