#[macro_use]
extern crate serde_derive;
extern crate serde_json;
#[macro_use]
extern crate prettytable;

use prettytable::Table;
use std::process;
use structopt::StructOpt;

pub type Result<T = ()> = std::result::Result<T, Box<dyn std::error::Error>>;
const CONF_PATH: &str = ".helium-console-config.toml";

mod client;
mod config;
mod ttn;
mod types;

use config::get_input;
use std::str::FromStr;
use types::*;

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
            println!("Generate a ttnctl access code at https://account.thethingsnetwork.org/");
            let mut ttn_client = ttn::Client::new()?;

            let account_token = ttn_client.get_account_token()?;

            let apps = ttn_client.get_apps(&account_token).await?;

            let mut table = Table::new();
            table.add_row(row!["Index", "Name", "ID"]);
            for (index, app) in apps.iter().enumerate() {
                table.add_row(row![index + 1, app.name, app.id,]);
            }

            table.printstd();

            let index_input = get_input(
                "Import which application? Type 0 for ALL (no more than 10 at a time supported)",
            );

            let index = usize::from_str(&index_input)?;

            if index > apps.len() {
                println!("There is no app with index {}", index);
                Ok(())
            } else {
                let mut devices = Vec::new();
                // 0 index is reserved to select all
                if index == 0 {
                    // You can restrict the OAuth2 token into having access to
                    // 10 items or less. So if we want to support more than 10
                    // applications imported at a time, we will need to ask
                    // the user for a new token
                    if apps.len() > 10 {
                        panic!("Due to TTN Auth limitations, importing more than 10 apps at once not currently supported");
                    }

                    // the account token is consumed
                    let token = ttn_client
                        .exchange_for_app_token(account_token, apps.clone())
                        .await?;
                    for app in &apps {
                        ttn_client.get_devices(&app, &token).await?;
                        devices.extend(ttn_client.get_devices(&app, &token).await?);
                    }
                // you can select one by one
                } else {
                    let app = apps[index - 1].clone();
                    // the account token is consumed
                    let token = ttn_client
                        .exchange_for_app_token(account_token, vec![app.clone()])
                        .await?;
                    devices.extend(ttn_client.get_devices(&app, &token).await?);
                }


                let config = config::load(CONF_PATH)?;
                let client = client::Client::new(config)?;

                for ttn_device in devices {
                    println!("{:?}", ttn_device);
                    let request = ttn_device.into_new_device_request()?;
                    println!("{:#?}", client.post_device(request).await?);

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
