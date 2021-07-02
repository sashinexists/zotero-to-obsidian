use std::collections::HashMap;
use std::fmt;



// use option type to get every single possible thing you want
#[derive(Serialize, Deserialize, Debug)]
pub struct Item {
    #[serde(rename(serialize = "citationKey", deserialize = "citationKey"))]
    pub id: String,
    #[serde(rename(serialize = "itemType", deserialize = "itemType"))]
    pub item_type: String,
    #[serde(rename(serialize = "DOI", deserialize = "DOI"))]
    pub doi: Option<String>,
    pub title: Option<String>,
    #[serde(rename(serialize = "libraryCatalog", deserialize = "libraryCatalog"))]
    pub library_catalog: Option<String>,
    pub url: Option<String>,
    pub creators: Vec<Creator>,
    #[serde(rename(serialize = "ISBN", deserialize = "ISBN"))]
    pub isbn: Option<String>,
    #[serde(rename(serialize = "shortTitle", deserialize = "shortTitle"))]
    pub short_title: Option<String>,
    pub select: Option<String>,
    pub uri: Option<String>,
    #[serde(rename(serialize = "date", deserialize = "date"))]
    pub published_date: Option<String>,
    #[serde(rename(serialize = "publicationTitle", deserialize = "publicationTitle"))]
    pub journal: Option<String>,
    pub tags: Vec<Tag>,
    pub notes: Vec<Note>
}

impl fmt::Display for Item {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", generate_item_string(self))
    }
}



#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Creator {
    pub creator_type: Option<String>,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub name: Option<String>,
}

impl fmt::Display for Creator {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut output:String = "".to_string();
        if self.creator_type.is_some() {
            output.push_str(&format!("creator_type: \"{}\", ", self.creator_type.clone().unwrap()));
        }
        if self.first_name.is_some() && self.last_name.is_some() {
            output.push_str(&format!("first_name: \"{}\", ", self.first_name.clone().unwrap()));
            output.push_str(&format!("last_name: \"{}\"", self.last_name.clone().unwrap()));
        } else if self.name.is_some() {
            output.push_str(&format!("name: \"{}\"", self.name.clone().unwrap()));

        }
        write!(f, "{}", output)
    }
}
#[derive(Serialize, Deserialize, Debug, Clone)]

pub struct Tag {
    pub tag: String
}

impl fmt::Display for Tag {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.tag)
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]

pub struct Note {
    pub date_added: String,
    pub date_modified: String,
    #[serde(rename(serialize = "note", deserialize = "note"))]
    pub content: String,
    pub uri: String
}

impl fmt::Display for Note {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.content)
    }
}

fn generate_item_string(item: &Item) -> String {
    format!(
        "---\nresouce_type: {}\nID: {}\n{}{}{}{}{}{}{}{}{}{}{}{}{}\n---\n",
        item.item_type,
        item.id,
        generate_display("URL", &item.url),
        generate_display("DOI", &item.doi),
        generate_display("isbn_13", &item.isbn),
        generate_display("title", &item.title),
        generate_display("journal", &item.journal),
        generate_display("source", &item.library_catalog),
        generate_display("published_date", &item.published_date),
        generate_display("#title", &item.short_title),
        generate_display("Open in Zotero (library)", &item.select),
        generate_display("Open in Zotero (cloud)", &item.uri),
        display_tags("tags", &item.tags),
        display_creators("creators", &item.creators),
        display_notes("notes", &item.notes)
    )
}

fn generate_display(label: &str, attr: &Option<String>) -> String {
    //change this to option string
    if attr.is_some() {
        return format!("{}: {}\n", label, attr.clone().unwrap());
    }
    "".to_string()
}

fn display_tags(label: &str, tags: &Vec<Tag>) -> String {
    let mut output = "tags: [".to_string();
    if tags.len() > 0 {
        tags.iter().for_each(|tag| {
            output.push_str(&format!("\"{}\", \"",&tag.tag));
        });
    }
    if !output.eq("tags: [") {
        output = output[0..output.len()-3].to_string();
    }
    output.push_str("]\n");
    output
}

fn display_creators(label: &str, creators: &Vec<Creator>) -> String {
    let mut output = "creators: [".to_string();
    if creators.len() > 0 {
        creators.iter().for_each(|creator| {
            output.push_str(&format!("\n{{ {} }},",&creator));
        });
    }
    output.push_str("]");
    output
}

fn display_notes(label: &str, notes: &Vec<Note>) -> String {
    let mut output = "\nnotes: [".to_string();
    if notes.len() > 0 {
        notes.iter().for_each(|note| {
            output.push_str(&format!("\n\t{{ {} }},",&note.content));
        });
    }
    output.push_str("\n]");
    output
}



#[derive(Serialize, Deserialize, Debug)]


pub struct ZoteroData {
    pub collections: HashMap<String, Collection>,
    pub config: Config,
    pub items: Vec<Item>,
}


#[derive(Serialize, Deserialize, Debug)]
pub struct Collection {
    collections: Vec<String>,
    items: Vec<i32>,
    key: String,
    name: String,
    parent: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    id: String,
    label: String,
    localeDateOrder: String,
    options: HashMap<String, bool>,
    preferences: Preferences,
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
    workersMax: i32,
}
