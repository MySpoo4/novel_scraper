use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct ExcludedWords {
    pub excluded_words: Vec<String>,
}
