#![allow(dead_code)]

use memlib::memory::{Address, OffsetDefinition, dump_offsets};
use serde::de::IntoDeserializer;

// 48 8D 15 ? ? ? ? 33 15 ? ? ? ? F3 0F 10 73 ?
pub const REFDEF: Address = 0x16D16290;

// 48 8D 0D ? ? ? ? 48 8B 0C C1 48 8B 01 FF 90 ? ? ? ?
// (last qword)
pub const NAME_ARRAY: Address = 0x16D21358;
pub const NAME_LIST_OFFSET: Address = 0x4C70;

// 48 8B 05 ? ? ? ? 48 8B 7C 24 ? 48 05 ? ? ? ?
pub const CAMERA_POINTER: Address = 0x13AFDE40;
pub const CAMERA_OFFSET: Address = 0x1D8;

// 48 83 BB ? ? ? ? ? 0F 84 ? ? ? ? 48 89 B4 24 ? ? ? ?
pub const LOCAL_INDEX_POINTER: Address = 0xF478;
pub const LOCAL_INDEX_OFFSET: Address = 0x1F4;

// 4C 8D 96 ? ? ? 00 B3 01
pub const NO_RECOIL: Address = 0x144;

pub const UNIT_SCALE: f32 = 0.025400;

pub mod entity {
    use memlib::memory::Address;

    // pub const ENCRYPTED_PTR: Address = 0x112F7598;
    // pub const REVERSED_ADDRESS: Address = 0x4E9E22E;
    // pub const DISPLACEMENT: Address = 0xD;
    // pub const SIZE: usize = 0x310;
}

pub mod character_info {
    use memlib::memory::Address;

    // 48 69 D3 ?? ?? ?? ?? 48 03 96 ?? ?? ?? ??
    pub const SIZE: usize = 0x3AA0;

    // C7 87 ?? ?? ?? ?? ?? ?? ?? ?? C7 87 ?? ?? ?? ?? ?? ?? ?? ?? 41
    pub const VALID: Address = 0x6E0;

    // 49 8B D9 41 0F B6 F0 8B F9 48 8B EA
    // Jump to CMP
    pub const POS_PTR: Address = 0x30E0;

    // 8B 87 ? ? ? ? 4C 8B BC 24 ? ? ? ? 4C 8B B4 24 ? ? ? ? 4C 8B AC 24 ? ? ? ? 4C 8B A4 24 ? ? ? ? 85 C0 74 16
    pub const TEAM: Address = 0x308C;

    // 83 BF ? ? ? ? ? 75 0A F3 0F 10 35 ? ? ? ? EB 08
    pub const STANCE: Address = 0x32A0;

    // C7 83 ? ? ? ? ? ? ? ? C7 83 ? ? ? ? ? ? ? ? E8 ? ? ? ? 44 0F B6 C6 48 8B D5 48 8B CF E8 ? ? ? ?
    pub const DEAD_1: Address = 0x3148;

    // 41 83 B8 ? ? ? ? ? 0F 85 ? ? ? ? 41 B8 ? ? ? ?
    pub const DEAD_2: Address = 0x2CC;

    // not working
    pub const ADS: Address = 0xBD4;
    // not working
    pub const RELOAD: Address = 0x644;
}

pub mod client_info {
    use memlib::memory::Address;

    // 48 8B 1D ? ? ? ? C6 44 24 ? ? 0F B6 44 24 ?
    pub const ENCRYPTED_PTR: Address = 0x16D13908;
}

pub mod client_base {
    use memlib::memory::Address;

    // 48 8B 83 ?? ?? ?? ?? C6 44 24 ?? ?? 0F B6
    pub const BASE_OFFSET: Address = 0x98CF8;
}

pub mod bones {
    use memlib::memory::Address;

    // 0F BF B4 ? ? ? ? ? 89 ? 24 ? 85 ?
    // points to decryption, encrypted_ptr is in mov r8, cs:qword_
    pub const ENCRYPTED_PTR: Address = 0x14B82A18;
    pub const BASE_POS: Address = 0x1F86C;
    pub const INDEX_STRUCT_SIZE: usize = 0x150;
}


pub fn get_sigs() -> Vec<OffsetDefinition> {
    vec![
        OffsetDefinition {
            sig: "4C 8D 96 ? ? ? 00 B3 01".to_string(),
            name: "NO_RECOIL".to_string(),
            index: 0,
            offset: 131,
            dword: true,
            rip_relative: false,
        },
        OffsetDefinition {
            sig: "83 BF ? ? ? ? ? 75 0A F3 0F 10 35 ? ? ? ? EB 08".to_string(),
            name: "STANCE".to_string(),
            index: 0,
            offset: 130,
            dword: true,
            rip_relative: false,
        },
        OffsetDefinition {
            sig: "C7 83 ? ? ? ? ? ? ? ? C7 83 ? ? ? ? ? ? ? ? E8 ? ? ? ? 44 0F B6 C6 48 8B D5 48 8B CF E8 ? ? ? ?".to_string(),
            name: "DEAD_1".to_string(),
            index: 0,
            offset: 130,
            dword: true,
            rip_relative: false,
        },
        OffsetDefinition {
            sig: "41 83 B8 ? ? ? ? ? 0F 85 ? ? ? ? 41 B8 ? ? ? ?".to_string(),
            name: "DEAD_2".to_string(),
            index: 0,
            offset: 131,
            dword: true,
            rip_relative: false,
        },
        OffsetDefinition {
            sig: "8B 87 ? ? ? ? 4C 8B BC 24 ? ? ? ? 4C 8B B4 24 ? ? ? ? 4C 8B AC 24 ? ? ? ? 4C 8B A4 24 ? ? ? ? 85 C0 74 16".to_string(),
            name: "TEAM".to_string(),
            index: 0,
            offset: 130,
            dword: true,
            rip_relative: false,
        }
    ]
}

