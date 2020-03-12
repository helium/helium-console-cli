[![Build Status](https://travis-ci.com/helium/helium-console-cli.svg?token=35YrBmyVB8LNrXzjrRop&branch=master)](https://travis-ci.com/helium/helium-console-cli)

# Helium Console CLI

Download a compiled release [here](https://github.com/helium/helium-console-cli/releases).

## Building on Windows
**Build Tools for Visual Studio 2019**  
Download Build Tools only [here](https://visualstudio.microsoft.com/thank-you-downloading-visual-studio/?sku=BuildTools&rel=16) or Download Visual Studio 2019 with Build Tools [here](https://visualstudio.microsoft.com/thank-you-downloading-visual-studio/?sku=Community&rel=16)  
When installer is complete, select C++ Build Tools from the tile menu, use default options, and install. 

## Setup 
First time you use the CLI, you will need to provide an API key. To create an account key, go to your [profile](https://console.helium.com/profile) on Helium Console. From the top right corner, click: Account->Profile.

From there, you may generate a key with a specific name and role. The key will only display once.

The first time you run the CLI, it will prompt you for this key. It will save the key in a local file: `.helium-console-config.toml`

## Usage

### Features

You can view the most current commands, features, and documentation by accessing the help menus. For example: `helium-console-cli --help` or `helium-console-cli device --help`.

Current high level features are:
* create and delete devices records, using (app_eui, app_key, dev_eui) or UUID
* list all device records
* create and delete labels by UUID
* create and delete device labels, by using (device_uuid, label_uuid)
* import devices from The Things Network (TTN)

### Examples

Let's create a device:

```
helium-console-cli device create 000A000100000046 CB67C92DD5898D07872224202DED7E76 DEADBEEF23F23454 foo
```

You'll get a response with the full record of this new device:

```
Device {
    app_eui: "000A000100000046",
    app_key: "CB67C92DD5898D07872224202DED7E76",
    dev_eui: "DEADBEEF23F23454",
    id: "03f5967a-7c79-42a2-bd5f-6e51015e73e3",
    name: "foo",
    organization_id: "07273bc4-4bc9-44ec-b4d5-ad320f162e15",
    oui: 1,
}
```

You can view this device again by either (app_eui, app_key, dev_eui) or by id/uuid. For example:

```
helium-console-cli device get 000A000100000046 CB67C92DD5898D07872224202DED7E76 DEADBEEF23F23454
```
or

```
helium-console-cli device get-by-id 03f5967a-7c79-42a2-bd5f-6e51015e73e3
```

These commands will return the same record as above.

Similarly, you may delete the device:

```
helium-console-cli device delete 000A000100000046 CB67C92DD5898D07872224202DED7E76 DEADBEEF23F23454
```
or

```
helium-console-cli device delete-by-id 03f5967a-7c79-42a2-bd5f-6e51015e73e3
```

Finally, at any time, you may list all your devices:

```
helium-console-cli device list
```

Which will return an array of all your devices:

```
[
    Device {
        app_eui: "000A000100000046",
        app_key: "CB67C92DD5898D07872224202DED7E76",
        dev_eui: "DEADBEEF23F23454",
        id: "03f5967a-7c79-42a2-bd5f-6e51015e73e3",
        name: "foo",
        organization_id: "07273bc4-4bc9-44ec-b4d5-ad320f162e15",
        oui: 1,
    }
]
```
### TTN Import

To start an import session, use the ttn import command:

```
helium-console-cli ttn import
```

You will be prompted for a ttnctl access code, which you can generate by clicking on `ttnctl access code` [here](https://account.thethingsnetwork.org/). This single-use access code is valid for 5 minutes. During this time the CLI may use the code to request an OAuth2 token which expires after 60 minutes.

The CLI's prompts will help you:
* select to import from a single application or all applications (maximum 10, otherwise the OAuth2 token is "used up")
* you may import all the devices at once or you may approve device import one by one.
* you may use the TTN App ID as an automatic label for every device or you may approve device labelling one by one

The import process tolerate attempts to re-import or re-label the same device. As such, you may re-run the script to label devices with the TTN App ID even if you already imported the devices during a previous session.