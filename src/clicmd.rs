use structopt::StructOpt;

#[derive(StructOpt, Debug)]
enum LabelCmd {
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

#[derive(StructOpt, Debug)]
enum TtnCmd {
    /// Imports devices from your TTN Account
    /// (requires ttnctl access code at https://account.thethingsnetwork.org/)
    Import,
}

/// Interact with Helium API via CLI
#[derive(Debug, StructOpt)]
enum Cli {
    /// List, create, and delete devices
    Device {
        #[structopt(subcommand)]
        cmd: DeviceCmd,
    },
    /// Import devices from TTN to Helium
    Ttn {
        #[structopt(subcommand)]
        cmd: TtnCmd,
    },
}