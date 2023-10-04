use crate::models::{DeviceRep, Direction, Role, State};
use std::collections::{HashMap, HashSet};
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
        let dev_2 = device_collection
            .into_iter()
            .filter_map(|device| device.ok())
            .collect::<Vec<_>>();
        let devices = filter_devices(dev_2, &instructions.states, &instructions.device_list);
        let defaults = [Role::Console, Role::Multimedia, Role::Communications]
            .iter()
            .map(|role| {
                let device = get_default_device_for_role(&d.into(), role).unwrap();
                (role.clone(), device.into())
            })
            .collect::<HashMap<_, _>>();
        DisplayDevicesDetailsInput::new(devices.into_iter().map(|d| d.into()).collect::<Vec<_>>(), defaults)
    });

    let output_devices = output_direction.map(|d| {
        let device_collection= DeviceCollection::new(&(d.into())).unwrap();
        let dev_2 = device_collection
            .into_iter()
            .filter_map(|device| device.ok())
            .collect::<Vec<_>>();
        let devices = filter_devices(dev_2, &instructions.states, &instructions.device_list);
        DisplayDevicesDetailsOutput::new(devices.into_iter().map(|d| d.into()).collect::<Vec<_>>())
    });

    // // 2. Get the default device
    // let default = get_default_device(&instructions.direction.into()).unwrap();
    //
    // println!("Found the following {:?} devices:", instructions.direction);
    // for device in &device_list {
    //     let state = &device.get_state_enum().unwrap();
    //     if device.get_id().unwrap() == default.get_id().unwrap() {
    //         println!("*** {:?}. State: {:?} ***", device.get_friendlyname().unwrap(), state);
    //     } else {
    //         println!("{:?}. State: {:?}", device.get_friendlyname().unwrap(), state);
    //     }
    // }
    //
    // Ok(())
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
            device_i.iter().map(|i| device_list.remove(*i)).for_each(|d| devices.push(d));
        }
        None => {
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
