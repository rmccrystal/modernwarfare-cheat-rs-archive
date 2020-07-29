use crate::config::Config;
use crate::sdk::*;

use log::*;
use memlib::util::LoopTimer;
use std::io::Read;
use memlib::memory::Address;
use crate::sdk::structs::character_info;
use memlib::math::Angles2;

mod aimbot;

// The main loop of the cheat
// Returns an error if there is an error with any of the tick functions
pub fn hack_loop(game: Game) -> Result<(), Box<dyn std::error::Error>> {
    // Use the default config. We can change this later to load from a file
    let mut config = Config::default();

    // Create a timer from the tickrate of the cheat
    let mut timer = LoopTimer::new(crate::CHEAT_TICKRATE);

    println!("{:X}", std::mem::size_of::<character_info>());

    loop {
        timer.wait();

        for player in game.get_players().unwrap() {
            if player.name == "draven" {
                dbg!(&player.origin);
                for i in 1..=250 {
                    info!("{}: {:?}", i, player.get_bone_position(&game, i));
                }
            }
        }

        // println!("Enter address: ");
        // let stdin = std::io::stdin();
        // let mut buff = String::new();
        // stdin.read_line(&mut buff).unwrap();
        // let buff: u64 = buff.parse().unwrap();

        // let buff = 2681898626048;

        // memlib::memory::new_interactive_scan::<f32>((buff, buff + 0x3A60), true);
    }
}
