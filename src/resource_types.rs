use super::json_parser::{Creator, Item, Note, Tag};
use dissolve::strip_html_tags;
use std::fmt::{self, Display};
use std::fs::File;
use std::io::Read;

pub trait ResourceList<T> {
    fn add(&mut self, resource: T) -> ();
    fn print(&self) -> String;
}

pub trait New<T> {
    fn new(item: &Item) -> Option<T>;
}

pub trait ReferenceList {
    fn populate(&mut self, items: &Vec<Item>);
    fn print(&self);
    fn new() -> References;
}

pub struct References {
    pub articles: Articles,
    pub books: Books,
    pub academic_papers: AcademicPapers,
    pub youtube_videos: YoutubeVideos,
    pub ted_talks: TEDTalks,
}

impl ReferenceList for References {
    fn populate(&mut self, items: &Vec<Item>) {
        items
            .iter()
            .for_each(|item: &Item| match &item.item_type[..] {
                "blogPost" => self
                    .articles
                    .add(Article::new(item).expect("failed to parse blogpost")),
                "webpage" => self
                    .articles
                    .add(Article::new(item).expect("failed to parse webpage")),
                "journalArticle" => self
                    .academic_papers
                    .add(AcademicPaper::new(item).expect("failed to parse academic paper")),
                "videoRecording" => {
                    if item.library_catalog.is_some()
                        && item.library_catalog.as_ref().unwrap() == "YouTube"
                    {
                        self.youtube_videos
                            .add(YoutubeVideo::new(item).expect("failed to parse youtube video"))
                    } else if item.library_catalog.is_some()
                        && item.library_catalog.as_ref().unwrap() == "www.ted.com"
                    {
                        self.ted_talks
                            .add(TEDTalk::new(item).expect("failed to parse TED talk"))
                    }
                }
                "book" => {
                    if item.isbn.is_some() {
                        self.books
                            .add(Book::new(item).expect("failed to parse book"))
                    }
                }
                _ => println!("failed to get reference from item"),
            })
    }

    fn print(&self) {
        println!(
            "\nAcademic Papers\n{}\nArticles\n{}\nBooks\n{}\nYoutube Videos\n{}\nTED Talks\n{}",
            self.academic_papers.print(),
            self.articles.print(),
            self.books.print(),
            self.youtube_videos.print(),
            self.ted_talks.print()
        );
    }

    fn new() -> References {
        References {
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
        }
    }
}
pub struct Resource {
    pub id: String,
    pub full_title: String,
    pub tags: Vec<Tag>,
    pub notes: Vec<Note>,
    pub zotero_cloud_link: String,
    pub zotero_local_link: String,
    pub creators: Vec<Creator>,
}

pub struct Book {
    pub resource_details: Resource,
    pub isbn_13: String,
    pub short_title: String,
    pub publish_date: String,
}

impl fmt::Display for Book {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut template_file =
            File::open("Templates/Book.md").expect("Failed to open book template file");
        let mut book_template = String::new();

        template_file
            .read_to_string(&mut book_template)
            .expect("failed to parse book template file");

        write!(
            f,
            r##"{}"##,
            book_template
                .replace("{{short_title}}", &self.short_title)
                .replace("{{publish_date}}", &self.publish_date)
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
                .replace(
                    "{{authors}}",
                    &display_authors(&self.resource_details.creators)
                )
                .replace("{{tags}}", &display_tags(&self.resource_details.tags))
                .replace("{{notes}}", &display_notes(&self.resource_details.notes))
        )
    }
}

impl New<Book> for Book {
    fn new(item: &Item) -> Option<Book> {
        if item.isbn.is_some() {
            Some(Book {
                resource_details: Resource {
                    id: item.id.clone(),
                    full_title: item.title.clone().expect("Failed to find book title"),
                    tags: item.tags.clone(),
                    notes: item
                        .notes
                        .clone()
                        .into_iter()
                        .map(|mut note: Note| {
                            note.content = strip_html_tags(&note.content).concat();
                            note
                        })
                        .collect(),
                    zotero_cloud_link: item
                        .uri
                        .clone()
                        .expect("Failed to get book's zotero cloud link"),
                    zotero_local_link: item
                        .select
                        .clone()
                        .expect("Failed to get book's zotero local link"),
                    creators: item.creators.clone(),
                },
                isbn_13: item.isbn.clone().expect("failed to get ISBN"),
                short_title: item
                    .short_title
                    .clone()
                    .expect("Failed to find book short_title"),
                publish_date: item
                    .published_date
                    .clone()
                    .expect("Failed to find book's publish_date"),
            })
        } else {
            None
        }
    }
}

