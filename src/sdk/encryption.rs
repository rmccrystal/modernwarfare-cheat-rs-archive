use memlib::memory::*;
use log::*;
use std::num::Wrapping;

use super::offsets;

pub fn get_client_info_address(game_base_address: Address) -> Result<Address> {
    // Get the encrypted base address
    let encrypted_address: Address = read_memory(game_base_address + offsets::client_info::ENCRYPTED_PTR);
    if encrypted_address == 0 {
        return Err("Could not find encrypted base address for client_info".into());
    }
    debug!("Found encrypted client_info address: 0x{:X}", encrypted_address);

    // Get last_key
    let last_key = get_last_key(game_base_address, offsets::client_info::REVERSED_ADDRESS, offsets::client_info::DISPLACEMENT)
        .ok_or_else(|| "Could not get last_key for decrypting the client_info base address")?;

    // Get not_peb
    let not_peb = get_not_peb();
    trace!("not_peb: 0x{:X}", not_peb);

    let mut decrypted_address = Wrapping(encrypted_address);
    let mut last_key          = Wrapping(last_key);
    let mut not_peb           = Wrapping(not_peb);

    decrypted_address *= last_key;
    decrypted_address ^= (decrypted_address >> 0x1D);
    decrypted_address ^= (decrypted_address >> 0x3A);
    decrypted_address -= Wrapping(0x1505400C26E50FA6);
    decrypted_address ^= (decrypted_address >> 0x22);
    decrypted_address *= Wrapping(0xBF8507FACC9977DB);
    decrypted_address += not_peb;

    info!("Found decrypted client_info address: 0x{:X}", decrypted_address.0);

    // println!("{:?}", read_bytes(decrypted_address.0, 2000));

    Ok(decrypted_address.0)
}

pub fn get_client_info_base_address(game_base_address: Address) -> Result<Address> {
    let client_info_address = get_client_info_address(game_base_address)?;

    let encrypted_address = read_memory(client_info_address + offsets::client_info_base::BASE_OFFSET);

    if encrypted_address == 0 {
        return Err("Could not find the encrypted client_info_base address".into());
    }

    debug!("Found encrypted client_info_base address: 0x{:X}", encrypted_address);

    let last_key = get_last_key(game_base_address, offsets::client_info_base::BASE_REVERSED_ADDR, offsets::client_info_base::BASE_DISPLACEMENT)
        .ok_or_else(|| "Could not get last_key for decrypting base_address")?;

    let not_peb = get_not_peb();

    let mut encrypted_address   = Wrapping(encrypted_address);
    let last_key                = Wrapping(last_key);
    let not_peb                 = Wrapping(not_peb);

    encrypted_address ^= (encrypted_address >> 0x1C);
    encrypted_address ^= (encrypted_address >> 0x38);
    encrypted_address *= last_key;
    encrypted_address ^= Wrapping(0xC8755D9A588BA9BF);
    encrypted_address *= Wrapping(0xAEABFCF6626F055B);

    let mut rax = Wrapping(!(game_base_address + 0xB278));
    rax += not_peb;
    encrypted_address += rax;

    info!("Found decrypted client_info_base address: 0x{:X}", encrypted_address.0);

    Ok(encrypted_address.0)
}

fn get_not_peb() -> u64 {
    !get_process_info().peb_base_address
}

fn get_last_key(game_base_address: Address, reversed_address_offset: Address, displacement_offset: Address) -> Option<Address> {
    let reserved_address: Address = read_memory(game_base_address + reversed_address_offset);
    let last_key = read_memory(u64::from_be(reserved_address) + displacement_offset);

    if last_key == 0 {
        return None;
    }

    Some(last_key)
}
