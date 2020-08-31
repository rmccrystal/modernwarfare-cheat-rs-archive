#![allow(dead_code)]

use memlib::memory::Address;

pub const REFDEF: Address = 0x14034F40;

pub const NAME_ARRAY: Address = 0x14040908;
pub const NAME_LIST_OFFSET: Address = 0x4C70;

pub const CAMERA_POINTER: Address = 0x11447980;
pub const CAMERA_OFFSET: Address = 0x1D8;

pub const INDEX_ARRAY: Address = 0x1228C550;

pub const LOCAL_INDEX_POINTER: Address = 0x15190;
pub const LOCAL_INDEX_OFFSET: Address = 0x1F4;

pub const NO_RECOIL: Address = 0x83840;

pub mod entity {
    use memlib::memory::Address;

    // pub const ENCRYPTED_PTR: Address = 0x112F7598;
    // pub const REVERSED_ADDRESS: Address = 0x4E9E22E;
    // pub const DISPLACEMENT: Address = 0xD;
    // pub const SIZE: usize = 0x310;
}

pub mod character_info {
    use memlib::memory::Address;

    pub const SIZE: usize = 0x3AA0;
    pub const VALID: Address = 0x397C;
    pub const POS_PTR: Address = 0x3968;
    pub const TEAM: Address = 0x574;
    pub const ENTITY_NUM: Address = 0xA08;
    pub const STANCE: Address = 0xC6C;
    pub const ADS: Address = 0xCAC;
}

pub mod client_info {
    use memlib::memory::Address;

    pub const ENCRYPTED_PTR: Address = 0x140331E8;
    pub const REVERSED_ADDRESS: Address = 0x4DF2106;
    pub const DISPLACEMENT: Address = 0x07;
}

pub mod client_base {
    use memlib::memory::Address;

    pub const BASE_OFFSET: Address = 0x97BD8;
    pub const BASE_REVERSED_ADDR: Address = 0x4DF214B;
    pub const BASE_DISPLACEMENT: Address = 0x13;
}

pub mod bones {
    use memlib::memory::Address;

    pub const ENCRYPTED_PTR: Address = 0x1226F898;
    pub const REVERSED_ADDRESS: Address = 0x4DF2218;
    pub const DISPLACEMENT: Address = 0x17;
    pub const BASE_POS: Address = 0x49B3C;
    pub const INDEX_STRUCT_SIZE: usize = 0x150;
}