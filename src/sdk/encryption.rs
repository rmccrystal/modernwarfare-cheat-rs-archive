use memlib::memory::*;
use anyhow::*;
use log::*;
use std::num::Wrapping;

use super::offsets;
use crate::sdk::structs::{RefDef};
use std::ptr::copy;
use std::ffi::c_void;
use super::globals;

#[cxx::bridge]
mod ffi {
    extern "Rust" {
        fn interop_read_bytes(address: u64, size: u64, buf: usize);
    }

    extern "C++" {
        include!("C:\\users\\draven\\git\\mwc\\src\\sdk\\encryption.h");

        pub fn decrypt_client_info(encrypted_address: u64, game_base_address: u64, last_key: u64, peb: u64) -> u64;
        pub fn decrypt_client_base(encrypted_address: u64, game_base_address: u64, last_key: u64, peb: u64) -> u64;
        pub fn decrypt_bone_base(encrypted_address: u64, game_base_address: u64, last_key: u64, peb: u64) -> u64;
        pub fn get_bone_index(index: u64, game_base_address: u64) -> u64;
    }
}

fn interop_read_bytes(address: u64, size: u64, buf: usize) {
    let buf = buf as *mut c_void;
    match memlib::memory::read_bytes(address, size as _) {
        Ok(value) => unsafe {
            copy(value.as_ptr(), buf as *mut u8, size as _);
        }
        Err(e) => {
            error!("Error reading memory in C++ encryption code: {:?}", e);
        }
    };
}

#[repr(C)]
#[derive(Debug, Default, Eq, PartialEq)]
pub struct RefdefKeyStruct {
    pub ref0: u32,
    pub ref1: u32,
    pub ref2: u32,
}

pub fn get_refdef_pointer(game_base_address: Address) -> Result<Pointer<RefDef>> {
    let crypt: RefdefKeyStruct = try_read_memory(game_base_address + offsets::REFDEF as u64)?;

    if crypt == RefdefKeyStruct::default() {
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

fn sanitize_decrypted_address(address: Address) -> Result<Address> {
    if address > 0xFFFFFFFFFFFFFF {
        bail!("Address was too large");
    }
    if read_bytes(address, 1).is_err() {
        bail!("Could not read address: {:X}", address);
    }

    Ok(address)
}

pub fn get_client_info_address(game_base_address: Address) -> Result<Address> {
    let encrypted_address: Address = read_memory(game_base_address + offsets::client_info::ENCRYPTED_PTR);
    if encrypted_address == 0 {
        bail!("Could not find encrypted base address for client_info");
    }
    trace!("Found encrypted client_info address: 0x{:X}", encrypted_address);

    let decrypted_address = unsafe { ffi::decrypt_client_info(encrypted_address, game_base_address, 0, globals::PEB.get()) };

    trace!("Got 0x{:X} for client_info", decrypted_address);
    sanitize_decrypted_address(decrypted_address)?;

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

    let decrypted_address = unsafe { ffi::decrypt_client_base(encrypted_address, game_base_address, 0, globals::PEB.get()) };

    trace!("Got 0x{:X} for client_base", decrypted_address);

    sanitize_decrypted_address(decrypted_address)?;

    trace!("Found decrypted client_info_base address: 0x{:X}", decrypted_address);

    Ok(decrypted_address)
}

pub fn get_bone_base_address(game_base_address: Address) -> Result<Address> {
    let encrypted_address = try_read_memory(game_base_address + offsets::bones::ENCRYPTED_PTR)?;
    if encrypted_address == 0 {
        bail!("Could not find the encrypted bone_base address");
    }
    trace!("Found encrypted bone_base address: 0x{:X}", encrypted_address);

    let decrypted_address = unsafe { ffi::decrypt_bone_base(encrypted_address, game_base_address, 0, globals::PEB.get()) };

    sanitize_decrypted_address(decrypted_address)?;

    trace!("Found decrypted bone_base address: 0x{:X}", decrypted_address);
    Ok(decrypted_address)
}

pub fn get_bone_index(index: u64, game_base_address: Address) -> u64 {
    unsafe { ffi::get_bone_index(index, game_base_address) }
}

#[cfg(test)]
mod tests {
    use super::super::globals;
    use super::*;
    use std::sync::Once;
    use memlib::logger::MinimalLogger;

    static INIT: Once = Once::new();

    pub fn init() {
        INIT.call_once(|| {
            let _ = MinimalLogger::init(LevelFilter::Trace);
            let handle = Handle::new(crate::PROCESS_NAME).unwrap();
            crate::sdk::init(handle).unwrap();
        })
    }

    #[test]
    fn client_info() {
        init();
        let client_info = get_client_info_address(globals::GAME_BASE_ADDRESS.get()).unwrap();
        info!("client_info: 0x{:X}", client_info)
    }

    #[test]
    fn client_base() {
        init();
        let client_info = get_client_info_address(globals::GAME_BASE_ADDRESS.get()).unwrap();
        let client_base = get_client_base_address(globals::GAME_BASE_ADDRESS.get(), client_info).unwrap();
        info!("client_base: 0x{:X}", client_base)
    }

    #[test]
    fn bone_base() {
        init();
        let bone_base = get_bone_base_address(globals::GAME_BASE_ADDRESS.get()).unwrap();
        info!("bone_base: 0x{:X}", bone_base);
    }
}