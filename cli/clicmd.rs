use structopt::StructOpt;

#[derive(StructOpt, Debug)]
pub enum DeviceCmd {
    /// List all your organization's devices
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
    /// Exchange OAuth access code for account token. Token may be scope to
    /// have permissions on up to 10 appliccations
    /// (requires ttnctl access code at https://account.thethingsnetwork.org/)
    GetAccountToken,
    /// Exchange OAuth access code for account token
    /// (requires ttnctl access code at https://account.thethingsnetwork.org/)
    GetApps { token: String },
    /// Exchange OAuth access code for account token
    /// (requires ttnctl access code at https://account.thethingsnetwork.org/)
    TokenExchange { token: String, apps: Vec<String> },
}
