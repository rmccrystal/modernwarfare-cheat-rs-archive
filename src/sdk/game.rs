#![allow(dead_code)]

use std::ops::{Add, Sub};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::time::{Duration, Instant};

use anyhow::*;
use log::*;
use memlib::math::{Angles2, Vector2, Vector3};
use memlib::memory::*;

use crate::sdk::globals::update_addresses_interval;
use crate::sdk::internal;
use crate::sdk::structs::RefDef;

use super::encryption;
use super::globals;
use super::offsets;
use super::player::Player;
use super::structs;
use super::w2s;

/// The single tick state of the cheat
#[derive(Clone, Debug)]
pub struct GameInfo {
    pub players: Vec<Player>,
    pub camera_pos: Vector3,
    pub local_view_angles: Angles2,
    pub local_index: i32,
}

impl GameInfo {
    pub fn get_player_by_id(&self, id: i32) -> Option<&Player> {
        self.players.iter().find(|&p| p.id == id)
    }

    pub fn get_local_player(&self) -> &Player {
        self.get_player_by_id(self.local_index).expect("Could not get local player by index")
    }
}

pub fn init(handle: Handle) -> Result<()> {
    // Set the global handle so we can use it anywhere
    set_global_handle(handle);

    // Get the base address or return an error
    let base_address = get_module(crate::PROCESS_NAME)
        .ok_or_else(|| anyhow!("Error getting module {}", crate::PROCESS_NAME))?
        .base_address;

    globals::init(base_address);

    Ok(())
}

pub fn get_game_info() -> Result<GameInfo> {
    let local_view_angles = internal::get_camera_angles().ok_or_else(|| anyhow!("Could not get local view angles"))?;
    let local_position = internal::get_camera_position().ok_or_else(|| anyhow!("Could not get camera pos"))?;
    let local_index = internal::get_local_index().ok_or_else(|| anyhow!("Could not get local index"))?;
    let players = internal::get_players().ok_or_else(|| anyhow!("Could not get players"))?;

    Ok(GameInfo {
        local_view_angles,
        camera_pos: local_position,
        local_index,
        players,
    })
}

pub fn world_to_screen(world_pos: &Vector3) -> Option<Vector2> {
    let refdef = globals::REFDEF.get().expect("world_to_screen was called without refdef").read();
    w2s::world_to_screen(
        &world_pos,
        internal::get_camera_position().expect("get_camera_position failed calling world_to_screen"),
        refdef.width,
        refdef.height,
        refdef.view.tan_half_fov,
        refdef.view.axis,
    )
}

// Converts units to in game meters
pub fn units_to_m(units: f32) -> f32 {
    units / offsets::UNIT_SCALE
}

pub fn m_to_units(meters: f32) -> f32 {
    meters * offsets::UNIT_SCALE
}