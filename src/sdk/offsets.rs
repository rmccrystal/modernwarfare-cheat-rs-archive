#![allow(dead_code)]

use memlib::memory::Address;

pub const GAMEMODE: Address = 0x13d09ae8;

pub const REFDEF: Address = 0x112FA5F0;

pub const REFDEF_1: u32 = 0x13F77AF0;
pub const REFDEF_2: u32 = 0x13F77AF4;
pub const REFDEF_3: u32 = 0x13F77AF8;

pub const NAME_ARRAY: Address = 0x13F83428;
pub const NAME_LIST_OFFSET: Address = 0x4C70;

pub const CAMERA_POINTER: Address = 0x1145F480;
pub const CAMERA_OFFSET: Address = 0x1D8;

pub const INDEX_ARRAY: Address = 0x12256CC0;

pub const LOCAL_INDEX_POINTER: Address = 0x1C970;
pub const LOCAL_INDEX_OFFSET: Address = 0x1F4;

pub const NO_RECOIL: Address = 0x0046EA0;

pub mod entity {
    use memlib::memory::Address;

    // pub const ENCRYPTED_PTR: Address = 0x112F7598;
    // pub const REVERSED_ADDRESS: Address = 0x4E9E22E;
    // pub const DISPLACEMENT: Address = 0xD;
    // pub const SIZE: usize = 0x310;
}

pub mod client_info {
    use memlib::memory::Address;

    pub const ENCRYPTED_PTR: Address = 0x13F75D98;
    pub const REVERSED_ADDRESS: Address = 0x4F671ED;
    pub const DISPLACEMENT: Address = 0x15;
}

pub mod client_base {
    use memlib::memory::Address;

    pub const BASE_OFFSET: Address = 0x97B68;
    pub const BASE_REVERSED_ADDR: Address = 0x4F67224;
    pub const BASE_DISPLACEMENT: Address = 0x13;
    pub const SIZE: usize = 0x3A78;
}

pub mod bones {
    use memlib::memory::Address;

    pub const ENCRYPTED_PTR: Address = 0x12258088;
    pub const REVERSED_ADDRESS: Address = 0x4F672C9;
    pub const DISPLACEMENT: Address = 0x19;
    pub const BASE_POS: Address = 0x777F4;
    pub const INDEX_STRUCT_SIZE: usize = 0x150;
}