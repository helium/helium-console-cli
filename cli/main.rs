use prettytable::{cell, row, Table};
use std::{process, str::FromStr};
use structopt::StructOpt;

pub type Result<T = ()> = std::result::Result<T, Box<dyn std::error::Error>>;
const CONF_PATH: &str = ".helium-console-config.toml";

mod clicmd;
mod config;
mod ttn;

use clicmd::*;
use config::get_input;
use helium_console::*;

/// Interact with Helium API via CLI
#[derive(StructOpt, Debug)]
pub enum Cli {
    /// List, create, and delete devices
    Device {
        #[structopt(subcommand)]
        cmd: DeviceCmd,
    },
    /// List, create, and delete labels
    Label {
        #[structopt(subcommand)]
        cmd: LabelCmd,
    },
    /// Import devices from TTN to Helium
    Ttn {
        #[structopt(subcommand)]
        cmd: TtnCmd,
    },
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
                    println!("{:#?}", client.get_device(&request).await?)
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
                    let new_device = NewDevice::from_user_input(app_eui, app_key, dev_eui, name)?;
                    println!("{:#?}", client.post_device(&new_device).await?);
                }
                DeviceCmd::Delete {
                    app_eui,
                    app_key,
                    dev_eui,
                } => {
                    let request = GetDevice::from_user_input(app_eui, app_key, dev_eui)?;
                    let device = client.get_device(&request).await?;
                    client.delete_device(device.id()).await?;
                }
                DeviceCmd::DeleteById { id } => {
                    validate_uuid_input(&id)?;
                    client.delete_device(&id).await?;
                }
                DeviceCmd::AddLabel { device, label } => {
                    let device_label = DeviceLabel::from_uuid(label)?;
                    client.add_device_label(device, &device_label).await?;
                }
                DeviceCmd::RemoveLabel { device, label } => {
                    let device_label = DeviceLabel::from_uuid(label)?;
                    client.remove_device_label(device, &device_label).await?;
                }
            }
        }
        Cli::Label { cmd } => {
            let config = config::load(CONF_PATH)?;
            let mut client = client::Client::new(config)?;

            match cmd {
                LabelCmd::List => println!("{:#?}", client.get_labels().await?),
                LabelCmd::Create { name } => {
                    let request = NewLabel::from_string(&name);
                    println!("{:#?}", client.post_label(&request).await?);
                }
                LabelCmd::DeleteById { id } => {
                    validate_uuid_input(&id)?;
                    client.delete_label(&id).await?;
                }
            }
        }
        Cli::Ttn { cmd } => match cmd {
            TtnCmd::Import => {
                ttn_import().await?;
            }
        },
    }
    Ok(())
}

async fn ttn_import() -> Result {
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

    let index_input =
        get_input("Import which application? Type 0 for ALL (no more than 10 at a time supported)");

    let index = get_number_from_user(index_input);

    if index > apps.len() {
        println!("There is no app with index {}", index);
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
        let mut client = client::Client::new(config)?;

        let first_answer =
            get_input(format!("Import all {} devices at once? Otherwise, proceed with device by device import. Please type y or n", devices.len()).as_str());
        let input_all = yes_or_no(first_answer, Some("Import ALL devices? Please type y or n"));
        let first_answer =
            get_input("Apply TTN application ID as Label to ALL devices? Please type y or n");
        let label_all = yes_or_no(first_answer, Some(" Please type y or n"));

        let do_label = if UserResponse::No == label_all {
            let first_answer =
                get_input("Skip applying TTN application ID as Label to ALL devices? Otherwise, proceed with device by device approval. Please type y or n");
            let dont_label_all = yes_or_no(first_answer, Some(" Please type y or n"));

            match dont_label_all {
                UserResponse::No => UserResponse::Maybe,
                UserResponse::Yes => UserResponse::No,
                UserResponse::Maybe => panic!("maybe not valid here"),
            }
        } else {
            UserResponse::Yes
        };

        for ttn_device in devices {
        
        
        	if ttn_device.appkey() == "" {
        		
        		if ttn_device.appskey() != "" {
        			println!("{}", format!("WARNING: ABP device not supported {:?}", ttn_device.get_simple_string()).as_str())
        		}
        	
        	} else {
				// if user elected to import all
				// create_device will always be Yes
				let create_device = match input_all {
					UserResponse::Yes => UserResponse::Yes,
					UserResponse::No => {
						let first_answer = get_input(
							format!("Import device? {:?}", ttn_device.get_simple_string()).as_str(),
						);
						yes_or_no(first_answer, Some("Please type y or n"))
					}
					UserResponse::Maybe => panic!("User reponse for create device must be yes or no"),
				};

				match create_device {
					UserResponse::Yes => {
						let appid = ttn_device.appid().clone();
						let request = ttn_device.into_new_device_request()?;

						let device = match client.post_device(&request).await {
							Ok(device) => {
								println!("Successly Created {:?}", device);
								Some(device)
							}
							Err(err) => {
								println!("{}", err);
								if let Some(error) = err.downcast_ref::<Error>() {
									match error {
										Error::NewDevice422 => {
											let request = GetDevice::from_user_input(
												request.app_eui().clone(),
												request.app_key().clone(),
												request.dev_eui().clone(),
											)?;
											Some(client.get_device(&request).await?)
										}
										_ => None,
									}
								} else {
									None
								}
							}
						};

						if let Some(device) = device {
							let confirm = match do_label {
								UserResponse::Yes => true,
								UserResponse::No => false,
								UserResponse::Maybe => {
									let first_answer = get_input("Add label to device?");
									let answer = yes_or_no(first_answer, Some("Please type y or n"));
									match answer {
										UserResponse::Yes => true,
										UserResponse::No => false,
										UserResponse::Maybe => panic!("Maybe should not occur here"),
									}
								}
							};
							if confirm {
								println!("Adding label to device {}", appid);
								let label_uuid = client.get_label_uuid(&appid).await?;
								let device_label = DeviceLabel::from_uuid(label_uuid)?;
								client
									.add_device_label(device.id().to_string(), &device_label)
									.await?;
							}
						}
					}
					UserResponse::No => {
						println!("Skipping device");
					}
					UserResponse::Maybe => panic!("User reponse for create device must be yes or no"),
				}
			}
        }
    }
    Ok(())
}

#[derive(PartialEq)]
enum UserResponse {
    Yes,
    No,
    Maybe,
}

fn yes_or_no(mut answer: String, repeated_prompt: Option<&str>) -> UserResponse {
    let prompt = if let Some(prompt) = repeated_prompt {
        prompt
    } else {
        ""
    };
    loop {
        match answer.as_str() {
            "Y" | "y" | "YES" | "Yes" | "yes" => {
                return UserResponse::Yes;
            }
            "N" | "n" | "NO" | "No" | "no" => {
                return UserResponse::No;
            }
            _ => {
                answer = get_input(prompt);
            }
        }
    }
}

fn get_number_from_user(mut answer: String) -> usize {
    loop {
        match usize::from_str(&answer) {
            Ok(num) => return num,
            _ => {
                answer = get_input("Invalid number. Please enter a number");
            }
        }
    }
}
