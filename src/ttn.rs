extern crate base64;
extern crate oauth2;
extern crate rand;
extern crate url;

use super::config::get_input;
use super::Result;
use oauth2::basic::BasicClient;
use oauth2::prelude::*;
use oauth2::{
    AccessToken, AuthUrl, AuthorizationCode, ClientId, ClientSecret,
    TokenResponse, TokenUrl,
};
use reqwest::Client as ReqwestClient;
use std::time::Duration;
use url::Url;

//use hyper::header::{Headers, Authorization, Bearer};

const ACCOUNT_BASE_URL: &str = "https://account.thethingsnetwork.org";

const APP_BASE_URL: &str = "http://us-west.thethings.network:8084";

const DEFAULT_TIMEOUT: u64 = 120;

pub struct Client {
    token: Option<AccessToken>,
    client: ReqwestClient,
}

impl Client {
    pub fn new() -> Result<Client> {
        // Create an OAuth2 client by specifying the client ID, client secret, authorization URL and
        // token URL.
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
        //println!("Creating exchan")
        let code = AuthorizationCode::new(access_code.to_string());

        // Exchange the code with a token.
        let token_res = client.exchange_code(code).unwrap();

        //println!("{:?}", token_res.access_token().clone().secret());
        Ok(Client {
            token: Some(token_res.access_token().clone()),
            client: ReqwestClient::builder()
                .timeout(Duration::from_secs(DEFAULT_TIMEOUT))
                .build()?,
        })
    }

    fn get(&self, path: &str) -> Result<reqwest::RequestBuilder> {
        if let Some(token) = &self.token {
            Ok(self
                .client
                .get(format!("{}{}", ACCOUNT_BASE_URL, path).as_str())
                .bearer_auth(token.secret()))
        } else {
            Err(Error::NoToken.into())
        }
    }

    fn post(&self, path: &str) -> Result<reqwest::RequestBuilder> {
        if let Some(token) = &self.token {
            Ok(self
                .client
                .post(format!("{}{}", ACCOUNT_BASE_URL, path).as_str())
                .bearer_auth(token.secret()))
        } else {
            Err(Error::NoToken.into())
        }

    }

    pub async fn get_applications(&self) -> Result<Vec<App>> {
        let request = self.get(format!("/api/v2/applications",).as_str())?;
        let response = request.send().await?;
        let body = response.text().await.unwrap();
        println!("{:?}", body);
        let apps: Vec<App> = serde_json::from_str(&body)?;
        Ok(apps)
    }

    pub async fn get_app_token(&self, app: &App) -> Result<String> {
        let token_request = RequestToken {
            scope: vec!(format!("apps:{}",app.id).to_string(), "apps".to_string())
        };

        println!("{:?}", token_request);
        let request = self
            .post("/users/restrict-token")?
            .json(&token_request);
        let response = request.send().await?;
        let body = response.text().await.unwrap();
        let token_response: RequestTokenResponse = serde_json::from_str(&body)?;
        Ok(token_response.access_token)
    }

    pub async fn get_devices(&self, app: &App, token: &String) -> Result<Vec<Device>> {
        let request = self
            .client
            .get(format!("{}/applications/{}/devices", APP_BASE_URL, app.id ).as_str())
            .bearer_auth(token);
        let response = request.send().await?;
        let body = response.text().await.unwrap();
        let devices: Devices = serde_json::from_str(&body)?;

        let mut ret = Vec::new();
        for device in devices.devices {
            ret.push(device.lorawan_device)
        }
        Ok(ret)
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
  scope: Vec<String>
}
#[derive(Clone, Deserialize, Serialize, Debug)]
struct RequestTokenResponse {
  access_token: String
}

#[derive(Clone, Deserialize, Serialize, Debug)]
struct Devices {
    devices: Vec<TtnDevice>
}

#[derive(Clone, Deserialize, Serialize, Debug)]
struct TtnDevice {
    app_id: String,
    dev_id: String,
    lorawan_device: Device
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

use std::error::Error as stdError;
use std::{fmt, str};

#[derive(Debug, Clone)]
pub enum Error {
    NoToken
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::NoToken => {
                write!(f, "Client has no token or it has been consumed")
            }
        }
    }
}

impl stdError for Error {
    fn description(&self) -> &str {
        match self {
            Error::NoToken => "Client has no token or it has been consumed",
        }
    }

    fn cause(&self) -> Option<&dyn stdError> {
        // Generic error, underlying cause isn't tracked.
        None
    }
}