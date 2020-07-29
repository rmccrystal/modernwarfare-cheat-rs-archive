use memlib::memory::*;
use log::*;
use super::encryption;
use super::structs;
use super::offsets;
use super::player::Player;
use memlib::math::{Angles2, Vector3};

/// A struct containing information and methods for the game.
/// This struct will be passed into the main hack loop and used accordingly.
pub struct Game {
    pub base_address: Address,
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

        Ok(Self {
            base_address,
        })
    }

    pub fn get_players(&self) -> Option<Vec<Player>> {
        let char_array = self.get_character_array()?;
        let players = char_array.iter().map(|c| Player::new(&self, c)).collect();
        Some(players)
    }

    pub fn get_camera_position(&self) -> Vector3 {
        let camera_addr: Address = read_memory(self.base_address + offsets::CAMERA_POINTER_OFFSET);
        read_memory(camera_addr + offsets::CAMERA_OFFSET)
    }

    pub fn get_camera_angles(&self) -> Angles2 {
        unimplemented!();
        read_memory(self.base_address + 0x8E5CC)
    }
}

// Internal functions
impl Game {
    pub fn get_character_array(&self) -> Option<Vec<structs::character_info>> {
        // Get the character array base address
        let character_array_address: Address = self.get_character_array_base()?;
        if character_array_address == 0 {
            warn!("Read an invalid character_array_address");
            return None;
        }

        // Read the character array
        let mut character_array: Vec<structs::character_info> =
            read_array::<_>(character_array_address, 155);

        // for (index, character) in character_array.iter().enumerate() {
        //     let pos = character.get_position();
        //     if pos.is_zero() {
        //         continue;
        //     }
        //     debug!("{}, {:?}", character_array_address + index as u64 * 0x3A60, pos);
        // }

        // Remove all the invalid characters
        character_array.retain(|character| {
            character.info_valid == 1 && !character.get_position().is_zero()
        });

        // If there are no characters, return None
        if character_array.is_empty() {
            trace!("Character array was empty");
            return None;
        }

        trace!("Found {} characters", character_array.len());

        // Return the character array
        Some(character_array)
    }

    pub fn get_name_struct(&self, character_id: u32) -> structs::name_t {
        let name_array_base: Address = read_memory(self.base_address + offsets::NAME_ARRAY);

        let character_id = character_id as u64;
        read_memory(name_array_base + offsets::NAME_LIST_OFFSET + ((character_id + character_id * 8) << 4))
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
        let client_info = self.get_client_info_base()?;
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