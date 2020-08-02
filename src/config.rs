#![allow(dead_code)]

use crate::hacks::aimbot::AimbotConfig;
use memlib::system;
use crate::hacks::closest_player::ClosestPlayerConfig;

// The config struct passed in the main hack loop
pub struct Config {
    pub aimbot_config: AimbotConfig,
    pub cloest_player_config: ClosestPlayerConfig,
    pub friends: Vec<String>    // Will consider friends teammates
}

impl Config {
    // Creates a config with the default settings
    pub fn default() -> Self {
        Self {
            aimbot_config: AimbotConfig::default(),
            cloest_player_config: ClosestPlayerConfig::default(),
            friends: vec![]
        }
    }
}

#[derive(Clone, Eq, PartialEq)]
pub enum Keybind {
    AlwaysOn,
    WhilePressed(Vec<i32>), // list of keys
    WhileNotPressed(Vec<i32>),
}

// TODO: Add caching
impl Keybind {
    // Returns true if the keystate is enabled
    pub fn get_state(&self) -> bool {
        match self {
            Keybind::AlwaysOn => true,
            Keybind::WhilePressed(keys) => {
                for &key in keys {
                    if system::get_key_state(key) {
                        return true
                    }
                }
                false
            },
            Keybind::WhileNotPressed(keys) => {
                for &key in keys {
                    if system::get_key_state(key) {
                        return false
                    }
                }
                true
            },
        }
    }
}