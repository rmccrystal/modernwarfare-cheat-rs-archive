use crate::config::Config;
use crate::sdk::*;

use memlib::util::LoopTimer;
use memlib::memory::{read_memory, Address, write_memory};
use memlib::math::Angles2;
use crate::hacks::no_recoil::NoRecoilState;


pub mod aimbot;
pub mod closest_player;
pub mod no_recoil;

// The main loop of the cheat
// Returns an error if there is an error with any of the tick functions
pub fn hack_loop(mut game: Game) -> Result<(), Box<dyn std::error::Error>> {
    // Use the default config. We can change this later to load from a file
    let config = Config::default();

    let mut aimbot_context = aimbot::AimbotContext::new();
    let no_recoil_state_sender = no_recoil::start_no_recoil_thread();

    // Create a timer from the tickrate of the cheat
    let mut timer = LoopTimer::new(crate::CHEAT_TICKRATE);


    loop {
        timer.wait();
        game.update();

        closest_player::closest_player(&game, &config);
        aimbot::aimbot(&game, &config, &mut aimbot_context);
        no_recoil_state_sender.send(NoRecoilState{
            enabled: config.no_recoil_enabled,
            client_info_base: game.client_info_base,
            in_game: game.in_game()
        }).unwrap();


        // println!("Enter address: ");
        // let stdin = std::io::stdin();
        // let mut buff = String::new();
        // stdin.read_line(&mut buff).unwrap();
        // let buff: u64 = buff.trim().parse().unwrap();
        //
        // memlib::memory::new_interactive_scan::<i32>((buff, buff + offsets::client_base::SIZE as u64), true);
    }
}
