use crate::core::types::Vector3;
use crate::core::{natives::*, types::*};

use super::game_utils::{apply_force, controls, is_player_on_mount};

const MAX_FORCE: f32 = 4_000.0;
const EXPECTED_MAX_SPEED: f32 = 50.0;

pub struct FlyState {
    pub fwd_speed: f32,
    pub super_speed: bool,
}

impl Default for FlyState {
    fn default() -> Self {
        Self {
            fwd_speed: 0.0,
            super_speed: false,
        }
    }
}

pub fn fly_mode(state: &mut FlyState) {
    if !is_player_on_mount() {
        return;
    }

    let player_ped = PLAYER::PLAYER_PED_ID();
    let mount = PED::GET_MOUNT(player_ped);
    let mount_pos = ENTITY::GET_ENTITY_COORDS(mount, false, false);
    let mut ground_z = 0_f32;
    MISC::GET_GROUND_Z_FOR_3D_COORD(mount_pos, &mut ground_z, false);

    handle_flight_setup(mount, player_ped);
    handle_flight_rotation(mount, state, &mount_pos, ground_z);
    handle_flight_movement(mount, state);
    limit_vertical_velocity(mount, &mount_pos, ground_z);
    prevent_jump(mount, player_ped);
}

fn handle_flight_setup(mount: Ped, player_ped: Ped) {
    PAD::DISABLE_CONTROL_ACTION(0, controls::INPUT_HORSE_MELEE, false);
    PAD::DISABLE_CONTROL_ACTION(0, controls::INPUT_HORSE_JUMP, false);
    PED::SET_PED_CAN_RAGDOLL(player_ped, false);
    PED::SET_PED_CAN_RAGDOLL(mount, false);
    ENTITY::SET_ENTITY_COLLISION(mount, false, true);
}

fn handle_flight_movement(mount: Ped, state: &mut FlyState) {
    let is_super_speed_pressed =
        PAD::IS_DISABLED_CONTROL_JUST_PRESSED(0, controls::INPUT_FRONTEND_RS);
    let is_speedup_pressed = PAD::IS_CONTROL_PRESSED(0, controls::INPUT_HORSE_SPRINT);
    let is_stop_pressed = PAD::IS_CONTROL_PRESSED(0, controls::INPUT_HORSE_STOP);
    let is_move_up_pressed = PAD::IS_DISABLED_CONTROL_PRESSED(0, controls::INPUT_FRONTEND_X);
    let is_move_down_pressed = PAD::IS_DISABLED_CONTROL_PRESSED(0, controls::INPUT_HORSE_MELEE);

    if is_super_speed_pressed {
        state.super_speed = !state.super_speed;
    }
    if is_speedup_pressed {
        state.fwd_speed = f32::min(state.fwd_speed + 50.0, MAX_FORCE);
    }
    if is_stop_pressed {
        state.fwd_speed = f32::max(state.fwd_speed - 100.0, 0.0);
    }
    if is_move_down_pressed {
        go_up(mount, -40.0);
    }

    let z_speed = ENTITY::GET_ENTITY_SPEED_VECTOR(mount, false).z;
    if is_move_up_pressed {
        go_up(mount, 60.0);
    } else if z_speed < 0.0 {
        // Hover
        go_up(mount, 20.0);
    }

    if state.fwd_speed > 0.0 {
        let multipler = if state.super_speed { 5.0 } else { 1.0 };
        go_fwd(mount, state.fwd_speed * multipler);
    }
}

