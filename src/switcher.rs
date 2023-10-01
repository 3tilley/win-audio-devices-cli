use crate::contract::{DefaultAudioDeviceSwitch, DeviceRep, DisplayInstructions, State};
use std::collections::HashSet;
use wasapi::{get_default_device, Device, DeviceCollection};

pub fn switch_default_device(instructions: DefaultAudioDeviceSwitch) -> Result<State, ()> {
    // 1. Get the current device
    let current = get_default_device(&instructions.direction.into()).unwrap();

    // 2. Get the list of devices, filtering if necessary
    let device_list = DeviceCollection::new(&instructions.direction.into()).unwrap();
    Err(())
}

pub fn display_devices(instructions: DisplayInstructions) -> Result<(), ()> {
    // 1. Get the list of devices, filtering if necessary
    let mut devices = DeviceCollection::new(&instructions.direction.into())
        .unwrap();
    let total = devices.get_nbr_devices().unwrap() as usize;
    log::debug!("Found {} devices", total);
    let dev_vec = devices.into_iter().filter_map(|device| device.ok()).collect::<Vec<_>>();
    let ref_vec = dev_vec.iter().map(|d| d).collect::<Vec<_>>();
    log::debug!("Errors unwrapping {} devices", total - dev_vec.len());
    let device_list = filter_devices(&ref_vec, instructions.states, instructions.device_list);

    // 2. Get the default device
    let default = get_default_device(&instructions.direction.into()).unwrap();

    println!("Found the following {:?} devices:", instructions.direction);
    for device in &device_list {
        let state = &device.get_state_enum().unwrap();
        if device.get_id().unwrap() == default.get_id().unwrap() {
            println!("*** {:?}. State: {:?} ***", device.get_friendlyname().unwrap(), state);
        } else {
            println!("{:?}. State: {:?}", device.get_friendlyname().unwrap(), state);
        }
    }

    Ok(())
}

pub fn filter_devices<'a>(
    device_list: &[&'a Device],
    //device_list: impl IntoIterator<Item = &'a Device>,
    states: Option<HashSet<State>>,
    device_matcher: Option<Vec<DeviceRep>>,
) -> Vec<&'a Device> {
    let mut devices = Vec::new();
    match device_matcher {
        Some(matcher) => {
            for dev_match in matcher {
                for &device in device_list {
                    if dev_match.check_match(&device) {
                        devices.push(device);
                    }
                }
            }
        }
        None => {
            for &device in device_list {
                devices.push(device);
            }
        }
    }

    match states {
        Some(states) => {
            devices = devices
                .into_iter()
                .filter_map(|device| {
                    let state = device.get_state_enum().unwrap();
                    if states.contains(&state.into()) {
                        Some(device)
                    } else {
                        None
                    }
                })
                .collect::<Vec<_>>()
        }
        None => (),
    }

    devices
}
