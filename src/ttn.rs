extern crate base64;
extern crate oauth2;
extern crate rand;
extern crate url;

use oauth2::prelude::*;
use oauth2::{
    AuthorizationCode,
    AuthUrl,
    ClientId,
    ClientSecret,
    CsrfToken,
    TokenResponse,
    TokenUrl,
    AccessToken,
    Scope
};
use oauth2::basic::BasicClient;
use url::Url;
use super::Result;
use super::config::get_input;
use reqwest::Client as ReqwestClient;
use std::time::Duration;
//use hyper::header::{Headers, Authorization, Bearer};

const AUTH_BASE_URL: &str = "https://account.thethingsnetwork.org";
const BASE_URL: &str = "https://account.thethingsnetwork.org/api/v2";


const SCRAPE_DEVICES: &str = "http:us-west.thethings.network:8084";
const DEFAULT_TIMEOUT: u64 = 120;
pub struct Client {
    token: AccessToken,
    client: ReqwestClient
}


/*
HTTP: http://<region>.thethings.network:8084
Replace <region> with the last part of the handler you registered your application to, e.g. eu, us-west, asia-se or brazil.
*/

use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
struct Point {
    x: i32,
    y: i32,
}

impl Client {
    pub fn new() -> Result<Client>{
        // Create an OAuth2 client by specifying the client ID, client secret, authorization URL and
        // token URL.
        let client =
            BasicClient::new(
                ClientId::new("ttnctl".to_string()),
                Some(ClientSecret::new("ttnctl".to_string())),
                AuthUrl::new(Url::parse(format!("{}/users/authorize", AUTH_BASE_URL).as_str())?),
                Some(TokenUrl::new(Url::parse(format!("{}/users/token", AUTH_BASE_URL).as_str())?)),
            );

        let access_code = get_input("Provide ttnctl access code\r\n");

        let code = AuthorizationCode::new(access_code.to_string());

        // Exchange the code with a token.
        let token_res = client.exchange_code(code).unwrap();
        println!("{:?}", token_res);

        println!("{:?}", token_res.access_token().clone().secret());
        Ok(Client {
            token: token_res.access_token().clone(),
            client: ReqwestClient::builder()
            .timeout(Duration::from_secs(DEFAULT_TIMEOUT))
            .build()?
        })
    }

    fn get(&self, path: &str) -> Result<reqwest::RequestBuilder> {

        Ok(self
            .client
            .get(format!("{}/{}", BASE_URL, path).as_str())
            .bearer_auth(self.token.secret())
            )
    }

    fn get2(&self, path: &str) -> Result<reqwest::RequestBuilder> {

        Ok(self
            .client
            .get(format!("{}/{}", SCRAPE_DEVICES, path).as_str())
            //.bearer_auth(self.token.secret())
            )
    }

    pub async fn get_applications(&self) -> Result<Vec<App>> {
        let request = self.get(
            format!(
                "applications",
            )
            .as_str(),
        )?;
        let response = request.send().await?;
        let body = response.text().await.unwrap();
        println!("{:?}", body);
        let apps: Vec<App> = serde_json::from_str(&body)?;
        Ok(apps)
    }

    pub async fn get_devices(&self, app: App) -> Result<()> {
        let request = self.get2(
            format!(
                "applications/{}/devices",
                app.id,
            )
            .as_str(),
        )?;
        println!("{:?}", request);
        let response = request.send().await?;
        let body = response.text().await.unwrap();
        println!("{:?}", body);
        //let apps: Vec<App> = serde_json::from_str(&body)?;
        Ok(())
    }
}

#[derive(Clone, Deserialize, Serialize, Debug)]
pub struct App {
    id: String,
    euis: Vec<String>,
    name: String,
}

// "[\n\n
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

