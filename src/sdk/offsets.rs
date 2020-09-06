#![allow(dead_code)]

use memlib::memory::Address;

pub const REFDEF: Address = 0x13F9EB40;

pub const NAME_ARRAY: Address = 0x13FAA508;
pub const NAME_LIST_OFFSET: Address = 0x4C70;

pub const CAMERA_POINTER: Address = 0x113B2980;
pub const CAMERA_OFFSET: Address = 0x1D8;

pub const INDEX_ARRAY: Address = 0x4D58F90;

pub const LOCAL_INDEX_POINTER: Address = 0x26AD0;
pub const LOCAL_INDEX_OFFSET: Address = 0x1F4;

pub const NO_RECOIL: Address = 0x300374E8;

pub mod entity {
    use memlib::memory::Address;

    // pub const ENCRYPTED_PTR: Address = 0x112F7598;
    // pub const REVERSED_ADDRESS: Address = 0x4E9E22E;
    // pub const DISPLACEMENT: Address = 0xD;
    // pub const SIZE: usize = 0x310;
}

pub mod character_info {
    use memlib::memory::Address;

    pub const SIZE: usize = 0x3A70;
    pub const VALID: Address = 0x12C8;
    pub const POS_PTR: Address = 0xF58;
    pub const TEAM: Address = 0x4C8;
    pub const ENTITY_NUM: Address = 0x4B4;
    pub const STANCE: Address = 0x7F8;
    pub const DEATH: Address = 0x470;
    pub const ADS: Address = 0xCFC;
    pub const RELOAD: Address = 0xA26;
}

pub mod client_info {
    use memlib::memory::Address;

    pub const ENCRYPTED_PTR: Address = 0x13F9CDE8;
    pub const REVERSED_ADDRESS: Address = 0x4D470EA;
    pub const DISPLACEMENT: Address = 0x5;
}

pub mod client_base {
    use memlib::memory::Address;

    pub const BASE_OFFSET: Address = 0x97B98;
    pub const BASE_REVERSED_ADDR: Address = 0x4D47115;
    pub const BASE_DISPLACEMENT: Address = 0x15;
}

pub mod bones {
    use memlib::memory::Address;

    pub const ENCRYPTED_PTR: Address = 0x1228D918;
    pub const REVERSED_ADDRESS: Address = 0x4DF2218;
    pub const DISPLACEMENT: Address = 0x17;
    pub const BASE_POS: Address = 0x6D2E4;
    pub const INDEX_STRUCT_SIZE: usize = 0x150;
}