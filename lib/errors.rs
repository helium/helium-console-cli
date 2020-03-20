use std::{fmt, str};

#[derive(Debug, Clone)]
pub enum Error {
    InvalidAppEui,
    InvalidAppKey,
    InvalidDevEui,
    InvalidApiKey,
    InvalidUuid,
    NewDevice422,
    NewDeviceApi,
    NewLabel422,
    NewLabelApi,
    NewDeviceLabelApi,
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
            Error::InvalidUuid => {
                write!(f, "Invalid UUID input. Expected in hyphenated form \"00000000-0000-0000-0000-000000000000\"")
            }
            Error::NewDevice422 => {
                write!(f, "Failed Creating Device! Device with identical credentials already exists")
            }
            Error::NewDeviceApi => {
                write!(f, "Failed Creating Device! Unknown server error")
            }
            Error::NewLabel422 => {
                write!(f, "Failed Creating Label! Label with same name already exists under organization")
            }
            Error::NewLabelApi => {
                write!(f, "Failed Creating Label! Unknown server error")
            }
            Error::NewDeviceLabelApi => {
                write!(f, "Failed Creating Device Label! Unknown server error")
            }


        }
    }
}

impl ::std::error::Error for Error {
    fn description(&self) -> &str {
        match self {
            Error::InvalidAppEui => "Invalid AppEui input. Must be 8 bytes represented in hex (\"0123456789ABCDEF\")",
            Error::InvalidAppKey => "Invalid AppKey input. Must be 16 bytes represented in hex (\"0123456789ABCDEF0123456789ABCDEF\")",
            Error::InvalidDevEui => "Invalid DevEui input. Must be 8 bytes represented in hex (\"0123456789ABCDEF\")",
            Error::InvalidApiKey => "Invalid Api Key. Must be 32 bytes represented in base64",
            Error::InvalidUuid => "Invalid UUID input. Expected in hyphenated form \"00000000-0000-0000-0000-000000000000\"",
            Error::NewDevice422 => "Failed Creating Device! Device with identical credentials already exists",
            Error::NewDeviceApi => "Failed Creating Device! Unknown server error", 
            Error::NewLabel422 => "Failed Creating Label! Label with same name already exists under organization",
            Error::NewLabelApi => "Failed Creating Label! Unknown server error",
            Error::NewDeviceLabelApi => "Failed Creating Device Label! Unknown server error",
        }
    }
}
