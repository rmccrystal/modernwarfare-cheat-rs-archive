use crate::sdk::{Game, Player, units_to_m};
use crate::sdk::structs::CharacterStance;
use memlib::overlay::{Overlay, OverlayInterface, Color, TextStyle, Font};
use memlib::math::{Vector3, Vector2};
use memlib::unwrap_or_return;

#[derive(Copy, Clone, Debug)]
pub struct EspConfig {
    name_color: Color,
    box_color: Color,
    max_distance: f32,
}

impl EspConfig {
    pub fn default() -> Self {
        Self { name_color: Color::from_rgb(255, 255, 255), box_color: Color::from_hex(0x7d32a8), max_distance: 500.0 }
    }
}

pub struct EspContext {
    highlighted_player: i32
}

pub fn esp(game: &Game, overlay: &mut Overlay, config: &EspConfig) {
    let game_info = match game.game_info.as_ref() {
        Some(info) => info,
        None => return
    };

    // todo: sort by distance
    for player in &game_info.players.clone() {
        if player.character_id == game_info.local_player.character_id {
            continue;
        }
        if player.team == game_info.local_player.team {
            continue;
        }
        draw_esp(&game, overlay, &config, player);
    }
}

pub fn draw_esp(game: &Game, overlay: &mut Overlay, config: &EspConfig, player: &Player) {
    let game_info = match game.game_info.as_ref() {
        Some(info) => info,
        None => return
    };

    let distance = units_to_m((game_info.local_player.origin - player.origin).length());
    if distance > config.max_distance {
        return;
    }

    let head_pos = player.get_head_position();
    let head_pos = unwrap_or_return!(game.world_to_screen(&(head_pos + Vector3{x: 0.0, y: 0.0, z: 10.0})));
    let feet_pos = unwrap_or_return!(game.world_to_screen(&(player.origin)));

    let height = feet_pos.y - head_pos.y;
    let width = match player.stance {
        CharacterStance::STANDING => height / 4.0,
        CharacterStance::CROUCHING => height / 2.5,
        CharacterStance::DOWNED => height * 2.0,
        CharacterStance::CRAWLING => height * 2.5,
    };

    let size = 1.0;

    let left_x = feet_pos.x - width - size;
    let right_x = feet_pos.x + width + size;
    let top_y = head_pos.y - size;
    let bottom_y = feet_pos.y + size;

    overlay.draw_box(
        Vector2 {
            x: left_x,
            y: bottom_y
        },
        Vector2 {
            x: right_x,
            y: top_y
        },
        Color::from_rgb(0, 0, 0),
        3.0,
        0.0,
        false
    );
    overlay.draw_box(
        Vector2 {
            x: left_x,
            y: bottom_y
        },
        Vector2 {
            x: right_x,
            y: top_y
        },
        config.box_color,
        1.0,
        0.0,
        false
    );


    // Draw name
    overlay.draw_text(
        Vector2{x: left_x + width, y: top_y - 15.0},
        &player.name,
        config.name_color,
        TextStyle::Shadow,
        Font::Verdana,
        0.0,
        true
    );

    let mut flag_offset = 0.0;
    let flag_height = 8.0;

    let mut draw_flag = |text: String, color: Color| {
        overlay.draw_text(Vector2{x: right_x + 2.0, y: top_y + flag_offset}, &text, color, TextStyle::Outlined, Font::Pixel, 0.0, false);
        flag_offset += flag_height;
    };

    draw_flag(format!("{}m", distance.round()), Color::from_hex(0x32a3bf));
    draw_flag(format!("{}hp", player.health), Color::from_rgb(255, 255, 255));
    let stance_color = Color::from_hex(0x84e307);
    match player.stance {
        CharacterStance::STANDING => draw_flag("ST".to_string(), stance_color),
        CharacterStance::CROUCHING => draw_flag("CR".to_string(), stance_color),
        CharacterStance::CRAWLING => draw_flag("PR".to_string(), stance_color),
        CharacterStance::DOWNED => draw_flag("DWND".to_string(), Color::from_hex(0xa83232))
    }
}