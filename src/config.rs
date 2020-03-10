use super::types::Error;
use super::Config;
use super::Result;
use std::fs;
use std::io::{stdin, Write};
use std::path::Path;
use toml;

const DEFAULT_BASE_URL: &str = "https://console.helium.com";
const DEFAULT_TIMEOUT: u64 = 120;

pub fn get_input(prompt: &str) -> String {
    print!("{}\r\n", prompt);
    let mut input = String::new();
    match stdin().read_line(&mut input) {
        Ok(_goes_into_input_above) => {}
        Err(_no_updates_is_fine) => {}
    }
    input.trim().to_string()
}

pub fn load(path: &str) -> Result<Config> {
    if !Path::new(path).exists() {
        let mut file = fs::File::create(path)?;
        let key = get_input("Enter API key");

        // verify API key
        let key_verify = base64::decode(&key)?;
        if key_verify.len() != 32 {
            println!("Invalid API key ipnut");
            return Err(Error::InvalidApiKey.into());
        }

        let config = Config {
            key,
            base_url: DEFAULT_BASE_URL.to_string(),
            request_timeout: DEFAULT_TIMEOUT,
        };

        file.write_all(&toml::to_string(&config)?.as_bytes())?;
    }

    let contents = fs::read_to_string(path)?;
    let config: Config = toml::from_str(&contents)?;
    Ok(config)
}
