[![Build Status](https://travis-ci.com/helium/helium-console-cli.svg?token=35YrBmyVB8LNrXzjrRop&branch=master)](https://travis-ci.com/helium/helium-console-cli)

# Helium Console CLI

Download a compiled release [here](https://github.com/helium/helium-console-cli/releases).

## Usage

### Setup 
First time you use the device, you will need to provide an API key. To create an account key, go to your [profile](https://console.helium.com/profile) on Helium Console. From the top right corner, click: Account->Profile.

From there, you may generate a key with a specific name and role. The key will only display once.

The first time you run the CLI, it will prompt you for this key. It will save the key in a local file: '.helium-console-config.toml'

### Features

Currently, the Console CLI only has `device` focused features. You can view the most current commands by doing accessing the help menu: `helium-console-cli device --help`:

```
USAGE:
    helium-console-cli device <SUBCOMMAND>

SUBCOMMANDS:
    create          Create a device by providing app_eui, app_key, dev_eui and name
    delete          Delete a device by providing app_eui, app_key, and dev_eui
    delete-by-id    Delete a device by the UUID
    get             Get the full record of your device by providing app_eui, app_key, and dev_eui
    get-by-id       Get the full record of your device by the UUID
    help            Prints this message or the help of the given subcommand(s)
    list            List all your account devices
```

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

Finally, at any time, you may list all your device:

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