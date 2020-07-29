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
    trace!("Found encrypted client_info address: 0x{:X}", encrypted_address);

    // Get last_key
    let last_key = get_last_key(game_base_address, offsets::client_info::REVERSED_ADDRESS, offsets::client_info::DISPLACEMENT)
        .ok_or_else(|| "Could not get last_key for decrypting the client_info base address")?;

    // Get not_peb
    let not_peb = get_not_peb();
    trace!("not_peb: 0x{:X}", not_peb);

    let mut decrypted_address = Wrapping(encrypted_address);
    let mut last_key          = Wrapping(last_key);
    let mut not_peb           = Wrapping(not_peb);


    let mut rax = Wrapping(encrypted_address + game_base_address);
    let mut r8 = Wrapping(game_base_address + 0x0FE4F);
    r8 += not_peb;
    r8 ^= rax;
    r8 ^= Wrapping(0x66567F4AFAD0293C);
    r8 *= Wrapping(0x3B4CCB1171E7D0CF);
    rax = r8;
    rax = rax >> 0x22;
    rax ^= r8;
    decrypted_address = last_key * rax;

    trace!("Found decrypted client_info address: 0x{:X}", decrypted_address.0);

    Ok(decrypted_address.0)
}

pub fn get_client_base_address(game_base_address: Address, client_info_address: Address) -> Result<Address> {
    let encrypted_address = read_memory(client_info_address + offsets::client_base::BASE_OFFSET);
    if encrypted_address == 0 {
        return Err("Could not find the encrypted client_info_base address".into());
    }
    trace!("Found encrypted client_info_base address: 0x{:X}", encrypted_address);

    let last_key = get_last_key(game_base_address, offsets::client_base::BASE_REVERSED_ADDR, offsets::client_base::BASE_DISPLACEMENT)
        .ok_or_else(|| "Could not get last_key for decrypting client_info_base")?;

    let not_peb = get_not_peb();

    let mut encrypted_address   = Wrapping(encrypted_address);
    let last_key                = Wrapping(last_key);
    let not_peb                 = Wrapping(not_peb);
    let game_base_address       = Wrapping(game_base_address);

    // Actual decryption

    let mut rcx = game_base_address + Wrapping(0x635339F1);
    rcx -= not_peb;
    encrypted_address ^= rcx;
    rcx = game_base_address;
    encrypted_address -= not_peb;
    encrypted_address ^= rcx;
    encrypted_address -= not_peb;
    rcx = encrypted_address;
    let rax = game_base_address + Wrapping(0x64B8);
    rcx += rax;
    encrypted_address = rcx * last_key;
    encrypted_address *= Wrapping(0xC6BBD73FBC54BCAF);
    encrypted_address ^= encrypted_address >> 0x25;
    encrypted_address += not_peb;

    trace!("Found decrypted client_info_base address: 0x{:X}", encrypted_address.0);

    Ok(encrypted_address.0)
}

pub fn get_bone_base_address(game_base_address: Address) -> Result<Address> {
    let encrypted_address = read_memory(game_base_address + offsets::bones::ENCRYPTED_PTR);
    if encrypted_address == 0 {
        return Err("Could not find the encrypted bone_base address".into());
    }
    trace!("Found encrypted bone_base address: 0x{:X}", encrypted_address);

    let last_key = get_last_key_byteswap(game_base_address, offsets::bones::REVERSED_ADDRESS, offsets::bones::DISPLACEMENT)
        .ok_or_else(|| "Could not get last_key for decrypting base_address")?;

    let not_peb = get_not_peb();

    let mut encrypted_address   = Wrapping(encrypted_address);
    let last_key                = Wrapping(last_key);
    let not_peb                 = Wrapping(not_peb);
    let game_base_address       = Wrapping(game_base_address);

    let mut rax = game_base_address;
    rax += rax;
    rax -= not_peb;
    rax += Wrapping(0xD9FB3C86C9E51DC0);
    encrypted_address += rax;
    encrypted_address *= last_key;
    encrypted_address += not_peb;
    encrypted_address *= Wrapping(0xE596C1B3DBFF68A9);
    encrypted_address ^= (encrypted_address >> 0x17);
    encrypted_address ^= (encrypted_address >> 0x2E);
    encrypted_address *= Wrapping(0x6F1B2E22A20C49A7);

    trace!("Found decrypted bone_base address: 0x{:X}", encrypted_address.0);

    Ok(encrypted_address.0)
}

fn get_not_peb() -> u64 {
    !get_process_info().peb_base_address
}

fn get_last_key(game_base_address: Address, reversed_address_offset: Address, displacement_offset: Address) -> Option<Address> {
    let reversed_address: Address = read_memory(game_base_address + reversed_address_offset);
    let last_key = read_memory(!reversed_address + displacement_offset);

    if last_key == 0 {
        return None;
    }

    Some(last_key)
}

fn get_last_key_byteswap(game_base_address: Address, reversed_address_offset: Address, displacement_offset: Address) -> Option<Address> {
    let reversed_address: Address = read_memory(game_base_address + reversed_address_offset);
    let last_key = read_memory(u64::from_be(reversed_address) + displacement_offset);

    if last_key == 0 {
        return None;
    }

    Some(last_key)
}