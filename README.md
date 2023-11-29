# win-audio-devices-cli

> Manage your audio interfaces from the command line

:cartwheeling: Alpha software, expect bugs and instability :cartwheeling:

## Installation

The command below will install a bin called `wadc` in your `~/.cargo/bin` directory.

`cargo install win-audio-devices-cli`

Access the program by running `wadc` in your terminal. Have fun!

## Usage

### `wadc list`
Use this to get a list of all available devices.
Output syntax is as follows:
```json 
{
    //input devices list
    "input": {
        // number of devices
        "n": 2,
        //default device
        "defaults": {
            "Console": {
                "id": "deviceID",
                "short_name": "Short name of device",
                "long_name": "Short Name (Adapter Name)",
                "adapter": "Adapter Name",
                "state": "Current state"
            },
            "Communications": {
                "id": "deviceID",
                "short_name": "Short name of device",
                "long_name": "Short Name (Adapter Name)",
                "adapter": "Adapter Name",
                "state": "Current state"
            },
            "Multimedia": {
                "id": "deviceID",
                "short_name": "Short name of device",
                "long_name": "Short Name (Adapter Name)",
                "adapter": "Adapter Name",
                "state": "Current state"
            }
        },
        // Array of list of all devices, including the default
        "devices": [
            {
                "id": "deviceID",
                "short_name": "Short name of device",
                "long_name": "Short Name (Adapter Name)",
                "adapter": "Adapter Name",
                "state": "Current state"
            }
        ]
    },
    // list of all output devices
    "output": {
        // Number of output devices
        "n": 2,
        // List of default output devices according to service
        "defaults": {
            "Multimedia": {
                "id": "deviceID",
                "short_name": "Short name of device",
                "long_name": "Short Name (Adapter Name)",
                "adapter": "Adapter Name",
                "state": "Current state"
            },
            "Communications": {
                "id": "deviceID",
                "short_name": "Short name of device",
                "long_name": "Short Name (Adapter Name)",
                "adapter": "Adapter Name",
                "state": "Current state"
            },
            "Console": {
                "id": "deviceID",
                "short_name": "Short name of device",
                "long_name": "Short Name (Adapter Name)",
                "adapter": "Adapter Name",
                "state": "Current state"
            }
        },
        // Array of list of all devices, including the default
        "devices": [
            {
                "id": "deviceID",
                "short_name": "Short name of device",
                "long_name": "Short Name (Adapter Name)",
                "adapter": "Adapter Name",
                "state": "Current state"
            }
        ]
    }
}
```

### `wadc switch`
Use this to switch the default device (Not stable)

## Scope

The goal is to provide the kind of functionality that normally requires a lot
of mouseclicking straight from the terminal. This can then be easily bound to
hotkeys or used in scripts.

For obvious reasons, in its current state this won't work outside Windows, but
if it's useful and a common set of interfaces can be found, it could be

Goals:
- [x] Output system info in JSON
- [ ] Pretty print info to terminal
- [ ] Set default devices
- [ ] Set default devices interactively
- [ ] Play sounds to quickly identify devices
- [ ] Mute or unmute microphones

Since they are already trivially accessible from the keyboard
the following are unikely to be implemented:
* Change volume
* Mute speakers
* Media controls
