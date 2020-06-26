use rand::Rng;
use serde_derive::{Deserialize, Serialize};

pub mod client;
pub mod errors;
pub use errors::*;
pub mod ttn;

pub use oauth2;

pub type Result<T = ()> = std::result::Result<T, Box<dyn std::error::Error>>;

#[derive(Clone, Deserialize, Debug)]
pub struct Device {
    app_eui: String,
    app_key: String,
    dev_eui: String,
    id: String,
    name: String,
    organization_id: String,
    oui: usize,
}

impl Device {
    pub fn id(&self) -> &String {
        &self.id
    }

    pub fn app_eui(&self) -> &String {
        &self.app_eui
    }

    pub fn dev_eui(&self) -> &String {
        &self.dev_eui
    }

    pub fn app_key(&self) -> &String {
        &self.app_key
    }

    pub fn random_dev_eui() -> String {
        let mut rng = rand::thread_rng();
        let mut deveui_bytes = Vec::new();
        for _ in 0..8 {
            deveui_bytes.push(rng.gen())
        }
        hex::encode(deveui_bytes)
    }

    pub fn random_app_key() -> String {
        let mut rng = rand::thread_rng();
        let mut app_key_bytes = Vec::new();
        for _ in 0..16 {
            app_key_bytes.push(rng.gen())
        }
        hex::encode(app_key_bytes)
    }
}

#[derive(Clone, Deserialize, Serialize, Debug)]
pub struct GetDevice {
    app_eui: String,
    app_key: String,
    dev_eui: String,
}

impl GetDevice {
    pub fn from_user_input(app_eui: String, app_key: String, dev_eui: String) -> Result<GetDevice> {
        let app_eui_decoded = hex::decode(app_eui.clone())?;
        if app_eui_decoded.len() != 8 {
            return Err(Error::InvalidAppEui.into());
        }

        let app_key_decoded = hex::decode(app_key.clone())?;
        if app_key_decoded.len() != 16 {
            return Err(Error::InvalidAppKey.into());
        }

        let dev_eui_decoded = hex::decode(dev_eui.clone())?;
        if dev_eui_decoded.len() != 8 {
            return Err(Error::InvalidDevEui.into());
        }

        Ok(GetDevice {
            app_eui,
            app_key,
            dev_eui,
        })
    }

    pub fn app_eui(&self) -> &String {
        &self.app_eui
    }

    pub fn dev_eui(&self) -> &String {
        &self.dev_eui
    }

    pub fn app_key(&self) -> &String {
        &self.app_key
    }
}

#[derive(Clone, Deserialize, Serialize, Debug)]
pub struct NewDevice {
    app_eui: String,
    app_key: String,
    dev_eui: String,
    name: String,
}

impl NewDevice {
    pub fn from_user_input(
        app_eui: String,
        app_key: String,
        dev_eui: String,
        name: String,
    ) -> Result<NewDevice> {
        let app_eui_decoded = hex::decode(app_eui.clone())?;
        if app_eui_decoded.len() != 8 {
            return Err(Error::InvalidAppEui.into());
        }

        let app_key_decoded = hex::decode(app_key.clone())?;
        if app_key_decoded.len() != 16 {
            return Err(Error::InvalidAppKey.into());
        }

        let dev_eui_decoded = hex::decode(dev_eui.clone())?;
        if dev_eui_decoded.len() != 8 {
            return Err(Error::InvalidDevEui.into());
        }

        Ok(NewDevice {
            app_eui,
            app_key,
            dev_eui,
            name,
        })
    }

    pub fn app_eui(&self) -> &String {
        &self.app_eui
    }

    pub fn app_key(&self) -> &String {
        &self.app_key
    }

    pub fn dev_eui(&self) -> &String {
        &self.dev_eui
    }
}

impl NewLabel {
    pub fn from_string(string: &str) -> NewLabel {
        NewLabel {
            name: string.to_owned(),
        }
    }
}

#[derive(Clone, Deserialize, Serialize, Debug)]
pub struct NewLabel {
    name: String,
}

#[derive(Clone, Deserialize, Serialize, Debug)]
pub struct Label {
    id: String,
    name: String,
}

impl Label {
    pub fn id(&self) -> &String {
        &self.id
    }
    pub fn name(&self) -> &String {
        &self.name
    }
}

#[derive(Clone, Deserialize, Serialize, Debug)]
pub struct DeviceLabel {
    label: String,
}

impl DeviceLabel {
    pub fn from_uuid(label: String) -> Result<DeviceLabel> {
        validate_uuid_input(&label)?;
        Ok(DeviceLabel { label })
    }

    pub fn get_uuid(&self) -> &String {
        &self.label
    }
}

/// Throws an error if UUID isn't properly input
pub fn validate_uuid_input(id: &str) -> Result {
    if let Err(err) = uuid::Uuid::parse_str(id) {
        println!("{} [input: {}]", err, id);
        return Err(Error::InvalidUuid.into());
    }
    Ok(())
}
