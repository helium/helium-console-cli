extern crate base64;
extern crate oauth2;
extern crate rand;
extern crate url;

use super::config::get_input;
use super::Result;
use oauth2::basic::BasicClient;
use oauth2::prelude::*;
use oauth2::{
    AccessToken, AuthUrl, AuthorizationCode, ClientId, ClientSecret, CsrfToken, Scope,
    TokenResponse, TokenUrl,
};
use reqwest::Client as ReqwestClient;
use std::time::Duration;
use url::Url;
//use hyper::header::{Headers, Authorization, Bearer};

const ACCOUNT_BASE_URL: &str = "https://account.thethingsnetwork.org";

const APP_BASE_URL: &str = "https://discovery.thethingsnetwork.org";

const DEFAULT_TIMEOUT: u64 = 120;
pub struct Client {
    token: AccessToken,
    client: ReqwestClient,
}

/*
HTTP: http://<region>.thethings.network:8084
Replace <region> with the last part of the handler you registered your application to, e.g. eu, us-west, asia-se or brazil.
*/

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct Point {
    x: i32,
    y: i32,
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
        )
        .add_scope(Scope::new("apps:soilsensor929".to_string()));

        let access_code = get_input("Provide ttnctl access code\r\n");
        //println!("Creating exchan")
        let code = AuthorizationCode::new(access_code.to_string());

        // Exchange the code with a token.
        let token_res = client.exchange_code(code).unwrap();
        println!("Token response {:?}", token_res);

        //println!("{:?}", token_res.access_token().clone().secret());
        Ok(Client {
            token: token_res.access_token().clone(),
            client: ReqwestClient::builder()
                .timeout(Duration::from_secs(DEFAULT_TIMEOUT))
                .build()?,
        })
    }

    fn get(&self, path: &str) -> Result<reqwest::RequestBuilder> {
        Ok(self
            .client
            .get(format!("{}{}", ACCOUNT_BASE_URL, path).as_str())
            .bearer_auth(self.token.secret()))
    }

    fn post(&self, path: &str) -> Result<reqwest::RequestBuilder> {
        Ok(self
            .client
            .post(format!("{}{}", ACCOUNT_BASE_URL, path).as_str())
            .bearer_auth(self.token.secret()))
    }

    fn post_with_key(&self, path: &str, key: &str) -> Result<reqwest::RequestBuilder> {
        Ok(self
            .client
            .post(format!("{}{}", ACCOUNT_BASE_URL, path).as_str()))
    }

    fn get_with_key(&self, path: &str, key: &str) -> Result<reqwest::RequestBuilder> {
        Ok(self
            .client
            .get(format!("{}{}", APP_BASE_URL, path).as_str())
            .header("key", key))
    }

    pub async fn get_applications(&self) -> Result<Vec<App>> {
        let request = self.get(format!("/api/v2/applications",).as_str())?;
        let response = request.send().await?;
        let body = response.text().await.unwrap();
        println!("{:?}", body);
        let apps: Vec<App> = serde_json::from_str(&body)?;
        Ok(apps)
    }

    pub async fn get_app_token(&self, app: App) -> Result<()> {
        let token_request = TokenRequest {
            username: app.id().to_string(),
            password: app.get_default_key()?.to_string(),
            grant_type: "password".to_string(),
        };

        println!("TokenRequest {:?}", token_request);

        let request = self
            .post_with_key("/api/v2/applications/token", app.get_default_key()?)?
            .json(&token_request);

        println!("REQUEST: {:?}", request);

        let response = request.send().await?;
        let body = response.text().await.unwrap();

        println!("BODY: {:?}", body);
        Ok(())
    }

    pub async fn get_devices(&self, app: App) -> Result<()> {
        #[derive(Serialize)]
        struct Request {
            app_id: String,
        }

        let request = self.get_with_key(
            format!("/applications/{}/devices", app.id(),).as_str(),
            app.get_default_key()?,
        )?;
        //.json(&Request {
        //    app_id: app.id().to_string()
        //});
        println!("{:?}", request);
        let response = request.send().await?;
        println!("Response {:?}", response);

        println!("Fetching body");
        let body = response.text().await.unwrap();
        println!("{:?}", body);
        //let apps: Vec<App> = serde_json::from_str(&body)?;
        Ok(())
    }

    //eiPq8mEeYRL_PNBZsOpPy-O3ABJXYWulODmQGR5PZzg
}

#[derive(Clone, Deserialize, Serialize, Debug)]
pub struct App {
    id: String,
    euis: Vec<String>,
    name: String,
    access_keys: Vec<Key>,
}
#[derive(Clone, Deserialize, Serialize, Debug)]
struct Key {
    name: String,
    key: String,
    _id: String,
    rights: Vec<String>,
}

impl App {
    fn id(&self) -> &String {
        &self.id
    }
    fn get_default_key(&self) -> Result<&String> {
        for key in &self.access_keys {
            if key.name == "default key" {
                return Ok(&key.key);
            }
        }
        panic!("could not find default key");
    }
}

#[derive(Clone, Deserialize, Serialize, Debug)]
pub struct TokenRequest {
    username: String,
    password: String,
    grant_type: String,
}

// {\"id\":\"soilsensor929\",
// \"name\":\"Soil Sensor\",
// \"euis\":[\"70B3D57ED002C177\"],
// \"created\":\"2020-03-09T17:01:30.086Z\",
// \"rights\":
// [\"settings\",\"delete\",\"collaborators\",\"devices\"],
// \"collaborators\":[{\"username\":\"lthiery929\",\"email\":\"thiery.louis@gmail.com\",\"rights\":[\"settings\",\"delete\",\"collaborators\",\"devices\"]}],\"access_keys\":[{\"name\":\"default key\",\"key\":\"ttn-account-v2.O5zL2lQ76Fq7coQb_BzYvoCJffTJ3RlKtxrDACI_mRM\",\"_id\":\"5e66766aa03b3d003b67cb37\",\"rights\":[\"messages:up:r\",\"messages:down:w\",\"devices\"]}]}\n]\n"

/*
use base64;
use reqwest::Client as ReqwestClient;

const BASE_URL: &str = "https://account.thethingsnetwork.org/api/v2";
const DEFAULT_TIMEOUT: u64 = 120;

#[derive(Clone, Debug)]
pub struct Client {
    key: String,
    client: ReqwestClient,
}

impl Client {
    pub fn new() -> Result<Client> {
        let key = "nAeam3v-jLSX22sSFqNubVFuSTs6Cfy4eC2aVeDWvR4".to_string();//
        let client = ReqwestClient::builder()
            .timeout(Duration::from_secs(DEFAULT_TIMEOUT))
            .build()?;

        Ok(Client {
            key,
            client
        })
    }


}
*/
