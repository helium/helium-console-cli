use std::process;
use structopt::StructOpt;
use reqwest::Client;
use std::collections::HashMap;
use std::path::Path;
use std::io::Write;
use std::io;
use std::fs::File;

pub type Result<T = ()> = std::result::Result<T, Box<dyn std::error::Error>>;

#[derive(StructOpt, Debug)]
enum DeviceCmd {
    List,
    Get,
    Post {
        name: String,
    },
}

/// Interact with Helium API via CLI
#[derive(Debug, StructOpt)]
enum Cli {
    /// Interact with device models
    Device {
        #[structopt(subcommand)] 
        cmd: DeviceCmd,
    },

}

const CONF_PATH: &str = ".helium-console-config.toml";


pub fn get_input(prompt: &str) -> String{
    print!("{}",prompt);
    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(_goes_into_input_above) => {},
        Err(_no_updates_is_fine) => {},
    }
    input.trim().to_string()
}

#[tokio::main]
async fn main() -> Result {

    if !Path::new(CONF_PATH).exists() {
        let mut file = File::create(CONF_PATH)?;

        let key = get_input("Enter API key\r\n");
        
        file.write_all(b"key = \"")?;
        file.write_all(key.as_bytes())?;
        file.write_all(b"\"")?;

    }

    let mut load_config = config::Config::default();
    load_config.merge(config::File::with_name(CONF_PATH))?;

    let config = load_config.try_into::<HashMap<String, String>>()?;
    let cli = Cli::from_args();
    if let Err(e) = run(cli, config).await {
        println!("error: {}", e);
        process::exit(1);
    }
    Ok(())
}

async fn run(cli: Cli, config: HashMap<String, String>) -> Result {
    match cli {
        Cli::Device { cmd } => {
            match cmd {
                DeviceCmd::List => {
                    let client = Client::new();
                    let request = client.get("https://console.helium.com/api/cli/devices")
                        .header("key", &config["key"]);
                    println!("{:?}", request);
                    let response = request.send().await?;
                    let body = response.text().await?;
                    println!("{:?}", body);
                },
                DeviceCmd::Get => {
                }
                DeviceCmd::Post { name } => {
                    println!("{:?}", name);
                }
            }
            Ok(())
        }
    }
}
