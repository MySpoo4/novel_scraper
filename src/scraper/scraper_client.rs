use crate::{client::HttpClient, errors::Result};
use regex::Regex;
use scraper::{
    html::{Html, Select},
    Selector,
};

#[derive(Debug, Default)]
pub struct ScraperClient {
    client: HttpClient,
    excluded_words: Vec<String>,
    document: Option<Html>,
    selector: Option<Selector>,
}

impl ScraperClient {
    pub fn new(proxy: Option<&str>) -> Result<Self> {
        Ok(ScraperClient {
            client: HttpClient::new(proxy)?,
            excluded_words: Vec::new(),
            document: None,
            selector: None,
        })
    }

    pub fn add_excluded_words(&mut self, words: Vec<String>) {
        self.excluded_words.extend(words);
    }

    pub async fn scrape_url(&mut self, url: &str) -> Result<()> {
        match self.client.get_html(url).await {
            Ok(mut html) => {
                self.exclude_words(&mut html);
                self.document = Some(Html::parse_document(&html));
                Ok(())
            }
            Err(e) => Err(e),
        }
    }

    pub fn get_text(&mut self, identifier: &str) -> Option<String> {
        let separator = format!("\n\n{}\n\n", char::from_u32(8200).unwrap());
        const PUNC_COUNT: usize = 3;
        self.get_elements(identifier).map(|s| {
            s.flat_map(|e| e.text())
                .map(|s| s.trim())
                .filter(|s| {
                    !s.chars().filter(|c| c.is_ascii_punctuation()).count() < PUNC_COUNT
                        && !s.is_empty()
                })
                .collect::<Vec<&str>>()
                .join(&separator)
        })
    }

    pub fn get_element_attribute(&mut self, identifier: &str, attribute: &str) -> Option<&str> {
        self.get_elements(identifier)
            .and_then(|mut s| s.next())
            .and_then(|e| e.attr(attribute))
    }

    fn get_elements<'a>(&'a mut self, identifier: &str) -> Option<Select<'a, 'a>> {
        self.document.as_ref().and_then(|html| {
            Self::change_selector(&mut self.selector, identifier).and_then(|s| Some(html.select(s)))
        })
    }

    fn exclude_words(&self, text: &mut String) {
        // Generate a regex pattern that matches any of the words with spaces allowed between characters
        let pattern = self
            .excluded_words
            .iter()
            .map(|word| {
                word.chars()
                    .map(|c| format!(r"\s*{}", c)) // Allow spaces between letters
                    .collect::<String>()
            })
            .collect::<Vec<_>>()
            .join("|"); // Combine patterns with '|'

        let regex = Regex::new(&format!(r"(?i)\b(?:{})\b", pattern)).unwrap();

        // Replace all matches with an empty string
        *text = regex.replace_all(text, "").to_string();
    }

    // free functions
    fn change_selector<'a>(selector: &'a mut Option<Selector>, s: &str) -> Option<&'a Selector> {
        Selector::parse(s).ok().and_then(|sel| {
            *selector = Some(sel);
            selector.as_ref()
        })
    }
}
