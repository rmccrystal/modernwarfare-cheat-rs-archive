#![allow(dead_code)]

use memlib::memory::Address;

pub const IN_GAME: Address = 0x1104EB48;

pub const NAME_ARRAY: Address = 0x112CDED8;
pub const NAME_LIST_OFFSET: Address = 0x4C70;

pub const CAMERA_POINTER: Address = 0xE816830;
pub const CAMERA_OFFSET: Address = 0x1D8;

pub const INDEX_ARRAY: Address = 0xF5D12C0;

pub const LOCAL_INDEX_POINTER: Address = 0x7C830;
pub const LOCAL_INDEX_OFFSET: Address = 0x1F4;

pub mod entity {
    use memlib::memory::Address;

    pub const ENCRYPTED_PTR: Address = 0x112BF598;
    pub const REVERSED_ADDRESS: Address = 0x4E9E22E;
    pub const DISPLACEMENT: Address = 0xD;
    pub const SIZE: usize = 0x310;
}

pub mod client_info {
    use memlib::memory::Address;

    pub const ENCRYPTED_PTR: Address = 0x112C08B8;
    pub const REVERSED_ADDRESS: Address = 0x4E9E2D9;
    pub const DISPLACEMENT: Address = 0x17;
}

pub mod client_base {
    use memlib::memory::Address;

    pub const BASE_OFFSET: Address = 0x97AF8;
    pub const BASE_REVERSED_ADDR: Address = 0x4E9E310;
    pub const BASE_DISPLACEMENT: Address = 0x13;
    pub const SIZE: usize = 0x3A20;
}

pub mod bones {
    use memlib::memory::Address;

    pub const ENCRYPTED_PTR: Address = 0xF5D2688;
    pub const REVERSED_ADDRESS: Address = 0x4E9E3F9;
    pub const DISPLACEMENT: Address = 0x9;
    pub const BASE_POS: Address = 0x0FDEC;
}