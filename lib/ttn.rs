use super::NewDevice;
use super::Result;
use oauth2::{
    basic::BasicClient,
    prelude::{NewType, SecretNewType},
    AccessToken, AuthUrl, AuthorizationCode, ClientId, ClientSecret, RequestTokenError,
    TokenResponse, TokenUrl,
};
use reqwest::Client as ReqwestClient;
use serde_derive::{Deserialize, Serialize};
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

const NULL_JSON: &str = "{}";

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

    pub fn get_account_token(&self, access_code: AuthorizationCode) -> Result<AccessToken> {
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

        match client.exchange_code(access_code) {
            Ok(token_res) => Ok(token_res.access_token().clone()),
            Err(e) => match e {
                RequestTokenError::ServerResponse(_) => Err(Error::CodeNotFound.into()),
                _ => panic!("Unhandled Error {}", e),
            },
        }
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
        let request = self.get_with_token(&token.secret(), "/api/v2/applications");
        let response = request.send().await?;
        let body = response.text().await.unwrap();
        let apps: Vec<App> = serde_json::from_str(&body)?;
        Ok(apps)
    }

    pub async fn exchange_for_app_token(
        &mut self,
        token: AccessToken,
        app_ids: Vec<String>,
    ) -> Result<String> {
        let mut token_request = RequestToken { scope: Vec::new() };

        for id in app_ids {
            token_request.scope.push(format!("apps:{}", id));
        }
        let request = self
            .post_with_token(token.secret(), "/users/restrict-token")
            .json(&token_request);

        let response = request.send().await?;
        let body = response.text().await.unwrap();

        let token_response: core::result::Result<RequestTokenResponse, serde_json::error::Error> =
            serde_json::from_str(&body);

        match token_response {
            Ok(token_response) => Ok(token_response.access_token),
            Err(_) => Err(Error::TokenNotFoundOrExpired.into()),
        }
    }

    pub async fn get_devices(&self, app: &App, token: &str) -> Result<Vec<TtnDevice>> {
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
                let mut ret = Vec::new();
                // if you get a 200 response but there is body is empty JSON ('{}')
                // we've hit the application server but there's no devices
                if body != NULL_JSON {
                    let devices: Devices = serde_json::from_str(&body)?;
                    for device in devices.devices {
                        ret.push(TtnDevice::from_raw(device, url));
                    }
                }
                return Ok(ret);
            }
        }
        Err(Error::NoHandler.into())
    }

    // DELETE /applications/{app_id}/devices/{dev_id}
    pub async fn delete_device(&self, device: TtnDevice, token: &str) -> Result<()> {
        let request = self
            .client
            .delete(
                format!(
                    "{}/applications/{}/devices/{}",
                    device.endpoint, device.app_id, device.dev_id
                )
                .as_str(),
            )
            .bearer_auth(token);
        let response = request.send().await?;
        println!("response {:?}", response);
        if response.status() == 200 {
            Ok(())
        } else {
            Err(Box::new(Error::DeviceNotFound))
        }
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
    devices: Vec<TtnDeviceRaw>,
}

#[derive(Clone, Deserialize, Serialize, Debug)]
pub struct TtnDeviceRaw {
    app_id: String,
    dev_id: String,
    lorawan_device: Device,
}

#[derive(Clone, Deserialize, Serialize, Debug)]
pub struct TtnDevice {
    app_id: String,
    dev_id: String,
    lorawan_device: Device,
    endpoint: &'static str,
}

impl TtnDevice {
    fn from_raw(raw: TtnDeviceRaw, endpoint: &'static str) -> TtnDevice {
        TtnDevice {
            app_id: raw.app_id,
            dev_id: raw.dev_id,
            lorawan_device: raw.lorawan_device,
            endpoint,
        }
    }
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

impl TtnDevice {
    pub fn derive_new_device_request(&self) -> Result<NewDevice> {
        NewDevice::from_user_input(
            self.lorawan_device.app_eui.clone(),
            self.lorawan_device.app_key.clone(),
            self.lorawan_device.dev_eui.clone(),
            // assign it some unique'ish name
            self.lorawan_device.dev_id.clone(),
        )
    }

    pub fn appid(&self) -> &String {
        &self.app_id
    }

    pub fn appkey(&self) -> &String {
        &self.lorawan_device.app_key
    }

    pub fn appskey(&self) -> &String {
        &self.lorawan_device.app_s_key
    }

    pub fn get_simple_string(&self) -> String {
        format!(
            "TtnDevice {{ app_eui: \"{}\", dev_eui: \"{}\", app_id: \"{}\", dev_id: \"{}\", app_id: \"{}\"}}",
            self.lorawan_device.app_eui, self.lorawan_device.dev_eui, self.lorawan_device.app_id, self.lorawan_device.dev_id, self.app_id
        )
    }
}

use std::error::Error as stdError;
use std::{fmt, str};

#[derive(Debug, Clone)]
pub enum Error {
    NoHandler,
    DeviceNotFound,
    CodeNotFound,
    TokenNotFoundOrExpired,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::NoHandler => write!(f, "No handler servers are associated with App"),
            Error::DeviceNotFound => write!(f, "Device not found for delete"),
            Error::CodeNotFound => write!(f, "Authorization code not found on TTN server"),
            Error::TokenNotFoundOrExpired => write!(f, "Token not found or expired"),
        }
    }
}

impl stdError for Error {
    fn description(&self) -> &str {
        match self {
            Error::NoHandler => "No handler servers are associated with App",
            Error::DeviceNotFound => "Device not found for delete",
            Error::CodeNotFound => "Authorization code not found on TTN server",
            Error::TokenNotFoundOrExpired => "Token not found or expired",
        }
    }

    fn cause(&self) -> Option<&dyn stdError> {
        // Generic error, underlying cause isn't tracked.
        None
    }
}
