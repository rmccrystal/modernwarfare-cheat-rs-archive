use memlib::math::{Angles2, Vector3};
use memlib::memory::{Address, read_memory, try_read_memory};
use log::*;

use crate::sdk::{globals, offsets, Player, structs};
use crate::sdk;

pub fn get_players() -> Option<Vec<Player>> {
    if !in_game() {
        return None;
    }
    let char_array = globals::CHAR_ARRAY_BASE.get()?;

    // Read the character array
    let player_addresses: Vec<Address> = {
        let mut addresses = Vec::new();
        for i in 0..155 {
            addresses.push(char_array + (i * offsets::character_info::SIZE) as Address)
        };
        addresses
    };

    let players: Vec<_> = player_addresses.
        iter()
        .enumerate()
        .map(|(idx, &addr)| Player::new(addr, idx as i32)
            .map_err(|e| {
                trace!("Invalidated player: {}", e);
                e
            })
        )
        .filter(|player| player.is_ok())
        .map(|player| player.unwrap())
        .collect();

    Some(players)
}

pub fn get_camera_position() -> Option<Vector3> {
    let camera_addr: Address = read_memory(globals::GAME_BASE_ADDRESS.get() + offsets::CAMERA_POINTER);
    if camera_addr == 0 {
        return None;
    }
    let pos: Vector3 = read_memory(camera_addr + offsets::CAMERA_OFFSET);
    if pos.is_zero() {
        return None;
    }
    Some(pos)
}

pub fn get_camera_angles() -> Option<Angles2> {
    let camera_addr: Address = try_read_memory(globals::GAME_BASE_ADDRESS.get() + offsets::CAMERA_POINTER).ok()?;
    let angles: Angles2 = try_read_memory(camera_addr + offsets::CAMERA_OFFSET + 12).ok()?;
    if angles.is_zero() {
        return None;
    }
    Some(angles)
}

// Gets the local player by finding the closest player to the camera
pub fn find_local_index(players: &[Player], camera_pos: &Vector3) -> Option<i32> {
    if players.len() == 0 {
        return None;
    }

    players.iter()
        .map(|player| {
            let distance = sdk::units_to_m((camera_pos - player.origin).length());
            (player, distance)
        })
        .filter(|(_, distance)| *distance < 5.0)
        .min_by_key(|&(_, distance)| (distance * 1000.0) as i32)
        .map(|(player, _)| player.id)
}

fn in_game() -> bool {
    return true;
    // read_memory::<i32>(self.base_address + offsets::GAMEMODE) > 1
}

pub fn get_name_struct(character_id: u32) -> structs::Name {
    let name_array_base: Address = read_memory(globals::GAME_BASE_ADDRESS.get() + offsets::NAME_ARRAY);

    let character_id = character_id as u64;
    // let base = name_array_base + offsets::NAME_LIST_OFFSET + ((character_id + character_id * 8) << 4);
    let base = name_array_base + offsets::NAME_LIST_OFFSET + (character_id * 0xD0);
    read_memory(base)
}

pub fn get_local_index() -> Option<i32> {
    let ptr: Address = try_read_memory(globals::CLIENT_INFO.get()? + offsets::LOCAL_INDEX_POINTER).ok()?;
    try_read_memory(ptr + offsets::LOCAL_INDEX_OFFSET).ok()
}

pub fn get_local_player() -> Option<Player> {
    get_player_by_index(get_local_index()?)
}

pub fn get_player_by_index(index: i32) -> Option<Player> {
    let char_array = globals::CHAR_ARRAY_BASE.get().expect("Char array was None while calling get_player_by_index");
    Player::new(
        char_array + index as u64 * offsets::character_info::SIZE as u64,
        index
    ).ok()
}