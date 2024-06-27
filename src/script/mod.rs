pub mod fly_mode;
pub mod game_utils;

use fly_mode::FlyState;
use game_utils::{controls, is_player_on_mount, is_using_controller, print_bottom};
use windows::Win32::UI::Input::KeyboardAndMouse::VK_O;
use crate::core::natives::PAD;
use crate::core::scripthook::wait;
use crate::core::keyboard::is_key_just_up;

enum FlyModeToggle {
    Enabled,
    Disabled,
    Disabling
}

impl FlyModeToggle {
    pub fn is_enabled(&self) -> bool {
        matches!(self, Self::Enabled)
    }
}

pub extern "C" fn script_main() {
    let mut fly_toggle = FlyModeToggle::Disabled;
    let mut fly_state = FlyState::default();

    loop {
        let enable_button_pressed = if is_using_controller() {
            PAD::IS_DISABLED_CONTROL_JUST_PRESSED(0, controls::INPUT_FRONTEND_LS)
        } else {
            is_key_just_up(VK_O)
        };
        if enable_button_pressed {
            if !fly_toggle.is_enabled() && !is_player_on_mount() {
                print_bottom("You need to be on a horse");
                continue;
            }

            fly_toggle = match fly_toggle {
                FlyModeToggle::Disabled | FlyModeToggle::Disabling => FlyModeToggle::Enabled,
                FlyModeToggle::Enabled => FlyModeToggle::Disabling,
            };
            if fly_toggle.is_enabled() {
                fly_state = FlyState::default();
            }

            let toggle_state_text = if fly_toggle.is_enabled() { "ON" } else { "OFF" };
            print_bottom(&format!("Horse fly mode: {toggle_state_text}"));
            wait(100);
        }
        
        match fly_toggle {
            FlyModeToggle::Enabled =>  fly_mode::fly_mode(&mut fly_state),
            FlyModeToggle::Disabling => {
                if fly_mode::land_and_disable() {
                    fly_toggle = FlyModeToggle::Disabled;
                }
            },
            FlyModeToggle::Disabled => {}
        }

        wait(0);
    }
}