pub struct Books {
    pub book_list: Vec<Book>,
}

impl ResourceList<Book> for Books {
    fn add(&mut self, book: Book) -> () {
        self.book_list.push(book);
    }

    fn print(&self) -> String {
        let mut output = "".to_string();
        self.book_list.iter().for_each(|book| {
            output.push_str(&book.to_string());
        });
        output
    }
}

pub struct Article {
    pub resource_details: Resource,
    pub url: String,
}

impl New<Article> for Article {
    fn new(item: &Item) -> Option<Article> {
        if item.url.is_some() && (item.item_type == "webpage" || item.item_type == "blogPost") {
            Some(Article {
                resource_details: Resource {
                    id: item.id.clone(),
                    full_title: item.title.clone().expect("Failed to find book title"),
                    tags: item.tags.clone(),
                    notes: item
                        .notes
                        .clone()
                        .into_iter()
                        .map(|mut note: Note| {
                            note.content = strip_html_tags(&note.content).concat();
                            note
                        })
                        .collect(),
                    zotero_cloud_link: item
                        .uri
                        .clone()
                        .expect("Failed to get book's zotero cloud link"),
                    zotero_local_link: item
                        .select
                        .clone()
                        .expect("Failed to get book's zotero local link"),
                    creators: item.creators.clone(),
                },
                url: item.url.clone().expect("Article url not found"),
            })
        } else {
            None
        }
    }
}

impl fmt::Display for Article {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut template_file =
            File::open("Templates/Article.md").expect("Failed to open article template file");
        let mut article_template = String::new();

        template_file
            .read_to_string(&mut article_template)
            .expect("failed to parse book template file");

        write!(
            f,
            r##"{}"##,
            article_template
                .replace("{{id}}", &self.resource_details.id)
                .replace("{{full_title}}", &self.resource_details.full_title)
                .replace(
                    "{{zotero_local_link}}",
                    &self.resource_details.zotero_local_link
                )
                .replace(
                    "{{zotero_cloud_link}}",
                    &self.resource_details.zotero_cloud_link
                )
                .replace(
                    "{{authors}}",
                    &display_authors(&self.resource_details.creators)
                )
                .replace("{{tags}}", &display_tags(&self.resource_details.tags))
                .replace("{{notes}}", &display_notes(&self.resource_details.notes))
                .replace("{{url}}", &self.url)
        )
    }
}

pub struct Articles {
    pub article_list: Vec<Article>,
}

impl ResourceList<Article> for Articles {
    fn add(&mut self, article: Article) -> () {
        self.article_list.push(article);
    }

    fn print(&self) -> String {
        let mut output = "".to_string();
        self.article_list.iter().for_each(|article| {
            output.push_str(&article.to_string());
        });
        output
    }
}

pub struct AcademicPaper {
    pub resource_details: Resource,
    pub doi: String,
    pub journal: String,
    pub publish_date: String,
}

impl New<AcademicPaper> for AcademicPaper {
    fn new(item: &Item) -> Option<AcademicPaper> {
        if item.doi.is_some() && (item.item_type == "journalArticle") {
            Some(AcademicPaper {
                resource_details: Resource {
                    id: item.id.clone(),
                    full_title: item.title.clone().expect("Failed to find book title"),
                    tags: item.tags.clone(),
                    notes: item
                        .notes
                        .clone()
                        .into_iter()
                        .map(|mut note: Note| {
                            note.content = strip_html_tags(&note.content).concat();
                            note
                        })
                        .collect(),
                    zotero_cloud_link: item
                        .uri
                        .clone()
                        .expect("Failed to get book's zotero cloud link"),
                    zotero_local_link: item
                        .select
                        .clone()
                        .expect("Failed to get book's zotero local link"),
                    creators: item.creators.clone(),
                },
                doi: item.doi.clone().expect("Failed to get paper's DOI"),
                publish_date: item
                    .published_date
                    .clone()
                    .expect("Failed to get paper's published date"),
                journal: item
                    .journal
                    .clone()
                    .expect("Failed to get journal name for paper"),
            })
        } else {
            None
        }
    }
}

impl fmt::Display for AcademicPaper {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut template_file = File::open("Templates/AcademicPaper.md")
            .expect("Failed to open academic paper template file");
        let mut academic_paper_template = String::new();

        template_file
            .read_to_string(&mut academic_paper_template)
            .expect("failed to parse academic paper template file");

