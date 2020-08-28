use log::*;
use super::{Game, structs, offsets};

/// Starts an interactive scan of the local player. Used to search for character_info offsets
pub fn scan_local_player<T>(game: &Game, fast_scan: bool)
    where T: PartialEq + PartialOrd + Sized + Clone + std::str::FromStr + std::fmt::Display {
    let local_addr = game.get_local_player().unwrap().base_address;
    debug!("Found local player address: 0x{:X}", local_addr);
    memlib::memory::new_interactive_scan::<T>((local_addr, local_addr + offsets::client_base::SIZE as u64), fast_scan);
}