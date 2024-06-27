use crate::core::{natives::*, string::UnsafeIntoCString, types::*};

#[allow(unused)]
pub mod controls {
    pub const INPUT_SPRINT: u32 = 0x8FFC75D6;
    pub const INPUT_JUMP: u32 = 0xD9D0E1C0;
    pub const INPUT_HORSE_MOVE_LR: u32 = 0x126796EB;
    pub const INPUT_HORSE_MOVE_UD: u32 = 0x3BBDEFEF;
    pub const INPUT_HORSE_STOP: u32 = 0xE16B9AAD;
    pub const INPUT_HORSE_MELEE: u32 = 0x1A3EABBB;
    pub const INPUT_FRONTEND_LS: u32 = 0x43CDA5B0;
    pub const INPUT_FRONTEND_RS: u32 = 0x7DA48D2A;
}

pub fn print_bottom(text: &str) {
    let text = text.c_string();
    UILOG::_UILOG_SET_CACHED_OBJECTIVE(&text);
    UILOG::_UILOG_PRINT_CACHED_OBJECTIVE();
    UILOG::_UILOG_CLEAR_CACHED_OBJECTIVE();
}

pub fn is_player_on_mount() -> bool {
    PED::GET_MOUNT(PLAYER::PLAYER_PED_ID()) != 0
}

pub fn apply_force(entity: Entity, force: &Vector3, offset: &Vector3) {
    ENTITY::APPLY_FORCE_TO_ENTITY(entity, 0, *force, *offset, 0, true, true, true, false, true);
}

pub fn is_using_controller() -> bool {
    !PAD::IS_USING_KEYBOARD_AND_MOUSE(2)
}