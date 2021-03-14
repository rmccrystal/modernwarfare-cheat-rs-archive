#![allow(dead_code)]

use memlib::memory::Address;

pub const REFDEF: Address = 0x16501130;

pub const NAME_ARRAY: Address = 0x1650C388;
pub const NAME_LIST_OFFSET: Address = 0x4C70;

pub const CAMERA_POINTER: Address = 0x1334E040;
pub const CAMERA_OFFSET: Address = 0x1D8;

pub const INDEX_ARRAY: Address = 0x4D58F90;

pub const LOCAL_INDEX_POINTER: Address = 0x836C8;
pub const LOCAL_INDEX_OFFSET: Address = 0x1F4;

pub const NO_RECOIL: Address = 0x8C048;

pub mod entity {
    use memlib::memory::Address;

    // pub const ENCRYPTED_PTR: Address = 0x112F7598;
    // pub const REVERSED_ADDRESS: Address = 0x4E9E22E;
    // pub const DISPLACEMENT: Address = 0xD;
    // pub const SIZE: usize = 0x310;
}

pub mod character_info {
    use memlib::memory::Address;

    pub const SIZE: usize = 0x3A88;
    pub const VALID: Address = 0x64C;
    pub const POS_PTR: Address = 0x37E8;
    pub const TEAM: Address = 0x12D8;
    pub const STANCE: Address = 0x958;
    pub const DEAD_1: Address = 0x81C;
    pub const DEAD_2: Address = 0x2BC;
    pub const ADS: Address = 0xBD4; // not working
    pub const RELOAD: Address = 0x644;  // not working
}

pub mod client_info {
    use memlib::memory::Address;

    pub const ENCRYPTED_PTR: Address = 0x164FE708;
    pub const REVERSED_ADDRESS: Address = 0x5c710f6;
    pub const DISPLACEMENT: Address = 0x7;
}

pub mod client_base {
    use memlib::memory::Address;

    pub const BASE_OFFSET: Address = 0x98C88;
    pub const BASE_REVERSED_ADDR: Address = 0x4D47115;
    pub const BASE_DISPLACEMENT: Address = 0x15;
}

pub mod bones {
    use memlib::memory::Address;

    pub const ENCRYPTED_PTR: Address = 0x121F7548;
    pub const REVERSED_ADDRESS: Address = 0x4D471DF;
    pub const DISPLACEMENT: Address = 0xD;
    pub const BASE_POS: Address = 0x6D2E4;
    pub const INDEX_STRUCT_SIZE: usize = 0x150;
}