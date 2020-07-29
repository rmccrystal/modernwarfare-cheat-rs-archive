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

    pub const ENCRYPTED_PTR: Address = 0x111AB838;
    pub const REVERSED_ADDRESS: Address = 0x4F23200;
    pub const DISPLACEMENT: Address = 0x9;
}

pub mod client_info {
    use memlib::memory::Address;

    pub const ENCRYPTED_PTR: Address = 0x111AB838;
    pub const REVERSED_ADDRESS: Address = 0x4D892BF;
    pub const DISPLACEMENT: Address = 0x15;
}

pub mod client_base {
    use memlib::memory::Address;

    pub const BASE_OFFSET: Address = 0x97AD8;
    pub const BASE_REVERSED_ADDR: Address = 0x4D89307;
    pub const BASE_DISPLACEMENT: Address = 0xB;
}

pub mod bones {
    use memlib::memory::Address;

    pub const ENCRYPTED_PTR: Address = 0xF4BD608;
    pub const REVERSED_ADDRESS: Address = 0x4D893FD;
    pub const DISPLACEMENT: Address = 0x9;
    pub const BASE_POS: Address = 0x5A3B4;
}