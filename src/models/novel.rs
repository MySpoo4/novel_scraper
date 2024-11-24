use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Novel {
    pub meta_data: MetaData,
    pub site: Site,
}

#[derive(Debug, Deserialize)]
pub struct Site {
    pub url: String,
    pub identifiers: Identifiers,
}

#[derive(Debug, Deserialize)]
pub struct Identifiers {
    pub next_btn: String,
    pub title: String,
    pub body: String,
}

#[derive(Debug, Deserialize)]
pub struct MetaData {
    pub title: String,
    pub author: String,
}
