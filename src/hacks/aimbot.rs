use crate::sdk::*;
use log::*;
use memlib::{math, system};
use crate::config::{Keybind, Config};
use crate::sdk::bone::Bone;
use crate::sdk::structs::CharacterStance;
use memlib::math::Vector3;

#[derive(Clone, Debug, imgui_ext::Gui)]
pub struct AimbotConfig {
    #[imgui(checkbox(label = "Aimbot enabled"))]
    pub enabled: bool,
    #[imgui(checkbox(label = "Aim at teams"))]
    pub teams: bool,
    pub bone: Bone,
    #[imgui(slider(min = 0.0, max = 180.0, label = "Aimbot FOV"))]
    pub fov: f32,
    // FOV in degrees
    #[imgui(slider(min = 0.5, max = 25.0, label = "Aimbot Smooth"))]
    pub smooth: f32,
    // 1 is instant, 1+ is smooth
    pub keybind: Keybind,
    #[imgui(checkbox(label = "Aimbot aimlock"))]
    pub aim_lock: bool,
    // Will lock onto the same player until button stops being pressed
    #[imgui(slider(min = 0.0, max = 2000.0, label = "Aimbot distance limit (m)"))]
    pub distance_limit: f32,
    // Distance limit in meteres
    #[imgui(checkbox(label = "Aim at downed"))]
    pub aim_at_downed: bool,
}

impl AimbotConfig {
    pub fn default() -> Self {
        Self {
            enabled: true,
            teams: true,
            bone: Bone::Head,
            fov: 30.0,
            smooth: 2.0,
            keybind: Keybind::WhilePressed(vec![win_key_codes::VK_XBUTTON1]),
            aim_lock: true,
            distance_limit: 400.0,
            aim_at_downed: false,
        }
    }
}

#[derive(Clone)]
pub struct AimbotContext {
    pub aim_lock_player_id: Option<i32> // The target ID we are aimlocking to
}

impl AimbotContext {
    pub fn new() -> Self {
        Self {
            aim_lock_player_id: None
        }
    }
}

pub fn aimbot(game_info: &GameInfo, global_config: &Config, ctx: &mut AimbotContext) {
    let config = &global_config.aimbot_config;

    if !config.enabled {
        return;
    }

    if !config.keybind.get_state() {
        ctx.aim_lock_player_id = None;
        return;
    }

    let get_head_pos = |player: &Player| {
        player.get_head_position(&game_info.game)
    };

    // Get target
    let target = {
        if let Some(id) = ctx.aim_lock_player_id {
            match game_info.get_player_by_id(id) {
                Some(pl) => Some(pl),
                None => get_target(&game_info, &config, get_head_pos, &global_config.friends)
            }
        } else {
            get_target(&game_info, &config, get_head_pos, &global_config.friends)
        }
    };

    if target.is_none() {
        debug!("No target");
        ctx.aim_lock_player_id = None;
        return;
    }
    let target = target.unwrap();
    if target.stance == CharacterStance::Downed {
        ctx.aim_lock_player_id = None;
    }

    ctx.aim_lock_player_id = Some(target.id);

    // Aim at target
    let _ = aim_at(&game_info, &target, &config, get_head_pos);
}

/// Returns the target to aim at or None otherwise
fn get_target<'a, 'g>(game_info: &'a GameInfo, config: &AimbotConfig, get_aim_position: impl Fn(&Player) -> Vector3, friends: &[String]) -> Option<&'a Player> {
    let local_player = &game_info.local_player;
    let mut player_list = game_info.players.clone();

    // Remove local_player
    player_list.retain(|player| player.id != local_player.id);

    let local_view_angles = &game_info.local_view_angles;
    let local_position = &game_info.local_position;

    game_info.players.iter()
        .filter(|&player| {
            let position = get_aim_position(&player);

            if player.id == game_info.local_player.id {
                return false;
            }

            // Filter out teammates
            if !config.teams && player.is_teammate(&game_info, friends) {
                return false;
            }

            // Filter out of fov
            let fov = math::calculate_relative_angles(&local_position, &position, &local_view_angles).length();
            if fov > config.fov {
                return false;
            }


            // Filter out players too far away
            let distance = (position - local_position).length();

            if units_to_m(distance) > config.distance_limit {
                return false;
            }

            // Ignore downed
            if !config.aim_at_downed && player.stance == CharacterStance::Downed {
                return false;
            }

            true
        })
        .min_by_key(|&player| {
            let position = get_aim_position(&player);

            let angle = math::calculate_relative_angles(&local_position, &position, &local_view_angles);
            let distance = (local_position - position).length();
            let fov = angle.length();

            // Combine fov and distance
            (fov + (distance / 100.0) * fov) as i32
        })
}

fn aim_at(game_info: &GameInfo, target: &Player, config: &AimbotConfig, get_aim_position: impl Fn(&Player) -> Vector3) {
    let target_position = get_aim_position(&target);
    let local_position = &game_info.local_position;
    let local_view_angles = &game_info.local_view_angles;

    let delta = math::calculate_relative_angles(&local_position, &target_position, &local_view_angles);

    info!("Aiming at {}\t({}m)\t({}°)\t({})\t({:?})",
           target.name,
           units_to_m((target_position - local_position).length()),
           delta.length(),
           target.health,
           target.stance
    );

    let new_delta = delta / ((config.smooth / 2.0) * crate::CHEAT_TICKRATE as f32);

    let mouse_multiplier = 250.0;
    let dx = -new_delta.yaw * mouse_multiplier;
    let dy = new_delta.pitch * mouse_multiplier;

    let dx = if dx as i32 == 0 { 1 } else { dx as i32 };
    let dy = if dy as i32 == 0 { 1 } else { dy as i32 };

    system::move_mouse_relative(dx as i32, dy as i32);
}