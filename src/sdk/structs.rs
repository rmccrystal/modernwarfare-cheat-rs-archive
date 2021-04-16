#![allow(non_camel_case_types, dead_code)]


use memlib::memory::{Address};
use memlib::math::{Vector3, Vector2};


#[repr(i32)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum CharacterStance {
    Standing = 0,
    Crouching = 1,
    Crawling = 2,
    Downed = 3,
}

impl ToString for CharacterStance {
    fn to_string(&self) -> String {
        String::from(match self {
            CharacterStance::Standing => "Standing",
            CharacterStance::Crouching => "Crouching",
            CharacterStance::Crawling => "Crawling",
            CharacterStance::Downed => "Downed",
        })
    }
}

#[repr(C)]
pub struct Name
{
    pub entity_index: u32,
    pub name: [u8; 0x24],
    unk1: [u8; 0x24],
    unk2: [u8; 0x40],
    pub health: i32,
}

#[repr(C)]
#[derive(Clone, Debug, Copy)]
pub struct RefdefView {
    pub tan_half_fov: Vector2,
    unk6: [u8; 0xC],
    pub axis: [Vector3; 3],
}

#[repr(C)]
#[derive(Clone, Debug, Copy)]
pub struct RefDef {
    pub x: i32,
    pub y: i32,
    pub width: i32,
    pub height: i32,
    pub view: RefdefView,
}
