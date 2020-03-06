use super::types::*;
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

    fn get(&self, path: &str) -> Result<reqwest::RequestBuilder> {
        Ok(self
            .client
            .get(format!("{}/{}", self.base_url, path).as_str())
            .header("key", self.key.as_str()))
    }

    fn post(&self, path: &str) -> Result<reqwest::RequestBuilder> {
        Ok(self
            .client
            .post(format!("{}/{}", self.base_url, path).as_str())
            .header("key", self.key.as_str()))
    }

    pub async fn get_devices(&self) -> Result<Vec<Device>> {
        let request = self.get("api/ext/devices")?;
        let response = request.send().await?;
        let body = response.text().await.unwrap();
        let devices: Vec<Device> = serde_json::from_str(&body)?;
        Ok(devices)
    }

    pub async fn post_device(&self, new_device: NewDevice) -> Result<()> {
        let new_device_request = NewDeviceRequest { device: new_device };
        let request = self.post("api/ext/devices")?.json(&new_device_request);
        let response = request.send().await?;
        let response_body = response.text().await?;
        println!("{:?}", response_body);
        Ok(())
    }
}

/*
POST /api/cli/devices
    Content-Type: application/json
    {
        "device": {
            "name": "test",
            "dev_eui": "0000000000000000",
            "app_eui": "0000000000000000",
            "app_key": "11111111111111111111111111111111"
        }
    }
*/
