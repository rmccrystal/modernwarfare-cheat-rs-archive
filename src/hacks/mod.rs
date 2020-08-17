use crate::config::Config;
use crate::sdk::*;

use memlib::util::LoopTimer;
use memlib::memory::{read_memory, Address, write_memory};
use memlib::math::{Angles2, Vector2};
use crate::hacks::no_recoil::NoRecoilState;
use memlib::overlay::{Overlay, OverlayInterface, Color, TextStyle, Font};
use std::sync::Arc;
use std::sync::mpsc::{Sender, channel};
use std::thread::spawn;
use crate::sdk::bone::Bone;
use crate::hacks::aimbot::AimbotContext;
use log::*;


pub mod aimbot;
pub mod closest_player;
pub mod no_recoil;
pub mod esp;

// The main loop of the cheat
// Returns an error if there is an error with any of the tick functions
pub fn hack_loop(mut game: Game, mut overlay: Overlay) -> Result<(), Box<dyn std::error::Error>> {
    // Use the default config. We can change this later to load from a file
    let config = Config::default();
    // Create a timer from the tickrate of the cheat
    let mut timer = LoopTimer::new(crate::CHEAT_TICKRATE);

    // Create contexts
    let mut aimbot_context = aimbot::AimbotContext::new();

    // Start render loop
    let render_game_sender = start_render_thread(
        RenderState { game: game.clone(), config: config.clone(), aimbot_context: aimbot_context.clone() },
        overlay,
    );
    let no_recoil_state_sender = no_recoil::start_no_recoil_thread();

    game.get_character_array();

    loop {
        timer.wait();
        game.update();

        // Send the updated game
        render_game_sender.send(RenderState {
            game: game.clone(),
            config: config.clone(),
            aimbot_context: aimbot_context.clone(),
        }).expect("Failed to send RenderState");
        no_recoil_state_sender.send(NoRecoilState {
            enabled: config.no_recoil_enabled,
            client_info_base: game.client_info_base,
            in_game: game.in_game(),
        }).expect("Failed to send NoRecoilState");

        aimbot::aimbot(&game, &config, &mut aimbot_context);
    }
}

pub struct RenderState {
    game: Game,
    config: Config,
    aimbot_context: AimbotContext,
}

/// Returns a sender for new game updates
pub fn start_render_thread(state: RenderState, mut overlay: Overlay) -> Sender<RenderState> {
    let (send, recv) = channel();
    let mut state = state;
    spawn(move || {
        let mut timer = LoopTimer::new(100);

        loop {
            timer.wait();
            state = recv.try_recv().unwrap_or(state);
            state.game.update();

            overlay.begin();

            esp::esp(&state.game, &mut overlay, &state.config.esp_config, &state.aimbot_context);
            closest_player::closest_player(&state.game, &state.config, &mut overlay);

            // dbg!(&state.game.game_info.as_ref().unwrap().players);

            // let game = &state.game;
            // let players = &game.game_info.as_ref().unwrap().players;
            // let player = players.iter().find(|p| p.name == "Denny").unwrap();
            // for i in 2..=24 {
            //     let pos = player.get_bone_position(&game, unsafe { std::mem::transmute(i) });
            //     if pos.is_err() { continue; };
            //     let pos = pos.unwrap();
            //
            //     let screen_pos = game.world_to_screen(&pos);
            //     if screen_pos.is_none() { continue; };
            //     let screen_pos = screen_pos.unwrap();
            //
            //     overlay.draw_text(screen_pos, &i.to_string(), Color::from_rgb(255, 255, 255), TextStyle::Outlined, Font::Pixel, 0.0, true);
            // }

            overlay.end();
        }
    });

    send
}