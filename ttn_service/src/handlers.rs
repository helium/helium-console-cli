use serde_derive::{Deserialize, Serialize};
use helium_console::{oauth2, ttn};
use oauth2::{prelude::SecretNewType, AccessToken, AuthorizationCode};
use reset_router::{Request, RequestExtensions, Response};


pub async fn auth(req: Request) -> Result<Response, Response> {
    #[derive(Serialize, Debug)]
    pub struct Response {
        account_token: String,
        apps: Vec<ttn::App>,
    }

    if let Some(access_code) = req.captures().unwrap().get(1) {
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
        let response = Response {
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

pub async fn exchange(req: Request) -> Result<Response, Response> {
    #[derive(Deserialize, Debug)]
    pub struct Request {
        account_token: String,
        apps: Vec<String>,
    }
    #[derive(Serialize, Debug)]
    pub struct Response {
        restricted_token: String,
    }
    let (_parts, body) = req.into_parts();
    let bytes = hyper::body::to_bytes(body).await.unwrap();

    let request: Result<Request, serde_json::error::Error> = serde_json::from_slice(&bytes);
    match request {
        Ok(request) => {
            let mut ttn_client = ttn::Client::new().unwrap();
            let account_token = AccessToken::new(request.account_token);

            let restricted_token = match ttn_client
                .exchange_for_app_token(account_token, request.apps)
                .await
            {
                Ok(token) => token,
                Err(e) => {
                    return Ok(http::Response::builder()
                        .status(401)
                        .body(format!("{}", e).into())
                        .unwrap())
                }
            };

            let response = Response { restricted_token };
            Ok(http::Response::builder()
                .status(200)
                .body(serde_json::to_string(&response).unwrap().into())
                .unwrap())
        }
        Err(e) => {
            return Ok(http::Response::builder()
                .status(400)
                .body(format!("{}", e).into())
                .unwrap())
        }
    }
}


pub async fn devices(req: Request) -> Result<Response, Response> {
    #[derive(Deserialize, Debug)]
    pub struct Request {
        restricted_token: String,
        appid: String,
    }
    let (_parts, body) = req.into_parts();
    let bytes = hyper::body::to_bytes(body).await.unwrap();
    let request: Result<Request, serde_json::error::Error> = serde_json::from_slice(&bytes);
    match request {
        Ok(request) => {
            let ttn_client = ttn::Client::new().unwrap();
            let devices = match ttn_client.get_devices(&request.appid, request.restricted_token.as_str()).await {
                Ok(devices) => devices,
                Err(e) => {
                    return Ok(http::Response::builder()
                        .status(401)
                        .body(format!("{}", e).into())
                        .unwrap())
                }
            };

            #[derive(Serialize, Debug)]
            pub struct Response {
                devices: Vec<ttn::TtnDevice>,
            }
            let response = Response {
                devices,
            };
            Ok(http::Response::builder()
                .status(200)
                .body(serde_json::to_string(&response).unwrap().into())
                .unwrap())
        }
        Err(e) => {
            return Ok(http::Response::builder()
                .status(400)
                .body(format!("{}", e).into())
                .unwrap())
        }
    }


}