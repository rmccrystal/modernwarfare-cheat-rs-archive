#![cfg(test)]

use memlib::logger::MinimalLogger;
use std::sync::{Mutex, MutexGuard, Once};
use log::LevelFilter;
use log::*;
use memlib::memory;
use super::encryption;
use crate::sdk::*;
use std::borrow::Borrow;
use memlib::memory::Handle;
use crate::sdk::internal::{get_players, get_camera_angles, get_camera_position, get_local_index, get_player_by_index};
use bone::Bone;
use crate::sdk::globals::update_addresses;

static INIT: Once = Once::new();

pub fn init() {
    INIT.call_once(|| {
        MinimalLogger::init(LevelFilter::Trace);
        let handle = Handle::new(crate::PROCESS_NAME).unwrap();
        super::init(handle).unwrap();
        update_addresses();
        assert!(globals::CLIENT_INFO.get().is_some());
    })
}

// must be in game
#[test]
fn players() {
    init();

    let players = get_players().expect("players returned None");
    assert!(!players.is_empty());

    info!("Players: {:?}", players);
}

#[test]
fn camera() {
    init();

    get_camera_angles().unwrap();
    get_camera_position().unwrap();
}

#[test]
fn get_local_player() {
    init();

    let p = internal::get_local_player().unwrap();
    assert!(p.name == "draven");
}

#[test]
fn character_names() {
    init();

    let players = get_players().unwrap();
    for player in players {
        trace!("Found player {:?}", player);
        if !player.name.is_empty() {
            return;
        }
    }

    panic!("No names found")
}

#[test]
fn get_bone_pos() {
    init();

    let local_player = internal::get_local_player().unwrap();
    let bone_pos = local_player.get_bone_position(Bone::Head).unwrap();
    assert!(units_to_m((bone_pos - local_player.origin).length()) < 5.0);
}