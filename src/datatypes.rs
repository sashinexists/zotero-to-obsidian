use std::fmt;
use std::fmt::write;
use std::option;
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug)]
pub struct ZoteroData {
    pub collections: HashMap<String, Collection>,
    pub config: Config,
    pub items: Vec<Item>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    id: String,
    label: String,
    localeDateOrder: String,
    options: HashMap<String, bool>,
    preferences: Preferences

}

#[derive(Serialize, Deserialize, Debug)]
pub struct Preferences {
        DOIandURL: String,
        ascii: String,
        asciiBibLaTeX: bool,
        asciiBibTeX: bool,
        autoAbbrev: bool,
        autoAbbrevStyle: String,
        autoExport: String,
        autoExportDelay: i32,
        autoExportIdleWait: i32,
        autoExportPathReplaceDiacritics: bool,
        autoExportPathReplaceDirSep: String,
        autoExportPathReplaceSpace: String,
        autoPinDelay: i32,
        automaticTags: bool,
        auxImport: bool,
        baseAttachmentPath: String,
        biblatexExtendedDateFormat: bool,
        biblatexExtendedNameFormat: bool,
        biblatexExtractEprint: bool,
        bibtexParticleNoOp: bool,
        bibtexURL: String,
        cacheFlushInterval: i32,
        citeCommand: String,
        citekeyFold: bool,
        citekeyFormat: String,
        citekeySearch: bool,
        citeprocNoteCitekey: bool,
        csquotes: String,
        debugLogDir: String,
        exportBibTeXStrings: String,
        exportBraceProtection: bool,
        exportTitleCase: bool,
        extraMergeCSL: bool,
        extraMergeCitekeys: bool,
        extraMergeTeX: bool,
        git: String,
        ignorePostscriptErrors: bool,
        import: bool,
        importBibTeXStrings: bool,
        importCaseProtection: String,
        importCitationKey: bool,
        importExtra: bool,
        importJabRefAbbreviations: bool,
        importJabRefStrings: bool,
        importSentenceCase: String,
        importUnknownTexCommand: String,
        itemObserverDelay: i32,
        jabrefFormat: i32,
        jieba: bool,
        keyConflictPolicy: String,
        keyScope: String,
        kuroshiro: bool,
        mapMath: String,
        mapText: String,
        mapUnicode: String,
        newTranslatorsAskRestart: bool,
        parseParticles: bool,
        patchDates: String,
        platform: String,
        postscript: String,
        postscriptOverride: String,
        qualityReport: bool,
        quickCopyEta: String,
        quickCopyMode: String,
        quickCopyOrgMode: String,
        quickCopyPandocBrackets: bool,
        quickCopySelectLink: String,
        rawImports: bool,
        rawLaTag: String,
        relativeFilePaths: bool,
        retainCache: bool,
        scrubDatabase: bool,
        skipFields: String,
        skipWords: String,
        startupProgress: String,
        strings: String,
        testing: bool,
        verbatimFields: String,
        warnBulkModify: i32,
        warnTitleCased: bool,
        workersCache: bool,
        workersMax: i32
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Collection {
    collections: Vec<String>,
    items: Vec<i32>,
    key: String,
    name: String,
    parent: Option<String>
}

#[derive(Serialize, Deserialize, Debug)]

pub struct Author {
    pub family: Option<String>,
    pub given: Option<String>,
    pub literal: Option<String>
}

/*impl fmt::Display for Author {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {}", self.given, self.family)
    }
}*/

// use option type to get every single possible thing you want
#[derive(Serialize, Deserialize, Debug)]
pub struct Item {
    #[serde(rename(serialize = "citationKey", deserialize = "citationKey"))]
    pub id: String,
    #[serde(rename(serialize = "itemType", deserialize = "itemType"))]
    pub item_type: String,
    pub doi: Option<String>,
    pub title: Option<String>,
    pub source: Option<String>,
    #[serde(rename(serialize = "URL", deserialize = "URL"))]
    pub url: Option<String>,
    pub author: Option<Vec<Author>>,
    #[serde(rename(serialize = "ISBN", deserialize = "ISBN"))]
    pub isbn: Option<String>,
    #[serde(rename(serialize = "shortTitle", deserialize = "shortTitle"))]
    pub short_title: Option<String>,
    pub select: Option<String>,
    pub uri: Option<String>,
    #[serde(rename(serialize = "date", deserialize = "date"))]
    pub published_date: Option<String>
}

impl fmt::Display for Item {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            generate_item_string(self)
        )
    }
}

fn generate_item_string(item:&Item) -> String { 
    format!(
        "---\nresouce_type: {}\nID: {}\n{}{}{}{}{}{}{}{}\n---\n",
        item.item_type, 
        item.id, 
        generate_display("URL", &item.url),
        generate_display("DOI", &item.doi),
        generate_display("isbn_13", &item.isbn),
        generate_display("title", &item.title),
        generate_display("published_date", &item.published_date),
        generate_display("#title", &item.short_title),
        generate_display("Open in Zotero (library)", &item.select),
        generate_display("Open in Zotero (cloud)", &item.uri)
    )
}

fn generate_display(label:&str, attr:&Option<String>) -> String { //change this to option string
    if attr.is_some() {
        return format!("{}: {}\n",label, attr.clone().unwrap())
    }
    "".to_string()
}


#[derive(Serialize, Deserialize, Debug)]

pub struct YoutubeVideo {
    #[serde(rename(serialize = "URL", deserialize = "URL"))]
    pub url: String,
    #[serde(rename(serialize = "abstract", deserialize = "abstract"))]
    pub item_abstract: String,
    //pub accessed: String,
    pub title: String,
    pub author: Vec<Author>,
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

/*{"URL":"https://www.ted.com/talks/sir_ken_robinson_do_schools_kill_creativity",
"abstract":"Sir Ken Robinson makes an entertaining and profoundly moving case for creating an education system that nurtures (rather than undermines) creativity.",
"accessed":{"date-parts":[["2021",6,11]]},"author":[{"family":"Robinson","given":"Sir Ken"}],"id":"robinsonSchoolsKillCreativity","language":"en","source":"www.ted.com","title":"Do schools kill creativity?","type":"motion_picture"}*/
