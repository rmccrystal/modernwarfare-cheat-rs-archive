use crate::sdk::*;
use log::*;
use memlib::hacks::aimbot::calculate_aimbot;
use memlib::math;

pub struct AimbotConfig {
    pub team_check: bool,
    pub bone: i32,
    pub fov: f32, // FOV in degrees
}

impl AimbotConfig {
    pub fn default() -> Self {
        Self { team_check: false, bone: 5, fov: 30.0 }
    }
}

pub fn aimbot(game: &Game, config: &AimbotConfig) {
    // Get player list
    if game.get_character_array().is_none() {
        trace!("No players");
        return;
    }

    // Get target
    let target = get_target(&game, &config);
    if target.is_none() {
        trace!("No target");
        return;
    }
    let target = target.unwrap();

    // Aim at target
    aim_at(&game, &target, &config);
}

/// Returns the target to aim at or None otherwise
fn get_target(game: &Game, config: &AimbotConfig) -> Option<Player> {
    let local_player = game.get_local_player()?;
    let mut player_list = game.get_players()?;

    // Remove local_player
    player_list.retain(|player| player.character_id != local_player.character_id);

    // Team check
    if config.team_check {
        player_list.retain(|player| player.team != local_player.team);
    }

    // If there are no players, return
    if player_list.is_empty() {
        return None;
    }

    let local_view_angles = game.get_camera_angles();
    let eye_position = game.get_camera_position();

    // Get the best player by FOV
    let mut target = None;
    let mut best_fov = config.fov;

    for player in player_list {
        let position = player.get_bone_position(&game, config.bone).ok()?;
        let angle = math::calculate_relative_angles(eye_position, position, &local_view_angles);
        let fov = angle.length();

        // Update best_fov if necessary
        if fov < config.fov && fov < best_fov {
            best_fov = fov;
            target = Some(player);
        }
    }

    target
}

fn aim_at(game: &Game, target: &Player, config: &AimbotConfig) -> Result<(), Box<dyn std::error::Error>> {
    let target_position = target.get_bone_position(&game, config.bone)?;

    let local_position = game.get_camera_position();
    let local_view_angles = game.get_camera_angles();

    let delta = math::calculate_relative_angles(local_position, target_position, &local_view_angles);

    dbg!(delta);

    Ok(())
}