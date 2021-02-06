use memlib::memory::*;
use anyhow::*;
use log::*;
use std::num::Wrapping;

use super::offsets;
use crate::sdk::structs::{RefDef};

#[cxx::bridge]
mod ffi {
    extern "Rust" {
        fn read_qword(address: u64) -> u64;
    }

    extern "C++" {
        include!("C:\\users\\draven\\git\\mwc\\src\\sdk\\encryption.h");

        pub fn decrypt_client_info(encrypted_address: u64, game_base_address: u64, last_key: u64, peb: u64) -> u64;
        pub fn decrypt_client_base(encrypted_address: u64, game_base_address: u64, last_key: u64, peb: u64) -> u64;
        pub fn decrypt_bone_base(encrypted_address: u64, game_base_address: u64, last_key: u64, peb: u64) -> u64;
    }
}

fn read_qword(address: u64) -> u64 {
    memlib::memory::try_read_memory(address).expect("Error reading memory in cpp encryption")
}

#[repr(C)]
#[derive(Debug, Default)]
pub struct RefdefKeyStruct {
    pub ref0: u32,
    pub ref1: u32,
    pub ref2: u32,
}

pub fn get_refdef_pointer(game_base_address: Address) -> Result<Pointer<RefDef>> {
    let crypt: RefdefKeyStruct = try_read_memory(game_base_address + offsets::REFDEF as u64)?;

    if crypt.ref0 == 0 && crypt.ref1 == 0 && crypt.ref2 == 0 {
        bail!("Read 0 for refdef key struct");
    }
    trace!("Found refdef_key_struct: {:?}", crypt);

    let lower: Wrapping<u64> = (Wrapping(crypt.ref0 as u64) ^ Wrapping(crypt.ref2 as u64 ^ (game_base_address + offsets::REFDEF as u64)) * Wrapping((crypt.ref2 as u64 ^ (game_base_address + offsets::REFDEF as u64)) + 2));
    let upper: Wrapping<u64> = (Wrapping(crypt.ref1 as u64) ^ Wrapping(crypt.ref2 as u64 ^ (game_base_address + offsets::REFDEF as u64 + 0x4)) * Wrapping((crypt.ref2 as u64 ^ (game_base_address + offsets::REFDEF as u64 + 0x4)) + 2));
    let ref_def_key = (upper.0 as u32 as u64) << 32 | (lower.0 as u32 as u64);

    if read_bytes(ref_def_key, 1).is_err() {
        bail!("Refdef not valid");
    }
    let ref_def_pointer: Pointer<RefDef> = Pointer::new(ref_def_key);

    trace!("ref_def_key: 0x{:X}", ref_def_key);
    trace!("ref_def_pointer.read() = {:?}", ref_def_pointer.read());
    if ref_def_pointer.read().height == 0 {
        bail!("Read an invalid refdef_t struct");
    }

    Ok(ref_def_pointer)
}

pub fn get_client_info_address(game_base_address: Address) -> Result<Address> {
    // Get the encrypted base address
    let encrypted_address: Address = read_memory(game_base_address + offsets::client_info::ENCRYPTED_PTR);
    if encrypted_address == 0 {
        bail!("Could not find encrypted base address for client_info");
    }
    trace!("Found encrypted client_info address: 0x{:X}", encrypted_address);

    // Get last_key
    let last_key = get_last_key_byteswap(game_base_address, offsets::client_info::REVERSED_ADDRESS, offsets::client_info::DISPLACEMENT)?;

    trace!("Found client_info last_key: 0x{:X}", last_key);

    let decrypted_address = unsafe { ffi::decrypt_client_info(encrypted_address, game_base_address, last_key, get_peb()) };

    if decrypted_address > 0xFFFFFFFFFFFFFF {
        trace!("Invalidated client_info because the address was too large: 0x{:X}", decrypted_address);
        bail!("Address was too large");
    }

    if read_bytes(decrypted_address, 1).is_err() {
        bail!("Decrypted client info address was invalid: {:X}", decrypted_address);
    }

    trace!("Found decrypted client_info address: 0x{:X}", decrypted_address);

    Ok(decrypted_address)
}

pub fn get_client_base_address(game_base_address: Address, client_info_address: Address) -> Result<Address> {
    trace!("client_info_address: 0x{:X}", client_info_address);

    let encrypted_address = read_memory(client_info_address + offsets::client_base::BASE_OFFSET);
    if encrypted_address == 0 {
        bail!("Could not find the encrypted client_info_base address");
    }
    trace!("Found encrypted client_info_base address: 0x{:X}", encrypted_address);

    // let last_key = get_last_key_byteswap(game_base_address, offsets::client_base::BASE_REVERSED_ADDR, offsets::client_base::BASE_DISPLACEMENT)?;
    // we don't need it, the cpp interop reads mem
    let last_key = 0;

    let decrypted_address = unsafe { ffi::decrypt_client_base(encrypted_address, game_base_address, last_key, get_peb() )};

    if read_bytes(decrypted_address, 1).is_err() {
        bail!("Decrypted client base was invalid: {:X}", decrypted_address);
    }

    trace!("Found decrypted client_info_base address: 0x{:X}", decrypted_address);

    Ok(decrypted_address)
}

pub fn get_bone_base_address(game_base_address: Address) -> Result<Address> {
    let encrypted_address = try_read_memory(game_base_address + offsets::bones::ENCRYPTED_PTR)?;
    if encrypted_address == 0 {
        bail!("Could not find the encrypted bone_base address");
    }
    trace!("Found encrypted bone_base address: 0x{:X}", encrypted_address);

    let last_key = get_last_key(game_base_address, offsets::bones::REVERSED_ADDRESS, offsets::bones::DISPLACEMENT)?;

    let decrypted_address = unsafe { ffi::decrypt_bone_base(encrypted_address, game_base_address, last_key, get_peb() )};

    trace!("Found decrypted bone_base address: 0x{:X}", decrypted_address);
    Ok(decrypted_address)
}

fn get_peb() -> u64 {
    let peb = get_process_info().peb_base_address;
    trace!("peb: {:#X}", peb);
    peb
}

fn get_last_key(game_base_address: Address, reversed_address_offset: Address, displacement_offset: Address) -> Result<Address> {
    let reversed_address: Address = try_read_memory(game_base_address + reversed_address_offset)?;
    let last_key = try_read_memory(!reversed_address + displacement_offset)?;

    if last_key == 0 {
        bail!("last_key was 0");
    }

    Ok(last_key)
}

fn get_last_key_byteswap(game_base_address: Address, reversed_address_offset: Address, displacement_offset: Address) -> Result<Address> {
    let reversed_address: Address = try_read_memory(game_base_address + reversed_address_offset).unwrap();
    let last_key = try_read_memory(u64::from_be(reversed_address) + displacement_offset).unwrap();

    if last_key == 0 {
        bail!("last_key was 0");
    }

    Ok(last_key)
}
