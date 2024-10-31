use std::sync::Mutex;

use windows::Win32::{System::SystemInformation::GetTickCount, UI::Input::KeyboardAndMouse::VIRTUAL_KEY};
use crate::core::types::{BOOL, BYTE, DWORD, WORD};

#[allow(unused)]
#[derive(Copy, Clone)]
struct KeyState {
    time: DWORD,
    is_with_alt: bool,
    was_down_before: bool,
    is_up_now: bool
}

impl KeyState {
    const fn initial() -> Self {
        Self {
            time: 0, is_with_alt: false, was_down_before: false, is_up_now: true
        }
    }
}

static KEY_STATES: Mutex<[KeyState; 255]> = Mutex::new([KeyState::initial(); 255]);
const NOW_PERIOD: DWORD = 100;
const MAX_DOWN: DWORD = 5000;

#[allow(unused_variables)]
pub extern "C" fn on_keyboard_message(key: DWORD, repeats: WORD, scan_code: BYTE, is_extended: BOOL, is_with_alt: BOOL, was_down_before: BOOL, is_up_now: BOOL) {
    let mut key_states = KEY_STATES.lock().unwrap();
    if let Some(key_state) = key_states.get_mut(key as usize) {
        *key_state = KeyState {
            time: unsafe { GetTickCount() },
            is_with_alt: is_with_alt > 0,
            was_down_before: was_down_before > 0,
            is_up_now: is_up_now > 0
        };
    }
}

pub fn is_key_down(key: VIRTUAL_KEY) -> bool {
    let key_states = KEY_STATES.lock().unwrap();
    key_states.get(key.0 as usize)
        .map(|k| !k.is_up_now)
        .unwrap_or(false)
}
pub fn is_key_down_limited(key: VIRTUAL_KEY) -> bool {
    let key_states = KEY_STATES.lock().unwrap();
    let tick_count = unsafe { GetTickCount() };
    key_states.get(key.0 as usize)
        .map(|k| (tick_count < k.time + MAX_DOWN) && !k.is_up_now)
        .unwrap_or(false)
}

pub fn is_key_just_up(key: VIRTUAL_KEY) -> bool {
    let key_states = KEY_STATES.lock().unwrap();
    let tick_count = unsafe { GetTickCount() };
    let is_up = key_states.get(key.0 as usize)
        .map(|k| tick_count < k.time + NOW_PERIOD && k.is_up_now)
        .unwrap_or(false);

    // Drop existing reference to KEY_STATES to prevent deadlock
    drop(key_states);
    if is_up {
        reset_key_state(key);
    }

    is_up
}

pub fn reset_key_state(key: VIRTUAL_KEY) {
    let mut key_states = KEY_STATES.lock().unwrap();
    if let Some(key) = key_states.get_mut(key.0 as usize) {
        *key = KeyState::initial();
    }
}