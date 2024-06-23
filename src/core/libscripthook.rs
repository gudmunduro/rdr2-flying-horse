use windows::Win32::Foundation::HMODULE;
use crate::core::types::{BOOL, BYTE, DWORD, WORD};

type KeyboardHandler = unsafe extern "C" fn(DWORD, WORD, BYTE, BOOL, BOOL, BOOL, BOOL);

#[link(name = "ScriptHookRDR2")]
extern "C" {
    #[link_name = "?scriptRegister@@YAXPEAUHINSTANCE__@@P6AXXZ@Z"]
    pub fn script_register(module: HMODULE, script_main: unsafe extern "C" fn());
    #[link_name = "?scriptUnregister@@YAXPEAUHINSTANCE__@@@Z"]
    pub fn script_unregister(hmodule: HMODULE);
    #[link_name = "?scriptWait@@YAXK@Z"]
    pub fn script_wait(time: u32);
    #[link_name = "?nativeInit@@YAX_K@Z"]
    pub fn native_init(hash: u64);
    #[link_name = "?nativePush64@@YAX_K@Z"]
    pub fn native_push_64(val: u64);
    #[link_name = "?nativeCall@@YAPEA_KXZ"]
    pub fn native_call() -> *mut u64;
    #[link_name = "?keyboardHandlerRegister@@YAXP6AXKGEHHHH@Z@Z"]
    pub fn keyboard_handler_register(handler: KeyboardHandler);
    #[link_name = "?keyboardHandlerUnregister@@YAXP6AXKGEHHHH@Z@Z"]
    pub fn keyboard_handler_unregister(handler: KeyboardHandler);
}