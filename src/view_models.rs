//! # View Models
//!
//! The structs in here are for rendering the output of commands, either with a pretty printer
//! or as JSON depending on how the command is invoked
use std::collections::{HashMap, HashSet};
use serde::{Deserialize, Serialize};
use crate::models::{Role, State};

// TODO: This module is super boilerplatey

#[derive(Serialize, Deserialize)]
pub struct DisplayDevicesDetails {
    pub input: Option<DisplayDevicesDetailsInput>,
    pub output: Option<DisplayDevicesDetailsOutput>,
}

// TODO: Give these some better names
#[derive(Serialize, Deserialize)]
pub struct InputDeviceDetails {
    pub id: String,
    pub short_name: String,
    pub long_name: String,
    pub adapter: String,
    pub state: State,
}

impl From<wasapi::Device> for InputDeviceDetails {
    // TODO: This is a copy of the implementation for OutputDeviceDetails.
    //       Refactor to avoid duplication.
    fn from(device: wasapi::Device) -> Self {
        let state = device.get_state().unwrap().into();
        let short_name = device.get_friendlyname().unwrap();
        let long_name = device.get_description().unwrap();
        InputDeviceDetails {
            id: device.get_id().unwrap(),
            short_name: long_name,
            long_name: short_name,
            adapter: device.get_interface_friendlyname().unwrap(),
            state,
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct DisplayDevicesDetailsInput {
    pub n: usize,
    pub defaults: HashMap<Role, InputDeviceDetails>,
    pub devices: Vec<InputDeviceDetails>,
}

impl DisplayDevicesDetailsInput {
    // Constructor takes vec of devices
    pub fn new(devices: Vec<InputDeviceDetails>, defaults: HashMap<Role, InputDeviceDetails>) -> Self {
        let n = devices.len();
        DisplayDevicesDetailsInput { n, devices, defaults }
    }
}
#[derive(Serialize, Deserialize)]
pub struct OutputDeviceDetails {
    pub id: String,
    pub short_name: String,
    pub long_name: String,
    pub adapter: String,
    pub state: State,
}

impl From<wasapi::Device> for OutputDeviceDetails {
    fn from(device: wasapi::Device) -> Self {
        let state = device.get_state().unwrap().into();
        let short_name = device.get_friendlyname().unwrap();
        let long_name = device.get_description().unwrap();
        OutputDeviceDetails {
            id: device.get_id().unwrap(),
            short_name: long_name,
            long_name: short_name,
            adapter: device.get_interface_friendlyname().unwrap(),
            state,
        }
    }
}


#[derive(Serialize, Deserialize)]
pub struct DisplayDevicesDetailsOutput {
    pub n: usize,
    pub defaults: HashMap<Role, OutputDeviceDetails>,
    pub devices: Vec<OutputDeviceDetails>,
}
impl DisplayDevicesDetailsOutput {
    // Constructor takes vec of devices
    pub fn new(devices: Vec<OutputDeviceDetails>, defaults: HashMap<Role, OutputDeviceDetails>) -> Self {
        let n = devices.len();
        DisplayDevicesDetailsOutput { n, devices, defaults }
    }
}
