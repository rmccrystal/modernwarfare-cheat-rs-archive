use crate::config::Config;
use crate::sdk::*;

use memlib::util::LoopTimer;
use memlib::memory::read_memory;
use memlib::math::Angles2;


pub mod aimbot;
pub mod closest_player;

// The main loop of the cheat
// Returns an error if there is an error with any of the tick functions
pub fn hack_loop(mut game: Game) -> Result<(), Box<dyn std::error::Error>> {
    // Use the default config. We can change this later to load from a file
    let config = Config::default();

    let mut aimbot_context = aimbot::AimbotContext::new();

    // Create a timer from the tickrate of the cheat
    let mut timer = LoopTimer::new(crate::CHEAT_TICKRATE);


    loop {
        timer.wait();
        game.update();

        closest_player::closest_player(&game, &config);
        aimbot::aimbot(&game, &config, &mut aimbot_context);


        // println!("Enter address: ");
        // let stdin = std::io::stdin();
        // let mut buff = String::new();
        // stdin.read_line(&mut buff).unwrap();
        // let buff: u64 = buff.trim().parse().unwrap();
        //
        // memlib::memory::new_interactive_scan::<i32>((buff, buff + offsets::client_base::SIZE as u64), true);
    }
}
