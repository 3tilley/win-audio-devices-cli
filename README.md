# win-audio-devices-cli

> Manage your audio interfaces from the command line

:cartwheeling: Alpha software, expect bugs and instability :cartwheeling:

## Installation

⛔ Awaiting PR in upstream crate ⛔

The command below will install a bin called `wadc` in your `~/.cargo/bin` directory.

`cargo install win-audio-devices-cli`

## Usage

`wadc --json` will print a JSON representation of all audio devices.
This is currently the only command.

## Scope

The goal is to provide the kind of functionality that normally requires a lot
of mouseclicking straight from the terminal. This can then be easily bound to
hotkeys or used in scripts.

For obvious reasons, in its current state this won't work outside Windows, but
if it's useful and a common set of interfaces can be found, it could be

Goals:
- [x] Output system info in JSON
- [] Pretty print info to terminal
- [] Set default devices
- [] Set default devices interactively
- [] Play sounds to quickly identify devices
- [] Mute or unmute microphones

Unlikely to be implemented:
* Change volume
* Mute speakers
* Media controls

For the simple reason that these are already easily accessible from the keyboard
