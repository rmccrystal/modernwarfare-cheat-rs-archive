#![allow(dead_code)]

use memlib::memory::Address;

pub const REFDEF: Address = 0x165970B0;

pub const NAME_ARRAY: Address = 0x165A2308;
pub const NAME_LIST_OFFSET: Address = 0x4C70;

pub const CAMERA_POINTER: Address = 0x133E4040;
pub const CAMERA_OFFSET: Address = 0x1D8;

pub const INDEX_ARRAY: Address = 0x4D58F90;

pub const LOCAL_INDEX_POINTER: Address = 0x836C8;
pub const LOCAL_INDEX_OFFSET: Address = 0x1F4;

pub const NO_RECOIL: Address = 0x93A1C;

pub mod entity {
    use memlib::memory::Address;

    // pub const ENCRYPTED_PTR: Address = 0x112F7598;
    // pub const REVERSED_ADDRESS: Address = 0x4E9E22E;
    // pub const DISPLACEMENT: Address = 0xD;
    // pub const SIZE: usize = 0x310;
}

pub mod character_info {
    use memlib::memory::Address;

    pub const SIZE: usize = 0x3A68;
    pub const VALID: Address = 0x6DC;
    pub const POS_PTR: Address = 0xA0;
    pub const TEAM: Address = 0x37C0;
    pub const STANCE: Address = 0xB94;
    pub const DEATH: Address = 0x3994;
    pub const ADS: Address = 0xBD4;
    pub const RELOAD: Address = 0x644;
}

pub mod client_info {
    use memlib::memory::Address;

    pub const ENCRYPTED_PTR: Address = 0x16594688;
    pub const REVERSED_ADDRESS: Address = 0x5c710f6;
    pub const DISPLACEMENT: Address = 0x7;
}

pub mod client_base {
    use memlib::memory::Address;

    pub const BASE_OFFSET: Address = 0x98C78;
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