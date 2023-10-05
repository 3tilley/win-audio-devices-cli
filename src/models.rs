use std::hash::{Hash, Hasher};
use std::ops::Deref;
use serde::{Deserialize, Serialize};
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

// Experiment with newtype
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Role(wasapi::Role);

// TODO: Review this or put the Hash up in the level above
impl Hash for Role {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        state.write_u8(self.0 as u8)
    }
}

impl Serialize for Role {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let role = match self.0 {
            wasapi::Role::Console => "Console",
            wasapi::Role::Multimedia => "Multimedia",
            wasapi::Role::Communications => "Communications",
        };
        serializer.serialize_str(role)
    }
}

impl<'de> Deserialize<'de> for Role {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where D: serde::Deserializer<'de> {
        let role = String::deserialize(deserializer)?;
        match role.as_str() {
            "Console" => Ok(Role(wasapi::Role::Console)),
            "Multimedia" => Ok(Role(wasapi::Role::Multimedia)),
            "Communications" => Ok(Role(wasapi::Role::Communications)),
            _ => Err(serde::de::Error::custom(format!(
                "Invalid role: {}",
                role
            ))),
        }
    }
}

impl Deref for Role {
    type Target = wasapi::Role;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

// It's either this or make the field pub crate. I think this is better?
impl From<wasapi::Role> for Role {
    fn from(role: wasapi::Role) -> Self {
        Role(role)
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