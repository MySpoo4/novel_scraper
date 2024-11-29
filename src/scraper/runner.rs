use std::path::PathBuf;

use super::ScraperClient;
use crate::epub::Epub;
use crate::errors::{Error, Result};
use crate::models::{Chapter, ExcludedWords, Novel};

pub struct Runner {
    client: ScraperClient,
    novel: Novel,
}

impl Runner {
    pub async fn new(
        novel: Novel,
        excluded_words: ExcludedWords,
        proxy: Option<&str>,
    ) -> Result<Self> {
        let mut client = ScraperClient::new(proxy).await?;
        client.add_excluded_words(excluded_words.excluded_words);
        Ok(Runner { client, novel })
    }

    pub async fn run(&mut self, output_path: PathBuf) -> Result<()> {
        let mut epub = Epub::new(&self.novel.meta_data);
        let result = self.build_epub(&mut epub).await;
        epub.build(output_path)?;
        result?;
        Ok(())
    }

    pub async fn close(self) -> Result<()> {
        self.client.close().await
    }

    async fn build_epub(&mut self, epub: &mut Epub) -> Result<()> {
        let mut cur_url: Option<&str> = Some(self.novel.site.url.as_ref());
        while let Some(url) = cur_url {
            {
                let url = url.to_string();
                self.client.scrape_url(url.as_ref()).await?;
            }

            let chapter = Self::get_text(
                &mut self.client,
                &self.novel.site.identifiers.title,
                &self.novel.site.identifiers.body,
            )?;
            epub.add_chapter(chapter)?;

            cur_url = self.get_next_url();
        }

        Ok(())
    }

    fn get_next_url(&mut self) -> Option<&str> {
        let btn = self.novel.site.identifiers.next_btn.as_ref();
        self.client.get_element_attribute(btn, "href")
    }

    fn get_text(client: &mut ScraperClient, title: &str, body: &str) -> Result<Chapter> {
        let title: Option<String> = client.get_text(title);
        let body: Option<String> = client.get_text(body);

        title
            .zip(body)
            .map(|(title, body)| Chapter { title, body })
            .ok_or_else(|| Error::AttributeError {
                selector: "title, body".to_string(),
                attr: "text".to_string(),
            })
    }
}
