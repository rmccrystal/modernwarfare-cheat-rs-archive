#![allow(non_camel_case_types, dead_code)]


use memlib::memory::{Address};
use memlib::math::{Vector3, Vector2};


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
    pub name: [u8; 0x24],
    unk1: [u8; 0x24],
    unk2: [u8; 0x40],
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