        write!(
            f,
            r##"{}"##,
            academic_paper_template
                .replace("{{id}}", &self.resource_details.id)
                .replace("{{full_title}}", &self.resource_details.full_title)
                .replace(
                    "{{zotero_local_link}}",
                    &self.resource_details.zotero_local_link
                )
                .replace(
                    "{{zotero_cloud_link}}",
                    &self.resource_details.zotero_cloud_link
                )
                .replace(
                    "{{authors}}",
                    &display_authors(&self.resource_details.creators)
                )
                .replace("{{tags}}", &display_tags(&self.resource_details.tags))
                .replace("{{notes}}", &display_notes(&self.resource_details.notes))
                .replace("{{doi}}", &self.doi)
                .replace("{{publish_date}}", &self.publish_date)
                .replace("{{journal}}", &self.journal)
        )
    }
}

pub struct AcademicPapers {
    pub academic_paper_list: Vec<AcademicPaper>,
}

impl ResourceList<AcademicPaper> for AcademicPapers {
    fn add(&mut self, academic_paper: AcademicPaper) -> () {
        self.academic_paper_list.push(academic_paper);
    }

    fn print(&self) -> String {
        let mut output = "".to_string();
        self.academic_paper_list.iter().for_each(|academic_paper| {
            output.push_str(&academic_paper.to_string());
        });
        output
    }
}

pub struct YoutubeVideo {
    pub resource_details: Resource,
    pub url_query_string: String,
    pub channel: String,
}

impl New<YoutubeVideo> for YoutubeVideo {
    fn new(item: &Item) -> Option<YoutubeVideo> {
        if item.url.is_some()
            && item.library_catalog.is_some()
            && (item.library_catalog.as_ref().unwrap() == "YouTube"
                && item.item_type == "videoRecording")
        {
            Some(YoutubeVideo {
                resource_details: Resource {
                    id: item.id.clone(),
                    full_title: item.title.clone().expect("Failed to find book title"),
                    tags: item.tags.clone(),
                    notes: item
                        .notes
                        .clone()
                        .into_iter()
                        .map(|mut note: Note| {
                            note.content = strip_html_tags(&note.content).concat();
                            note
                        })
                        .collect(),
                    zotero_cloud_link: item
                        .uri
                        .clone()
                        .expect("Failed to get book's zotero cloud link"),
                    zotero_local_link: item
                        .select
                        .clone()
                        .expect("Failed to get book's zotero local link"),
                    creators: item.creators.clone(),
                },
                url_query_string: get_youtube_query_string(
                    &item.url.clone().expect("Article url not found"),
                )
                .expect("Failed to parse URL Query String"),
                channel: item.creators[0]
                    .name
                    .clone()
                    .expect("Youtube channel not found"),
            })
        } else {
            None
        }
    }
}

fn get_youtube_query_string(url: &str) -> Option<String> {
    if url.starts_with("https://www.youtube.com/watch?v=") {
        Some(url[32..].to_string())
    } else {
        None
    }
}

impl fmt::Display for YoutubeVideo {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut template_file =
            File::open("Templates/YoutubeVideo.md").expect("Failed to open article template file");
        let mut youtube_template = String::new();

        template_file
            .read_to_string(&mut youtube_template)
            .expect("failed to parse YouTube Video template file");

        write!(
            f,
            r##"{}"##,
            youtube_template
                .replace("{{id}}", &self.resource_details.id)
                .replace("{{full_title}}", &self.resource_details.full_title)
                .replace(
                    "{{zotero_local_link}}",
                    &self.resource_details.zotero_local_link
                )
                .replace(
                    "{{zotero_cloud_link}}",
                    &self.resource_details.zotero_cloud_link
                )
                .replace("{{channel}}", &self.channel)
                .replace("{{tags}}", &display_tags(&self.resource_details.tags))
                .replace("{{notes}}", &display_notes(&self.resource_details.notes))
                .replace("{{url_query_string}}", &self.url_query_string)
        )
    }
}

pub struct YoutubeVideos {
    pub youtube_video_list: Vec<YoutubeVideo>,
}

impl ResourceList<YoutubeVideo> for YoutubeVideos {
    fn add(&mut self, youtube_video: YoutubeVideo) -> () {
        self.youtube_video_list.push(youtube_video);
    }

    fn print(&self) -> String {
        let mut output = "".to_string();
        self.youtube_video_list
            .iter()
            .for_each(|youtube_video: &YoutubeVideo| {
                output.push_str(&youtube_video.to_string());
            });
        output
    }
}

