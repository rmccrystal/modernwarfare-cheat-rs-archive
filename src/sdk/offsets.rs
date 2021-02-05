#![allow(dead_code)]

use memlib::memory::Address;

pub const REFDEF: Address = 0x1627D8D0;

pub const NAME_ARRAY: Address = 0x16288B28;
pub const NAME_LIST_OFFSET: Address = 0x4C70;

pub const CAMERA_POINTER: Address = 0x131172C0;
pub const CAMERA_OFFSET: Address = 0x1D8;

pub const INDEX_ARRAY: Address = 0x4D58F90;

pub const LOCAL_INDEX_POINTER: Address = 0x6ECE8;
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

    pub const SIZE: usize = 0x3A28;
    pub const VALID: Address = 0x12C8;
    pub const POS_PTR: Address = 0x1538;
    pub const TEAM: Address = 0x8A0;
    pub const ENTITY_NUM: Address = 0x4B4;
    pub const STANCE: Address = 0x5AC;
    pub const DEATH: Address = 0x11A8;
    pub const ADS: Address = 0xCFC;
    pub const RELOAD: Address = 0xA26;
}

pub mod client_info {
    use memlib::memory::Address;

    pub const ENCRYPTED_PTR: Address = 0x1627AEA8;
    pub const REVERSED_ADDRESS: Address = 0x5B430EA;
    pub const DISPLACEMENT: Address = 0xD;
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