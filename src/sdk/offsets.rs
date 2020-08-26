#![allow(dead_code)]

use memlib::memory::Address;

pub const REFDEF: Address = 0x13F47D40;

pub const NAME_ARRAY: Address = 0x13F53708;
pub const NAME_LIST_OFFSET: Address = 0x4C70;

pub const CAMERA_POINTER: Address = 0x1135A900;
pub const CAMERA_OFFSET: Address = 0x1D8;

pub const INDEX_ARRAY: Address = 0x1219F4D0;

pub const LOCAL_INDEX_POINTER: Address = 0x1270;
pub const LOCAL_INDEX_OFFSET: Address = 0x1F4;

pub const NO_RECOIL: Address = 0x00000274;

pub mod entity {
    use memlib::memory::Address;

    // pub const ENCRYPTED_PTR: Address = 0x112F7598;
    // pub const REVERSED_ADDRESS: Address = 0x4E9E22E;
    // pub const DISPLACEMENT: Address = 0xD;
    // pub const SIZE: usize = 0x310;
}

pub mod client_info {
    use memlib::memory::Address;

    pub const ENCRYPTED_PTR: Address = 0x13F45FE8;
    pub const REVERSED_ADDRESS: Address = 0x4D050F5;
    pub const DISPLACEMENT: Address = 0xB;
}

pub mod client_base {
    use memlib::memory::Address;

    pub const BASE_OFFSET: Address = 0x97B48;
    pub const BASE_REVERSED_ADDR: Address = 0x4D0514E;
    pub const BASE_DISPLACEMENT: Address = 0x11;
    pub const SIZE: usize = 0x3A20; //
}

pub mod bones {
    use memlib::memory::Address;

    pub const ENCRYPTED_PTR: Address = 0x121A0898;
    pub const REVERSED_ADDRESS: Address = 0x4D051C4;
    pub const DISPLACEMENT: Address = 0x15;
    pub const BASE_POS: Address = 0x644;
    pub const INDEX_STRUCT_SIZE: usize = 0x150;
}