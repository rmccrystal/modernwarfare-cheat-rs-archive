#![allow(dead_code)]

use memlib::memory::{Address, OffsetDefinition, dump_offsets};
use serde::de::IntoDeserializer;

pub const REFDEF: Address = 0x16C1D290;

pub const NAME_ARRAY: Address = 0x16C28358;
pub const NAME_LIST_OFFSET: Address = 0x4C70;

pub const CAMERA_POINTER: Address = 0x13A04E40;
pub const CAMERA_OFFSET: Address = 0x1D8;

pub const INDEX_ARRAY: Address = 0x4D58F90;

pub const LOCAL_INDEX_POINTER: Address = 0x28ED8;
pub const LOCAL_INDEX_OFFSET: Address = 0x1F4;

pub const NO_RECOIL: Address = 0x337E8;

pub mod entity {
    use memlib::memory::Address;

    // pub const ENCRYPTED_PTR: Address = 0x112F7598;
    // pub const REVERSED_ADDRESS: Address = 0x4E9E22E;
    // pub const DISPLACEMENT: Address = 0xD;
    // pub const SIZE: usize = 0x310;
}

pub mod character_info {
    use memlib::memory::Address;

    pub const SIZE: usize = 0x3A90;
    pub const VALID: Address = 0x168;
    pub const POS_PTR: Address = 0x78;
    pub const TEAM: Address = 0x1324;
    pub const STANCE: Address = 0xB80;
    pub const DEAD_1: Address = 0x80C;
    pub const DEAD_2: Address = 0xA0;
    // not working
    pub const ADS: Address = 0xBD4;
    // not working
    pub const RELOAD: Address = 0x644;
}

pub mod client_info {
    use memlib::memory::Address;

    pub const ENCRYPTED_PTR: Address = 0x16C1A908;
}

pub mod client_base {
    use memlib::memory::Address;

    pub const BASE_OFFSET: Address = 0x98CF8;
}

pub mod bones {
    use memlib::memory::Address;

    pub const ENCRYPTED_PTR: Address = 0x121F7548;
    pub const REVERSED_ADDRESS: Address = 0x4D471DF;
    pub const DISPLACEMENT: Address = 0xD;
    pub const BASE_POS: Address = 0x6D2E4;
    pub const INDEX_STRUCT_SIZE: usize = 0x150;
}


pub fn get_sigs() -> Vec<OffsetDefinition> {
    vec![
        OffsetDefinition {
            name: "NO_RECOIL".into(),
            sig: "4C 8D 96 ? ? ? 00 B3 01".into(),
            offset: 3,
            dword: true,
            ..Default::default()
        },
    ]
}

#[cfg(test)]
mod tests {
    use memlib::memory::{Handle, dump_offsets};
    use memlib::logger::MinimalLogger;
    use log::LevelFilter;
    use crate::sdk::offsets::get_sigs;

    #[test]
    fn test_get_sigs() -> anyhow::Result<()> {
        let _ = MinimalLogger::init(LevelFilter::Info);

        let handle = Handle::new("ModernWarfare.exe")?;
        let module = handle.get_module("ModernWarfare.exe").unwrap();
        let buf = handle.dump_memory(module.get_memory_range());

        let sigs = get_sigs();

        let offsets = dump_offsets(&buf, &sigs);
        println!("{}", offsets);

        Ok(())
    }
}
