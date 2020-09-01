#![allow(dead_code)]

use memlib::memory::Address;

pub const REFDEF: Address = 0x14107F40;

pub const NAME_ARRAY: Address = 0x14113908;
pub const NAME_LIST_OFFSET: Address = 0x4C70;

pub const CAMERA_POINTER: Address = 0x1151A980;
pub const CAMERA_OFFSET: Address = 0x1D8;

pub const INDEX_ARRAY: Address = 0x1235F550;

pub const LOCAL_INDEX_POINTER: Address = 0x15190;
pub const LOCAL_INDEX_OFFSET: Address = 0x1F4;

pub const NO_RECOIL: Address = 0x83840;

pub mod entity {
    use memlib::memory::Address;

    // pub const ENCRYPTED_PTR: Address = 0x112F7598;
    // pub const REVERSED_ADDRESS: Address = 0x4E9E22E;
    // pub const DISPLACEMENT: Address = 0xD;
    // pub const SIZE: usize = 0x310;
}

pub mod character_info {
    use memlib::memory::Address;

    pub const SIZE: usize = 0x3A88;
    pub const VALID: Address = 0x7C8;
    pub const POS_PTR: Address = 0x1528;
    pub const TEAM: Address = 0xADC;
    pub const ENTITY_NUM: Address = 0x6B8;
    pub const STANCE: Address = 0xCBC;
    pub const DEATH: Address = 0x2CC;
    pub const ADS: Address = 0xCFC;
    pub const RELOAD: Address = 0xA26;
}

pub mod client_info {
    use memlib::memory::Address;

    pub const ENCRYPTED_PTR: Address = 0x141061E8;
    pub const REVERSED_ADDRESS: Address = 0x4EC50D1;
    pub const DISPLACEMENT: Address = 0x7;
}

pub mod client_base {
    use memlib::memory::Address;

    pub const BASE_OFFSET: Address = 0x97BC8;
    pub const BASE_REVERSED_ADDR: Address = 0x4EC5114;
    pub const BASE_DISPLACEMENT: Address = 0xD;
}

pub mod bones {
    use memlib::memory::Address;

    pub const ENCRYPTED_PTR: Address = 0x1228D918;
    pub const REVERSED_ADDRESS: Address = 0x4DF2218;
    pub const DISPLACEMENT: Address = 0x17;
    pub const BASE_POS: Address = 0x49B3C;
    pub const INDEX_STRUCT_SIZE: usize = 0x150;
}