#![allow(dead_code)]

use memlib::memory::Address;

pub const NAME_ARRAY: Address = 0x111B8E58;
pub const NAME_LIST_OFFSET: Address = 0x4C70;

pub const CAMERA_POINTER: Address = 0xE701820;
pub const CAMERA_OFFSET: Address = 0x1D8;

pub const INDEX_ARRAY: Address = 0xF4BC240;

pub const LOCAL_INDEX_POINTER: Address = 0x20878;
pub const LOCAL_INDEX_OFFSET: Address = 0x1F4;

pub mod entity {
    use memlib::memory::Address;

    pub const ENCRYPTED_PTR: Address = 0x111BF698;
    pub const REVERSED_ADDRESS: Address = 0x4D9E226;
    pub const DISPLACEMENT: Address = 0x19;
    pub const SIZE: usize = 0x378;
}

pub mod client_info {
    use memlib::memory::Address;

    pub const ENCRYPTED_PTR: Address = 0x111C09B8;
    pub const REVERSED_ADDRESS: Address = 0x4D9E2CD;
    pub const DISPLACEMENT: Address = 0x9;
}

pub mod client_base {
    use memlib::memory::Address;

    pub const BASE_OFFSET: Address = 0x97B68;
    pub const BASE_REVERSED_ADDR: Address = 0x4D9E306;
    pub const BASE_DISPLACEMENT: Address = 0xB;
    pub const SIZE: usize = 0x3A98;
}

pub mod bones {
    use memlib::memory::Address;

    pub const ENCRYPTED_PTR: Address = 0xF4D2608;
    pub const REVERSED_ADDRESS: Address = 0x4D9E3F6;
    pub const DISPLACEMENT: Address = 0x13;
    pub const BASE_POS: Address = 0x5A3B4;
}