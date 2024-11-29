use fantoccini::{Client, ClientBuilder, Locator};
use futures::future::join_all;
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
        const PORT: &str = "8000";
        let driver = Command::new("geckodriver")
            .arg("--port")
            .arg(PORT)
            .stdout(std::process::Stdio::null()) // disables logs
            .stderr(std::process::Stdio::null())
            .spawn()
            .map_err(|e| Error::CommandError {
                cmd: format!("geckodriver --port {}", PORT),
                message: e.to_string(),
            })?;

        // Default host for geckodriver
        let host: String = format!("http://localhost:{}", PORT);

        // Always return a valid map, either with proxy settings or empty
        let capabilities = Self::build_capabilities(proxy);

        // Initialize the client with the capabilities
        let client = ClientBuilder::native()
            .capabilities(capabilities) // Pass the map (not Option)
            .connect(&host)
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

        // capabilities.insert(
        //     "moz:firefoxOptions".to_string(),
        //     json!({
        //         "args": ["--headless"],
        //     }),
        // );

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
            .wait()
            .for_element(Locator::Css("body"))
            .await
            .map_err(|_e| Error::ElementError {
                selector: "body".to_string(),
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

    pub async fn get_attr_xpath(&self, xpath: &str, attr: &str) -> Option<String> {
        self.client
            .find(Locator::XPath(xpath))
            .await
            .ok()?
            .attr(attr)
            .await
            .ok()?
    }

    pub async fn get_text_xpath<F>(&self, xpath: &str, separator: &str, filter: F) -> Option<String>
    where
        F: Fn(&str) -> bool,
    {
        // Find all elements matching the XPath and map their text
        let elements = self
            .client
            .find_all(Locator::XPath(xpath))
            .await
            .map_err(|_e| Error::ElementError {
                selector: xpath.to_string(),
            })
            .ok()?;

        // Collect the futures to resolve
        let texts = elements.iter().map(|e| e.text()).collect::<Vec<_>>();

        // Resolve all futures concurrently
        let resolved_texts = join_all(texts).await;

        // Process the resolved texts
        let result = resolved_texts
            .into_iter()
            .filter_map(|res| res.ok())
            .map(|s| s.trim().to_string())
            .filter(|s| filter(s))
            .collect::<Vec<String>>()
            .join(separator);

        Some(result)
    }
}
