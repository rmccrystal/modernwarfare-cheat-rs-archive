#![allow(dead_code)]

use memlib::memory::*;
use log::*;
use super::encryption;
use super::structs;
use super::offsets;
use super::player::Player;
use memlib::math::{Angles2, Vector3, Vector2};
use crate::sdk::structs::{refdef_t};
use crate::sdk::world_to_screen::world_to_screen;
use std::time::{Duration, Instant};

/// Contains information about a game. Only exists when in a game
#[derive(Clone, Debug)]
pub struct GameInfo {
    pub players: Vec<Player>,
    pub local_position: Vector3,
    pub local_view_angles: Angles2,
    pub local_player: Player,
}

/// A struct containing information and methods for the game.
/// This struct will be passed into the main hack loop and used accordingly.
#[derive(Clone, Debug)]
pub struct Game {
    pub base_address: Address,
    pub game_info: Option<GameInfo>,
    pub client_info_base: Option<Address>,
    pub character_array_base: Option<Address>,
    pub bone_base: Option<Address>,
    pub refdef: Option<Pointer<refdef_t>>,
    address_update_frequency: Duration,
    last_update: Instant
}

impl Game {
    /// Creates a new `Game` struct using a process handle
    pub fn new(handle: Handle) -> Result<Self> {
        // Set the global handle so we can use it anywhere
        set_global_handle(handle);

        // Get the base address or return an error
        let base_address = get_module(crate::PROCESS_NAME)
            .ok_or_else(|| format!("Error getting module {}", crate::PROCESS_NAME))?
            .base_address;

        let mut game = Self {
            base_address,
            client_info_base: None,
            character_array_base: None,
            bone_base: None,
            game_info: None,
            refdef: None,
            address_update_frequency: Duration::from_secs(1),
            last_update: Instant::now() - Duration::from_secs(1000),
        };

        game.update();
        game.update_addresses();

        Ok(game)
    }

    /// This function updates the game data. Should be ran every game tick
    pub fn update(&mut self) {
        if self.last_update + self.address_update_frequency < Instant::now() {
            self.update_addresses();
            self.last_update = Instant::now();
            self.refdef = encryption::get_refdef_pointer(self.base_address).ok()
        }

        self.game_info = self.get_game_info();
    }

    pub fn update_addresses(&mut self) {
        self.bone_base = self.get_bone_base();
        self.character_array_base = self.get_character_array_base();
        self.client_info_base = self.get_client_info_base();
    }

    pub fn get_game_info(&self) -> Option<GameInfo> {
        Some(GameInfo {
            local_view_angles: self.get_camera_angles()?,
            local_position: self.get_camera_position()?,
            local_player: self.get_local_player()?,
            players: self.get_players()?,
        })
    }

    pub fn get_players(&self) -> Option<Vec<Player>> {
        if !self.in_game() {
            return None;
        }
        let char_array = self.character_array_base?;

        // Read the character array
        let mut player_addresses: Vec<Address> = {
            let mut addresses = Vec::new();
            for i in 0..155 {
                addresses.push(char_array + (i * offsets::character_info::SIZE) as Address)
            };
            addresses
        };

        let players = player_addresses.
            iter()
            .map(|&addr| Player::new(&self, addr))
            .filter(|player| player.is_some())
            .map(|player| player.unwrap())
            .collect();

        Some(players)
    }

    pub fn get_player_by_id(&self, id: i32) -> Option<Player> {
        let player_base = self.character_array_base? + (id as u64 * offsets::character_info::SIZE as u64);
        Player::new(&self, player_base)
    }

    pub fn world_to_screen(&self, world_pos: &Vector3) -> Option<Vector2> {
        let refdef = encryption::get_refdef_pointer(self.base_address).ok()?.read();
        world_to_screen(
            &world_pos,
            self.get_camera_position()?,
            refdef.width,
            refdef.height,
            refdef.view.tan_half_fov,
            refdef.view.axis,
        )
    }
}

// Internal functions
impl Game {
    pub fn get_camera_position(&self) -> Option<Vector3> {
        let camera_addr: Address = read_memory(self.base_address + offsets::CAMERA_POINTER);
        let pos: Vector3 = read_memory(camera_addr + offsets::CAMERA_OFFSET);
        if pos.is_zero() {
            return None;
        }
        Some(pos)
    }

    pub fn get_camera_angles(&self) -> Option<Angles2> {
        let camera_addr: Address = read_memory(self.base_address + offsets::CAMERA_POINTER);
        let angles: Angles2 = read_memory(camera_addr + offsets::CAMERA_OFFSET + 12);
        if angles.is_zero() {
            return None;
        }
        Some(angles)
    }

    pub fn get_local_player(&self) -> Option<Player> {
        // return self.get_local_player_fallback();
        let local_index = self.get_local_index()?;
        trace!("Local index: {}", local_index);
        self.get_player_by_id(local_index)
    }

    // Gets the local player by finding the closest player to the camera
    fn get_local_player_fallback(&self) -> Option<Player> {
        let players = self.get_players()?;
        let camera_pos = self.get_camera_position()?;

        if players.len() == 0 {
            return None;
        }

        let mut closest_player: (&Player, f32) = (&players[0], 999999999.0);

        for player in &players {
            let head_pos = player.get_head_position(&self);
            let dist = (camera_pos - head_pos).length();
            if dist < closest_player.1 {
                closest_player.0 = player;
                closest_player.1 = dist;
                trace!("Found new closest player to camera: {}, {}m", player.name, units_to_m(dist));
            }
        }

        trace!("Found local player using fallback method: {}, {}m from camera", closest_player.0.name, closest_player.1);

        Some(closest_player.0.clone())
    }

    pub fn in_game(&self) -> bool {
        return true;
        // read_memory::<i32>(self.base_address + offsets::GAMEMODE) > 1
    }

    pub fn get_name_struct(&self, character_id: u32) -> structs::name_t {
        let name_array_base: Address = read_memory(self.base_address + offsets::NAME_ARRAY);

        let character_id = character_id as u64;
        // let base = name_array_base + offsets::NAME_LIST_OFFSET + ((character_id + character_id * 8) << 4);
        let base = name_array_base + offsets::NAME_LIST_OFFSET + (character_id * 0xD0);
        read_memory(base)
    }

    pub fn get_local_index(&self) -> Option<i32> {
        let ptr: Address = read_memory(self.client_info_base? + offsets::LOCAL_INDEX_POINTER);
        Some(read_memory(ptr + offsets::LOCAL_INDEX_OFFSET))
    }
}

// Addresses
impl Game {
    pub fn get_client_info_base(&self) -> Option<Address> {
        let client_info = encryption::get_client_info_address(self.base_address);
        if let Err(error) = &client_info {
            warn!("Failed to find client_info address with error: {}", error)
        }
        client_info.ok()
    }

    pub fn get_character_array_base(&self) -> Option<Address> {
        let client_info = self.client_info_base?;
        let client_base = encryption::get_client_base_address(self.base_address, client_info);
        if let Err(error) = &client_base {
            warn!("Failed to find client_base address with error: {}", error);
        }
        client_base.ok()
    }

    pub fn get_bone_base(&self) -> Option<Address> {
        let bone_base = encryption::get_bone_base_address(self.base_address);
        if let Err(error) = &bone_base {
            warn!("Failed to find bone_base address with error: {}", error)
        }
        bone_base.ok()
    }
}

// Converts units to in game meters
pub fn units_to_m(units: f32) -> f32 {
    units / 39.5
}