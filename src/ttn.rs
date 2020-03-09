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
    RedirectUrl,
    Scope,
    TokenResponse,
    TokenUrl
};
use oauth2::basic::BasicClient;
use url::Url;
use super::Result;
pub struct Client;

impl Client {
    pub fn new() -> Result<()>{
        // Create an OAuth2 client by specifying the client ID, client secret, authorization URL and
        // token URL.
        let client =
            BasicClient::new(
                ClientId::new("ttnctl".to_string()),
                Some(ClientSecret::new("ttnctl".to_string())),
                AuthUrl::new(Url::parse("https://account.thethingsnetwork.org/api/v2/applications/token")?),
                Some(TokenUrl::new(Url::parse("https://account.thethingsnetwork.org/api/v2/applications/token")?)),
            );
        // Generate the authorization URL to which we'll redirect the user.
        let (authorize_url, csrf_state) = client.authorize_url(CsrfToken::new_random);
        
        let code = AuthorizationCode::new("JGksMQMgTI_RtVHmYA-NMZbpAEnH7FM4Afudn37E624".to_string());;


        // Exchange the code with a token.
        let token_res = client.exchange_code(code);

        println!("TTN returned the following token:\n{:?}\n", token_res);

        Ok(())
    }
}



/*
use base64;
use reqwest::Client as ReqwestClient;
use std::time::Duration;
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
*/

