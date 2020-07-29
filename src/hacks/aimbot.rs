use crate::sdk::*;
use log::*;
use memlib::system;
use memlib::math;
use crate::config::Keybind;
use crate::sdk::bone::Bone;
use crate::sdk::structs::CharacterStance;
use memlib::math::Vector3;

pub struct AimbotConfig {
    pub team_check: bool,
    pub bone: Bone,
    pub fov: f32,
    // FOV in degrees
    pub smooth: f32,
    // 1 is instant, 1+ is smooth
    pub keybind: Keybind,
    pub aim_lock: bool,
    // Will lock onto the same player until button stops being pressed
    pub distance_limit: f32,    // Distance limit in meteres
}

impl AimbotConfig {
    pub fn default() -> Self {
        Self {
            team_check: true,
            bone: Bone::HeadAlternate,
            fov: 30.0,
            smooth: 2.5,
            keybind: Keybind::WhilePressed(vec![win_key_codes::VK_XBUTTON1]),
            aim_lock: true,
            distance_limit: 400.0,
        }
    }
}

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

pub fn aimbot(game: &Game, config: &AimbotConfig, ctx: &mut AimbotContext) {
    if !config.keybind.get_state() {
        ctx.aim_lock_player_id = None;
        return;
    }

    // Get player list
    if game.get_character_array().is_none() {
        debug!("No players");
        return;
    }

    let get_aim_position = |player: &Player| {
        let delta_z = match player.stance {
            CharacterStance::STANDING => 60.0,
            CharacterStance::CROUCHING => 40.0,
            CharacterStance::CRAWLING => 10.0,
            CharacterStance::DOWNED => 20.0,
        };
        player.origin + Vector3 { x: 0.0, y: 0.0, z: delta_z }
    };

    // Get target
    let target = {
        if let Some(id) = ctx.aim_lock_player_id {
            game.get_player_by_id(id)
        } else {
            get_target(&game, &config, &get_aim_position)
        }
    };

    if target.is_none() {
        debug!("No target");
        ctx.aim_lock_player_id = None;
        return;
    }
    let target = target.unwrap();

    ctx.aim_lock_player_id = Some(target.character_id);

    // Aim at target
    aim_at(&game, &target, &config, get_aim_position);
}

/// Returns the target to aim at or None otherwise
fn get_target(game: &Game, config: &AimbotConfig, get_aim_position: impl Fn(&Player) -> Vector3) -> Option<Player> {
    let local_player = game.get_local_player()?;
    let mut player_list = game.get_players()?;

    // Remove local_player
    player_list.retain(|player| player.character_id != local_player.character_id);

    // If there are no players, return
    if player_list.is_empty() {
        return None;
    }

    let local_view_angles = game.get_camera_angles();
    let local_position = game.get_camera_position();

    // Get the best player by FOV
    let mut target = None;
    let mut best_score = 9999999.0;


    for player in player_list {
        if config.team_check && player.team == local_player.team {
            continue;
        }

        // let position = player.get_bone_position(&game, config.bone);
        // if position.is_err() {
        //     error!("Failed to get bone position for {}: {}", player.name, position.err().unwrap());
        //     return None;
        // }
        // let position = position.unwrap();
        let position = get_aim_position(&player);

        if units_to_m((position - local_position).length()) > config.distance_limit {
            continue;
        }

        let angle = math::calculate_relative_angles(local_position, position, &local_view_angles);
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

fn aim_at(game: &Game, target: &Player, config: &AimbotConfig, get_aim_position: impl Fn(&Player) -> Vector3) -> Result<(), Box<dyn std::error::Error>> {
    // let target_position = target.get_bone_position(&game, config.bone)?;
    let mut target_position = get_aim_position(&target);

    let local_position = game.get_camera_position();
    let local_view_angles = game.get_camera_angles();

    let mut delta = math::calculate_relative_angles(local_position, target_position, &local_view_angles);

    debug!("Aiming at {}\t({}m)\t({}°)\t({:?})",
           target.name,
           units_to_m((target_position - local_position).length()),
           delta.length(),
           target.stance
    );

    let new_delta = delta / (config.smooth * crate::CHEAT_TICKRATE as f32);

    let mouse_multiplier = 250.0;
    let dx = -new_delta.yaw * mouse_multiplier;
    let dy = new_delta.pitch * mouse_multiplier;

    let dx = if dx as i32 == 0 { 1 } else { dx as i32 };
    let dy = if dy as i32 == 0 { 1 } else { dy as i32 };

    system::move_mouse_relative(dx as i32, dy as i32);

    Ok(())
}