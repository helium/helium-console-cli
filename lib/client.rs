use super::*;
use reqwest::Client as ReqwestClient;
use std::collections::HashMap;
use std::time::Duration;

#[derive(Debug, Deserialize, Serialize)]
pub struct Config {
    key: String,
    base_url: String,
    request_timeout: u64,
}

const DEFAULT_BASE_URL: &str = "https://console.helium.com";
const DEFAULT_TIMEOUT: u64 = 120;

impl Config {
    pub fn new(key: String) -> Config {
        Self::new_with_url(key, DEFAULT_BASE_URL)
    }

    pub fn new_with_url(key: String, url: &str) -> Config {
        Config {
            key,
            base_url: url.to_string(),
            request_timeout: DEFAULT_TIMEOUT,
        }
    }
}

#[derive(Clone, Debug)]
pub struct Client {
    base_url: String,
    key: String,
    client: ReqwestClient,
    // map label to uuid
    labels: HashMap<String, String>,
}

impl Client {
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
            base_url: config.base_url,
            key: config.key,
            client,
            labels: HashMap::new(),
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

    pub async fn get_detailed_devices(&self) -> Result<Vec<DetailedDevice>> {
        let request = self.get("api/v1/devices")?;
        let response = request.send().await?;
        if response.status() == 200 {
            let body = response.text().await.unwrap();
            let devices: Vec<DetailedDevice> = serde_json::from_str(&body)?;
            Ok(devices)
        } else if response.status() == 401 {
            let body = response.text().await.unwrap();
            println!("{}", body);
            Err(Error::UnauthorizedApi.into())
        } else {
            Err(Error::HttpErrorApi.into())
        }
    }

    pub async fn get_devices(&self) -> Result<Vec<Device>> {
        let request = self.get("api/v1/devices")?;
        let response = request.send().await?;
        if response.status() == 200 {
            let body = response.text().await.unwrap();
            let devices: Vec<Device> = serde_json::from_str(&body)?;
            Ok(devices)
        } else if response.status() == 401 {
            let body = response.text().await.unwrap();
            println!("{}", body);
            Err(Error::UnauthorizedApi.into())
        } else {
            Err(Error::HttpErrorApi.into())
        }
    }

    pub async fn get_device(&self, get_device: &GetDevice) -> Result<Device> {
        let request = self.get(
            format!(
                "api/v1/devices?dev_eui={}&app_eui={}&app_key={}",
                get_device.dev_eui(),
                get_device.app_eui(),
                get_device.app_key()
            )
            .as_str(),
        )?;
        let response = request.send().await?;
        if response.status() == 200 {
            let body = response.text().await.unwrap();
            let devices: Device = serde_json::from_str(&body)?;
            Ok(devices)
        } else if response.status() == 401 {
            let body = response.text().await.unwrap();
            println!("{}", body);
            Err(Error::UnauthorizedApi.into())
        } else {
            Err(Error::HttpErrorApi.into())
        }
    }

    pub async fn get_device_by_id(&self, id: &str) -> Result<Device> {
        let request = self.get(format!("api/v1/devices/{}", id).as_str())?;
        let response = request.send().await?;
        if response.status() == 200 {
            let body = response.text().await.unwrap();
            let device: Device = serde_json::from_str(&body)?;
            Ok(device)
        } else if response.status() == 401 {
            let body = response.text().await.unwrap();
            println!("{}", body);
            Err(Error::UnauthorizedApi.into())
        } else {
            Err(Error::HttpErrorApi.into())
        }
    }

    pub async fn post_device(&self, new_device_request: &NewDevice) -> Result<Device> {
        let request = self.post("api/v1/devices")?.json(&new_device_request);
        let response = request.send().await?;
        if response.status() == 201 {
            let body = response.text().await?;
            let device: Device = serde_json::from_str(&body)?;
            Ok(device)
        } else if response.status() == 401 {
            let body = response.text().await.unwrap();
            println!("{}", body);
            Err(Error::UnauthorizedApi.into())
        } else if response.status() == 422 {
            Err(Error::NewDevice422.into())
        } else {
            Err(Error::NewDeviceApi.into())
        }
    }

