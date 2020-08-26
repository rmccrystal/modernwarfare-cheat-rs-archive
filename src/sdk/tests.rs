#![cfg(test)]

use memlib::logger::MinimalLogger;
use log::LevelFilter;
use log::*;
use memlib::memory;
use super::encryption;
use crate::sdk::*;

lazy_static::lazy_static! {
    static ref GAME: Game = {
        // Initialize the logger
        let _ = MinimalLogger::init(LevelFilter::Trace);

        // Create a handle to the game
        let handle = memory::Handle::new(crate::PROCESS_NAME).expect("Failed to create a handle to MW");

        Game::new(handle).unwrap()
    };
}

#[test]
fn decrypt_client_info() {
    let base_address = GAME.base_address;
    let _client_info = encryption::get_client_info_address(base_address).unwrap();
}

#[test]
fn decrypt_client_base() {
    let base_address = GAME.base_address;

    let client_info = encryption::get_client_info_address(base_address).unwrap();
    let _client_base = encryption::get_client_base_address(base_address, client_info).unwrap();
}

#[test]
fn decrypt_bone_base() {
    let base_address = GAME.base_address;

    let _bone_base = encryption::get_bone_base_address(base_address).unwrap();
}

// must be in game
#[test]
fn players() {
    let players = GAME.get_players();

    let players = players.expect("No players were found");

    assert!(players.len() > 0);

    info!("Players: {:?}", players);
}

// Tests if the character_info_t size matches the one in the offsets
#[test]
fn character_info_t_size() {
    assert_eq!(std::mem::size_of::<structs::character_info>(), offsets::client_base::SIZE);
}

#[test]
fn camera() {
    GAME.get_camera_position().unwrap();
    GAME.get_camera_angles().unwrap();
}

#[test]
fn get_local_player() {
    assert_eq!(GAME.get_local_player().unwrap().name, "draven");
}

#[test]
fn character_names() {
    trace!("{:?}", GAME.get_players().unwrap());
    for player in &GAME.get_players().unwrap() {
        trace!("Found player {:?}", player);
        if player.name != "" {
            return
        }
    }

    panic!("No names found")
}

#[test]
fn get_bone_pos() {
    GAME.get_local_player().unwrap().get_bone_position(&GAME, bone::Bone::Head).unwrap();
}

#[test]
fn get_refdef() {
    encryption::get_refdef_pointer(GAME.base_address).unwrap();
}