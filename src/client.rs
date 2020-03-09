use super::types::*;
use super::Config;
use super::Result;
use base64;
use reqwest::Client as ReqwestClient;
use std::time::Duration;

#[derive(Clone, Debug)]
pub struct Client {
    base_url: String,
    key: String,
    client: ReqwestClient,
}

impl Client {
    /// Create client from configuration HashMap
    pub fn new(config: Config) -> Result<Client> {
        let timeout = config.request_timeout;
        let client = ReqwestClient::builder()
            .timeout(Duration::from_secs(timeout))
            .build()?;

        // verify API key
        let key = base64::decode(&config.key)?;
        if key.len() != 32 {
            println!("Invalid key in config file");
            return Err(Error::InvalidApiKey.into());
        }

        Ok(Client {
            base_url: config.base_url.clone(),
            key: config.key.clone(),
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

    fn delete(&self, path: &str) -> Result<reqwest::RequestBuilder> {
        Ok(self
            .client
            .delete(format!("{}/{}", self.base_url, path).as_str())
            .header("key", self.key.as_str()))
    }

    pub async fn get_devices(&self) -> Result<Vec<Device>> {
        let request = self.get("api/ext/devices")?;
        let response = request.send().await?;
        let body = response.text().await.unwrap();
        let devices: Vec<Device> = serde_json::from_str(&body)?;
        Ok(devices)
    }

    pub async fn get_device(&self, get_device: GetDevice) -> Result<Device> {
        // TODO: API will change and app_key will also be used
        let request = self.get(
            format!(
                "api/ext/devices/yolo?dev_eui={}&app_eui={}",
                get_device.dev_eui(),
                get_device.app_eui()
            )
            .as_str(),
        )?;
        let response = request.send().await?;
        let body = response.text().await.unwrap();
        let devices: Device = serde_json::from_str(&body)?;
        Ok(devices)
    }

    pub async fn get_device_by_id(&self, id: &String) -> Result<Device> {
        let request = self.get(format!("api/ext/devices/{}", id).as_str())?;
        let response = request.send().await?;
        let body = response.text().await.unwrap();
        let device: Device = serde_json::from_str(&body)?;
        Ok(device)
    }

    pub async fn post_device(&self, new_device_request: NewDeviceRequest) -> Result<Device> {
        let request = self.post("api/ext/devices")?.json(&new_device_request);
        let response = request.send().await?;
        let body = response.text().await?;
        let device: Device = serde_json::from_str(&body)?;
        Ok(device)
    }

    pub async fn delete_device(&self, id: &String) -> Result<()> {
        let request = self.delete(format!("api/ext/devices/{}", id).as_str())?;
        let response = request.send().await?;
        let _response_body = response.text().await?;
        println!("Delete successful");
        Ok(())
    }
}
