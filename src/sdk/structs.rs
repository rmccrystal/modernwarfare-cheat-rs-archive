#![allow(non_camel_case_types, dead_code)]


use memlib::memory::{Address};
use memlib::math::{Vector3, Vector2};


#[repr(C)]
#[derive(Clone)]
pub struct character_info
{
    unk1: [u8; 0x1cc],
    pub team: i32,
    unk2: [u8; 0x434],
    pub entity_num: i32,
    unk3: [u8; 0x5c4],
    pub stance: CharacterStance,
    unk4: [u8; 0xa00],
    pub position_pointer: Address,
    unk5: [u8; 0x24a0],
} // Size: 0x3a78

impl character_info {
    pub fn is_valid(&self) -> bool {
        let stance: i32 = unsafe {std::mem::transmute(self.stance)};
        self.entity_num >= 0 && self.entity_num <= 155
            && self.team > 0 && self.team < 255
            && stance >= 0 && stance <= 3
            && self.position_pointer != 0
            && self.position_pointer < 0xFFFFFFFFFFFFFF
    }
}

#[repr(i32)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum CharacterStance {
    STANDING = 0,
    CROUCHING = 1,
    CRAWLING = 2,
    DOWNED = 3,
}

impl ToString for CharacterStance {
    fn to_string(&self) -> String {
        String::from(match self {
            CharacterStance::STANDING => "Standing",
            CharacterStance::CROUCHING => "Crouching",
            CharacterStance::CRAWLING => "Crawling",
            CharacterStance::DOWNED => "Downed",
        })
    }
}

#[repr(C)]
pub struct name_t
{
    pub entity_index: u32,
    pub name: [u8; 36],
    unk1: [u8; 0x24],
    pub health: i32,
}

#[repr(C)]
#[derive(Clone, Debug)]
pub struct refdef_view {
    pub tan_half_fov: Vector2,
    unk6: [u8; 0xC],
    pub axis: [Vector3; 3],
}

#[repr(C)]
#[derive(Clone, Debug)]
pub struct refdef_t {
    pub x: i32,
    pub y: i32,
    pub width: i32,
    pub height: i32,
    pub view: refdef_view,
}
