//! Specifications
//!
//! These are the instructions for how to process various commands. They are generally built from
//! the union of the command line args and anything read out of config
use std::collections::HashSet;
use crate::models::{DeviceRep, Direction, State};

pub struct DefaultAudioDeviceSwitch {
    pub device_list: Option<Vec<DeviceRep>>,
    pub direction: Direction,
    pub states: Option<HashSet<State>>,
    pub json: bool,
    pub dry_run: bool,
}

pub struct DisplayInstructions {
    pub device_list: Option<Vec<DeviceRep>>,
    pub direction: Option<Direction>,
    pub states: Option<HashSet<State>>,
    pub json: bool,
}
