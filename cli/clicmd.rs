use structopt::StructOpt;

#[derive(StructOpt, Debug)]
pub enum DeviceCmd {
    /// List all your organization's devices
    List {
        #[structopt(short, long)]
        oneline: bool,
    },
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
    GetById {
        id: String,
    },
    /// Delete a device
    /// by the UUID
    DeleteById {
        id: String,
    },
    /// Create a device
    /// by providing app_eui, app_key, dev_eui and name
    Create {
        app_eui: String,
        app_key: String,
        dev_eui: String,
        name: String,
    },
    /// Create a device by app_eui and name prefix
    /// DevEui & AppKey are randomly generated
    /// Last four characters of DevEui are appended
    CreateByAppEui {
        app_eui: String,
        name: String,
    },
    // Add a label to the device
    // by providing device_uuid and label_uuid
    AddLabel {
        device: String,
        label: String,
    },
    // Remove a label from device
    // by providing device_uuid and label_uuid
    RemoveLabel {
        device: String,
        label: String,
    },
}

#[derive(StructOpt, Debug)]
pub enum LabelCmd {
    /// List all your organization's labels
    List,
    /// Delete a label by id
    DeleteById { id: String },
    /// Create a device
    /// by providing a label name
    Create { name: String },
}

#[derive(StructOpt, Debug)]
pub enum TtnCmd {
    /// Imports devices from your TTN Account
    /// (requires ttnctl access code at https://account.thethingsnetwork.org/)
    Import,
}
