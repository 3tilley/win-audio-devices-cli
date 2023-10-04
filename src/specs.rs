use std::collections::HashSet;
use crate::models::{DeviceRep, Direction, State};

pub struct DefaultAudioDeviceSwitch {
    pub device_list: Option<Vec<DeviceRep>>,
    pub direction: Direction,
    pub states: Option<HashSet<State>>,
    pub dry_run: bool,
}

pub struct DisplayInstructions {
    pub device_list: Option<Vec<DeviceRep>>,
    pub direction: Option<Direction>,
    pub states: Option<HashSet<State>>,
}
