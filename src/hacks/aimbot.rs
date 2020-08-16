use crate::sdk::*;
use log::*;
use memlib::system;
use memlib::math;
use crate::config::{Keybind, Config};
use crate::sdk::bone::Bone;
use crate::sdk::structs::CharacterStance;
use memlib::math::Vector3;

#[derive(Clone, Debug)]
pub struct AimbotConfig {
    pub enabled: bool,
    pub team_check: bool,
    pub bone: Bone,
    pub fov: f32,
    // FOV in degrees
    pub smooth: f32,
    // 1 is instant, 1+ is smooth
    pub keybind: Keybind,
    pub aim_lock: bool,
    // Will lock onto the same player until button stops being pressed
    pub distance_limit: f32,
    // Distance limit in meteres
    pub aim_at_downed: bool,
}

impl AimbotConfig {
    pub fn default() -> Self {
        Self {
            enabled: true,
            team_check: true,
            bone: Bone::Head,
            fov: 30.0,
            smooth: 1.2,
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

pub fn aimbot(game: &Game, global_config: &Config, ctx: &mut AimbotContext) {
    let config = &global_config.aimbot_config;

    if !config.enabled {
        return;
    }

    if !config.keybind.get_state() {
        ctx.aim_lock_player_id = None;
        return;
    }

    let game_info = {
        if game.game_info.is_none() {
            debug!("Not in game");
            return;
        }
        game.game_info.as_ref().unwrap()
    };

    let get_head_pos = |player: &Player| {
        player.get_head_position(&game)
    };

    // Get target
    let target = {
        if let Some(id) = ctx.aim_lock_player_id {
            game.get_player_by_id(id)
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
    if target.stance == CharacterStance::DOWNED {
        ctx.aim_lock_player_id = None;
    }

    ctx.aim_lock_player_id = Some(target.character_id);

    // Aim at target
    let _ = aim_at(&game_info, &target, &config, get_head_pos);
}

/// Returns the target to aim at or None otherwise
fn get_target(game_info: &GameInfo, config: &AimbotConfig, get_aim_position: impl Fn(&Player) -> Vector3, friends: &Vec<String>) -> Option<Player> {
    let local_player = &game_info.local_player;
    let mut player_list = game_info.players.clone();

    // Remove local_player
    player_list.retain(|player| player.character_id != local_player.character_id);

    let local_view_angles = &game_info.local_view_angles;
    let local_position = &game_info.local_position;

    // Get the best player by FOV
    let mut target = None;
    let mut best_score = 9999999.0;

    for player in player_list {
        if config.team_check && player.team == local_player.team {
            continue;
        }
        if friends.contains(&player.name) {
            continue;
        }

        // let position = player.get_bone_position(&game, config.bone);
        // if position.is_err() {
        //     error!("Failed to get bone position for {}: {}", player.name, position.err().unwrap());
        //     return None;
        // }
        // let position = position.unwrap();
        let position = get_aim_position(&player);

        let distance = (position - local_position).length();

        if units_to_m(distance) > config.distance_limit {
            continue;
        }

        if !config.aim_at_downed && player.stance == CharacterStance::DOWNED {
            continue;
        }

        let angle = math::calculate_relative_angles(&local_position, &position, &local_view_angles);
        let distance = (local_position - position).length();
        let fov = angle.length();

        // Combine fov and distance
        let score = fov + (distance / 100.0) * fov;

        // Update best_fov if necessary
        if fov < config.fov && score < best_score {
            best_score = score;
            target = Some(player);
        }
    }

    target
}

fn aim_at(game_info: &GameInfo, target: &Player, config: &AimbotConfig, get_aim_position: impl Fn(&Player) -> Vector3) {
    let target_position = get_aim_position(&target);
    let local_position = &game_info.local_position;
    let local_view_angles = &game_info.local_view_angles;

    let delta = math::calculate_relative_angles(&local_position, &target_position, &local_view_angles);

    debug!("Aiming at {}\t({}m)\t({}°)\t({})\t({:?})",
           target.name,
           units_to_m((target_position - local_position).length()),
           delta.length(),
           target.health,
           target.stance
    );

    let new_delta = delta / (config.smooth * crate::CHEAT_TICKRATE as f32);

    let mouse_multiplier = 250.0;
    let dx = -new_delta.yaw * mouse_multiplier;
    let dy = new_delta.pitch * mouse_multiplier;

    let dx = if dx as i32 == 0 { 1 } else { dx as i32 };
    let dy = if dy as i32 == 0 { 1 } else { dy as i32 };

    system::move_mouse_relative(dx as i32, dy as i32);
}