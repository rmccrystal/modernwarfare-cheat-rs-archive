#![allow(dead_code)]

use memlib::memory::{Address, Pointer, get_process_info};
use crate::sdk::structs::RefDef;
use memlib::util::{GlobalCell, InitCell};
use super::encryption;
use log::*;
use std::time::{Instant, Duration};

pub static PEB: InitCell<u64> = InitCell::new();

/// The base address of the process
pub static GAME_BASE_ADDRESS: InitCell<u64> = InitCell::new();

pub static CLIENT_INFO: GlobalCell<Option<u64>> = GlobalCell::new(None);
pub static CHAR_ARRAY_BASE: GlobalCell<Option<u64>> = GlobalCell::new(None);
pub static BONE_BASE: GlobalCell<Option<u64>> = GlobalCell::new(None);
pub static REFDEF: GlobalCell<Option<Pointer<RefDef>>> = GlobalCell::new(None);

pub static LAST_UPDATE: GlobalCell<Option<Instant>> = GlobalCell::new(None);

pub fn init(game_base_address: Address) {
    GAME_BASE_ADDRESS.init(game_base_address);
    PEB.init(get_process_info().peb_base_address);
}

/// Updates the addresses if LAST_UPDATE + interval < Instant::now()
pub fn update_addresses_interval(interval: Duration) {
    match LAST_UPDATE.get() {
        None => update_addresses(),
        Some(last_update) => {
            if last_update + interval < Instant::now() {
                update_addresses()
            }
        }
    }
}

pub fn update_addresses() {
    trace!("Updating addresses");

    LAST_UPDATE.set(Some(Instant::now()));

    let base = GAME_BASE_ADDRESS.get();

    let client_info = encryption::get_client_info_address(base);
    if let Err(e) = client_info {
        debug!("Could not get client_info: {}", e);

        CHAR_ARRAY_BASE.set(None);
        BONE_BASE.set(None);
        REFDEF.set(None);

        return;
    }
    let client_info = client_info.unwrap();
    CLIENT_INFO.set(Some(client_info));

    match encryption::get_client_base_address(base, client_info) {
        Ok(char_array) => CHAR_ARRAY_BASE.set(Some(char_array)),
        Err(e) => {
            debug!("Could not get character_array: {}", e);
            CHAR_ARRAY_BASE.set(None);
        }
    }
    match encryption::get_bone_base_address(base) {
        Ok(bone) => BONE_BASE.set(Some(bone)),
        Err(e) => {
            debug!("Could not get bone_base: {}", e);
            BONE_BASE.set(None);
        }
    }
    match encryption::get_refdef_pointer(base) {
        Ok(refdef) => REFDEF.set(Some(refdef)),
        Err(e) => {
            debug!("Could not get refdef: {}", e);
            REFDEF.set(None);
        }
    }
}

