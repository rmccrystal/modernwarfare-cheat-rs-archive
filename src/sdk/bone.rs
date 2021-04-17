#![allow(dead_code)]

use memlib::math::Vector3;
use memlib::memory::{read_memory, Address};
use anyhow::*;
use log::*;
use super::{offsets};
use super::globals;
use serde::{Serialize, Deserialize};

#[repr(u32)]
#[derive(Serialize, Deserialize, PartialEq, Clone, Copy, Debug, Hash, Eq)]
pub enum Bone {
    Head = 7,
    Neck = 6,
    Chest = 5,
    Mid = 4,
    Tummy = 3,

    RightFoot1 = 21,
    RightFoot2 = 22,
    RightFoot3 = 23,
    RightFoot4 = 24,

    LeftFoot1 = 17,
    LeftFoot2 = 18,
    LeftFoot3 = 19,
    LeftFoot4 = 20,

    LeftHand1 = 13,
    LeftHand2 = 14,
    LeftHand3 = 15,
    LeftHand4 = 16,

    RightHand1 = 9,
    RightHand2 = 10,
    RightHand3 = 11,
    RightHand4 = 12
}

use Bone::*;
use std::mem::size_of;
use crate::sdk::encryption::get_bone_index;
use offsets::bones;
use bones::INDEX_STRUCT_SIZE;
use crate::sdk::globals::CLIENT_INFO;

// Bone connections for a skeleton ESP
pub static BONE_CONNECTIONS: &[(Bone, Bone)] = &[
    (Head, Neck),
    (Neck, Chest),
    (Chest, Mid),
    (Mid, Tummy),

    (Tummy, LeftFoot1),
    (LeftFoot1, LeftFoot2),
    (LeftFoot2, LeftFoot3),
    (LeftFoot3, LeftFoot4),

    (Tummy, RightFoot1),
    (RightFoot1, RightFoot2),
    (RightFoot2, RightFoot3),
    (RightFoot3, RightFoot4),

    (Neck, LeftHand1),
    (LeftHand1, LeftHand2),
    (LeftHand2, LeftHand3),
    (LeftHand3, LeftHand4),

    (Neck, RightHand1),
    (RightHand1, RightHand2),
    (RightHand2, RightHand3),
    (RightHand3, RightHand4),
];


pub fn get_bone_position(entity_num: i32, bone: u32) -> Result<Vector3> {
    let bone_base = globals::BONE_BASE.get().context("Called get_bone_position without bone_base")?;

    // let bone_ptr_index: u16 = read_memory(addresses.game_base_address + offsets::bones::INDEX_ARRAY + (entity_num as u64 * size_of::<u16>() as u64));
    let bone_index = get_bone_index(entity_num as _, globals::GAME_BASE_ADDRESS.get());

    let bone_ptr: Address = read_memory(bone_base + (bone_index * INDEX_STRUCT_SIZE as u64) + 0xC0);
    if bone_ptr == 0 {
        bail!("Could not find bone_ptr");
    }
    trace!("Found bone_ptr: 0x{:X}", bone_ptr);

    let bone_pos: Vector3 = read_memory(bone_ptr as u64 + (bone as u64 * 0x20) + 0x10);
    if bone_pos.is_zero() {
        bail!("Could not find bone_pos");
    }

    let client_info = CLIENT_INFO.get().ok_or(anyhow!("Could not find client_info_base"))?;
    let base_pos: Vector3 = read_memory(client_info + bones::BASE_POS);

    Ok(bone_pos + base_pos)
}