fn handle_flight_rotation(mount: Ped, state: &FlyState, mount_pos: &Vector3, ground_z: f32) {
    let is_low_flying = (mount_pos.z - ground_z) < 26.0;
    let is_aiming = PLAYER::IS_PLAYER_FREE_AIMING(PLAYER::PLAYER_ID());
    let speed = ENTITY::GET_ENTITY_SPEED(mount);
    let cam_rot = CAM::GET_GAMEPLAY_CAM_ROT(0);

    let mut mount_rot = ENTITY::GET_ENTITY_ROTATION(mount, 0);

    // 0: Left - 250: Right
    let lr_value = PAD::GET_CONTROL_VALUE(0, controls::INPUT_HORSE_MOVE_LR) as f32;

    mount_rot.x =
        if (-12.0 < cam_rot.x && cam_rot.x < 12.0) || state.fwd_speed < 1_000.0 || is_aiming {
            0.0
        } else {
            let max_rot = 8.0 * (speed / EXPECTED_MAX_SPEED).powi(3);
            let rot_start = if cam_rot.x > 0.0 { 12.0 } else { -12.0 };
            let reduce_rot_factor = if is_low_flying {
                if cam_rot.x > 0.0 {
                    6.0
                } else {
                    8.0
                }
            } else {
                4.0
            };
            ((cam_rot.x - rot_start) / reduce_rot_factor).clamp(-max_rot, max_rot)
        };
    mount_rot.z += -(lr_value / 125.0 - 1.0) * 2.0;

    mount_rot.x = mount_rot.x.clamp(-8.0, 20.0);
    ENTITY::SET_ENTITY_ROTATION(mount, mount_rot, 0, true);
}

fn limit_vertical_velocity(mount: Ped, mount_pos: &Vector3, ground_z: f32) {
    let mut mount_velocity = ENTITY::GET_ENTITY_VELOCITY(mount, 0);
    let is_close_to_ground = mount_pos.z - ground_z < 2.0;
    if is_close_to_ground {
        mount_velocity.z = 4.0;
    } else {
        mount_velocity.z = mount_velocity.z.clamp(-20.0, 20.0);
    }
    ENTITY::SET_ENTITY_VELOCITY(mount, mount_velocity);
}

fn prevent_jump(mount: Ped, player_ped: Ped) {
    if PED::IS_PED_JUMPING(mount) {
        TASK::CLEAR_PED_TASKS_IMMEDIATELY(mount, true, false);
        PED::SET_PED_ONTO_MOUNT(player_ped, mount, -1, false);

        let max_vel = 5.0;
        let Vector3 { x, y, z, .. } = ENTITY::GET_ENTITY_VELOCITY(mount, 0);
        let vel = Vector3::new(
            x.clamp(-max_vel, max_vel),
            y.clamp(-max_vel, max_vel),
            z.clamp(-max_vel, max_vel),
        );
        ENTITY::SET_ENTITY_VELOCITY(mount, vel);
    }
}

fn go_up(mount: Ped, force: f32) {
    apply_force(
        mount,
        &Vector3::new(0.0, 0.0, force),
        &Vector3::new(0.0, 0.5, 0.0),
    );
    apply_force(
        mount,
        &Vector3::new(0.0, 0.0, force),
        &Vector3::new(0.0, -0.5, 0.0),
    );
}

fn go_fwd(mount: Ped, force: f32) {
    apply_force(
        mount,
        &Vector3::new(0.0, force, 0.0),
        &Vector3::new(0.0, 0.0, 0.0),
    );
}

pub fn land_and_disable() -> bool {
    let player_ped = PLAYER::PLAYER_PED_ID();
    let mount = PED::GET_MOUNT(player_ped);
    let mount_pos = ENTITY::GET_ENTITY_COORDS(mount, false, false);

    ENTITY::SET_ENTITY_COLLISION(mount, true, true);

    let mut ground_z = 0_f32;
    MISC::GET_GROUND_Z_FOR_3D_COORD(mount_pos, &mut ground_z, false);

    if (mount_pos.z - ground_z) < 1.0 {
        PAD::ENABLE_CONTROL_ACTION(0, controls::INPUT_HORSE_MELEE, false);
        PAD::ENABLE_CONTROL_ACTION(0, controls::INPUT_HORSE_JUMP, false);
        PED::SET_PED_CAN_RAGDOLL(player_ped, true);
        PED::SET_PED_CAN_RAGDOLL(mount, true);

        true
    } else {
        false
    }
}
