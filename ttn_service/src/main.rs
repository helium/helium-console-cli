use reset_router::{Request, RequestExtensions, Response, Router};
use helium_console::{oauth2, ttn};
use oauth2::{prelude::SecretNewType, AuthorizationCode};
use serde_derive::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct AuthResponse {
    account_token: String,
    apps: Vec<ttn::App>
}

async fn auth(req: Request) -> Result<Response, Response> {

    if let Some(access_code) = req.captures().unwrap().get(1){
        let auth_code = AuthorizationCode::new(access_code.to_string());
        let ttn_client = ttn::Client::new().unwrap();
        let account_token = match ttn_client.get_account_token(auth_code) {
            Ok(account_token) => account_token,
            Err(e) => {
                return Ok(http::Response::builder()
                    .status(400)
                    .body(format!("{}", e).into())
                    .unwrap())
            }
        };

        let apps = ttn_client.get_apps(&account_token).await.unwrap();
        let response = AuthResponse {
            account_token: account_token.secret().clone(),
            apps,
        };

        Ok(http::Response::builder()
            .status(200)
            .body(serde_json::to_string(&response).unwrap().into())
            .unwrap())

    } else {
        Ok(http::Response::builder()
            .status(404)
            .body("404".into())
            .unwrap())
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let router = Router::build()
        .add(http::Method::POST, r"^/access_code/([^/]+)$", auth)
        .add_not_found(|_| {
            async {
                Ok::<_, Response>(http::Response::builder().status(404).body("404".into()).unwrap())
            }
        })
        .finish()?;

    let addr = "0.0.0.0:8080".parse()?;
    let server = hyper::Server::bind(&addr).serve(router);

    server.await?;

    Ok(())
}