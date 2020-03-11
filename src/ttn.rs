extern crate base64;
extern crate oauth2;
extern crate rand;
extern crate url;

use super::config::get_input;
use super::types::NewDeviceRequest;
use super::Result;
use oauth2::basic::BasicClient;
use oauth2::prelude::*;
use oauth2::AccessToken;
use oauth2::{AuthUrl, AuthorizationCode, ClientId, ClientSecret, TokenResponse, TokenUrl};
use reqwest::Client as ReqwestClient;
use std::time::Duration;
use url::Url;

const ACCOUNT_BASE_URL: &str = "https://account.thethingsnetwork.org";

// This is a collection of "handlers" (ie: Network Servers?)
// They are queried one by one in hopes of finding the device data
const APP_BASE_URL: [&str; 4] = [
    "http://us-west.thethings.network:8084",
    "http://eu.thethings.network:8084",
    "http://asia-se.thethings.network:8084",
    "http://brazil.thethings.network:8084",
];

const DEFAULT_TIMEOUT: u64 = 120;

pub struct Client {
    client: ReqwestClient,
}

impl Client {
    pub fn new() -> Result<Client> {
        Ok(Client {
            client: ReqwestClient::builder()
                .timeout(Duration::from_secs(DEFAULT_TIMEOUT))
                .build()?,
        })
    }

    pub fn get_account_token(&self) -> Result<AccessToken> {
        let client = BasicClient::new(
            ClientId::new("ttnctl".to_string()),
            Some(ClientSecret::new("ttnctl".to_string())),
            AuthUrl::new(Url::parse(
                format!("{}/users/authorize", ACCOUNT_BASE_URL).as_str(),
            )?),
            Some(TokenUrl::new(Url::parse(
                format!("{}/users/token", ACCOUNT_BASE_URL).as_str(),
            )?)),
        );
        let access_code = get_input("Provide ttnctl access code");
        let code = AuthorizationCode::new(access_code.to_string());
        let token_res = client.exchange_code(code).unwrap();
        Ok(token_res.access_token().clone())
    }

    fn get_with_token(&self, token: &str, path: &str) -> reqwest::RequestBuilder {
        self.client
            .get(format!("{}{}", ACCOUNT_BASE_URL, path).as_str())
            .bearer_auth(token)
    }

    fn post_with_token(&self, token: &str, path: &str) -> reqwest::RequestBuilder {
        self.client
            .post(format!("{}{}", ACCOUNT_BASE_URL, path).as_str())
            .bearer_auth(token)
    }

    pub async fn get_apps(&self, token: &AccessToken) -> Result<Vec<App>> {
        let request =
            self.get_with_token(&token.secret(), format!("/api/v2/applications").as_str());
        let response = request.send().await?;
        let body = response.text().await.unwrap();
        let apps: Vec<App> = serde_json::from_str(&body)?;
        Ok(apps)
    }

    pub async fn exchange_for_app_token(
        &mut self,
        token: AccessToken,
        apps: Vec<App>,
    ) -> Result<String> {
        let mut token_request = RequestToken { scope: Vec::new() };

        for app in apps {
            token_request.scope.push(format!("apps:{}", app.id));
        }
        let request = self
            .post_with_token(token.secret(), "/users/restrict-token")
            .json(&token_request);

        let response = request.send().await?;
        let body = response.text().await.unwrap();
        let token_response: RequestTokenResponse = serde_json::from_str(&body)?;
        Ok(token_response.access_token)
    }

    pub async fn get_devices(&self, app: &App, token: &String) -> Result<Vec<Device>> {
        // We brute force going through handler URLs
        for url in &APP_BASE_URL {
            let request = self
                .client
                .get(format!("{}/applications/{}/devices", url, app.id).as_str())
                .bearer_auth(token);
            let response = request.send().await?;
            // Response 200 means we got a hit
            // this server has device information
            if response.status() == 200 {
                let body = response.text().await.unwrap();
                let devices: Devices = serde_json::from_str(&body)?;

                let mut ret = Vec::new();
                for device in devices.devices {
                    ret.push(device.lorawan_device)
                }
                return Ok(ret);
            }
        }
        Err(Error::NoHandler.into())
    }
}

#[derive(Clone, Deserialize, Serialize, Debug)]
pub struct App {
    pub id: String,
    euis: Vec<String>,
    pub name: String,
    access_keys: Vec<Key>,
}
#[derive(Clone, Deserialize, Serialize, Debug)]
struct Key {
    name: String,
    key: String,
    _id: String,
    rights: Vec<String>,
}

#[derive(Clone, Deserialize, Serialize, Debug)]
struct RequestToken {
    scope: Vec<String>,
}
#[derive(Clone, Deserialize, Serialize, Debug)]
struct RequestTokenResponse {
    access_token: String,
}

#[derive(Clone, Deserialize, Serialize, Debug)]
struct Devices {
    devices: Vec<TtnDevice>,
}

#[derive(Clone, Deserialize, Serialize, Debug)]
struct TtnDevice {
    app_id: String,
    dev_id: String,
    lorawan_device: Device,
}

#[derive(Clone, Deserialize, Serialize, Debug)]
pub struct Device {
    app_eui: String,
    dev_eui: String,
    app_id: String,
    dev_id: String,
    dev_addr: String,
    nwk_s_key: String,
    app_s_key: String,
    app_key: String,
    uses32_bit_f_cnt: bool,
    activation_constraints: String,
}

impl Device {
    pub fn into_new_device_request(self) -> Result<NewDeviceRequest> {
        NewDeviceRequest::from_user_input(
            self.app_eui,
            self.app_key,
            self.dev_eui,
            // assign it some unique'ish name
            self.dev_id,
        )
    }

    pub fn get_simple_string(&self) -> String {
        format!(
            "TtnDevice {{ app_eui: \"{}\", dev_eui: \"{}\", app_id: \"{}\", dev_id: \"{}\"}}",
            self.app_eui, self.dev_eui, self.app_id, self.dev_id
        )
    }
}

use std::error::Error as stdError;
use std::{fmt, str};

#[derive(Debug, Clone)]
pub enum Error {
    NoHandler,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::NoHandler => write!(f, "No handler servers are associated with App"),
        }
    }
}

impl stdError for Error {
    fn description(&self) -> &str {
        match self {
            Error::NoHandler => "No handler servers are associated with App",
        }
    }

    fn cause(&self) -> Option<&dyn stdError> {
        // Generic error, underlying cause isn't tracked.
        None
    }
}
