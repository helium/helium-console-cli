#[macro_use]
extern crate serde_derive;
extern crate serde_json;

pub mod client;
pub mod errors;

pub use errors::*;

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
struct NewDevice {
    app_eui: String,
    app_key: String,
    dev_eui: String,
    name: String,
}

#[derive(Clone, Deserialize, Serialize, Debug)]
pub struct NewDeviceRequest {
    device: NewDevice,
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
            },
        })
    }

    pub fn app_eui(&self) -> &String {
        &self.device.app_eui
    }

    pub fn app_key(&self) -> &String {
        &self.device.app_key
    }

    pub fn dev_eui(&self) -> &String {
        &self.device.dev_eui
    }
}

#[derive(Clone, Deserialize, Serialize, Debug)]
pub struct NewLabelRequest {
    label: LabelRequest,
}

impl NewLabelRequest {
    pub fn from_string(string: &String) -> NewLabelRequest {
        NewLabelRequest {
            label: LabelRequest {
                name: string.clone(),
            },
        }
    }
}

#[derive(Clone, Deserialize, Serialize, Debug)]
pub struct LabelRequest {
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
    device: String,
    label: String,
}

impl DeviceLabel {
    pub fn from_uuids(device: String, label: String) -> Result<DeviceLabel> {
        validate_uuid_input(&device)?;
        validate_uuid_input(&label)?;
        Ok(DeviceLabel { device, label })
    }
}

/// Throws an error if UUID isn't properly input
pub fn validate_uuid_input(id: &String) -> Result {
    if let Err(err) = uuid::Uuid::parse_str(id.as_str()) {
        println!("{} [input: {}]", err, id);
        return Err(Error::InvalidUuid.into());
    }
    Ok(())
}
