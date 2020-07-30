#![cfg(test)]

use memlib::logger::MinimalLogger;
use log::LevelFilter;
use log::*;
use memlib::memory;
use memlib::memory::{set_global_handle, get_module, Address};
use super::encryption;
use crate::sdk::Game;

static mut BASE_ADDRSES: Option<Address> = None;

/// Inits the global handle and logging and returns the base address (for testing)
fn init() -> Result<Address, Box<dyn std::error::Error>> {
    // only init once
    unsafe {
        println!("{:?}", BASE_ADDRSES);
        if BASE_ADDRSES.is_some() {
            return Ok(BASE_ADDRSES.unwrap())
        }
    }

    // Initialize the logger
    MinimalLogger::init(LevelFilter::Trace);

    // Create a handle to the game
    let handle = memory::Handle::new(crate::PROCESS_NAME)?;
    set_global_handle(handle);

    // Get the base address or return an error
    let base_address = get_module(crate::PROCESS_NAME)
        .ok_or_else(|| format!("Error getting module {}", crate::PROCESS_NAME))?
        .base_address;

    println!("cache");
    unsafe { BASE_ADDRSES = Some(base_address); }

    Ok(base_address)
}

fn get_game() -> Result<Game, Box<dyn std::error::Error>> {
    // Initialize the logger
    MinimalLogger::init(LevelFilter::Trace);

    // Create a handle to the game
    let handle = memory::Handle::new(crate::PROCESS_NAME)?;

    Game::new(handle)
}

#[test]
fn test_decrypt_client_info() {
    let base_address = init().unwrap();
    let _client_info = encryption::get_client_info_address(base_address).unwrap();
}

#[test]
fn test_decrypt_client_base() {
    let base_address = init().unwrap();

    let client_info = encryption::get_client_info_address(base_address).unwrap();
    let _client_base = encryption::get_client_base_address(base_address, client_info).unwrap();
}

#[test]
fn test_decrypt_bone_base() {
    let base_address = init().unwrap();

    let _bone_base = encryption::get_bone_base_address(base_address).unwrap();
}

// must be in game
#[test]
fn test_players() {
    let game = get_game().unwrap();

    let players = game.get_players();

    players.expect("No players were found");

    info!("Players: {:?}", players.unwrap());
}