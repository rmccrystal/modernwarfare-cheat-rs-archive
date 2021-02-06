#![allow(dead_code)]

use memlib::math::Vector3;
use memlib::memory::{read_memory, Address};
use anyhow::*;
use log::*;
use super::{offsets};
use crate::sdk::{Game, GameAddresses};
use serde::{Serialize, Deserialize};

#[repr(u32)]
#[derive(Serialize, Deserialize, PartialEq, Clone, Copy, Debug)]
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


pub fn get_bone_position(addresses: &GameAddresses, entity_num: i32, bone_index: u32) -> Result<Vector3> {
    let bone_ptr_index: u16 = read_memory(addresses.game_base_address + offsets::INDEX_ARRAY + (entity_num as u64 * std::mem::size_of::<u16>() as u64));
    trace!("bone_ptr_index: 0x{:X}", bone_ptr_index);

    let bone_base = addresses.bone_base.ok_or(anyhow!("Could not find bone_base"))?;

    let bone_ptr: Address = read_memory(bone_base + (bone_ptr_index as u64 * offsets::bones::INDEX_STRUCT_SIZE as u64) + 0xC0);
    if bone_ptr == 0 {
        bail!("Could not find bone_ptr");
    }
    trace!("Found bone_ptr: 0x{:X}", bone_ptr);

    let mut bone_pos: Vector3 = read_memory(bone_ptr as u64 + (bone_index as u64 * 0x20) + 0x10);
    if bone_pos.is_zero() {
        bail!("Could not find bone_pos");
    }

    let client_info = addresses.client_info_base.ok_or(anyhow!("Could not find client_info_base"))?;
    let base_pos: Vector3 = read_memory(client_info + offsets::bones::BASE_POS);

    bone_pos = bone_pos + base_pos;

    Ok(bone_pos)
}