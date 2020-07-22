use memlib::memory::*;
use super::encryption;

/// A struct containing information and methods for the game.
/// This struct will be passed into the main hack loop and used accordingly.
pub struct Game {
    pub base_address: Address,
    client_info_base: Address,
    base_offset: Address
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

        let client_info_base = encryption::get_client_info_address(base_address)?;
        let base_offset = encryption::get_client_info_base_address(base_address)?;

        Ok(Self {
            base_address,
            client_info_base,
            base_offset
        })
    }
}