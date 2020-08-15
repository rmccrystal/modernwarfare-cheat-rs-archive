#![allow(dead_code)]

use memlib::memory::Address;

// not updated
pub const GAMEMODE: Address = 0x13A56AE8;

pub const REFDEF: Address = 0x13D2C9D0;

pub const NAME_ARRAY: Address = 0x13D38328;
pub const NAME_LIST_OFFSET: Address = 0x4C70;

pub const CAMERA_POINTER: Address = 0x11214500;
pub const CAMERA_OFFSET: Address = 0x1D8;

pub const INDEX_ARRAY: Address = 0x1200BD50;

pub const LOCAL_INDEX_POINTER: Address = 0x698;
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

    pub const ENCRYPTED_PTR: Address = 0x13D2AC98;
    pub const REVERSED_ADDRESS: Address = 0x4D1C1E4;
    pub const DISPLACEMENT: Address = 0xD;
}

pub mod client_base {
    use memlib::memory::Address;

    pub const BASE_OFFSET: Address = 0x97B28;
    pub const BASE_REVERSED_ADDR: Address = 0x4D1C1F9;
    pub const BASE_DISPLACEMENT: Address = 0xD;
    pub const SIZE: usize = 0x3A40;
}

pub mod bones {
    use memlib::memory::Address;

    pub const ENCRYPTED_PTR: Address = 0x1200D118;
    pub const REVERSED_ADDRESS: Address = 0x4D1C2A1;
    pub const DISPLACEMENT: Address = 0x13;
    pub const BASE_POS: Address = 0x604E4;
    pub const INDEX_STRUCT_SIZE: usize = 0x150;
}