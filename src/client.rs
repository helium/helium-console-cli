use super::Result;
use reqwest::Client as ReqwestClient;
use std::collections::HashMap;
use std::time::Duration;

#[derive(Clone, Debug)]
pub struct Client {
    base_url: String,
    key: String,
    client: ReqwestClient,
}

impl Client {
    /// Create client from configuration HashMap
    pub fn new(config: HashMap<String, String>) -> Result<Client> {
        let timeout = config["request_timeout"].parse::<u64>()?;
        let client = ReqwestClient::builder()
            .timeout(Duration::from_secs(timeout))
            .build()?;

        Ok(Client {
            base_url: config["base_url"].clone(),
            key: config["key"].clone(),
            client,
        })
    }

    pub async fn get(&self, path: &str) -> Result<reqwest::Response> {
        let request = self
            .client
            .get(format!("{}/{}", self.base_url, path).as_str())
            .header("key", self.key.as_str());
        Ok(request.send().await?)
    }
}
