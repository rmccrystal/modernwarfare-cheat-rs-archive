use memlib::memory::{write_memory, Address, read_memory, write_bytes};
use crate::sdk::offsets;
use std::sync::mpsc::{Sender, channel};
use std::num::Wrapping;
use std::sync::{Mutex, Arc};
use log::*;
use std::{time, thread};
use crate::hacks::CONFIG;

pub fn start_no_recoil_thread() {
    thread::spawn(move || {
        loop {
            if !CONFIG.get_ref().no_recoil_enabled {
                return
            }
            if let Some(base) = crate::sdk::globals::CLIENT_INFO.get_clone() {
                no_recoil_tick(base);
            }
        }
    });
}

fn no_recoil_tick(client_info_base: Address) {
    let mut r12 = Wrapping(client_info_base + offsets::NO_RECOIL);
    let mut rsi = r12 + Wrapping(0x4);
    let edx: Wrapping<u32> = Wrapping(read_memory((r12 + Wrapping(0xC)).0));
    let mut ecx = edx;
    ecx = Wrapping(ecx.0 ^ r12.0 as u32);
    let mut eax = ecx + Wrapping(2);
    eax = eax * ecx;
    ecx = edx;
    ecx = Wrapping(ecx.0 ^ rsi.0 as u32);
    let address = r12.0;
    let val1 = eax.0;
    eax = ecx + Wrapping(2);
    eax = eax * ecx;
    let val2 = eax.0;

    let mut bytes = [0; 8];
    bytes[0..=3].clone_from_slice(&val1.to_ne_bytes());
    bytes[4..=7].clone_from_slice(&val2.to_ne_bytes());

    write_bytes(address, &bytes);
}
