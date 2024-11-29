use fantoccini::{Client, ClientBuilder, Locator};
use serde_json::{json, Map};
use std::process::{Child, Command};

use crate::errors::{Error, Result};

#[derive(Debug)]
pub struct HttpClient {
    client: Client,
    driver: Child,
}

impl HttpClient {
    pub async fn new(proxy: Option<&str>) -> Result<Self> {
        let driver = Command::new("geckodriver")
            .arg("--port")
            .arg("4444")
            .spawn()
            .map_err(|e| Error::CommandError {
                cmd: "geckodriver --port 4444".to_string(),
                message: e.to_string(),
            })?;

        // Default host for geckodriver
        const HOST: &str = "http://localhost:4444";

        // Always return a valid map, either with proxy settings or empty
        let capabilities = Self::build_capabilities(proxy);

        // Initialize the client with the capabilities
        let client = ClientBuilder::native()
            .capabilities(capabilities) // Pass the map (not Option)
            .connect(HOST)
            .await
            .map_err(|_| Error::ClientBuildError)?;

        Ok(HttpClient { client, driver })
    }

    pub async fn close(mut self) -> Result<()> {
        let _ = self.driver.kill().map_err(|e| Error::Error {
            message: e.to_string(),
        })?;
        let _ = self.client.close().await.map_err(|e| Error::Error {
            message: e.to_string(),
        })?;
        Ok(())
    }

    fn build_capabilities(proxy: Option<&str>) -> Map<String, serde_json::Value> {
        let mut capabilities = Map::new();

        // If a proxy is provided, add the proxy settings
        if let Some(proxy) = proxy {
            let proxy_config = json!({
                "proxyType": "manual",
                "httpProxy": proxy,
                "sslProxy": proxy,
                "ftpProxy": proxy,
                "noProxy": "" // Optional: Domains to exclude
            });
            capabilities.insert("proxy".to_string(), proxy_config);
        }

        // Return the capabilities map, which may be empty or contain the proxy
        capabilities
    }

    pub async fn get_html<'a>(&self, url: &'a str) -> Result<String> {
        self.client
            .goto(url)
            .await
            .map_err(|e| Error::RequestError {
                url: url.to_string(),
                message: e.to_string(),
            })?;

        self.client
            .find(Locator::Css("body"))
            .await
            .map_err(|_e| Error::ElementError {
                selector: "body".to_string(),
            })?
            .html(true)
            .await
            .map_err(|_e| Error::AttributeError {
                selector: "body".to_string(),
                attr: "html".to_string(),
            })
    }
}
