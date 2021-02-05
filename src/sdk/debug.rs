use log::*;
use super::{Game, structs, offsets};
use std::sync::atomic::Ordering;
use memlib::memory::Address;
use crate::sdk::Player;

/// Starts an interactive scan of the local player. Used to search for character_info offsets
pub fn scan_local_player<T>(game: &Game, fast_scan: bool)
    where T: PartialEq + PartialOrd + Sized + Clone + std::str::FromStr + std::fmt::Display {
    let local_addr = game.get_local_player().unwrap().base_address;
    debug!("Found local player address: 0x{:X}", local_addr);
    memlib::memory::new_interactive_scan::<T>((local_addr, local_addr + offsets::character_info::SIZE as u64), fast_scan);
}

pub fn find_charinfo_size(game: &mut Game) -> usize {
    game.update_addresses();
    let char_array = game.addresses.character_array_base.unwrap();

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