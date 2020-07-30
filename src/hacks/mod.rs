use crate::config::Config;
use crate::sdk::*;

use memlib::util::LoopTimer;





pub mod aimbot;

// The main loop of the cheat
// Returns an error if there is an error with any of the tick functions
pub fn hack_loop(game: Game) -> Result<(), Box<dyn std::error::Error>> {
    // Use the default config. We can change this later to load from a file
    let config = Config::default();

    let mut aimbot_context = aimbot::AimbotContext::new();

    // Create a timer from the tickrate of the cheat
    let mut timer = LoopTimer::new(crate::CHEAT_TICKRATE);


    loop {
        timer.wait();

        for player in game.get_players().unwrap() {
            print!("({}, {}), ", player.origin.x, player.origin.y);
        }
        println!();
        dbg!(game.get_camera_position());

        // dbg!(game.get_local_player());
        // dbg!(game.get_camera_position());

        // aimbot::aimbot(&game, &config.aimbot_config, &mut aimbot_context);

        // for player in game.get_players().unwrap() {
        //     if player.name == "draven" {
        //         let mut max_id = (0, -1.0);
        //         let mut valid_bone_count = 0;
        //         for i in 1..=200 {
        //             let pos = player.get_bone_position(&game, i);
        //             if let Err(err) = &pos {
        //                 println!("{}", err);
        //                 continue;
        //             };
        //             valid_bone_count += 1;
        //             let pos = pos.unwrap();
        //             if pos.z > max_id.1 {
        //                 max_id.0 = i;
        //                 max_id.1 = pos.z;
        //             }
        //         }
        //
        //         println!("Searched {} bones. bone ID {}, z value {}", valid_bone_count, max_id.0, max_id.1);
        //         println!("camera Z: {}", game.get_camera_position().z);
        //     }
        // }

        // println!("Enter address: ");
        // let stdin = std::io::stdin();
        // let mut buff = String::new();
        // stdin.read_line(&mut buff).unwrap();
        // let buff: u64 = buff.parse().unwrap();

        // let buff = 2681898626048;

        // memlib::memory::new_interactive_scan::<f32>((buff, buff + 0x3A60), true);
    }
}
