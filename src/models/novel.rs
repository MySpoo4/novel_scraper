use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Novel {
    pub meta_data: MetaData,
    pub site: Site,
}

#[derive(Debug, Deserialize)]
pub struct Site {
    pub url: String,
    pub identifiers: Selectors,
}

#[derive(Debug, Deserialize)]
pub struct Selectors {
    pub next_btn: Selector,
    pub title: Selector,
    pub body: Selector,
}

#[derive(Debug, Deserialize)]
pub enum SelectorType {
    XPATH,
    CSS,
}

#[derive(Debug, Deserialize)]
pub struct Selector {
    pub selector_type: SelectorType,
    pub val: String,
}

#[derive(Debug, Deserialize)]
pub struct MetaData {
    pub title: String,
    pub author: String,
}
