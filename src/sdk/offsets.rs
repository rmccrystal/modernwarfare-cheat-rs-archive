#![allow(dead_code)]

use memlib::memory::Address;

pub const REFDEF: Address = 0x14016EC0;

pub const NAME_ARRAY: Address = 0x14022888;
pub const NAME_LIST_OFFSET: Address = 0x4C70;

pub const CAMERA_POINTER: Address = 0x11429900;
pub const CAMERA_OFFSET: Address = 0x1D8;

pub const INDEX_ARRAY: Address = 0x1219F4D0;

pub const LOCAL_INDEX_POINTER: Address = 0x176D8;
pub const LOCAL_INDEX_OFFSET: Address = 0x1F4;

pub const NO_RECOIL: Address = 0x83840;

pub mod entity {
    use memlib::memory::Address;

    // pub const ENCRYPTED_PTR: Address = 0x112F7598;
    // pub const REVERSED_ADDRESS: Address = 0x4E9E22E;
    // pub const DISPLACEMENT: Address = 0xD;
    // pub const SIZE: usize = 0x310;
}

pub mod client_info {
    use memlib::memory::Address;

    pub const ENCRYPTED_PTR: Address = 0x14015168;
    pub const REVERSED_ADDRESS: Address = 0x4DD40DE;
    pub const DISPLACEMENT: Address = 0xB;
}

pub mod client_base {
    use memlib::memory::Address;

    pub const BASE_OFFSET: Address = 0x97BD8;
    pub const BASE_REVERSED_ADDR: Address = 0x4DD4109;
    pub const BASE_DISPLACEMENT: Address = 0x11;
    pub const SIZE: usize = 0x3A88;
}

pub mod bones {
    use memlib::memory::Address;

    pub const ENCRYPTED_PTR: Address = 0x1226F898;
    pub const REVERSED_ADDRESS: Address = 0x4DD4198;
    pub const DISPLACEMENT: Address = 0x15;
    pub const BASE_POS: Address = 0x49B3C;
    pub const INDEX_STRUCT_SIZE: usize = 0x150;
}