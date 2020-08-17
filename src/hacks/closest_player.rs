use crate::sdk::*;
use crate::sdk::structs::CharacterStance;
use crate::config::{Keybind, Config};
use log::*;
use memlib::overlay::{Overlay, Color, TextStyle, Font, TextOptions};
use memlib::math::Vector2;

#[derive(Clone, Debug)]
pub struct ClosestPlayerConfig {
    pub enabled: bool,
    pub ignore_downed: bool,
}

impl ClosestPlayerConfig {
    pub fn default() -> Self {
        Self {
            enabled: true,
            ignore_downed: true,
        }
    }
}

pub fn closest_player(game: &Game, global_config: &Config, overlay: &mut Overlay) {
    let config = &global_config.cloest_player_config;
    if !config.enabled {
        return;
    }

    let game_info = {
        if game.game_info.is_none() {
            return
        }
        game.game_info.as_ref().unwrap()
    };

    let players = &game_info.players;
    let local_player = &game_info.local_player;

    let mut closest_player: (Option<Player>, f32) = (None, 9999999999.0);

    for player in players {
        if player.character_id == local_player.character_id || player.team == local_player.team {
            continue;
        }
        if config.ignore_downed && player.stance == CharacterStance::DOWNED {
            continue;
        }
        if global_config.friends.contains(&player.name) {
            continue
        }

        let distance = units_to_m((player.origin - local_player.origin).length());
        if distance < closest_player.1 {
            closest_player.0 = Some(player.clone());
            closest_player.1 = distance;
        }
    }

    // If there aren't any players that fit the criteria, return
    if closest_player.0.is_none() {
        return
    }

    let player = closest_player.0.unwrap();
    let distance = closest_player.1;
    let angle = memlib::math::calculate_relative_angles(&local_player.origin, &player.origin, &game_info.local_view_angles);

    overlay.draw_text(
        Vector2{x: 7.0, y: 20.0},
        &format!("Closest player: {}\t({}m),\t({})", player.name, distance, -angle.yaw),
        TextOptions::default().color(if distance < 50.0 { Color::from_rgb(255, 0, 0) } else { Color::from_rgb(255, 255, 255) })
    );
}