#[cfg(test)]
mod tests {
    use memlib::memory::{Handle, dump_offsets, offset_definition_from_sig, find_offset};
    use memlib::logger::MinimalLogger;
    use log::LevelFilter;
    use crate::sdk::offsets::get_sigs;

    #[test]
    fn print_offsets() -> anyhow::Result<()> {
        let _ = MinimalLogger::init(LevelFilter::Info);

        let handle = Handle::new("ModernWarfare.exe")?;
        let module = handle.get_module("ModernWarfare.exe").unwrap();
        let buf = handle.dump_memory(module.get_memory_range());

        let offsets = get_sigs();
        println!("{}", dump_offsets(&buf, &offsets));

        Ok(())
    }

    #[test]
    fn generate_offset_defs() -> anyhow::Result<()> {
        let _ = MinimalLogger::init(LevelFilter::Info);

        let handle = Handle::new("ModernWarfare.exe")?;
        let module = handle.get_module("ModernWarfare.exe").unwrap();
        let buf = handle.dump_memory(module.get_memory_range());

        use super::*;
        use super::character_info::*;

        // let sigs = get_sigs();

        // let offsets = dump_offsets(&buf, &sigs);
        // println!("{}", offsets);
        macro_rules! print_def {
            ($sig:expr, $value:expr, $dword:expr) => {
                println!("{}", match offset_definition_from_sig(&buf, $sig, $value as _, ::std::stringify!($value), $dword, 128, 128, None) {
                    Ok(n) => format!("{:?}", n),
                    Err(e) => format!("Could not get {}: {}", ::std::stringify!(value), e)
                });
            };
            ($sig:expr, $value:expr, $dword:expr, true) => {
                println!("{}", match offset_definition_from_sig(&buf, $sig, $value as _, ::std::stringify!($value), $dword, 128, 128, Some(module.base_address)) {
                    Ok(n) => format!("{:?}", n),
                    Err(e) => format!("Could not get {}: {}", ::std::stringify!(value), e)
                });
            };
        }

        print_def!("4C 8D 96 ? ? ? 00 B3 01", NO_RECOIL, true);
        print_def!("49 8B D9 41 0F B6 F0 8B F9 48 8B EA", POS_PTR, true);
        print_def!("83 BF ? ? ? ? ? 75 0A F3 0F 10 35 ? ? ? ? EB 08", STANCE, true);
        print_def!("C7 83 ? ? ? ? ? ? ? ? C7 83 ? ? ? ? ? ? ? ? E8 ? ? ? ? 44 0F B6 C6 48 8B D5 48 8B CF E8 ? ? ? ?", DEAD_1, true);
        print_def!("41 83 B8 ? ? ? ? ? 0F 85 ? ? ? ? 41 B8 ? ? ? ?", DEAD_2, true);
        print_def!("8B 87 ? ? ? ? 4C 8B BC 24 ? ? ? ? 4C 8B B4 24 ? ? ? ? 4C 8B AC 24 ? ? ? ? 4C 8B A4 24 ? ? ? ? 85 C0 74 16", TEAM, true);
        print_def!("C7 87 ?? ?? ?? ?? ?? ?? ?? ?? C7 87 ?? ?? ?? ?? ?? ?? ?? ?? 41", VALID, true);
        print_def!("48 69 D3 ?? ?? ?? ?? 48 03 96 ?? ?? ?? ??", SIZE, true);
        print_def!("48 8B 1D ? ? ? ? C6 44 24 ? ? 0F B6 44 24 ?", client_info::ENCRYPTED_PTR, true, true);
        print_def!("48 8B 83 ?? ?? ?? ?? C6 44 24 ?? ?? 0F B6", client_base::BASE_OFFSET, true);
        print_def!("48 83 BB ? ? ? ? ? 0F 84 ? ? ? ? 48 89 B4 24 ? ? ? ?", LOCAL_INDEX_OFFSET, true);
        print_def!("48 8B 05 ? ? ? ? 48 8B 7C 24 ? 48 05 ? ? ? ?", CAMERA_POINTER, true, true);
        print_def!("4C 8D 1D ? ? ? ? 44 8B 15 ? ? ? ? 48 8D 1D ? ? ? ? 4C 8B C9", REFDEF, true, true);
        print_def!("48 8D 0D ? ? ? ? 48 8B 0C C1 48 8B 01 FF 90 ? ? ? ?", NAME_ARRAY, true, true);
        // println!("{:?}", offset_definition_from_sig(&buf, "49 8B D9 41 0F B6 F0 8B F9 48 8B EA", super::character_info::POS_PTR, "POS_PTR", true, 32));

        Ok(())
    }
}
