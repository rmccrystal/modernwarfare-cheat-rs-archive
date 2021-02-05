#![cfg(test)]

use memlib::logger::MinimalLogger;
use std::sync::{Mutex, MutexGuard, Once};
use log::LevelFilter;
use log::*;
use memlib::memory;
use super::encryption;
use crate::sdk::*;
use std::borrow::Borrow;

lazy_static::lazy_static! {
    static ref GAME: Game = {
        // Initialize the logger
        let _ = MinimalLogger::init(LevelFilter::Trace);

        let handle = memory::Handle::from_boxed_interface(Box::new(
            memory::handle_interfaces::driver_handle::DriverProcessHandle::attach(crate::PROCESS_NAME).expect("Failed to create a handle to MW")
        ));

        let mut game = Game::new(handle).unwrap();

        game.update_addresses();

        game
    };
}

macro_rules! get_game {
    () => { &GAME };
}

#[test]
fn decrypt_client_info() {
    let game = get_game!();
    let base_address = game.addresses.game_base_address;
    let _client_info = encryption::get_client_info_address(base_address).unwrap();
}

#[test]
fn decrypt_client_base() {
    let game = get_game!();

    let base_address = game.addresses.game_base_address;

    let client_info = encryption::get_client_info_address(base_address).unwrap();
    let _client_base = encryption::get_client_base_address(base_address, client_info).unwrap();
}

#[test]
fn decrypt_bone_base() {
    let game = get_game!();

    let base_address = game.addresses.game_base_address;

    let _bone_base = encryption::get_bone_base_address(base_address).unwrap();
}

// must be in game
#[test]
fn players() {
    let game = get_game!();

    let players = game.get_players();

    let players = players.expect("No players were found");

    assert!(!players.is_empty());

    info!("Players: {:?}", players);
}

#[test]
fn camera() {
    let game = get_game!();

    dbg!(game.get_camera_position().unwrap());
    dbg!(game.get_camera_angles().unwrap());
}

#[test]
fn get_local_player() {
    let game = get_game!();

    assert_eq!(game.get_local_player().unwrap().name, "draven");
}

#[test]
fn character_names() {
    let game = get_game!();

    trace!("{:?}", game.get_players().unwrap());
    for player in &game.get_players().unwrap() {
        trace!("Found player {:?}", player);
        if !player.name.is_empty() {
            return;
        }
    }

    panic!("No names found")
}

#[test]
fn get_bone_pos() {
    let game = get_game!();

    game.get_local_player().unwrap().get_bone_position(&game, bone::Bone::Head).unwrap();
}

#[test]
fn get_refdef() {
    let game = get_game!();

    encryption::get_refdef_pointer(game.addresses.game_base_address).unwrap();
}