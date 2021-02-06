use std::cmp;
use std::cmp::Ordering;

use memlib::math::{Vector2, Vector3};
use memlib::unwrap_or_return;
use rand::{RngCore, SeedableRng};

use crate::config::Config;
use crate::hacks::aimbot::AimbotContext;
use crate::sdk::{Game, Player, units_to_m, GameInfo};
use crate::sdk::bone::Bone;
use crate::sdk::structs::CharacterStance;
use serde::{Serialize, Deserialize};

use imgui;
use memlib::overlay::{Color, Draw};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, imgui_ext::Gui)]
pub struct EspConfig {
    #[imgui(checkbox(label = "ESP Enabled"))]
    enabled: bool,
    name_color: Color,
    box_color: Color,
    highlighted_box_color: Color,
    #[imgui(slider(min = 0.0, max = 2000.0, label = "ESP max distance"))]
    max_distance: f32,
    #[imgui(checkbox(label = "ESP teams"))]
    teams: bool,
    opacity: u8,
    skeleton: bool,
    #[imgui(slider(min = 0.0, max = 500.0, label = "ESP extra info cutoff"))]
    extra_info_distance: f32,
}

impl EspConfig {
    pub fn default() -> Self {
        Self {
            name_color: Color::from_rgb(255, 255, 255),
            box_color: Color::from_hex(0x7d32a8),
            highlighted_box_color: Color::from_hex(0xd32bfc),
            max_distance: 500.0,
            teams: true,
            opacity: 200,
            skeleton: false,
            extra_info_distance: 200.0,
            enabled: true,
        }
    }
}

pub struct EspContext {
    highlighted_player: i32
}

pub fn esp(game_info: &GameInfo, overlay: &mut impl Draw, config: &Config, aimbot_context: &AimbotContext) {
    let esp_config = &config.esp_config;

    if !esp_config.enabled {
        return
    }

    let mut players = game_info.players.clone();
    let local_origin = &game_info.local_position;
    // sort players
    // players.sort_by(|a, b|
    //     (a.origin - local_origin).length().partial_cmp(
    //         &(b.origin - local_origin).length()).unwrap_or(Ordering::Equal));
    // players.reverse()
    // ;

    for player in &players {
        if player.id == game_info.local_player.id {
            continue;
        }
        if !esp_config.teams && player.is_teammate(&game_info, &config.friends) {
            continue;
        }
        let distance = units_to_m((game_info.local_player.origin - player.origin).length());
        if distance > 350.0 {
            continue;
        }
        let highlighted = player.id == aimbot_context.aim_lock_player_id.unwrap_or(-1);
        draw_esp(&game_info, overlay, &esp_config, player, highlighted);
    }
}

pub fn draw_esp(game_info: &GameInfo, overlay: &mut impl Draw, config: &EspConfig, player: &Player, highlighted: bool) {
    use memlib::overlay::*;

    let distance = units_to_m((game_info.local_player.origin - player.origin).length());
    if distance > config.max_distance {
        return;
    }

    let bbox = unwrap_or_return!(player.get_bounding_box(&game_info.game));

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
        BoxOptions::default()
            .color(Color::from_rgb(0, 0, 0).opacity(config.opacity))
            .width(3.0)
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
        BoxOptions::default()
            .color(if highlighted { config.highlighted_box_color } else { config.box_color }.opacity(config.opacity))
    );

    if distance < config.extra_info_distance {
        // Draw name
        overlay.draw_text(
            Vector2 { x: left_x + (width / 2.0), y: top_y - 15.0 },
            &player.name,
            TextOptions::default()
                .color(config.name_color.opacity(config.opacity))
                .style(TextStyle::Shadow)
                .font(Font::Verdana)
                .centered_horizontal(true)
        );

        let health_color = Color::from_hsv((player.health as f32 / 127.0) * 120.0, 45.0, 100.0).opacity(config.opacity);
        // health bar
        overlay.draw_box(
            Vector2 { x: left_x - 6.0, y: bottom_y + 1.0 }, // bottom left
            Vector2 { x: left_x - 2.0, y: top_y - 1.0 }, // top right
            BoxOptions::default()
                .color(Color::from_rgba(0, 0, 0, 180).opacity(config.opacity / 2))
                .filled(true)
        );
        overlay.draw_box(
            Vector2 { x: left_x - 5.0, y: bottom_y }, // bottom left
            Vector2 { x: left_x - 3.0, y: bottom_y - ((height) * (player.health as f32 / 127.0)) }, // top right
            BoxOptions::default()
                .color(health_color)
                .filled(true)
        );

    }

    // draw flags
    let mut flag_offset = -3.0;
    let flag_height = 8.0;

    let mut draw_flag = |text: &str, color: Color| {
        overlay.draw_text(
            Vector2 { x: right_x + 3.0, y: top_y + flag_offset },
            text,
            TextOptions::default()
                .color(color)
                .style(TextStyle::Outlined)
                .font(Font::Pixel)
        );
        flag_offset += flag_height;
    };

    let team_color = Color::new(rand::rngs::SmallRng::seed_from_u64(player.team as u64).next_u32()).opacity(255);

    // Draw distance
    draw_flag(&format!("{}m", distance.round()), team_color.opacity(config.opacity));

    if distance < config.extra_info_distance {
        match player.stance {
            CharacterStance::Standing => {}//draw_flag("S", Color::from_hex(0x1fdb1f).opacity(config.opacity)),
            CharacterStance::Crouching => draw_flag("C", Color::from_hex(0x1f9cdb).opacity(config.opacity)),
            CharacterStance::Crawling => draw_flag("P", Color::from_hex(0xdb931f).opacity(config.opacity)),
            CharacterStance::Downed => draw_flag("D", Color::from_hex(0xa83232).opacity(config.opacity))
        }
        if player.ads {
            draw_flag("ADS", Color::from_hex(0xA75A97).opacity(config.opacity));
        }
        if player.reloading {
            draw_flag("R", Color::from_hex(0xA75A97).opacity(config.opacity));
        }
        // draw_flag(&format!("{}", player.team), team_color.opacity(config.opacity));
    }

    if config.skeleton {
        draw_skeleton(&game_info.game, overlay, &player, Color::from_rgb(255, 255, 255).opacity(config.opacity), 1.0);
    }
}

pub fn draw_skeleton(game: &Game, overlay: &mut impl Draw, player: &Player, color: Color, thickness: f32) {
    use memlib::overlay::*;

    for (bone1, bone2) in crate::sdk::bone::BONE_CONNECTIONS {
        let pos1 = unwrap_or_return!(game.world_to_screen(&unwrap_or_return!(player.get_bone_position(&game, *bone1).ok())));
        let pos2 = unwrap_or_return!(game.world_to_screen(&unwrap_or_return!(player.get_bone_position(&game, *bone2).ok())));
        overlay.draw_line(pos1, pos2, LineOptions::default().color(color).width(thickness));
    }

    // Draw head circle
    let trans = Vector3 { x: 0.0, y: 0.0, z: 4.0 };
    let head_pos = unwrap_or_return!(game.world_to_screen(&(unwrap_or_return!(player.get_bone_position(&game, Bone::Head).ok()) + trans)));
    overlay.draw_circle(head_pos, 2.5, CircleOptions::default().color(color));
}