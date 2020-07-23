use memlib::memory::*;
use log::*;
use super::encryption;
use super::structs;

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

        // for character in &character_array {
        //     print!("{}, ", character.info_valid);
        //     let pos = character.position.read();
        //     if pos.is_zero() {
        //         continue;
        //     }
        //     debug!("{:?}", pos);
        // }


        // Remove all the invalid characters
        character_array.retain(|character| {
            character.info_valid == 1 && !character.get_origin().is_zero()
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
}

// Addresses
impl Game {
    fn get_client_info_base(&self) -> Option<Address> {
        let client_info = encryption::get_client_info_address(self.base_address);
        if let Err(error) = &client_info {
            warn!("Failed to find client_info address with error: {}", error)
        }
        client_info.ok()
    }

    fn get_character_array_base(&self) -> Option<Address> {
        let client_info = self.get_client_info_base()?;
        let client_base = encryption::get_client_base_address(self.base_address, client_info);
        if let Err(error) = &client_base {
            warn!("Failed to find client_base address with error: {}", error);
        }
        client_base.ok()
    }
}