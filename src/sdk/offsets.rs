#![allow(dead_code)]

use memlib::memory::Address;

pub const GAMEMODE: Address = 0x139699E8;

pub const REFDEF: Address = 0x13BD7A50;

pub const NAME_ARRAY: Address = 0x13BE33A8;
pub const NAME_LIST_OFFSET: Address = 0x4C70;

pub const CAMERA_POINTER: Address = 0x110BF400;
pub const CAMERA_OFFSET: Address = 0x1D8;

pub const INDEX_ARRAY: Address = 0x11EB6C40;

pub const LOCAL_INDEX_POINTER: Address = 0x24698;
pub const LOCAL_INDEX_OFFSET: Address = 0x1F4;

pub const NO_RECOIL: Address = 0x0039D94;

pub mod entity {
    use memlib::memory::Address;

    // pub const ENCRYPTED_PTR: Address = 0x112F7598;
    // pub const REVERSED_ADDRESS: Address = 0x4E9E22E;
    // pub const DISPLACEMENT: Address = 0xD;
    // pub const SIZE: usize = 0x310;
}

pub mod client_info {
    use memlib::memory::Address;

    pub const ENCRYPTED_PTR: Address = 0x13BD5D18;
    pub const REVERSED_ADDRESS: Address = 0x4BC71A2;
    pub const DISPLACEMENT: Address = 0xD;
}

pub mod client_base {
    use memlib::memory::Address;

    pub const BASE_OFFSET: Address = 0x97BB8;
    pub const BASE_REVERSED_ADDR: Address = 0x4BC71F1;
    pub const BASE_DISPLACEMENT: Address = 0xF;
    pub const SIZE: usize = 0x3A98;
}

pub mod bones {
    use memlib::memory::Address;

    pub const ENCRYPTED_PTR: Address = 0x11EB8008;
    pub const REVERSED_ADDRESS: Address = 0x4BC7291;
    pub const DISPLACEMENT: Address = 0xD;
    pub const BASE_POS: Address = 0x40C;
    pub const INDEX_STRUCT_SIZE: usize = 0x150;
}