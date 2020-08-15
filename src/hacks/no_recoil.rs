use memlib::memory::{write_memory, Address, read_memory};
use crate::sdk::offsets;
use std::sync::mpsc::{Sender, channel};
use std::num::Wrapping;
use std::sync::{Mutex, Arc};
use log::*;
use std::sync::atomic::{AtomicI64, Ordering};
use std::{time, thread};

// since we're running on a new thread we have to send messages
#[derive(Clone)]
pub struct NoRecoilState {
    pub enabled: bool,
    pub client_info_base: Option<Address>,
    pub in_game: bool
}

impl NoRecoilState {
    pub fn default() -> Self {
        Self {
            enabled: true,
            client_info_base: None,
            in_game: false,
        }
    }
}

const NO_RECOIL_THREADS: u32 = 5;
static WRITE_COUNTER: AtomicI64 = AtomicI64::new(0);

pub fn start_no_recoil_thread() -> Sender<NoRecoilState> {
    let (send, recv) = channel();

    let mut local_state = NoRecoilState::default();

    // this will be updated by the channel
    let shared_state: Arc<Mutex<NoRecoilState>> = Arc::new(Mutex::new(local_state.clone()));

    // start no recoil threads
    for thread in 1..=NO_RECOIL_THREADS {
        debug!("Starting no recoil thread {}", thread);
        let state = shared_state.clone();
        thread::spawn(move || {
            loop {
                let state = state.lock().unwrap();
                if state.client_info_base.is_none() || !state.in_game || !state.enabled {
                    continue;
                }
                no_recoil_tick(state.client_info_base.unwrap());
            }
        });
    }

    // this thread will update the shared state from the channel
    thread::spawn(move || {
        loop {
            // Update the state from thread
            local_state = match recv.try_recv() {
                Ok(new_state) => new_state,
                Err(_) => local_state
            };

            // update the shared state between threads
            *shared_state.lock().unwrap() = local_state.clone();
        }
    });

    thread::spawn(move || {
        loop {
            thread::sleep(time::Duration::from_secs(1));
            debug!("No recoil threads: {} writes / s", WRITE_COUNTER.load(Ordering::SeqCst));
            let _ = WRITE_COUNTER.fetch_update(Ordering::SeqCst, Ordering::SeqCst, |_| Some(0));
        }
    });

    send
}

fn no_recoil_tick(client_info_base: Address) {
    let mut r12 = Wrapping(client_info_base + offsets::NO_RECOIL);
    let mut rsi = r12 + Wrapping(0x4);
    let edx: Wrapping<u32> = Wrapping(read_memory((r12 + Wrapping(0xC)).0));
    let mut ecx = edx;
    ecx = Wrapping(ecx.0 ^ r12.0 as u32);
    let mut eax = ecx + Wrapping(2);
    eax = eax * ecx;
    ecx = edx;
    ecx = Wrapping(ecx.0 ^ rsi.0 as u32);
    let address = r12.0;
    let val1 = eax.0;
    eax = ecx + Wrapping(2);
    eax = eax * ecx;
    let val2 = eax.0;

    write_memory(address, ((val1 as u64) << 32) + val2 as u64);
    WRITE_COUNTER.fetch_add(1, Ordering::SeqCst);
}
