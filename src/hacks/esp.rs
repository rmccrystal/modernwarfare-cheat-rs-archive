use crate::sdk::{Game, Player, units_to_m};
use std::cmp;
use crate::sdk::structs::CharacterStance;
use memlib::overlay::{Overlay, OverlayInterface, Color, TextStyle, Font};
use memlib::math::{Vector3, Vector2};
use memlib::unwrap_or_return;
use crate::hacks::aimbot::AimbotContext;
use crate::sdk::bone::Bone;
use std::cmp::Ordering;

#[derive(Copy, Clone, Debug)]
pub struct EspConfig {
    name_color: Color,
    box_color: Color,
    highlighted_box_color: Color,
    max_distance: f32,
    teams: bool,
    opacity: u8,
    skeleton: bool,
}

impl EspConfig {
    pub fn default() -> Self {
        Self {
            name_color: Color::from_rgb(255, 255, 255),
            box_color: Color::from_hex(0x7d32a8),
            highlighted_box_color: Color::from_hex(0xd32bfc),
            max_distance: 500.0,
            teams: false,
            opacity: 200,
            skeleton: false,
        }
    }
}

pub struct EspContext {
    highlighted_player: i32
}

pub fn esp(game: &Game, overlay: &mut Overlay, config: &EspConfig, aimbot_context: &AimbotContext) {
    let game_info = match game.game_info.as_ref() {
        Some(info) => info,
        None => return
    };

    let mut players = game_info.players.clone();
    let local_origin = &game_info.local_position;
    // sort players
    players.sort_by(|a, b|
        (a.origin - local_origin).length().partial_cmp(
            &(b.origin - local_origin).length()).unwrap_or(Ordering::Equal));
    players.reverse();

    for player in &players {
        if player.character_id == game_info.local_player.character_id {
            continue;
        }
        if !config.teams && player.team == game_info.local_player.team {
            continue;
        }
        let distance = units_to_m((game_info.local_player.origin - player.origin).length());
        if distance > 350.0 {
            continue;
        }
        let highlighted = player.character_id == aimbot_context.aim_lock_player_id.unwrap_or(-1);
        draw_esp(&game, overlay, &config, player, highlighted);
    }
}

pub fn draw_esp(game: &Game, mut overlay: &mut Overlay, config: &EspConfig, player: &Player, highlighted: bool) {
    let game_info = match game.game_info.as_ref() {
        Some(info) => info,
        None => return
    };

    let distance = units_to_m((game_info.local_player.origin - player.origin).length());
    if distance > config.max_distance {
        return;
    }

    let bbox = unwrap_or_return!(player.get_bounding_box(&game));

    let left_x = bbox.0.x;
    let bottom_y = bbox.0.y;
    let right_x = bbox.1.x;
    let top_y = bbox.1.y;

    let width = right_x - left_x;
    let height = bottom_y - top_y;

    // outline
    overlay.draw_box(
        Vector2 {
            x: left_x,
            y: bottom_y,
        },
        Vector2 {
            x: right_x,
            y: top_y,
        },
        Color::from_rgb(0, 0, 0).opacity(config.opacity),
        3.0,
        0.0,
        false,
    );
    // bbox
    overlay.draw_box(
        Vector2 {
            x: left_x,
            y: bottom_y,
        },
        Vector2 {
            x: right_x,
            y: top_y,
        },
        if highlighted { config.highlighted_box_color } else { config.box_color }.opacity(config.opacity),
        1.0,
        0.0,
        false,
    );


    if distance < 150.0 {
        // Draw name
        overlay.draw_text(
            Vector2 { x: left_x + (width / 2.0), y: top_y - 15.0 },
            &player.name,
            config.name_color.opacity(config.opacity),
            TextStyle::Shadow,
            Font::Verdana,
            0.0,
            true,
        );
    }

    let health_color = Color::from_hsv((player.health as f32 / 127.0) * 120.0, 45.0, 100.0).opacity(config.opacity);
    // health bar
    overlay.draw_box(
        Vector2 { x: left_x - 6.0, y: bottom_y + 1.0 }, // bottom left
        Vector2 { x: left_x - 2.0, y: top_y - 1.0 }, // top right
        Color::from_rgba(0, 0, 0, 180).opacity(config.opacity / 2),
        -1.0,
        0.0,
        true,
    );
    overlay.draw_box(
        Vector2 { x: left_x - 5.0, y: bottom_y }, // bottom left
        Vector2 { x: left_x - 3.0, y: bottom_y - ((height) * (player.health as f32 / 127.0)) }, // top right
        health_color,
        -1.0,
        0.0,
        true,
    );

    // draw flags
    let mut flag_offset = -3.0;
    let flag_height = 8.0;

    let mut draw_flag = |text: String, color: Color| {
        overlay.draw_text(Vector2 { x: right_x + 3.0, y: top_y + flag_offset }, &text, color, TextStyle::Outlined, Font::Pixel, 0.0, false);
        flag_offset += flag_height;
    };

    draw_flag(format!("{}m", distance.round()), Color::from_hex(0x32a3bf).opacity(config.opacity));
    match player.stance {
        CharacterStance::STANDING => draw_flag("S".to_string(), Color::from_hex(0x1fdb1f).opacity(config.opacity)),
        CharacterStance::CROUCHING => draw_flag("C".to_string(), Color::from_hex(0x1f9cdb).opacity(config.opacity)),
        CharacterStance::CRAWLING => draw_flag("P".to_string(), Color::from_hex(0xdb931f).opacity(config.opacity)),
        CharacterStance::DOWNED => draw_flag("D".to_string(), Color::from_hex(0xa83232).opacity(config.opacity))
    }
    draw_flag(format!("{}", player.team), Color::from_rgb(17, 161, 250).opacity(config.opacity));

    if config.skeleton {
        draw_skeleton(&game, &mut overlay, &player, Color::from_rgb(255, 255, 255).opacity(config.opacity), 1.0);
    }
}

pub fn draw_skeleton(game: &Game, overlay: &mut Overlay, player: &Player, color: Color, thickness: f32) {
    for (bone1, bone2) in crate::sdk::bone::BONE_CONNECTIONS {
        let pos1 = unwrap_or_return!(game.world_to_screen(&unwrap_or_return!(player.get_bone_position(&game, *bone1).ok())));
        let pos2 = unwrap_or_return!(game.world_to_screen(&unwrap_or_return!(player.get_bone_position(&game, *bone2).ok())));
        overlay.draw_line(pos1, pos2, color, thickness);
    }
    // draw head line (the head bone is really low)
    let trans = Vector3 { x: 0.0, y: 0.0, z: 5.0 };
    let pos1 = unwrap_or_return!(game.world_to_screen(&unwrap_or_return!(player.get_bone_position(&game, Bone::Head).ok())));
    let pos2 = unwrap_or_return!(game.world_to_screen(&(unwrap_or_return!(player.get_bone_position(&game, Bone::Head).ok()) + trans)));
    overlay.draw_line(pos1, pos2, color, thickness);
}