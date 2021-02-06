#![allow(dead_code)]

use crate::hacks::aimbot::AimbotConfig;
use crate::hacks::closest_player::ClosestPlayerConfig;
use crate::hacks::esp::EspConfig;
use memlib::winutil::is_key_down;
use serde::{Serialize, Deserialize};
use std::io::{Read, Write, BufReader, BufWriter};
use std::fs::{File, OpenOptions};
use log::*;

// The config struct passed in the main hack loop
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, imgui_ext::Gui)]
pub struct Config {
    #[imgui(text_wrap("Aimbot"),
            separator(),
            nested)]
    pub aimbot_config: AimbotConfig,

    #[imgui(text_wrap("ESP"),
            separator(),
            nested)]
    pub esp_config: EspConfig,

    #[imgui(text_wrap("Util"),
            separator(),
            nested)]
    pub cloest_player_config: ClosestPlayerConfig,

    #[imgui(checkbox(label = "No Recoil"))]
    pub no_recoil_enabled: bool,
    pub friends: Vec<String>    // Will consider friends teammates
}

impl Default for Config {
    fn default() -> Self {
        Self {
            aimbot_config: AimbotConfig::default(),
            cloest_player_config: ClosestPlayerConfig::default(),
            esp_config: EspConfig::default(),
            no_recoil_enabled: false,
            friends: vec![]
        }
    }
}

impl Config {
    fn get_config_loc() -> String {
        "config.json".to_owned()
    }

    /// Loads the config from a file or returns None
    pub fn load() -> Option<Self> {
        let file = File::open(Self::get_config_loc()).ok()?;
        let reader = BufReader::new(file);

        match serde_json::from_reader(reader) {
            Ok(cfg) => Some(cfg),
            Err(e) => {
                error!("Error reading config file: {}", e);
                None
            }
        }
    }

    pub fn save(&self) {
        let file = OpenOptions::new()
            .write(true)
            .create(true)
            .open(Self::get_config_loc())
            .unwrap();

        let writer = BufWriter::new(file);

        serde_json::to_writer(writer, &self).unwrap()
    }
}

#[derive(Clone, Eq, PartialEq, Debug, Serialize, Deserialize)]
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
                    if is_key_down(key) {
                        return true
                    }
                }
                false
            },
            Keybind::WhileNotPressed(keys) => {
                for &key in keys {
                    if is_key_down(key) {
                        return false
                    }
                }
                true
            },
        }
    }
}