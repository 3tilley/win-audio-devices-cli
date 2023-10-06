use crate::models::{DeviceRep, Direction, Role, State};
use std::collections::{HashMap, HashSet};
use log::{info, trace};
use wasapi::{Device, DeviceCollection, get_default_device, get_default_device_for_role};
use crate::specs::{DefaultAudioDeviceSwitch, DisplayInstructions};
use crate::view_models::{DisplayDevicesDetails, DisplayDevicesDetailsInput, DisplayDevicesDetailsOutput, InputDeviceDetails};

pub fn switch_default_device(instructions: DefaultAudioDeviceSwitch) -> Result<State, ()> {
    // 1. Get the current device
    let current = get_default_device(&instructions.direction.into()).unwrap();

    // 2. Get the list of devices, filtering if necessary
    let device_list = DeviceCollection::new(&instructions.direction.into()).unwrap();
    Err(())
}


pub fn output_devices(instructions: DisplayInstructions) -> Result<(), ()> {
    let output = read_devices(instructions)?;
    println!("{}", serde_json::to_string_pretty(&output).unwrap());
    Ok(())
}

pub fn read_devices(instructions: DisplayInstructions) -> Result<DisplayDevicesDetails, ()> {
    // 1. Which directions are required
    let (input_direction, output_direction) =
        match instructions.direction {
            Some(Direction::Input) => (Some(Direction::Input), None),
            Some(Direction::Output) => (None, Some(Direction::Output)),
            None => (Some(Direction::Input), Some(Direction::Output)),
        };
    let input_devices = input_direction.map(|d| {
        let device_collection = DeviceCollection::new(&(d.into())).unwrap();
        info!("{} devices found", device_collection.get_nbr_devices().unwrap());
        let dev_2 = device_collection
            .into_iter()
            .filter_map(|device| device.ok())
            .collect::<Vec<_>>();
        trace!("{} devices found without errors", dev_2.len());
        let devices = filter_devices(dev_2, &instructions.states, &instructions.device_list);
        let defaults = [wasapi::Role::Console , wasapi::Role::Multimedia, wasapi::Role::Communications]
            .into_iter()
            .map(|role| {
                let device = get_default_device_for_role(&d.into(), &role).unwrap();
                (role.into(), device.into())
            })
            .collect::<HashMap<Role, _>>();
        DisplayDevicesDetailsInput::new(devices.into_iter().map(|d| d.into()).collect::<Vec<_>>(), defaults)
    });

    let output_devices = output_direction.map(|d| {
        let device_collection= DeviceCollection::new(&(d.into())).unwrap();
        info!("{} devices found", device_collection.get_nbr_devices().unwrap());
        let dev_2 = device_collection
            .into_iter()
            .filter_map(|device| device.ok())
            .collect::<Vec<_>>();
        trace!("{} devices found without errors", dev_2.len());
        let devices = filter_devices(dev_2, &instructions.states, &instructions.device_list);
        let defaults = [wasapi::Role::Console , wasapi::Role::Multimedia, wasapi::Role::Communications]
            .into_iter()
            .map(|role| {
                let device = get_default_device_for_role(&d.into(), &role).unwrap();
                (role.into(), device.into())
            })
            .collect::<HashMap<Role, _>>();
        DisplayDevicesDetailsOutput::new(devices.into_iter().map(|d| d.into()).collect::<Vec<_>>(), defaults)
    });

    Ok(DisplayDevicesDetails {
        input: input_devices,
        output: output_devices,
    })
}

pub fn filter_devices(
    mut device_list: Vec<Device>,
    states: &Option<HashSet<State>>,
    device_matcher: &Option<Vec<DeviceRep>>,
) -> Vec<Device> {
    let mut devices: Vec<Device> = Vec::new();
    let mut device_i = Vec::new();
    match device_matcher {
        Some(matcher) => {
            for dev_match in matcher {
                for (i, device) in device_list.iter().enumerate() {
                    if dev_match.check_match(&device) {
                        device_i.push(i);
                    }
                }
            }
            //devices.append(device_list.drain(device_i).collect());
            info!("Removing {} devices for not matching the criteria", &device_i.len());
            device_i.iter().map(|i| device_list.remove(*i)).for_each(|d| devices.push(d));
        }
        None => {
            info!("No devices filtered by name");
            for device in device_list {
                devices.push(device);
            }
        }
    }

    match states {
        Some(states) => {
            devices = devices
                .into_iter()
                .filter_map(|device| {
                    let state = device.get_state().unwrap();
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
// pub fn filter_devices<'a>(
//     device_list: &[&'a Device],
//     //device_list: impl IntoIterator<Item = &'a Device>,
//     states: Option<HashSet<State>>,
//     device_matcher: Option<Vec<DeviceRep>>,
// ) -> Vec<&'a Device> {
//     let mut devices = Vec::new();
//     match device_matcher {
//         Some(matcher) => {
//             for dev_match in matcher {
//                 for &device in device_list {
//                     if dev_match.check_match(&device) {
//                         devices.push(device);
//                     }
//                 }
//             }
//         }
//         None => {
//             for &device in device_list {
//                 devices.push(device);
//             }
//         }
//     }
//
//     match states {
//         Some(states) => {
//             devices = devices
//                 .into_iter()
//                 .filter_map(|device| {
//                     let state = device.get_state().unwrap();
//                     if states.contains(&state.into()) {
//                         Some(device)
//                     } else {
//                         None
//                     }
//                 })
//                 .collect::<Vec<_>>()
//         }
//         None => (),
//     }
//
//     devices
// }
