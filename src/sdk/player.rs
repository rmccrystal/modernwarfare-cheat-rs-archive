use super::structs::character_info;
use memlib::math::Vector3;
use memlib::memory::{read_memory};

impl character_info {
    pub fn get_origin(&self) -> Vector3 {
        read_memory(self.position_pointer + 0x40)
    }
}