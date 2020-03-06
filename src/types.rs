//use serde::{Deserialize, Serialize};

/*
"app_eui\":\"0000000100000046\",
\"app_key\":\"CB67C92DD5898D07872224202DED7E76\",
\"dev_eui\":\"1234567890110000\",
\"id\":\"5bdb0128-f0e9-4458-86cf-305faf8f48c2\",
\"name\":\"Okapi\",
\"organization_id\":
\"07273bc4-4bc9-44ec-b4d5-ad320f162e15\",
\"oui\":1}
*/
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
pub struct NewDevice {
    app_eui: String,
    app_key: String,
    dev_eui: String,
    name: String,
}


#[derive(Clone, Deserialize, Serialize, Debug)]
pub struct NewDeviceRequest {
    pub device: NewDevice
}

impl NewDevice {
    pub fn from_user_input(
        app_eui: String,
        app_key: String,
        dev_eui: String,
        name: String,
    ) -> Result<NewDevice> {
        println!("OI");

        let app_eui_decoded = hex::decode(app_eui.clone())?;
        if app_eui_decoded.len() != 8 {
            return Err(Error::InvalidAppEui.into())
        }

        let app_key_decoded = hex::decode(app_key.clone())?;
        if app_key_decoded.len() != 16 {
            return Err(Error::InvalidAppKey.into())
        }

        let dev_eui_decoded = hex::decode(dev_eui.clone())?;
        if dev_eui_decoded.len() != 8 {
            return Err(Error::InvalidDevEui.into())
        }

        Ok(NewDevice {
            app_eui,
            app_key,
            dev_eui,
            name,
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
        }
    }
}

impl stdError for Error {
    fn description(&self) -> &str {
        match self {
            Error::InvalidAppEui => "Invalid AppEui input. Must be 8 bytes represented in hex (\"0123456789ABCDEF\")",
            Error::InvalidAppKey => "Invalid AppKey input. Must be 16 bytes represented in hex (\"0123456789ABCDEF0123456789ABCDEF\")",
            Error::InvalidDevEui => "Invalid DevEui input. Must be 8 bytes represented in hex (\"0123456789ABCDEF\")",
        }
    }

    fn cause(&self) -> Option<&dyn stdError> {
        // Generic error, underlying cause isn't tracked.
        None
    }
}
