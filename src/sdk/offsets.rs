pub mod entity {
    use memlib::memory::Address;

    pub const ENCRYPTED_PTR: Address = 0x112D0598;
    pub const REVERSED_ADDRESS: Address = 0x4EAF1F5;
    pub const DISPLACEMENT: Address = 0x13;
}

pub mod client_info {
    use memlib::memory::Address;

    pub const ENCRYPTED_PTR: Address = 0x112D18B8;
    pub const REVERSED_ADDRESS: Address = 0x4EAF2B4;
    pub const DISPLACEMENT: Address = 0x15;
}

pub mod base {
    use memlib::memory::Address;

    pub const BASE_OFFSET: Address = 0x97AF8;
    pub const BASE_REVERSED_ADDR: Address = 0x4EAF2E5;
    pub const BASE_DISPLACEMENT: Address = 0x17;
}

pub mod bones {
    use memlib::memory::Address;

    pub const ENCRYPTED_PTR: Address = 0xF5E3578;
    pub const REVERSED_ADDRESS: Address = 0x4EAF40F;
    pub const DISPLACEMENT: Address = 0x13;
}