    pub async fn delete_device(&self, id: &str) -> Result<()> {
        let request = self.delete(format!("api/v1/devices/{}", id).as_str())?;
        let response = request.send().await?;
        if response.status() == 200 {
            println!("Device delete successful");
            let _response_body = response.text().await?;
            Ok(())
        } else if response.status() == 401 {
            let body = response.text().await.unwrap();
            println!("{}", body);
            Err(Error::UnauthorizedApi.into())
        } else if response.status() == 404 {
            println!("Device not found. Delete failed.");
            Ok(())
        } else {
            Err(Error::HttpErrorApi.into())
        }
    }

    /// Labels
    pub async fn get_labels(&mut self) -> Result<Vec<Label>> {
        let request = self.get("api/v1/labels")?;
        let response = request.send().await?;
        let body = response.text().await.unwrap();
        let labels: Vec<Label> = serde_json::from_str(&body)?;

        for label in &labels {
            self.labels.insert(label.name().clone(), label.id().clone());
        }
        Ok(labels)
    }

    pub async fn post_label(&self, new_label_request: &NewLabel) -> Result<Label> {
        let request = self.post("api/v1/labels")?.json(&new_label_request);
        let response = request.send().await?;
        if response.status() == 201 {
            let body = response.text().await?;
            let label: Label = serde_json::from_str(&body)?;
            Ok(label)
        } else if response.status() == 422 {
            Err(Error::NewLabel422.into())
        } else {
            Err(Error::NewLabelApi.into())
        }
    }

    pub async fn delete_label(&self, id: &str) -> Result<()> {
        let request = self.delete(format!("api/v1/labels/{}", id).as_str())?;
        let response = request.send().await?;
        if response.status() == 200 {
            println!("Label delete successful");
        } else if response.status() == 404 {
            println!("Label not found. Delete failed.");
        }
        let _response_body = response.text().await?;
        Ok(())
    }

    /// Device Label
    pub async fn add_device_label(
        &self,
        device_id: String,
        device_label: &DeviceLabel,
    ) -> Result<()> {
        let request = self
            .post(format!("api/v1/devices/{:}/labels", device_id).as_str())?
            .json(&device_label);
        let response = request.send().await?;
        if response.status() == 201 || response.status() == 200 {
            let body = response.text().await?;
            println!("{}", body);
            Ok(())
        } else {
            let body = response.text().await?;
            println!("{}", body);
            Err(Error::NewDeviceLabelApi.into())
        }
    }

    pub async fn remove_device_label(
        &self,
        device_id: String,
        device_label: &DeviceLabel,
    ) -> Result<()> {
        let request = self.delete(
            format!(
                "api/v1/devices/{:}/labels/{:}",
                device_id,
                device_label.get_uuid()
            )
            .as_str(),
        )?;
        let response = request.send().await?;
        if response.status() == 200 {
            let body = response.text().await?;
            println!("{:}", body);
        } else if response.status() == 404 {
            println!("Device label not found. Delete failed.");
        }
        Ok(())
    }

    pub async fn get_label_uuid(&mut self, device_label: &str) -> Result<String> {
        let label_upper = device_label.to_uppercase();

        // we probably haven't fetched labels if length is 0
        if self.labels.is_empty() {
            self.get_labels().await?;
        }

        // if the uuid still doesn't exist even after an intial fetch
        // create it
        if !self.labels.contains_key(&label_upper) {
            println!("Label does not exist. Creating label: {}", label_upper);
            let request = NewLabel::from_string(&label_upper);
            let label = self.post_label(&request).await?;
            self.labels.insert(label.name().clone(), label.id().clone());
        }
        // at this point, the above either errored or the label exists
        if let Some(id) = self.labels.get(&label_upper) {
            Ok(id.clone())
        } else {
            panic!("Label should exist here. Error out.")
        }
    }
}
