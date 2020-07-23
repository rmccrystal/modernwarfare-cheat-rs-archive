#![allow(non_camel_case_types, dead_code)]

use memlib::math::{Vector3, Angles2};
use memlib::memory::{Pointer, Address};

#[repr(C)]
pub struct character_info
{
    unk1: [u8; 0x58],
    pub entity_num: i32,
    unk2: [u8; 0x534],
    pub position_pointer: Address,
    unk3: [u8; 0x1CC],
    pub info_valid: i32,
    unk4: [u8; 0x28],
    pub team: i32,
    unk5: [u8; 0x2B20],
    pub stance: CharacterStance,
    unk6: [u8; 0x75C],
    pub view_angles: Angles2,
    unk7: [u8; 0x40],
} // Size: 0x3A60

#[repr(i32)]
#[derive(Copy, Clone, Debug)]
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
