use reset_router::{Response, Router};
use std::{
    env,
    net::{IpAddr, Ipv4Addr, SocketAddr},
};

mod handlers;
use handlers::{auth, devices, exchange};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Get the port number to listen on.
    let port = env::var("PORT")
        .unwrap_or_else(|_| "3000".to_string())
        .parse()
        .expect("PORT must be a number");

    let router = Router::build()
        .add(http::Method::POST, r"^/access_code/([^/]+)$", auth)
        .add(http::Method::POST, r"^/exchange", exchange)
        .add(http::Method::GET, r"^/devices", devices)
        .add_not_found(|_| async {
            Ok::<_, Response>(
                http::Response::builder()
                    .status(404)
                    .body("404".into())
                    .unwrap(),
            )
        })
        .finish()?;

    let addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)), port);
    let server = hyper::Server::bind(&addr).serve(router);

    server.await?;

    Ok(())
}
