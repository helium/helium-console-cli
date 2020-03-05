#[macro_use]
extern crate serde_derive;
extern crate serde_json;

use std::process;
use structopt::StructOpt;

pub type Result<T = ()> = std::result::Result<T, Box<dyn std::error::Error>>;
const CONF_PATH: &str = ".helium-console-config.toml";

mod client;
mod config;
mod types;

#[derive(StructOpt, Debug)]
enum DeviceCmd {
    List,
    Get,
    Post { name: String },
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

#[tokio::main]
async fn main() -> Result {
    let config = config::load(CONF_PATH)?;
    let client = client::Client::new(config)?;

    let cli = Cli::from_args();

    if let Err(e) = run(cli, client).await {
        println!("error: {}", e);
        process::exit(1);
    }
    Ok(())
}

async fn run(cli: Cli, client: client::Client) -> Result {
    match cli {
        Cli::Device { cmd } => {
            match cmd {
                DeviceCmd::List => {
                    println!("{:#?}", client.get_devices().await?);
                }
                DeviceCmd::Get => {}
                DeviceCmd::Post { name } => {
                    println!("{:?}", name);
                }
            }
            Ok(())
        }
    }
}
