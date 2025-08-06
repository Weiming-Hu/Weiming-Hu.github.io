// This file could contain parsed BibTeX data as Rust structs
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Publication {
    pub title: String,
    pub authors: Vec<String>,
    pub journal: Option<String>,
    pub volume: Option<String>,
    pub pages: Option<String>,
    pub year: u32,
    pub website: Option<String>,
    pub pdf: Option<String>,
    pub publisher: Option<String>,
}

// You could add a build script to parse the .bib file and generate this data
pub fn get_publications() -> Vec<Publication> {
    // This would be generated from your .bib file
    vec![
        // Publications would be parsed from the .bib file
    ]
}
