#![allow(non_camel_case_types, dead_code)]


use memlib::memory::{Address};

#[repr(C)]
#[derive(Clone)]
pub struct character_info
{
    unk1: [u8; 0x2C],
    pub info_valid: i32,
    unk2: [u8; 0x59C],
    pub entity_num: i32,
    unk3: [u8; 0xC0],
    pub stance: CharacterStance,
    unk9: [u8; 0x7DC],
    pub position_pointer: Address,
    unk4: [u8; 0x398],
    pub team: i32,
    unk5: [u8; 0x280C],
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
