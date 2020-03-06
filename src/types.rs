use super::Result;

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

#[derive(Clone, Deserialize, Serialize, Debug)]
pub struct GetDevice {
    app_eui: String,
    app_key: String,
    dev_eui: String,
}
impl GetDevice {
    pub fn from_user_input(
        app_eui: String,
        app_key: String,
        dev_eui: String,
    ) -> Result<GetDevice> {

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

    pub fn app_key(&self) -> &String {
        &self.app_key
    }

    pub fn dev_eui(&self) -> &String {
        &self.dev_eui
    }
}

#[derive(Clone, Deserialize, Serialize, Debug)]
struct NewDevice {
    app_eui: String,
    app_key: String,
    dev_eui: String,
    name: String,
 }

#[derive(Clone, Deserialize, Serialize, Debug)]
pub struct NewDeviceRequest {
    device: NewDevice
}

impl NewDeviceRequest {
    pub fn from_user_input(
        app_eui: String,
        app_key: String,
        dev_eui: String,
        name: String,
    ) -> Result<NewDeviceRequest> {

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

        Ok(NewDeviceRequest { 
            device: NewDevice {
                app_eui,
                app_key,
                dev_eui,
                name,
            }
        })
    }
}

use std::error::Error as stdError;
use std::{fmt, str};

#[derive(Debug, Clone)]
pub enum Error {
    InvalidAppEui,
    InvalidAppKey,
    InvalidDevEui,
    InvalidApiKey
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::InvalidAppEui => {
                write!(f, "Invalid AppEui input. Must be 8 bytes represented in hex (\"0123456789ABCDEF\")")
            }
            Error::InvalidAppKey => {
                write!(f, "Invalid AppKey input. Must be 16 bytes represented in hex (\"0123456789ABCDEF0123456789ABCDEF\")")
            }
            Error::InvalidDevEui => {
                write!(f, "Invalid DevEui input. Must be 8 bytes represented in hex (\"0123456789ABCDEF\")")
            }
            Error::InvalidApiKey => {
                write!(f, "Invalid Api Key. Must be 32 bytes represented in base64")
            }
        }
    }
}

impl stdError for Error {
    fn description(&self) -> &str {
        match self {
            Error::InvalidAppEui => "Invalid AppEui input. Must be 8 bytes represented in hex (\"0123456789ABCDEF\")",
            Error::InvalidAppKey => "Invalid AppKey input. Must be 16 bytes represented in hex (\"0123456789ABCDEF0123456789ABCDEF\")",
            Error::InvalidDevEui => "Invalid DevEui input. Must be 8 bytes represented in hex (\"0123456789ABCDEF\")",
            Error::InvalidApiKey => "Invalid Api Key. Must be 32 bytes represented in base64",
        }
    }

    fn cause(&self) -> Option<&dyn stdError> {
        // Generic error, underlying cause isn't tracked.
        None
    }
}
