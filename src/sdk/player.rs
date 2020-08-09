#![allow(dead_code)]

use super::structs::{character_info, name_t};
use memlib::math::{Vector3};
use memlib::memory::{read_memory};
use super::{Game, bone};
use crate::sdk::bone::Bone;
use crate::sdk::structs::CharacterStance;

#[derive(Debug, Clone)]
pub struct Player {
    pub name: String,
    pub origin: Vector3,
    pub team: i32,
    pub character_id: i32,
    pub stance: CharacterStance,
    pub health: i32,
}

impl Player {
    pub fn new(game: &Game, char_info: &character_info) -> Self {
        let name_struct = game.get_name_struct(char_info.entity_num as u32);
        Self {
            origin: char_info.get_position(),
            character_id: char_info.entity_num,
            team: char_info.team,
            name: name_struct.get_name(),
            stance: char_info.stance,
            health: name_struct.health,
        }
    }

    pub fn get_bone_position(&self, game: &Game, bone_index: Bone) -> Result<Vector3, Box<dyn std::error::Error>> {
        bone::get_bone_position(&game, self.character_id, unsafe { std::mem::transmute(bone_index)})
    }

    pub fn get_head_position(&self) -> Vector3 {
        let delta_z = match self.stance {
            CharacterStance::STANDING => 60.0,
            CharacterStance::CROUCHING => 40.0,
            CharacterStance::CRAWLING => 10.0,
            CharacterStance::DOWNED => 20.0,
        };
        self.origin + Vector3 { x: 0.0, y: 0.0, z: delta_z }
    }
}


impl character_info {
    pub fn get_position(&self) -> Vector3 {
        read_memory(self.position_pointer + 0x40)
    }

    pub fn get_bone_position(&self, game: &Game, bone_index: Bone) -> Result<Vector3, Box<dyn std::error::Error>> {
        bone::get_bone_position(&game, self.entity_num, unsafe { std::mem::transmute(bone_index)})
    }
}

impl name_t {
    pub fn get_name(&self) -> String {
        String::from_utf8(self.name.to_vec())
            .unwrap_or("".to_string())
            .trim_matches(char::from(0))
            .to_string()
    }
}