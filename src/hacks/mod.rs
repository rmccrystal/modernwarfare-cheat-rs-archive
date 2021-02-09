use crate::config::Config;
use crate::sdk::*;
use anyhow::*;

use memlib::util::LoopTimer;
use memlib::memory::{read_memory, Address, write_memory};
use memlib::math::{Angles2, Vector2};
use crate::hacks::no_recoil::NoRecoilState;
use memlib::overlay::{Color, TextStyle, Font, Draw, TextOptions};
use std::sync::{Arc, Mutex};
use std::sync::mpsc::{Sender, channel};
use std::thread::spawn;
use crate::sdk::bone::Bone;
use crate::hacks::aimbot::AimbotContext;
use log::*;
use memlib::overlay::imgui::{Imgui, ImguiConfig};
use memlib::overlay::window::Window;
use win_key_codes::VK_INSERT;
use std::time::Instant;
use imgui_ext::UiExt;


pub mod aimbot;
pub mod closest_player;
pub mod no_recoil;
pub mod esp;

/// The main loop of the cheat
/// Returns an error if there is an error with any of the tick functions
pub fn hack_main(mut _game: Game, window: Window) -> Result<()> {
    // Use the default config. We can change this later to load from a file
    let config = Config::load().unwrap_or_default();

    // Create a timer from the tickrate of the cheat
    let mut timer = LoopTimer::new(crate::CHEAT_TICKRATE);

    // Create contexts
    let mut aimbot_context = aimbot::AimbotContext::new();

    let shared_state = Arc::new(Mutex::new(
        CheatState { game: _game, config, aimbot_context }
    ));

    // Start render loop
    start_overlay_thread(
        shared_state.clone(),
        window
    );
    let no_recoil_state_sender = no_recoil::start_no_recoil_thread();

    loop {
        timer.wait();

        let mut state = shared_state.lock().unwrap().clone();

        state.game.update_addresses();
        let game_info = match state.game.get_game_info() {
            Ok(info) => info,
            Err(_) => continue
        };

        no_recoil_state_sender.send(NoRecoilState {
            enabled: state.config.no_recoil_enabled,
            client_info_base: state.game.addresses.client_info_base,
            in_game: true,
        }).expect("Failed to send NoRecoilState");

        aimbot::aimbot(&game_info, &state.config, &mut state.aimbot_context);

        {
            let mut current_state = shared_state.lock().unwrap();
            current_state.aimbot_context = state.aimbot_context;
            current_state.game = state.game;
        }
    }
}

#[derive(Clone)]
pub struct CheatState {
    game: Game,
    config: Config,
    aimbot_context: AimbotContext,
}

/// Returns a sender for new game updates
pub fn start_overlay_thread(shared_state: Arc<Mutex<CheatState>>, window: Window) {
    spawn(move || {
        let mut imgui = Imgui::from_window(
            window,
            ImguiConfig { toggle_menu_key: Some(VK_INSERT), align_to_pixel: true },
        ).unwrap();

        imgui.main_loop(|ui, ctx| {
            use memlib::overlay::imgui::*;

            let mut state = shared_state.lock().unwrap().clone();

            let prev_config = state.config.clone();

            Window::new(im_str!("test"))
                .build(&ui, || {
                    ui.draw_gui(&mut state.config);
                });

            if state.config != prev_config {
                debug!("Saving config");
                state.config.save();
            }

            {
                let mut new_state = shared_state.lock().unwrap();
                new_state.config = state.config;
            }
        }, |overlay| {
            let mut state = shared_state.lock().unwrap().clone();

            let game_info = match state.game.get_game_info() {
                Ok(n) => n,
                Err(e) => {
                    return;
                }
            };

            esp::esp(&game_info, overlay, &state.config, &state.aimbot_context);
            closest_player::closest_player(&game_info, &state.config, overlay);
        });
    });
}