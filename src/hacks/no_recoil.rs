use std::sync::atomic::AtomicBool;
use memlib::memory::{write_memory, Address};
use crate::sdk::offsets;
use std::sync::mpsc::{Sender, channel};

// since we're running on a new thread we have to send messages
pub struct NoRecoilState {
    pub enabled: bool,
    pub client_info_base: Option<Address>,
}

impl NoRecoilState {
    pub fn default() -> Self {
        Self {
            enabled: true,
            client_info_base: None
        }
    }
}

pub fn start_no_recoil_thread() -> Sender<NoRecoilState> {
    let (send, recv) = channel();

    let mut state = NoRecoilState::default();

    std::thread::spawn(move || {
        loop {
            // Update the state from thread
            state = match recv.try_recv() {
                Ok(new_state) => new_state,
                Err(_) => state
            };
            if state.client_info_base.is_none() {
                continue;
            }
            write_memory(state.client_info_base.unwrap() + offsets::NO_RECOIL, 0f32);
            write_memory(state.client_info_base.unwrap() + offsets::NO_RECOIL + 0x4, 0f32);
        }
    });

    send
}