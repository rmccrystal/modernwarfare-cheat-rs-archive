use memlib::memory::*;
use log::*;

use super::offsets;
use super::*;

pub fn get_client_info_address(game: &Game) -> Result<Address> {
    // Get the encrypted base address
    let mut encrypted_client_info_address: Address = read_memory(game.base_address + offsets::client_info::ENCRYPTED_PTR);
    if encrypted_client_info_address == 0 {
        return Err("Could not find encrypted base address for client_info".into());
    }
    debug!("Found encrypted client_info address: 0x{:X}", encrypted_client_info_address);

    let last_key = get_last_key(&game, offsets::client_info::REVERSED_ADDRESS, offsets::client_info::DISPLACEMENT)
        .ok_or_else(|| "Could not get last_key for decrypting the client_info base address")?;
    let not_peb = get_not_peb(&game);

    let mut rdx = !(game.base_address + 0x472047FF);
    rdx ^= not_peb;
    rdx += not_peb;
    rdx += encrypted_client_info_address;
    rdx *= 0x84284B9805F27C2D;
    rdx ^= (rdx >> 0x8);
    rdx ^= (rdx >> 0x10);
    rdx ^= (rdx >> 0x20);
    encrypted_client_info_address = last_key;
    encrypted_client_info_address *= rdx;
    encrypted_client_info_address += 0x282638D0F2256416;

    Ok(encrypted_client_info_address)
}

pub fn get_base_address(game: &Game) -> Result<Address> {
    let client_info_address = get_client_info_address(&game)?;

    let mut encrypted_address = read_memory(client_info_address + offsets::base::BASE_OFFSET);

    let last_key = get_last_key(&game, offsets::base::BASE_REVERSED_ADDR, offsets::base::BASE_DISPLACEMENT)
        .ok_or_else(|| "Could not get last_key for decrypting base_address")?;

    let not_peb = get_not_peb();

    encrypted_address ^= (encrypted_address >> 0x1C);
    encrypted_address ^= (encrypted_address >> 0x38);
    encrypted_address *= last_key;
    encrypted_address ^= 0xC8755D9A588BA9BF;
    encrypted_address *= 0xAEABFCF6626F055B;

    let mut rax = !(game.base_address + 0xB278);
    rax += not_peb;
    encrypted_address += rax;

    Ok(encrypted_address)
}

fn get_not_peb() -> u64 {
    !get_process_info().peb_base_address
}

fn get_last_key(game: &Game, reversed_address_offset: Address, displacement_offset: Address) -> Option<Address> {
    let reserved_address: Address = read_memory(game.base_address + reversed_address_offset);
    let last_key = read_memory(u64::from_be(reserved_address) + displacement_offset);

    if last_key == 0 {
        return None;
    }

    Some(last_key)
}
