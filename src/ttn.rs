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
PkceCodeChallengeS256,
    RedirectUrl,
    Scope,
    TokenResponse,
    TokenUrl
};
use oauth2::basic::BasicClient;
use url::Url;
use super::Result;

use oauth2::http_client;

pub struct Client;

impl Client {
    pub fn new() -> Result<()>{
        // Create an OAuth2 client by specifying the client ID, client secret, authorization URL and
        // token URL.
        let client =
            BasicClient::new(
                ClientId::new("ttnctl".to_string()),
                Some(ClientSecret::new("ttnctl".to_string())),
                AuthUrl::new(Url::parse("https://account.thethingsnetwork.org")?),
                None
            )
                // Set the desired scopes.
                .add_scope(Scope::new("read".to_string()));

        // Generate a PKCE challenge.
        let (pkce_challenge, pkce_verifier) = PkceCodeChallengeS256::new_random_sha256();

        // Generate the full authorization URL.
        let (auth_url, csrf_token) = client
            .authorize_url(CsrfToken::new_random)
            // Set the desired scopes.
            .add_scope(Scope::new("read".to_string()))
            .add_scope(Scope::new("write".to_string()))
            // Set the PKCE code challenge.
            .set_pkce_challenge(pkce_challenge)
            .url();

        // This is the URL you should redirect the user to, in order to trigger the authorization
        // process.
        println!("Browse to: {}", auth_url);

        // Once the user has been redirected to the redirect URL, you'll have access to the
        // authorization code. For security reasons, your code should verify that the `state`
        // parameter returned by the server matches `csrf_state`.

        // Now you can trade it for an access token.
        let token_result =
            client
                .exchange_code(AuthorizationCode::new("some authorization code".to_string()))
                // Set the PKCE code verifier.
                .set_pkce_verifier(pkce_verifier)
                .request(http_client)?;
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

