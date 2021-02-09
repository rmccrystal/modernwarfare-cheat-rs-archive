#![feature(iterator_fold_self)]
#![feature(in_band_lifetimes)]

use memlib::logger::MinimalLogger;
use memlib::memory;
use memlib::overlay;

use log::*;
use anyhow::*;
use std::error::Error;
use memlib::memory::handle_interfaces::driver_handle::DriverProcessHandle;
use memory::Handle;
use memlib::overlay::imgui::{Imgui, ImguiConfig};
use memlib::overlay::window::Window;
use win_key_codes::VK_INSERT;
use msgbox::IconType;

mod sdk;
mod hacks;
mod config;

pub const PROCESS_NAME: &str = "ModernWarfare.exe";
pub const CHEAT_TICKRATE: u64 = 50;

const LOG_LEVEL: LevelFilter = LevelFilter::Debug;

fn run() -> Result<()> {
    // Initialize the logger
    MinimalLogger::init(LOG_LEVEL)?;

    // Create a handle to the game
    let handle = Handle::from_interface(DriverProcessHandle::attach(PROCESS_NAME)?);

    let window = Window::hijack_nvidia()?;
    window.bypass_screenshots(true);

    memlib::system::init().unwrap();

    // Create a game struct from the handle
    let game = sdk::Game::new(handle)?;

    // Run the hack loop
    hacks::hack_main(game, window)?;

    Ok(())
}

fn main() {
    std::process::exit(match run() {
        Ok(_) => {
            info!("Exiting cheat");
            0
        }
        Err(err) => {
            error!("{}", err);
            msgbox::create("Cryptx Error", &err.to_string(), IconType::Error);
            1
        }
    })
}
