use memlib::logger::MinimalLogger;
use memlib::memory;
use memlib::system;

use log::*;
use std::error::Error;

mod sdk;
mod hacks;
mod config;

pub const PROCESS_NAME: &str = "modernwarfare.exe";
pub const CHEAT_TICKRATE: u64 = 50;

const LOG_LEVEL: LevelFilter = LevelFilter::Debug;

fn run() -> Result<(), Box<dyn Error>> {
    // Initialize the logger
    MinimalLogger::init(LOG_LEVEL)?;

    // Create a handle to the game
    let handle = memory::Handle::new(PROCESS_NAME)?;

    // Init system by connecting to RPC running on guest
    system::connect(&"192.168.122.129:9800".parse().unwrap()).unwrap();

    // Create a game struct from the handle
    let mut game = sdk::Game::new(handle)?;

    // Run the hack loop
    hacks::hack_loop(game)?;

    Ok(())
}

fn main() {
    std::process::exit(match run() {
        Ok(_) => {
            info!("Exiting cheat");
            0
        },
        Err(err) => {
            error!("{}", err);
            1
        }
    })
}
