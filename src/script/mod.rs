pub mod fly_mode;
pub mod game_utils;

use fly_mode::{on_fly_mode_disabled, FlyState};
use game_utils::{controls, is_player_on_mount, is_using_controller, print_bottom};
use windows::Win32::UI::Input::KeyboardAndMouse::VK_O;
use crate::core::natives::PAD;
use crate::core::scripthook::wait;
use crate::core::keyboard::is_key_just_up;


pub extern "C" fn script_main() {
    let mut fly_mode_enabled = false;
    let mut fly_state = FlyState::default();

    loop {
        let enable_fly_mode = if is_using_controller() {
            PAD::IS_DISABLED_CONTROL_JUST_PRESSED(0, controls::INPUT_FRONTEND_LS)
        } else {
            is_key_just_up(VK_O)
        };
        if enable_fly_mode {
            if !fly_mode_enabled && !is_player_on_mount() {
                print_bottom("You need to be on a horse");
                continue;
            }

            fly_mode_enabled = !fly_mode_enabled;
            let update_state = if fly_mode_enabled { "ON" } else { "OFF" };
            print_bottom(&format!("Horse fly mode: {update_state}"));
            fly_state = FlyState::default();

            if !fly_mode_enabled {
                on_fly_mode_disabled();
            }
            wait(100);
        }
        if fly_mode_enabled {
            fly_mode::fly_mode(&mut fly_state);
        }
        wait(0);
    }
}