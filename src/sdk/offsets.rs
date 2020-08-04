#![allow(dead_code)]

use memlib::memory::Address;

pub const IN_GAME: Address = 0x11086B48;

pub const NAME_ARRAY: Address = 0x11305ED8;
pub const NAME_LIST_OFFSET: Address = 0x4C70;

pub const CAMERA_POINTER: Address = 0xE84E820;
pub const CAMERA_OFFSET: Address = 0x1D8;

pub const INDEX_ARRAY: Address = 0xF6092C0;

pub const LOCAL_INDEX_POINTER: Address = 0x4D808;
pub const LOCAL_INDEX_OFFSET: Address = 0x1F4;

pub mod entity {
    use memlib::memory::Address;

    // pub const ENCRYPTED_PTR: Address = 0x112F7598;
    // pub const REVERSED_ADDRESS: Address = 0x4E9E22E;
    // pub const DISPLACEMENT: Address = 0xD;
    // pub const SIZE: usize = 0x310;
}

pub mod client_info {
    use memlib::memory::Address;

    pub const ENCRYPTED_PTR: Address = 0x112F88B8;
    pub const REVERSED_ADDRESS: Address = 0x4ED6285;
    pub const DISPLACEMENT: Address = 0x17;
}

pub mod client_base {
    use memlib::memory::Address;

    pub const BASE_OFFSET: Address = 0x97B08;
    pub const BASE_REVERSED_ADDR: Address = 0x4ED62C9;
    pub const BASE_DISPLACEMENT: Address = 0x13;
    pub const SIZE: usize = 0x3A28;
}

pub mod bones {
    use memlib::memory::Address;

    pub const ENCRYPTED_PTR: Address = 0xF60A688;
    pub const REVERSED_ADDRESS: Address = 0x4ED63FD;
    pub const DISPLACEMENT: Address = 0x7;
    pub const BASE_POS: Address = 0x53694;
    pub const INDEX_STRUCT_SIZE: usize = 0x150;
}