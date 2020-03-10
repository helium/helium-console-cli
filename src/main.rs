#[macro_use] extern crate serde_derive;
extern crate serde_json;
#[macro_use] extern crate prettytable;

use std::process;
use structopt::StructOpt;
use prettytable::Table;

pub type Result<T = ()> = std::result::Result<T, Box<dyn std::error::Error>>;
const CONF_PATH: &str = ".helium-console-config.toml";

mod client;
mod config;
mod ttn;
mod types;

use types::*;
use config::get_input;
use std::str::FromStr;

#[derive(StructOpt, Debug)]
enum DeviceCmd {
    /// List all your account devices
    List,
    /// Get the full record of your device
    /// by providing app_eui, app_key, and dev_eui
    Get {
        app_eui: String,
        app_key: String,
        dev_eui: String,
    },
    /// Delete a device
    /// by providing app_eui, app_key, and dev_eui
    Delete {
        app_eui: String,
        app_key: String,
        dev_eui: String,
    },
    /// Get the full record of your device
    /// by the UUID
    GetById { id: String },
    /// Delete a device
    /// by the UUID
    DeleteById { id: String },
    /// Create a device
    /// by providing app_eui, app_key, dev_eui and name
    Create {
        app_eui: String,
        app_key: String,
        dev_eui: String,
        name: String,
    },
}

/// Interact with Helium API via CLI
#[derive(Debug, StructOpt)]
enum Cli {
    /// Device model API allows you list, create, and delete devices
    Device {
        #[structopt(subcommand)]
        cmd: DeviceCmd,
    },
    Ttn,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Config {
    key: String,
    base_url: String,
    request_timeout: u64,
}

#[tokio::main]
async fn main() -> Result {
    let cli = Cli::from_args();

    if let Err(e) = run(cli).await {
        println!("error: {}", e);
        process::exit(1);
    }
    Ok(())
}

async fn run(cli: Cli) -> Result {
    match cli {
        Cli::Device { cmd } => {
            let config = config::load(CONF_PATH)?;
            let client = client::Client::new(config)?;

            match cmd {
                DeviceCmd::List => println!("{:#?}", client.get_devices().await?),
                DeviceCmd::Get {
                    app_eui,
                    app_key,
                    dev_eui,
                } => {
                    let request = GetDevice::from_user_input(app_eui, app_key, dev_eui)?;
                    println!("{:#?}", client.get_device(request).await?)
                }
                DeviceCmd::GetById { id } => {
                    validate_uuid_input(&id)?;
                    println!("{:#?}", client.get_device_by_id(&id).await?)
                }
                DeviceCmd::Create {
                    app_eui,
                    app_key,
                    dev_eui,
                    name,
                } => {
                    let new_device =
                        NewDeviceRequest::from_user_input(app_eui, app_key, dev_eui, name)?;
                    println!("{:#?}", client.post_device(new_device).await?);
                }
                DeviceCmd::Delete {
                    app_eui,
                    app_key,
                    dev_eui,
                } => {
                    let request = GetDevice::from_user_input(app_eui, app_key, dev_eui)?;
                    let device = client.get_device(request).await?;
                    client.delete_device(device.id()).await?;
                }
                DeviceCmd::DeleteById { id } => {
                    validate_uuid_input(&id)?;
                    client.delete_device(&id).await?;
                }
            }
            Ok(())
        }
        Cli::Ttn => {
            println!("TTN Import");
            let mut client = ttn::Client::new()?;
            let apps = client.get_applications().await?;
            
            let mut table = Table::new();
            table.add_row(row!["Index", "Name", "ID"]);
            for (index,app) in apps.iter().enumerate() {
                table.add_row(row![
                    index + 1,
                    app.name,
                    app.id,
                ]);
            }

            table.printstd();

            let index_input = get_input("Import which application? Type 0 for ALL");

            let index = usize::from_str(&index_input)?;

            if index > apps.len() {
                println!("There is no app with index {}", index);
                Ok(())
            } else {
                if index == 0 {
                    let token = client.get_app_token(apps.clone()).await?;
                    for app in &apps {
                        client.get_devices(&app, &token).await?;
                        let devices = client.get_devices(&app, &token).await?;
                        for device in devices {
                            println!("{:?}", device);
                        }
                    }
                } else {
                    let app = apps[index-1].clone();
                    let token = client.get_app_token(vec![app.clone()]).await?;
                    client.get_devices(&app, &token).await?;
                    let devices = client.get_devices(&app, &token).await?;
                    for device in devices {
                        println!("{:?}", device);
                    }
                }

                Ok(())
            }

        }
    }
}

/// Throws an error if UUID isn't properly input
fn validate_uuid_input(id: &String) -> Result {
    if let Err(err) = uuid::Uuid::parse_str(id.as_str()) {
        println!("{} [input: {}]", err, id);
        return Err(Error::InvalidUuid.into());
    }
    Ok(())
}

