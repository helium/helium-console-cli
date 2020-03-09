<<<<<<< HEAD
=======
use super::types::*;
use super::Config;
>>>>>>> lthiery/ttn
use super::Result;
use base64;
use reqwest::Client as ReqwestClient;
use std::time::Duration;
<<<<<<< HEAD
use super::config::get_input;

const BASE_URL: &str = "https://account.thethingsnetwork.org/api/v2";
const DEFAULT_TIMEOUT: u64 = 120;

#[derive(Clone, Debug)]
pub struct Client {
    key: String,
    client: ReqwestClient,
}

impl Client {
    pub fn new() -> Result<Client> {
        let key = "nAeam3v-jLSX22sSFqNubVFuSTs6Cfy4eC2aVeDWvR4".to_string();//get_input("Provide ttnctl access code");
        let client = ReqwestClient::builder()
            .timeout(Duration::from_secs(DEFAULT_TIMEOUT))
            .build()?;

        Ok(Client {
            key,
            client
        })
    }

    fn get(&self, path: &str) -> Result<reqwest::RequestBuilder> {
        Ok(self
            .client
            .get(format!("{}/{}", BASE_URL, path).as_str())
            .header("key", self.key.as_str()))
    }

    pub async fn get_applications(&self) -> Result<()> {
        let request = self.get(
            format!(
                "applications",
            )
            .as_str(),
        )?;
        let response = request.send().await?;
        println!("{:?}", response);

        let body = response.text().await.unwrap();
        println!("{:?}", body);

        Ok(())
    }
}
=======
>>>>>>> lthiery/ttn
