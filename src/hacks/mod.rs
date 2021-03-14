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
        window,
    );
    let no_recoil_state_sender = no_recoil::start_no_recoil_thread();

    loop {
        timer.wait();

        let CheatState { config, mut game, mut aimbot_context } = shared_state.lock().unwrap().clone();

        game.update_addresses();
        let game_info = match game.get_game_info() {
            Ok(info) => info,
            Err(_) => continue
        };

        no_recoil_state_sender.send(NoRecoilState {
            enabled: config.no_recoil_enabled,
            client_info_base: game.addresses.client_info_base,
            in_game: true,
        }).expect("Failed to send NoRecoilState");

        aimbot::aimbot(&game_info, &config, &mut aimbot_context);

        {
            let mut shared_state = shared_state.lock().unwrap();
            shared_state.aimbot_context = aimbot_context;
            shared_state.game = game;
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

            let CheatState { mut config, .. } = shared_state.lock().unwrap().clone();

            let prev_config = config.clone();

            Window::new(im_str!("test"))
                .build(&ui, || {
                    ui.draw_gui(&mut config);
                });

            if config != prev_config {
                // save to file
                debug!("Saving config");
                config.save();

                // save to shared state
                let mut new_state = shared_state.lock().unwrap();
                new_state.config = config;
            }
        }, |overlay| {
            let CheatState {game, config, aimbot_context} = shared_state.lock().unwrap().clone();

            let game_info = match game.get_game_info() {
                Ok(n) => n,
                Err(e) => {
                    return;
                }
            };

            esp::esp(&game_info, overlay, &config, &aimbot_context);
            closest_player::closest_player(&game_info, &config, overlay);
        });
    });
}