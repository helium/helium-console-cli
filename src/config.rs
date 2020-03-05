use std::collections::HashMap;
use std::fs::File;
use std::io::{stdin, Write};
use std::path::Path;

use super::Result;

const DEFAULT_BASE_URL: &str = "https://console.helium.com";
const DEFAULT_TIMEOUT: usize = 120;

fn get_input(prompt: &str) -> String {
    print!("{}", prompt);
    let mut input = String::new();
    match stdin().read_line(&mut input) {
        Ok(_goes_into_input_above) => {}
        Err(_no_updates_is_fine) => {}
    }
    input.trim().to_string()
}

pub fn load(path: &str) -> Result<HashMap<String, String>> {
    if !Path::new(path).exists() {
        let mut file = File::create(path)?;
        let key = get_input("Enter API key\r\n");
        file.write_all(b"key = \"")?;
        file.write_all(key.as_bytes())?;
        file.write_all(b"\"\r\n")?;
        file.write_all(b"base_url = \"")?;
        file.write_all(DEFAULT_BASE_URL.as_bytes())?;
        file.write_all(b"\"\r\n")?;
        file.write_all(b"request_timeout = \"")?;
        file.write_all(DEFAULT_TIMEOUT.to_string().as_bytes())?;
        file.write_all(b"\"\r\n")?;
    }

    let mut load_config = config::Config::default();
    load_config.merge(config::File::with_name(path))?;
    Ok(load_config.try_into::<HashMap<String, String>>()?)
}
