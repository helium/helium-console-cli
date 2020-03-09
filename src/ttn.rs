use super::types::*;
use super::Config;
use super::Result;
use base64;
use reqwest::Client as ReqwestClient;
use std::time::Duration;

const DEFAULT_BASE_URL: &str = "https://console.helium.com";
const DEFAULT_TIMEOUT: u64 = 120;

#[derive(Clone, Debug)]
pub struct Client {
    key: String,
    client: ReqwestClient,
}
