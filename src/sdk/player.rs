#![allow(dead_code)]

use super::structs::{character_info, name_t};
use memlib::math::{Vector3, Vector2};
use memlib::memory::{read_memory};
use super::{Game, bone};
use crate::sdk::bone::Bone;
use crate::sdk::structs::CharacterStance;
use log::*;

#[derive(Debug, Clone)]
pub struct Player {
    pub name: String,
    pub origin: Vector3,
    pub team: i32,
    pub character_id: i32,
    pub stance: CharacterStance,
    pub health: i32,
    pub ads: bool,
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
            ads: char_info.ads == 1,
            // stance: CharacterStance::STANDING,
            health: name_struct.health,
        }
    }

    pub fn get_bone_position(&self, game: &Game, bone: Bone) -> Result<Vector3, Box<dyn std::error::Error>> {
        let pos = bone::get_bone_position(&game, self.character_id, unsafe { std::mem::transmute(bone) })?;
        let distance_from_origin = crate::sdk::units_to_m((pos - self.origin).length());
        if distance_from_origin > 2.0 {
            warn!("bone {:?} ({}) {}m away from {}'s origin was read", bone, unsafe { std::mem::transmute::<Bone, i32>(bone) }, distance_from_origin, self.name);
            return Err("Bone was too far away from player body".into());
        }
        Ok(pos)
    }

    pub fn get_head_position(&self, game: &Game) -> Vector3 {
        match self.get_bone_position(&game, Bone::Head) {
            Ok(pos) => pos,
            Err(err) => {
                // warn!("Error getting head bone position for {}: {}; using fallback", self.name, err);
                self.assume_head_position()
            }
        }
    }

    /// Gets the bounding box of the player from bottom left to top right
    /// Returns None if world_to_screen fails
    pub fn get_bounding_box(&self, game: &Game) -> Option<(Vector2, Vector2)> {
        match self.get_bounding_box_bones(&game) {
            Some(val) => Some(val),
            None => self.get_boudning_box_fallback(&game)
        }
    }

    /// Gets the player bounding box using bone locations
    fn get_bounding_box_bones(&self, game: &Game) -> Option<(Vector2, Vector2)> {
        // THe bones kind of flicker atm, so we will just use fallback
        return None;
        let bones = vec![ Bone::Head, Bone::Neck, Bone::Chest, Bone::Mid, Bone::Tummy,
        Bone::RightFoot1, Bone::RightFoot2, Bone::RightFoot3, Bone::RightFoot4,
        Bone::LeftFoot1, Bone::LeftFoot2, Bone::LeftFoot3, Bone::LeftFoot4,
        Bone::LeftHand1, Bone::LeftHand2, Bone::LeftHand3, Bone::LeftHand4,
        Bone::RightHand1, Bone::RightHand2, Bone::RightHand3, Bone::RightHand4 ];
        let mut bone_locations = Vec::new();

        for bone in bones {
            bone_locations.push(game.world_to_screen(&self.get_bone_position(&game, bone).ok()?)?);
        }

        Some(memlib::util::get_boudning_box(bone_locations))
    }

    fn get_boudning_box_fallback(&self, game: &Game) -> Option<(Vector2, Vector2)> {
        let head_pos = self.get_head_position(&game);
        let head_pos = game.world_to_screen(&(head_pos + Vector3{x: 0.0, y: 0.0, z: 10.0}))?;
        let feet_pos = game.world_to_screen(&(self.origin))?;

        let height = feet_pos.y - head_pos.y;
        let width = match self.stance {
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

        Some((
            Vector2{x: left_x, y: bottom_y},
            Vector2{x: right_x, y: top_y}
        ))
    }

    pub fn assume_head_position(&self) -> Vector3 {
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
        bone::get_bone_position(&game, self.entity_num, unsafe { std::mem::transmute(bone_index) })
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