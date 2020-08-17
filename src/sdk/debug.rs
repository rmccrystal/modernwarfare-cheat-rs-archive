use log::*;
use super::{Game, structs, offsets};

/// Starts an interactive scan of the local player. Used to search for character_info offsets
pub fn scan_local_player<T>(game: &Game, fast_scan: bool)
    where T: PartialEq + PartialOrd + Sized + Clone + std::str::FromStr + std::fmt::Display {
    let id = game.get_local_index().unwrap();
    debug!("Found local player id: {}", id);
    let local_addr = game.get_character_array_base().unwrap() + id as u64 * std::mem::size_of::<structs::character_info>() as u64;
    debug!("Found local player address: 0x{:X}", local_addr);
    memlib::memory::new_interactive_scan::<T>((local_addr, local_addr + offsets::client_base::SIZE as u64), fast_scan);
}