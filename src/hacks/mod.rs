use crate::config::Config;
use crate::sdk::*;
use anyhow::*;

use memlib::util::{LoopTimer, GlobalCell, InitCell};
use memlib::memory::{read_memory, Address, write_memory};
use memlib::math::{Angles2, Vector2};
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
use std::time::{Instant, Duration};
use imgui_ext::UiExt;
use crate::sdk::globals::update_addresses_interval;
use std::borrow::BorrowMut;

pub mod aimbot;
pub mod closest_player;
pub mod no_recoil;
pub mod esp;

pub static CONFIG: InitCell<Config> = InitCell::new();
pub static STATE: InitCell<CheatState> = InitCell::new();

#[derive(Clone, Default)]
pub struct CheatState {
    pub aimbot_context: AimbotContext
}

/// The main loop of the cheat
/// Returns an error if there is an error with any of the tick functions
pub fn hack_main(window: Window) -> Result<()> {
    CONFIG.init(Config::load().unwrap_or_default());
    CONFIG.get_ref().save();
    STATE.init(Default::default());

    update_addresses_interval(Duration::from_secs(2));

    start_overlay_thread(window);
    no_recoil::start_no_recoil_thread();

    loop {
        let result = std::panic::catch_unwind(hack_loop);
        if let Err(e) = result {
            error!("Panic in hack_loop: {:?}", e);
            continue;
        } else {
            break;
        }
    }

    Ok(())
}

fn hack_loop() {
    let mut timer = LoopTimer::new(crate::CHEAT_TICKRATE);

    loop {
        update_addresses_interval(Duration::from_secs(2));
        timer.wait();

        let game_info = match get_game_info() {
            Ok(info) => info,
            Err(_) => continue
        };

        let config = CONFIG.get_ref();
        let mut state = STATE.get_mut();

        aimbot::aimbot(&config, &game_info, &mut state.aimbot_context);
    }
}

/// Returns a sender for new game updates
pub fn start_overlay_thread(window: Window) {
    spawn(move || {
        let mut imgui = Imgui::from_window(
            window,
            ImguiConfig { toggle_menu_key: Some(VK_INSERT), align_to_pixel: true },
        ).unwrap();

        let mut config = CONFIG.get_clone();
        imgui.main_loop(|ui, ctx| {
            use memlib::overlay::imgui::*;

            config = CONFIG.get_clone();

            Window::new(im_str!("test"))
                .build(&ui, || {
                    ui.draw_gui(&mut config);
                });

            // check if config was modified
            if !CONFIG.get_ref().eq(&config) {
                // save to file
                debug!("Saving config");
                config.save();

                CONFIG.set(config.clone());
            }
        }, |overlay| {
            let game_info = match get_game_info() {
                Ok(n) => n,
                Err(e) => {
                    return;
                }
            };

            let config = CONFIG.get_ref();
            let cheat_state = STATE.get_clone();
            esp::esp(&game_info, overlay, &config, &cheat_state.aimbot_context);
            closest_player::closest_player(&game_info, &config, overlay);
            // dbg!(1.0 / ((start.elapsed().as_micros() as f64) * 1_000_000.0));
        });
    });
}