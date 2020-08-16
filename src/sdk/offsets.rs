#![allow(dead_code)]

use memlib::memory::Address;

// not updated
pub const GAMEMODE: Address = 0x13D7EB68;

pub const REFDEF: Address = 0x13FECAF0;

pub const NAME_ARRAY: Address = 0x13FF8428;
pub const NAME_LIST_OFFSET: Address = 0x4C70;

pub const CAMERA_POINTER: Address = 0x114D4500;
pub const CAMERA_OFFSET: Address = 0x1D8;

pub const INDEX_ARRAY: Address = 0x122CBD40;

pub const LOCAL_INDEX_POINTER: Address = 0x2A3C0;
pub const LOCAL_INDEX_OFFSET: Address = 0x1F4;

pub const NO_RECOIL: Address = 0x634;

pub mod entity {
    use memlib::memory::Address;

    // pub const ENCRYPTED_PTR: Address = 0x112F7598;
    // pub const REVERSED_ADDRESS: Address = 0x4E9E22E;
    // pub const DISPLACEMENT: Address = 0xD;
    // pub const SIZE: usize = 0x310;
}

pub mod client_info {
    use memlib::memory::Address;

    pub const ENCRYPTED_PTR: Address = 0x13FEAD98;
    pub const REVERSED_ADDRESS: Address = 0x4F791ED;
    pub const DISPLACEMENT: Address = 0x15;
}

pub mod client_base {
    use memlib::memory::Address;

    pub const BASE_OFFSET: Address = 0x97B38;
    pub const BASE_REVERSED_ADDR: Address = 0x4F79224;
    pub const BASE_DISPLACEMENT: Address = 0x13;
    pub const SIZE: usize = 0x3A60;
}

pub mod bones {
    use memlib::memory::Address;

    pub const ENCRYPTED_PTR: Address = 0x122CD108;
    pub const REVERSED_ADDRESS: Address = 0x4F792C9;
    pub const DISPLACEMENT: Address = 0x19;
    pub const BASE_POS: Address = 0x587C;
    pub const INDEX_STRUCT_SIZE: usize = 0x150;
}