pub struct TEDTalk {
    pub resource_details: Resource,
    pub url: String,
    pub speaker: String,
}

impl New<TEDTalk> for TEDTalk {
    fn new(item: &Item) -> Option<TEDTalk> {
        if item.url.is_some()
            && item.library_catalog.is_some()
            && (item.library_catalog.as_ref().unwrap() == "www.ted.com"
                && item.item_type == "videoRecording")
        {
            Some(TEDTalk {
                resource_details: Resource {
                    id: item.id.clone(),
                    full_title: item.title.clone().expect("Failed to find book title"),
                    tags: item.tags.clone(),
                    notes: item
                        .notes
                        .clone()
                        .into_iter()
                        .map(|mut note: Note| {
                            note.content = strip_html_tags(&note.content).concat();
                            note
                        })
                        .collect(),
                    zotero_cloud_link: item
                        .uri
                        .clone()
                        .expect("Failed to get TED Talks's zotero cloud link"),
                    zotero_local_link: item
                        .select
                        .clone()
                        .expect("Failed to get TED Talk's zotero local link"),
                    creators: item.creators.clone(),
                },
                url: item.url.clone().expect("TED talk url not found"),
                speaker: item.creators[0]
                    .first_name
                    .clone()
                    .expect("couldn't get TED speaker first name")
                    + " "
                    + &item.creators[0]
                        .last_name
                        .clone()
                        .expect("couldn't get TED Speaker last name"),
            })
        } else {
            None
        }
    }
}

impl fmt::Display for TEDTalk {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut template_file =
            File::open("Templates/TEDTalk.md").expect("Failed to open TED Talk template file");
        let mut ted_talk_template = String::new();

        template_file
            .read_to_string(&mut ted_talk_template)
            .expect("failed to parse TED Talk Video template file");

        write!(
            f,
            r##"{}"##,
            ted_talk_template
                .replace("{{id}}", &self.resource_details.id)
                .replace("{{full_title}}", &self.resource_details.full_title)
                .replace(
                    "{{zotero_local_link}}",
                    &self.resource_details.zotero_local_link
                )
                .replace(
                    "{{zotero_cloud_link}}",
                    &self.resource_details.zotero_cloud_link
                )
                .replace("{{speaker}}", &format!("[[{}]]", &self.speaker))
                .replace("{{tags}}", &display_tags(&self.resource_details.tags))
                .replace("{{notes}}", &display_notes(&self.resource_details.notes))
                .replace("{{url}}", &self.url)
        )
    }
}

pub struct TEDTalks {
    pub ted_talk_list: Vec<TEDTalk>,
}

impl ResourceList<TEDTalk> for TEDTalks {
    fn add(&mut self, ted_talk: TEDTalk) -> () {
        self.ted_talk_list.push(ted_talk);
    }

    fn print(&self) -> String {
        let mut output = "".to_string();
        self.ted_talk_list.iter().for_each(|ted_talk: &TEDTalk| {
            output.push_str(&ted_talk.to_string());
        });
        output
    }
}

pub struct Person {
    //later
}

pub struct Quote {
    //later
}

fn display_authors(authors: &Vec<Creator>) -> String {
    let mut output = "".to_string();
    authors.iter().for_each(|author| {
        if author.name.is_some() {
            output.push_str(&format!(
                "\n- [[{}]],",
                author
                    .name
                    .clone()
                    .expect("failed to get first name for author of {}")
            ));
        } else if author.first_name.is_some() && author.last_name.is_some() {
            output.push_str(&format!(
                "\n- [[{} {}]],",
                author.first_name.clone().expect(&format!(
                    "failed to get first name for author of {}",
                    &author
                        .creator_type
                        .clone()
                        .expect("failed to get creator even")
                )),
                author.last_name.clone().expect("failed to get last name")
            ))
        } else {
            output.push_str("Failed to get creator's name!")
        }
    });
    output.pop();
    output
}

fn display_tags(tags: &Vec<Tag>) -> String {
    let mut output = "".to_string();
    tags.iter().for_each(|tag| {
        output.push_str(&format!("\n- #{},", tag.tag));
    });
    output.pop();
    output
}

fn display_notes(notes: &Vec<Note>) -> String {
    let mut output = "".to_string();
    notes.iter().rev().for_each(|note| {
        // figure out way to order notes
        output.push_str(&format!("{}\n---\n", note.content));
    });
    output.pop();
    output
}
