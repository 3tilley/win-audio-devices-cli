use std::collections::HashSet;
use wasapi;
use wasapi::{Device, DeviceState};

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum DeviceRep {
    PartialString(String),
    ExactString(String),
    DeviceId(String),
}

impl DeviceRep {
    pub fn check_match(&self, device: &Device) -> bool {
        match self {
            DeviceRep::PartialString(s) => {
                let name = device.get_friendlyname().unwrap().to_lowercase();
                name.contains(&s.to_lowercase())
            }
            DeviceRep::ExactString(s) => {
                let name = device.get_friendlyname().unwrap();
                name == *s
            }
            DeviceRep::DeviceId(s) => {
                let id = device.get_id().unwrap();
                id == *s
            }
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Direction {
    Input,
    Output,
}

impl From<wasapi::Direction> for Direction {
    fn from(direction: wasapi::Direction) -> Self {
        match direction {
            wasapi::Direction::Capture => Direction::Input,
            wasapi::Direction::Render => Direction::Output,
        }
    }
}

impl From<Direction> for wasapi::Direction {
    fn from(direction: Direction) -> Self {
        match direction {
            Direction::Input => wasapi::Direction::Capture,
            Direction::Output => wasapi::Direction::Render,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum State {
    Active,
    Disabled,
    NotPresent,
    Unplugged,
}

impl From<wasapi::DeviceState> for State {
    fn from(state: wasapi::DeviceState) -> Self {
        match state {
            wasapi::DeviceState::Active => State::Active,
            wasapi::DeviceState::Disabled => State::Disabled,
            wasapi::DeviceState::NotPresent => State::NotPresent,
            wasapi::DeviceState::Unplugged => State::Unplugged,
        }
    }
}

impl Into<wasapi::DeviceState> for State {
    fn into(self) -> DeviceState {
        match self {
            State::Active => DeviceState::Active,
            State::Disabled => DeviceState::Disabled,
            State::NotPresent => DeviceState::NotPresent,
            State::Unplugged => DeviceState::Unplugged,
        }
    }
}

pub struct DefaultAudioDeviceSwitch {
    pub device_list: Option<Vec<DeviceRep>>,
    pub direction: Direction,
    pub states: Option<HashSet<State>>,
    pub dry_run: bool,
}

pub struct DisplayInstructions {
    pub device_list: Option<Vec<DeviceRep>>,
    pub direction: Direction,
    pub states: Option<HashSet<State>>,
}
