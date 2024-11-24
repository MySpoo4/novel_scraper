use reqwest::{Client, Proxy};

use crate::errors::{Error, Result};

#[derive(Debug, Default)]
pub struct HttpClient {
    client: Client,
}

impl HttpClient {
    pub fn new(proxy: Option<&str>) -> Result<Self> {
        let client = match proxy {
            Some(proxy_url) => {
                let proxy = Proxy::all(proxy_url).map_err(|_| Error::ProxyError)?;
                Client::builder()
                    .proxy(proxy)
                    .build()
                    .map_err(|_| Error::ClientBuildError)?
            }
            None => Client::new(),
        };
        Ok(HttpClient { client })
    }

    pub async fn get_html<'a>(&self, url: &'a str) -> Result<String> {
        let html = match self.client.get(url).send().await {
            Ok(res) if res.status().is_success() => Ok(html_escape::decode_html_entities(
                res.text().await.unwrap_or_default().as_str(),
            )
            .into_owned()),
            Ok(res) => Err(Error::RequestError {
                url: url.to_string(),
                message: format!(
                    "Failed to retrieve the webpage. Status code: {}",
                    res.status()
                ),
            }),
            Err(e) => Err(Error::RequestError {
                url: url.to_string(),
                message: e.to_string(),
            }),
        };

        html
    }
}
