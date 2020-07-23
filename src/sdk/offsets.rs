use memlib::memory::Address;

pub const NAME_ARRAY: Address = 0x11352f58;
pub const NAME_LIST_OFFSET: Address = 0x4C70;
pub const CAMERA_OFFSET: Address = 0xe89b7b0;

pub mod entity {
    use memlib::memory::Address;

    pub const ENCRYPTED_PTR: Address = 0x11344618;
    pub const REVERSED_ADDRESS: Address = 0x4F23200;
    pub const DISPLACEMENT: Address = 0x9;
}

pub mod client_info {
    use memlib::memory::Address;

    pub const ENCRYPTED_PTR: Address = 0x11345938;
    pub const REVERSED_ADDRESS: Address = 0x4F2327E;
    pub const DISPLACEMENT: Address = 0x11;
}

pub mod client_base {
    use memlib::memory::Address;

    pub const BASE_OFFSET: Address = 0x97B48;
    pub const BASE_REVERSED_ADDR: Address = 0x4F2329F;
    pub const BASE_DISPLACEMENT: Address = 0xD;
}

pub mod bones {
    use memlib::memory::Address;

    pub const ENCRYPTED_PTR: Address = 0xF657608;
    pub const REVERSED_ADDRESS: Address = 0x4F2338C;
    pub const DISPLACEMENT: Address = 0x7;
}