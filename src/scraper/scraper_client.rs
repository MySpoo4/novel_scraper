use crate::models::{Selector as NSelector, SelectorType};
use crate::{client::HttpClient, errors::Result};
use futures::executor::block_on;
use regex::Regex;
use scraper::{
    html::{Html, Select},
    Selector,
};

#[derive(Debug)]
pub struct ScraperClient {
    client: HttpClient,
    excluded_words: Vec<String>,
    document: Option<Html>,
    selector: Option<Selector>,
}

impl ScraperClient {
    pub async fn new(proxy: Option<&str>) -> Result<Self> {
        Ok(ScraperClient {
            client: HttpClient::new(proxy).await?,
            excluded_words: Vec::new(),
            document: None,
            selector: None,
        })
    }

    pub async fn close(self) -> Result<()> {
        self.client.close().await
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

    pub fn get_text(&mut self, selector: &NSelector) -> Option<String> {
        const PUNC_COUNT: usize = 3;
        let separator = format!("\n\n{}\n\n", char::from_u32(8200).unwrap());
        let filter = |s: &str| -> bool {
            !s.is_empty() && {
                let punctuation_count = s.chars().filter(|c| c.is_ascii_punctuation()).count();
                punctuation_count == s.len() && punctuation_count > PUNC_COUNT
                    || punctuation_count != s.len()
            }
        };

        match selector.selector_type {
            SelectorType::XPATH => block_on(self.client.get_text_xpath(
                &selector.val,
                &separator,
                filter,
            )),
            SelectorType::CSS => self.get_elements(&selector.val).map(|s| {
                s.flat_map(|e| e.text())
                    .map(|s| s.trim())
                    .filter(|s| filter(s))
                    .collect::<Vec<&str>>()
                    .join(&separator)
            }),
        }
    }

    pub fn get_element_attribute(
        &mut self,
        selector: &NSelector,
        attribute: &str,
    ) -> Option<String> {
        match selector.selector_type {
            SelectorType::XPATH => block_on(self.client.get_attr_xpath(&selector.val, attribute)),
            SelectorType::CSS => self
                .get_elements(&selector.val)
                .and_then(|mut s| s.next())
                .and_then(|e| e.attr(attribute))
                .map(|s| s.to_string()),
        }
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
