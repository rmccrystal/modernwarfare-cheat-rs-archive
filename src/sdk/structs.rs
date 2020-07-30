#![allow(non_camel_case_types, dead_code)]


use memlib::memory::{Address};

#[repr(C)]
#[derive(Clone)]
pub struct character_info
{
    unk1: [u8; 0x1E8],
    pub position_pointer: Address,
    unk2: [u8; 0x120],
    pub info_valid: i32,
    pub entity_num: i32,
    unk3: [u8; 0x26C4],
    pub stance: CharacterStance,
    unk4: [u8; 0x830],
    pub team: i32,
    unk5: [u8; 0x880],
} // Size: 0x3A60

#[repr(i32)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum CharacterStance {
    STANDING = 0,
    CROUCHING = 1,
    CRAWLING = 2,
    DOWNED = 3,
}

#[repr(C)]
pub struct name_t
{
    pub entity_index: u32,
    pub name: [u8; 36],
    unk1: [u8; 0x24],
    pub health: i32,
}
