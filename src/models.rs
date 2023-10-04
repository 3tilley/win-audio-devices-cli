use serde::{Deserialize, Serialize};
use wasapi;
use wasapi::{Device, DeviceState};

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
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

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
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

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
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


// impl Serialize for DeviceState {
//     fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
//         let state = match self {
//             DeviceState::Active => "Active",
//             DeviceState::Disabled => "Disabled",
//             DeviceState::NotPresent => "NotPresent",
//             DeviceState::Unplugged => "Unplugged",
//         };
//         serializer.serialize_str(state)
//     }
// }
//
// impl Deserialize for DeviceState {
//     fn deserialize<D: serde::Deserializer>(deserializer: D) -> Result<Self, D::Error> {
//         let state = String::deserialize(deserializer)?;
//         match state.as_str() {
//             "Active" => Ok(DeviceState::Active),
//             "Disabled" => Ok(DeviceState::Disabled),
//             "NotPresent" => Ok(DeviceState::NotPresent),
//             "Unplugged" => Ok(DeviceState::Unplugged),
//             _ => Err(serde::de::Error::custom(format!(
//                 "Invalid device state: {}",
//                 state
//             ))),
//         }
//     }
// }