use super::json_parser::{Creator, Item, Note, Tag};

pub trait Resource {
    id: String,
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

pub struct Resource {
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

pub struct Article {
    pub id: String,
    pub url: String,
    pub title: String,
    pub short_title: String,
    pub authors: Vec<Creator>,
    pub publish_date: String,
    pub tags: Vec<Tag>,
    pub notes: Vec<Note>,
    pub zotero_cloud_link: String,
    pub zotero_local_link: String,
}

pub struct AcademicPaper {
    
}

pub struct YoutubeVideo {
    
}

pub struct TEDTalk {
    
}

pub struct Person {

}

pub struct Quote {

}