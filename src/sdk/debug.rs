use std::sync::atomic::Ordering;

use log::*;
use memlib::memory::Address;

use crate::sdk::globals::{CHAR_ARRAY_BASE, CLIENT_INFO, update_addresses_interval};
use crate::sdk::internal::{get_local_index, get_local_player};
use crate::sdk::Player;

use super::{offsets, structs};

/// Starts an interactive scan of the local player. Used to search for character_info offsets
pub fn scan_local_player<T>(fast_scan: bool)
    where T: PartialEq + PartialOrd + Sized + Clone + std::str::FromStr + std::fmt::Display {
    let local_index = get_local_index().unwrap() as u64;
    let local_addr = CLIENT_INFO.get().unwrap() + local_index * offsets::character_info::SIZE as u64;
    debug!("Found local player address: 0x{:X}", local_addr);
    memlib::memory::new_interactive_scan::<T>((local_addr, local_addr + offsets::character_info::SIZE as u64), fast_scan);
}

/*
pub fn find_charinfo_size() -> usize {
    let char_array = CHAR_ARRAY_BASE.get().unwrap();

    let mut size = 0x1538;
    let valid_player_count = 50;

    loop {
        size += 1;
        info!("size: {:X}", size);

        // Read the character array
        let player_addresses: Vec<Address> = {
            let mut addresses = Vec::new();
            for i in 0..155 {
                // addresses.push(char_array + (i * offsets::character_info::SIZE) as Address)
                addresses.push(char_array + (i * size) as Address)
            };
            addresses
        };

        let players: Vec<_> = player_addresses.
            iter()
            .enumerate()
            .map(|(idx, &addr)| Player::new(&game, addr, idx as i32)
                .map_err(|e| {
                    trace!("Invalidated player: {}", e);
                    e
                })
            )
            .filter(|player| player.is_ok())
            .map(|player| player.unwrap())
            .collect();

        info!("Found {} players", players.len());

        if players.len() >= valid_player_count {
            return size;
        }
    }
}
*/