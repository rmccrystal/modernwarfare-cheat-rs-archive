#![allow(dead_code)]

use memlib::memory::Address;

pub const GAMEMODE: Address = 0x13b66ae8;

pub const REFDEF: Address = 0x13DD4B50;

pub const NAME_ARRAY: Address = 0x13DE04A8;
pub const NAME_LIST_OFFSET: Address = 0x4C70;

pub const CAMERA_POINTER: Address = 0x112BC500;
pub const CAMERA_OFFSET: Address = 0x1D8;

pub const INDEX_ARRAY: Address = 0x120B3D40;

pub const LOCAL_INDEX_POINTER: Address = 0x12160;
pub const LOCAL_INDEX_OFFSET: Address = 0x1F4;

pub const NO_RECOIL: Address = 0x0F3A0;

pub mod entity {
    use memlib::memory::Address;

    // pub const ENCRYPTED_PTR: Address = 0x112F7598;
    // pub const REVERSED_ADDRESS: Address = 0x4E9E22E;
    // pub const DISPLACEMENT: Address = 0xD;
    // pub const SIZE: usize = 0x310;
}

pub mod client_info {
    use memlib::memory::Address;

    pub const ENCRYPTED_PTR: Address = 0x13DD2E18;
    pub const REVERSED_ADDRESS: Address = 0x4D611BF;
    pub const DISPLACEMENT: Address = 0x7;
}

pub mod client_base {
    use memlib::memory::Address;

    pub const BASE_OFFSET: Address = 0x97B88;
    pub const BASE_REVERSED_ADDR: Address = 0x4D61205;
    pub const BASE_DISPLACEMENT: Address = 0x17;
    pub const SIZE: usize = 0x3A78; //
}

pub mod bones {
    use memlib::memory::Address;

    pub const ENCRYPTED_PTR: Address = 0x120B5108;
    pub const REVERSED_ADDRESS: Address = 0x4D612DF;
    pub const DISPLACEMENT: Address = 0x17;
    pub const BASE_POS: Address = 0x1160C;
    pub const INDEX_STRUCT_SIZE: usize = 0x150;
}