use crate::config::Config;
use crate::sdk::*;

use memlib::util::LoopTimer;
use log::*;
use crate::sdk::structs::CharacterStance;


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

        if memlib::system::get_key_state(win_key_codes::VK_P) {
            if let Some(players) = game.get_players() {
                let local_player = game.get_local_player().unwrap();
                let mut closest_player: (Option<Player>, f32) = (None, 9999999999.0);
                for player in players {
                    if player.character_id == local_player.character_id || player.team == local_player.team {
                        continue;
                    }
                    if player.name.to_lowercase().contains("totsugeki banzai") {
                        continue;
                    }
                    if player.stance == CharacterStance::DOWNED {
                        continue;
                    }
                    let distance = units_to_m((player.origin - local_player.origin).length());
                    if distance < closest_player.1 {
                        closest_player.0 = Some(player.clone());
                        closest_player.1 = distance;
                    }
                }

                let player = closest_player.0.unwrap();
                let angle = memlib::math::calculate_relative_angles(local_player.origin, player.origin, &game.get_camera_angles());

                info!("Closest player: {}\t({}m),\t({})", player.name, closest_player.1, -angle.yaw);
            }
        }

        aimbot::aimbot(&game, &config.aimbot_config, &mut aimbot_context);

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
