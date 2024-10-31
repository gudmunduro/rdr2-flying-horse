pub mod fly_mode;
pub mod game_utils;

use crate::core::keyboard::is_key_just_up;
use crate::core::natives::PAD;
use crate::core::scripthook::wait;
use fly_mode::FlyState;
use game_utils::{controls, is_player_on_mount, is_using_controller};
use windows::Win32::UI::Input::KeyboardAndMouse::VK_O;

enum FlyModeToggle {
    Enabled,
    Disabled,
    Disabling,
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
        wait(0);
        
        let enable_button_pressed = if is_using_controller() {
            PAD::IS_DISABLED_CONTROL_JUST_PRESSED(0, controls::INPUT_FRONTEND_LS)
        } else {
            is_key_just_up(VK_O)
        };
        if enable_button_pressed {
            if !fly_toggle.is_enabled() && !is_player_on_mount() {
                continue;
            }

            fly_toggle = match fly_toggle {
                FlyModeToggle::Disabled | FlyModeToggle::Disabling => FlyModeToggle::Enabled,
                FlyModeToggle::Enabled => FlyModeToggle::Disabling,
            };
            if fly_toggle.is_enabled() {
                fly_state = FlyState::default();
            }
        }

        match fly_toggle {
            FlyModeToggle::Enabled if !is_player_on_mount() => fly_toggle = FlyModeToggle::Disabled,
            FlyModeToggle::Enabled => fly_mode::fly_mode(&mut fly_state),
            FlyModeToggle::Disabling => {
                if fly_mode::land_and_disable() {
                    fly_toggle = FlyModeToggle::Disabled;
                }
            }
            FlyModeToggle::Disabled => {}
        }
    }
}
