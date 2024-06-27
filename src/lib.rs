pub mod core;
mod script;

use crate::core::keyboard::on_keyboard_message;
use crate::core::libscripthook::{
    keyboard_handler_register, keyboard_handler_unregister, script_register, script_unregister,
};
use crate::script::script_main;
use windows::Win32::Foundation::HMODULE;
use windows::Win32::System::SystemServices::{DLL_PROCESS_ATTACH, DLL_PROCESS_DETACH};

#[allow(unused_variables)]
#[no_mangle]
extern "system" fn DllMain(h_instance: HMODULE, reason: u32, lp_reserved: *const u8) -> u32 {
    match reason {
        DLL_PROCESS_ATTACH => unsafe {
            script_register(h_instance, script_main);
            keyboard_handler_register(on_keyboard_message);
        }
        DLL_PROCESS_DETACH => unsafe {
            script_unregister(h_instance);
            keyboard_handler_unregister(on_keyboard_message);
        },
        _ => {}
    };

    1
}
