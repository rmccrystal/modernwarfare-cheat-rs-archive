use crate::sdk::*;
use log::*;
use memlib::{math, system};
use crate::config::{Keybind, Config};
use crate::sdk::bone::Bone;
use crate::sdk::CharacterStance;
use serde::{Serialize, Deserialize};
use memlib::math::Vector3;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, imgui_ext::Gui)]
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
    pub speed: f32,
    // 1 is instant, 1+ is smooth
    pub keybind: Keybind,
    #[imgui(checkbox(label = "Aimbot aimlock"))]
    pub aim_lock: bool,
    // Will lock onto the same player until button stops being pressed
    #[imgui(slider(min = 0.0, max = 2000.0, label = "Aimbot distance limit (m)"))]
    pub distance_limit: f32,
    // Distance limit in metres
    #[imgui(checkbox(label = "Aim at downed"))]
    pub aim_at_downed: bool,
    /// Smooth based on scope
    pub scale_speed: bool,
}

impl AimbotConfig {
    pub fn default() -> Self {
        Self {
            enabled: true,
            teams: true,
            bone: Bone::Head,
            fov: 30.0,
            speed: 2.0,
            keybind: Keybind::WhilePressed(vec![win_key_codes::VK_XBUTTON1]),
            aim_lock: true,
            distance_limit: 400.0,
            aim_at_downed: false,
            scale_speed: true,
        }
    }
}

#[derive(Clone, Default)]
pub struct AimbotContext {
    pub aim_lock_player_id: Option<i32> // The target ID we are aimlocking to
}

pub fn aimbot(global_config: &Config, game_info: &GameInfo, ctx: &mut AimbotContext) {
    let config = &global_config.aimbot_config;

    if !config.enabled {
        return;
    }

    if !config.keybind.get_state() {
        ctx.aim_lock_player_id = None;
        return;
    }

    // Get target
    let target = {
        if let Some(id) = ctx.aim_lock_player_id {
            match game_info.get_player_by_id(id) {
                Some(pl) => Some((pl, get_aim_position(&pl, &game_info, &ctx))),
                None => get_target(&game_info, &config, &ctx, &global_config.friends)
            }
        } else {
            get_target(&game_info, &config, &ctx, &global_config.friends)
        }
    };

    if target.is_none() {
        debug!("No target");
        ctx.aim_lock_player_id = None;
        return;
    }

    let (player, target) = target.unwrap();
    if player.stance == CharacterStance::Downed {
        ctx.aim_lock_player_id = None;
    }

    ctx.aim_lock_player_id = Some(player.id);

    // Aim at target
    aim_at(&game_info, &player, &target, &config);
}

/// Gets the position to aim at given a player.
/// This is where prediction should be implemented
fn get_aim_position(player: &Player, game_info: &GameInfo, ctx: &AimbotContext) -> Vector3 {
    player.estimate_head_position()
}

/// Returns the target player and the position to aim at
fn get_target<'a>(game_info: &'a GameInfo, config: &AimbotConfig, ctx: &AimbotContext, friends: &[String]) -> Option<(&'a Player, Vector3)> {
    let local_player = game_info.get_local_player();

    game_info.players.iter().filter_map(|player| {
        if player.id == local_player.id {
            return None;
        }

        // Ignore downed
        if !config.aim_at_downed && player.stance == CharacterStance::Downed {
            return None;
        }

        // Check team
        if !config.teams && player.is_teammate(&game_info, &friends) {
            return None;
        }

        // Check distance
        let distance = units_to_m((player.origin - local_player.origin).length());
        if distance > config.distance_limit {
            return None;
        }

        // first calculate fov to origin so we don't have to run prediction for every player
        let fov_to_origin = math::calculate_relative_angles(&game_info.camera_pos, &player.origin, &game_info.local_view_angles).length();
        dbg!(fov_to_origin);
        if fov_to_origin * 1.5 > config.fov {
            return None;
        }

        let aim_position = get_aim_position(&player, &game_info, &ctx);
        let angle = math::calculate_relative_angles(&game_info.camera_pos, &aim_position, &game_info.local_view_angles).length();
        dbg!(angle);
        if angle > config.fov {
            return None;
        }

        Some((player, aim_position, angle, distance))
    })
        .min_by_key(|(player, aim_position, angle, distance)| {
            // Combine fov and distance
            (angle + (distance / 100.0) * angle) as i32
        })
        .map(|(player, aim_position, angle, distance)| (player, aim_position))
}

/// Aims at `target`
fn aim_at(game_info: &GameInfo, player: &Player, target: &Vector3, config: &AimbotConfig) {
    let absolute_delta = math::calculate_relative_angles(&game_info.camera_pos, &target, &game_info.local_view_angles);

    info!("Aiming at {}\t({}m)\t({}°)\t({})\t({:?})",
          player.name,
          units_to_m((target - game_info.camera_pos).length()),
          absolute_delta.length(),
          player.health,
          player.stance
    );

    let fov_multiplier = 120.0 / get_fov();
    let tickrate_multiplier = (crate::CHEAT_TICKRATE as f32) / 120.0;
    let speed_multiplier = config.speed;
    let scale = 1.0 / 2.5;

    dbg!(absolute_delta);
    let multiplier = fov_multiplier * tickrate_multiplier * speed_multiplier;
    dbg!(multiplier);

    let scaled_delta = absolute_delta * (multiplier * scale);
    dbg!(scaled_delta);

    let dx = -scaled_delta.yaw.ceil() as i32;
    let dy = scaled_delta.pitch.ceil() as i32;

    dbg!(dx, dy);

    system::move_mouse_relative(dx, dy);
}