#![allow(non_snake_case, unused)]

pub mod BUILTIN {
	use std::ffi::CStr;
	use crate::core::scripthook::native::*;
	use crate::core::types::*;

	pub fn WAIT(ms: i32) { invoke_ignore!(0x4EDE34FBADD967A6, ms) }
	pub fn TIMERA() -> i32 { invoke!(0x83666F9FB8FEBD4B) }
	pub fn TIMERB() -> i32 { invoke!(0xC9D9444186B5A374) }
	pub fn SETTIMERA(value: i32) { invoke_ignore!(0xC1B1E9A034A63A62, value) }
	pub fn SETTIMERB(value: i32) { invoke_ignore!(0x5AE11BC36633DE4E, value) }
	pub fn TIMESTEP() -> f32 { invoke!(0x0000000050597EE2) }
	pub fn SIN(value: f32) -> f32 { invoke!(0x0BADBFA3B172435F, value) }
	pub fn COS(value: f32) -> f32 { invoke!(0xD0FFB162F40A139C, value) }
	pub fn SQRT(value: f32) -> f32 { invoke!(0x71D93B57D07F9804, value) }
	pub fn POW(base: f32, exponent: f32) -> f32 { invoke!(0xE3621CC40F31FE2E, base, exponent) }
	pub fn LOG10(value: f32) -> f32 { invoke!(0xE816E655DE37FE20, value) }
	pub fn VMAG(x: f32, y: f32, z: f32) -> f32 { invoke!(0x652D2EEEF1D3E62C, x, y, z) }
	pub fn VMAG2(x: f32, y: f32, z: f32) -> f32 { invoke!(0xA8CEACB4F35AE058, x, y, z) }
	pub fn VDIST(x1: f32, y1: f32, z1: f32, x2: f32, y2: f32, z2: f32) -> f32 { invoke!(0x2A488C176D52CCA5, x1, y1, z1, x2, y2, z2) }
	pub fn VDIST2(x1: f32, y1: f32, z1: f32, x2: f32, y2: f32, z2: f32) -> f32 { invoke!(0xB7A628320EFF8E47, x1, y1, z1, x2, y2, z2) }
	pub fn SHIFT_LEFT(value: i32, bitShift: i32) -> i32 { invoke!(0xEDD95A39E5544DE8, value, bitShift) }
	pub fn SHIFT_RIGHT(value: i32, bitShift: i32) -> i32 { invoke!(0x97EF1E5BCE9DC075, value, bitShift) }
	pub fn FLOOR(value: f32) -> i32 { invoke!(0xF34EE736CF047844, value) }
	pub fn CEIL(value: f32) -> i32 { invoke!(0x11E019C8F43ACC8A, value) }
	pub fn ROUND(value: f32) -> i32 { invoke!(0xF2DB717A73826179, value) }
	pub fn TO_FLOAT(value: i32) -> f32 { invoke!(0xBBDA792448DB5A89, value) }
	pub fn SET_THIS_THREAD_PRIORITY(priority: i32) { invoke_ignore!(0x42B65DEEF2EDF2A1, priority) }
}
pub mod AICOVERPOINT {
	use std::ffi::CStr;
	use crate::core::scripthook::native::*;
	use crate::core::types::*;

	pub fn _0x53E4D0C079CA6855(handle: ScrHandle) -> Entity { invoke!(0x53E4D0C079CA6855, handle) }
	pub fn _DOES_COVER_POINT_EXIST(handle: ScrHandle) -> bool { invoke!(0xC276FE69DDA22BAD, handle) }
	pub fn _GET_COVER_POINT_STATE_FROM_PED(ped: Ped) -> i32 { invoke!(0x5F5B1B7E8E8F94C6, ped) }
	pub fn TASK_ENTER_COVER(ped: Ped) { invoke_ignore!(0x4972A022AE6DAFA1, ped) }
	pub fn TASK_EXIT_COVER(ped: Ped) { invoke_ignore!(0x2BC4A6D92D140112, ped) }
	pub fn _0x957D7E750216D74B(ped: Ped) -> i32 { invoke!(0x957D7E750216D74B, ped) }
	pub fn _TASK_AI_SEEK_COVER_TO_COVER_POINT(args: &mut Any) { invoke_ignore!(0x89783FDDF079C88D, args) }
	pub fn _0x64340DC208D671D5(coverLayer: & CStr) { invoke_ignore!(0x64340DC208D671D5, coverLayer) }
	pub fn _0x7A1FDCF35EAA140F(coverLayer: & CStr) { invoke_ignore!(0x7A1FDCF35EAA140F, coverLayer) }
	pub fn _REQUEST_FLINCH_COVER_ANIM(ped: Ped) { invoke_ignore!(0x2A31D13C5F021D0D, ped) }
	pub fn _0x3C7A9C2C953128FE(ped: Ped) { invoke_ignore!(0x3C7A9C2C953128FE, ped) }
	pub fn _0xEBA51A294C73292E(args: &mut Any) { invoke_ignore!(0xEBA51A294C73292E, args) }
	pub fn _0x140B3CB1D424A945(ped: Ped, weaponHash: Hash) { invoke_ignore!(0x140B3CB1D424A945, ped, weaponHash) }
	pub fn _ARE_LOAD_COVER_ANIMS_LOADED(ped: Ped) -> bool { invoke!(0x8CBE916CFC64AD5C, ped) }
	pub fn _STOP_RUNNING_COVER_ANIMS(ped: Ped) { invoke_ignore!(0x1A7A802B2301EDC0, ped) }
	pub fn _0x733077295AB51304(args: &mut Any) { invoke_ignore!(0x733077295AB51304, args) }
	pub fn _ADD_SCRIPTED_COVER_POINT(data: &mut Any) -> ScrHandle { invoke!(0x975BD6351648935F, data) }
}
pub mod AITRANSPORT {
	use std::ffi::CStr;
	use crate::core::scripthook::native::*;
	use crate::core::types::*;

	pub fn TASK_ENTER_TRANSPORT(args: &mut Any) { invoke_ignore!(0xAEE3ADD08829CB6F, args) }
	pub fn TASK_EXIT_TRANSPORT(args: &mut Any) { invoke_ignore!(0xC273A5B8488F7838, args) }
	pub fn SET_PED_ON_TRANSPORT_SEAT(ped: Ped, transportEntity: Entity, seat: i32, flags: i32) { invoke_ignore!(0xE588B5A8A005CB5E, ped, transportEntity, seat, flags) }
	pub fn SET_PED_OFF_TRANSPORT_SEAT(ped: Ped, flags: i32) { invoke_ignore!(0x8886D83A430537FD, ped, flags) }
	pub fn SET_TRANSPORT_CONFIG_FLAG(transportEntity: Entity, flagId: i32, value: bool) { invoke_ignore!(0xBA8818212633500A, transportEntity, flagId, value) }
	pub fn GET_TRANSPORT_CONFIG_FLAG(transportEntity: Entity, flagId: i32, p2: bool) -> bool { invoke!(0xF382C92CCC1CCDBC, transportEntity, flagId, p2) }
	pub fn _GET_TRANSPORT_USAGE_FLAGS(transportEntity: Entity, flags: &mut i32) -> Any { invoke!(0xE195C5A82156321D, transportEntity, flags) }
	pub fn _SET_TRANSPORT_USAGE_FLAGS(transportEntity: Entity, flags: i32) { invoke_ignore!(0xE2487779957FE897, transportEntity, flags) }
	pub fn SET_TRANSPORT_ACCESSIBLE_SEAT_FLAGS(transportEntity: Entity, flags: i32) { invoke_ignore!(0xDD0660C997DE94FD, transportEntity, flags) }
	pub fn _0x4B6C9A43F7D9109B(p0: Any, p1: Any) { invoke_ignore!(0x4B6C9A43F7D9109B, p0, p1) }
	pub fn _IS_PED_ON_TRANSPORT_ENTITY(ped: Ped, transportEntity: Entity) -> bool { invoke!(0x159EF5B6EDCE00E8, ped, transportEntity) }
	pub fn _IS_PED_ON_TRANSPORT_SEAT(ped: Ped, p1: bool) -> bool { invoke!(0xDC44F405A6B98D03, ped, p1) }
	pub fn _IS_TRANSPORT_SEAT_OCCUPIED(transportEntity: Entity, seatIndex: i32) -> bool { invoke!(0x2E2E06023D07631E, transportEntity, seatIndex) }
	pub fn _IS_TRANSPORT_SEAT_FREE(transportEntity: Entity, seatIndex: i32) -> bool { invoke!(0x43FF27FC1829C202, transportEntity, seatIndex) }
	pub fn _GET_PED_IN_TRANSPORT_SEAT(transportEntity: Entity, seatIndex: i32) -> Ped { invoke!(0xFFEC4B0A1A3ED515, transportEntity, seatIndex) }
	pub fn IS_PED_ENTERING_TRANSPORT(ped: Ped, transportEntity: Entity, p2: bool) -> bool { invoke!(0x619E63980BFC0096, ped, transportEntity, p2) }
	pub fn IS_PED_EXITING_TRANSPORT(ped: Ped, transportEntity: Entity) -> bool { invoke!(0x660639BC60157048, ped, transportEntity) }
	pub fn _0x4248AB2EEB3C75AD(transportEntity: Entity, ped: Ped, p2: bool) { invoke_ignore!(0x4248AB2EEB3C75AD, transportEntity, ped, p2) }
	pub fn _SET_PED_USE_TRANSPORT_SEAT_PREFERENCE(ped: Ped, transportEntity: Entity, preferenceSlot: i32, p3: i32, seatIndex: i32) { invoke_ignore!(0xB7079F4C72896756, ped, transportEntity, preferenceSlot, p3, seatIndex) }
	pub fn _0x5639FBEA922788DA(transportEntity: Entity) { invoke_ignore!(0x5639FBEA922788DA, transportEntity) }
	pub fn _SET_AI_CAN_USE_TRANSPORT(transportEntity: Entity, state: bool) { invoke_ignore!(0x67F7CEAC2391E114, transportEntity, state) }
	pub fn _0x8C8371EDFAF014A0(ped: Ped, p1: f32) { invoke_ignore!(0x8C8371EDFAF014A0, ped, p1) }
	pub fn _0xF8C20282B237E3F7(ped: Ped) { invoke_ignore!(0xF8C20282B237E3F7, ped) }
	pub fn _SET_TRANSPORT_PRIORITY_SEAT(transportEntity: Entity, seatIndex: i32) { invoke_ignore!(0x13F138225C202F66, transportEntity, seatIndex) }
}
pub mod ANIMSCENE {
	use std::ffi::CStr;
	use crate::core::scripthook::native::*;
	use crate::core::types::*;

	pub fn _CREATE_ANIM_SCENE(animDict: & CStr, flags: i32, playbackListName: & CStr, p3: bool, p4: bool) -> AnimScene { invoke!(0x1FCA98E33C1437B3, animDict, flags, playbackListName, p3, p4) }
	pub fn _DELETE_ANIM_SCENE(animScene: AnimScene) { invoke_ignore!(0x84EEDB2C6E650000, animScene) }
	pub fn TRIGGER_ANIM_SCENE_SKIP(animScene: AnimScene) { invoke_ignore!(0x4B85B3CF9197AEDF, animScene) }
	pub fn _0x4B85B3CF91972222(animScene: AnimScene) -> bool { invoke!(0x4B85B3CF91972222, animScene) }
	pub fn _CLEAR_ANIM_SCENE_WAS_SKIPPED(animScene: AnimScene) { invoke_ignore!(0x8A8208AE92BF87A5, animScene) }
	pub fn DOES_ANIM_SCENE_EXIST(animScene: AnimScene) -> bool { invoke!(0x25557E324489393C, animScene) }
	pub fn _DOES_ENTITY_WITH_ID_EXIST_IN_ANIM_SCENE(animScene: AnimScene, entityId: & CStr) -> bool { invoke!(0x6F1F0B17109309DA, animScene, entityId) }
	pub fn _DOES_ANIM_SCENE_OWNERSHIP_OF_ENTITY_EXIST(animScene: AnimScene, entityName: & CStr) -> bool { invoke!(0x9D1ECA9337BE9FC3, animScene, entityName) }
	pub fn LOAD_ANIM_SCENE(animScene: AnimScene) { invoke_ignore!(0xAF068580194D9DC7, animScene) }
	pub fn IS_ANIM_SCENE_LOADED(animScene: AnimScene, p1: bool, p2: bool) -> bool { invoke!(0x477122B8D05E7968, animScene, p1, p2) }
	pub fn _IS_ANIM_SCENE_LOADING(animScene: AnimScene, p1: bool) -> bool { invoke!(0x59606519FF9D3EC2, animScene, p1) }
	pub fn _IS_ANIM_SCENE_METADATA_ASSET_IN_RANGE_LOADING(animScene: AnimScene, p1: bool) -> bool { invoke!(0xF8D1D2DAB6007EEF, animScene, p1) }
	pub fn IS_ANIM_SCENE_METADATA_LOADED(animScene: AnimScene, p1: bool) -> bool { invoke!(0x95531A4A20CCE7BC, animScene, p1) }
	pub fn _GET_ANIM_SCENE_PLAYBACK_LIST_PHASE_AUDIO_LOAD_STRESS(animScene: AnimScene, phaseName: & CStr) -> i32 { invoke!(0x9E036D5204FFBBC8, animScene, phaseName) }
	pub fn START_ANIM_SCENE(animScene: AnimScene) { invoke_ignore!(0xF4D94AF761768700, animScene) }
	pub fn RESET_ANIM_SCENE(animScene: AnimScene, playbackListName: & CStr) { invoke_ignore!(0x8FDF221F13537936, animScene, playbackListName) }
	pub fn ABORT_ANIM_SCENE(animScene: AnimScene, p1: bool) { invoke_ignore!(0x718CF1328D20C2B3, animScene, p1) }
	pub fn RESUME_ANIM_SCENE_FROM_LAST_CHECKPOINT(animScene: AnimScene) { invoke_ignore!(0x8E1BA705F63C1925, animScene) }
	pub fn IS_ANIM_SCENE_RUNNING(animScene: AnimScene, p1: bool) -> bool { invoke!(0xCBFC7725DE6CE2E0, animScene, p1) }
	pub fn IS_ANIM_SCENE_FINISHED(animScene: AnimScene, p1: bool) -> bool { invoke!(0xD8254CB2C586412B, animScene, p1) }
	pub fn IS_ANIM_SCENE_EXITING_THIS_FRAME(animScene: AnimScene) -> bool { invoke!(0xCDC5512A407CF08D, animScene) }
	pub fn HAS_ANIM_SCENE_EXITED(animScene: AnimScene, p1: bool) -> bool { invoke!(0xF94692EB9DC15D74, animScene, p1) }
	pub fn _0x73616E64696C132E(animScene: AnimScene, p1: bool) -> bool { invoke!(0x73616E64696C132E, animScene, p1) }
	pub fn IS_ANIM_SCENE_IN_SECTION(animScene: AnimScene, sectionName: & CStr, p2: bool) -> bool { invoke!(0x8D81E7824B7753F7, animScene, sectionName, p2) }
	pub fn _IS_ANIM_SCENE_SKIPPABLE(animScene: AnimScene) -> bool { invoke!(0x4CDFFE3189EBDBD0, animScene) }
	pub fn _IS_ANIM_SCENE_ABORTED(animScene: AnimScene) -> bool { invoke!(0x34A0671BE613D3D0, animScene) }
	pub fn _0xD70C7A30412F8FA0(animScene: AnimScene) -> bool { invoke!(0xD70C7A30412F8FA0, animScene) }
	pub fn _0x9AAE3C1148A09BCA(animScene: AnimScene) -> bool { invoke!(0x9AAE3C1148A09BCA, animScene) }
	pub fn _0xA96619FE85159ED2(animScene: AnimScene) -> bool { invoke!(0xA96619FE85159ED2, animScene) }
	pub fn FADE_ANIM_SCENE_AUDIO_IN(animScene: AnimScene, p1: f32) { invoke_ignore!(0xA41351EA2A18A0AD, animScene, p1) }
	pub fn FADE_ANIM_SCENE_AUDIO_OUT(animScene: AnimScene, p1: f32) { invoke_ignore!(0x323E3AD772BA5D57, animScene, p1) }
	pub fn BLOCK_ANIM_SCENE_FADING_NEXT_FRAME(p0: bool, p1: bool) { invoke_ignore!(0x1B70811D3BF75DB9, p0, p1) }
	pub fn SET_ANIM_SCENE_ORIGIN(animScene: AnimScene, posX: f32, posY: f32, posZ: f32, rotX: f32, rotY: f32, rotZ: f32, order: i32) { invoke_ignore!(0x020894BF17A02EF2, animScene, posX, posY, posZ, rotX, rotY, rotZ, order) }
	pub fn GET_ANIM_SCENE_ORIGIN(animScene: AnimScene, position: &mut Vector3, rotation: &mut Vector3, order: i32) { invoke_ignore!(0xADF1D53F3B1FE0A7, animScene, position, rotation, order) }
	pub fn SET_ANIM_SCENE_PAUSED(animScene: AnimScene, toggle: bool) { invoke_ignore!(0xD6824B7D24DC0CE0, animScene, toggle) }
	pub fn _IS_ANIM_SCENE_PAUSED(animScene: AnimScene) -> bool { invoke!(0x4B4038796F0D6566, animScene) }
	pub fn SET_ANIM_SCENE_RATE(animScene: AnimScene, rate: f32) { invoke_ignore!(0x75820B801CFF262A, animScene, rate) }
	pub fn _GET_ANIM_SCENE_RATE(animScene: AnimScene) -> f32 { invoke!(0x43C21623E42B821B, animScene) }
	pub fn GET_ANIM_SCENE_PHASE(animScene: AnimScene) -> f32 { invoke!(0x3FBC3F51BF12DFBF, animScene) }
	pub fn _GET_ANIM_SCENE_TIME(animScene: AnimScene) -> f32 { invoke!(0x61BE7D6186260002, animScene) }
	pub fn _GET_ANIM_SCENE_DURATION(animScene: AnimScene) -> f32 { invoke!(0x49F1D143ADE32656, animScene) }
	pub fn SET_ANIM_SCENE_ENTITY(animScene: AnimScene, entityName: & CStr, entity: Entity, flags: i32) { invoke_ignore!(0x8B720AD451CA2AB3, animScene, entityName, entity, flags) }
	pub fn REMOVE_ANIM_SCENE_ENTITY(animScene: AnimScene, entityName: & CStr, entity: Entity) { invoke_ignore!(0x2BF96692C67F3E53, animScene, entityName, entity) }
	pub fn IS_ENTITY_EXITING_ANIM_SCENE_THIS_FRAME(animScene: AnimScene, entityName: & CStr) -> bool { invoke!(0x005E6F28DD7ED58D, animScene, entityName) }
	pub fn COULD_ANIM_SCENE_ENTITY_REACH_EXIT_NEXT_FRAME(animScene: AnimScene, entityName: & CStr, p2: Any, p3: Any) -> bool { invoke!(0x73616E64696C616E, animScene, entityName, p2, p3) }
	pub fn HAS_ENTITY_EXITED_ANIM_SCENE(animScene: AnimScene, entityName: & CStr) -> bool { invoke!(0xB89FCFF19DAFFF28, animScene, entityName) }
	pub fn _HAS_ENTITY_ENTERED_ANIM_SCENE(animScene: AnimScene, entityName: & CStr) -> bool { invoke!(0x337F1CC8EE895601, animScene, entityName) }
	pub fn _GET_ANIM_SCENE_PED(animScene: AnimScene, name: & CStr, isNetwork: bool) -> Ped { invoke!(0xE5822422197BBBA3, animScene, name, isNetwork) }
	pub fn _GET_ANIM_SCENE_OBJECT(animScene: AnimScene, name: & CStr, isNetwork: bool) -> Object { invoke!(0xFB5674687A1B2814, animScene, name, isNetwork) }
	pub fn _GET_ANIM_SCENE_VEHICLE(animScene: AnimScene, name: & CStr, isNetwork: bool) -> Vehicle { invoke!(0x430EE0A19BC5A287, animScene, name, isNetwork) }
	pub fn SET_ANIM_SCENE_BOOL(animScene: AnimScene, name: & CStr, value: bool, p3: bool) { invoke_ignore!(0x519E96C2C68B404B, animScene, name, value, p3) }
	pub fn GET_ANIM_SCENE_BOOL(animScene: AnimScene, name: & CStr) -> bool { invoke!(0x07A6F6447ECA9B64, animScene, name) }
	pub fn SET_ANIM_SCENE_FLOAT(animScene: AnimScene, name: & CStr, value: f32, p3: bool, p4: bool) { invoke_ignore!(0x6BC5104E68CBEFE8, animScene, name, value, p3, p4) }
	pub fn GET_ANIM_SCENE_FLOAT(animScene: AnimScene, name: & CStr) -> f32 { invoke!(0xCC24CB07F60B496E, animScene, name) }
	pub fn SET_ANIM_SCENE_INT(animScene: AnimScene, name: & CStr, value: i32, p3: bool) { invoke_ignore!(0x3A379D2166CF5B92, animScene, name, value, p3) }
	pub fn GET_ANIM_SCENE_INT(animScene: AnimScene, name: & CStr) -> i32 { invoke!(0x2B7277484CC095FD, animScene, name) }
	pub fn GET_ANIM_SCENE_ENTITY_LOCATION_DATA(animScene: AnimScene, entityName: & CStr, matrix: &mut Vector3, p3: bool, playbackListName: & CStr, p5: i32) -> bool { invoke!(0x8398438D8F14F56D, animScene, entityName, matrix, p3, playbackListName, p5) }
	pub fn IS_ENTITY_PLAYING_ANIM_SCENE(entity: Entity, animScene: AnimScene) -> bool { invoke!(0x3AB6C7B0BB0DF4B1, entity, animScene) }
	pub fn ATTACH_ANIM_SCENE_TO_ENTITY(animScene: AnimScene, entity: Entity, p2: i32) { invoke_ignore!(0xDC418495DBA327A1, animScene, entity, p2) }
	pub fn ATTACH_ANIM_SCENE_TO_ENTITY_PRESERVING_LOCATION(animScene: AnimScene, entity: Entity, p2: i32) { invoke_ignore!(0x1C0B105C3F30B88D, animScene, entity, p2) }
	pub fn DETACH_ANIM_SCENE(animScene: AnimScene) { invoke_ignore!(0x6843A1AA3A336DFF, animScene) }
	pub fn DETACH_ANIM_SCENE_PRESERVING_LOCATION(animScene: AnimScene) { invoke_ignore!(0xA2507C4948C83D2E, animScene) }
	pub fn TAKE_OWNERSHIP_OF_ANIM_SCENE(animScene: AnimScene) { invoke_ignore!(0xF7A4C571E572D237, animScene) }
	pub fn CHECK_OWNERSHIP_OF_ANIM_SCENE(animScene: AnimScene) -> bool { invoke!(0x661B8683611B9B97, animScene) }
	pub fn SET_ANIM_SCENE_PLAYBACK_LIST(animScene: AnimScene, playbackListName: & CStr) { invoke_ignore!(0xAB5E7CAB074D6B84, animScene, playbackListName) }
	pub fn _0x1C5D33A4293E6DDE(animScene: AnimScene, phaseName: & CStr) -> bool { invoke!(0x1C5D33A4293E6DDE, animScene, phaseName) }
	pub fn _DOES_ANIM_SCENE_PLAY_LIST_EXIST(animScene: AnimScene, playbackListName: & CStr) -> bool { invoke!(0xA9016536015DE29D, animScene, playbackListName) }
	pub fn SET_ANIM_SCENE_PLAY_LIST(animScene: AnimScene, playlistName: & CStr, p2: bool) { invoke_ignore!(0x15598CFB25F3DC7E, animScene, playlistName, p2) }
	pub fn _IS_ANIM_SCENE_PLAYBACK_LIST_PHASE_ACTIVE(animScene: AnimScene, phaseName: & CStr) -> bool { invoke!(0x1F0E401031E20146, animScene, phaseName) }
	pub fn REQUEST_ANIM_SCENE_PLAY_LIST(animScene: AnimScene, playlistName: & CStr) -> bool { invoke!(0xDF7B5144E25CD3FE, animScene, playlistName) }
	pub fn _RELEASE_ANIM_SCENE_PLAY_LIST(animScene: AnimScene, playlistName: & CStr) -> bool { invoke!(0xAE6ADA8FE7E84ACC, animScene, playlistName) }
	pub fn _GET_ANIM_SCENE_DICT(animScene: AnimScene) -> Hash { invoke!(0xAE5ADA4FE3E21ADC, animScene) }
	pub fn _0x1407F5115FB9583E(animScene: AnimScene, p1: & CStr) -> bool { invoke!(0x1407F5115FB9583E, animScene, p1) }
	pub fn _IS_ANIM_SCENE_PLAYBACK_LIST_PHASE_LOADED(animScene: AnimScene, phaseName: & CStr) -> bool { invoke!(0x23E33CB9F4A3F547, animScene, phaseName) }
	pub fn _IS_ANIM_SCENE_PLAYBACK_LIST_PHASE_LOADING(animScene: AnimScene, phaseName: & CStr) -> bool { invoke!(0x0DF57F86FE71DBE5, animScene, phaseName) }
	pub fn _0x1AD896BF43619551() { invoke_ignore!(0x1AD896BF43619551) }
	pub fn GET_ANIM_SCENE_CURRENT_ACTIVE_CAMERA_COUNT(animScene: AnimScene) -> i32 { invoke!(0x4822A65D5AF64E69, animScene) }
	pub fn _0x5D7BFDA2290B4E39(p0: & CStr) -> bool { invoke!(0x5D7BFDA2290B4E39, p0) }
	pub fn _IS_MGM_SYSTEM_LOADED(mgmFilename: & CStr) -> bool { invoke!(0xFDFC14799373283F, mgmFilename) }
	pub fn _LOAD_MGM_ASSETS(mgmFilename: & CStr) -> bool { invoke!(0xB727A847862CB00A, mgmFilename) }
	pub fn _CREATE_MGM_SYSTEM(mgmFilename: & CStr) -> i32 { invoke!(0xA1300DE03E5D1973, mgmFilename) }
	pub fn _DELETE_MGM_SYSTEM(mgmHandle: i32) { invoke_ignore!(0x53CB3970BA02E3CC, mgmHandle) }
	pub fn _0xB1A196BAFE650402(mgmHandle: i32, ped: Ped) { invoke_ignore!(0xB1A196BAFE650402, mgmHandle, ped) }
	pub fn _0xAE6DE22DE0ED4554(mgmHandle: i32, ped: Ped) { invoke_ignore!(0xAE6DE22DE0ED4554, mgmHandle, ped) }
	pub fn _0x61B2AAEF645DDAF0(mgmEventHandle: i32, p1: & CStr, seatId: i32, p3: i32, p4: bool) -> bool { invoke!(0x61B2AAEF645DDAF0, mgmEventHandle, p1, seatId, p3, p4) }
	pub fn _SET_MGM_EVENT(mgmEventHandle: i32, p1: & CStr, seatId: Any, p3: i32, p4: f32) { invoke_ignore!(0x07706C4CC9C6CC9E, mgmEventHandle, p1, seatId, p3, p4) }
	pub fn _0x3641FCD53E59B335(mgmHandle: i32, ped: Ped, secondaryVoiceString: & CStr) { invoke_ignore!(0x3641FCD53E59B335, mgmHandle, ped, secondaryVoiceString) }
	pub fn _SET_BREAKOUT_ARCHETYPE(ped: Ped, archetype: & CStr) { invoke_ignore!(0x99B2A2E3655DEAF1, ped, archetype) }
	pub fn _CLEAR_BREAKOUT_ARCHETYPE(ped: Ped) { invoke_ignore!(0xBC781D24AA11F179, ped) }
	pub fn _0x3B393716C3FD8237(ped: Ped) -> bool { invoke!(0x3B393716C3FD8237, ped) }
	pub fn _0xE12D7B4B959644CD() { invoke_ignore!(0xE12D7B4B959644CD) }
	pub fn _0xC1193521E3B9FADD(entity: Entity, p1: bool) { invoke_ignore!(0xC1193521E3B9FADD, entity, p1) }
	pub fn _REQUEST_PHOTO_MODE_FREEZE() { invoke_ignore!(0x7C709C01D43D94CD) }
	pub fn _REQUEST_PHOTO_MODE_DEFREEZE() { invoke_ignore!(0x41AFA5F228B0B6B0) }
	pub fn _0x2DB524750DC41ED4() -> bool { invoke!(0x2DB524750DC41ED4) }
	pub fn _0xEA41D44A8D42057B() -> bool { invoke!(0xEA41D44A8D42057B) }
	pub fn _PAUSE_SCRIPT_THREADS(toggle: bool) { invoke_ignore!(0x37C1257849DEF24A, toggle) }
	pub fn _0xCDCD7B2D49AEE73A(p0: bool) { invoke_ignore!(0xCDCD7B2D49AEE73A, p0) }
	pub fn WAS_ANIM_SCENE_SKIPPED(animScene: AnimScene) -> bool { invoke!(0xEF324E9550A394D5, animScene) }
}
pub mod _NAMESPACE4 {
	use std::ffi::CStr;
	use crate::core::scripthook::native::*;
	use crate::core::types::*;

	pub fn _REPORT_PLAYER_BAD_SPORT_BEHAVIOR(gamerHandle: &mut Any, badSportBehaviorType: i32) { invoke_ignore!(0xC31C44C43B48FDE3, gamerHandle, badSportBehaviorType) }
}
pub mod ATTRIBUTE {
	use std::ffi::CStr;
	use crate::core::scripthook::native::*;
	use crate::core::types::*;

	pub fn SET_ATTRIBUTE_BASE_RANK(ped: Ped, attributeIndex: i32, newValue: i32) { invoke_ignore!(0x5DA12E025D47D4E5, ped, attributeIndex, newValue) }
	pub fn GET_ATTRIBUTE_RANK(ped: Ped, attributeIndex: i32) -> i32 { invoke!(0xA4C8E23E29040DE0, ped, attributeIndex) }
	pub fn GET_ATTRIBUTE_BASE_RANK(ped: Ped, attributeIndex: i32) -> i32 { invoke!(0x147149F2E909323C, ped, attributeIndex) }
	pub fn GET_ATTRIBUTE_BONUS_RANK(ped: Ped, coreIndex: i32) -> i32 { invoke!(0x0EFA71F4B4330E04, ped, coreIndex) }
	pub fn GET_MAX_ATTRIBUTE_RANK(ped: Ped, attributeIndex: i32) -> i32 { invoke!(0x704674A0535A471D, ped, attributeIndex) }
	pub fn SET_ATTRIBUTE_BONUS_RANK(ped: Ped, attributeIndex: i32, newValue: i32) { invoke_ignore!(0x920F9488BD115EFB, ped, attributeIndex, newValue) }
	pub fn GET_DEFAULT_ATTRIBUTE_RANK(ped: Ped, attributeIndex: i32) -> i32 { invoke!(0x958DD43D41F89A47, ped, attributeIndex) }
	pub fn GET_DEFAULT_MAX_ATTRIBUTE_RANK(ped: Ped, attributeIndex: i32) -> i32 { invoke!(0x7C059C55AD940CB4, ped, attributeIndex) }
	pub fn ADD_ATTRIBUTE_POINTS(ped: Ped, attributeIndex: i32, p2: i32) { invoke_ignore!(0x75415EE0CB583760, ped, attributeIndex, p2) }
	pub fn SET_ATTRIBUTE_POINTS(ped: Ped, attributeIndex: i32, p2: i32) { invoke_ignore!(0x09A59688C26D88DF, ped, attributeIndex, p2) }
	pub fn GET_ATTRIBUTE_POINTS(ped: Ped, attributeIndex: i32) -> i32 { invoke!(0x219DA04BAA9CB065, ped, attributeIndex) }
	pub fn GET_MAX_ATTRIBUTE_POINTS(ped: Ped, attributeIndex: i32) -> i32 { invoke!(0x223BF310F854871C, ped, attributeIndex) }
	pub fn _SET_ATTRIBUTE_CORE_VALUE(ped: Ped, coreIndex: i32, value: i32) { invoke_ignore!(0xC6258F41D86676E0, ped, coreIndex, value) }
	pub fn _GET_ATTRIBUTE_CORE_VALUE(ped: Ped, coreIndex: i32) -> i32 { invoke!(0x36731AC041289BB1, ped, coreIndex) }
	pub fn GET_DEFAULT_ATTRIBUTE_POINTS_NEEDED_FOR_RANK(modelHash: Hash, attributeIndex: i32, rank: i32) -> i32 { invoke!(0x94A7F191DB49A44D, modelHash, attributeIndex, rank) }
	pub fn ENABLE_ATTRIBUTE_OVERPOWER(ped: Ped, attributeIndex: i32, value: f32, makeSound: bool) { invoke_ignore!(0xF6A7C08DF2E28B28, ped, attributeIndex, value, makeSound) }
	pub fn _ENABLE_ATTRIBUTE_CORE_OVERPOWER(ped: Ped, coreIndex: i32, value: f32, makeSound: bool) { invoke_ignore!(0x4AF5A4C7B9157D14, ped, coreIndex, value, makeSound) }
	pub fn DISABLE_ATTRIBUTE_OVERPOWER(ped: Ped, attributeIndex: i32) { invoke_ignore!(0xF8DAC3D85636C241, ped, attributeIndex) }
	pub fn _IS_ATTRIBUTE_OVERPOWERED(ped: Ped, attributeIndex: i32) -> bool { invoke!(0x103C2F885ABEB00B, ped, attributeIndex) }
	pub fn _IS_ATTRIBUTE_CORE_OVERPOWERED(ped: Ped, coreIndex: i32) -> bool { invoke!(0x200373A8DF081F22, ped, coreIndex) }
	pub fn _GET_ATTRIBUTE_OVERPOWER_SECONDS_LEFT(ped: Ped, attributeIndex: i32) -> f32 { invoke!(0x4C9F782180712742, ped, attributeIndex) }
	pub fn _GET_ATTRIBUTE_CORE_OVERPOWER_SECONDS_LEFT(ped: Ped, coreIndex: i32) -> f32 { invoke!(0xB429F58803D285B1, ped, coreIndex) }
	pub fn _START_ITEM_PREVIEW(p0: Any, p1: i32) { invoke_ignore!(0x7E2C766ADB2C5F1A, p0, p1) }
	pub fn STOP_ITEM_PREVIEW() { invoke_ignore!(0xD962F8579D702DB5) }
	pub fn _SET_STATUS_EFFECT_CORE_ICON(statusEffectType: i32) { invoke_ignore!(0xA4D3A1C008F250DF, statusEffectType) }
	pub fn _SET_STATUS_EFFECT_PERIODIC_ICON(statusEffectType: i32) { invoke_ignore!(0xFB6E111908502871, statusEffectType) }
	pub fn _STOP_STATUS_EFFECT_PERIODIC_ICON(statusEffectType: i32) { invoke_ignore!(0x3FC4C027FD0936F4, statusEffectType) }
}
pub mod AUDIO {
	use std::ffi::CStr;
	use crate::core::scripthook::native::*;
	use crate::core::types::*;

	pub fn _0x7455CD705F7E933E() { invoke_ignore!(0x7455CD705F7E933E) }
	pub fn CLEAR_CONVERSATION_HISTORY() { invoke_ignore!(0x33D51F801CB16E4F) }
	pub fn _CLEAR_CONVERSATION_HISTORY_FOR_SCRIPTED_CONVERSATION(convoRoot: & CStr) { invoke_ignore!(0xEF51242E35242B47, convoRoot) }
	pub fn CREATE_NEW_SCRIPTED_CONVERSATION(convoRoot: & CStr) -> bool { invoke!(0xD2C91A0B572AAE56, convoRoot) }
	pub fn _0xDF947FE0D551684E(ped: Ped, p1: & CStr) -> bool { invoke!(0xDF947FE0D551684E, ped, p1) }
	pub fn ADD_PED_TO_CONVERSATION(convoRoot: & CStr, ped: Ped, characterName: & CStr) { invoke_ignore!(0x95D9F4BC443956E7, convoRoot, ped, characterName) }
	pub fn _0xA2323A2EAE32A290(listeningToPed: Ped, ped: Ped, listenerName: & CStr) { invoke_ignore!(0xA2323A2EAE32A290, listeningToPed, ped, listenerName) }
	pub fn _0x79F9C57B8D0DFE90(convoRoot: & CStr, animScene: AnimScene) -> bool { invoke!(0x79F9C57B8D0DFE90, convoRoot, animScene) }
	pub fn START_SCRIPT_CONVERSATION(convoRoot: & CStr, p1: bool, p2: bool, clone: bool) { invoke_ignore!(0x6B17C62C9635D2DC, convoRoot, p1, p2, clone) }
	pub fn PRELOAD_SCRIPT_CONVERSATION(convoRoot: & CStr, p1: bool, p2: bool, clone: bool) { invoke_ignore!(0x3B3CAD6166916D87, convoRoot, p1, p2, clone) }
	pub fn START_PRELOADED_CONVERSATION(convoRoot: & CStr) { invoke_ignore!(0x23641AFE870AF385, convoRoot) }
	pub fn _0x0CB3D1919E8D7CBA(convoRoot: & CStr) -> bool { invoke!(0x0CB3D1919E8D7CBA, convoRoot) }
	pub fn _0xFE5C6177064BD390(p0: bool) -> bool { invoke!(0xFE5C6177064BD390, p0) }
	pub fn _IS_SCRIPTED_CONVERSATION_CREATED(convoRoot: & CStr) -> bool { invoke!(0xD89504D9D7D5057D, convoRoot) }
	pub fn _0x5A13586A9447931F(p0: bool) -> bool { invoke!(0x5A13586A9447931F, p0) }
	pub fn IS_SCRIPTED_CONVERSATION_LOADED(convoRoot: & CStr) -> bool { invoke!(0xDF0D54BE7A776737, convoRoot) }
	pub fn _IS_ANY_CONVERSATION_PLAYING(p0: bool) -> bool { invoke!(0xA2CAC9DEF0195E6F, p0) }
	pub fn IS_SCRIPTED_CONVERSATION_PLAYING(p0: & CStr) -> bool { invoke!(0x1ECC76792F661CF5, p0) }
	pub fn _IS_SCRIPTED_CONVERSION_ONGOING(p0: & CStr) -> bool { invoke!(0xF01C570E0A0A1E67, p0) }
	pub fn _0x847748AE5D7B1071(p0: bool) -> bool { invoke!(0x847748AE5D7B1071, p0) }
	pub fn _0xD0730C1FA40348D9(convoRoot: & CStr) -> bool { invoke!(0xD0730C1FA40348D9, convoRoot) }
	pub fn GET_CURRENT_SCRIPTED_CONVERSATION_LINE(p0: & CStr) -> i32 { invoke!(0x480357EE890C295A, p0) }
	pub fn PAUSE_SCRIPTED_CONVERSATION(p0: & CStr, p1: bool, p2: bool, p3: bool, p4: bool) { invoke_ignore!(0x8530AD776CD72B12, p0, p1, p2, p3, p4) }
	pub fn RESTART_SCRIPTED_CONVERSATION(p0: & CStr) { invoke_ignore!(0x9AEB285D1818C9AC, p0) }
	pub fn _STOP_ALL_SCRIPTED_CONVERSIONS(p0: bool, p1: bool, p2: bool) { invoke_ignore!(0x36559148B78853B3, p0, p1, p2) }
	pub fn STOP_SCRIPTED_CONVERSATION(p0: & CStr, p1: bool, p2: bool) -> i32 { invoke!(0xD79DEEFB53455EBA, p0, p1, p2) }
	pub fn SKIP_TO_NEXT_SCRIPTED_CONVERSATION_LINE(p0: & CStr) { invoke_ignore!(0x9663FE6B7A61EB00, p0) }
	pub fn _0xF336E9F989B3518F(p0: & CStr) -> i32 { invoke!(0xF336E9F989B3518F, p0) }
	pub fn _0x254B0241E964B450(p0: & CStr, currentScriptedConvoLine: i32) -> Ped { invoke!(0x254B0241E964B450, p0, currentScriptedConvoLine) }
	pub fn _0x152ED1B56E8F1F50(p0: & CStr, currentScriptedConvoLine: i32) -> Ped { invoke!(0x152ED1B56E8F1F50, p0, currentScriptedConvoLine) }
	pub fn _0x935DBD96D4A3DA1F(p0: & CStr, currentScriptedConvoLine: i32) -> i32 { invoke!(0x935DBD96D4A3DA1F, p0, currentScriptedConvoLine) }
	pub fn _0x295859EB18F48D82(p0: & CStr) -> i32 { invoke!(0x295859EB18F48D82, p0) }
	pub fn _0x40CA665AB9D8D505(convoRoot: & CStr, singleLineIndex: i32) { invoke_ignore!(0x40CA665AB9D8D505, convoRoot, singleLineIndex) }
	pub fn _0xF232C2C546AC16D0(p0: & CStr) { invoke_ignore!(0xF232C2C546AC16D0, p0) }
	pub fn _0x1E6F9A9FE1A99F36(audSpeechEvent: & CStr) { invoke_ignore!(0x1E6F9A9FE1A99F36, audSpeechEvent) }
	pub fn REGISTER_SCRIPT_WITH_AUDIO(p0: bool) { invoke_ignore!(0xC6ED9D5092438D91, p0) }
	pub fn UNREGISTER_SCRIPT_WITH_AUDIO() { invoke_ignore!(0xA8638BE228D4751A) }
	pub fn REQUEST_SCRIPT_AUDIO_BANK(audioBank: & CStr) -> bool { invoke!(0x2F844A8B08D76685, audioBank) }
	pub fn RELEASE_NAMED_SCRIPT_AUDIO_BANK(audioBank: & CStr) { invoke_ignore!(0x77ED170667F50170, audioBank) }
	pub fn RELEASE_SCRIPT_AUDIO_BANK() { invoke_ignore!(0x7A2D8AD0A9EB9C3F) }
	pub fn GET_SOUND_ID() -> i32 { invoke!(0x430386FE9BF80B45) }
	pub fn RELEASE_SOUND_ID(soundId: i32) { invoke_ignore!(0x353FC880830B88FA, soundId) }
	pub fn PLAY_SOUND(audioName: & CStr, audioRef: & CStr, p2: bool, p3: Any, p4: bool, p5: Any) { invoke_ignore!(0x7FF4944CC209192D, audioName, audioRef, p2, p3, p4, p5) }
	pub fn PLAY_SOUND_FRONTEND(audioName: & CStr, audioRef: & CStr, p2: bool, p3: Any) { invoke_ignore!(0x67C540AA08E4A6F5, audioName, audioRef, p2, p3) }
	pub fn _PLAY_SOUND_FROM_ITEM(item: Hash, soundSet: Hash, p2: Any) { invoke_ignore!(0xE8EAFF7B41EDD291, item, soundSet, p2) }
	pub fn PLAY_SOUND_FROM_ENTITY(audioName: & CStr, entity: Entity, audioRef: & CStr, isNetwork: bool, p4: Any, p5: Any) { invoke_ignore!(0x6FB1DA3CA9DA7D90, audioName, entity, audioRef, isNetwork, p4, p5) }
	pub fn _PLAY_SOUND_FROM_POSITION(audioName: & CStr, x: f32, y: f32, z: f32, audioRef: & CStr, isNetwork: bool, p6: Any, p7: bool, p8: Any) { invoke_ignore!(0xCCE219C922737BFA, audioName, x, y, z, audioRef, isNetwork, p6, p7, p8) }
	pub fn _STOP_SOUND_WITH_NAME(audioName: & CStr, audioRef: & CStr) { invoke_ignore!(0x0F2A2175734926D8, audioName, audioRef) }
	pub fn _0x580D71DFE0088E34(audioName: & CStr, audioRef: & CStr) -> bool { invoke!(0x580D71DFE0088E34, audioName, audioRef) }
	pub fn _IS_SCRIPTED_AUDIO_CUSTOM(item: Hash, soundSet: Hash) -> bool { invoke!(0x6DF942C4179BE5AB, item, soundSet) }
	pub fn _SET_VARIABLE_ON_SOUND_WITH_NAME(variableName: & CStr, variableValue: f32, audioName: & CStr, audioRef: & CStr) { invoke_ignore!(0x9821B68CD3E05F2B, variableName, variableValue, audioName, audioRef) }
	pub fn _SET_WHISTLE_CONFIG_FOR_PED(ped: Ped, whistleConfig: & CStr, value: f32) { invoke_ignore!(0x9963681A8BC69BF3, ped, whistleConfig, value) }
	pub fn _SET_SOUND_RELATIONSHIP_ON_PED(ped: Ped, p1: & CStr, p2: & CStr) { invoke_ignore!(0x2E31ACA7477CF00F, ped, p1, p2) }
	pub fn _PLAY_SOUND_FRONTEND_WITH_SOUND_ID(soundId: i32, name: & CStr, soundSet: & CStr, p3: bool) { invoke_ignore!(0xCE5D0FFE83939AF1, soundId, name, soundSet, p3) }
	pub fn _PLAY_SOUND_FROM_ENTITY_WITH_SET(soundId: i32, soundName: & CStr, entity: Entity, soundsetName: & CStr, p4: bool, p5: Any) { invoke_ignore!(0xF1C5310FEAA36B48, soundId, soundName, entity, soundsetName, p4, p5) }
	pub fn _PLAY_SOUND_FROM_POSITION_WITH_ID(soundId: i32, soundName: & CStr, x: f32, y: f32, z: f32, soundsetName: & CStr, p6: bool, p7: i32, p8: bool) { invoke_ignore!(0xDCF5BA95BBF0FABA, soundId, soundName, x, y, z, soundsetName, p6, p7, p8) }
	pub fn _UPDATE_SOUND_POSITION(soundId: i32, x: f32, y: f32, z: f32) { invoke_ignore!(0x0286617C8FC50A53, soundId, x, y, z) }
	pub fn _STOP_SOUND_WITH_ID(soundId: i32) { invoke_ignore!(0x3210BCB36AF7621B, soundId) }
	pub fn _SET_VARIABLE_ON_SOUND_WITH_ID(soundId: i32, variableName: & CStr, variableValue: f32) { invoke_ignore!(0x503703EC1781B7D6, soundId, variableName, variableValue) }
	pub fn PREPARE_SOUND(soundName: & CStr, soundsetName: & CStr, soundId: i32) -> bool { invoke!(0xE368E8422C860BA7, soundName, soundsetName, soundId) }
	pub fn _RELEASE_SHARD_SOUNDS(soundName: & CStr, soundsetName: & CStr) { invoke_ignore!(0x9D746964E0CF2C5F, soundName, soundsetName) }
	pub fn PREPARE_SOUNDSET(soundsetName: & CStr, p1: bool) -> bool { invoke!(0xD9130842D7226045, soundsetName, p1) }
	pub fn _RELEASE_SOUNDSET(soundsetName: & CStr) { invoke_ignore!(0x531A78D6BF27014B, soundsetName) }
	pub fn PREPARE_SOUND_WITH_ENTITY(soundName: & CStr, entity: Entity, soundsetName: & CStr, soundId: i32) -> bool { invoke!(0x4AD019591E94C064, soundName, entity, soundsetName, soundId) }
	pub fn _0x3E93DDDCBB6111E4(p0: & CStr, p1: f32) { invoke_ignore!(0x3E93DDDCBB6111E4, p0, p1) }
	pub fn _HAS_SOUND_AUDIO_NAME_FINISHED(audioName: & CStr, soundsetName: & CStr) -> bool { invoke!(0x714A0EA7DE1167BE, audioName, soundsetName) }
	pub fn _HAS_SOUND_ID_FINISHED(soundId: i32) -> bool { invoke!(0x84848E1C0FC67DBB, soundId) }
	pub fn PLAY_PED_AMBIENT_SPEECH_NATIVE(speaker: Ped, params: &mut Any) -> bool { invoke!(0x8E04FEDD28D42462, speaker, params) }
	pub fn PLAY_AMBIENT_SPEECH_FROM_POSITION_NATIVE(x: f32, y: f32, z: f32, params: &mut Any) -> bool { invoke!(0xED640017ED337E45, x, y, z, params) }
	pub fn _0x72E4D1C4639BC465(p0: Entity, p1: Any) -> Any { invoke!(0x72E4D1C4639BC465, p0, p1) }
	pub fn _0xB18FEC133C7C6C69(p0: Any) -> Any { invoke!(0xB18FEC133C7C6C69, p0) }
	pub fn _0xDC93F0948F2C28F4(p0: Any) { invoke_ignore!(0xDC93F0948F2C28F4, p0) }
	pub fn _0x0D7FD6A55FD63AEF(speechEventType: i32, p1: i32, p2: bool) { invoke_ignore!(0x0D7FD6A55FD63AEF, speechEventType, p1, p2) }
	pub fn _0x660A8F876DF1D4F8(speechEventType: i32) { invoke_ignore!(0x660A8F876DF1D4F8, speechEventType) }
	pub fn _0x380A2E353AD30917(p0: Any, p1: Any, p2: Any) { invoke_ignore!(0x380A2E353AD30917, p0, p1, p2) }
	pub fn _0x0FAF7171BF613B80(p0: Any) { invoke_ignore!(0x0FAF7171BF613B80, p0) }
	pub fn PLAY_PAIN(ped: Ped, painId: i32, p2: f32, p3: bool, isNetwork: bool) { invoke_ignore!(0xBC9AE166038A5CEC, ped, painId, p2, p3, isNetwork) }
	pub fn _0x6652B0C8F3D414D0(p0: Any) { invoke_ignore!(0x6652B0C8F3D414D0, p0) }
	pub fn _0xF092B6030D6FD49C(ropeId: i32, name: & CStr) { invoke_ignore!(0xF092B6030D6FD49C, ropeId, name) }
	pub fn _0x2651DDC0EA269073(ropeId: i32, p1: f32) { invoke_ignore!(0x2651DDC0EA269073, ropeId, p1) }
	pub fn SET_AMBIENT_VOICE_NAME(ped: Ped, name: & CStr) { invoke_ignore!(0x6C8065A3B780185B, ped, name) }
	pub fn _SET_VOFX_PED_VOICE(ped: Ped, voice: Hash) { invoke_ignore!(0x2703EFB583F0949A, ped, voice) }
	pub fn STOP_CURRENT_PLAYING_SPEECH(ped: Ped, p1: Any) { invoke_ignore!(0x79D2F0E66F81D90D, ped, p1) }
	pub fn STOP_CURRENT_PLAYING_AMBIENT_SPEECH(ped: Ped, p1: Any) { invoke_ignore!(0xB8BEC0CA6F0EDB0F, ped, p1) }
	pub fn IS_AMBIENT_SPEECH_PLAYING(ped: Ped) -> bool { invoke!(0x9072C8B49907BFAD, ped) }
	pub fn IS_SCRIPTED_SPEECH_PLAYING(p0: Any) -> bool { invoke!(0xCC9AA18DCC7084F4, p0) }
	pub fn IS_ANY_SPEECH_PLAYING(ped: Ped) -> bool { invoke!(0x729072355FA39EC9, ped) }
	pub fn _0x2B101AD9F651243A() -> Any { invoke!(0x2B101AD9F651243A) }
	pub fn _0x4A98E228A936DBCC(p0: Any) -> Any { invoke!(0x4A98E228A936DBCC, p0) }
	pub fn _0x6BFFB7C276866996(p0: Any) -> Any { invoke!(0x6BFFB7C276866996, p0) }
	pub fn DOES_CONTEXT_EXIST_FOR_THIS_PED(ped: Ped, speechName: & CStr, unk: bool) -> bool { invoke!(0x49B99BF3FDA89A7A, ped, speechName, unk) }
	pub fn _0xF0EE69F500952FA5(p0: Any) -> Any { invoke!(0xF0EE69F500952FA5, p0) }
	pub fn _0x9D6DEC9791A4E501(p0: Any, p1: Any, p2: Any, p3: Any) -> Any { invoke!(0x9D6DEC9791A4E501, p0, p1, p2, p3) }
	pub fn _0x864A842B86993851(ped: Ped) { invoke_ignore!(0x864A842B86993851, ped) }
	pub fn IS_PED_IN_CURRENT_CONVERSATION(p0: & CStr, ped: Ped, p2: Any) -> bool { invoke!(0x049E937F18F4020C, p0, ped, p2) }
	pub fn _IS_PED_IN_ANY_CONVERSATION(ped: Ped, p1: bool) -> bool { invoke!(0x54B187F111D9C6F8, ped, p1) }
	pub fn SET_PED_IS_DRUNK(ped: Ped, toggle: bool) { invoke_ignore!(0x95D2D383D5396B8A, ped, toggle) }
	pub fn _0x3A00D87B20A2A5E4(p0: Any, p1: Any) { invoke_ignore!(0x3A00D87B20A2A5E4, p0, p1) }
	pub fn _0xD47D47EFBF103FB8(p0: Any, p1: Any) { invoke_ignore!(0xD47D47EFBF103FB8, p0, p1) }
	pub fn PLAY_ANIMAL_VOCALIZATION(ped: Ped, vocalizationName: & CStr, p2: bool) { invoke_ignore!(0xEE066C7006C49C0A, ped, vocalizationName, p2) }
	pub fn _PLAY_ANIMAL_VOCALIZATION_PHEROMONE_VIAL_RESPONSE(ped: Ped, p1: Hash, p2: bool) { invoke_ignore!(0x0E53530D9B2DB01D, ped, p1, p2) }
	pub fn IS_ANIMAL_VOCALIZATION_PLAYING(pedHandle: Ped) -> bool { invoke!(0xC265DF9FB44A9FBD, pedHandle) }
	pub fn SET_ANIMAL_MOOD(animal: Ped, mood: i32) { invoke_ignore!(0xCC97B29285B1DC3B, animal, mood) }
	pub fn _0xFCDEC42B1C78B7F8(p0: Any, p1: Any) { invoke_ignore!(0xFCDEC42B1C78B7F8, p0, p1) }
	pub fn _0xEB4D592620B8C209(p0: Any) { invoke_ignore!(0xEB4D592620B8C209, p0) }
	pub fn _0xA6847BBA4FCDD13F(p0: Any, p1: Any) { invoke_ignore!(0xA6847BBA4FCDD13F, p0, p1) }
	pub fn SET_STATIC_EMITTER_ENABLED(emitterName: & CStr, toggle: bool) { invoke_ignore!(0x399D2D3B33F1B8EB, emitterName, toggle) }
	pub fn PLAY_END_CREDITS_MUSIC(play: bool) { invoke_ignore!(0xCD536C4D33DCC900, play) }
	pub fn _0x7678FE0455ED1145(p0: Any, p1: Any, p2: Any) -> Any { invoke!(0x7678FE0455ED1145, p0, p1, p2) }
	pub fn _0xFFE9C53DEEA3DB0B(p0: Any, p1: Any, x: f32, y: f32, z: f32, isSrlLoaded: bool, p6: Any) -> Any { invoke!(0xFFE9C53DEEA3DB0B, p0, p1, x, y, z, isSrlLoaded, p6) }
	pub fn _0x5E3CCF03995388B5(p0: Any, p1: Any, p2: Any, p3: Any) { invoke_ignore!(0x5E3CCF03995388B5, p0, p1, p2, p3) }
	pub fn _0x43037ABFE214A851() { invoke_ignore!(0x43037ABFE214A851) }
	pub fn SET_AMBIENT_ZONE_STATE(zoneName: & CStr, isEnabled: bool, p2: bool) { invoke_ignore!(0xBDA07E5950085E46, zoneName, isEnabled, p2) }
	pub fn CLEAR_AMBIENT_ZONE_STATE(zoneName: & CStr, p1: bool) { invoke_ignore!(0x218DD44AAAC964FF, zoneName, p1) }
	pub fn SET_AMBIENT_ZONE_LIST_STATE(ambientZone: & CStr, p1: bool, p2: bool) { invoke_ignore!(0x9748FA4DE50CCE3E, ambientZone, p1, p2) }
	pub fn CLEAR_AMBIENT_ZONE_LIST_STATE(ambientZone: & CStr, p1: bool) { invoke_ignore!(0x120C48C614909FA4, ambientZone, p1) }
	pub fn SET_AMBIENT_ZONE_STATE_PERSISTENT(ambientZone: & CStr, p1: bool, p2: bool) { invoke_ignore!(0x1D6650420CEC9D3B, ambientZone, p1, p2) }
	pub fn SET_AMBIENT_ZONE_LIST_STATE_PERSISTENT(ambientZone: & CStr, p1: bool, p2: bool) { invoke_ignore!(0xF3638DAE8C4045E1, ambientZone, p1, p2) }
	pub fn _SET_AMBIENT_ZONE_POSITION(ambientZone: & CStr, x: f32, y: f32, z: f32, heading: f32) { invoke_ignore!(0x3743CE6948194349, ambientZone, x, y, z, heading) }
	pub fn IS_HORN_ACTIVE(vehicle: Vehicle) -> bool { invoke!(0x9D6BFC12B05C6121, vehicle) }
	pub fn _0xFD461D0ABA5559B1(p0: Any, p1: Any) { invoke_ignore!(0xFD461D0ABA5559B1, p0, p1) }
	pub fn IS_STREAM_PLAYING(streamId: i32) -> bool { invoke!(0xD11FA52EB849D978, streamId) }
	pub fn LOAD_STREAM(streamName: & CStr, soundSet: & CStr) -> bool { invoke!(0x1F1F957154EC51DF, streamName, soundSet) }
	pub fn PLAY_STREAM_FROM_PED(ped: Ped, streamId: i32) { invoke_ignore!(0x89049DD63C08B5D1, ped, streamId) }
	pub fn PLAY_STREAM_FRONTEND(streamId: i32) { invoke_ignore!(0x58FCE43488F9F5F4, streamId) }
	pub fn PLAY_STREAM_FROM_POSITION(x: f32, y: f32, z: f32, streamId: i32) { invoke_ignore!(0x21442F412E8DE56B, x, y, z, streamId) }
	pub fn _0x3A3BE6B920525237(p0: Any, p1: Any) { invoke_ignore!(0x3A3BE6B920525237, p0, p1) }
	pub fn STOP_STREAM(streamId: i32) { invoke_ignore!(0xA4718A1419D18151, streamId) }
	pub fn STOP_PED_SPEAKING(ped: Ped, shaking: bool) { invoke_ignore!(0x9D64D7405520E3D3, ped, shaking) }
	pub fn DISABLE_PED_PAIN_AUDIO(ped: Ped, toggle: bool) { invoke_ignore!(0xA9A41C1E940FB0E8, ped, toggle) }
	pub fn IS_AMBIENT_SPEECH_DISABLED(ped: Ped) -> bool { invoke!(0x932C2D096A2C3FFF, ped) }
	pub fn SET_IS_SCRIPTED_SPEECH_DISABLED(ped: Ped, disabled: bool) -> Any { invoke!(0xB2DE3AEBE31150E2, ped, disabled) }
	pub fn _BLOCK_SPEECH_CONTEXT(context: & CStr, block: bool) { invoke_ignore!(0x6378A235374B852F, context, block) }
	pub fn _UNLOAD_SPEECH_CONTEXT(speechContext: & CStr) { invoke_ignore!(0x87E6302FC61208CC, speechContext) }
	pub fn SET_HORN_ENABLED(vehicle: Vehicle, toggle: bool) { invoke_ignore!(0x76D683C108594D0E, vehicle, toggle) }
	pub fn SET_AUDIO_VEHICLE_PRIORITY(vehicle: Vehicle, p1: Any) { invoke_ignore!(0xE5564483E407F914, vehicle, p1) }
	pub fn _0x259ACC5B52A2B2D9(p0: Any, p1: Any) { invoke_ignore!(0x259ACC5B52A2B2D9, p0, p1) }
	pub fn FORCE_USE_AUDIO_GAME_OBJECT(vehicle: Vehicle, audioName: & CStr) { invoke_ignore!(0x4F0C413926060B38, vehicle, audioName) }
	pub fn SET_GPS_ACTIVE(active: bool) { invoke_ignore!(0x3BD3F52BA9B1E4E8, active) }
	pub fn _START_AUDIO_SCENESET(audioName: & CStr, sceneset: & CStr) -> bool { invoke!(0x6339C1EA3979B5F7, audioName, sceneset) }
	pub fn _STOP_AUDIO_SCENESET(sceneset: & CStr) { invoke_ignore!(0x9428447DED71FC7E, sceneset) }
	pub fn _SET_AUDIO_SCENESET(audioName: & CStr, sceneset: & CStr) -> bool { invoke!(0xAC84686C06184B0D, audioName, sceneset) }
	pub fn START_AUDIO_SCENE(scene: & CStr) -> bool { invoke!(0x013A80FC08F6E4F2, scene) }
	pub fn _0xDC2F83A0612CA34D(p0: Any) -> Any { invoke!(0xDC2F83A0612CA34D, p0) }
	pub fn STOP_AUDIO_SCENE(scene: & CStr) { invoke_ignore!(0xDFE8422B3B94E688, scene) }
	pub fn _0x6AB944DF68B512D3(p0: Any) { invoke_ignore!(0x6AB944DF68B512D3, p0) }
	pub fn STOP_AUDIO_SCENES() { invoke_ignore!(0xBAC7FC81A75EC1A1) }
	pub fn IS_AUDIO_SCENE_ACTIVE(scene: & CStr) -> bool { invoke!(0xB65B60556E2A9225, scene) }
	pub fn SET_AUDIO_SCENE_VARIABLE(scene: & CStr, variable: & CStr, value: f32) { invoke_ignore!(0xEF21A9EF089A2668, scene, variable, value) }
	pub fn _GET_ENTITY_AUDIO_MIX_GROUP(entity: Entity) -> Hash { invoke!(0x8B25A18E390F75BF, entity) }
	pub fn ADD_ENTITY_TO_AUDIO_MIX_GROUP(entity: Entity, groupName: & CStr, p2: f32) { invoke_ignore!(0x153973AB99FE8980, entity, groupName, p2) }
	pub fn _0x131EC9247E7A2903(p0: Any) -> Any { invoke!(0x131EC9247E7A2903, p0) }
	pub fn REMOVE_ENTITY_FROM_AUDIO_MIX_GROUP(entity: Entity, p1: f32) { invoke_ignore!(0x18EB48CFC41F2EA0, entity, p1) }
	pub fn _0xE600F61F54A444A6() -> Any { invoke!(0xE600F61F54A444A6) }
	pub fn AUDIO_IS_MUSIC_PLAYING() -> bool { invoke!(0x845FFC3A4FEEFA3E) }
	pub fn _0xBE28DB99556FF8D9(entity: Entity) -> Hash { invoke!(0xBE28DB99556FF8D9, entity) }
	pub fn _0x8E901B65206C2D3E(ped: Ped) { invoke_ignore!(0x8E901B65206C2D3E, ped) }
	pub fn _0xC4CFCE4C656EF480(ped: Ped) { invoke_ignore!(0xC4CFCE4C656EF480, ped) }
	pub fn _0xABDB4863D3D72021(entity: Entity, p1: Any, p2: Any, p3: f32, p4: Any) { invoke_ignore!(0xABDB4863D3D72021, entity, p1, p2, p3, p4) }
	pub fn _0xB93A769B8B726950(ped: Ped, p1: Hash) { invoke_ignore!(0xB93A769B8B726950, ped, p1) }
	pub fn _0xE891504B2F0E2DBA(p0: Any, p1: Any) { invoke_ignore!(0xE891504B2F0E2DBA, p0, p1) }
	pub fn _0x9EB779765E68C52E(p0: Any, p1: Any) { invoke_ignore!(0x9EB779765E68C52E, p0, p1) }
	pub fn _0xE9694B2D6CB87B06(entity: Entity, p1: Any) { invoke_ignore!(0xE9694B2D6CB87B06, entity, p1) }
	pub fn _0x886657C5B3D8EDE3(entity: Entity) -> Any { invoke!(0x886657C5B3D8EDE3, entity) }
	pub fn _0xC68C02DE259C927C(p0: Any) -> Any { invoke!(0xC68C02DE259C927C, p0) }
	pub fn _0x2FFF4A78384AFFDF(entity: Entity) -> Any { invoke!(0x2FFF4A78384AFFDF, entity) }
	pub fn _0x62377977E4F08668(entity: Entity) -> AnimScene { invoke!(0x62377977E4F08668, entity) }
	pub fn _GET_PED_SONG_INDEX_HOST(ped: Ped) -> Any { invoke!(0x2DBBF0C5E19383EE, ped) }
	pub fn _0xD05A460328560477(p0: Any) -> Any { invoke!(0xD05A460328560477, p0) }
	pub fn _0x8D29FDF565DED9AE(p0: Any, p1: Any, p2: Any) { invoke_ignore!(0x8D29FDF565DED9AE, p0, p1, p2) }
	pub fn _0x448F2647DD6F2E27(p0: Any, p1: Any, p2: Any, p3: Any, p4: Any) { invoke_ignore!(0x448F2647DD6F2E27, p0, p1, p2, p3, p4) }
	pub fn _0x139A4B9DF2D26CBF(p0: Any, p1: Any) { invoke_ignore!(0x139A4B9DF2D26CBF, p0, p1) }
	pub fn _0x018ABE833CA64D2A(p0: Any, p1: Any) { invoke_ignore!(0x018ABE833CA64D2A, p0, p1) }
	pub fn _0xBC07CA8FD710E7FD(p0: Any, p1: Any) { invoke_ignore!(0xBC07CA8FD710E7FD, p0, p1) }
	pub fn PREPARE_MUSIC_EVENT(eventName: & CStr) -> bool { invoke!(0x1E5185B72EF5158A, eventName) }
	pub fn CANCEL_MUSIC_EVENT(eventName: & CStr) -> bool { invoke!(0x5B17A90291133DA5, eventName) }
	pub fn TRIGGER_MUSIC_EVENT(eventName: & CStr) -> bool { invoke!(0x706D57B0F50DA710, eventName) }
	pub fn _TRIGGER_MUSIC_EVENT_WITH_HASH(eventName: Hash) -> Any { invoke!(0x05D6195FB4D428F4, eventName) }
	pub fn GET_MUSIC_PLAYTIME() -> i32 { invoke!(0xE7A0D23DC414507B) }
	pub fn _0xF64034D533CE8AAC(p0: Any, p1: Any, p2: Any) { invoke_ignore!(0xF64034D533CE8AAC, p0, p1, p2) }
	pub fn SET_PED_WALLA_DENSITY(p0: f32, p1: f32) { invoke_ignore!(0x149AEE66F0CB3A99, p0, p1) }
	pub fn _0xDAD6CD07CAA4F382() { invoke_ignore!(0xDAD6CD07CAA4F382) }
	pub fn SET_PED_INTERIOR_WALLA_DENSITY(p0: f32, p1: f32) { invoke_ignore!(0x8BF907833BE275DE, p0, p1) }
	pub fn FORCE_PED_PANIC_WALLA() { invoke_ignore!(0x062D5EAD4DA2FA6A) }
	pub fn _0x138ADB94F8B90616() { invoke_ignore!(0x138ADB94F8B90616) }
	pub fn USE_FOOTSTEP_SCRIPT_SWEETENERS(ped: Ped, p1: bool, hash: Hash) { invoke_ignore!(0xBF4DC1784BE94DFA, ped, p1, hash) }
	pub fn SET_AUDIO_FLAG(flagName: & CStr, toggle: bool) { invoke_ignore!(0xB9EFD5C25018725A, flagName, toggle) }
	pub fn _0x6DA15746D5CC1A92(p0: Any, p1: Any, p2: Any, p3: Any, p4: Any, p5: Any) { invoke_ignore!(0x6DA15746D5CC1A92, p0, p1, p2, p3, p4, p5) }
	pub fn _0x078F77FD1A43EAB3(p0: Any, p1: Any) { invoke_ignore!(0x078F77FD1A43EAB3, p0, p1) }
	pub fn _0x3D0BBCCF401B5FDB() { invoke_ignore!(0x3D0BBCCF401B5FDB) }
	pub fn SET_PORTAL_SETTINGS_OVERRIDE(p0: & CStr, p1: & CStr) { invoke_ignore!(0x044DBAD7A7FA2BE5, p0, p1) }
	pub fn REMOVE_PORTAL_SETTINGS_OVERRIDE(p0: & CStr) { invoke_ignore!(0xB4BBFD9CD8B3922B, p0) }
	pub fn _0xEA546C31FD45F8CD(p0: Any) { invoke_ignore!(0xEA546C31FD45F8CD, p0) }
	pub fn _0x44A5EEF54F62E823(p0: Any) -> Any { invoke!(0x44A5EEF54F62E823, p0) }
	pub fn _0x017492B2201E3428(p0: Any, p1: Any, p2: Any, p3: Any) { invoke_ignore!(0x017492B2201E3428, p0, p1, p2, p3) }
	pub fn _0xC886CD666ADD42E1(p0: Any, p1: Any) { invoke_ignore!(0xC886CD666ADD42E1, p0, p1) }
	pub fn _0x5AE0CB5F35F034FD(p0: Any, p1: Any, p2: Any, p3: Any, p4: Any, p5: Any, p6: Any, p7: Any) { invoke_ignore!(0x5AE0CB5F35F034FD, p0, p1, p2, p3, p4, p5, p6, p7) }
	pub fn _0x821C32C728B24477(p0: Any, p1: Any, p2: Any) { invoke_ignore!(0x821C32C728B24477, p0, p1, p2) }
	pub fn _0x06C5DF5EE444BC6B(p0: Any, p1: Any, p2: Any, p3: Any, p4: Any) { invoke_ignore!(0x06C5DF5EE444BC6B, p0, p1, p2, p3, p4) }
	pub fn _0x341CDD17EFC2472E(p0: Any, p1: Any) { invoke_ignore!(0x341CDD17EFC2472E, p0, p1) }
	pub fn _0x7E176C676F8652A9(p0: Any) { invoke_ignore!(0x7E176C676F8652A9, p0) }
	pub fn _0x2B9C37C01BF25EDB(p0: Any) -> Any { invoke!(0x2B9C37C01BF25EDB, p0) }
	pub fn _0xA6A3A3F96B8B030E() -> Any { invoke!(0xA6A3A3F96B8B030E) }
	pub fn _GET_LOADED_STREAM_ID_FROM_CREATION(streamName: & CStr, soundSet: & CStr) -> i32 { invoke!(0x0556C784FA056628, streamName, soundSet) }
	pub fn _0xC369E2234E34A0CA(p0: Any, p1: Any) -> Any { invoke!(0xC369E2234E34A0CA, p0, p1) }
	pub fn _0x35B8C070E0C16E2F(p0: Any, p1: Any) { invoke_ignore!(0x35B8C070E0C16E2F, p0, p1) }
	pub fn _0xE7E6CB8B713ED190() { invoke_ignore!(0xE7E6CB8B713ED190) }
	pub fn _0x569ABC36E28DDEAA() { invoke_ignore!(0x569ABC36E28DDEAA) }
	pub fn _0x839C9F124BE74D94(p0: Any, p1: Any, p2: Any, p3: Any, p4: Any) { invoke_ignore!(0x839C9F124BE74D94, p0, p1, p2, p3, p4) }
	pub fn AUDIO_TRIGGER_EXPLOSION(name: & CStr, x: f32, y: f32, z: f32) { invoke_ignore!(0x374F0E716BFCDE82, name, x, y, z) }
	pub fn _0x3E98AC9D8C56C62C(p0: Any) { invoke_ignore!(0x3E98AC9D8C56C62C, p0) }
	pub fn _0xCBF2BEBB468A34F3(p0: Any) { invoke_ignore!(0xCBF2BEBB468A34F3, p0) }
	pub fn _0xA2B851605748AD0E() { invoke_ignore!(0xA2B851605748AD0E) }
	pub fn _0xCFAD2C8CD1054523(p0: Any, p1: Any, p2: Any, p3: Any) { invoke_ignore!(0xCFAD2C8CD1054523, p0, p1, p2, p3) }
	pub fn _0xD733528B6C35647A(p0: Any, p1: Any) { invoke_ignore!(0xD733528B6C35647A, p0, p1) }
	pub fn _0x5BC885EBD75FAA7D(p0: Any, p1: Any) { invoke_ignore!(0x5BC885EBD75FAA7D, p0, p1) }
	pub fn _0x6B7A88A61B41E589(p0: Any) { invoke_ignore!(0x6B7A88A61B41E589, p0) }
	pub fn _0x4BE3EC91C01F0FE8() { invoke_ignore!(0x4BE3EC91C01F0FE8) }
	pub fn SET_AUDIO_ONLINE_TRANSITION_STAGE(p0: & CStr) { invoke_ignore!(0x9B1FC259187C97C0, p0) }
	pub fn _STOP_ALL_SCRIPTED_AUDIO_SOUNDS() { invoke_ignore!(0x2E399EAFBEEA74D5) }
	pub fn _0x64B956F4E761DF5C(p0: Any) { invoke_ignore!(0x64B956F4E761DF5C, p0) }
}
pub mod BOUNTY {
	use std::ffi::CStr;
	use crate::core::scripthook::native::*;
	use crate::core::types::*;

	pub fn _BOUNTY_REQUEST_PAY_OFF_BOUNTY(outRpcGuid: &mut Any) -> bool { invoke!(0x537CE992BD2D7BCB, outRpcGuid) }
	pub fn _BOUNTY_REQUEST_PAY_OFF_BOUNTY_EX(outRpcGuid: &mut Any, p1: Hash, costType: Hash) -> bool { invoke!(0x587BCEC31D64F382, outRpcGuid, p1, costType) }
	pub fn _BOUNTY_REQUEST_SELF_REPORT_CRIME(outRpcGuid: &mut Any, crimeType: Hash, p2: bool) -> bool { invoke!(0x188B748861B5BA17, outRpcGuid, crimeType, p2) }
	pub fn _BOUNTY_REQUEST_SELF_REPORT_KILLED_BY_BOUNTY_HUNTER(outRpcGuid: &mut Any) -> bool { invoke!(0xB462D69D406A2602, outRpcGuid) }
	pub fn BOUNTY_GET_BOUNTY_ON_PLAYER(gamerHandle: &mut Any, bountyData: &mut Any) -> bool { invoke!(0x4EF23E04A0C8FF51, gamerHandle, bountyData) }
	pub fn _BOUNTY_IS_REQUEST_PENDING(rpcGuid: &mut Any) -> bool { invoke!(0x03B61CD51097DE60, rpcGuid) }
	pub fn _BOUNTY_REQUEST_BEGIN_WANTED_POSTER(outRpcGuid: &mut Any, p1: i32) -> bool { invoke!(0xFFA13742E43507E3, outRpcGuid, p1) }
	pub fn _0x81847C2134039BDC(p0: &mut Any) -> bool { invoke!(0x81847C2134039BDC, p0) }
	pub fn _BOUNTY_REQUEST_COMPLETE_WANTED_POSTER(outRpcGuid: &mut Any, p1: &mut Any) -> bool { invoke!(0x727AB6F008BB9F29, outRpcGuid, p1) }
	pub fn _BOUNTY_REQUEST_COMPLETE_SPLIT_WANTED_POSTER(outRpcGuid: &mut Any, p1: &mut Any) -> bool { invoke!(0xFBD137BF0EC50FC9, outRpcGuid, p1) }
	pub fn _BOUNTY_CANCEL_WANTED_POSTER() { invoke_ignore!(0x6A9DF0FCD0C87FF9) }
	pub fn BOUNTY_REQUEST_BEGIN_LEGENDARY_MISSION(outRpcGuid: &mut Any, p1: i32, p2: i32) -> bool { invoke!(0xFC81D7C7A151CFAA, outRpcGuid, p1, p2) }
	pub fn BOUNTY_REQUEST_BEGIN_LEGENDARY_MISSION_FOR_POSSE(outRpcGuid: &mut Any, p1: i32, p2: i32) -> bool { invoke!(0x48E4E23F1739E197, outRpcGuid, p1, p2) }
	pub fn _BOUNTY_REQUEST_COMPLETE_LEGENDARY_MISSION(outRpcGuid: &mut Any, p1: &mut Any) -> bool { invoke!(0xA7309AC0DCF6D950, outRpcGuid, p1) }
	pub fn _BOUNTY_CANCEL_LEGENDARY_MISSION() { invoke_ignore!(0x2BA1BCC99826CDA2) }
	pub fn BOUNTY_GET_WANTED_POSTER_SLOT(p0: Hash, p1: Hash, p2: &mut Any) -> bool { invoke!(0xB395A44A0C7CA615, p0, p1, p2) }
	pub fn BOUNTY_GET_LEGENDARY_TARGET(p0: Any, p1: &mut Any) -> bool { invoke!(0x85E4D7B225A30ED1, p0, p1) }
	pub fn _0x86EC5F83867C4B70(p0: &mut Any) -> bool { invoke!(0x86EC5F83867C4B70, p0) }
	pub fn BOUNTY_GET_COOLDOWN_COLLECTION(p0: &mut Any) -> bool { invoke!(0x8FAF4D262FABA99C, p0) }
	pub fn _BOUNTY_REQUEST_BECOME_TARGET_OF_CHARACTER_BOUNTY_HUNT(outRpcGuid: &mut Any) -> bool { invoke!(0xB096547D61868254, outRpcGuid) }
	pub fn _0x27D3A0E1FE090A43(p0: &mut Any) -> bool { invoke!(0x27D3A0E1FE090A43, p0) }
	pub fn _BOUNTY_REQUEST_CLAIM_CHARACTER_BOUNTY(outRpcGuid: &mut Any, p1: i32, p2: &mut Any) -> bool { invoke!(0xA9C3B0F746375162, outRpcGuid, p1, p2) }
	pub fn _BOUNTY_REQUEST_POSSE_LEADER_CLAIM_CHARACTER_BOUNTY(outRpcGuid: &mut Any, p1: i32, p2: &mut Any) -> bool { invoke!(0x5B53CA0E2AC3FF45, outRpcGuid, p1, p2) }
	pub fn _BOUNTY_REQUEST_POSSE_MEMBER_CLAIM_CHARACTER_BOUNTY_SHARE(outRpcGuid: &mut Any, p1: &mut Any) -> bool { invoke!(0x22D3A61CE053270C, outRpcGuid, p1) }
	pub fn _BOUNTY_CLEAR_BEING_BOUNTY_HUNTER() { invoke_ignore!(0xA59D1997ECD99F0A) }
	pub fn _BOUNTY_REQUEST_ESCAPED_CHARACTER_BOUNTY_HUNT(outRpcGuid: &mut Any) -> bool { invoke!(0x12E981D53B07BF48, outRpcGuid) }
	pub fn _BOUNTY_REQUEST_POSSE_LEADER_ESCAPED_CHARACTER_BOUNTY_HUNT(outRpcGuid: &mut Any) -> bool { invoke!(0x2D874BA20E8E1F20, outRpcGuid) }
	pub fn _BOUNTY_REQUEST_POSSE_MEMBER_ESCAPED_CHARACTER_BOUNTY_HUNT(outRpcGuid: &mut Any) -> bool { invoke!(0x8521C2E235558278, outRpcGuid) }
	pub fn _BOUNTY_CLEAR_BEING_TARGET() { invoke_ignore!(0x932DB3C05A7465D1) }
	pub fn _BOUNTY_REQUEST_SERVED_FULL_JAIL_SENTENCE(outRpcGuid: &mut Any) -> bool { invoke!(0x3F73AED12A5EF0FF, outRpcGuid) }
	pub fn _BOUNTY_REQUEST_BRIBE_JAIL_GUARD(outRpcGuid: &mut Any, p1: i32) -> bool { invoke!(0x28717806D3BDD0D0, outRpcGuid, p1) }
	pub fn _0xF8BCC5ECA33AC9C1() -> i32 { invoke!(0xF8BCC5ECA33AC9C1) }
	pub fn _0xD6A67E2FF373D0E3(p0: i32) -> i32 { invoke!(0xD6A67E2FF373D0E3, p0) }
}
pub mod BRAIN {
	use std::ffi::CStr;
	use crate::core::scripthook::native::*;
	use crate::core::types::*;

	pub fn REGISTER_OBJECT_SCRIPT_BRAIN(scriptName: & CStr, modelHash: Hash, p2: i32, activationRange: f32, p4: i32, p5: i32) { invoke_ignore!(0x16AF9B4EEAC3B305, scriptName, modelHash, p2, activationRange, p4, p5) }
	pub fn _START_PRELOADED_SCRIPT_BRAIN(entity: Entity, scriptName: & CStr, scriptStackSize: i32, p3: bool) -> i32 { invoke!(0x4E4507CC5E4DB869, entity, scriptName, scriptStackSize, p3) }
	pub fn _START_SCRIPT_BRAIN(entity: Entity, scriptName: & CStr, p2: i32, p3: &mut Any, p4: i32, p5: bool) -> i32 { invoke!(0x6F62FAE266DCFC81, entity, scriptName, p2, p3, p4, p5) }
	pub fn _REMOVE_SCRIPT_BRAIN_ENTITY(entity: Entity) { invoke_ignore!(0x38F1E09224EECA09, entity) }
	pub fn _GET_SCRIPT_BRAIN_ENTITY() -> Entity { invoke!(0x6818D1A194E29983) }
	pub fn _0xA6AC35DB4A7957A8(flag: i32) { invoke_ignore!(0xA6AC35DB4A7957A8, flag) }
	pub fn _0x4AA5EA1EDFB25786(flag: i32) { invoke_ignore!(0x4AA5EA1EDFB25786, flag) }
	pub fn ENABLE_SCRIPT_BRAIN_SET(brainSet: i32) { invoke_ignore!(0x1CF6E5C6750EADBD, brainSet) }
	pub fn DISABLE_SCRIPT_BRAIN_SET(brainSet: i32) { invoke_ignore!(0x3F44EA613A5B2676, brainSet) }
	pub fn REACTIVATE_ALL_OBJECT_BRAINS_THAT_ARE_WAITING_TILL_OUT_OF_RANGE() { invoke_ignore!(0xA32B0B05EFF75730) }
	pub fn REACTIVATE_NAMED_OBJECT_BRAINS_WAITING_TILL_OUT_OF_RANGE(scriptName: & CStr) { invoke_ignore!(0x74C333E34DF74E8A, scriptName) }
}
pub mod CAM {
	use std::ffi::CStr;
	use crate::core::scripthook::native::*;
	use crate::core::types::*;

	pub fn RENDER_SCRIPT_CAMS(render: bool, ease: bool, easeTime: i32, p3: bool, p4: bool, p5: i32) { invoke_ignore!(0x33281167E4942E4F, render, ease, easeTime, p3, p4, p5) }
	pub fn STOP_RENDERING_SCRIPT_CAMS_USING_CATCH_UP(render: bool, distance: f32, blendBackSmoothingType: i32, p3: bool, p4: bool, p5: bool) { invoke_ignore!(0x8C7C7FF7CF0E5153, render, distance, blendBackSmoothingType, p3, p4, p5) }
	pub fn CREATE_CAM(camName: & CStr, p1: bool) -> Cam { invoke!(0xE72CDBA7F0A02DD6, camName, p1) }
	pub fn CREATE_CAM_WITH_PARAMS(camName: & CStr, posX: f32, posY: f32, posZ: f32, rotX: f32, rotY: f32, rotZ: f32, fov: f32, p8: bool, p9: i32) -> Cam { invoke!(0x40C23491CE83708E, camName, posX, posY, posZ, rotX, rotY, rotZ, fov, p8, p9) }
	pub fn CREATE_CAMERA(camHash: Hash, p1: bool) -> Cam { invoke!(0x57CDF879EA466C46, camHash, p1) }
	pub fn CREATE_CAMERA_WITH_PARAMS(camHash: Hash, posX: f32, posY: f32, posZ: f32, rotX: f32, rotY: f32, rotZ: f32, fov: f32, p8: bool, p9: Any) -> Cam { invoke!(0x98B99B9F27E2D60B, camHash, posX, posY, posZ, rotX, rotY, rotZ, fov, p8, p9) }
	pub fn DESTROY_CAM(cam: Cam, p1: bool) { invoke_ignore!(0x4E67E0B6D7FD5145, cam, p1) }
	pub fn DESTROY_ALL_CAMS(p0: bool) { invoke_ignore!(0x163600D6E136C9F8, p0) }
	pub fn DOES_CAM_EXIST(cam: Cam) -> bool { invoke!(0x153AD457764FD704, cam) }
	pub fn SET_CAM_ACTIVE(cam: Cam, active: bool) { invoke_ignore!(0x87295BCA613800C8, cam, active) }
	pub fn IS_CAM_ACTIVE(cam: Cam) -> bool { invoke!(0x63EFCC7E1810B8E6, cam) }
	pub fn IS_CAM_RENDERING(cam: Cam) -> bool { invoke!(0x4415F8A6C536D39F, cam) }
	pub fn GET_RENDERING_CAM() -> Cam { invoke!(0x03A8931ECC8015D6) }
	pub fn GET_CAM_COORD(cam: Cam) -> Vector3 { invoke!(0x6B12F11C2A9F0344, cam) }
	pub fn GET_CAM_ROT(cam: Cam, rotationOrder: i32) -> Vector3 { invoke!(0x9BF96B57254E7889, cam, rotationOrder) }
	pub fn GET_CAM_FOV(cam: Cam) -> f32 { invoke!(0x8101D32A0A6B0F60, cam) }
	pub fn SET_CAM_PARAMS(cam: Cam, posX: f32, posY: f32, posZ: f32, rotX: f32, rotY: f32, rotZ: f32, fieldOfView: f32, p8: Any, p9: i32, p10: i32, p11: i32, p12: Any, p13: Any) { invoke_ignore!(0xA47BBFFFB83D4D0A, cam, posX, posY, posZ, rotX, rotY, rotZ, fieldOfView, p8, p9, p10, p11, p12, p13) }
	pub fn SET_CAM_COORD(cam: Cam, posX: f32, posY: f32, posZ: f32) { invoke_ignore!(0xF9EE7D419EE49DE6, cam, posX, posY, posZ) }
	pub fn SET_CAM_ROT(cam: Cam, rotX: f32, rotY: f32, rotZ: f32, rotationOrder: i32) { invoke_ignore!(0x63DFA6810AD78719, cam, rotX, rotY, rotZ, rotationOrder) }
	pub fn SET_CAM_FOV(cam: Cam, fieldOfView: f32) { invoke_ignore!(0x27666E5988D9D429, cam, fieldOfView) }
	pub fn SET_CAM_NEAR_CLIP(cam: Cam, nearClip: f32) { invoke_ignore!(0xA924028272A61364, cam, nearClip) }
	pub fn SET_CAM_FAR_CLIP(cam: Cam, farClip: f32) { invoke_ignore!(0x5E32817BF6302111, cam, farClip) }
	pub fn SET_CAM_MOTION_BLUR_STRENGTH(cam: Cam, strength: f32) { invoke_ignore!(0x45FD891364181F9E, cam, strength) }
	pub fn _0xFC3F638BE2B6BB02() { invoke_ignore!(0xFC3F638BE2B6BB02) }
	pub fn _0xE4B7945EF4F1BFB2(cam: Cam, args: &mut Any) { invoke_ignore!(0xE4B7945EF4F1BFB2, cam, args) }
	pub fn _0x1FC6C727D30FFDDE(p0: Any) { invoke_ignore!(0x1FC6C727D30FFDDE, p0) }
	pub fn ATTACH_CAM_TO_ENTITY(cam: Cam, entity: Entity, xOffset: f32, yOffset: f32, zOffset: f32, isRelative: bool) { invoke_ignore!(0xFDC0DF7F6FB0A592, cam, entity, xOffset, yOffset, zOffset, isRelative) }
	pub fn ATTACH_CAM_TO_PED_BONE(cam: Cam, ped: Ped, boneIndex: i32, x: f32, y: f32, z: f32, heading: bool) { invoke_ignore!(0xDFC1E4A44C0324CA, cam, ped, boneIndex, x, y, z, heading) }
	pub fn DETACH_CAM(cam: Cam) { invoke_ignore!(0x05B41DDBEB559556, cam) }
	pub fn POINT_CAM_AT_COORD(cam: Cam, x: f32, y: f32, z: f32) { invoke_ignore!(0x948B39341C3A40C2, cam, x, y, z) }
	pub fn POINT_CAM_AT_ENTITY(cam: Cam, entity: Entity, p2: f32, p3: f32, p4: f32, p5: bool) { invoke_ignore!(0xFC2867E6074D3A61, cam, entity, p2, p3, p4, p5) }
	pub fn STOP_CAM_POINTING(cam: Cam) { invoke_ignore!(0xCA1B30A3357C71F1, cam) }
	pub fn _SET_CAM_FOCUS_DISTANCE(cam: Cam, distance: f32) { invoke_ignore!(0x11F32BB61B756732, cam, distance) }
	pub fn _PAUSE_CAMERA_FOCUS(cam: Cam, pause: bool) { invoke_ignore!(0x9F97E85EC142255E, cam, pause) }
	pub fn SET_CAM_AFFECTS_AIMING(cam: Cam, toggle: bool) { invoke_ignore!(0x3CB9E8BDE5E76F33, cam, toggle) }
	pub fn SET_CAM_CONTROLS_MINI_MAP_HEADING(cam: Cam, p1: bool) { invoke_ignore!(0x1B8F3CE5A6001298, cam, p1) }
	pub fn ALLOW_MOTION_BLUR_DECAY(cam: Cam, p1: bool) { invoke_ignore!(0x42ED56B02E05D109, cam, p1) }
	pub fn ADD_CAM_SPLINE_NODE(camera: Cam, x: f32, y: f32, z: f32, xRot: f32, yRot: f32, zRot: f32, length: i32, p8: i32, p9: i32) { invoke_ignore!(0xF1F57F9D230F9CD1, camera, x, y, z, xRot, yRot, zRot, length, p8, p9) }
	pub fn SET_CAM_SPLINE_PHASE(cam: Cam, p1: f32) { invoke_ignore!(0xF1898A68E7C15636, cam, p1) }
	pub fn GET_CAM_SPLINE_PHASE(cam: Cam) -> f32 { invoke!(0x095EDCD24D90033A, cam) }
	pub fn SET_CAM_SPLINE_DURATION(cam: Cam, timeDuration: i32) { invoke_ignore!(0xFF6311652CA91015, cam, timeDuration) }
	pub fn SET_CAM_SPLINE_SMOOTHING_STYLE(cam: Cam, smoothingStyle: i32) { invoke_ignore!(0x84B3645618E726B0, cam, smoothingStyle) }
	pub fn SET_CAM_ACTIVE_WITH_INTERP(camTo: Cam, camFrom: Cam, duration: i32, easeLocation: i32, easeRotation: i32) { invoke_ignore!(0x8B15AE2987C1AC8F, camTo, camFrom, duration, easeLocation, easeRotation) }
	pub fn IS_CAM_INTERPOLATING(cam: Cam) -> bool { invoke!(0x578F8F1CAA17BD2B, cam) }
	pub fn SHAKE_CAM(cam: Cam, type_: & CStr, amplitude: f32) { invoke_ignore!(0xF9A7BCF5D050D4E7, cam, type_, amplitude) }
	pub fn IS_CAM_SHAKING(cam: Cam) -> bool { invoke!(0x2EEB402BD7320159, cam) }
	pub fn STOP_CAM_SHAKING(cam: Cam, p1: bool) { invoke_ignore!(0xB78CC4B4706614B0, cam, p1) }
	pub fn PLAY_CAM_ANIM(cam: Cam, animName: & CStr, animDictionary: & CStr, x: f32, y: f32, z: f32, xRot: f32, yRot: f32, zRot: f32, animFlags: i32, rotOrder: i32) -> bool { invoke!(0xA263DDF694D563F6, cam, animName, animDictionary, x, y, z, xRot, yRot, zRot, animFlags, rotOrder) }
	pub fn _0xCF69EA05CD9C33C9() { invoke_ignore!(0xCF69EA05CD9C33C9) }
	pub fn _IS_ANIM_SCENE_CAM_ACTIVE() -> bool { invoke!(0x20389408F0E93B9A) }
	pub fn IS_SCREEN_FADED_OUT() -> bool { invoke!(0xF5472C80DF2FF847) }
	pub fn IS_SCREEN_FADED_IN() -> bool { invoke!(0x37F9A426FBCF4AF2) }
	pub fn IS_SCREEN_FADING_OUT() -> bool { invoke!(0x02F39BEFE7B88D00) }
	pub fn IS_SCREEN_FADING_IN() -> bool { invoke!(0x0CECCC63FFA2EF24) }
	pub fn DO_SCREEN_FADE_IN(duration: i32) { invoke_ignore!(0x6A053CF596F67DF7, duration) }
	pub fn DO_SCREEN_FADE_OUT(duration: i32) { invoke_ignore!(0x40C719A5E410B9E4, duration) }
	pub fn HAS_LETTER_BOX() -> bool { invoke!(0x81DCFD13CF39920E) }
	pub fn _REQUEST_LETTER_BOX_NOW(p0: bool, p1: bool) { invoke_ignore!(0x69D65E89FFD72313, p0, p1) }
	pub fn _REQUEST_LETTER_BOX_OVERTIME(p0: i32, p1: i32, p2: bool, p3: i32, p4: bool, p5: bool) { invoke_ignore!(0xE296208C273BD7F0, p0, p1, p2, p3, p4, p5) }
	pub fn _FORCE_LETTER_BOX_THIS_UPDATE() { invoke_ignore!(0xC64ABC0676AF262B) }
	pub fn GET_LETTER_BOX_RATIO() -> f32 { invoke!(0xA2B1C7EF759A63CE) }
	pub fn _0x73FF6BE63DC18819() -> Any { invoke!(0x73FF6BE63DC18819) }
	pub fn _0x450769C833D58844() -> Any { invoke!(0x450769C833D58844) }
	pub fn SET_WIDESCREEN_BORDERS(p0: bool, p1: i32) { invoke_ignore!(0xD7F4D54CF80AFA34, p0, p1) }
	pub fn GET_GAMEPLAY_CAM_COORD() -> Vector3 { invoke!(0x595320200B98596E) }
	pub fn GET_GAMEPLAY_CAM_ROT(rotationOrder: i32) -> Vector3 { invoke!(0x0252D2B5582957A6, rotationOrder) }
	pub fn GET_GAMEPLAY_CAM_FOV() -> f32 { invoke!(0xF6A96E5ACEEC6E50) }
	pub fn SET_GAMEPLAY_CAM_MAX_MOTION_BLUR_STRENGTH_THIS_UPDATE(p0: f32) { invoke_ignore!(0x8459B3E64257B21D, p0) }
	pub fn GET_GAMEPLAY_CAM_RELATIVE_HEADING() -> f32 { invoke!(0xC4ABF536048998AA) }
	pub fn SET_GAMEPLAY_CAM_RELATIVE_HEADING(heading: f32, p1: f32) { invoke_ignore!(0x5D1EB123EAC5D071, heading, p1) }
	pub fn GET_GAMEPLAY_CAM_RELATIVE_PITCH() -> f32 { invoke!(0x99AADEBBA803F827) }
	pub fn SET_GAMEPLAY_CAM_RELATIVE_PITCH(x: f32, Value2: f32) { invoke_ignore!(0xFB760AF4F537B8BF, x, Value2) }
	pub fn _0x0961B089947BA6D0(p0: Any) { invoke_ignore!(0x0961B089947BA6D0, p0) }
	pub fn _0x04084490CC302CFB() { invoke_ignore!(0x04084490CC302CFB) }
	pub fn SET_SCRIPTED_CAMERA_IS_FIRST_PERSON_THIS_FRAME(p0: bool) { invoke_ignore!(0x1DD95A8D6B24A0C9, p0) }
	pub fn _IS_IN_FULL_FIRST_PERSON_MODE() -> bool { invoke!(0xD1BA66940E94C547) }
	pub fn SHAKE_GAMEPLAY_CAM(shakeName: & CStr, intensity: f32) { invoke_ignore!(0xD9B31B4650520529, shakeName, intensity) }
	pub fn _0xC3E9E5D4F413B773(shakeName: & CStr, intensity: f32) { invoke_ignore!(0xC3E9E5D4F413B773, shakeName, intensity) }
	pub fn IS_GAMEPLAY_CAM_SHAKING() -> bool { invoke!(0xEA4C5F4AA0A4DBEF) }
	pub fn _0x0060B31968E60E41(shakeName: & CStr) -> bool { invoke!(0x0060B31968E60E41, shakeName) }
	pub fn SET_GAMEPLAY_CAM_SHAKE_AMPLITUDE(amplitude: f32) { invoke_ignore!(0x570E35F5C4A44838, amplitude) }
	pub fn _0xFEFDDC6E8FDF8A75(shakeName: & CStr, intensity: f32) { invoke_ignore!(0xFEFDDC6E8FDF8A75, shakeName, intensity) }
	pub fn STOP_GAMEPLAY_CAM_SHAKING(p0: bool) { invoke_ignore!(0xE0DE43D290FB65F9, p0) }
	pub fn _STOP_GAMEPLAY_CAM_SHAKING_WITH_NAME(shakeName: & CStr, p1: bool) { invoke_ignore!(0x4285804FD65D8066, shakeName, p1) }
	pub fn SET_GAMEPLAY_CAM_FOLLOW_PED_THIS_UPDATE(ped: Ped) { invoke_ignore!(0x82E41D6ADE924FCA, ped) }
	pub fn IS_GAMEPLAY_CAM_RENDERING() -> bool { invoke!(0x8660EA714834E412) }
	pub fn IS_INTERPOLATING_FROM_SCRIPT_CAMS() -> bool { invoke!(0x251241CAEC707106) }
	pub fn IS_INTERPOLATING_TO_SCRIPT_CAMS() -> bool { invoke!(0x43AB9D5A7D415478) }
	pub fn IS_GAMEPLAY_CAM_LOOKING_BEHIND() -> bool { invoke!(0x8FE0D24FFD04D5A2) }
	pub fn SET_GAMEPLAY_CAM_IGNORE_ENTITY_COLLISION_THIS_UPDATE(entity: Entity) { invoke_ignore!(0xD904F75DBD7AB865, entity) }
	pub fn DISABLE_CAM_COLLISION_FOR_OBJECT(entity: Entity) { invoke_ignore!(0x7E3F546ACFE6C8D9, entity) }
	pub fn _0x39073DA4EDDBC91D(p0: Any) { invoke_ignore!(0x39073DA4EDDBC91D, p0) }
	pub fn _0x70A6658D476C6187() { invoke_ignore!(0x70A6658D476C6187) }
	pub fn _0x18C3DFAC458783BB() { invoke_ignore!(0x18C3DFAC458783BB) }
	pub fn _0xF1A6FEEDF3776EF9() { invoke_ignore!(0xF1A6FEEDF3776EF9) }
	pub fn _0xE6F364DE6C2FDEFE() { invoke_ignore!(0xE6F364DE6C2FDEFE) }
	pub fn _0x0F1FFEF5D54AE832() { invoke_ignore!(0x0F1FFEF5D54AE832) }
	pub fn _0x3C8F74E8FE751614() { invoke_ignore!(0x3C8F74E8FE751614) }
	pub fn _0x06557F6D96C86881() { invoke_ignore!(0x06557F6D96C86881) }
	pub fn IS_SPHERE_VISIBLE(x: f32, y: f32, z: f32, radius: f32) -> bool { invoke!(0x2E941B5FFA2989C6, x, y, z, radius) }
	pub fn _0x190F7DA1AC09A8EF() -> Any { invoke!(0x190F7DA1AC09A8EF) }
	pub fn _SET_GAMEPLAY_CAM_INITIAL_ZOOM(camInitialZoom: f32) { invoke_ignore!(0xBCDA0BA8762FACB9, camInitialZoom) }
	pub fn _SET_GAMEPLAY_CAM_INITIAL_HEADING(camInitialHeading: f32) { invoke_ignore!(0x6C1053C433A573CF, camInitialHeading) }
	pub fn _SET_GAMEPLAY_CAM_INITIAL_PITCH(camInitialPitch: f32) { invoke_ignore!(0x449995EA846D3FC2, camInitialPitch) }
	pub fn SET_THIRD_PERSON_CAM_RELATIVE_HEADING_LIMITS_THIS_UPDATE(minimum: f32, maximum: f32) { invoke_ignore!(0x14F3947318CA8AD2, minimum, maximum) }
	pub fn SET_THIRD_PERSON_CAM_RELATIVE_PITCH_LIMITS_THIS_UPDATE(minimum: f32, maximum: f32) { invoke_ignore!(0x326C7AA308F3DF6A, minimum, maximum) }
	pub fn SET_THIRD_PERSON_CAM_ORBIT_DISTANCE_LIMITS_THIS_UPDATE(p0: f32, distance: f32) { invoke_ignore!(0x2126C740A4AC370B, p0, distance) }
	pub fn SET_IN_VEHICLE_CAM_STATE_THIS_UPDATE(vehicle: Vehicle, p1: i32) { invoke_ignore!(0xFA1D5E8D1C3CCD67, vehicle, p1) }
	pub fn DISABLE_ON_FOOT_FIRST_PERSON_VIEW_THIS_UPDATE() { invoke_ignore!(0x9C473089A934C930) }
	pub fn DISABLE_FIRST_PERSON_FLASH_EFFECT_THIS_UPDATE() { invoke_ignore!(0x77D65669A05D1A1A) }
	pub fn _DISABLE_CINEMATIC_MODE_THIS_FRAME() { invoke_ignore!(0x8910C24B7E0046EC) }
	pub fn _IS_IN_CINEMATIC_MODE() -> bool { invoke!(0x74F1D22EFA71FAB8) }
	pub fn _0x718C6ECF5E8CBDD4() { invoke_ignore!(0x718C6ECF5E8CBDD4) }
	pub fn _FORCE_THIRD_PERSON_CAM_THIS_FRAME() { invoke_ignore!(0x8370D34BD2E60B73) }
	pub fn _FORCE_THIRD_PERSON_CAM_FAR_THIS_FRAME() { invoke_ignore!(0x1CFB749AD4317BDE) }
	pub fn _FORCE_FIRST_PERSON_CAM_THIS_FRAME() -> bool { invoke!(0x90DA5BA5C2635416) }
	pub fn _DISABLE_ON_FOOT_FIRST_PERSON_VIEW_THIS_UPDATE_2() { invoke_ignore!(0x05AB44D906738426) }
	pub fn _0x632BE8D84846FA56() { invoke_ignore!(0x632BE8D84846FA56) }
	pub fn _0x71D71E08A7ED5BD7(p0: Any) { invoke_ignore!(0x71D71E08A7ED5BD7, p0) }
	pub fn IS_FOLLOW_VEHICLE_CAM_ACTIVE() -> bool { invoke!(0xA40C2F51FB589E9A) }
	pub fn _0x7E40A01B11398FCB() { invoke_ignore!(0x7E40A01B11398FCB) }
	pub fn IS_AIM_CAM_ACTIVE() -> bool { invoke!(0x698F456FB909E077) }
	pub fn IS_FIRST_PERSON_AIM_CAM_ACTIVE() -> bool { invoke!(0xF63134C54B6EC212) }
	pub fn IS_FIRST_PERSON_CAMERA_ACTIVE(p0: Any, p1: Any, p2: Any) -> bool { invoke!(0xA24C1D341C6E0D53, p0, p1, p2) }
	pub fn _0xDC62CD70658E7A02() -> Any { invoke!(0xDC62CD70658E7A02) }
	pub fn _0x796085220ADCC847() -> Any { invoke!(0x796085220ADCC847) }
	pub fn _0xB6A80E1E3A5444F1() -> Any { invoke!(0xB6A80E1E3A5444F1) }
	pub fn _0x8B1A5FE7E41E52B2() -> Any { invoke!(0x8B1A5FE7E41E52B2) }
	pub fn GET_FIRST_PERSON_AIM_CAM_ZOOM_FACTOR() -> f32 { invoke!(0xB4132CA1B0EE1365) }
	pub fn SET_FIRST_PERSON_AIM_CAM_RELATIVE_HEADING_LIMITS_THIS_UPDATE(p0: f32, p1: f32) { invoke_ignore!(0x05BD5E4088B30A66, p0, p1) }
	pub fn SET_FIRST_PERSON_AIM_CAM_RELATIVE_PITCH_LIMITS_THIS_UPDATE(p0: f32, p1: f32) { invoke_ignore!(0x715B7F5E8BED32A2, p0, p1) }
	pub fn _0xC205B3C54C6A4E37(p0: Any) { invoke_ignore!(0xC205B3C54C6A4E37, p0) }
	pub fn GET_FINAL_RENDERED_CAM_COORD() -> Vector3 { invoke!(0x5352E025EC2B416F) }
	pub fn GET_FINAL_RENDERED_CAM_ROT(rotationOrder: i32) -> Vector3 { invoke!(0x602685BD85DD26CA, rotationOrder) }
	pub fn GET_FINAL_RENDERED_CAM_FOV() -> f32 { invoke!(0x04AF77971E508F6A) }
	pub fn SET_GAMEPLAY_COORD_HINT(x: f32, y: f32, z: f32, duration: i32, blendOutDuration: i32, blendInDuration: i32, p6: Hash) { invoke_ignore!(0xFA33B8C69A4A6A0F, x, y, z, duration, blendOutDuration, blendInDuration, p6) }
	pub fn SET_GAMEPLAY_PED_HINT(p0: Ped, x1: f32, y1: f32, z1: f32, p4: bool, p5: Any, p6: Any, p7: Any) { invoke_ignore!(0x90FB951648851733, p0, x1, y1, z1, p4, p5, p6, p7) }
	pub fn SET_GAMEPLAY_VEHICLE_HINT(p0: Any, p1: f32, p2: f32, p3: f32, p4: bool, p5: Any, p6: Any, p7: Any) { invoke_ignore!(0xE2B2BB7DAC280515, p0, p1, p2, p3, p4, p5, p6, p7) }
	pub fn SET_GAMEPLAY_OBJECT_HINT(p0: Any, p1: f32, p2: f32, p3: f32, p4: bool, p5: Any, p6: Any, p7: Any) { invoke_ignore!(0xC40551D65F2BF297, p0, p1, p2, p3, p4, p5, p6, p7) }
	pub fn SET_GAMEPLAY_ENTITY_HINT(entity: Entity, xOffset: f32, yOffset: f32, zOffset: f32, p4: bool, p5: i32, p6: i32, p7: i32, p8: Any) { invoke_ignore!(0xD1F7F32640ADFD12, entity, xOffset, yOffset, zOffset, p4, p5, p6, p7, p8) }
	pub fn IS_GAMEPLAY_HINT_ACTIVE() -> bool { invoke!(0x2E04AB5FEE042D4A) }
	pub fn STOP_GAMEPLAY_HINT(p0: bool) { invoke_ignore!(0x1BCEC33D54CFCA8A, p0) }
	pub fn STOP_CODE_GAMEPLAY_HINT(p0: bool) { invoke_ignore!(0x93759A83D0D844E7, p0) }
	pub fn _0x88544C0E3291DCAE(p0: bool) { invoke_ignore!(0x88544C0E3291DCAE, p0) }
	pub fn SET_GAMEPLAY_HINT_FOV(FOV: f32) { invoke_ignore!(0x661E58BC6F00A49A, FOV) }
	pub fn SET_GAMEPLAY_HINT_FOLLOW_DISTANCE_SCALAR(p0: f32) { invoke_ignore!(0xDDDC54181868F81F, p0) }
	pub fn SET_GAMEPLAY_HINT_BASE_ORBIT_PITCH_OFFSET(p0: f32) { invoke_ignore!(0x421192A2DA48FD01, p0) }
	pub fn SET_GAMEPLAY_HINT_CAMERA_RELATIVE_SIDE_OFFSET(p0: f32) { invoke_ignore!(0xF86B6F93727C59C9, p0) }
	pub fn SET_GAMEPLAY_HINT_CAMERA_RELATIVE_VERTICAL_OFFSET(p0: f32) { invoke_ignore!(0x29E74F819150CC32, p0) }
	pub fn _0xF48664E9C83825E3(p0: Any, p1: Any) { invoke_ignore!(0xF48664E9C83825E3, p0, p1) }
	pub fn _0x1F6EBD94680252CE(p0: Any, p1: Any) { invoke_ignore!(0x1F6EBD94680252CE, p0, p1) }
	pub fn _0xE28F73212A813E82(p0: Any, p1: Any, p2: Any, p3: Any) { invoke_ignore!(0xE28F73212A813E82, p0, p1, p2, p3) }
	pub fn _0x4D2F46D1B28D90FB(p0: Any, p1: Any) { invoke_ignore!(0x4D2F46D1B28D90FB, p0, p1) }
	pub fn _START_CAMERA_ORBIT(p0: &mut Any) { invoke_ignore!(0x65B205BF30C13DDB, p0) }
	pub fn _0x641092322A8852AB() { invoke_ignore!(0x641092322A8852AB) }
	pub fn _0xDB382FE20C2DA222(p0: Any) { invoke_ignore!(0xDB382FE20C2DA222, p0) }
	pub fn _0x2DD3149DC34A3F4C(p0: Any) { invoke_ignore!(0x2DD3149DC34A3F4C, p0) }
	pub fn _FREEZE_GAMEPLAY_CAM_THIS_FRAME() { invoke_ignore!(0x027CAB2C3AF27010) }
	pub fn _0x3C486E334520579D() { invoke_ignore!(0x3C486E334520579D) }
	pub fn _0x41E452A3C580D1A7() { invoke_ignore!(0x41E452A3C580D1A7) }
	pub fn SET_CINEMATIC_BUTTON_ACTIVE(p0: bool) { invoke_ignore!(0xB90411F480457A6C, p0) }
	pub fn IS_CINEMATIC_CAM_RENDERING() -> bool { invoke!(0xBF7C780731AADBF8) }
	pub fn DISABLE_CINEMATIC_BONNET_CAMERA_THIS_UPDATE() { invoke_ignore!(0xA5929C2E57AC90D1) }
	pub fn INVALIDATE_CINEMATIC_VEHICLE_IDLE_MODE() { invoke_ignore!(0x634F4A0562CF19B8) }
	pub fn FORCE_CINEMATIC_RENDERING_THIS_UPDATE(p0: bool) { invoke_ignore!(0x702B75DC9D3EDE56, p0) }
	pub fn _0x9AC65A36D3C0C189(p0: Any) { invoke_ignore!(0x9AC65A36D3C0C189, p0) }
	pub fn _0x975F6EBB62632FE3() -> bool { invoke!(0x975F6EBB62632FE3) }
	pub fn SET_CINEMATIC_MODE_ACTIVE(p0: bool) { invoke_ignore!(0xCE7A90B160F75046, p0) }
	pub fn _0x1811A02277A9E49D() -> bool { invoke!(0x1811A02277A9E49D) }
	pub fn _FORCE_CINEMATIC_DEATH_CAM_ON_PED(targetPed: Ped) { invoke_ignore!(0xE3639DB78B3B5400, targetPed) }
	pub fn _REACTIVATE_PED_HEADSHOT_EXECUTE_SLOWCAM(ped: Ped, p1: i32) { invoke_ignore!(0x986F7A51EE3E1F92, ped, p1) }
	pub fn _0x5B637D6F3B67716A(p0: Any) { invoke_ignore!(0x5B637D6F3B67716A, p0) }
	pub fn _0xC252C0CC969AF79A(p0: Any) { invoke_ignore!(0xC252C0CC969AF79A, p0) }
	pub fn _0xE2BB2D6A9FE2ECDE(p0: Any) { invoke_ignore!(0xE2BB2D6A9FE2ECDE, p0) }
	pub fn _SET_START_CINEMATIC_DEATH_CAM(p0: bool) { invoke_ignore!(0x6E969927CF632608, p0) }
	pub fn _0x6072B7420A83A03F() -> Any { invoke!(0x6072B7420A83A03F) }
	pub fn _0x1204EB53A5FBC63D() -> bool { invoke!(0x1204EB53A5FBC63D) }
	pub fn _0x6519238858AF5479(p0: Any) { invoke_ignore!(0x6519238858AF5479, p0) }
	pub fn _CREATE_KILL_CAM(ped: Ped) { invoke_ignore!(0x2F994CC29CAA9D22, ped) }
	pub fn _IS_CAM_PHOTOFX_RUNNING() -> bool { invoke!(0xA14D5FE82BCB1D9E) }
	pub fn _0x6DFD37E586D4F44F() -> Any { invoke!(0x6DFD37E586D4F44F) }
	pub fn _0x80D7A3E39B120BC4() -> Any { invoke!(0x80D7A3E39B120BC4) }
	pub fn _0x63E5841A9264D016(toggle: bool) { invoke_ignore!(0x63E5841A9264D016, toggle) }
	pub fn _TRIGGER_MISSION_FAILED_CAM() { invoke_ignore!(0x9A92C06ACBAF9731) }
	pub fn _0x16E9ABDD34DDD931() { invoke_ignore!(0x16E9ABDD34DDD931) }
	pub fn IS_DEATH_FAIL_CAMERA_RUNNING() -> bool { invoke!(0x139EFB0A71DD9011) }
	pub fn _0x7CE9DC58E3E4755F() -> Any { invoke!(0x7CE9DC58E3E4755F) }
	pub fn _0x3B8E3AD9677CE12B(p0: Any, p1: Any, p2: Any) { invoke_ignore!(0x3B8E3AD9677CE12B, p0, p1, p2) }
	pub fn _0xDF7F5BE9150E47E4(p0: Any) { invoke_ignore!(0xDF7F5BE9150E47E4, p0) }
	pub fn _0xB85C13E0BF1F2A1C(p0: Any) { invoke_ignore!(0xB85C13E0BF1F2A1C, p0) }
	pub fn _0x066167C63111D8CF(p0: Any, p1: Any, p2: Any, p3: Any, p4: Any) { invoke_ignore!(0x066167C63111D8CF, p0, p1, p2, p3, p4) }
	pub fn _0xA8BA2E0204D8486F() { invoke_ignore!(0xA8BA2E0204D8486F) }
	pub fn _0xC3742F1FDF0A6824() { invoke_ignore!(0xC3742F1FDF0A6824) }
	pub fn _0x0FF7125F07DEB84F(p0: Any, p1: Any) { invoke_ignore!(0x0FF7125F07DEB84F, p0, p1) }
	pub fn _0x6CAB0BA160B168D2() { invoke_ignore!(0x6CAB0BA160B168D2) }
	pub fn _LOAD_CINEMATIC_CAM_LOCATION(locationDictName: & CStr) { invoke_ignore!(0x1B3C2D961F5FC0E1, locationDictName) }
	pub fn _UNLOAD_CINEMATIC_CAMERA_LOCATION(dictionaryName: & CStr) { invoke_ignore!(0x2412216FCC7B4E3E, dictionaryName) }
	pub fn _IS_CINEMATIC_CAM_LOCATION_LOADED(sLocationDictName: & CStr) -> bool { invoke!(0xAA235E2F2C09E952, sLocationDictName) }
	pub fn _IS_CINEMATIC_CAM_LOCATION_LOADED_2(locationDictName: & CStr) -> bool { invoke!(0x595550376B7EA230, locationDictName) }
	pub fn _0x465F04F68AD38197(dictionary: & CStr, shotName: & CStr, duration: i32) -> Any { invoke!(0x465F04F68AD38197, dictionary, shotName, duration) }
	pub fn _0xEA113BF9B0C0C5D7(dictionary: & CStr, shotName: & CStr, duration: i32) -> Any { invoke!(0xEA113BF9B0C0C5D7, dictionary, shotName, duration) }
	pub fn _CINEMATIC_LOCATION_TRIGGER_SCRIPTED_SHOT_EVENT_2(dictionary: & CStr, shotName: & CStr, duration: i32) { invoke_ignore!(0xBC016635D6A73B31, dictionary, shotName, duration) }
	pub fn CINEMATIC_LOCATION_TRIGGER_SCRIPTED_SHOT_EVENT(dictionary: & CStr, shotName: & CStr, cameraName: & CStr, p3: Any) { invoke_ignore!(0x02389579A53C3276, dictionary, shotName, cameraName, p3) }
	pub fn _0xA54D643D0773EB65(dictionary: & CStr, shotName: & CStr, duration: i32) { invoke_ignore!(0xA54D643D0773EB65, dictionary, shotName, duration) }
	pub fn CINEMATIC_LOCATION_STOP_SCRIPTED_SHOT_EVENT(p0: Any, p1: Any, p2: Any) { invoke_ignore!(0x6D4D25C2137FF511, p0, p1, p2) }
	pub fn _0xC3AEBB276825A359(dictionary: & CStr, shotName: & CStr, duration: i32) -> bool { invoke!(0xC3AEBB276825A359, dictionary, shotName, duration) }
	pub fn _0x1D931B7CC0EE3956(dictionary: & CStr, shotName: & CStr, cameraName: & CStr) -> bool { invoke!(0x1D931B7CC0EE3956, dictionary, shotName, cameraName) }
	pub fn _CINEMATIC_LOCATION_SET_LOCATION_AND_ROTATION(name: & CStr, x: f32, y: f32, z: f32, rotX: f32, rotY: f32, rotZ: f32) { invoke_ignore!(0x0E94C95EC3185FA9, name, x, y, z, rotX, rotY, rotZ) }
	pub fn CINEMATIC_LOCATION_OVERRIDE_TARGET_ENTITY_THIS_UPDATE(name: & CStr, entity: Entity) { invoke_ignore!(0x0B0F914459731F60, name, entity) }
	pub fn _LOAD_CAMERA_DATA_DICT(cameraDictionary: & CStr) { invoke_ignore!(0x6A4D224FC7643941, cameraDictionary) }
	pub fn _UNLOAD_CAMERA_DATA_DICT(cameraDictionary: & CStr) { invoke_ignore!(0x798BE43C9393632B, cameraDictionary) }
	pub fn _IS_CAM_DATA_DICT_LOADED(cameraDictionary: & CStr) -> bool { invoke!(0xDD0B7C5AE58F721D, cameraDictionary) }
	pub fn _0xC285FD21294A1C49(cameraDictionary: & CStr) -> bool { invoke!(0xC285FD21294A1C49, cameraDictionary) }
	pub fn _CAM_CREATE(cameraDictionary: & CStr) { invoke_ignore!(0xB8B207C34285E978, cameraDictionary) }
	pub fn _CAM_DESTROY(cameraDictionary: & CStr) { invoke_ignore!(0x0A5A4F1979ABB40E, cameraDictionary) }
	pub fn _IS_CAMERA_AVAILABLE(cameraDictionary: & CStr) -> bool { invoke!(0x927B810E43E99932, cameraDictionary) }
	pub fn _0x4138EE36BC3DC0A7(p0: Any, p1: Any) -> Any { invoke!(0x4138EE36BC3DC0A7, p0, p1) }
	pub fn _0xFEB8646818294C75(p0: Any, p1: Any) { invoke_ignore!(0xFEB8646818294C75, p0, p1) }
	pub fn _0x29E6655DF3590B0D(p0: Any) { invoke_ignore!(0x29E6655DF3590B0D, p0) }
	pub fn _0xAC77757C05DE9E5A(cameraDictionary: & CStr) { invoke_ignore!(0xAC77757C05DE9E5A, cameraDictionary) }
	pub fn _0x8E036B41C37D0E5F(p0: Any) { invoke_ignore!(0x8E036B41C37D0E5F, p0) }
	pub fn _0x1D9F72DD4FD9A9D7(p0: Any) { invoke_ignore!(0x1D9F72DD4FD9A9D7, p0) }
	pub fn _CAM_CREATE_2(cameraDictionary: & CStr) { invoke_ignore!(0x7B0279170961A73F, cameraDictionary) }
	pub fn _0x728491FB3DFFEF99(p0: Any) { invoke_ignore!(0x728491FB3DFFEF99, p0) }
	pub fn _0x14C4A49E36C29E49() -> Any { invoke!(0x14C4A49E36C29E49) }
	pub fn _0xF824530B612FE0CE() -> Any { invoke!(0xF824530B612FE0CE) }
	pub fn _0xEF9A3132A0AA6B19() -> Any { invoke!(0xEF9A3132A0AA6B19) }
	pub fn _0x5060FA977CEA4455() -> Any { invoke!(0x5060FA977CEA4455) }
	pub fn _GET_PHOTO_MODE_FOCAL_LENGTH() -> f32 { invoke!(0x2533BAFFBE737E54) }
	pub fn _GET_PHOTO_MODE_FOCUS_DISTANCE() -> f32 { invoke!(0x18FC740FFDCD7454) }
	pub fn _GET_PHOTO_MODE_DOF() -> f32 { invoke!(0x4653A741D17F2CD0) }
	pub fn _0x2AB7C81B3F70570C() -> Any { invoke!(0x2AB7C81B3F70570C) }
	pub fn _0x8505E05FC8822843(p0: Any) { invoke_ignore!(0x8505E05FC8822843, p0) }
}
pub mod CLOCK {
	use std::ffi::CStr;
	use crate::core::scripthook::native::*;
	use crate::core::types::*;

	pub fn SET_CLOCK_TIME(hour: i32, minute: i32, second: i32) { invoke_ignore!(0x3A52C59FFB2DEED8, hour, minute, second) }
	pub fn PAUSE_CLOCK(toggle: bool, unused: Any) { invoke_ignore!(0x4D1A590C92BF377E, toggle, unused) }
	pub fn _PAUSE_CLOCK_THIS_FRAME(toggle: bool) { invoke_ignore!(0x568D998A9FF96774, toggle) }
	pub fn ADVANCE_CLOCK_TIME_TO(hour: i32, minute: i32, second: i32) { invoke_ignore!(0x0184750AE88D0B1D, hour, minute, second) }
	pub fn ADD_TO_CLOCK_TIME(hours: i32, minutes: i32, seconds: i32) { invoke_ignore!(0xAB7C251C7701D336, hours, minutes, seconds) }
	pub fn GET_CLOCK_HOURS() -> i32 { invoke!(0xC82CF208C2B19199) }
	pub fn GET_CLOCK_MINUTES() -> i32 { invoke!(0x4E162231B823DBBF) }
	pub fn GET_CLOCK_SECONDS() -> i32 { invoke!(0xB6101ABE62B5F080) }
	pub fn _SET_MILLISECONDS_PER_GAME_MINUTE(ms: i32) { invoke_ignore!(0x04EEDB3848DACF68, ms) }
	pub fn _GET_SECONDS_SINCE_BASE_YEAR() -> i32 { invoke!(0x78FD8BE812E436B2) }
	pub fn SET_CLOCK_DATE(day: i32, month: i32, year: i32) { invoke_ignore!(0x02AD3092562941E2, day, month, year) }
	pub fn GET_CLOCK_DAY_OF_WEEK() -> i32 { invoke!(0x4DD02D4C7FB30076) }
	pub fn GET_CLOCK_DAY_OF_MONTH() -> i32 { invoke!(0xDF2FD796C54480A5) }
	pub fn GET_CLOCK_MONTH() -> i32 { invoke!(0x2D44E8FC79EAB1AC) }
	pub fn GET_CLOCK_YEAR() -> i32 { invoke!(0xE136DCA28C4A48BA) }
	pub fn GET_MILLISECONDS_PER_GAME_MINUTE() -> i32 { invoke!(0xE4CB8D126501EC52) }
	pub fn GET_POSIX_TIME(year: &mut i32, month: &mut i32, day: &mut i32, hour: &mut i32, minute: &mut i32, second: &mut i32) { invoke_ignore!(0x90338AD4A784E455, year, month, day, hour, minute, second) }
	pub fn _GET_POSIX_TIME_STRUCT(outTime: &mut Any) { invoke_ignore!(0x86A68E84E5884951, outTime) }
	pub fn _ADD_TIME_TO_DATE_TIME(inDateTime: &mut Any, timeToAdd: &mut Any, outDateTime: &mut Any) { invoke_ignore!(0x28EEACE9B43D9597, inDateTime, timeToAdd, outDateTime) }
}
pub mod COLLECTION {
	use std::ffi::CStr;
	use crate::core::scripthook::native::*;
	use crate::core::types::*;

	pub fn _COLLECTABLE_GET_CATEGORY(collectableItem: Hash) -> Hash { invoke!(0x725D52F21A5E9EF6, collectableItem) }
	pub fn _COLLECTABLE_GET_SUBCATEGORY(collectableItem: Hash) -> Hash { invoke!(0x6052B4DE6657684F, collectableItem) }
	pub fn _COLLECTABLE_GET_IPL(collectableItem: Hash) -> Hash { invoke!(0x922A79CD4A033B8B, collectableItem) }
	pub fn _COLLECTABLE_GET_PLACEMENT_LOCATION(collectableItem: Hash) -> Vector3 { invoke!(0x1F1DD794908C2BFA, collectableItem) }
	pub fn _COLLECTABLE_GET_NUM_FOUND(collectableItem: Hash) -> i32 { invoke!(0xF83D3DDA4D3C8169, collectableItem) }
	pub fn _COLLECTABLE_GET_NUM_TURNED_IN(collectableItem: Hash) -> i32 { invoke!(0x9A03F22AD446EEAC, collectableItem) }
	pub fn _COLLECTABLE_INCREMENT_NUM_FOUND(collectableItem: Hash, amount: i32) { invoke_ignore!(0x3EA62E56F386C997, collectableItem, amount) }
	pub fn _COLLECTABLE_INCREMENT_NUM_TURNED_IN(collectableItem: Hash, amount: i32) { invoke_ignore!(0x398FAB9C96A81924, collectableItem, amount) }
	pub fn _COLLECTABLE_CATEGORY_GET_NUM_COLLECTABLES(collectableCategory: Hash, collectableSubcategory: Hash) -> i32 { invoke!(0x62CAB7DB62EAD434, collectableCategory, collectableSubcategory) }
	pub fn _COLLECTABLE_GET_COLLECTABLE_ITEM_HASH(index: i32, collectableCategory: Hash, collectableSubcategory: Hash) -> Hash { invoke!(0x126CBEBBA46693CF, index, collectableCategory, collectableSubcategory) }
	pub fn _COLLECTABLE_CATEGORY_GET_NUM_FOUND(collectableCategory: Hash, collectableSubcategory: Hash) -> i32 { invoke!(0x5461C821D00FE15A, collectableCategory, collectableSubcategory) }
	pub fn _COLLECTABLE_CATEGORY_GET_NUM_TURNED_IN(collectableCategory: Hash, collectableSubcategory: Hash) -> i32 { invoke!(0x3A65F4844913A047, collectableCategory, collectableSubcategory) }
	pub fn _0x33825A7388A6B9F6(collectableCategory: Hash, p1: i32) -> i32 { invoke!(0x33825A7388A6B9F6, collectableCategory, p1) }
	pub fn _0x755901C7598B97BC(collectableCategory: Hash, p1: i32) -> i32 { invoke!(0x755901C7598B97BC, collectableCategory, p1) }
	pub fn _0xB9020EC89C07DF04(collectableCategory: Hash, p1: i32, index: i32) -> Any { invoke!(0xB9020EC89C07DF04, collectableCategory, p1, index) }
	pub fn _0xFC832B06127D8E99(collectableCategory: Hash, p1: i32) -> bool { invoke!(0xFC832B06127D8E99, collectableCategory, p1) }
	pub fn COLLECTABLE_CATEGORY_SET_HAS_RECEIVED_LIST(p0: Any, p1: Any, p2: Any) { invoke_ignore!(0x0B6D275D2F242E17, p0, p1, p2) }
	pub fn _0x6BAB7ACED1017204(collectableCategory: Hash, p1: i32) -> bool { invoke!(0x6BAB7ACED1017204, collectableCategory, p1) }
	pub fn _0x61BEFBA3CE7A3BC8(collectableCategory: Hash, p1: i32) -> bool { invoke!(0x61BEFBA3CE7A3BC8, collectableCategory, p1) }
	pub fn _0xC4AC39719C1BB559(collectableCategory: Hash, p1: Any) -> Any { invoke!(0xC4AC39719C1BB559, collectableCategory, p1) }
	pub fn _0x93F2E7B5DB85657B(p0: Any, p1: Any) -> Any { invoke!(0x93F2E7B5DB85657B, p0, p1) }
	pub fn COLLECTABLE_GET_CATEGORY_ITEM_SET_BUY_AWARD(collectableCategory: Hash, p1: Hash) -> Hash { invoke!(0xCC644BC1DD655269, collectableCategory, p1) }
	pub fn _0x9ADEE485726025D4(collectableCategory: Hash) -> Hash { invoke!(0x9ADEE485726025D4, collectableCategory) }
	pub fn _0xD1806FB3EDED6D11(collectableCategory: Hash, p1: i32) -> Any { invoke!(0xD1806FB3EDED6D11, collectableCategory, p1) }
	pub fn _0x3FD91F1A148A0468(collectableCategory: Hash, p1: i32) -> Any { invoke!(0x3FD91F1A148A0468, collectableCategory, p1) }
	pub fn _0xC3CA424E1F12ED0C(collectableCategory: Hash, p1: i32) -> Any { invoke!(0xC3CA424E1F12ED0C, collectableCategory, p1) }
	pub fn _COLLECTABLE_CATEGORY_GET_TOAST_TEXTURE_NAME(collectableCategory: Hash, collectableSubcategory: Hash) -> Hash { invoke!(0xD52D20B0C76BB26D, collectableCategory, collectableSubcategory) }
	pub fn _COLLECTABLE_CATEGORY_GET_TOAST_TEXTURE_DICTIONARY(collectableCategory: Hash, collectableSubcategory: Hash) -> Hash { invoke!(0x13AAECDA43318BFE, collectableCategory, collectableSubcategory) }
	pub fn _0xD297F68928A58130(collectableCategory: Hash, p1: i32) -> Any { invoke!(0xD297F68928A58130, collectableCategory, p1) }
	pub fn _0x775FA1FC87666847(collectableCategory: Hash, p1: i32) -> Any { invoke!(0x775FA1FC87666847, collectableCategory, p1) }
	pub fn _0xEC3959E9950BF56B(p0: i32) -> Any { invoke!(0xEC3959E9950BF56B, p0) }
}
pub mod COMPANION {
	use std::ffi::CStr;
	use crate::core::scripthook::native::*;
	use crate::core::types::*;

	pub fn _0xD730281E496621FB(ped: Ped, p1: Hash) { invoke_ignore!(0xD730281E496621FB, ped, p1) }
	pub fn _0xBF6583E926D13890(p0: Any, p1: Any) { invoke_ignore!(0xBF6583E926D13890, p0, p1) }
	pub fn _0x0A8FD91EDE7B328A(p0: Any, p1: Any) { invoke_ignore!(0x0A8FD91EDE7B328A, p0, p1) }
	pub fn _0x991E3346D788F20F(p0: Any, p1: Any) { invoke_ignore!(0x991E3346D788F20F, p0, p1) }
	pub fn _0xCE27824B5968B79A(p0: Any, p1: Any) { invoke_ignore!(0xCE27824B5968B79A, p0, p1) }
	pub fn _0xF06CBB8CCCA823C0(p0: Any, p1: Any) { invoke_ignore!(0xF06CBB8CCCA823C0, p0, p1) }
	pub fn _0x2917E634206B9E17(ped: Ped, p1: i32) { invoke_ignore!(0x2917E634206B9E17, ped, p1) }
	pub fn _0xD428C3F92FC3F6F8(ped: Ped, p1: & CStr) { invoke_ignore!(0xD428C3F92FC3F6F8, ped, p1) }
	pub fn _ADD_COMPANION_FLAG(ped: Ped, p1: i32) { invoke_ignore!(0xDEB369F6AD168C58, ped, p1) }
	pub fn _REMOVE_COMPANION_FLAG(ped: Ped, p1: i32) { invoke_ignore!(0x1740E3DEE0AE4D27, ped, p1) }
	pub fn _0x0DE02DA3C0F66955(ped: Ped, p1: Hash) { invoke_ignore!(0x0DE02DA3C0F66955, ped, p1) }
	pub fn _SET_COMPANION_ACTIVITY(groupId: i32, activity: Hash) { invoke_ignore!(0x0F1CD8CA9E65D5F6, groupId, activity) }
	pub fn _GET_COMPANION_ACTIVITY(groupId: i32) -> Hash { invoke!(0xB7E0590C86E1711F, groupId) }
	pub fn _0xA079FF7CFB9AC8BD(p0: Any, p1: Any) { invoke_ignore!(0xA079FF7CFB9AC8BD, p0, p1) }
	pub fn _0x61BDA07407754A5C(p0: Any, p1: Any, p2: Any, p3: Any) { invoke_ignore!(0x61BDA07407754A5C, p0, p1, p2, p3) }
	pub fn _0xD55A871E1CE3481B(p0: Any, p1: Any, p2: Any, p3: Any) { invoke_ignore!(0xD55A871E1CE3481B, p0, p1, p2, p3) }
	pub fn _0x0C6A00DAE896614C(p0: Any, p1: Any) { invoke_ignore!(0x0C6A00DAE896614C, p0, p1) }
	pub fn _0x8FB98B719AA0075A(p0: Any, p1: Any, p2: Any, p3: Any, p4: Any) { invoke_ignore!(0x8FB98B719AA0075A, p0, p1, p2, p3, p4) }
	pub fn _0x9C902084F48D2E6C(p0: Any) { invoke_ignore!(0x9C902084F48D2E6C, p0) }
	pub fn _0xD747979C053EFA7A(p0: Any) { invoke_ignore!(0xD747979C053EFA7A, p0) }
	pub fn _0x3CAAD93FA5B9579A(volume: Volume, p1: i32, p2: i32) { invoke_ignore!(0x3CAAD93FA5B9579A, volume, p1, p2) }
	pub fn _0x7274F84B1501B523(p0: Any) { invoke_ignore!(0x7274F84B1501B523, p0) }
	pub fn _0x722FBE08EF5B87BD(p0: Any, p1: Any, p2: Any, p3: Any, p4: Any) -> Any { invoke!(0x722FBE08EF5B87BD, p0, p1, p2, p3, p4) }
	pub fn _ACTIVATE_COMPANION_ANALYSIS(groupId: i32) { invoke_ignore!(0xCBD9EC60495C728C, groupId) }
	pub fn _DEACTIVATE_COMPANION_ANALYSIS(groupId: i32) { invoke_ignore!(0x72B7F65F11FC8896, groupId) }
}
pub mod COMPENDIUM {
	use std::ffi::CStr;
	use crate::core::scripthook::native::*;
	use crate::core::types::*;

	pub fn _0x725D52F21A5E9E22(category: Hash) -> f32 { invoke!(0x725D52F21A5E9E22, category) }
	pub fn _COMPENDIUM_GET_NUM_OF_ENTRIES_IN_CATEGORY(category: Hash) -> i32 { invoke!(0x729D52F61A5A9E22, category) }
	pub fn _0x729D52461AEA9E22(category: Hash) -> i32 { invoke!(0x729D52461AEA9E22, category) }
	pub fn COMPENDIUM_GET_SHORT_DESCRIPTION_FROM_PED(ped: Ped) -> Hash { invoke!(0x6C5E5D48E48B4C65, ped) }
	pub fn COMPENDIUM_GET_SUBCATEGORY_PED_IS_IN(category: Hash, ped: Ped) -> Hash { invoke!(0x9B657550DF55EC96, category, ped) }
	pub fn _COMPENDIUM_GET_SUBCATEGORY_HASH_FROM_ANIMAL_TYPE(category: Hash, animalType: Hash) -> Hash { invoke!(0xCD278B6BFBDBDC22, category, animalType) }
	pub fn COMPENDIUM_GET_NUM_ENTRIES_IN_SUBCATEGORY(category: Hash, subcategory: Hash) -> i32 { invoke!(0xF58A0C0E086E8E36, category, subcategory) }
	pub fn COMPENDIUM_GET_ENTRY_BY_INDEX_IN_SUBCATEGORY(category: Hash, subcategory: Hash, count: i32) -> i32 { invoke!(0x5CEB63B2E3D9895F, category, subcategory, count) }
	pub fn COMPENDIUM_GET_ENTRY_BY_PED_INDEX(category: Hash, ped: Ped) -> i32 { invoke!(0x1CFA0219D8E1CF25, category, ped) }
	pub fn COMPENDIUM_GET_ENTRY_BY_STAT_ITEM(category: Hash, animalType: Hash) -> i32 { invoke!(0x66EC938394D76C85, category, animalType) }
	pub fn COMPENDIUM_GET_SUBCATEGORY_TOAST_APP_ID(category: Hash, subcategory: Hash) -> Any { invoke!(0x2BF30D9D4D680112, category, subcategory) }
	pub fn COMPENDIUM_GET_MAP_DISCOVERABLE_FROM_STAT_ITEM(animalStatItem: Hash, x: f32, y: f32, z: f32) -> Hash { invoke!(0x729D54121A5E9E20, animalStatItem, x, y, z) }
	pub fn COMPENDIUM_ANIMAL_OBSERVED_BY_STAT_NAME(animalType: Hash, disableCompendiumToast: bool) { invoke_ignore!(0x725D52F26A5E9E10, animalType, disableCompendiumToast) }
	pub fn COMPENDIUM_WAS_ANIMAL_OBSERVED(ped: Ped) -> bool { invoke!(0x23B5E9C5160BC04F, ped) }
	pub fn COMPENDIUM_ANIMAL_SET_DISCOVERED(compendiumEntry: i32) { invoke_ignore!(0x67F35C7C9F2BDCFE, compendiumEntry) }
	pub fn COMPENDIUM_GET_STUDY_AWARD_ID(ped: Ped) -> Any { invoke!(0x9F678782720349E4, ped) }
	pub fn COMPENDIUM_ANIMAL_GET_SAMPLE_INVENTORY_ITEM(compendiumEntry: i32) -> Any { invoke!(0x4E4ACAE1C671A9DA, compendiumEntry) }
	pub fn COMPENDIUM_ANIMAL_HAS_SAMPLE(compendiumEntry: i32) -> bool { invoke!(0x6FC24625E4FCAC27, compendiumEntry) }
	pub fn COMPENDIUM_ANIMAL_HAS_STAMP(compendiumEntry: i32) -> bool { invoke!(0xBCF569FC32FFF456, compendiumEntry) }
	pub fn COMPENDIUM_GET_SUBCATEGORY_SAMPLE_TOAST_TITLE(category: Hash, subcategory: Hash) -> *const char { invoke!(0x5E50C67EB60951E6, category, subcategory) }
	pub fn COMPENDIUM_GET_SUBCATEGORY_SAMPLE_TOAST_DESC_PROGRESS(category: Hash, subcategory: Hash) -> *const char { invoke!(0x82BFB5B367957699, category, subcategory) }
	pub fn COMPENDIUM_GET_SUBCATEGORY_SAMPLE_TOAST_DESC_COMPLETE(category: Hash, subcategory: Hash) -> *const char { invoke!(0x59D4D68CDB82427C, category, subcategory) }
	pub fn COMPENDIUM_FISH_CAUGHT(ped: Ped, category: Hash) { invoke_ignore!(0x725D52F21A5E9E00, ped, category) }
	pub fn COMPENDIUM_FISH_GET_LURE_SUITABILITY_BY_STAT_ITEM(animalType: Hash, baitType: Hash) -> i32 { invoke!(0x725D52F21A5E9E81, animalType, baitType) }
	pub fn COMPENDIUM_GANG_CAMP_FOUND(p0: Any, p1: Any) { invoke_ignore!(0x725D52F21A5E9E03, p0, p1) }
	pub fn COMPENDIUM_GANG_AMBUSH_SURVIVED(p0: Any) { invoke_ignore!(0x725D52F21A5E9E04, p0) }
	pub fn COMPENDIUM_GANG_ENCOUNTERED(p0: Any) { invoke_ignore!(0x725D52F21A5E9E05, p0) }
	pub fn COMPENDIUM_GANG_BOUNTY_CAPTURED(p0: Any) { invoke_ignore!(0x725D52F21A5E9E06, p0) }
	pub fn COMPENDIUM_GANG_MEMBER_KILLED(p0: Any) { invoke_ignore!(0x725D52F21A5E9E07, p0) }
	pub fn COMPENDIUM_GANG_HIDEOUT_FOUND(p0: Any, p1: Any) { invoke_ignore!(0x725D52F21A5E9E08, p0, p1) }
	pub fn COMPENDIUM_HERB_PICKED(herbType: Hash, x: f32, y: f32, z: f32) { invoke_ignore!(0x725D52F21A5E9E09, herbType, x, y, z) }
	pub fn COMPENDIUM_HORSE_BONDING(ped: Ped, bondingLevel: i32) { invoke_ignore!(0x725D52F21A5E9E50, ped, bondingLevel) }
	pub fn COMPENDIUM_HORSE_WILD_BROKEN(ped: Ped) { invoke_ignore!(0x725852D21A2E9E50, ped) }
	pub fn COMPENDIUM_HORSE_OBSERVED(ped: Ped, disableCompendiumToast: bool) { invoke_ignore!(0x725D58F2125E5E50, ped, disableCompendiumToast) }
}
pub mod CRASHLOG {
	use std::ffi::CStr;
	use crate::core::scripthook::native::*;
	use crate::core::types::*;

	pub fn _0x0FD3ECF9D0C8655F(p0: & CStr) { invoke_ignore!(0x0FD3ECF9D0C8655F, p0) }
	pub fn _0xCA0BAC376C541978(p0: & CStr) { invoke_ignore!(0xCA0BAC376C541978, p0) }
	pub fn _0x3A66F1963B223F61(p0: & CStr) -> bool { invoke!(0x3A66F1963B223F61, p0) }
	pub fn _0x7C680FF55617F82F() -> bool { invoke!(0x7C680FF55617F82F) }
	pub fn _0xD8E3D22AA4F0E0A5(p0: & CStr) -> bool { invoke!(0xD8E3D22AA4F0E0A5, p0) }
	pub fn _0xA67F0B039D9CD513(p0: bool) -> bool { invoke!(0xA67F0B039D9CD513, p0) }
	pub fn _0xE72E234B30DA7B7A(p0: i32) -> bool { invoke!(0xE72E234B30DA7B7A, p0) }
	pub fn _0x87F005C969EF1563(p0: f32) -> bool { invoke!(0x87F005C969EF1563, p0) }
	pub fn _0x23CCAB8F40B9CBEE(x: f32, y: f32, z: f32) -> bool { invoke!(0x23CCAB8F40B9CBEE, x, y, z) }
	pub fn _0xF0D545C1EEAD614A() -> bool { invoke!(0xF0D545C1EEAD614A) }
	pub fn _0x33C1D63E55FA4284(p0: & CStr) -> bool { invoke!(0x33C1D63E55FA4284, p0) }
	pub fn _0x4E42CA5BCD45444A() { invoke_ignore!(0x4E42CA5BCD45444A) }
	pub fn _0xDA05310EA94DC8C6(p0: & CStr, p1: & CStr) { invoke_ignore!(0xDA05310EA94DC8C6, p0, p1) }
}
pub mod CREW {
	use std::ffi::CStr;
	use crate::core::scripthook::native::*;
	use crate::core::types::*;

	pub fn NETWORK_FIND_GAMERS_IN_CREW(crewId: i32) -> bool { invoke!(0xE532D6811B3A4D2A, crewId) }
	pub fn NETWORK_CLAN_SERVICE_IS_VALID() -> bool { invoke!(0x579CCED0265D4896) }
	pub fn NETWORK_CLAN_PLAYER_IS_ACTIVE(gamerHandle: &mut Any) -> bool { invoke!(0xB124B57F571D8F18, gamerHandle) }
	pub fn NETWORK_CLAN_PLAYER_GET_DESC(clanDesc: &mut Any, bufferSize: i32, gamerHandle: &mut Any) -> bool { invoke!(0xEEE6EACBE8874FBA, clanDesc, bufferSize, gamerHandle) }
	pub fn NETWORK_CLAN_GET_LOCAL_MEMBERSHIPS_COUNT() -> i32 { invoke!(0x1F471B79ACC90BEF) }
	pub fn NETWORK_CLAN_GET_MEMBERSHIP_DESC(memberDesc: &mut Any, p1: i32) -> bool { invoke!(0x48DE78AF2C8885B8, memberDesc, p1) }
	pub fn _0x58D378AF2C8765B7(p0: Any) -> bool { invoke!(0x58D378AF2C8765B7, p0) }
	pub fn _NETWORK_CLAN_INVITE_PLAYER(p0: Any) -> bool { invoke!(0xC685B014CE3D988B, p0) }
	pub fn _NETWORK_ACCEPT_CLAN_INVITE(crewInviteIndex: i32) -> bool { invoke!(0x8E2143144B8E188D, crewInviteIndex) }
	pub fn NETWORK_CLAN_REQUEST_EMBLEM(p0: Any) -> bool { invoke!(0x13518FF1C6B28938, p0) }
	pub fn NETWORK_CLAN_IS_EMBLEM_READY(p0: Any, p1: &mut Any) -> bool { invoke!(0xA134777FF7F33331, p0, p1) }
	pub fn NETWORK_CLAN_RELEASE_EMBLEM(p0: Any) { invoke_ignore!(0x113E6E3E50E286B0, p0) }
	pub fn NETWORK_GET_PRIMARY_CLAN_DATA_CLEAR() -> Any { invoke!(0x9AA46BADAD0E27ED) }
	pub fn NETWORK_GET_PRIMARY_CLAN_DATA_START(p0: &mut Any, p1: Any) -> bool { invoke!(0xCE86D8191B762107, p0, p1) }
	pub fn NETWORK_GET_PRIMARY_CLAN_DATA_PENDING() -> Any { invoke!(0xB5074DB804E28CE7) }
	pub fn NETWORK_GET_PRIMARY_CLAN_DATA_SUCCESS() -> Any { invoke!(0x5B4F04F19376A0BA) }
	pub fn NETWORK_GET_PRIMARY_CLAN_DATA_NEW(p0: &mut Any, p1: &mut Any) -> bool { invoke!(0xC080FF658B2E41DA, p0, p1) }
	pub fn _NETWORK_CLAN_SET_ACTIVE(p0: Any) -> Any { invoke!(0xC080FF658B2E51DA, p0) }
}
pub mod DATABINDING {
	use std::ffi::CStr;
	use crate::core::scripthook::native::*;
	use crate::core::types::*;

	pub fn _DATABINDING_GET_DATA_CONTAINER_FROM_PATH(p0: & CStr) -> Any { invoke!(0x0C827D175F1292F2, p0) }
	pub fn _DATABINDING_GET_DATA_CONTAINER_FROM_CHILD_INDEX(entryId: Hash, p1: i32) -> Any { invoke!(0x0C827D175F1292F3, entryId, p1) }
	pub fn _0xD7DB94AB78E8EBE4(p0: & CStr, p1: Hash) -> Any { invoke!(0xD7DB94AB78E8EBE4, p0, p1) }
	pub fn _DATABINDING_ADD_DATA_CONTAINER_FROM_PATH(p0: & CStr, p1: & CStr) -> Any { invoke!(0x0C827D175F1292F4, p0, p1) }
	pub fn _DATABINDING_ADD_DATA_CONTAINER_BY_HASH(p0: Any, p1: Hash) -> Any { invoke!(0x98BB14345BB68257, p0, p1) }
	pub fn _DATABINDING_ADD_DATA_CONTAINER(entryId: Hash, p1: & CStr) -> Hash { invoke!(0xEB4F9A3537EEABCD, entryId, p1) }
	pub fn _DATABINDING_ADD_DATA_BOOL_FROM_PATH(p0: & CStr, p1: & CStr, p2: bool) -> Any { invoke!(0x37BB86A751148A6A, p0, p1, p2) }
	pub fn _DATABINDING_ADD_DATA_BOOL_BY_HASH(p0: Any, p1: Hash, p2: bool) -> Any { invoke!(0xBC95D3AE2ECA70D6, p0, p1, p2) }
	pub fn _DATABINDING_ADD_DATA_BOOL(p0: Any, p1: & CStr, p2: bool) -> Any { invoke!(0x58BAA5F635DA2FF4, p0, p1, p2) }
	pub fn _DATABINDING_ADD_DATA_INT_BY_HASH(p0: Any, p1: Hash, p2: i32) -> Any { invoke!(0x267F9527F4350ADE, p0, p1, p2) }
	pub fn _DATABINDING_ADD_DATA_INT(p0: Any, p1: & CStr, p2: Hash) -> Any { invoke!(0x307A3247C5457BDE, p0, p1, p2) }
	pub fn _DATABINDING_ADD_DATA_FLOAT(p0: Any, p1: & CStr, p2: f32) -> Any { invoke!(0x5154228273ADB9A6, p0, p1, p2) }
	pub fn _DATABINDING_ADD_DATA_HASH_BY_HASH(p0: Any, p1: Hash, p2: Hash) -> Any { invoke!(0x8E173DFB041993C6, p0, p1, p2) }
	pub fn _DATABINDING_ADD_DATA_HASH(p0: Any, p1: & CStr, p2: Hash) -> Any { invoke!(0x8538F1205D60ECA6, p0, p1, p2) }
	pub fn _DATABINDING_ADD_DATA_STRING_FROM_PATH(p0: & CStr, p1: & CStr, p2: & CStr) -> Any { invoke!(0xA381DE86EE170C4A, p0, p1, p2) }
	pub fn _DATABINDING_ADD_DATA_STRING_BY_HASH(p0: Any, p1: Hash, p2: & CStr) -> Any { invoke!(0xEAD09E76E22630C3, p0, p1, p2) }
	pub fn _DATABINDING_ADD_DATA_STRING(p0: Any, p1: & CStr, p2: & CStr) -> Any { invoke!(0x617FCA1C5652BBAD, p0, p1, p2) }
	pub fn _DATABINDING_ADD_DATA_GANG_ID(p0: Any, p1: & CStr, gangId: Any) -> Any { invoke!(0x7D0F2014DB28DD00, p0, p1, gangId) }
	pub fn _DATABINDING_ADD_DATA_POSSE_ID(p0: Any, p1: & CStr, posseId: Any) -> Any { invoke!(0x7D0F2014DB28DD01, p0, p1, posseId) }
	pub fn _0x294AF5323F44B053(p0: Any, p1: & CStr, p2: Any) -> Any { invoke!(0x294AF5323F44B053, p0, p1, p2) }
	pub fn _DATABINDING_ADD_UI_ITEM_LIST_FROM_PATH(p0: & CStr, p1: & CStr) -> Any { invoke!(0xDB5B9A474148F699, p0, p1) }
	pub fn _DATABINDING_ADD_UI_ITEM_LIST_BY_HASH(p0: Any, p1: Hash) -> Any { invoke!(0x3C7799283325181B, p0, p1) }
	pub fn _DATABINDING_ADD_UI_ITEM_LIST(p0: Any, p1: & CStr) -> Any { invoke!(0xFE74FA57E0CE6824, p0, p1) }
	pub fn _DATABINDING_INSERT_UI_ITEM_TO_LIST_FROM_CONTEXT_STRING_ALIAS(p0: Any, index: i32, p2: & CStr, p3: Any) { invoke_ignore!(0x5859E970794D92F3, p0, index, p2, p3) }
	pub fn _DATABINDING_INSERT_UI_ITEM_TO_LIST_FROM_CONTEXT_HASH_ALIAS(p0: Any, index: i32, p2: Hash, p3: Any) { invoke_ignore!(0xEE97A05C05F16E41, p0, index, p2, p3) }
	pub fn _DATABINDING_INSERT_UI_ITEM_TO_LIST_FROM_PATH_STRING_ALIAS(p0: Any, p1: Any, p2: & CStr, p3: Any) { invoke_ignore!(0x5740774F608E4FC8, p0, p1, p2, p3) }
	pub fn _0x6318FB3BE37E11B3(entryId: Hash, index: i32) { invoke_ignore!(0x6318FB3BE37E11B3, entryId, index) }
	pub fn _DATABINDING_REMOVE_BINDING_ARRAY_ITEM_BY_DATA_CONTEXT_ID(p0: Any, entryId: Hash) { invoke_ignore!(0xF68B1726EAF7B285, p0, entryId) }
	pub fn _DATABINDING_CLEAR_BINDING_ARRAY(entryId: Hash) { invoke_ignore!(0xA1F15C1D03DF802D, entryId) }
	pub fn _DATABINDING_GET_ITEM_CONTEXT_BY_INDEX(p0: Any, index: i32) -> Any { invoke!(0xE96D7F9FEFCC105F, p0, index) }
	pub fn _DATABINDING_SET_TEMPLATED_UI_ITEM_LIST_SIZE(p0: Any, p1: i32) { invoke_ignore!(0xFE74FA57E0CE6825, p0, p1) }
	pub fn _DATABINDING_SET_TEMPLATED_UI_ITEM_HASH_ALIAS(p0: Any, p1: i32, p2: Hash) { invoke_ignore!(0x0AE7138D0541F2DE, p0, p1, p2) }
	pub fn _0xD48993A61938C64D(p0: Any, p1: & CStr) -> Any { invoke!(0xD48993A61938C64D, p0, p1) }
	pub fn _DATABINDING_ADD_HASH_ARRAY(p0: Any, p1: & CStr) -> Any { invoke!(0x52F5F08278EA5D75, p0, p1) }
	pub fn _DATABINDING_ADD_STRING_ARRAY(p0: Any, p1: & CStr) -> Any { invoke!(0x1B23E0627BDBFE85, p0, p1) }
	pub fn _0x1919D59E60FD516E(p0: Any, p1: i32, p2: i32) { invoke_ignore!(0x1919D59E60FD516E, p0, p1, p2) }
	pub fn _0x7FC60C94C83C5CD7(p0: Any, p1: Hash, p2: i32) { invoke_ignore!(0x7FC60C94C83C5CD7, p0, p1, p2) }
	pub fn _0xC900CEC8A172375B(p0: Any, p1: & CStr, p2: i32) { invoke_ignore!(0xC900CEC8A172375B, p0, p1, p2) }
	pub fn _0x02B21B6BEEDD83CC(entryId: Hash, p1: i32) -> Any { invoke!(0x02B21B6BEEDD83CC, entryId, p1) }
	pub fn _0xF47E33F8D2523825(p0: Any, p1: i32) -> Any { invoke!(0xF47E33F8D2523825, p0, p1) }
	pub fn _0x3BF0767CF33FCC88(entryId: Hash) { invoke_ignore!(0x3BF0767CF33FCC88, entryId) }
	pub fn _DATABINDING_GET_ARRAY_COUNT(entryId: Hash) -> Any { invoke!(0xD23F5DE04FE717E2, entryId) }
	pub fn _DATABINDING_WRITE_DATA_SCRIPT_VARIABLES(p0: i32, p1: i32) { invoke_ignore!(0xAB888B4B91046771, p0, p1) }
	pub fn _DATABINDING_WRITE_DATA_BOOL(p0: Any, p1: bool) { invoke_ignore!(0xAB888B4B91046770, p0, p1) }
	pub fn _DATABINDING_WRITE_DATA_BOOL_FROM_PARENT(p0: Any, p1: & CStr, p2: bool) { invoke_ignore!(0xBDFE546E4C2D0E21, p0, p1, p2) }
	pub fn _0xBFC83DA249BEFCC9(p0: Any, p1: Hash, p2: Any) { invoke_ignore!(0xBFC83DA249BEFCC9, p0, p1, p2) }
	pub fn _DATABINDING_WRITE_DATA_INT(p0: Any, p1: i32) { invoke_ignore!(0x335C3F6B3766B8D9, p0, p1) }
	pub fn _DATABINDING_WRITE_DATA_INT_FROM_PARENT(p0: Any, p1: & CStr, p2: i32) { invoke_ignore!(0x9EFA98238BA08FC4, p0, p1, p2) }
	pub fn _DATABINDING_WRITE_DATA_INT_FROM_PARENT_BY_HASH(p0: Any, p1: Hash, p2: Any) { invoke_ignore!(0x9D6E10A41D6ED6EC, p0, p1, p2) }
	pub fn _DATABINDING_WRITE_DATA_FLOAT(p0: Any, p1: f32) { invoke_ignore!(0xDF504BECEB15DA93, p0, p1) }
	pub fn _0x05AC9E1E02975AFB(p0: Any, p1: & CStr, p2: f32) { invoke_ignore!(0x05AC9E1E02975AFB, p0, p1, p2) }
	pub fn _DATABINDING_WRITE_DATA_HASH_STRING(p0: Any, p1: Hash) { invoke_ignore!(0xACDEF586BD71B1FD, p0, p1) }
	pub fn _DATABINDING_WRITE_DATA_HASH_STRING_FROM_PARENT(p0: Any, p1: & CStr, p2: Hash) { invoke_ignore!(0x0971F04E1EAA7AE8, p0, p1, p2) }
	pub fn _DATABINDING_WRITE_DATA_HASH_STRING_FROM_PARENT_BY_HASH(p0: Any, p1: Hash, p2: Any) { invoke_ignore!(0x20209529689E0953, p0, p1, p2) }
	pub fn _DATABINDING_WRITE_DATA_STRING(p0: Any, p1: & CStr) { invoke_ignore!(0xE1BD342F2872AEE9, p0, p1) }
	pub fn DATABINDING_WRITE_STRING_FROM_PARENT(p0: Any, p1: & CStr, p2: & CStr) { invoke_ignore!(0x4FF713B2F17A391E, p0, p1, p2) }
	pub fn _DATABINDING_WRITE_STRING_FROM_HASH(p0: Any, p1: Hash, p2: & CStr) { invoke_ignore!(0xA3BD6FF95E713EE5, p0, p1, p2) }
	pub fn _DATABINDING_WRITE_DATA_GANG_ID(p0: Any, p1: & CStr, gangId: Any) { invoke_ignore!(0xC70041408E16BE2D, p0, p1, gangId) }
	pub fn _DATABINDING_WRITE_DATA_POSSE_ID(p0: Any, p1: & CStr, posseId: Any) { invoke_ignore!(0xC70041408E16BE2E, p0, p1, posseId) }
	pub fn _0x422179C7F6AD9304(p0: Any, gamerHandle: &mut Any) { invoke_ignore!(0x422179C7F6AD9304, p0, gamerHandle) }
	pub fn _DATABINDING_READ_DATA_BOOL(p0: Any) -> Any { invoke!(0x5EEFBD4B6D7CD6EB, p0) }
	pub fn _DATABINDING_READ_DATA_BOOL_FROM_PARENT(p0: Any, p1: & CStr) -> Any { invoke!(0xA8EDE09FE07BD77F, p0, p1) }
	pub fn _DATABINDING_READ_DATA_BOOL_FROM_PARENT_BY_HASH(p0: Any, p1: Hash) -> Any { invoke!(0x4CDC3FDDFAE07EB3, p0, p1) }
	pub fn DATABINDING_READ_INT(p0: Any) -> i32 { invoke!(0x570784D782597512, p0) }
	pub fn _DATABINDING_READ_DATA_INT_FROM_PARENT(p0: Any, p1: & CStr) -> Any { invoke!(0xFFC566A4801F6B40, p0, p1) }
	pub fn _DATABINDING_READ_DATA_INT_FROM_PARENT_BY_HASH(p0: Any, p1: Hash) -> Any { invoke!(0xB5F668B648EC0970, p0, p1) }
	pub fn _DATABINDING_READ_FLOAT(entryId: Hash) -> f32 { invoke!(0x5FE444EB67C70AD4, entryId) }
	pub fn _DATABINDING_READ_HASH(entryId: Hash) -> Hash { invoke!(0x81D7183E7A8ECA72, entryId) }
	pub fn _DATABINDING_READ_DATA_HASH_STRING_FROM_PARENT(p0: Any, p1: & CStr) -> Any { invoke!(0x9B535990B01B62DE, p0, p1) }
	pub fn _DATABINDING_READ_DATA_HASH_STRING_FROM_PARENT_BY_HASH(p0: Any, p1: Hash) -> Any { invoke!(0x1F43BC25A119B252, p0, p1) }
	pub fn _DATABINDING_READ_DATA_STRING(p0: Any) -> Any { invoke!(0x3D290B5FFA7C5151, p0) }
	pub fn _DATABINDING_READ_DATA_STRING_FROM_PARENT(p0: Any, p1: & CStr) -> Any { invoke!(0x6323AD277C4A2AFB, p0, p1) }
	pub fn _0x6329C34BEE5BFF4B(p0: Any, p1: Hash) -> Any { invoke!(0x6329C34BEE5BFF4B, p0, p1) }
	pub fn _0xE6AAB897120492D6(p0: Any, p1: & CStr) -> Any { invoke!(0xE6AAB897120492D6, p0, p1) }
	pub fn _0xE6AAB897120492D7(p0: Any, p1: & CStr, p2: Any) { invoke_ignore!(0xE6AAB897120492D7, p0, p1, p2) }
	pub fn _0xB138CA787F3DD858(p0: Any, p1: & CStr, p2: Any) { invoke_ignore!(0xB138CA787F3DD858, p0, p1, p2) }
	pub fn DATABINDING_IS_ENTRY_VALID(entryId: Hash) -> bool { invoke!(0x1E7130793AAAAB8D, entryId) }
	pub fn _DATABINDING_REMOVE_DATA_ENTRY(entryId: Hash) { invoke_ignore!(0x0AE9938D0541F2DA, entryId) }
	pub fn _VIRTUAL_COLLECTION_SET_SIZE(p0: Any, size: i32) { invoke_ignore!(0x9DCE9B01A93B58BC, p0, size) }
	pub fn _VIRTUAL_COLLECTION_SET_INTEREST_INDEX(p0: Any, interestIndex: i32) { invoke_ignore!(0x49A8447533308BCF, p0, interestIndex) }
	pub fn _VIRTUAL_COLLECTION_ITEM_ADD(p0: Any, index: i32, p2: Hash, p3: Any) { invoke_ignore!(0x6DCBF187221CF73D, p0, index, p2, p3) }
	pub fn _VIRTUAL_COLLECTION_RESET(p0: Any) { invoke_ignore!(0x09D95666ED2B5F60, p0) }
	pub fn _VIRTUAL_COLLECTION_EXISTS(p0: Any) -> Any { invoke!(0x37963B56755BFB35, p0) }
}
pub mod DATAFILE {
	use std::ffi::CStr;
	use crate::core::scripthook::native::*;
	use crate::core::types::*;

	pub fn DATAFILE_WATCH_REQUEST_ID(id: i32) { invoke_ignore!(0xA5834834CA8FD7FC, id) }
	pub fn DATAFILE_HAS_LOADED_FILE_DATA(p0: Any) -> bool { invoke!(0x17279C820464CEE0, p0) }
	pub fn DATAFILE_HAS_VALID_FILE_DATA(p0: Any) -> bool { invoke!(0xE60100389E50EADE, p0) }
	pub fn DATAFILE_SELECT_ACTIVE_FILE(p0: Any, p1: Any) -> bool { invoke!(0x46102A0989AD80B5, p0, p1) }
	pub fn DATAFILE_DELETE_REQUESTED_FILE(p0: Any) -> bool { invoke!(0x604B8ED1A482F9DF, p0) }
	pub fn UGC2_SET_PLAYER_DATA(p0: Any, p1: Any, p2: Any, p3: Any) -> Any { invoke!(0xE79C70E77E0973C7, p0, p1, p2, p3) }
	pub fn DATAFILE_UGC_SELECT_DATA(ugcRequestId: Any, index: i32, p2: Any) -> Any { invoke!(0x790EC421078F5C4E, ugcRequestId, index, p2) }
	pub fn DATAFILE_CREATE(index: i32) { invoke_ignore!(0x56B7291FB953DD51, index) }
	pub fn DATAFILE_DELETE(index: i32) { invoke_ignore!(0x9FB90EEDEA9F2D5C, index) }
	pub fn DATAFILE_GET_FILE_DICT(index: i32) -> *mut Any { invoke!(0xBBD8CF823CAE557C, index) }
	pub fn DATADICT_IS_DICT_VALID(fileDict: &mut Any) -> bool { invoke!(0x4607D57C5F7D332A, fileDict) }
	pub fn DATADICT_IS_ARRAY_VALID(fileDict: &mut Any) -> bool { invoke!(0xB04B69CF277D15C0, fileDict) }
	pub fn DATADICT_SET_INT(objectData: &mut Any, key: & CStr, value: i32) { invoke_ignore!(0x26FDF5E99AA2F3E9, objectData, key, value) }
	pub fn DATADICT_GET_BOOL(objectData: &mut Any, key: & CStr) -> bool { invoke!(0x175E915A486EE548, objectData, key) }
	pub fn DATADICT_GET_INT(objectData: &mut Any, key: & CStr) -> i32 { invoke!(0x9D896A3B87D96E2B, objectData, key) }
	pub fn DATADICT_GET_FLOAT(objectData: &mut Any, key: & CStr) -> f32 { invoke!(0x814643ECA258ADF5, objectData, key) }
	pub fn DATADICT_GET_STRING(objectData: &mut Any, key: & CStr) -> *const char { invoke!(0xE37B38C0B4E95DFA, objectData, key) }
	pub fn DATADICT_GET_VECTOR(objectData: &mut Any, key: & CStr) -> Vector3 { invoke!(0xE459C941431E0FFA, objectData, key) }
	pub fn DATADICT_GET_DICT(objectData: &mut Any, key: & CStr) -> *mut Any { invoke!(0x4D7A30130F46AC9C, objectData, key) }
	pub fn DATADICT_GET_ARRAY(objectData: &mut Any, key: & CStr) -> *mut Any { invoke!(0x1B5447CF18544B18, objectData, key) }
	pub fn DATADICT_GET_TYPE(objectData: &mut Any, key: & CStr) -> i32 { invoke!(0x92E11E3CA4C7CDF0, objectData, key) }
	pub fn _0xBC0DF006A4952C68(p0: Any, p1: Any, p2: Any) { invoke_ignore!(0xBC0DF006A4952C68, p0, p1, p2) }
	pub fn _0x9F130129EBC31B34(p0: Any, p1: Any, p2: Any) { invoke_ignore!(0x9F130129EBC31B34, p0, p1, p2) }
	pub fn _0x277251C161B4C3F4(p0: Any, p1: Any, p2: Any) { invoke_ignore!(0x277251C161B4C3F4, p0, p1, p2) }
	pub fn _0x1C65CC931C0F946F(p0: Any, p1: Any, p2: Any) { invoke_ignore!(0x1C65CC931C0F946F, p0, p1, p2) }
	pub fn _0x7681B677400C7071(p0: Any, p1: Any, p2: Any, p3: Any, p4: Any) { invoke_ignore!(0x7681B677400C7071, p0, p1, p2, p3, p4) }
	pub fn DATAARRAY_GET_BOOL(arrayData: &mut Any, arrayIndex: i32) -> bool { invoke!(0xAB1231D2DE52F2D3, arrayData, arrayIndex) }
	pub fn DATAARRAY_GET_INT(arrayData: &mut Any, arrayIndex: i32) -> i32 { invoke!(0x96DEA500B6EBBE53, arrayData, arrayIndex) }
	pub fn DATAARRAY_GET_FLOAT(arrayData: &mut Any, arrayIndex: i32) -> f32 { invoke!(0xA9D003CF419CB81E, arrayData, arrayIndex) }
	pub fn DATAARRAY_GET_STRING(arrayData: &mut Any, arrayIndex: i32) -> *const char { invoke!(0xB6790A8FF80F889F, arrayData, arrayIndex) }
	pub fn DATAARRAY_GET_VECTOR(arrayData: &mut Any, arrayIndex: i32) -> Vector3 { invoke!(0x850DA2750DA14E9A, arrayData, arrayIndex) }
	pub fn DATAARRAY_GET_DICT(arrayData: &mut Any, arrayIndex: i32) -> *mut Any { invoke!(0xA010655985853485, arrayData, arrayIndex) }
	pub fn DATAARRAY_GET_COUNT(arrayData: &mut Any) -> i32 { invoke!(0x6A885BF69239E539, arrayData) }
	pub fn DATAARRAY_GET_TYPE(arrayData: &mut Any, arrayIndex: i32) -> i32 { invoke!(0x151DAFE6B3B9888F, arrayData, arrayIndex) }
	pub fn _0x4F9E3ED7617123AC(p0: Any) -> Any { invoke!(0x4F9E3ED7617123AC, p0) }
	pub fn _0xCA56DD6AB7A39F64(p0: Any) -> Any { invoke!(0xCA56DD6AB7A39F64, p0) }
	pub fn _PARSEDDATA_LOAD_FILE_HASH(p0: Hash) -> i32 { invoke!(0xD97D8D905F1562F2, p0) }
	pub fn _PARSEDDATA_UNLOAD_FILE(fileHandle: i32) { invoke_ignore!(0x129567F0C05F81B9, fileHandle) }
	pub fn PARSEDDATA_IS_FILE_VALID(fileHandle: i32) -> bool { invoke!(0x7907969497EA92F5, fileHandle) }
	pub fn PARSEDDATA_IS_FILE_LOADED(fileHandle: i32) -> bool { invoke!(0x603AC35FD4602C76, fileHandle) }
	pub fn _0x3168BA5D6DECE323() { invoke_ignore!(0x3168BA5D6DECE323) }
	pub fn _PARSEDDATA_REGISTER_QUERY(p0: Any, p1: Any, p2: Any) -> Any { invoke!(0xAE156A747C39A741, p0, p1, p2) }
	pub fn PARSEDDATA_RQ_FILLOUT_NODE(p0: &mut i32, p1: &mut Any) -> bool { invoke!(0x83C3ED532B6E5D07, p0, p1) }
	pub fn _PARSEDDATA_RQ_GET_NUM_NODES(p0: Any) -> Any { invoke!(0xDF01B1F7A886B42D, p0) }
	pub fn _0xE13634BB6BAF0734(p0: i32, p1: i32) -> i32 { invoke!(0xE13634BB6BAF0734, p0, p1) }
	pub fn _PARSEDDATA_GET_NUM_CHILDREN(p0: Any, p1: Any) -> Any { invoke!(0x6BEB168D5195E7AB, p0, p1) }
	pub fn PARSEDDATA_RQ_FILLOUT_HASH(p0: &mut Hash, p1: &mut Any) -> bool { invoke!(0xFBFF3FF2F5E80C0B, p0, p1) }
	pub fn _PARSEDDATA_RQ_FILLOUT_STRING_63(p0: &mut CStr, p1: &mut Any) -> bool { invoke!(0x08EAF8E9F2EB7B2E, p0, p1) }
	pub fn PARSEDDATA_RQ_FILLOUT_STRING_127(p0: &mut CStr, p1: &mut Any) -> bool { invoke!(0x951327435DC5164B, p0, p1) }
	pub fn _PARSEDDATA_RQ_FILLOUT_VECTOR(p0: &mut Vector3, p1: &mut Any) -> bool { invoke!(0x06FBF89B12DA279C, p0, p1) }
	pub fn _PARSEDDATA_RQ_FILLOUT_FLOAT(p0: &mut f32, p1: &mut Any) -> bool { invoke!(0x7F034FC3E891B57A, p0, p1) }
	pub fn _PARSEDDATA_RQ_FILLOUT_INT(p0: &mut i32, p1: &mut Any) -> bool { invoke!(0xEF44ACC657352A35, p0, p1) }
	pub fn _PARSEDDATA_RQ_FILLOUT_BOOL(p0: &mut bool, p1: &mut Any) -> bool { invoke!(0x0D9138F3F8261DF7, p0, p1) }
	pub fn _PARSEDDATA_GET_FILE(p0: &mut Any) { invoke_ignore!(0x91DED5DD64BB2691, p0) }
	pub fn _PARSEDDATA_GET_ENTRIES(p0: &mut Any) -> bool { invoke!(0xED4413CEE1BF142C, p0) }
	pub fn _PARSEDDATA_GET_FLOAT(p0: &mut Any, p1: &mut Any, p2: Hash) -> bool { invoke!(0xB2B42607F7867576, p0, p1, p2) }
	pub fn _PARSEDDATA_GET_INT(p0: &mut Any, p1: &mut Any, p2: Hash) -> bool { invoke!(0x52FC26D2D2FC2987, p0, p1, p2) }
	pub fn _PARSEDDATA_GET_SECTION(p0: &mut Any, p1: &mut Any, section: Hash) -> bool { invoke!(0x44B3A36933AC009C, p0, p1, section) }
	pub fn _PARSEDDATA_GET_BOOL(p0: &mut bool, p1: &mut Any, p2: Hash) -> bool { invoke!(0xA63CD20F19B961AB, p0, p1, p2) }
}
pub mod DEBUG {
	use std::ffi::CStr;
	use crate::core::scripthook::native::*;
	use crate::core::types::*;

	pub fn _0xACF9CB705BEFA8CB() -> Any { invoke!(0xACF9CB705BEFA8CB) }
	pub fn _0xA8D970D8A72640A6() -> Any { invoke!(0xA8D970D8A72640A6) }
	pub fn GET_GAME_VERSION_NAME() -> *const char { invoke!(0x05A5F662AD35C573) }
}
pub mod DECORATOR {
	use std::ffi::CStr;
	use crate::core::scripthook::native::*;
	use crate::core::types::*;

	pub fn DECOR_SET_BOOL(entity: Entity, propertyName: & CStr, value: bool) -> bool { invoke!(0xFE26E4609B1C3772, entity, propertyName, value) }
	pub fn DECOR_SET_FLOAT(entity: Entity, propertyName: & CStr, value: f32) -> bool { invoke!(0x238F8B0C1C7FE834, entity, propertyName, value) }
	pub fn DECOR_SET_INT(entity: Entity, propertyName: & CStr, value: i32) -> bool { invoke!(0xE88F4D7F52A6090F, entity, propertyName, value) }
	pub fn _DECOR_SET_UINT8(entity: Entity, propertyName: & CStr, value: i32) -> bool { invoke!(0x4BDC83150D43772D, entity, propertyName, value) }
	pub fn DECOR_SET_STRING(entity: Entity, propertyName: & CStr, value: & CStr) -> bool { invoke!(0x0671C1A3FF7AFDFC, entity, propertyName, value) }
	pub fn DECOR_GET_BOOL(entity: Entity, propertyName: & CStr) -> bool { invoke!(0xDEF3F1B071ABB197, entity, propertyName) }
	pub fn DECOR_GET_FLOAT(entity: Entity, propertyName: & CStr) -> f32 { invoke!(0xE5FF70CD842CA9D4, entity, propertyName) }
	pub fn DECOR_GET_INT(entity: Entity, propertyName: & CStr) -> i32 { invoke!(0x44DB62727762FD9B, entity, propertyName) }
	pub fn _DECOR_GET_UINT8(entity: Entity, propertyName: & CStr) -> i32 { invoke!(0xB1682B2443F0540B, entity, propertyName) }
	pub fn DECOR_EXIST_ON(entity: Entity, propertyName: & CStr) -> bool { invoke!(0xD9D1CDBF3464DCDF, entity, propertyName) }
	pub fn DECOR_REMOVE(entity: Entity, propertyName: & CStr) -> bool { invoke!(0x2BA7F5877A088A1D, entity, propertyName) }
	pub fn DECOR_REMOVE_ALL(entity: Entity) -> bool { invoke!(0x88942780E0ADEA42, entity) }
	pub fn DECOR_REGISTER(propertyName: & CStr, type_: i32) { invoke_ignore!(0x0B253D644E3C36B3, propertyName, type_) }
	pub fn _DECOR_REGISTER_2(propertyName: & CStr, type_: i32, p2: bool) { invoke_ignore!(0x4587374F88B7F6C2, propertyName, type_, p2) }
	pub fn DECOR_IS_REGISTERED_AS_TYPE(propertyName: & CStr, type_: i32) -> bool { invoke!(0x72355278C069F272, propertyName, type_) }
}
pub mod DLC {
	use std::ffi::CStr;
	use crate::core::scripthook::native::*;
	use crate::core::types::*;

	pub fn IS_DLC_PRESENT(dlcHash: Hash) -> bool { invoke!(0x2763DC12BBE2BB6F, dlcHash) }
	pub fn GET_IS_LOADING_SCREEN_ACTIVE() -> bool { invoke!(0x71D4BF5890659B0C) }
	pub fn _GET_SPECIAL_EDITION_CORE_STATS_BONUS_ENABLED() -> bool { invoke!(0xA16B4FBA7887D7BA) }
	pub fn _GET_SPECIAL_EDITION_CASH_CAMP_BONUS_ENABLED() -> bool { invoke!(0x1DB9D61E505AE3FC) }
}
pub mod ENTITY {
	use std::ffi::CStr;
	use crate::core::scripthook::native::*;
	use crate::core::types::*;

	pub fn DOES_ENTITY_EXIST(entity: Entity) -> bool { invoke!(0xD42BD6EB2E0F1677, entity) }
	pub fn DOES_ENTITY_BELONG_TO_THIS_SCRIPT(entity: Entity, p1: bool) -> bool { invoke!(0x622B1980CBE13332, entity, p1) }
	pub fn DOES_ENTITY_HAVE_DRAWABLE(entity: Entity) -> bool { invoke!(0x20487F0DA9AF164A, entity) }
	pub fn DOES_ENTITY_HAVE_PHYSICS(entity: Entity) -> bool { invoke!(0xA512B3F1B2A0B51C, entity) }
	pub fn HAS_ENTITY_BEEN_DAMAGED_BY_ANY_OBJECT(entity: Entity) -> bool { invoke!(0x73BB763880CD23A6, entity) }
	pub fn HAS_ENTITY_BEEN_DAMAGED_BY_ANY_PED(entity: Entity) -> bool { invoke!(0x9934E9C42D52D87E, entity) }
	pub fn HAS_ENTITY_BEEN_DAMAGED_BY_ANY_VEHICLE(entity: Entity) -> bool { invoke!(0x695D7C26DE65C423, entity) }
	pub fn HAS_ENTITY_BEEN_DAMAGED_BY_ENTITY(entity1: Entity, entity2: Entity, p2: bool, p3: bool) -> bool { invoke!(0x7B6E7BEC1143AC86, entity1, entity2, p2, p3) }
	pub fn _0x3EC28DA1FFAC9DDD(entity1: Entity, entity2: Entity, p2: Any, p3: Any) -> bool { invoke!(0x3EC28DA1FFAC9DDD, entity1, entity2, p2, p3) }
	pub fn _0xAF72EC7E1B54539B(entity: Entity) -> Entity { invoke!(0xAF72EC7E1B54539B, entity) }
	pub fn HAS_ENTITY_CLEAR_LOS_TO_ENTITY(entity1: Entity, entity2: Entity, traceType: i32) -> bool { invoke!(0xFCDFF7B72D23A1AC, entity1, entity2, traceType) }
	pub fn HAS_ENTITY_CLEAR_LOS_TO_COORD(entity: Entity, x: f32, y: f32, z: f32, flags: i32) -> bool { invoke!(0x0C9DBF48C6BA6E4C, entity, x, y, z, flags) }
	pub fn HAS_ENTITY_CLEAR_LOS_TO_ENTITY_IN_FRONT(entity1: Entity, entity2: Entity, traceType: i32) -> bool { invoke!(0xE88F19660651D566, entity1, entity2, traceType) }
	pub fn HAS_ENTITY_COLLIDED_WITH_ANYTHING(entity: Entity) -> bool { invoke!(0xDF18751EC74F90FF, entity) }
	pub fn _0x6D58167F62238284(vehicle: Vehicle) -> f32 { invoke!(0x6D58167F62238284, vehicle) }
	pub fn _0xDFC2B226D56D85F6(p0: Any, p1: Any) -> f32 { invoke!(0xDFC2B226D56D85F6, p0, p1) }
	pub fn GET_ANIM_DURATION(animDict: & CStr, animName: & CStr) -> f32 { invoke!(0x9FFAF4940A54CC09, animDict, animName) }
	pub fn GET_ENTITY_ATTACHED_TO(entity: Entity) -> Entity { invoke!(0x56D713888A566481, entity) }
	pub fn GET_ENTITY_COORDS(entity: Entity, alive: bool, realCoords: bool) -> Vector3 { invoke!(0xA86D5F069399F44D, entity, alive, realCoords) }
	pub fn GET_ENTITY_FORWARD_VECTOR(entity: Entity) -> Vector3 { invoke!(0x2412D9C05BB09B97, entity) }
	pub fn GET_ENTITY_FORWARD_X(entity: Entity) -> f32 { invoke!(0xDB0954E9960F6457, entity) }
	pub fn GET_ENTITY_FORWARD_Y(entity: Entity) -> f32 { invoke!(0x9A5C073ECBDA7EE7, entity) }
	pub fn _GET_ENTITY_FORWARD_VECTOR_YX(entity: Entity) -> Vector3 { invoke!(0x935A30AA88FB1014, entity) }
	pub fn GET_ENTITY_HEADING(entity: Entity) -> f32 { invoke!(0xC230DD956E2F5507, entity) }
	pub fn GET_ENTITY_HEALTH(entity: Entity) -> i32 { invoke!(0x82368787EA73C0F7, entity) }
	pub fn _GET_ENTITY_HEALTH_FLOAT(entity: Entity) -> f32 { invoke!(0x96C638784DB4C815, entity) }
	pub fn _CHANGE_ENTITY_HEALTH(entity: Entity, amount: f32, entity2: Entity, weaponHash: Hash) -> bool { invoke!(0x835F131E7DC8F97A, entity, amount, entity2, weaponHash) }
	pub fn GET_ENTITY_MAX_HEALTH(entity: Entity, p1: bool) -> i32 { invoke!(0x15D757606D170C3C, entity, p1) }
	pub fn SET_ENTITY_MAX_HEALTH(entity: Entity, value: i32) { invoke_ignore!(0x166E7CF68597D8B5, entity, value) }
	pub fn GET_ENTITY_HEIGHT(entity: Entity, X: f32, Y: f32, Z: f32, atTop: bool, inWorldCoords: bool) -> f32 { invoke!(0x296DEBC84474B375, entity, X, Y, Z, atTop, inWorldCoords) }
	pub fn GET_ENTITY_HEIGHT_ABOVE_GROUND(entity: Entity) -> f32 { invoke!(0x0D3B5BAEA08F63E9, entity) }
	pub fn _GET_ENTITY_WORLD_POSITION_OF_DIMENSIONS(entity: Entity, minimum: &mut Vector3, maximum: &mut Vector3) { invoke_ignore!(0xF3FDA9A617A15145, entity, minimum, maximum) }
	pub fn GET_ENTITY_MATRIX(entity: Entity, rightVector: &mut Vector3, forwardVector: &mut Vector3, upVector: &mut Vector3, position: &mut Vector3) { invoke_ignore!(0x3A9B1120AF13FBF2, entity, rightVector, forwardVector, upVector, position) }
	pub fn GET_ENTITY_MODEL(entity: Entity) -> Hash { invoke!(0xDA76A9F39210D365, entity) }
	pub fn _GET_PED_ANIMAL_TYPE(ped: Ped) -> Hash { invoke!(0x964000D355219FC0, ped) }
	pub fn GET_IS_ANIMAL(entity: Entity) -> bool { invoke!(0x9A100F1CF4546629, entity) }
	pub fn _GET_IS_BIRD(entity: Entity) -> bool { invoke!(0xC346A546612C49A9, entity) }
	pub fn _GET_IS_PREDATOR(entity: Entity) -> bool { invoke!(0x5594AFE9DE0C01B7, entity) }
	pub fn GET_OFFSET_FROM_ENTITY_GIVEN_WORLD_COORDS(entity: Entity, posX: f32, posY: f32, posZ: f32) -> Vector3 { invoke!(0x497C6B1A2C9AE69C, entity, posX, posY, posZ) }
	pub fn GET_OFFSET_FROM_ENTITY_IN_WORLD_COORDS(entity: Entity, offsetX: f32, offsetY: f32, offsetZ: f32) -> Vector3 { invoke!(0x1899F328B0E12848, entity, offsetX, offsetY, offsetZ) }
	pub fn GET_ENTITY_PITCH(entity: Entity) -> f32 { invoke!(0xEF355ABEFF7F5005, entity) }
	pub fn GET_ENTITY_ROLL(entity: Entity) -> f32 { invoke!(0xBF966536FA8B6879, entity) }
	pub fn GET_ENTITY_ROTATION(entity: Entity, rotationOrder: i32) -> Vector3 { invoke!(0xE09CAF86C32CB48F, entity, rotationOrder) }
	pub fn _GET_ENTITY_SCRIPT(entity: Entity, argStruct: &mut Any) -> Hash { invoke!(0x2A08A32B6D49906F, entity, argStruct) }
	pub fn GET_ENTITY_SPEED(entity: Entity) -> f32 { invoke!(0xFB6BA510A533DF81, entity) }
	pub fn GET_ENTITY_SPEED_VECTOR(entity: Entity, relative: bool) -> Vector3 { invoke!(0xF2DB09816A419DC5, entity, relative) }
	pub fn GET_ENTITY_UPRIGHT_VALUE(entity: Entity) -> f32 { invoke!(0x56398BE65160C3BE, entity) }
	pub fn GET_ENTITY_VELOCITY(entity: Entity, p1: i32) -> Vector3 { invoke!(0x4805D2B1D8CF94A9, entity, p1) }
	pub fn GET_OBJECT_INDEX_FROM_ENTITY_INDEX(entity: Entity) -> Object { invoke!(0x280BBE5601EAA983, entity) }
	pub fn GET_PED_INDEX_FROM_ENTITY_INDEX(entity: Entity) -> Ped { invoke!(0x0F16D042BD640EA3, entity) }
	pub fn GET_VEHICLE_INDEX_FROM_ENTITY_INDEX(entity: Entity) -> Vehicle { invoke!(0xDF1E5AAC561AFC59, entity) }
	pub fn GET_WORLD_POSITION_OF_ENTITY_BONE(entity: Entity, boneIndex: i32) -> Vector3 { invoke!(0x82CFA50E34681CA5, entity, boneIndex) }
	pub fn _0x5E214112806591EA(entity: Entity, boneIndex: i32) -> Vector3 { invoke!(0x5E214112806591EA, entity, boneIndex) }
	pub fn _0x3AB3A77672F6473F(p0: Any, p1: Any, p2: Any, p3: Any) -> Vector3 { invoke!(0x3AB3A77672F6473F, p0, p1, p2, p3) }
	pub fn GET_NEAREST_PLAYER_TO_ENTITY(entity: Entity, playerPedToIgnore: Ped, flags: i32) -> Player { invoke!(0x990E294FC387FB88, entity, playerPedToIgnore, flags) }
	pub fn GET_NEAREST_PLAYER_TO_ENTITY_ON_TEAM(entity: Entity, team: i32, playerPedToIgnore: Ped, flags: i32) -> Player { invoke!(0xB2C30C3B4AFF718C, entity, team, playerPedToIgnore, flags) }
	pub fn GET_NEAREST_PARTICIPANT_TO_ENTITY(entity: Entity) -> Player { invoke!(0x6888A43C35A5F630, entity) }
	pub fn PLACE_ENTITY_ON_GROUND_PROPERLY(entity: Entity, p1: bool) -> bool { invoke!(0x9587913B9E772D29, entity, p1) }
	pub fn GET_ENTITY_TYPE(entity: Entity) -> i32 { invoke!(0x97F696ACA466B4E0, entity) }
	pub fn GET_ENTITY_POPULATION_TYPE(entity: Entity) -> i32 { invoke!(0xADE28862B6D7B85B, entity) }
	pub fn IS_AN_ENTITY(handle: ScrHandle) -> bool { invoke!(0x27CFF3E5A286D3DF, handle) }
	pub fn IS_ENTITY_A_PED(entity: Entity) -> bool { invoke!(0xCF8176912DDA4EA5, entity) }
	pub fn IS_ENTITY_A_MISSION_ENTITY(entity: Entity) -> bool { invoke!(0x138190F64DB4BBD1, entity) }
	pub fn IS_ENTITY_A_VEHICLE(entity: Entity) -> bool { invoke!(0xC3D96AF45FCCEC4C, entity) }
	pub fn IS_ENTITY_AN_OBJECT(entity: Entity) -> bool { invoke!(0x0A27A546A375FDEF, entity) }
	pub fn IS_ENTITY_AT_COORD(entity: Entity, xPos: f32, yPos: f32, zPos: f32, xSize: f32, ySize: f32, zSize: f32, p7: bool, p8: bool, p9: i32) -> bool { invoke!(0x5E58342602E94718, entity, xPos, yPos, zPos, xSize, ySize, zSize, p7, p8, p9) }
	pub fn IS_ENTITY_AT_ENTITY(entity1: Entity, entity2: Entity, xSize: f32, ySize: f32, zSize: f32, p5: bool, p6: bool, p7: i32) -> bool { invoke!(0xC057F02B837A27F6, entity1, entity2, xSize, ySize, zSize, p5, p6, p7) }
	pub fn IS_ENTITY_ATTACHED(entity: Entity) -> bool { invoke!(0xEE6AD63ABF59C0B7, entity) }
	pub fn IS_ENTITY_ATTACHED_TO_ANY_OBJECT(entity: Entity) -> bool { invoke!(0x306C1F6178F01AB3, entity) }
	pub fn IS_ENTITY_ATTACHED_TO_ANY_PED(entity: Entity) -> bool { invoke!(0xC841153DED2CA89A, entity) }
	pub fn IS_ENTITY_ATTACHED_TO_ANY_VEHICLE(entity: Entity) -> bool { invoke!(0x12DF6E0D2E736749, entity) }
	pub fn IS_ENTITY_ATTACHED_TO_ENTITY(from: Entity, to: Entity) -> bool { invoke!(0x154A3C529497053E, from, to) }
	pub fn _IS_ENTITY_OWNED_BY_PERSISTENCE_SYSTEM(entity: Entity) -> bool { invoke!(0xA7E51B53309EAC97, entity) }
	pub fn IS_ENTITY_DEAD(entity: Entity) -> bool { invoke!(0x7D5B1F88E7504BBA, entity) }
	pub fn IS_ENTITY_IN_AIR(entity: Entity, p1: Any) -> bool { invoke!(0x886E37EC497200B6, entity, p1) }
	pub fn IS_ENTITY_IN_ANGLED_AREA(entity: Entity, originX: f32, originY: f32, originZ: f32, edgeX: f32, edgeY: f32, edgeZ: f32, angle: f32, p8: bool, p9: bool, p10: Any) -> bool { invoke!(0xD3151E53134595E5, entity, originX, originY, originZ, edgeX, edgeY, edgeZ, angle, p8, p9, p10) }
	pub fn IS_ENTITY_IN_AREA(entity: Entity, x1: f32, y1: f32, z1: f32, x2: f32, y2: f32, z2: f32, p7: bool, p8: bool, p9: Any) -> bool { invoke!(0x0C2634C40A16193E, entity, x1, y1, z1, x2, y2, z2, p7, p8, p9) }
	pub fn IS_ENTITY_IN_VOLUME(entity: Entity, volume: ScrHandle, p2: bool, p3: i32) -> bool { invoke!(0x5A5526BC09C06623, entity, volume, p2, p3) }
	pub fn IS_ENTITY_IN_WATER(entity: Entity) -> bool { invoke!(0xDDE5C125AC446723, entity) }
	pub fn _IS_ENTITY_UNDERWATER(entity: Entity, p1: bool) -> bool { invoke!(0xD4E5C1E93C466127, entity, p1) }
	pub fn GET_ENTITY_SUBMERGED_LEVEL(entity: Entity) -> f32 { invoke!(0x4A77C3F73FD9E831, entity) }
	pub fn SET_ENTITY_REQUIRES_MORE_EXPENSIVE_RIVER_CHECK(entity: Entity, toggle: bool) { invoke_ignore!(0x850C940EE3E7B8B5, entity, toggle) }
	pub fn _0x7A49D40DE437BC8D(p0: Any, p1: Any) { invoke_ignore!(0x7A49D40DE437BC8D, p0, p1) }
	pub fn _0x978AA2323ED32209(p0: Any, p1: Any) { invoke_ignore!(0x978AA2323ED32209, p0, p1) }
	pub fn _0x002AAC783ED323ED(p0: Any, p1: Any) { invoke_ignore!(0x002AAC783ED323ED, p0, p1) }
	pub fn _0x007AAC783ED323ED(p0: Any, p1: Any, p2: Any) { invoke_ignore!(0x007AAC783ED323ED, p0, p1, p2) }
	pub fn IS_ENTITY_ON_SCREEN(entity: Entity) -> bool { invoke!(0x613C15D5D8DB781F, entity) }
	pub fn IS_ENTITY_PLAYING_ANIM(entity: Entity, animDict: & CStr, animName: & CStr, animType: i32) -> bool { invoke!(0xDEE49D5CA6C49148, entity, animDict, animName, animType) }
	pub fn _IS_ENTITY_PLAYING_ANY_ANIM(entity: Entity, p1: i32) -> bool { invoke!(0x0B7CB1300CBFE19C, entity, p1) }
	pub fn IS_ENTITY_STATIC(entity: Entity) -> bool { invoke!(0x86468ADFA0F6B861, entity) }
	pub fn IS_ENTITY_TOUCHING_ENTITY(entity: Entity, targetEntity: Entity) -> bool { invoke!(0x9A2304A64C3C8423, entity, targetEntity) }
	pub fn IS_ENTITY_TOUCHING_MODEL(entity: Entity, modelHash: Hash) -> bool { invoke!(0x2AE3EBC8DEB9768B, entity, modelHash) }
	pub fn IS_ENTITY_UPRIGHT(entity: Entity, angle: f32) -> bool { invoke!(0xF6F6AFD8D4FB2658, entity, angle) }
	pub fn IS_ENTITY_UPSIDEDOWN(entity: Entity) -> bool { invoke!(0x109DE3DA41AAD94A, entity) }
	pub fn IS_ENTITY_VISIBLE(entity: Entity) -> bool { invoke!(0xFFC96ECB7FA404CA, entity) }
	pub fn _0xFF9965C47FA404DA(entity: Entity, toggle: bool) { invoke_ignore!(0xFF9965C47FA404DA, entity, toggle) }
	pub fn IS_ENTITY_VISIBLE_TO_SCRIPT(entity: Entity) -> bool { invoke!(0xF213C724E77F321A, entity) }
	pub fn _0x3F08C6163A4AB1D6(p0: Any) { invoke_ignore!(0x3F08C6163A4AB1D6, p0) }
	pub fn _0x0DB41D59E0F1502B(p0: Any) { invoke_ignore!(0x0DB41D59E0F1502B, p0) }
	pub fn _IS_TRACKED_ENTITY_VISIBLE(entity: Entity) -> bool { invoke!(0xC8CCDB712FBCBA92, entity) }
	pub fn IS_ENTITY_OCCLUDED(entity: Entity) -> bool { invoke!(0x140188E884645624, entity) }
	pub fn WOULD_ENTITY_BE_OCCLUDED(entityModelHash: Hash, x: f32, y: f32, z: f32, p4: bool) -> bool { invoke!(0x3546FAB293FF2981, entityModelHash, x, y, z, p4) }
	pub fn IS_ENTITY_WAITING_FOR_WORLD_COLLISION(entity: Entity) -> bool { invoke!(0x5E1CC2E8DC3111DD, entity) }
	pub fn _IS_ENTITY_ON_TRAIN_TRACK(entity: Entity) -> bool { invoke!(0x857ACB0AB4BD0D55, entity) }
	pub fn _0xCDB682BB47C02F0A(entity: Entity, p1: Hash) { invoke_ignore!(0xCDB682BB47C02F0A, entity, p1) }
	pub fn _0xE19035EB65AB2932(p0: Any, p1: Any) { invoke_ignore!(0xE19035EB65AB2932, p0, p1) }
	pub fn SCRIPT_OVERRIDE_ENTITY_LOOT_TABLE_PERMANENT(entity: Entity, lootTable: Hash) -> bool { invoke!(0x8C03CD6B5E0E85E8, entity, lootTable) }
	pub fn _GET_SCRIPT_OVERRIDE_ENTITY_LOOT_TABLE_PERMANENT(entity: Entity, lootTable: &mut Hash) -> bool { invoke!(0x1E804EA9B12030A4, entity, lootTable) }
	pub fn _REQUEST_ENTITY_LOOT_LIST(mount: Ped, argStruct: &mut Any, visiblelootslotrequestType: Hash, flag: i32, p4: i32, p5: bool) -> bool { invoke!(0xA88E215CEB0435C0, mount, argStruct, visiblelootslotrequestType, flag, p4, p5) }
	pub fn _0xE31FC20319874CB3(p0: Any, p1: Any, p2: Any) -> Any { invoke!(0xE31FC20319874CB3, p0, p1, p2) }
	pub fn _0x582F73ACFE969571(p0: Any, p1: Any, p2: Any) -> Any { invoke!(0x582F73ACFE969571, p0, p1, p2) }
	pub fn _0xBA2A089E60ED1163(p0: Any, p1: Any, p2: Any, p3: Any, p4: Any) -> Any { invoke!(0xBA2A089E60ED1163, p0, p1, p2, p3, p4) }
	pub fn GET_CARRIABLE_ENTITY_STATE(entity: Entity) -> i32 { invoke!(0x61914209C36EFDDB, entity) }
	pub fn _0xD46BF94C4C66FAB0(p0: Any, p1: Any, p2: Any, p3: Any) -> Any { invoke!(0xD46BF94C4C66FAB0, p0, p1, p2, p3) }
	pub fn _GET_OPTIMAL_CARRY_CONFIG(entity: Entity, index: i32) -> Hash { invoke!(0x34F008A7E48C496B, entity, index) }
	pub fn _0xD21C7418C590BB40(p0: Any) -> Any { invoke!(0xD21C7418C590BB40, p0) }
	pub fn _GET_ENTITY_CARRY_CONFIG(entity: Entity) -> Hash { invoke!(0x0FD25587BB306C86, entity) }
	pub fn _IS_CARRIABLE_MODEL(model: Hash) -> bool { invoke!(0x5AFFA9DDC87846F8, model) }
	pub fn _GET_CARRIABLE_FROM_ENTITY(entity: Entity) -> Hash { invoke!(0x31FEF6A20F00B963, entity) }
	pub fn _SET_ENTITY_CARCASS_TYPE(entity: Entity, type_: Hash) { invoke_ignore!(0x399657ED871B3A6C, entity, type_) }
	pub fn _0x2A77EF9BEC8518F4(p0: Any) -> Any { invoke!(0x2A77EF9BEC8518F4, p0) }
	pub fn _DELETE_CARRIABLE(entity: &mut Entity) { invoke_ignore!(0x0D0DB2B6AF19A987, entity) }
	pub fn _IS_ENTITY_FULLY_LOOTED(entity: Entity) -> bool { invoke!(0x8DE41E9902E85756, entity) }
	pub fn _SET_ENTITY_FULLY_LOOTED(entity: Entity, looted: bool) { invoke_ignore!(0x6BCF5F3D8FFE988D, entity, looted) }
	pub fn _0xEF2D9ED7CE684F08(ped: Ped) -> Ped { invoke!(0xEF2D9ED7CE684F08, ped) }
	pub fn _0x0CCEFC6C2C95DA2A(p0: Any, p1: Any, p2: Any, p3: Any) -> Any { invoke!(0x0CCEFC6C2C95DA2A, p0, p1, p2, p3) }
	pub fn _0x383F64263F946E45(p0: &mut i32, entity: Entity, p2: i32, ped: Ped, p4: Any, p5: i32) -> bool { invoke!(0x383F64263F946E45, p0, entity, p2, ped, p4, p5) }
	pub fn _0x8E10DF0FFA63FB65(p0: Any, p1: Any, p2: Any, p3: Any, p4: Any) -> Any { invoke!(0x8E10DF0FFA63FB65, p0, p1, p2, p3, p4) }
	pub fn _GET_ENTITY_CARRYING_FLAG(entity: Entity, flagId: i32) -> bool { invoke!(0x808077647856DE62, entity, flagId) }
	pub fn _SET_ENTITY_CARRYING_FLAG(entity: Entity, flagId: i32, value: bool) { invoke_ignore!(0x18FF3110CF47115D, entity, flagId, value) }
	pub fn _0xC3ABCFBC7D74AFA5(ped: Ped, p1: i32, p2: bool) { invoke_ignore!(0xC3ABCFBC7D74AFA5, ped, p1, p2) }
	pub fn _0x371D179701D9C082(entity: Entity) { invoke_ignore!(0x371D179701D9C082, entity) }
	pub fn _0xA48E4801DEBDF7E4(entity: Entity, p1: bool) { invoke_ignore!(0xA48E4801DEBDF7E4, entity, p1) }
	pub fn _GET_IS_CARRIABLE_PELT(entity: Entity) -> bool { invoke!(0x255B6DB4E3AD3C3E, entity) }
	pub fn _0xF59FDE7B4D31A630(p0: Any) -> Any { invoke!(0xF59FDE7B4D31A630, p0) }
	pub fn _0x120376C23F019C6C(p0: Any, p1: Any) -> Any { invoke!(0x120376C23F019C6C, p0, p1) }
	pub fn _0x5744562E973E33CD(p0: Any, p1: Any, p2: Any, p3: Any, p4: Any) -> Any { invoke!(0x5744562E973E33CD, p0, p1, p2, p3, p4) }
	pub fn _0xDD03FC2089AD093C(p0: Any, p1: Any, p2: Any, p3: Any) { invoke_ignore!(0xDD03FC2089AD093C, p0, p1, p2, p3) }
	pub fn _0xB16C780C51E51E2B(p0: Any) -> Any { invoke!(0xB16C780C51E51E2B, p0) }
	pub fn _0xEF259AA1E097E0AD(entity: Entity, p1: Any) { invoke_ignore!(0xEF259AA1E097E0AD, entity, p1) }
	pub fn _0xBD94CECFB2D65119(p0: Any, p1: Any, p2: Any, p3: Any, p4: Any, p5: Any) { invoke_ignore!(0xBD94CECFB2D65119, p0, p1, p2, p3, p4, p5) }
	pub fn _SET_ENTITY_CUSTOM_PICKUP_RADIUS(entity: Entity, radius: f32) { invoke_ignore!(0x482D17E45665DA44, entity, radius) }
	pub fn _0xE75EEA8DB59A9F39(p0: Any, p1: Any, p2: Any, p3: Any, p4: Any, p5: Any) { invoke_ignore!(0xE75EEA8DB59A9F39, p0, p1, p2, p3, p4, p5) }
	pub fn _0x188736456D1DEDE6(p0: Any, p1: Any) -> Any { invoke!(0x188736456D1DEDE6, p0, p1) }
	pub fn _0xC6A1A3D63F122DE7(p0: Any, p1: Any) { invoke_ignore!(0xC6A1A3D63F122DE7, p0, p1) }
	pub fn APPLY_FORCE_TO_ENTITY_CENTER_OF_MASS(entity: Entity, forceType: i32, x: f32, y: f32, z: f32, component: i32, isDirectionRel: bool, isForceRel: bool, p8: bool) { invoke_ignore!(0x31DA7CEC5334DB37, entity, forceType, x, y, z, component, isDirectionRel, isForceRel, p8) }
	pub fn APPLY_FORCE_TO_ENTITY(entity: Entity, forceFlags: i32, x: f32, y: f32, z: f32, offX: f32, offY: f32, offZ: f32, boneIndex: i32, isDirectionRel: bool, ignoreUpVec: bool, isForceRel: bool, p12: bool, p13: bool) { invoke_ignore!(0xF15E8F5D333F09C4, entity, forceFlags, x, y, z, offX, offY, offZ, boneIndex, isDirectionRel, ignoreUpVec, isForceRel, p12, p13) }
	pub fn ATTACH_ENTITY_TO_ENTITY(entity1: Entity, entity2: Entity, boneIndex: i32, xPos: f32, yPos: f32, zPos: f32, xRot: f32, yRot: f32, zRot: f32, p9: bool, useSoftPinning: bool, collision: bool, isPed: bool, vertexIndex: i32, fixedRot: bool, p15: bool, p16: bool) { invoke_ignore!(0x6B9BBD38AB0796DF, entity1, entity2, boneIndex, xPos, yPos, zPos, xRot, yRot, zRot, p9, useSoftPinning, collision, isPed, vertexIndex, fixedRot, p15, p16) }
	pub fn ATTACH_ENTITY_TO_ENTITY_PHYSICALLY(entity1: Entity, entity2: Entity, p2: i32, boneIndex: i32, offsetX: f32, offsetY: f32, offsetZ: f32, p7: f32, p8: f32, p9: f32, p10: f32, p11: f32, p12: f32, p13: f32, p14: bool, p15: bool, p16: bool, p17: bool, p18: i32, p19: bool, p20: f32, p21: f32) { invoke_ignore!(0xB629A43CA1643481, entity1, entity2, p2, boneIndex, offsetX, offsetY, offsetZ, p7, p8, p9, p10, p11, p12, p13, p14, p15, p16, p17, p18, p19, p20, p21) }
	pub fn _0x445D7D8EA66E373E(p0: Any, p1: Any, p2: Any, p3: Any, p4: Any, p5: Any, p6: Any, p7: Any, p8: Any, p9: Any, p10: Any, p11: Any, p12: Any, p13: Any, p14: Any, p15: Any) { invoke_ignore!(0x445D7D8EA66E373E, p0, p1, p2, p3, p4, p5, p6, p7, p8, p9, p10, p11, p12, p13, p14, p15) }
	pub fn _0x16908E859C3AB698(p0: Any, p1: Any, p2: Any, p3: Any, p4: Any) { invoke_ignore!(0x16908E859C3AB698, p0, p1, p2, p3, p4) }
	pub fn GET_ENTITY_BONE_INDEX_BY_NAME(entity: Entity, boneName: & CStr) -> i32 { invoke!(0xBACA8FE9C76C124E, entity, boneName) }
	pub fn CLEAR_ENTITY_LAST_DAMAGE_ENTITY(entity: Entity) { invoke_ignore!(0xBB19AC7D4DCEFD0F, entity) }
	pub fn DELETE_ENTITY(entity: &mut Entity) { invoke_ignore!(0x4CD38C78BD19A497, entity) }
	pub fn _DELETE_ENTITY_2(entity: &mut Entity) { invoke_ignore!(0x5E94EA09E7207C16, entity) }
	pub fn DETACH_ENTITY(entity: Entity, p1: bool, collision: bool) { invoke_ignore!(0x64CDE9D6BF8ECAD3, entity, p1, collision) }
	pub fn _IS_ENTITY_FROZEN(entity: Entity) -> bool { invoke!(0x083D497D57B7400F, entity) }
	pub fn FREEZE_ENTITY_POSITION(entity: Entity, toggle: bool) { invoke_ignore!(0x7D9EFB7AD6B19754, entity, toggle) }
	pub fn SET_ENTITY_SHOULD_FREEZE_WAITING_ON_COLLISION(entity: Entity, toggle: bool) { invoke_ignore!(0x740CB4F3F602C9F4, entity, toggle) }
	pub fn SET_ENTITY_AS_MISSION_ENTITY(entity: Entity, p1: bool, p2: bool) { invoke_ignore!(0xDC19C288082E586E, entity, p1, p2) }
	pub fn SET_ENTITY_AS_NO_LONGER_NEEDED(entity: &mut Entity) { invoke_ignore!(0x4971D2F8162B9674, entity) }
	pub fn SET_PED_AS_NO_LONGER_NEEDED(ped: &mut Ped) { invoke_ignore!(0x2595DD4236549CE3, ped) }
	pub fn SET_VEHICLE_AS_NO_LONGER_NEEDED(vehicle: &mut Vehicle) { invoke_ignore!(0x629BFA74418D6239, vehicle) }
	pub fn SET_OBJECT_AS_NO_LONGER_NEEDED(object: &mut Object) { invoke_ignore!(0x3AE22DEB5BA5A3E6, object) }
	pub fn _0x20FAEE47427A4497() { invoke_ignore!(0x20FAEE47427A4497) }
	pub fn _DOES_THREAD_OWN_THIS_ENTITY(entity: Entity) -> bool { invoke!(0x88AD6CC10D8D35B2, entity) }
	pub fn _0x56E0735D6273B227(p0: Any, p1: Any) { invoke_ignore!(0x56E0735D6273B227, p0, p1) }
	pub fn _0xC0EDEF16D90661EE(entity: Entity, p1: f32) { invoke_ignore!(0xC0EDEF16D90661EE, entity, p1) }
	pub fn _0x0FD7D7C232876E72(p0: Any) { invoke_ignore!(0x0FD7D7C232876E72, p0) }
	pub fn _0x0939E773925C4719() { invoke_ignore!(0x0939E773925C4719) }
	pub fn SET_ENTITY_CAN_BE_DAMAGED(entity: Entity, toggle: bool) { invoke_ignore!(0x0D06D522B90E861F, entity, toggle) }
	pub fn _GET_ENTITY_CAN_BE_DAMAGED(entity: Entity) -> bool { invoke!(0x75DF9E73F2F005FD, entity) }
	pub fn SET_ENTITY_CAN_BE_DAMAGED_BY_RELATIONSHIP_GROUP(entity: Entity, bCanBeDamaged: bool, relGroup: Hash) { invoke_ignore!(0x0EF1AFB18649E015, entity, bCanBeDamaged, relGroup) }
	pub fn _0xFF83AF534156B399(p0: Any, p1: Any) { invoke_ignore!(0xFF83AF534156B399, p0, p1) }
	pub fn SET_ENTITY_CAN_BE_TARGETED_WITHOUT_LOS(entity: Entity, toggle: bool) { invoke_ignore!(0x6D09F32E284D0FB7, entity, toggle) }
	pub fn GET_ENTITY_COLLISION_DISABLED(entity: Entity) -> bool { invoke!(0xAA2FADD30F45A9DA, entity) }
	pub fn SET_ENTITY_COLLISION(entity: Entity, toggle: bool, keepPhysics: bool) { invoke_ignore!(0xF66F820909453B8C, entity, toggle, keepPhysics) }
	pub fn SET_ENTITY_COMPLETELY_DISABLE_COLLISION(entity: Entity, toggle: bool, keepPhysics: bool) { invoke_ignore!(0xE0580EC84813875A, entity, toggle, keepPhysics) }
	pub fn SET_ENTITY_COORDS(entity: Entity, xPos: f32, yPos: f32, zPos: f32, xAxis: bool, yAxis: bool, zAxis: bool, clearArea: bool) { invoke_ignore!(0x06843DA7060A026B, entity, xPos, yPos, zPos, xAxis, yAxis, zAxis, clearArea) }
	pub fn SET_ENTITY_COORDS_NO_OFFSET(entity: Entity, xPos: f32, yPos: f32, zPos: f32, xAxis: bool, yAxis: bool, zAxis: bool) { invoke_ignore!(0x239A3351AC1DA385, entity, xPos, yPos, zPos, xAxis, yAxis, zAxis) }
	pub fn SET_ENTITY_DYNAMIC(entity: Entity, toggle: bool) { invoke_ignore!(0xFBFC4473F66CE344, entity, toggle) }
	pub fn SET_ENTITY_HEADING(entity: Entity, heading: f32) { invoke_ignore!(0xCF2B9C0645C4651B, entity, heading) }
	pub fn _SET_ENTITY_COORDS_AND_HEADING(entity: Entity, xPos: f32, yPos: f32, zPos: f32, heading: f32, xAxis: bool, yAxis: bool, zAxis: bool) { invoke_ignore!(0x203BEFFDBE12E96A, entity, xPos, yPos, zPos, heading, xAxis, yAxis, zAxis) }
	pub fn _SET_ENTITY_COORDS_AND_HEADING_NO_OFFSET(entity: Entity, xPos: f32, yPos: f32, zPos: f32, heading: f32, p5: bool, p6: bool) { invoke_ignore!(0x0918E3565C20F03C, entity, xPos, yPos, zPos, heading, p5, p6) }
	pub fn SET_ENTITY_HEALTH(entity: Entity, healthAmount: i32, entityKilledBy: Entity) { invoke_ignore!(0xAC2767ED8BDFAB15, entity, healthAmount, entityKilledBy) }
	pub fn SET_ENTITY_INVINCIBLE(entity: Entity, toggle: bool) { invoke_ignore!(0xA5C38736C426FCB8, entity, toggle) }
	pub fn _0xAF7F3099B9FEB535(entity: Entity, p1: f32, p2: f32, p3: f32) { invoke_ignore!(0xAF7F3099B9FEB535, entity, p1, p2, p3) }
	pub fn SET_ENTITY_IS_TARGET_PRIORITY(entity: Entity, p1: bool, p2: f32) { invoke_ignore!(0x0A5D170C44CB2189, entity, p1, p2) }
	pub fn _0xB38A29CCD5447783(p0: Any, p1: Any, p2: Any) { invoke_ignore!(0xB38A29CCD5447783, p0, p1, p2) }
	pub fn _SET_ENTITY_THREAT_TIER(entity: Entity, tier: i32, p2: bool) { invoke_ignore!(0x4B436BAC8CBE9B07, entity, tier, p2) }
	pub fn _GET_ENTITY_THREAT_TIER(entity: Entity) -> i32 { invoke!(0xE12F56CB25D9CE23, entity) }
	pub fn _0x2D40BCBFE9305DEA(p0: Any, p1: Any) { invoke_ignore!(0x2D40BCBFE9305DEA, p0, p1) }
	pub fn _SET_ENTITY_LIGHTS_ENABLED(entity: Entity, enabled: bool) { invoke_ignore!(0xEBDC12861D079ABA, entity, enabled) }
	pub fn SET_ENTITY_LOAD_COLLISION_FLAG(entity: Entity, toggle: bool) { invoke_ignore!(0x9B9EE31AED48072E, entity, toggle) }
	pub fn HAS_COLLISION_LOADED_AROUND_ENTITY(entity: Entity) -> bool { invoke!(0xBEB1600952B9CF5C, entity) }
	pub fn HAS_COLLISION_LOADED_AROUND_POSITION(xPos: f32, yPos: f32, zPos: f32) -> bool { invoke!(0x6BFBDC46139C45AB, xPos, yPos, zPos) }
	pub fn SET_ENTITY_ONLY_DAMAGED_BY_PLAYER(entity: Entity, toggle: bool) { invoke_ignore!(0x473598683095D430, entity, toggle) }
	pub fn SET_ENTITY_ONLY_DAMAGED_BY_RELATIONSHIP_GROUP(entity: Entity, p1: bool, relationshipGroup: Hash) { invoke_ignore!(0x6C1F6AA2F0ADD104, entity, p1, relationshipGroup) }
	pub fn SET_ENTITY_PROOFS(entity: Entity, proofsBitset: i32, specialFlag: bool) { invoke_ignore!(0xFAEE099C6F890BB8, entity, proofsBitset, specialFlag) }
	pub fn _GET_ENTITY_PROOFS(entity: Entity) -> i32 { invoke!(0x6CF0DAD7FA1088EA, entity) }
	pub fn SET_ENTITY_QUATERNION(entity: Entity, x: f32, y: f32, z: f32, w: f32) { invoke_ignore!(0x100E7007D13E3687, entity, x, y, z, w) }
	pub fn SET_ENTITY_ROTATION(entity: Entity, pitch: f32, roll: f32, yaw: f32, rotationOrder: i32, p5: bool) { invoke_ignore!(0x9CC8314DFEDE441E, entity, pitch, roll, yaw, rotationOrder, p5) }
	pub fn _0xD45BB89B53FC0CFD(p0: Any, p1: Any, p2: Any, p3: Any, p4: Any, p5: Any, p6: Any, p7: Any) { invoke_ignore!(0xD45BB89B53FC0CFD, p0, p1, p2, p3, p4, p5, p6, p7) }
	pub fn SET_ENTITY_VISIBLE(entity: Entity, toggle: bool) { invoke_ignore!(0x1794B4FCC84D812F, entity, toggle) }
	pub fn _0x80FDEB3A9E9AA578(entity: Entity, p1: bool) { invoke_ignore!(0x80FDEB3A9E9AA578, entity, p1) }
	pub fn _0x9C6906EF8CB20C5F(entity: Entity) { invoke_ignore!(0x9C6906EF8CB20C5F, entity) }
	pub fn SET_ENTITY_VELOCITY(entity: Entity, x: f32, y: f32, z: f32) { invoke_ignore!(0x1C99BB7B6E96D16F, entity, x, y, z) }
	pub fn SET_ENTITY_HAS_GRAVITY(entity: Entity, toggle: bool) { invoke_ignore!(0x0CEDB728A1083FA7, entity, toggle) }
	pub fn SET_ENTITY_LOD_DIST(entity: Entity, value: i32) { invoke_ignore!(0x5FB407F0A7C877BF, entity, value) }
	pub fn GET_ENTITY_LOD_DIST(entity: Entity) -> i32 { invoke!(0xDF240D0C2A948683, entity) }
	pub fn SET_ENTITY_ALPHA(entity: Entity, alphaLevel: i32, skin: bool) { invoke_ignore!(0x0DF7692B1D9E7BA7, entity, alphaLevel, skin) }
	pub fn GET_ENTITY_ALPHA(entity: Entity) -> i32 { invoke!(0x1BB501624FAF2BEA, entity) }
	pub fn RESET_ENTITY_ALPHA(entity: Entity) { invoke_ignore!(0x744B9EF44779D9AB, entity) }
	pub fn _SET_ENTITY_FADE_IN(entity: Entity) { invoke_ignore!(0xA91E6CF94404E8C9, entity) }
	pub fn SET_ENTITY_ALWAYS_PRERENDER(entity: Entity, toggle: bool) { invoke_ignore!(0xACAD101E1FB66689, entity, toggle) }
	pub fn SET_ENTITY_RENDER_SCORCHED(entity: Entity, toggle: bool) { invoke_ignore!(0x85B8A7534E44BC23, entity, toggle) }
	pub fn _0x37B01666BAE8F7EF(entity: Entity) -> Any { invoke!(0x37B01666BAE8F7EF, entity) }
	pub fn _0xA9E6D8F2DDFC4DB9(p0: Any, p1: Any) { invoke_ignore!(0xA9E6D8F2DDFC4DB9, p0, p1) }
	pub fn CREATE_MODEL_SWAP(x: f32, y: f32, z: f32, radius: f32, originalModel: Hash, newModel: Hash, p6: bool) { invoke_ignore!(0x10B2218320B6F5AC, x, y, z, radius, originalModel, newModel, p6) }
	pub fn REMOVE_MODEL_SWAP(x: f32, y: f32, z: f32, radius: f32, originalModel: Hash, newModel: Hash, p6: bool) { invoke_ignore!(0x824E1C26A14CB817, x, y, z, radius, originalModel, newModel, p6) }
	pub fn CREATE_MODEL_HIDE(x: f32, y: f32, z: f32, radius: f32, model: Hash, p5: bool) { invoke_ignore!(0x069848B3FB3C4426, x, y, z, radius, model, p5) }
	pub fn CREATE_MODEL_HIDE_EXCLUDING_SCRIPT_OBJECTS(x: f32, y: f32, z: f32, radius: f32, model: Hash, p5: bool) { invoke_ignore!(0xD136090A9AAAB17D, x, y, z, radius, model, p5) }
	pub fn REMOVE_MODEL_HIDE(p0: Any, p1: Any, p2: Any, p3: Any, p4: Any, p5: Any) { invoke_ignore!(0x3F38A98576F6213A, p0, p1, p2, p3, p4, p5) }
	pub fn _0xD4636C2EDB0DEA8A(p0: Any) -> Any { invoke!(0xD4636C2EDB0DEA8A, p0) }
	pub fn CREATE_FORCED_OBJECT(x: f32, y: f32, z: f32, p3: Any, modelHash: Hash, p5: bool) { invoke_ignore!(0x0961A905AFBC34C7, x, y, z, p3, modelHash, p5) }
	pub fn REMOVE_FORCED_OBJECT(p0: Any, p1: Any, p2: Any, p3: Any, p4: Any) { invoke_ignore!(0x553FA683F2BCD814, p0, p1, p2, p3, p4) }
	pub fn SET_ENTITY_NO_COLLISION_ENTITY(entity1: Entity, entity2: Entity, thisFrameOnly: bool) { invoke_ignore!(0xE037BF068223C38D, entity1, entity2, thisFrameOnly) }
	pub fn SET_ENTITY_MOTION_BLUR(entity: Entity, toggle: bool) { invoke_ignore!(0x516C6ABD18322B63, entity, toggle) }
	pub fn SET_CAN_AUTO_VAULT_ON_ENTITY(entity: Entity, toggle: bool) { invoke_ignore!(0x80646744FA88F9D7, entity, toggle) }
	pub fn SET_CAN_CLIMB_ON_ENTITY(entity: Entity, toggle: bool) { invoke_ignore!(0x24AED2A608F93C4C, entity, toggle) }
	pub fn SET_ENTITY_NOWEAPONDECALS(entity: Entity, toggle: bool) { invoke_ignore!(0xC64E597783BE9A1D, entity, toggle) }
	pub fn _GET_ENTITIES_NEAR_POINT(x: f32, y: f32, z: f32, radius: f32, itemSet: ItemSet, p5: i32) -> i32 { invoke!(0x59B57C4B06531E1E, x, y, z, radius, itemSet, p5) }
	pub fn GET_MATCHING_ENTITIES(volume: Volume, itemSet: ItemSet, entityType: i32, p3: Any, p4: Hash, p5: & CStr) -> i32 { invoke!(0x84CCF9A12942C83D, volume, itemSet, entityType, p3, p4, p5) }
	pub fn _GET_ENTITIES_IN_VOLUME(volume: Volume, itemSet: ItemSet, entityType: i32) -> i32 { invoke!(0x886171A12F400B89, volume, itemSet, entityType) }
	pub fn _SEARCH_BUILDING_POOL_FOR_ENTITY_WITH_THIS_MODEL(modelHash: Hash) -> Entity { invoke!(0x66B2B83B94B22458, modelHash) }
	pub fn _0xC2E71D7E0A7B4C89(p0: Any) -> Any { invoke!(0xC2E71D7E0A7B4C89, p0) }
	pub fn _0x6C31B06E91518269(p0: Any, p1: Any) { invoke_ignore!(0x6C31B06E91518269, p0, p1) }
	pub fn _0x119A5714578F4E05(p0: Any, p1: Any) { invoke_ignore!(0x119A5714578F4E05, p0, p1) }
	pub fn _GET_ENTITY_BY_DOORHASH(doorHash: Hash, p1: i32) -> Entity { invoke!(0xF7424890E4A094C0, doorHash, p1) }
	pub fn FIND_ANIM_EVENT_PHASE(animDictionary: & CStr, animName: & CStr, p2: & CStr, p3: &mut Any, p4: &mut Any) -> bool { invoke!(0x42718CC559BD7776, animDictionary, animName, p2, p3, p4) }
	pub fn FORCE_ENTITY_AI_AND_ANIMATION_UPDATE(entity: Entity, p1: bool) { invoke_ignore!(0x4C9E96473D4F1A88, entity, p1) }
	pub fn _GET_ENTITY_ANIM_CURRENT_TIME(entity: Entity, animDict: & CStr, animName: & CStr) -> f32 { invoke!(0x627520389E288A73, entity, animDict, animName) }
	pub fn _0x8E46E18AA828334F(entity: Entity, animDict: & CStr, animClip: & CStr) -> f32 { invoke!(0x8E46E18AA828334F, entity, animDict, animClip) }
	pub fn _0xDF8E49EA89A01DB1(p0: Any, p1: Any, p2: Any) -> Any { invoke!(0xDF8E49EA89A01DB1, p0, p1, p2) }
	pub fn HAS_ANIM_EVENT_FIRED(entity: Entity, actionHash: Hash) -> bool { invoke!(0x5851CC48405F4A07, entity, actionHash) }
	pub fn HAS_ENTITY_ANIM_FINISHED(entity: Entity, animDict: & CStr, animName: & CStr, p3: i32) -> bool { invoke!(0xAEB40615337EF1E3, entity, animDict, animName, p3) }
	pub fn PLAY_ENTITY_ANIM(entity: Entity, animName: & CStr, animDict: & CStr, p3: f32, loop_: bool, stayInAnim: bool, p6: bool, delta: f32, bitset: Any) -> bool { invoke!(0xDC6D22FAB76D4874, entity, animName, animDict, p3, loop_, stayInAnim, p6, delta, bitset) }
	pub fn _SET_ENTITY_ANIM_CURRENT_TIME(entity: Entity, animDict: & CStr, animName: & CStr, time: f32) { invoke_ignore!(0x11CDABDC7783B2BC, entity, animDict, animName, time) }
	pub fn _SET_ENTITY_ANIM_SPEED(entity: Entity, animDict: & CStr, animName: & CStr, speedMultiplier: f32) { invoke_ignore!(0xEAA885BA3CEA4E4A, entity, animDict, animName, speedMultiplier) }
	pub fn STOP_ENTITY_ANIM(entity: Entity, animation: & CStr, animGroup: & CStr, p3: f32) -> Any { invoke!(0x786591D986DE9159, entity, animation, animGroup, p3) }
	pub fn _0x669655FFB29EF1A9(p0: Any, p1: Any, p2: Any, p3: Any) { invoke_ignore!(0x669655FFB29EF1A9, p0, p1, p2, p3) }
	pub fn _ADD_ENTITY_TRACKING_TRAILS(entity: Entity) { invoke_ignore!(0x1AD922AB5038DEF3, entity) }
	pub fn _0x5826EFD6D73C4DE5(entity: Entity) { invoke_ignore!(0x5826EFD6D73C4DE5, entity) }
	pub fn _PAUSE_ENTITY_TRACKING(entity: Entity, pause: bool) { invoke_ignore!(0x36EB4D34D4A092C5, entity, pause) }
	pub fn _CREATE_FOOTPATH_TRAIL(p0: Any, waypointRecord: & CStr, bUseSnowOffset: bool, p3: f32, p4: f32, p5: Any, p6: Any, p7: Any, p8: Any, p9: Any, p10: Any, bInit: bool) -> Any { invoke!(0x29BA9F78321E5A6C, p0, waypointRecord, bUseSnowOffset, p3, p4, p5, p6, p7, p8, p9, p10, bInit) }
	pub fn _0xC76E94A78127412B(p0: Any, p1: Any, p2: Any) { invoke_ignore!(0xC76E94A78127412B, p0, p1, p2) }
	pub fn _0x7F20092547B4DDEA(p0: Any) { invoke_ignore!(0x7F20092547B4DDEA, p0) }
	pub fn _0xF41E2979D5BC5370(p0: Any) { invoke_ignore!(0xF41E2979D5BC5370, p0) }
	pub fn _0xAAACB74442C1BED3(p0: Any) -> Any { invoke!(0xAAACB74442C1BED3, p0) }
	pub fn PIN_CLOSEST_MAP_ENTITY(modelHash: Hash, x: f32, y: f32, z: f32, flags: i32) -> Any { invoke!(0x6F3068258A499E52, modelHash, x, y, z, flags) }
	pub fn _UNPIN_MAP_ENTITY(entity: Entity) { invoke_ignore!(0xD2B9C78537ED5759, entity) }
	pub fn IS_MAP_ENTITY_PINNED(p0: Any) -> bool { invoke!(0x1FF441D7954F8709, p0) }
	pub fn _GET_PINNED_MAP_ENTITY(p0: Any) -> Entity { invoke!(0x4735E2A4BB83D9DA, p0) }
	pub fn _0xEAB3D91D30A344F1(p0: Any) { invoke_ignore!(0xEAB3D91D30A344F1, p0) }
	pub fn _0x37CEB637BA3B1A47(p0: Any) { invoke_ignore!(0x37CEB637BA3B1A47, p0) }
	pub fn _0x350E9211074955AF(p0: Any, p1: Any) -> Any { invoke!(0x350E9211074955AF, p0, p1) }
	pub fn _0x898586729DB5221D(ped: Ped) { invoke_ignore!(0x898586729DB5221D, ped) }
	pub fn _0xE9E7A0BAC7F57746(p0: Any, p1: Any) { invoke_ignore!(0xE9E7A0BAC7F57746, p0, p1) }
}
pub mod EVENT {
	use std::ffi::CStr;
	use crate::core::scripthook::native::*;
	use crate::core::types::*;

	pub fn SET_DECISION_MAKER(ped: Ped, name: Hash) { invoke_ignore!(0x8AE2F981CDDB8FA4, ped, name) }
	pub fn SET_DECISION_MAKER_TO_DEFAULT(ped: Ped) { invoke_ignore!(0x6B9C5C38838FB6E6, ped) }
	pub fn _CREATE_SHOCKING_EVENT(args: &mut Any) -> ScrHandle { invoke!(0xCA1315C33B9A2847, args) }
	pub fn ADD_SHOCKING_EVENT_AT_POSITION(eventType: Hash, x: f32, y: f32, z: f32, p4: f32, p5: f32, p6: f32, p7: f32, p8: f32, p9: i32, p10: i32) -> ScrHandle { invoke!(0xD9F8455409B525E9, eventType, x, y, z, p4, p5, p6, p7, p8, p9, p10) }
	pub fn ADD_SHOCKING_EVENT_FOR_ENTITY(eventType: Hash, entity: Entity, p2: f32, p3: f32, p4: f32, p5: f32, p6: f32, p7: f32, p8: bool, p9: bool, p10: i32, p11: i32) -> ScrHandle { invoke!(0x7FD8F3BE76F89422, eventType, entity, p2, p3, p4, p5, p6, p7, p8, p9, p10, p11) }
	pub fn IS_SHOCKING_EVENT_IN_SPHERE(eventType: Hash, x: f32, y: f32, z: f32, radius: f32) -> bool { invoke!(0x9DB47E16060D6354, eventType, x, y, z, radius) }
	pub fn REMOVE_SHOCKING_EVENT(event: ScrHandle) -> bool { invoke!(0xE8BB3CC253A34559, event) }
	pub fn REMOVE_ALL_SHOCKING_EVENTS(p0: bool) { invoke_ignore!(0xD47A168C2AB90DC4, p0) }
	pub fn _REMOVE_ALL_SHOCKING_EVENTS_IN_AREA(x: f32, y: f32, z: f32, radius: f32, p4: bool) { invoke_ignore!(0xB4C71BA9CAB097BD, x, y, z, radius, p4) }
	pub fn _REMOVE_ALL_SHOCKING_EVENTS_OF_TYPE_IN_AREA(eventType: Hash, x: f32, y: f32, z: f32, radius: f32, p5: bool) { invoke_ignore!(0x6A648D42BF271DC7, eventType, x, y, z, radius, p5) }
	pub fn REMOVE_ALL_SHOCKING_EVENTS_OF_TYPE(eventType: Hash, p1: bool) { invoke_ignore!(0x118873DD538490B4, eventType, p1) }
	pub fn REMOVE_SHOCKING_EVENT_SPAWN_BLOCKING_AREAS() { invoke_ignore!(0xDB249021652420C5) }
	pub fn _0x36D0F2BA2C0D9BDE(entity: Entity, p1: i32) -> Any { invoke!(0x36D0F2BA2C0D9BDE, entity, p1) }
	pub fn _0x7C511E91738A0828(ped1: Ped, ped2: Ped, p2: i32, p3: Hash) { invoke_ignore!(0x7C511E91738A0828, ped1, ped2, p2, p3) }
	pub fn SUPPRESS_SHOCKING_EVENTS_NEXT_FRAME() { invoke_ignore!(0x84994FAD4E4E4E69) }
	pub fn _0xB6F4825153920582() { invoke_ignore!(0xB6F4825153920582) }
	pub fn _0x4B2B1A891D437CA7(p0: f32) { invoke_ignore!(0x4B2B1A891D437CA7, p0) }
	pub fn _0x9520175B35E2268D(ped: Ped, p1: bool) { invoke_ignore!(0x9520175B35E2268D, ped, p1) }
	pub fn _0x18E93EBFC1FCFA48(volume: Volume, p1: bool, p2: bool) -> Any { invoke!(0x18E93EBFC1FCFA48, volume, p1, p2) }
	pub fn _0x56B3410626A473E7(p0: Any) { invoke_ignore!(0x56B3410626A473E7, p0) }
	pub fn _ADD_MODEL_TO_EVENT_MONITOR(model: Hash, p1: bool, p2: bool) { invoke_ignore!(0x608AD36A644A97FE, model, p1, p2) }
	pub fn _0x4465C3D1475BD3FD(model: Hash) { invoke_ignore!(0x4465C3D1475BD3FD, model) }
	pub fn _0x2DD42FAD06E6F19E(object: Object, p1: bool, p2: bool) -> Any { invoke!(0x2DD42FAD06E6F19E, object, p1, p2) }
	pub fn _0xA86B0EE9B39D15D6(object: Object) { invoke_ignore!(0xA86B0EE9B39D15D6, object) }
	pub fn _0x26054EB81AC0893B(object: Object) -> bool { invoke!(0x26054EB81AC0893B, object) }
	pub fn _SET_EVENT_TRACKER_FOR_PED(ped: Ped, eventName: & CStr, p2: i32) { invoke_ignore!(0xBB1E41DD3D3C6250, ped, eventName, p2) }
	pub fn _0xAD17A18215DD23D6(entity: Entity, p1: i32, p2: i32) -> i32 { invoke!(0xAD17A18215DD23D6, entity, p1, p2) }
	pub fn _EVENT_GET_TIME_SINCE_EVENT(entity: Entity, eventType: Hash, p2: i32, p3: i32) -> i32 { invoke!(0xC6A7DC546E94FED5, entity, eventType, p2, p3) }
	pub fn _EVENT_GET_RECENT_EVENT(entity: Entity, p1: i32, p2: i32) -> Hash { invoke!(0x796EECFF0C6D39BE, entity, p1, p2) }
	pub fn _IS_EVENT_TRACKER_ACTIVE(eventName: & CStr, shockingEvent: Hash) -> bool { invoke!(0x797B3D4D92E56094, eventName, shockingEvent) }
	pub fn _EVENT_GET_SOURCE_ENTITY_FROM_EVENT(entity: Entity, eventType: Hash, p2: i32, p3: i32) -> Entity { invoke!(0x822A001BCEA5BD81, entity, eventType, p2, p3) }
	pub fn _EVENT_GET_TARGET_ENTITY_FROM_EVENT(entity: Entity, eventType: Hash, p2: i32, p3: i32) -> Entity { invoke!(0x38497F139981C5C9, entity, eventType, p2, p3) }
	pub fn _0x1D1B448D719415AB(ped: Ped) -> Any { invoke!(0x1D1B448D719415AB, ped) }
	pub fn _0x83D43F0FD5276E4D(entity: Entity, p1: i32) -> Any { invoke!(0x83D43F0FD5276E4D, entity, p1) }
	pub fn _0xE28D7FC9FD32ABEB(entity: Entity, eventType: Hash, p2: i32) { invoke_ignore!(0xE28D7FC9FD32ABEB, entity, eventType, p2) }
	pub fn _0x1A5C5D350068A673(ped: Ped, p1: i32) { invoke_ignore!(0x1A5C5D350068A673, ped, p1) }
	pub fn _EVENT_FLUSH_ALL_EVENT_TRACKERS(ped: Ped) { invoke_ignore!(0xAD8F2424C6E1E3A8, ped) }
	pub fn _0xE2C2FBB7825FFC66() { invoke_ignore!(0xE2C2FBB7825FFC66) }
}
pub mod FIRE {
	use std::ffi::CStr;
	use crate::core::scripthook::native::*;
	use crate::core::types::*;

	pub fn START_SCRIPT_FIRE(x: f32, y: f32, z: f32, p3: i32, p4: f32, p5: bool, soundsetName: & CStr, p7: f32, p8: i32) -> FireId { invoke!(0x6B83617E04503888, x, y, z, p3, p4, p5, soundsetName, p7, p8) }
	pub fn REMOVE_SCRIPT_FIRE(fireHandle: FireId) { invoke_ignore!(0x790125C36E194069, fireHandle) }
	pub fn START_ENTITY_FIRE(p0: Any, p1: Any, p2: Any, p3: Any) { invoke_ignore!(0xC4DC7418A44D6822, p0, p1, p2, p3) }
	pub fn STOP_ENTITY_FIRE(p0: Any, p1: Any) { invoke_ignore!(0x8390751DC40C1E98, p0, p1) }
	pub fn IS_ENTITY_ON_FIRE(entity: Entity) -> bool { invoke!(0x1BD7C371CE257C3E, entity) }
	pub fn _0x754937C28271BC65(p0: Any) { invoke_ignore!(0x754937C28271BC65, p0) }
	pub fn GET_NUMBER_OF_FIRES_IN_RANGE(x: f32, y: f32, z: f32, radius: f32) -> i32 { invoke!(0xF9617BC6FAE61E08, x, y, z, radius) }
	pub fn STOP_FIRE_IN_RANGE(x: f32, y: f32, z: f32, radius: f32) { invoke_ignore!(0xDB38F247BD421708, x, y, z, radius) }
	pub fn _STOP_FIRE_IN_BOX(posX: f32, posY: f32, posZ: f32, rotX: f32, rotY: f32, rotZ: f32, scaleX: f32, scaleY: f32, scaleZ: f32) { invoke_ignore!(0xB7C7BDC375AEA9A4, posX, posY, posZ, rotX, rotY, rotZ, scaleX, scaleY, scaleZ) }
	pub fn GET_CLOSEST_FIRE_POS(outPosition: &mut Vector3, x: f32, y: f32, z: f32) -> bool { invoke!(0xB646FB657F448261, outPosition, x, y, z) }
	pub fn _0x559FC1D310813031(p0: Any, p1: Any, p2: Any, p3: Any, p4: Any, p5: Any, p6: Any, p7: Any, p8: Any, p9: Any) -> Any { invoke!(0x559FC1D310813031, p0, p1, p2, p3, p4, p5, p6, p7, p8, p9) }
	pub fn _0x41B87A6495EE13DD(p0: Any, p1: Any, p2: Any, p3: Any, p4: Any, p5: Any, p6: Any, p7: Any, p8: Any, p9: Any) -> Any { invoke!(0x41B87A6495EE13DD, p0, p1, p2, p3, p4, p5, p6, p7, p8, p9) }
	pub fn _0xA4454592DCF7C992(p0: Any) -> Any { invoke!(0xA4454592DCF7C992, p0) }
	pub fn _IS_ENTITY_CONSUMED_BY_FIRE(entity: Entity) -> bool { invoke!(0xCDC25355C0D65963, entity) }
	pub fn ADD_EXPLOSION(x: f32, y: f32, z: f32, explosionType: i32, damageScale: f32, isAudible: bool, isInvisible: bool, cameraShake: f32) { invoke_ignore!(0x7D6F58F69DA92530, x, y, z, explosionType, damageScale, isAudible, isInvisible, cameraShake) }
	pub fn ADD_OWNED_EXPLOSION(ped: Ped, x: f32, y: f32, z: f32, explosionType: i32, damageScale: f32, isAudible: bool, isInvisible: bool, cameraShake: f32) { invoke_ignore!(0xD84A917A64D4D016, ped, x, y, z, explosionType, damageScale, isAudible, isInvisible, cameraShake) }
	pub fn _0xB7DF150605EEDC9B(entity: Entity, p1: i32, x: f32, y: f32, z: f32, explosionType: i32, damageScale: f32, isAudible: bool, isInvisible: bool, cameraShake: f32) { invoke_ignore!(0xB7DF150605EEDC9B, entity, p1, x, y, z, explosionType, damageScale, isAudible, isInvisible, cameraShake) }
	pub fn ADD_EXPLOSION_WITH_USER_VFX(x: f32, y: f32, z: f32, explosionType: i32, explosionFx: Hash, damageScale: f32, isAudible: bool, isInvisible: bool, cameraShake: f32) { invoke_ignore!(0x53BA259F3A67A99E, x, y, z, explosionType, explosionFx, damageScale, isAudible, isInvisible, cameraShake) }
	pub fn _0x34AE85C7CA4857AA(p0: Any, p1: Any, p2: Any, p3: Any, p4: Any, p5: Any, p6: Any, p7: Any, p8: Any, p9: Any, p10: Any) { invoke_ignore!(0x34AE85C7CA4857AA, p0, p1, p2, p3, p4, p5, p6, p7, p8, p9, p10) }
	pub fn IS_EXPLOSION_IN_AREA(explosionType: i32, x1: f32, y1: f32, z1: f32, x2: f32, y2: f32, z2: f32) -> bool { invoke!(0x8391BA4313A25AD3, explosionType, x1, y1, z1, x2, y2, z2) }
	pub fn IS_EXPLOSION_ACTIVE_IN_AREA(explosionType: i32, x1: f32, y1: f32, z1: f32, x2: f32, y2: f32, z2: f32) -> bool { invoke!(0xD96E82AEBFFAAFF0, explosionType, x1, y1, z1, x2, y2, z2) }
	pub fn IS_EXPLOSION_IN_SPHERE(explosionType: i32, x: f32, y: f32, z: f32, radius: f32) -> bool { invoke!(0xD62DD846D82CBB90, explosionType, x, y, z, radius) }
	pub fn IS_EXPLOSION_IN_ANGLED_AREA(explosionType: i32, x1: f32, y1: f32, z1: f32, x2: f32, y2: f32, z2: f32, angle: f32) -> bool { invoke!(0x5AE661ECD18524C9, explosionType, x1, y1, z1, x2, y2, z2, angle) }
	pub fn _IS_EXPLOSION_IN_VOLUME(explosionType: i32, volume: Volume) -> bool { invoke!(0xE24822A4CFC9107A, explosionType, volume) }
	pub fn GET_OWNER_OF_EXPLOSION_IN_ANGLED_AREA(explosionType: i32, x1: f32, y1: f32, z1: f32, x2: f32, y2: f32, z2: f32, radius: f32) -> Entity { invoke!(0x8002DDAB58594D78, explosionType, x1, y1, z1, x2, y2, z2, radius) }
	pub fn _0x68F6A75FDF5A70D6(x: f32, y: f32, z: f32, p3: f32) { invoke_ignore!(0x68F6A75FDF5A70D6, x, y, z, p3) }
	pub fn _0x24DB6B9F2B719043(p0: f32) { invoke_ignore!(0x24DB6B9F2B719043, p0) }
	pub fn _IS_PED_SHOCKING_EVENT_ACTIVE(ped: Ped, p1: i32) -> bool { invoke!(0xAB7993BA61A4674F, ped, p1) }
}
pub mod FLOCK {
	use std::ffi::CStr;
	use crate::core::scripthook::native::*;
	use crate::core::types::*;

	pub fn GET_SPECIES_TUNING_FLOAT_PARAM(p0: Hash, p1: i32, p2: i32) -> f32 { invoke!(0xE108489621422F91, p0, p1, p2) }
	pub fn SET_SPECIES_TUNING_FLOAT_PARAM(p0: Hash, p1: i32, p2: i32, p3: f32) { invoke_ignore!(0x963240B6C252BA49, p0, p1, p2, p3) }
	pub fn SET_SPECIES_TUNING_BOOL_PARAM(p0: Hash, p1: i32, p2: i32, p3: bool) { invoke_ignore!(0x6D1D94C2459B42EE, p0, p1, p2, p3) }
	pub fn GET_ANIMAL_TUNING_FLOAT_PARAM(animal: Ped, index: i32) -> f32 { invoke!(0x4BC3ECFDA0297E27, animal, index) }
	pub fn SET_ANIMAL_TUNING_FLOAT_PARAM(animal: Ped, index: i32, value: f32) { invoke_ignore!(0xCBDA22C87977244F, animal, index, value) }
	pub fn RESET_ANIMAL_TUNING_FLOAT_PARAM(animal: Ped, index: i32) { invoke_ignore!(0xE776A195488FC520, animal, index) }
	pub fn GET_ANIMAL_TUNING_BOOL_PARAM(animal: Ped, index: i32) -> bool { invoke!(0x1C1993824A396603, animal, index) }
	pub fn SET_ANIMAL_TUNING_BOOL_PARAM(animal: Ped, index: i32, value: bool) { invoke_ignore!(0x9FF1E042FA597187, animal, index, value) }
	pub fn RESET_ANIMAL_TUNING_BOOL_PARAM(animal: Ped, index: i32) { invoke_ignore!(0x96AA1304D30E6BC3, animal, index) }
	pub fn _0x8049B17BEC937662(p0: Any, p1: Any, p2: Any, p3: Any, p4: Any, p5: Any, p6: Any) -> Any { invoke!(0x8049B17BEC937662, p0, p1, p2, p3, p4, p5, p6) }
	pub fn _0xE93415B3307208E5(p0: Any, p1: Any, p2: Any, p3: Any, p4: Any, p5: Any, p6: Any, p7: Any, p8: Any) -> Any { invoke!(0xE93415B3307208E5, p0, p1, p2, p3, p4, p5, p6, p7, p8) }
	pub fn _0x19870C40C7EE15BE(p0: Any, p1: Any) -> Any { invoke!(0x19870C40C7EE15BE, p0, p1) }
	pub fn _0x0816C31480764AB0(p0: Any, p1: Any, p2: Any, p3: Any) { invoke_ignore!(0x0816C31480764AB0, p0, p1, p2, p3) }
	pub fn _0xC3D581A34BC0A1F0(p0: Any, p1: Any) { invoke_ignore!(0xC3D581A34BC0A1F0, p0, p1) }
	pub fn _0xF2CCA7B68CFAB2B9(species: Hash, x1: f32, y1: f32, z1: f32, x2: f32, y2: f32, z2: f32, x3: f32, y3: f32, z3: f32, p10: f32, p11: f32, p12: f32, p13: f32) { invoke_ignore!(0xF2CCA7B68CFAB2B9, species, x1, y1, z1, x2, y2, z2, x3, y3, z3, p10, p11, p12, p13) }
	pub fn _0xFB16F08F47B83B4C(p0: Any) { invoke_ignore!(0xFB16F08F47B83B4C, p0) }
	pub fn _GET_ANIMAL_IS_WILD(ped: Ped) -> bool { invoke!(0x3B005FF0538ED2A9, ped) }
	pub fn _SET_ANIMAL_IS_WILD(ped: Ped, toggle: bool) { invoke_ignore!(0xAEB97D84CDF3C00B, ped, toggle) }
	pub fn _GET_ANIMAL_RARITY(ped: Ped) -> i32 { invoke!(0xF8B48A361DC388AE, ped) }
	pub fn _SET_ANIMAL_RARITY(ped: Ped, rarityLevel: i32) { invoke_ignore!(0x8B6F0F59B1B99801, ped, rarityLevel) }
	pub fn _0xFF1E339CE40EAAAF(p0: Any, p1: Any) { invoke_ignore!(0xFF1E339CE40EAAAF, p0, p1) }
	pub fn _IS_HERD_VALID(herdHandle: ScrHandle) -> bool { invoke!(0x8D913E493BAFE0A3, herdHandle) }
	pub fn _CREATE_HERD() -> ScrHandle { invoke!(0xCB4EF7EDAE2E16F1) }
	pub fn _0xE0961AED72642B80(p0: Any) { invoke_ignore!(0xE0961AED72642B80, p0) }
	pub fn _ADD_PED_TO_FLOCK(p0: Any, ped: Ped) { invoke_ignore!(0x933E5D31A7D13069, p0, ped) }
	pub fn _0x408D1149C5E39C1E(p0: Any, p1: Any) { invoke_ignore!(0x408D1149C5E39C1E, p0, p1) }
	pub fn _0x9E13ACC38BA8F9C3(p0: Any, p1: Any) -> Any { invoke!(0x9E13ACC38BA8F9C3, p0, p1) }
	pub fn _0x34B9C4D86DF2C2F3(p0: Any) -> Any { invoke!(0x34B9C4D86DF2C2F3, p0) }
	pub fn _0x1DA6CB02071055D5(p0: Any) -> Vector3 { invoke!(0x1DA6CB02071055D5, p0) }
	pub fn _0xC95611869E14F8AF(p0: Any, p1: Any, p2: Any, p3: Any) { invoke_ignore!(0xC95611869E14F8AF, p0, p1, p2, p3) }
	pub fn _0x09EE00B8F858E0BE(p0: Any, p1: Any, p2: Any, p3: Any, p4: Any, p5: Any, p6: Any) -> Any { invoke!(0x09EE00B8F858E0BE, p0, p1, p2, p3, p4, p5, p6) }
	pub fn _0xE36D2CB540597EF7(p0: Any, p1: Any, p2: Any, p3: Any, p4: Any, p5: Any, p6: Any, p7: Any) { invoke_ignore!(0xE36D2CB540597EF7, p0, p1, p2, p3, p4, p5, p6, p7) }
	pub fn _0xD95F04A4E73BE85E(p0: Any, p1: Any) -> Any { invoke!(0xD95F04A4E73BE85E, p0, p1) }
	pub fn _0x17E3E5C46ECCD308(p0: Any, p1: Any, p2: Any) { invoke_ignore!(0x17E3E5C46ECCD308, p0, p1, p2) }
	pub fn _0xC72CE37081DAE625(p0: Any, p1: Any, p2: Any, p3: Any) { invoke_ignore!(0xC72CE37081DAE625, p0, p1, p2, p3) }
	pub fn _0x67A43EA3F6FE0076(p0: Any) { invoke_ignore!(0x67A43EA3F6FE0076, p0) }
	pub fn _0x36486AF7DA93A464(p0: Any) -> Any { invoke!(0x36486AF7DA93A464, p0) }
	pub fn _0x6C57BEA886A20C6B(p0: Any, p1: Any) { invoke_ignore!(0x6C57BEA886A20C6B, p0, p1) }
	pub fn _0x1520626FFAFFFA8F(p0: Any, p1: Any) { invoke_ignore!(0x1520626FFAFFFA8F, p0, p1) }
	pub fn _0xFA821997794F48E7(p0: Any, p1: Any, p2: Any) { invoke_ignore!(0xFA821997794F48E7, p0, p1, p2) }
	pub fn _0xCC6B5AAFC87BFC7B(p0: Any, p1: Any, p2: Any) { invoke_ignore!(0xCC6B5AAFC87BFC7B, p0, p1, p2) }
	pub fn _0xFDB008B3BCF5992F(p0: Any, p1: Any, p2: Any) { invoke_ignore!(0xFDB008B3BCF5992F, p0, p1, p2) }
	pub fn _0x2DF3D457D86F8E57(p0: Any, p1: Any) { invoke_ignore!(0x2DF3D457D86F8E57, p0, p1) }
	pub fn _0x706B434FEFAD6A24(p0: Any) { invoke_ignore!(0x706B434FEFAD6A24, p0) }
	pub fn _0xA881F5C77A560906(p0: Any) { invoke_ignore!(0xA881F5C77A560906, p0) }
	pub fn _0x53187E563F938E76(p0: Any) -> Any { invoke!(0x53187E563F938E76, p0) }
}
pub mod GRAPHICS {
	use std::ffi::CStr;
	use crate::core::scripthook::native::*;
	use crate::core::types::*;

	pub fn DRAW_LINE(fromX: f32, fromY: f32, fromZ: f32, toX: f32, toY: f32, toZ: f32, r: i32, g: i32, b: i32, a: i32) { invoke_ignore!(0xec2ab68b,fromX,fromY, fromZ, toX, toY, toZ, r, g, b, a) }
	pub fn FREE_MEMORY_FOR_MISSION_CREATOR_PHOTO() { invoke_ignore!(0x7DFF8F94937D2659) }
	pub fn LOAD_MISSION_CREATOR_PHOTO(p0: &mut Any, p1: Any, p2: Any, p3: Any) -> bool { invoke!(0x84F0BA7462FF8D58, p0, p1, p2, p3) }
	pub fn GET_STATUS_OF_LOAD_MISSION_CREATOR_PHOTO(contentId: & CStr) -> i32 { invoke!(0xC71B50AE58D07369, contentId) }
	pub fn BEGIN_TAKE_HIGH_QUALITY_PHOTO() -> bool { invoke!(0xA15BFFC0A01B34E1) }
	pub fn GET_STATUS_OF_TAKE_HIGH_QUALITY_PHOTO() -> i32 { invoke!(0x4A3DA74C3CCB1725) }
	pub fn FREE_MEMORY_FOR_HIGH_QUALITY_PHOTO() { invoke_ignore!(0xD45547D8396F002A) }
	pub fn _SET_PHOTO_SELF_STAT(p0: bool) { invoke_ignore!(0x2705D18C11B61046, p0) }
	pub fn _SET_PHOTO_STUDIO_STAT(p0: i32) { invoke_ignore!(0x8E6AFF353C09652E, p0) }
	pub fn _SET_POSSE_ID_FOR_PHOTO(posseId: Any) { invoke_ignore!(0x564837D4A9EDE296, posseId) }
	pub fn _0x9F6D859C80708B26(p0: bool, p1: bool) { invoke_ignore!(0x9F6D859C80708B26, p0, p1) }
	pub fn IS_PHOTO_FRAME() -> bool { invoke!(0x86076AE35CBBE55F) }
	pub fn _SET_PHOTO_IN_PHOTOMODE_STAT(p0: bool) { invoke_ignore!(0xFA91736933AB3D93, p0) }
	pub fn _SET_PHOTO_OVERLAY_EFFECT_STAT(p0: i32) { invoke_ignore!(0x8B3296278328B5EB, p0) }
	pub fn _0x0D5B19C34068FEE7(p0: Any) { invoke_ignore!(0x0D5B19C34068FEE7, p0) }
	pub fn _SET_PLAYER_APPEAR_IN_PHOTO(player: Player) { invoke_ignore!(0x75D568607909333E, player) }
	pub fn _SET_REGION_PHOTO_TAKEN_STAT(p0: & CStr) { invoke_ignore!(0xD1031B83AC093BC7, p0) }
	pub fn _SET_DISTRICT_PHOTO_TAKEN_STAT(p0: & CStr) { invoke_ignore!(0x9937FACBBF267244, p0) }
	pub fn _SET_STATE_PHOTO_TAKEN_STAT(p0: & CStr) { invoke_ignore!(0x8952E857696B8A79, p0) }
	pub fn SAVE_HIGH_QUALITY_PHOTO(unused: i32) -> bool { invoke!(0x57639FD876B68A91, unused) }
	pub fn GET_STATUS_OF_SAVE_HIGH_QUALITY_PHOTO() -> i32 { invoke!(0xD6663EC374092383) }
	pub fn BEGIN_CREATE_LOW_QUALITY_COPY_OF_PHOTO(p0: i32) -> bool { invoke!(0x494A9874F17A7D50, p0) }
	pub fn GET_STATUS_OF_CREATE_LOW_QUALITY_COPY_OF_PHOTO(p0: Any) -> i32 { invoke!(0x13430D3D5A45F14B, p0) }
	pub fn FREE_MEMORY_FOR_LOW_QUALITY_PHOTO() { invoke_ignore!(0x614682E715ADBAAC) }
	pub fn DRAW_LOW_QUALITY_PHOTO_TO_PHONE(p0: bool, photoRotation: i32) { invoke_ignore!(0xF1142E5D64B47802, p0, photoRotation) }
	pub fn _GET_MAX_NUMBER_OF_LOCAL_PHOTOS() -> i32 { invoke!(0x8E587FCD30E05592) }
	pub fn _GET_CURRENT_NUMBER_OF_LOCAL_PHOTOS() -> i32 { invoke!(0x78C56B8A7B1D000C) }
	pub fn QUEUE_OPERATION_TO_CREATE_SORTED_LIST_OF_PHOTOS() -> Any { invoke!(0xA42EDF1E88734A7E) }
	pub fn GET_STATUS_OF_SORTED_LIST_OPERATION() -> i32 { invoke!(0xB28894CD7408BD0C) }
	pub fn DRAW_LIGHT_WITH_RANGE(posX: f32, posY: f32, posZ: f32, colorR: i32, colorG: i32, colorB: i32, range: f32, intensity: f32) { invoke_ignore!(0xD2D9E04C0DF927F4, posX, posY, posZ, colorR, colorG, colorB, range, intensity) }
	pub fn UPDATE_LIGHTS_ON_ENTITY(entity: Entity) { invoke_ignore!(0xBDBACB52A03CC760, entity) }
	pub fn _SET_LIGHTS_COLOR_FOR_ENTITY(entity: Entity, red: i32, green: i32, blue: i32) { invoke_ignore!(0x6EC2A67962296F49, entity, red, green, blue) }
	pub fn _SET_LIGHTS_INTENSITY_FOR_ENTITY(entity: Entity, intensity: f32) { invoke_ignore!(0x07C0F87AAC57F2E4, entity, intensity) }
	pub fn _SET_LIGHTS_TYPE_FOR_ENTITY(entity: Entity, type_: i32) { invoke_ignore!(0xAB72C67163DC4DB4, entity, type_) }
	pub fn _DRAW_MARKER(type_: Hash, posX: f32, posY: f32, posZ: f32, dirX: f32, dirY: f32, dirZ: f32, rotX: f32, rotY: f32, rotZ: f32, scaleX: f32, scaleY: f32, scaleZ: f32, red: i32, green: i32, blue: i32, alpha: i32, bobUpAndDown: bool, faceCamera: bool, p19: i32, rotate: bool, textureDict: Option<&CStr>, textureName: Option<&CStr>, drawOnEnts: bool) { invoke_ignore!(0x2A32FAA57B937173, type_, posX, posY, posZ, dirX, dirY, dirZ, rotX, rotY, rotZ, scaleX, scaleY, scaleZ, red, green, blue, alpha, bobUpAndDown, faceCamera, p19, rotate, textureDict, textureName, drawOnEnts) }
	pub fn CREATE_CHECKPOINT_WITH_NAMEHASH(typeHash: Hash, posX1: f32, posY1: f32, posZ1: f32, posX2: f32, posY2: f32, posZ2: f32, radius: f32, red: i32, green: i32, blue: i32, alpha: i32, reserved: i32) -> i32 { invoke!(0x175668836B44CBB0, typeHash, posX1, posY1, posZ1, posX2, posY2, posZ2, radius, red, green, blue, alpha, reserved) }
	pub fn _DOES_CHECKPOINT_HAVE_FX(checkpoint: i32) -> bool { invoke!(0x4C11CCACB7C02B6E, checkpoint) }
	pub fn SET_CHECKPOINT_RGBA(checkpoint: i32, red: i32, green: i32, blue: i32, alpha: i32) { invoke_ignore!(0xCAAFC225E33B1D15, checkpoint, red, green, blue, alpha) }
	pub fn SET_CHECKPOINT_RGBA2(checkpoint: i32, red: i32, green: i32, blue: i32, alpha: i32) { invoke_ignore!(0x99AFF17222D4DEB4, checkpoint, red, green, blue, alpha) }
	pub fn _0xCC3B787E73E64160(p0: Any, p1: Any, p2: Any, p3: Any, p4: Any) { invoke_ignore!(0xCC3B787E73E64160, p0, p1, p2, p3, p4) }
	pub fn _0x171C18E994C1A395(p0: Any, p1: Any, p2: Any, p3: Any, p4: Any) { invoke_ignore!(0x171C18E994C1A395, p0, p1, p2, p3, p4) }
	pub fn DELETE_CHECKPOINT(checkpoint: i32) { invoke_ignore!(0x0DED5B0C8EBAAE12, checkpoint) }
	pub fn DRAW_RECT(x: f32, y: f32, width: f32, height: f32, red: i32, green: i32, blue: i32, alpha: i32, p8: bool, p9: bool) { invoke_ignore!(0x405224591DF02025, x, y, width, height, red, green, blue, alpha, p8, p9) }
	pub fn SET_SCRIPT_GFX_DRAW_BEHIND_PAUSEMENU(toggle: bool) { invoke_ignore!(0x906B86E6D7896B9E, toggle) }
	pub fn SET_SCRIPT_GFX_DRAW_ORDER(drawOrder: i32) { invoke_ignore!(0xCFCC78391C8B3814, drawOrder) }
	pub fn DRAW_SPRITE(textureDict: & CStr, textureName: & CStr, screenX: f32, screenY: f32, width: f32, height: f32, heading: f32, red: i32, green: i32, blue: i32, alpha: i32, p11: bool) { invoke_ignore!(0xC9884ECADE94CB34, textureDict, textureName, screenX, screenY, width, height, heading, red, green, blue, alpha, p11) }
	pub fn ATTACH_TV_AUDIO_TO_ENTITY(entity: Entity) { invoke_ignore!(0x40866A418EB8EFDE, entity) }
	pub fn SET_TV_AUDIO_FRONTEND(toggle: bool) { invoke_ignore!(0x64437C98FCC5F291, toggle) }
	pub fn GET_SCREEN_RESOLUTION(x: &mut i32, y: &mut i32) { invoke_ignore!(0x66773C92835D0909, x, y) }
	pub fn _0xA04EF43030593ABC(p0: Any, p1: Any) { invoke_ignore!(0xA04EF43030593ABC, p0, p1) }
	pub fn _0xA21AF60C9F99CCC5() { invoke_ignore!(0xA21AF60C9F99CCC5) }
	pub fn _0xC28F62AC9774FC1B() -> Any { invoke!(0xC28F62AC9774FC1B) }
	pub fn _0xEB48CE48EEC41FD4(p0: Any) { invoke_ignore!(0xEB48CE48EEC41FD4, p0) }
	pub fn GET_SCREEN_COORD_FROM_WORLD_COORD(worldX: f32, worldY: f32, worldZ: f32, screenX: &mut f32, screenY: &mut f32) -> bool { invoke!(0xCB50D7AFCC8B0EC6, worldX, worldY, worldZ, screenX, screenY) }
	pub fn _IS_TEXTURE_IN_DICT(txdHash: Hash, dict: Hash) -> bool { invoke!(0xA2A51869BDED733B, txdHash, dict) }
	pub fn SET_ARTIFICIAL_LIGHTS_STATE(state: bool) { invoke_ignore!(0xB2797619A7C7747B, state) }
	pub fn DISABLE_HDTEX_THIS_FRAME() { invoke_ignore!(0x98A7CD5EA379A854) }
	pub fn _0x1A9F09AB458D49C6(p0: bool) { invoke_ignore!(0x1A9F09AB458D49C6, p0) }
	pub fn CREATE_TRACKED_POINT() -> i32 { invoke!(0xFB405CB357C69CB9) }
	pub fn SET_TRACKED_POINT_INFO(point: i32, x: f32, y: f32, z: f32, radius: f32) { invoke_ignore!(0xF6FDA3D4404D4F2C, point, x, y, z, radius) }
	pub fn IS_TRACKED_POINT_VISIBLE(point: i32) -> bool { invoke!(0xCBB056BA159FB48D, point) }
	pub fn _0xDFE332A5DA6FE7C9(iTrackedPoint: i32) -> i32 { invoke!(0xDFE332A5DA6FE7C9, iTrackedPoint) }
	pub fn DESTROY_TRACKED_POINT(point: i32) { invoke_ignore!(0x37A59922109F8F1C, point) }
	pub fn _IS_TRACKED_POINT_VALID(point: i32) -> bool { invoke!(0xF2FDDCC8C6BAE1B3, point) }
	pub fn SET_GRASS_CULL_SPHERE(x: f32, y: f32, z: f32, p3: f32, p4: i32) -> i32 { invoke!(0x27219300C36A8D40, x, y, z, p3, p4) }
	pub fn REMOVE_GRASS_CULL_SPHERE(handle: i32) { invoke_ignore!(0xAE7BF7CA9E4BA48D, handle) }
	pub fn _ADD_VEG_MODIFIER_ZONE(volume: Volume, p1: i32, flags: i32, p3: i32) -> i32 { invoke!(0xBD3324281E8B9933, volume, p1, flags, p3) }
	pub fn ADD_VEG_MODIFIER_SPHERE(x: f32, y: f32, z: f32, radius: f32, modType: i32, flags: i32, p6: i32) -> i32 { invoke!(0xFA50F79257745E74, x, y, z, radius, modType, flags, p6) }
	pub fn REMOVE_VEG_MODIFIER_SPHERE(vegModifierHandle: i32, p1: i32) { invoke_ignore!(0x9CF1836C03FB67A2, vegModifierHandle, p1) }
	pub fn _ENABLE_STATIC_VEG_MODIFIER(p0: Hash) { invoke_ignore!(0xDFEA23EC90113657, p0) }
	pub fn _DISABLE_STATIC_VEG_MODIFIER(p0: Hash) { invoke_ignore!(0xDD0BC0EDCB2162F6, p0) }
	pub fn _IS_STATIC_VEG_MODIFIER_ENABLED(p0: Hash) -> bool { invoke!(0xDE9BAD3292AA6D5E, p0) }
	pub fn _0xEC3F7F24EEEB3BA3() { invoke_ignore!(0xEC3F7F24EEEB3BA3) }
	pub fn _0x9F158A49B0D84C3C(p0: Any) { invoke_ignore!(0x9F158A49B0D84C3C, p0) }
	pub fn _0x910E260AEAD855DE() { invoke_ignore!(0x910E260AEAD855DE) }
	pub fn _CREATE_SWATCH_TEXTURE_DICT(slots: i32) -> bool { invoke!(0x3D084D5568FB4028, slots) }
	pub fn _DESTROY_SWATCH_TEXTURE_DICT() { invoke_ignore!(0xDAD7FB8402651654) }
	pub fn _GENERATE_SWATCH_TEXTURE_DIRECTLY(slot: i32, p1: Any) { invoke_ignore!(0x646ED1A1D28487DF, slot, p1) }
	pub fn _GENERATE_SWATCH_TEXTURE(slotId: i32, componentHash: Hash, metapedType: i32, p3: bool) { invoke_ignore!(0x160921255327C591, slotId, componentHash, metapedType, p3) }
	pub fn CASCADE_SHADOWS_SET_CASCADE_BOUNDS(p0: Any, p1: bool, p2: f32, p3: f32, p4: f32, p5: f32, p6: bool, p7: f32) { invoke_ignore!(0xD9EDB2E4512D563E, p0, p1, p2, p3, p4, p5, p6, p7) }
	pub fn CASCADE_SHADOWS_ENABLE_ENTITY_TRACKER(toggle: bool) { invoke_ignore!(0x8FBFD2AEB196B369, toggle) }
	pub fn CASCADE_SHADOWS_SET_SHADOW_SAMPLE_TYPE(type_: & CStr) { invoke_ignore!(0xCE4774E0F9AD48D1, type_) }
	pub fn CASCADE_SHADOWS_CLEAR_SHADOW_SAMPLE_TYPE() { invoke_ignore!(0xF7C29D7C12C36F03) }
	pub fn _0x503941F65DBA24EC(p0: Any) { invoke_ignore!(0x503941F65DBA24EC, p0) }
	pub fn _0x815653A42C5ABE76() { invoke_ignore!(0x815653A42C5ABE76) }
	pub fn _0xFF8018C778349234(p0: Any) { invoke_ignore!(0xFF8018C778349234, p0) }
	pub fn RESET_ADAPTATION(unk: i32) { invoke_ignore!(0x297B72E2AF094742, unk) }
	pub fn TOGGLE_PAUSED_RENDERPHASES(toggle: bool) { invoke_ignore!(0xEF9E1C45732F55FA, toggle) }
	pub fn GET_TOGGLE_PAUSED_RENDERPHASES_STATUS() -> bool { invoke!(0x86ED21BDB2791CE8) }
	pub fn RESET_PAUSED_RENDERPHASES() { invoke_ignore!(0xCCD9AAD85E1B559E) }
	pub fn SET_HIDOF_OVERRIDE(p0: bool, p1: bool, p2: f32, p3: f32, p4: f32, p5: f32) { invoke_ignore!(0xCC23AA1A7CBFE840, p0, p1, p2, p3, p4, p5) }
	pub fn _0x21F00E08CBB5F37B(component: & CStr) { invoke_ignore!(0x21F00E08CBB5F37B, component) }
	pub fn _0x5AC6E0FA028369DE() { invoke_ignore!(0x5AC6E0FA028369DE) }
	pub fn _0xEC3D8C228FE553D7(p0: bool) -> bool { invoke!(0xEC3D8C228FE553D7, p0) }
	pub fn _0xF5793BB386E1FF9C(p0: Any) { invoke_ignore!(0xF5793BB386E1FF9C, p0) }
	pub fn _0x5CD6A2CCE5087161(p0: Any) { invoke_ignore!(0x5CD6A2CCE5087161, p0) }
	pub fn _0xC8D0611D9A0CF5D3(p0: Any) { invoke_ignore!(0xC8D0611D9A0CF5D3, p0) }
	pub fn _GET_PHOTO_MODE_EXPOSURE() -> f32 { invoke!(0x06C0D8BB6B04A709) }
	pub fn _0x62B9F9A1272AED80(p0: Any) { invoke_ignore!(0x62B9F9A1272AED80, p0) }
	pub fn _GET_PHOTO_MODE_CONTRAST() -> f32 { invoke!(0x98F4154989B81EC6) }
	pub fn _0x9229ED770975BD9E() { invoke_ignore!(0x9229ED770975BD9E) }
	pub fn START_PARTICLE_FX_NON_LOOPED_AT_COORD(effectName: & CStr, xPos: f32, yPos: f32, zPos: f32, xRot: f32, yRot: f32, zRot: f32, scale: f32, xAxis: bool, yAxis: bool, zAxis: bool) -> bool { invoke!(0x2E80BF72EF7C87AC, effectName, xPos, yPos, zPos, xRot, yRot, zRot, scale, xAxis, yAxis, zAxis) }
	pub fn START_NETWORKED_PARTICLE_FX_NON_LOOPED_AT_COORD(effectName: & CStr, xPos: f32, yPos: f32, zPos: f32, xRot: f32, yRot: f32, zRot: f32, scale: f32, xAxis: bool, yAxis: bool, zAxis: bool) -> bool { invoke!(0xFB97618457994A62, effectName, xPos, yPos, zPos, xRot, yRot, zRot, scale, xAxis, yAxis, zAxis) }
	pub fn START_PARTICLE_FX_NON_LOOPED_ON_PED_BONE(effectName: & CStr, ped: Ped, offsetX: f32, offsetY: f32, offsetZ: f32, rotX: f32, rotY: f32, rotZ: f32, boneIndex: i32, scale: f32, axisX: bool, axisY: bool, axisZ: bool) -> bool { invoke!(0x3FAA72BD940C3AC0, effectName, ped, offsetX, offsetY, offsetZ, rotX, rotY, rotZ, boneIndex, scale, axisX, axisY, axisZ) }
	pub fn START_PARTICLE_FX_NON_LOOPED_ON_ENTITY(effectName: & CStr, entity: Entity, offsetX: f32, offsetY: f32, offsetZ: f32, rotX: f32, rotY: f32, rotZ: f32, scale: f32, axisX: bool, axisY: bool, axisZ: bool) -> bool { invoke!(0xFF4C64C513388C12, effectName, entity, offsetX, offsetY, offsetZ, rotX, rotY, rotZ, scale, axisX, axisY, axisZ) }
	pub fn START_NETWORKED_PARTICLE_FX_NON_LOOPED_ON_ENTITY(effectName: & CStr, entity: Entity, offsetX: f32, offsetY: f32, offsetZ: f32, rotX: f32, rotY: f32, rotZ: f32, scale: f32, axisX: bool, axisY: bool, axisZ: bool) -> bool { invoke!(0xE6CFE43937061143, effectName, entity, offsetX, offsetY, offsetZ, rotX, rotY, rotZ, scale, axisX, axisY, axisZ) }
	pub fn _START_PARTICLE_FX_NON_LOOPED_ON_PED_BONE_2(effectName: & CStr, ped: Ped, offsetX: f32, offsetY: f32, offsetZ: f32, rotX: f32, rotY: f32, rotZ: f32, boneIndex: i32, scale: f32, axisX: bool, axisY: bool, axisZ: bool) -> bool { invoke!(0xC695870B8A149B96, effectName, ped, offsetX, offsetY, offsetZ, rotX, rotY, rotZ, boneIndex, scale, axisX, axisY, axisZ) }
	pub fn SET_PARTICLE_FX_NON_LOOPED_COLOUR(r: f32, g: f32, b: f32) { invoke_ignore!(0x60B85BED6577A35B, r, g, b) }
	pub fn SET_PARTICLE_FX_NON_LOOPED_ALPHA(alpha: f32) { invoke_ignore!(0xE8A35938A7026CEA, alpha) }
	pub fn _SET_PARTICLE_FX_NON_LOOPED_EMITTER_SCALE(p0: f32, p1: f32, p2: f32) { invoke_ignore!(0x56C392C2BD78B024, p0, p1, p2) }
	pub fn START_PARTICLE_FX_LOOPED_AT_COORD(effectName: & CStr, x: f32, y: f32, z: f32, xRot: f32, yRot: f32, zRot: f32, scale: f32, xAxis: bool, yAxis: bool, zAxis: bool, p11: bool) -> i32 { invoke!(0xBA32867E86125D3A, effectName, x, y, z, xRot, yRot, zRot, scale, xAxis, yAxis, zAxis, p11) }
	pub fn START_PARTICLE_FX_LOOPED_ON_PED_BONE(effectName: & CStr, ped: Ped, xOffset: f32, yOffset: f32, zOffset: f32, xRot: f32, yRot: f32, zRot: f32, boneIndex: i32, scale: f32, xAxis: bool, yAxis: bool, zAxis: bool) -> i32 { invoke!(0xE689C1B1432BB8AF, effectName, ped, xOffset, yOffset, zOffset, xRot, yRot, zRot, boneIndex, scale, xAxis, yAxis, zAxis) }
	pub fn START_PARTICLE_FX_LOOPED_ON_ENTITY(effectName: & CStr, entity: Entity, xOffset: f32, yOffset: f32, zOffset: f32, xRot: f32, yRot: f32, zRot: f32, scale: f32, xAxis: bool, yAxis: bool, zAxis: bool) -> i32 { invoke!(0xBD41E1440CE39800, effectName, entity, xOffset, yOffset, zOffset, xRot, yRot, zRot, scale, xAxis, yAxis, zAxis) }
	pub fn START_PARTICLE_FX_LOOPED_ON_ENTITY_BONE(effectName: & CStr, entity: Entity, xOffset: f32, yOffset: f32, zOffset: f32, xRot: f32, yRot: f32, zRot: f32, boneIndex: i32, scale: f32, xAxis: bool, yAxis: bool, zAxis: bool) -> i32 { invoke!(0xD3BA6EC7F2FBD5E9, effectName, entity, xOffset, yOffset, zOffset, xRot, yRot, zRot, boneIndex, scale, xAxis, yAxis, zAxis) }
	pub fn START_NETWORKED_PARTICLE_FX_LOOPED_ON_ENTITY(effectName: & CStr, entity: Entity, xOffset: f32, yOffset: f32, zOffset: f32, xRot: f32, yRot: f32, zRot: f32, scale: f32, xAxis: bool, yAxis: bool, zAxis: bool) -> i32 { invoke!(0x8F90AB32E1944BDE, effectName, entity, xOffset, yOffset, zOffset, xRot, yRot, zRot, scale, xAxis, yAxis, zAxis) }
	pub fn START_NETWORKED_PARTICLE_FX_LOOPED_ON_ENTITY_BONE(effectName: & CStr, entity: Entity, xOffset: f32, yOffset: f32, zOffset: f32, xRot: f32, yRot: f32, zRot: f32, boneIndex: i32, scale: f32, xAxis: bool, yAxis: bool, zAxis: bool) -> i32 { invoke!(0x9C56621462FFE7A6, effectName, entity, xOffset, yOffset, zOffset, xRot, yRot, zRot, boneIndex, scale, xAxis, yAxis, zAxis) }
	pub fn STOP_PARTICLE_FX_LOOPED(ptfxHandle: i32, p1: bool) { invoke_ignore!(0x22970F3A088B133B, ptfxHandle, p1) }
	pub fn DOES_PARTICLE_FX_LOOPED_EXIST(ptfxHandle: i32) -> bool { invoke!(0x9DD5AFF561E88F2A, ptfxHandle) }
	pub fn SET_PARTICLE_FX_LOOPED_OFFSETS(ptfxHandle: i32, x: f32, y: f32, z: f32, rotX: f32, rotY: f32, rotZ: f32) { invoke_ignore!(0xD3A4A95FC94FE83B, ptfxHandle, x, y, z, rotX, rotY, rotZ) }
	pub fn SET_PARTICLE_FX_LOOPED_EVOLUTION(ptfxHandle: i32, propertyName: & CStr, amount: f32, noNetwork: bool) { invoke_ignore!(0x3674F389B0FACD80, ptfxHandle, propertyName, amount, noNetwork) }
	pub fn SET_PARTICLE_FX_LOOPED_COLOUR(ptfxHandle: i32, r: f32, g: f32, b: f32, p4: bool) { invoke_ignore!(0x239879FC61C610CC, ptfxHandle, r, g, b, p4) }
	pub fn SET_PARTICLE_FX_LOOPED_ALPHA(ptfxHandle: i32, alpha: f32) { invoke_ignore!(0x88786E76234F7054, ptfxHandle, alpha) }
	pub fn SET_PARTICLE_FX_LOOPED_SCALE(ptfxHandle: i32, scale: f32) { invoke_ignore!(0x1A9E1C0D98D093B7, ptfxHandle, scale) }
	pub fn SET_PARTICLE_FX_LOOPED_FAR_CLIP_DIST(ptfxHandle: i32, range: f32) { invoke_ignore!(0x9B04D471DA0AD7AA, ptfxHandle, range) }
	pub fn _SET_PARTICLE_FX_LOOPED_UPDATE_DISTANT_SMOKE(ptfxHandle: i32, scalar: f32) { invoke_ignore!(0x9DDC222D85D5AF2A, ptfxHandle, scalar) }
	pub fn REMOVE_PARTICLE_FX(ptfxHandle: i32, p1: bool) { invoke_ignore!(0x459598F579C98929, ptfxHandle, p1) }
	pub fn REMOVE_PARTICLE_FX_FROM_ENTITY(entity: Entity) { invoke_ignore!(0x92884B4A49D81325, entity) }
	pub fn REMOVE_PARTICLE_FX_IN_RANGE(X: f32, Y: f32, Z: f32, radius: f32) { invoke_ignore!(0x87B5905ECA623B68, X, Y, Z, radius) }
	pub fn USE_PARTICLE_FX_ASSET(fxName: & CStr) { invoke_ignore!(0xA10DB07FC234DD12, fxName) }
	pub fn SET_PARTICLE_FX_OVERRIDE(oldAsset: & CStr, newAsset: & CStr) { invoke_ignore!(0xBE711A169E9C7E95, oldAsset, newAsset) }
	pub fn RESET_PARTICLE_FX_OVERRIDE(name: & CStr) { invoke_ignore!(0x274B3DABF7E72DEF, name) }
	pub fn _0x4FB67D172C4476F3(entity: Entity, p1: & CStr, p2: & CStr, p3: f32) { invoke_ignore!(0x4FB67D172C4476F3, entity, p1, p2, p3) }
	pub fn SET_PARTICLE_FX_AMBIENT_COLOUR(entity: Entity, p1: & CStr, r: f32, g: f32, b: f32) { invoke_ignore!(0x3C61B52B00848C26, entity, p1, r, g, b) }
	pub fn _0xD1472AFF30C103D6(p0: f32) { invoke_ignore!(0xD1472AFF30C103D6, p0) }
	pub fn SET_PARTICLE_FX_BULLET_IMPACT_SCALE(scale: f32) { invoke_ignore!(0xA53C8D7D0F8C74D0, scale) }
	pub fn SET_PARTICLE_FX_BULLET_IMPACT_LODRANGE_SCALE(p0: f32) { invoke_ignore!(0x8DCCC98DC0DBF9E4, p0) }
	pub fn _SET_SNIPER_GLINTS_ENABLED(enabled: bool) { invoke_ignore!(0x6E8EB45A4F4460EB, enabled) }
	pub fn SET_PARTICLE_FX_FOOT_LODRANGE_SCALE(p0: f32) { invoke_ignore!(0x2A1625858887D4E6, p0) }
	pub fn _0x4046493D2EEACA0E() { invoke_ignore!(0x4046493D2EEACA0E) }
	pub fn SET_PICKUP_LIGHT(object: Object, toggle: bool) { invoke_ignore!(0x7DFB49BCDB73089A, object, toggle) }
	pub fn _BLOCK_PICKUP_OBJECT_LIGHT(pickupObject: Object, toggle: bool) { invoke_ignore!(0x50C14328119E1DD1, pickupObject, toggle) }
	pub fn BLOCK_PICKUP_PLACEMENT_LIGHT(pickup: Pickup, toggle: bool) { invoke_ignore!(0x0552AA3FFC5B87AA, pickup, toggle) }
	pub fn ALLOW_PICKUP_LIGHT_SYNC(pickupObject: Object, allow: bool) { invoke_ignore!(0x7C348310A6E2FB91, pickupObject, allow) }
	pub fn _SET_PEARLESCENT_FX_ENABLED(object: Object, toggle: bool) { invoke_ignore!(0x72E30372E7CC4415, object, toggle) }
	pub fn REMOVE_DECALS_IN_RANGE(x: f32, y: f32, z: f32, range: f32) { invoke_ignore!(0x86DE59FA02902B40, x, y, z, range) }
	pub fn REMOVE_DECALS_FROM_OBJECT(obj: Object) { invoke_ignore!(0xFB8972BAE0013140, obj) }
	pub fn ADD_DECAL(p0: Any, p1: Any, p2: Any, p3: Any, p4: Any, p5: Any, p6: Any, p7: Any, p8: Any, p9: Any, p10: Any, p11: Any, p12: Any, p13: Any, p14: Any, p15: Any, p16: Any, p17: Any, p18: Any, p19: Any, p20: Any, p21: Any) -> i32 { invoke!(0x57CB267624EF85C0, p0, p1, p2, p3, p4, p5, p6, p7, p8, p9, p10, p11, p12, p13, p14, p15, p16, p17, p18, p19, p20, p21) }
	pub fn _ADD_BLOOD_POOL(x: f32, y: f32, z: f32, unused: bool) { invoke_ignore!(0xFA2ECC78A6014D4F, x, y, z, unused) }
	pub fn _ADD_BLOOD_POOL_2(x: f32, y: f32, z: f32, p3: f32, size: f32, p5: f32, permanent: bool, p7: f32, p8: bool) { invoke_ignore!(0xF708298675ABDC6A, x, y, z, p3, size, p5, permanent, p7, p8) }
	pub fn _ADD_BLOOD_POOLS_FOR_PED(ped: Ped) { invoke_ignore!(0xDFCE8CE9F3EBE93F, ped) }
	pub fn _ADD_BLOOD_POOLS_FOR_PED_WITH_PARAMS(ped: Ped, p1: f32, size: f32, p3: f32) { invoke_ignore!(0xC349EE1E6EFA494B, ped, p1, size, p3) }
	pub fn START_PETROL_TRAIL_DECALS(p0: Any, p1: Any) { invoke_ignore!(0x46F246D6504F0031, p0, p1) }
	pub fn ADD_PETROL_TRAIL_DECAL_INFO(x: f32, y: f32, z: f32, p3: f32) { invoke_ignore!(0x73354FB6D03D2E8A, x, y, z, p3) }
	pub fn END_PETROL_TRAIL_DECALS() { invoke_ignore!(0x0E126AAE933F3B56) }
	pub fn _0xE63D68F455CA0B47(p0: Any, p1: Any, p2: Any, p3: Any, p4: Any, p5: Any, p6: Any) -> Any { invoke!(0xE63D68F455CA0B47, p0, p1, p2, p3, p4, p5, p6) }
	pub fn REMOVE_DECAL(decal: i32) { invoke_ignore!(0x49A720552EB0BB88, decal) }
	pub fn IS_DECAL_ALIVE(decal: i32) -> bool { invoke!(0x3E4B4E5CF5D3EEB5, decal) }
	pub fn SET_DISABLE_PETROL_DECALS_IGNITING_THIS_FRAME() { invoke_ignore!(0x53ED07BF368EDA59) }
	pub fn _0xB032C085D9A03907() { invoke_ignore!(0xB032C085D9A03907) }
	pub fn _0xFB680A9B33D0EDBE(p0: bool) { invoke_ignore!(0xFB680A9B33D0EDBE, p0) }
	pub fn _0x41F88A85A579A61D(p0: f32) { invoke_ignore!(0x41F88A85A579A61D, p0) }
	pub fn _BLOOD_TRAIL_FOR_WAYPOINT(waypointRecording: & CStr, p1: f32) { invoke_ignore!(0xB9C92616929CC25D, waypointRecording, p1) }
	pub fn _ADD_BLOOD_TRAIL_POINT(x: f32, y: f32, z: f32) { invoke_ignore!(0xDD9DC1AB63D513CE, x, y, z) }
	pub fn _0x812C1563185C6FB2() { invoke_ignore!(0x812C1563185C6FB2) }
	pub fn _0x4BD66B4E3427689B(p0: & CStr) { invoke_ignore!(0x4BD66B4E3427689B, p0) }
	pub fn _ADD_BLOOD_TRAIL_SPLAT(x: f32, y: f32, z: f32) { invoke_ignore!(0xF5E45CB1CF965D2D, x, y, z) }
	pub fn _0xF2F543D48F319A3A() { invoke_ignore!(0xF2F543D48F319A3A) }
	pub fn _0x1460B644397453EB() { invoke_ignore!(0x1460B644397453EB) }
	pub fn _DISABLE_FAR_ARTIFICIAL_LIGHTS(disable: bool) { invoke_ignore!(0xCD284E2F6AC27EE9, disable) }
	pub fn _0x453D16D41FC51D3E(p0: bool) { invoke_ignore!(0x453D16D41FC51D3E, p0) }
	pub fn _0xC06F2F45A73EABCD(entity: Entity) { invoke_ignore!(0xC06F2F45A73EABCD, entity) }
	pub fn SET_TIMECYCLE_MODIFIER(modifierName: & CStr) { invoke_ignore!(0xFA08722A5EA82DA7, modifierName) }
	pub fn SET_TIMECYCLE_MODIFIER_STRENGTH(strength: f32) { invoke_ignore!(0xFDB74C9CC54C3F37, strength) }
	pub fn SET_TRANSITION_TIMECYCLE_MODIFIER(modifierName: & CStr, transitionBlend: f32) { invoke_ignore!(0xFF927A09F481D80C, modifierName, transitionBlend) }
	pub fn SET_TRANSITION_OUT_OF_TIMECYCLE_MODIFIER(strength: f32) { invoke_ignore!(0xBB6C707F20D955D4, strength) }
	pub fn CLEAR_TIMECYCLE_MODIFIER() { invoke_ignore!(0x0E3F4AF2D63491FB) }
	pub fn GET_TIMECYCLE_MODIFIER_INDEX() -> i32 { invoke!(0xA705394293E2B3D3) }
	pub fn GET_TIMECYCLE_TRANSITION_MODIFIER_INDEX() -> i32 { invoke!(0x2DA67BA3C8A6755D) }
	pub fn _0x67B0778C62E74423(p0: Any) { invoke_ignore!(0x67B0778C62E74423, p0) }
	pub fn _0x6C03118E9E5C1A14(p0: Any) { invoke_ignore!(0x6C03118E9E5C1A14, p0) }
	pub fn _GET_MODIFIED_VISIBILITY_DISTANCE() -> f32 { invoke!(0x25CA89B2A39DCC69) }
	pub fn ENABLE_MOON_CYCLE_OVERRIDE(strength: f32) { invoke_ignore!(0x6FE93BCC7BF12B63, strength) }
	pub fn SET_TV_CHANNEL(channel: i32) { invoke_ignore!(0x593FAF7FC9401A56, channel) }
	pub fn GET_TV_CHANNEL() -> i32 { invoke!(0xF90FBFD68F3C59AE) }
	pub fn SET_TV_VOLUME(volume: f32) { invoke_ignore!(0x73A97068787D7231, volume) }
	pub fn DRAW_TV_CHANNEL(xPos: f32, yPos: f32, xScale: f32, yScale: f32, rotation: f32, red: i32, green: i32, blue: i32, alpha: i32) { invoke_ignore!(0xC0A145540254A840, xPos, yPos, xScale, yScale, rotation, red, green, blue, alpha) }
	pub fn SET_TV_CHANNEL_PLAYLIST(tvChannel: i32, playlistName: & CStr, restart: bool) { invoke_ignore!(0xDEC6B25F5DC8925B, tvChannel, playlistName, restart) }
	pub fn IS_TVSHOW_CURRENTLY_PLAYING(videoCliphash: Hash) -> bool { invoke!(0x4D562223E0EB65F3, videoCliphash) }
	pub fn _0x5C674EB487891F6B() -> Any { invoke!(0x5C674EB487891F6B) }
	pub fn ENABLE_MOVIE_SUBTITLES(toggle: bool) { invoke_ignore!(0x6FC9B065229C0787, toggle) }
	pub fn _0x32DE2BFFDA43E62A() { invoke_ignore!(0x32DE2BFFDA43E62A) }
	pub fn _0xD543487A1F12828F(p0: Any, p1: Any, p2: Any, p3: Any) { invoke_ignore!(0xD543487A1F12828F, p0, p1, p2, p3) }
	pub fn _0xD9BC98B55BCFAA9B(p0: Any) { invoke_ignore!(0xD9BC98B55BCFAA9B, p0) }
	pub fn _0x48FE0DB54045B975(p0: Any, p1: Any, p2: Any, p3: Any, p4: Any, p5: Any, p6: Any) { invoke_ignore!(0x48FE0DB54045B975, p0, p1, p2, p3, p4, p5, p6) }
	pub fn _0x735762E8D7573E42(p0: Any, p1: Any, p2: Any) { invoke_ignore!(0x735762E8D7573E42, p0, p1, p2) }
	pub fn _0x981C7D863980FA51() { invoke_ignore!(0x981C7D863980FA51) }
	pub fn _ANIMPOSTFX_PRELOAD_POSTFX(effectName: & CStr) { invoke_ignore!(0x5199405EABFBD7F0, effectName) }
	pub fn _ANIMPOSTFX_HAS_LOADED(effectName: & CStr) -> bool { invoke!(0xBF2DD155B2ADCD0A, effectName) }
	pub fn _ANIMPOSTFX_SET_TO_UNLOAD(effectName: & CStr) { invoke_ignore!(0x37D7BDBA89F13959, effectName) }
	pub fn ANIMPOSTFX_PLAY(effectName: & CStr) { invoke_ignore!(0x4102732DF6B4005F, effectName) }
	pub fn _ANIMPOSTFX_PLAY_TIMED(effectName: & CStr, duration: i32) { invoke_ignore!(0x3A9A281FF71249E9, effectName, duration) }
	pub fn ANIMPOSTFX_STOP(effectName: & CStr) { invoke_ignore!(0xB4FD7446BAB2F394, effectName) }
	pub fn _0x26DD2FB0A88CC412(effectName: & CStr, effectName2: & CStr, p2: Any, p3: Any) { invoke_ignore!(0x26DD2FB0A88CC412, effectName, effectName2, p2, p3) }
	pub fn _ANIMPOSTFX_CLEAR_EFFECT(effectName: & CStr) { invoke_ignore!(0xC5CB91D65852ED7E, effectName) }
	pub fn ANIMPOSTFX_IS_RUNNING(effectName: & CStr) -> bool { invoke!(0x4A123E85D7C4CA0B, effectName) }
	pub fn _ANIMPOSTFX_IS_TAG_PLAYING(effectName: & CStr) -> bool { invoke!(0x2D4F9C852CE8A253, effectName) }
	pub fn ANIMPOSTFX_STOP_ALL() { invoke_ignore!(0x66560A0D4C64FD21) }
	pub fn _ANIMPOSTFX_STOP_TAG(effectName: & CStr) { invoke_ignore!(0xAD74C22A541AB987, effectName) }
	pub fn _ANIMPOSTFX_SET_STRENGTH(effectName: & CStr, strength: f32) { invoke_ignore!(0xCAB4DD2D5B2B7246, effectName, strength) }
	pub fn _ANIMPOSTFX_SET_POTENCY(effectName: & CStr, p1: i32, potency: f32) { invoke_ignore!(0xF972F0AB16DC5260, effectName, p1, potency) }
	pub fn _ANIMPOSTFX_SET_POSTFX_COLOR(effectName: & CStr, p1: i32, red: i32, green: i32, blue: i32, alpha: i32) { invoke_ignore!(0x63011D0C7C6519E0, effectName, p1, red, green, blue, alpha) }
	pub fn _0xB958D97A0DFAA0C2(effectName: & CStr) -> bool { invoke!(0xB958D97A0DFAA0C2, effectName) }
	pub fn _0xA201A3D0AC087C37(effectName: & CStr) { invoke_ignore!(0xA201A3D0AC087C37, effectName) }
	pub fn _0xFBF161FCFEC8589E(effectName: & CStr, p1: i32, p2: bool, p3: &mut bool) -> bool { invoke!(0xFBF161FCFEC8589E, effectName, p1, p2, p3) }
	pub fn _ANIMPOSTFX_GET_STACKHASH(effectName: & CStr) -> Hash { invoke!(0x842CCC9491FFCD9B, effectName) }
	pub fn _ANIMPOSTFX_PRELOAD_POSTFX_BY_STACKHASH(effectNameHash: Hash) { invoke_ignore!(0xF3E039322BFBD4D8, effectNameHash) }
	pub fn ANIMPOSTFX_IS_PRELOADING_BY_STACKHASH(effectNameHash: Hash) -> bool { invoke!(0x59EA80079B86D8C7, effectNameHash) }
	pub fn _0x38D9D50F2085E9B3(effectNameHash: Hash) { invoke_ignore!(0x38D9D50F2085E9B3, effectNameHash) }
	pub fn _ANIMPOSTFX_PLAY_TAG(effectNameHash: Hash) { invoke_ignore!(0x9B8D5D4CB8AF58B3, effectNameHash) }
	pub fn _0xC76FC4C2FC5F4405(effectNameHash: Hash) { invoke_ignore!(0xC76FC4C2FC5F4405, effectNameHash) }
	pub fn _ANIMPOSTFX_STOP_STACKHASH_POSTFX(effectNameHash: Hash) { invoke_ignore!(0xEDA5CBECF56E1386, effectNameHash) }
	pub fn _ANIMPOSTFX_IS_STACKHASH_PLAYING(effectNameHash: Hash) -> bool { invoke!(0xEEF83A759AE06A27, effectNameHash) }
	pub fn _0xE75CDDEBF618C8FF(effectNameHash: Hash) -> bool { invoke!(0xE75CDDEBF618C8FF, effectNameHash) }
	pub fn _0x71845905BCCDE781(effectNameHash: Hash) { invoke_ignore!(0x71845905BCCDE781, effectNameHash) }
	pub fn ANIMPOSTFX_HAS_EVENT_TRIGGERED_BY_STACKHASH(effectNameHash: Hash, p1: i32, p2: bool, p3: &mut bool) -> bool { invoke!(0x9AB192A9EF980EED, effectNameHash, p1, p2, p3) }
	pub fn _0xFF584F097C17FA8F() -> bool { invoke!(0xFF584F097C17FA8F) }
	pub fn _0x3DA7A10583A4BEC0() -> bool { invoke!(0x3DA7A10583A4BEC0) }
	pub fn _0xC37792A3F9C90771() -> Any { invoke!(0xC37792A3F9C90771) }
	pub fn _0xA0F4D12D6042F6D5(p0: Any, p1: Any) { invoke_ignore!(0xA0F4D12D6042F6D5, p0, p1) }
	pub fn _0x8996FA6AD9FE4E90(p0: Any) { invoke_ignore!(0x8996FA6AD9FE4E90, p0) }
	pub fn _SET_ENTITY_RENDER_GUARMA_SHIP(vehicle: Vehicle, toggle: bool) { invoke_ignore!(0xC38B4952B728397A, vehicle, toggle) }
	pub fn PEDSHOT_IS_AVAILABLE() -> bool { invoke!(0xAF6E67D073D2DCE2) }
	pub fn _0xFD05B1DDE83749FA(p0: & CStr) -> bool { invoke!(0xFD05B1DDE83749FA, p0) }
	pub fn _PEDSHOT_FINISH_CLEANUP_DATA() { invoke_ignore!(0xC2B8164C3BE871A4) }
	pub fn _PEDSHOT_PREVIOUS_PERSONA_PHOTO_DATA_CLEANUP() { invoke_ignore!(0x3E2FDDBE435A8787) }
	pub fn _PEDSHOT_INIT_CLEANUP_DATA() { invoke_ignore!(0x55285F885F662169) }
	pub fn _PEDSHOT_GENERATE_PERSONA_PHOTO(texture: & CStr, ped: Ped, playerSlot: i32) -> bool { invoke!(0xD9C24F53631F2372, texture, ped, playerSlot) }
	pub fn _PEDSHOT_SET_PERSONA_PHOTO_TYPE(personaPhotoLocalCacheType: i32) { invoke_ignore!(0x196D3ACBEBA4A44B, personaPhotoLocalCacheType) }
	pub fn _0xA1A86055792FB249(personaPhotoLocalCacheType: i32) { invoke_ignore!(0xA1A86055792FB249, personaPhotoLocalCacheType) }
	pub fn _0x402E1A61D2587FCD(p0: Any, x: f32, y: f32, z: f32, p4: f32, p5: f32, heading: f32) -> bool { invoke!(0x402E1A61D2587FCD, p0, x, y, z, p4, p5, heading) }
	pub fn _0x5C9C3A466B3296A8(p0: Any) -> Any { invoke!(0x5C9C3A466B3296A8, p0) }
	pub fn _0xA15CCAB8AD038291(p0: Any, p1: Any, p2: Any, p3: Any) -> Any { invoke!(0xA15CCAB8AD038291, p0, p1, p2, p3) }
	pub fn _0x285438C26C732F9D() -> Any { invoke!(0x285438C26C732F9D) }
	pub fn _SET_PROXY_INTERIOR_INDEX_ARTIFICIAL_LIGHTS_STATE(proxyInteriorIndex: i32, state: bool) { invoke_ignore!(0xBFCB17895BB99E4E, proxyInteriorIndex, state) }
	pub fn _IS_PROXY_INTERIOR_INDEX_ARTIFICIAL_LIGHTS_ENABLED(proxyInteriorIndex: i32) -> bool { invoke!(0x113857D66A9CABE6, proxyInteriorIndex) }
	pub fn _GET_PROXY_INTERIOR_INDEX(interiorId: i32) -> i32 { invoke!(0x5D1C5D8E62E8EE1C, interiorId) }
	pub fn _0x9D1B0B5066205692() { invoke_ignore!(0x9D1B0B5066205692) }
	pub fn _0xC489FE31AC726512(p0: Any, p1: Any) { invoke_ignore!(0xC489FE31AC726512, p0, p1) }
	pub fn _SET_CLOUD_LAYER(x: f32, y: f32, p2: i32) { invoke_ignore!(0xB8C984C0D47F4F07, x, y, p2) }
	pub fn _SET_CLOUD_NOISE(x: f32, y: f32, z: f32) { invoke_ignore!(0xFE7966DF01452F32, x, y, z) }
	pub fn _SET_CLOUD_POSITION(x: f32, y: f32, z: f32) { invoke_ignore!(0x10C1767B93257480, x, y, z) }
	pub fn _SET_CLOUD_HEIGHT(height: f32) { invoke_ignore!(0xC332C91388F5580B, height) }
	pub fn _0x085C5B61A0114F32(p0: Any, p1: Any) { invoke_ignore!(0x085C5B61A0114F32, p0, p1) }
	pub fn _0x1FF8731BE1DFC0C0(p0: Any, p1: Any) { invoke_ignore!(0x1FF8731BE1DFC0C0, p0, p1) }
	pub fn _0xFC9B53C072F418E0() -> Any { invoke!(0xFC9B53C072F418E0) }
	pub fn _0x94B261F1F35293E1(p0: Any) { invoke_ignore!(0x94B261F1F35293E1, p0) }
	pub fn ENABLE_ENTITYMASK() { invoke_ignore!(0xFAAD23DE7A54FC14) }
	pub fn DISABLE_ENTITYMASK() { invoke_ignore!(0x5C9978A2A3DC3D0D) }
	pub fn _ADD_ENTITY_TO_ENTITY_MASK(entity: Entity, mask: i32) { invoke_ignore!(0xC6F81FCD15350323, entity, mask) }
	pub fn _ADD_ENTITY_TO_ENTITY_MASK_WITH_INTENSITY(entity: Entity, mask: i32, intensity: f32) { invoke_ignore!(0x958DEBD9353C0935, entity, mask, intensity) }
	pub fn _REMOVE_ENTITY_FROM_ENTITY_MASK(entity: Entity) { invoke_ignore!(0x56A786E87FF53478, entity) }
	pub fn _GET_ENTITY_MASK_LAYERS(entity: Entity, layer0: &mut f32, layer1: &mut f32, layer2: &mut f32, layer3: &mut f32) -> bool { invoke!(0xE8A8378BF651079C, entity, layer0, layer1, layer2, layer3) }
	pub fn _SET_ENTITY_MASK_LAYERS(entity: Entity, layer0: &mut f32, layer1: &mut f32, layer2: &mut f32, layer3: &mut f32) { invoke_ignore!(0xE92012611461A42A, entity, layer0, layer1, layer2, layer3) }
	pub fn _SET_ENTITY_AURA(p0: f32, p1: f32, p2: f32) { invoke_ignore!(0x249CD6B7285536F2, p0, p1, p2) }
	pub fn _RESET_ENTITY_AURA() { invoke_ignore!(0xAF4D239B8903FCBE) }
	pub fn _SET_SNOW_COVERAGE_TYPE(type_: i32) { invoke_ignore!(0xF02A9C330BBFC5C7, type_) }
	pub fn _0x519928DF02EB5101(p0: Any) { invoke_ignore!(0x519928DF02EB5101, p0) }
	pub fn _0x1C6306E5BC25C29C() { invoke_ignore!(0x1C6306E5BC25C29C) }
}
pub mod GANG {
	use std::ffi::CStr;
	use crate::core::scripthook::native::*;
	use crate::core::types::*;

	pub fn NETWORK_IS_GANG_ID_VALID(gangId: Any) -> bool { invoke!(0xD6F6ACF4392187FB, gangId) }
	pub fn NETWORK_IS_GANG_IN_SESSION(gangId: Any) -> bool { invoke!(0x93A91A351A07360E, gangId) }
	pub fn NETWORK_IS_GANG_ACTIVE(gangId: Any) -> bool { invoke!(0x0F99F6436528A089, gangId) }
	pub fn _NETWORK_IS_GANG_OPEN(gangId: Any) -> bool { invoke!(0xFCF96CCBD81B24C8, gangId) }
	pub fn _NETWORK_GET_GANG_PRIVACY() -> i32 { invoke!(0x9970AE8C3D706139) }
	pub fn _NETWORK_START_GANG(openStatus: bool, campSize: i32) { invoke_ignore!(0xD1A226F2E05E58FC, openStatus, campSize) }
	pub fn _NETWORK_SET_GANG_PRIVACY(privacyType: i32) -> bool { invoke!(0xC5BF29F4035277C2, privacyType) }
	pub fn _NETWORK_LEAVE_GANG(disband: bool) { invoke_ignore!(0x0A04A07BC3074EDB, disband) }
	pub fn _NETWORK_KICK_GANG_MEMBER(player: Player, banTimeSeconds: i32) { invoke_ignore!(0xCD9E2D9BC52FD80F, player, banTimeSeconds) }
	pub fn _NETWORK_REQUEST_GANG_JOIN(gangId: Any) -> bool { invoke!(0xC0474C8BCF6787AD, gangId) }
	pub fn NETWORK_IS_GANG_LEADER(player: Player) -> bool { invoke!(0x424B17A7DC5C90BC, player) }
	pub fn _NETWORK_IS_GANG_MEMBER(gangId: Any, player: Player) -> bool { invoke!(0x9BE7DCB22D32CCBE, gangId, player) }
	pub fn NETWORK_IS_IN_SAME_GANG(player1: Player, player2: Player) -> bool { invoke!(0x3F59FE6F37869576, player1, player2) }
	pub fn _NETWORK_IS_IN_MY_GANG(player: Player) -> bool { invoke!(0x81FB74C83C2ED69F, player) }
	pub fn NETWORK_GET_NUM_GANG_MEMBERS(gangId: Any) -> i32 { invoke!(0x149A2751AB66AC02, gangId) }
	pub fn _NETWORK_GET_GANG_SIZE(gangId: Any) -> i32 { invoke!(0x853B0FA4D8732C57, gangId) }
	pub fn _NETWORK_SET_GANG_SIZE(size: i32) -> bool { invoke!(0x833D8268D51B4522, size) }
	pub fn NETWORK_GET_GANG_ID(player: Player) -> Any { invoke!(0x901E0DC25080C8B9, player) }
	pub fn _NETWORK_GET_GANG_MEMBERS(gangId: Any, memberHandles: &mut Any) -> i32 { invoke!(0xD1BF325C8252A982, gangId, memberHandles) }
	pub fn _0xDA801F7F6A5278D3(player: Player) -> bool { invoke!(0xDA801F7F6A5278D3, player) }
	pub fn _0x2F7EB8B6F6AFE79C(p0: Any) -> Any { invoke!(0x2F7EB8B6F6AFE79C, p0) }
	pub fn _0x53A94294FDDCF98C(p0: Any, p1: Any) -> Any { invoke!(0x53A94294FDDCF98C, p0, p1) }
	pub fn NETWORK_GET_GANG_LEADER(gangId: Any) -> Player { invoke!(0x4BE6C13A45CCA8EC, gangId) }
	pub fn _NETWORK_GET_GANG_LEADER_HANDLE(gangId: Any, gamerHandle: &mut Any) -> bool { invoke!(0xCE88A261DCBBA0D9, gangId, gamerHandle) }
	pub fn _0x6102830F764B3DE1(player: Player) -> bool { invoke!(0x6102830F764B3DE1, player) }
	pub fn _0xB38C256498748413() { invoke_ignore!(0xB38C256498748413) }
	pub fn _0xE4C64CD37CB176AA(p0: i32) -> Any { invoke!(0xE4C64CD37CB176AA, p0) }
	pub fn _0x7BAA30C9BBE8AEE7(p0: Any, p1: Any) -> Any { invoke!(0x7BAA30C9BBE8AEE7, p0, p1) }
	pub fn _0x0E5C9FB9ED5DFF1C(p0: Any) -> Any { invoke!(0x0E5C9FB9ED5DFF1C, p0) }
	pub fn _0xB22B1D9F74095382(p0: Any) { invoke_ignore!(0xB22B1D9F74095382, p0) }
	pub fn _0xEE4F20004D0288B7() { invoke_ignore!(0xEE4F20004D0288B7) }
	pub fn _0xAD22AB64FA428DF3(p0: Any) { invoke_ignore!(0xAD22AB64FA428DF3, p0) }
	pub fn _0x48D82C83987E18E4(p0: Any) -> Any { invoke!(0x48D82C83987E18E4, p0) }
	pub fn _0xA9CEAE8D6637FBAD(p0: Any) { invoke_ignore!(0xA9CEAE8D6637FBAD, p0) }
	pub fn _0x51C5EF47086AA0D7() -> Any { invoke!(0x51C5EF47086AA0D7) }
	pub fn _0x644E02F24F9D4E98(p0: Any, p1: Any) -> Any { invoke!(0x644E02F24F9D4E98, p0, p1) }
	pub fn _0x3ADC71A66356D706() -> Any { invoke!(0x3ADC71A66356D706) }
	pub fn _0xFA7C5B7E087A4CEB() -> Any { invoke!(0xFA7C5B7E087A4CEB) }
	pub fn _0x1F11702DDBD915C6(p0: Any, p1: Any) -> Any { invoke!(0x1F11702DDBD915C6, p0, p1) }
	pub fn _0x7933754F260B428A(player: Player) -> Any { invoke!(0x7933754F260B428A, player) }
	pub fn _0xAFD3599A3CC5637D() -> Any { invoke!(0xAFD3599A3CC5637D) }
	pub fn _0xC81A9E2C8EFD28D5(p0: Any) { invoke_ignore!(0xC81A9E2C8EFD28D5, p0) }
}
pub mod GOOGLE_ANALYTICS {
	use std::ffi::CStr;
	use crate::core::scripthook::native::*;
	use crate::core::types::*;

	pub fn _GOOGLE_ANALYTICS_PUSH_PAGE(pageName: & CStr) { invoke_ignore!(0xD43A616AE3AC4EF6, pageName) }
	pub fn _GOOGLE_ANALYTICS_POP_PAGE(pageName: & CStr) { invoke_ignore!(0xC6DE040378364798, pageName) }
	pub fn _GOOGLE_ANALYTICS_START_EVENT(eventCategory: & CStr, eventAction: & CStr, eventLabel: & CStr, eventValue: i32) -> bool { invoke!(0x1C54F031D7C0F7AC, eventCategory, eventAction, eventLabel, eventValue) }
	pub fn _GOOGLE_ANALYTICS_END_EVENT() -> bool { invoke!(0x87BBCC4360A9BDE3) }
}
pub mod HUD {
	use std::ffi::CStr;
	use crate::core::scripthook::native::*;
	use crate::core::types::*;

	pub fn _ENABLE_REDUCED_MENU_TIME_SCALE() { invoke_ignore!(0x26F6BBEA2CE3E3DC) }
	pub fn _DISABLE_REDUCED_MENU_TIME_SCALE() { invoke_ignore!(0xC5C7A2F6567FCCBC) }
	pub fn _ENABLE_HUD_CONTEXT_THIS_FRAME(component: Hash) { invoke_ignore!(0xC9CAEAEEC1256E54, component) }
	pub fn _ENABLE_HUD_CONTEXT(component: Hash) { invoke_ignore!(0x4CC5F2FC1332577F, component) }
	pub fn _DISABLE_HUD_CONTEXT(component: Hash) { invoke_ignore!(0x8BC7C1F929D07BF3, component) }
	pub fn _0x7EC0D68233E391AC(p0: i32) -> i32 { invoke!(0x7EC0D68233E391AC, p0) }
	pub fn _BUSYSPINNER_SET_TEXT(text: & CStr) { invoke_ignore!(0x7F78CD75CC4539E4, text) }
	pub fn BUSYSPINNER_OFF() { invoke_ignore!(0x58F441B90EA84D06) }
	pub fn BUSYSPINNER_IS_ON() -> bool { invoke!(0x823BF7B1DF613A21) }
	pub fn GET_CHARACTER_FROM_AUDIO_CONVERSATION_FILENAME(text: & CStr, position: i32, length: i32) -> *const char { invoke!(0x9D7E12EC6A1EE4E5, text, position, length) }
	pub fn GET_FILENAME_FOR_AUDIO_CONVERSATION(labelName: & CStr) -> *const char { invoke!(0xCFEDCCAD3C5BA90D, labelName) }
	pub fn CLEAR_ALL_HELP_MESSAGES() { invoke_ignore!(0x916ED8321F087059) }
	pub fn TEXT_BLOCK_IS_LOADED(textBlock: & CStr) -> bool { invoke!(0xD0976CC34002DB57, textBlock) }
	pub fn _TEXT_BLOCK_IS_STREAMED(textBlock: & CStr) -> bool { invoke!(0x3CF96E16265B7DC8, textBlock) }
	pub fn TEXT_BLOCK_REQUEST(textBlock: & CStr) { invoke_ignore!(0xF66090013DE648D5, textBlock) }
	pub fn _TEXT_BLOCK_DELETE(textBlock: & CStr) { invoke_ignore!(0xAA03F130A637D923, textBlock) }
	pub fn _DOES_TEXT_BLOCK_EXIST(textDatabase: & CStr) -> bool { invoke!(0x2C729F2B94CEA911, textDatabase) }
	pub fn DOES_TEXT_LABEL_EXIST(label: & CStr) -> bool { invoke!(0x73C258C68D6F55B6, label) }
	pub fn _GET_LABEL_TEXT_2(label: & CStr) -> *const char { invoke!(0x3429670F9B9EF2D3, label) }
	pub fn _GET_TEXT_SUBSTRING_2(text: & CStr, length: i32) -> *const char { invoke!(0xD8402B858F4DDD88, text, length) }
	pub fn _0x806862E5D266CF38(p0: Any, p1: Any, p2: Any) -> Any { invoke!(0x806862E5D266CF38, p0, p1, p2) }
	pub fn GET_LENGTH_OF_LITERAL_STRING(string: & CStr) -> i32 { invoke!(0x481FBF588B0B76DB, string) }
	pub fn GET_LENGTH_OF_LITERAL_STRING_IN_BYTES(string: & CStr) -> i32 { invoke!(0xDC5AD6B7AB8184F5, string) }
	pub fn GET_STRING_FROM_HASH_KEY(labelHash: Hash) -> *const char { invoke!(0xBD5DD5EAE2B6CE14, labelHash) }
	pub fn IS_RADAR_PREFERENCE_SWITCHED_ON() -> bool { invoke!(0x81E47F0EE1F2B21E) }
	pub fn IS_SUBTITLE_PREFERENCE_SWITCHED_ON() -> bool { invoke!(0x7C4AC9573587F2DF) }
	pub fn DISPLAY_HUD(toggle: bool) { invoke_ignore!(0xD63FE3AF9FB3D53F, toggle) }
	pub fn IS_HUD_HIDDEN() -> bool { invoke!(0x71B72B478F8189DC) }
	pub fn IS_RADAR_HIDDEN() -> bool { invoke!(0x1B82FD5FFA4D666E) }
	pub fn IS_RADAR_HIDDEN_BY_SCRIPT() -> bool { invoke!(0x66F35DD9D2B58579) }
	pub fn _GET_COLOR_FROM_NAME(colorNameHash: Hash, red: &mut i32, green: &mut i32, blue: &mut i32, alpha: &mut i32) { invoke_ignore!(0xB981DD2DFAF9B1C9, colorNameHash, red, green, blue, alpha) }
	pub fn _DISPLAY_TEXT(text: & CStr, xPos: f32, yPos: f32) { invoke_ignore!(0xD79334A4BB99BAD1, text, xPos, yPos) }
	pub fn _SET_TEXT_COLOR(r: i32, g: i32, b: i32, a: i32) { invoke_ignore!(0x50A41AD966910F03, r, g, b, a) }
	pub fn SET_TEXT_RENDER_ID(renderId: i32) { invoke_ignore!(0xE550CDE128D56757, renderId) }
	pub fn REGISTER_NAMED_RENDERTARGET(name: & CStr, p1: bool) -> bool { invoke!(0x98AF2BB6F62BD588, name, p1) }
	pub fn IS_NAMED_RENDERTARGET_REGISTERED(name: & CStr) -> bool { invoke!(0x3EE32F7964C40FE6, name) }
	pub fn RELEASE_NAMED_RENDERTARGET(name: & CStr) -> bool { invoke!(0x0E692EE61761361F, name) }
	pub fn LINK_NAMED_RENDERTARGET(modelHash: Hash) { invoke_ignore!(0x2F506B8556242DDB, modelHash) }
	pub fn GET_NAMED_RENDERTARGET_RENDER_ID(name: & CStr) -> i32 { invoke!(0xB6762A85EE29AA60, name) }
	pub fn IS_NAMED_RENDERTARGET_LINKED(modelHash: Hash) -> bool { invoke!(0x707032835FF09AE7, modelHash) }
	pub fn _0x9D37EB5003E0F2CF(p0: Any, p1: Any) { invoke_ignore!(0x9D37EB5003E0F2CF, p0, p1) }
	pub fn HIDE_LOADING_ON_FADE_THIS_FRAME() { invoke_ignore!(0xEA600AABAF4B9084) }
	pub fn _0x052D4AC0922AF91A(p0: Any, p1: Any) { invoke_ignore!(0x052D4AC0922AF91A, p0, p1) }
	pub fn HIDE_HUD_AND_RADAR_THIS_FRAME() { invoke_ignore!(0x36CDD81627A6FCD2) }
	pub fn _0x8A59D44189AF2BC5(p0: Any, p1: Any) { invoke_ignore!(0x8A59D44189AF2BC5, p0, p1) }
	pub fn _0x160825DADF1B04B3() { invoke_ignore!(0x160825DADF1B04B3) }
	pub fn _0x9C409BBC492CB5B1() -> Any { invoke!(0x9C409BBC492CB5B1) }
	pub fn _0x0501D52D24EA8934(p0: Any) -> Any { invoke!(0x0501D52D24EA8934, p0) }
	pub fn _0x100157D6D7FE32CA(p0: Any, p1: Any) -> Any { invoke!(0x100157D6D7FE32CA, p0, p1) }
	pub fn _0x28AE29D909C8FDCE(p0: Any) -> Any { invoke!(0x28AE29D909C8FDCE, p0) }
	pub fn _0x2F7BB105144ACF30() { invoke_ignore!(0x2F7BB105144ACF30) }
	pub fn _0xBFFF81E12A745A5F() { invoke_ignore!(0xBFFF81E12A745A5F) }
	pub fn SET_MISSION_NAME(p0: bool, name: & CStr) { invoke_ignore!(0x402669A4BDAA72DA, p0, name) }
	pub fn SET_MISSION_NAME_FOR_UGC_MISSION(p0: bool, name: & CStr) { invoke_ignore!(0xD98630CE73C61E98, p0, name) }
	pub fn _0xCE0D2F5586627CCE(p0: Any, p1: Any, p2: Any, p3: Any, p4: Any) { invoke_ignore!(0xCE0D2F5586627CCE, p0, p1, p2, p3, p4) }
	pub fn GET_HUD_SCREEN_POSITION_FROM_WORLD_POSITION(worldX: f32, worldY: f32, worldZ: f32, screenX: &mut f32, screenY: &mut f32) -> i32 { invoke!(0xB39C81628EF10B42, worldX, worldY, worldZ, screenX, screenY) }
	pub fn _HIDE_HUD_THIS_FRAME() { invoke_ignore!(0xBF4F34A85CA2970D) }
	pub fn DISABLE_FRONTEND_THIS_FRAME() { invoke_ignore!(0x56CE42A528156A67) }
	pub fn _0x5651516D947ABC53() { invoke_ignore!(0x5651516D947ABC53) }
	pub fn ALLOW_PAUSE_WHEN_NOT_IN_STATE_OF_PLAY_THIS_FRAME() { invoke_ignore!(0x30996422DF1EE561) }
	pub fn SET_FRONTEND_ACTIVE(active: bool) { invoke_ignore!(0xCE47C21C0687EBC2, active) }
	pub fn IS_PAUSE_MENU_ACTIVE() -> bool { invoke!(0x535384D6067BA42E) }
	pub fn _SHOW_PLAYER_CORES(state: bool) { invoke_ignore!(0x50C803A4CD5932C5, state) }
	pub fn _SHOW_HORSE_CORES(state: bool) { invoke_ignore!(0xD4EE21B7CC7FD350, state) }
	pub fn _0x3FE4FB41EF7D2196(p0: Any) { invoke_ignore!(0x3FE4FB41EF7D2196, p0) }
	pub fn _JOURNAL_WRITE_ENTRY(p0: Any) { invoke_ignore!(0x6DFDD665E416B093, p0) }
	pub fn _JOURNAL_CAN_WRITE_ENTRY(p0: Any) -> Any { invoke!(0xCF782691D91F270E, p0) }
	pub fn _JOURNAL_GET_ENTRY_COUNT() -> Any { invoke!(0xE65B5DE53351BE22) }
	pub fn _JOURNAL_GET_ENTRY_AT_INDEX(p0: Any) -> Any { invoke!(0x3D16ABD7A1FD8C96, p0) }
	pub fn _JOURNAL_GET_GRIME_AT_INDEX(p0: Any) -> Any { invoke!(0xCB5945E1B855852F, p0) }
	pub fn _JOURNAL_GET_ENTRY_INFO(p0: Any, p1: Any) -> Any { invoke!(0x5514C3E60673530F, p0, p1) }
	pub fn _JOURNAL_GET_TEXTURE_WITH_LAYOUT(p0: Any, p1: Any, p2: Any) -> Any { invoke!(0x62CC549B3B8EA2AA, p0, p1, p2) }
	pub fn _JOURNAL_MARK_READ(p0: Any) { invoke_ignore!(0xE4509BABE59BD24E, p0) }
	pub fn _JOURNAL_CLEAR_ALL_PROGRESS() { invoke_ignore!(0xF402978DE6F88D6E) }
	pub fn UI_REQUEST_SCENE(p0: Any, p1: Any) -> Any { invoke!(0xB6857100F8FD433C, p0, p1) }
	pub fn _0xF1E6979C0B779985(uiscene: i32) { invoke_ignore!(0xF1E6979C0B779985, uiscene) }
	pub fn UI_GET_SCENE_UIOBJECT(p0: Any) -> Any { invoke!(0xBE1067CD1C9570F6, p0) }
	pub fn UI_MOVIEVIEW_SET_RENDER_TARGET(p0: Any, p1: Any) { invoke_ignore!(0x51DE09A2196BD951, p0, p1) }
	pub fn _UI_PROMPT_CREATE(inputHash: Hash, labelName: & CStr, p2: Any, p3: Any, p4: Any, p5: i32) -> Prompt { invoke!(0x29FA7910726C3889, inputHash, labelName, p2, p3, p4, p5) }
	pub fn _UI_PROMPT_REGISTER_BEGIN() -> Prompt { invoke!(0x04F97DE45A519419) }
	pub fn _UI_PROMPT_REGISTER_END(prompt: Prompt) { invoke_ignore!(0xF7AA2696A22AD8B9, prompt) }
	pub fn _UI_PROMPT_SET_PRIORITY(prompt: Prompt, priority: i32) { invoke_ignore!(0xCA24F528D0D16289, prompt, priority) }
	pub fn _UI_PROMPT_SET_CONTROL_ACTION(prompt: Prompt, action: Hash) -> Any { invoke!(0xB5352B7494A08258, prompt, action) }
	pub fn _UI_PROMPT_SET_ALLOWED_ACTION(prompt: Prompt, action: Hash) { invoke_ignore!(0x565C1CE183CB0EAF, prompt, action) }
	pub fn _UI_PROMPT_SET_STANDARD_MODE(prompt: Prompt, releaseMode: bool) { invoke_ignore!(0xCC6656799977741B, prompt, releaseMode) }
	pub fn _UI_PROMPT_HAS_STANDARD_MODE_COMPLETED(prompt: Prompt, p1: i32) -> bool { invoke!(0xC92AC953F0A982AE, prompt, p1) }
	pub fn _UI_PROMPT_SET_PRESSED_TIMED_MODE(prompt: Prompt, depletionTimeMs: i32) { invoke_ignore!(0x1473D3AF51D54276, prompt, depletionTimeMs) }
	pub fn _UI_PROMPT_HAS_PRESSED_TIMED_MODE_COMPLETED(prompt: Prompt) -> bool { invoke!(0x3CE854D250A88DAF, prompt) }
	pub fn _UI_PROMPT_HAS_PRESSED_TIMED_MODE_FAILED(prompt: Prompt) -> bool { invoke!(0x1A17B9ECFF617562, prompt) }
	pub fn _UI_PROMPT_SET_HOLD_MODE(prompt: Prompt, holdTimeMs: i32) { invoke_ignore!(0x94073D5CA3F16B7B, prompt, holdTimeMs) }
	pub fn _UI_PROMPT_SET_STANDARDIZED_HOLD_MODE(prompt: Prompt, timedEventHash: Hash) { invoke_ignore!(0x74C7D7B72ED0D3CF, prompt, timedEventHash) }
	pub fn _UI_PROMPT_SET_HOLD_INDEFINITELY_MODE(prompt: Prompt) { invoke_ignore!(0xEA5CCF4EEB2F82D1, prompt) }
	pub fn _UI_PROMPT_SET_HOLD_AUTO_FILL_MODE(prompt: Prompt, autoFillTimeMs: i32, holdTimeMs: i32) { invoke_ignore!(0x3CE932E737C145D6, prompt, autoFillTimeMs, holdTimeMs) }
	pub fn _UI_PROMPT_SET_HOLD_AUTO_FILL_WITH_DECAY_MODE(prompt: Prompt, autoFillTimeMs: i32, holdTimeMs: i32) { invoke_ignore!(0xA3F2149AA24F3D8E, prompt, autoFillTimeMs, holdTimeMs) }
	pub fn _UI_PROMPT_HAS_HOLD_AUTO_FILL_MODE(prompt: Prompt) -> bool { invoke!(0x8010BEBD0D5ED5BC, prompt) }
	pub fn _UI_PROMPT_HAS_HOLD_MODE(prompt: Prompt) -> bool { invoke!(0xB60C9F9ED47ABB76, prompt) }
	pub fn _UI_PROMPT_IS_HOLD_MODE_RUNNING(prompt: Prompt) -> bool { invoke!(0xC7D70EAEF92EFF48, prompt) }
	pub fn _UI_PROMPT_HAS_HOLD_MODE_COMPLETED(prompt: Prompt) -> bool { invoke!(0xE0F65F0640EF0617, prompt) }
	pub fn _UI_PROMPT_GET_PROGRESS(prompt: Prompt) -> f32 { invoke!(0x81801291806DBC50, prompt) }
	pub fn _UI_PROMPT_SET_MASH_MODE(prompt: Prompt, mashes: i32) { invoke_ignore!(0xDF6423BF071C7F71, prompt, mashes) }
	pub fn _UI_PROMPT_SET_MASH_INDEFINITELY_MODE(prompt: Prompt) { invoke_ignore!(0x7B66E89312727274, prompt) }
	pub fn _UI_PROMPT_SET_MASH_WITH_RESISTANCE_MODE(prompt: Prompt, mashes: i32, p2: f32, p3: f32) { invoke_ignore!(0xCD1BDFF15EFA79F5, prompt, mashes, p2, p3) }
	pub fn _UI_PROMPT_SET_MASH_WITH_RESISTANCE_CAN_FAIL_MODE(prompt: Prompt, mashes: i32, p2: f32, p3: f32) { invoke_ignore!(0xDC0CB602DEADBA53, prompt, mashes, p2, p3) }
	pub fn _UI_PROMPT_SET_MASH_AUTO_FILL_MODE(prompt: Prompt, autoFillTimeMs: i32, mashes: i32) { invoke_ignore!(0x6C39587D7CC66801, prompt, autoFillTimeMs, mashes) }
	pub fn _UI_PROMPT_SET_MASH_MANUAL_MODE(prompt: Prompt, p1: f32, p2: f32, p3: f32, p4: Any) { invoke_ignore!(0x32DF729D8BD3C1C6, prompt, p1, p2, p3, p4) }
	pub fn _UI_PROMPT_SET_MASH_MANUAL_CAN_FAIL_MODE(prompt: Prompt, p1: f32, p2: f32, p3: f32, p4: Any) { invoke_ignore!(0x179DCF71F705DA20, prompt, p1, p2, p3, p4) }
	pub fn _UI_PROMPT_HAS_MANUAL_MASH_MODE(prompt: Prompt) -> bool { invoke!(0xA6C6A4ADB3BAC409, prompt) }
	pub fn _UI_PROMPT_HAS_MASH_MODE(prompt: Prompt) -> bool { invoke!(0xCD072523791DDC1B, prompt) }
	pub fn _0xB0E8599243B3F568(p0: Any) -> Any { invoke!(0xB0E8599243B3F568, p0) }
	pub fn _UI_PROMPT_HAS_MASH_MODE_COMPLETED(prompt: Prompt) -> bool { invoke!(0x845CE958416DC473, prompt) }
	pub fn _UI_PROMPT_HAS_MASH_MODE_FAILED(prompt: Prompt) -> bool { invoke!(0x25B18E530CF39D6F, prompt) }
	pub fn _UI_PROMPT_GET_MASH_MODE_PROGRESS(prompt: Prompt) -> f32 { invoke!(0x8A9585293863B8A5, prompt) }
	pub fn _UI_PROMPT_SET_MASH_MANUAL_MODE_INCREASE_PER_PRESS(prompt: Prompt, rate: f32) { invoke_ignore!(0xA0D1D79C6036A855, prompt, rate) }
	pub fn _UI_PROMPT_SET_MASH_MANUAL_MODE_DECAY_SPEED(prompt: Prompt, speed: f32) { invoke_ignore!(0x7D393C247FB9B431, prompt, speed) }
	pub fn _UI_PROMPT_SET_MASH_MANUAL_MODE_PRESSED_GROWTH_SPEED(prompt: Prompt, speed: f32) { invoke_ignore!(0x56DBB26F98582C29, prompt, speed) }
	pub fn _UI_PROMPT_SET_ROTATE_MODE(prompt: Prompt, p1: f32, counterclockwise: bool) { invoke_ignore!(0x7ABE7095FB3D2581, prompt, p1, counterclockwise) }
	pub fn _UI_PROMPT_SET_TARGET_MODE(prompt: Prompt, p1: f32, p2: f32, p3: Any) { invoke_ignore!(0x5F6503D9CD2754EB, prompt, p1, p2, p3) }
	pub fn _UI_PROMPT_SET_TARGET_MODE_TARGET(prompt: Prompt, p1: f32, p2: f32) { invoke_ignore!(0x5E019C45DD3B6A14, prompt, p1, p2) }
	pub fn _UI_PROMPT_SET_TARGET_MODE_PROGRESS(prompt: Prompt, progress: f32) { invoke_ignore!(0x00123054BEC8A30F, prompt, progress) }
	pub fn _UI_PROMPT_SET_BEAT_MODE(prompt: Prompt, toggle: bool) { invoke_ignore!(0xF957A1654C6322FE, prompt, toggle) }
	pub fn _UI_PROMPT_WAS_BEAT_MODE_PRESSED_IN_TIME_WINDOW(prompt: Prompt) -> bool { invoke!(0x1FE4788AB1430C55, prompt) }
	pub fn _UI_PROMPT_SET_BEAT_MODE_GRAYED_OUT(prompt: Prompt, p1: Any) { invoke_ignore!(0xB487A4936FBF40AC, prompt, p1) }
	pub fn _UI_PROMPT_RESTART_MODES(prompt: Prompt) { invoke_ignore!(0xDC6C55DFA2C24EE5, prompt) }
	pub fn _UI_PROMPT_SET_SPINNER_SPEED(prompt: Prompt, p1: Any) { invoke_ignore!(0xAC6586A7FDCD4B68, prompt, p1) }
	pub fn _UI_PROMPT_SET_SPINNER_POSITION(prompt: Prompt, p1: Any) { invoke_ignore!(0x832CB510DE546282, prompt, p1) }
	pub fn _UI_PROMPT_SET_URGENT_PULSING_ENABLED(prompt: Prompt, toggle: bool) { invoke_ignore!(0xC5F428EE08FA7F2C, prompt, toggle) }
	pub fn _UI_PROMPT_GET_URGENT_PULSING_ENABLED(prompt: Prompt) -> bool { invoke!(0x1FBA0DABECDDB52B, prompt) }
	pub fn _UI_PROMPT_SET_TAG(prompt: Prompt, p1: Any) { invoke_ignore!(0xDEC85C174751292B, prompt, p1) }
	pub fn _UI_PROMPT_GET_GROUP_ID_FOR_TARGET_ENTITY(entity: Entity) -> i32 { invoke!(0xB796970BD125FCE8, entity) }
	pub fn _UI_PROMPT_GET_GROUP_ID_FOR_SCENARIO_POINT(p0: Any, p1: i32) -> i32 { invoke!(0xCB73D7521E7103F0, p0, p1) }
	pub fn _UI_PROMPT_SET_GROUP(prompt: Prompt, groupId: i32, tabIndex: i32) { invoke_ignore!(0x2F11D3A254169EA4, prompt, groupId, tabIndex) }
	pub fn _UI_PROMPT_REMOVE_GROUP(prompt: Prompt, p1: Any) { invoke_ignore!(0x4E52C800A28F7BE8, prompt, p1) }
	pub fn _UI_PROMPT_SET_ACTIVE_GROUP_THIS_FRAME(hash: Hash, name: & CStr, tabAmount: i32, tabDefaultIndex: i32, p4: i32, prompt: Prompt) -> Any { invoke!(0xC65A45D4453C2627, hash, name, tabAmount, tabDefaultIndex, p4, prompt) }
	pub fn _UI_PROMPT_GET_GROUP_ACTIVE_PAGE(hash: Hash) -> i32 { invoke!(0xC1FCC36C3F7286C8, hash) }
	pub fn _UI_PROMPT_SET_AMBIENT_GROUP_THIS_FRAME(entity: Entity, p1: f32, p2: i32, p3: i32, p4: Hash, name: & CStr, p6: i32) -> Any { invoke!(0x315C81D760609108, entity, p1, p2, p3, p4, name, p6) }
	pub fn _0x8B55B324A9123F6B(groupId: i32, volume: Volume, p2: & CStr, p3: Any, p4: Any, p5: Any) -> Any { invoke!(0x8B55B324A9123F6B, groupId, volume, p2, p3, p4, p5) }
	pub fn _UI_PROMPT_DOES_AMBIENT_GROUP_EXIST(hash: Hash) -> bool { invoke!(0xEB550B927B34A1BB, hash) }
	pub fn _UI_PROMPT_ADD_GROUP_LINK(p0: Any, prompt: Prompt, p2: Any) { invoke_ignore!(0x684C96CC7C66E8EF, p0, prompt, p2) }
	pub fn _UI_PROMPT_ADD_GROUP_RETURN_LINK(p0: Any, prompt: Prompt) { invoke_ignore!(0x837972ED28159536, p0, prompt) }
	pub fn _UI_PROMPT_SET_TRANSPORT_MODE(prompt: Prompt, mode: i32) { invoke_ignore!(0x876E4A35C73A6655, prompt, mode) }
	pub fn _UI_PROMPT_DISABLE_PROMPTS_THIS_FRAME() { invoke_ignore!(0xF1622CE88A1946FB) }
	pub fn _UI_PROMPT_DELETE(prompt: Prompt) { invoke_ignore!(0x00EDE88D4D13CF59, prompt) }
	pub fn _UI_PROMPT_IS_VALID(prompt: Prompt) -> bool { invoke!(0x347469FBDD1589A9, prompt) }
	pub fn _UI_PROMPT_IS_ACTIVE(prompt: Prompt) -> bool { invoke!(0x546E342E01DE71CF, prompt) }
	pub fn _UI_PROMPT_SET_VISIBLE(prompt: Prompt, toggle: bool) { invoke_ignore!(0x71215ACCFDE075EE, prompt, toggle) }
	pub fn _UI_PROMPT_SET_ENABLED(prompt: Prompt, toggle: bool) { invoke_ignore!(0x8A0FB4D03A630D21, prompt, toggle) }
	pub fn _UI_PROMPT_SET_TEXT(prompt: Prompt, text: & CStr) { invoke_ignore!(0x5DD02A8318420DD7, prompt, text) }
	pub fn _UI_PROMPT_SET_ATTRIBUTE(prompt: Prompt, attribute: i32, enabled: bool) { invoke_ignore!(0x560E76D5E2E1803F, prompt, attribute, enabled) }
	pub fn _UI_PROMPT_SET_TYPE(prompt: Prompt, type_: i32) { invoke_ignore!(0xF4A5C4509BF923B1, prompt, type_) }
	pub fn _0x53CE46C01A089DA1(prompt: Prompt, p1: bool) { invoke_ignore!(0x53CE46C01A089DA1, prompt, p1) }
	pub fn _UI_PROMPT_SET_MANUAL_RESOLVED(prompt: Prompt, p1: Any) { invoke_ignore!(0xA520C7B05FA4EB2A, prompt, p1) }
	pub fn _UI_PROMPT_CONTEXT_SET_VOLUME(prompt: Prompt, volume: Volume) { invoke_ignore!(0x4D107406667423BE, prompt, volume) }
	pub fn _UI_PROMPT_CONTEXT_SET_POINT(prompt: Prompt, x: f32, y: f32, z: f32) { invoke_ignore!(0xAE84C5EE2C384FB3, prompt, x, y, z) }
	pub fn _UI_PROMPT_CONTEXT_SET_RADIUS(prompt: Prompt, radius: f32) { invoke_ignore!(0x0C718001B77CA468, prompt, radius) }
	pub fn _UI_PROMPT_IS_PRESSED(prompt: Prompt) -> bool { invoke!(0x21E60E230086697F, prompt) }
	pub fn _UI_PROMPT_IS_JUST_PRESSED(prompt: Prompt) -> bool { invoke!(0x2787CC611D3FACC5, prompt) }
	pub fn _UI_PROMPT_IS_JUST_RELEASED(prompt: Prompt) -> bool { invoke!(0x635CC82FA297A827, prompt) }
	pub fn _UI_PROMPT_IS_RELEASED(prompt: Prompt) -> bool { invoke!(0xAFC887BA7A7756D6, prompt) }
	pub fn _UI_PROMPT_ENABLE_PROMPT_TYPE_THIS_FRAME(p0: i32) { invoke_ignore!(0x06565032897BA861, p0) }
	pub fn _UI_PROMPT_DISABLE_PROMPT_TYPE_THIS_FRAME(p0: i32) { invoke_ignore!(0xFC094EF26DD153FA, p0) }
	pub fn _UI_PROMPT_FILTER_CLEAR() { invoke_ignore!(0x6A2F820452017EA2) }
	pub fn _UI_PROMPT_SET_PROMPT_PRIORITY_PREFERENCE(ped: Ped) { invoke_ignore!(0x530A428705BE5DEF, ped) }
	pub fn _UI_PROMPT_CLEAR_PROMPT_PRIORITY_PREFERENCE() { invoke_ignore!(0x51259AE5C72D4A1B) }
	pub fn _UI_PROMPT_IS_ENABLED(prompt: Prompt) -> bool { invoke!(0x0D00EDDFB58B7F28, prompt) }
	pub fn UI_PROMPT_IS_CONTROL_ACTION_ACTIVE(controlAction: Hash) -> bool { invoke!(0x1BE19185B8AFE299, controlAction) }
	pub fn _0xD6BD313CFA41E57A(p0: Any) -> Any { invoke!(0xD6BD313CFA41E57A, p0) }
	pub fn _UI_PROMPT_SET_REGISTER_HORIZONTAL_ORIENTATION() -> i32 { invoke!(0xD9459157EB22C895) }
	pub fn _UI_PROMPT_CLEAR_HORIZONTAL_ORIENTATION(id: i32) { invoke_ignore!(0x6095358C4142932A, id) }
	pub fn _UI_PROMPT_SET_ORDERING_AS_INPUT_TYPE(prompt: Prompt, p1: Any) { invoke_ignore!(0x2F385ECC5200938D, prompt, p1) }
	pub fn _0x066725A9D52B3641() -> Any { invoke!(0x066725A9D52B3641) }
	pub fn _0x958278B97C4AFFD8(p0: Any, p1: Any) { invoke_ignore!(0x958278B97C4AFFD8, p0, p1) }
	pub fn _CREATE_MP_GAMER_TAG(player: Player, username: & CStr, pointedClanTag: bool, isRockstarClan: bool, clanTag: & CStr, clanFlag: i32) -> i32 { invoke!(0xD877AF112AD2B41B, player, username, pointedClanTag, isRockstarClan, clanTag, clanFlag) }
	pub fn CREATE_FAKE_MP_GAMER_TAG(ped: Ped, username: & CStr, pointedClanTag: bool, isRockstarClan: bool, clanTag: & CStr, clanFlag: i32) -> i32 { invoke!(0x53CB4B502E1C57EA, ped, username, pointedClanTag, isRockstarClan, clanTag, clanFlag) }
	pub fn _CREATE_MP_GAMER_TAG_ON_ENTITY(entity: Entity, text: & CStr) -> i32 { invoke!(0xE961BF23EAB76B12, entity, text) }
	pub fn REMOVE_MP_GAMER_TAG(gamerTagId: i32) { invoke_ignore!(0x839BFD7D7E49FE09, gamerTagId) }
	pub fn IS_MP_GAMER_TAG_ACTIVE(gamerTagId: i32) -> bool { invoke!(0x6E1C31E14C7A5F97, gamerTagId) }
	pub fn _IS_MP_GAMER_TAG_ACTIVE_ON_ENTITY(gamerTagId: i32, entity: Entity) -> bool { invoke!(0x502E1591A504F843, gamerTagId, entity) }
	pub fn _SET_MP_GAMER_TAG_VISIBILITY(gamerTagId: i32, visibility: i32) { invoke_ignore!(0x93171DDDAB274EB8, gamerTagId, visibility) }
	pub fn _SET_MP_GAMER_TAG_TYPE(gamerTagId: i32, type_: Hash) { invoke_ignore!(0x25B9C78A25105C35, gamerTagId, type_) }
	pub fn _SET_MP_GAMER_TAG_COLOUR(gamerTagId: i32, colour: Hash) { invoke_ignore!(0x84BD27DDF9575816, gamerTagId, colour) }
	pub fn _SET_MP_GAMER_TAG_UNK_ALLOW_LOCALIZED(gamerTagId: i32, allow: bool) { invoke_ignore!(0xEF7AB1A0E8C86170, gamerTagId, allow) }
	pub fn SET_MP_GAMER_TAG_NAME(gamerTagId: i32, string: & CStr) { invoke_ignore!(0xEA6F4B8D4B4B5B3E, gamerTagId, string) }
	pub fn _SET_MP_GAMER_TAG_NAME_POSSE(gamerTagId: i32, text: & CStr) { invoke_ignore!(0x1EA716E0628A6F44, gamerTagId, text) }
	pub fn SET_MP_GAMER_TAG_BIG_TEXT(gamerTagId: i32, string: & CStr) { invoke_ignore!(0xA0D7CE5F83259663, gamerTagId, string) }
	pub fn _SET_MP_GAMER_TAG_TOP_ICON(gamerTagId: i32, icon: Hash) { invoke_ignore!(0x5F57522BC1EB9D9D, gamerTagId, icon) }
	pub fn _SET_MP_GAMER_TAG_SECONDARY_ICON(gamerTagId: i32, icon: Hash) { invoke_ignore!(0x95384C6CE1526EFF, gamerTagId, icon) }
	pub fn _MP_GAMER_TAG_ENABLE_REVIVE_TOP_ICON(gamerTagId: i32) { invoke_ignore!(0xFFF6579CF0139FCE, gamerTagId) }
	pub fn _MP_GAMER_TAG_DISABLE_REVIVE_TOP_ICON(gamerTagId: i32) { invoke_ignore!(0x1F9A64C2804B3471, gamerTagId) }
}
pub mod IK {
	use std::ffi::CStr;
	use crate::core::scripthook::native::*;
	use crate::core::types::*;

	pub fn _INVERSE_KINEMATICS_REQUEST_LOOK_AT(ped: Ped, args: &mut Any) { invoke_ignore!(0x66F9EB44342BB4C5, ped, args) }
	pub fn _0x0B9F7A01EC50448D(ped: Ped, args: &mut Any) { invoke_ignore!(0x0B9F7A01EC50448D, ped, args) }
	pub fn _INVERSE_KINEMATICS_SET_DISABLED_FOR_PED(ped: Ped, p1: i32, p2: bool) { invoke_ignore!(0x0EABF182FBB63D72, ped, p1, p2) }
	pub fn _0x6098139150DCC745(ped: Ped, p1: i32) -> bool { invoke!(0x6098139150DCC745, ped, p1) }
	pub fn _0x873C792E07A32C8B(ped1: Ped, ped2: Ped) { invoke_ignore!(0x873C792E07A32C8B, ped1, ped2) }
}
pub mod INTERACTION {
	use std::ffi::CStr;
	use crate::core::scripthook::native::*;
	use crate::core::types::*;

	pub fn SET_MOUSE_CURSOR_THIS_FRAME() { invoke_ignore!(0xF12E4CCAF249DC10) }
	pub fn SET_MOUSE_CURSOR_STYLE(spriteId: i32) { invoke_ignore!(0x7F5858AAB5A58CCE, spriteId) }
	pub fn _SET_ALLOW_FIRST_PERSON_MOUSE_CAMERA_MOVEMENT() { invoke_ignore!(0x0546B117BB17548B) }
	pub fn _POINTER_IS_BEING_MOVED() -> bool { invoke!(0x2B8B605F2A9E64BF) }
	pub fn _POINTER_IS_LEFT_BUTTON_HELD() -> bool { invoke!(0x61CAE9D1FD055E44) }
	pub fn _POINTER_IS_LEFT_BUTTON_JUST_RELEASED() -> bool { invoke!(0xF7F51A57349739F2) }
}
pub mod INTERIOR {
	use std::ffi::CStr;
	use crate::core::scripthook::native::*;
	use crate::core::types::*;

	pub fn IS_VALID_INTERIOR(interior: Interior) -> bool { invoke!(0x017C1B3159F79F6C, interior) }
	pub fn GET_INTERIOR_LOCATION_AND_NAMEHASH(interior: Interior, position: &mut Vector3, nameHash: &mut Hash) { invoke_ignore!(0x8451E87D3C2B0286, interior, position, nameHash) }
	pub fn _GET_INTERIOR_MINIMAP_HASH(interior: Interior) -> Hash { invoke!(0x3039BE60B3749716, interior) }
	pub fn _GET_INTERIOR_POSITION(interior: Interior) -> Vector3 { invoke!(0x2C9746D0CA15BE1C, interior) }
	pub fn IS_INTERIOR_SCENE() -> bool { invoke!(0x4200F14D6F840A9A) }
	pub fn CLEAR_ROOM_FOR_ENTITY(entity: Entity) { invoke_ignore!(0xA1762D5BBFCA13A8, entity) }
	pub fn FORCE_ROOM_FOR_ENTITY(entity: Entity, interior: Interior, roomHashKey: Hash) { invoke_ignore!(0xBC29A9894C976945, entity, interior, roomHashKey) }
	pub fn GET_ROOM_KEY_FROM_ENTITY(entity: Entity) -> Hash { invoke!(0x076E46E0EB52AFC6, entity) }
	pub fn GET_KEY_FOR_ENTITY_IN_ROOM(entity: Entity) -> Hash { invoke!(0x27D7B6F79E1F4603, entity) }
	pub fn GET_INTERIOR_FROM_ENTITY(entity: Entity) -> Interior { invoke!(0xB417689857646F61, entity) }
	pub fn RETAIN_ENTITY_IN_INTERIOR(entity: Entity, interior: Interior) { invoke_ignore!(0x5BD616735F16BF5C, entity, interior) }
	pub fn FORCE_ROOM_FOR_GAME_VIEWPORT(interiorID: i32, roomHashKey: Hash) { invoke_ignore!(0x115B4AA8FB28AB43, interiorID, roomHashKey) }
	pub fn CLEAR_ROOM_FOR_GAME_VIEWPORT() { invoke_ignore!(0x951A049765E0D450) }
	pub fn GET_INTERIOR_FROM_PRIMARY_VIEW() -> Interior { invoke!(0xBC8A281FF125C655) }
	pub fn GET_INTERIOR_AT_COORDS(x: f32, y: f32, z: f32) -> Interior { invoke!(0xCDD36C9E5C469070, x, y, z) }
	pub fn PIN_INTERIOR_IN_MEMORY(interior: Interior) { invoke_ignore!(0xBD3D33EABF680168, interior) }
	pub fn UNPIN_INTERIOR(interior: Interior) { invoke_ignore!(0x07FD1A0B814F6055, interior) }
	pub fn IS_INTERIOR_READY(interior: Interior) -> bool { invoke!(0x941560D2D45DBFC8, interior) }
	pub fn SET_INTERIOR_IN_USE(interior: Interior) -> bool { invoke!(0xB5EF6FEF2DC9EBED, interior) }
	pub fn GET_INTERIOR_AT_COORDS_WITH_TYPE(x: f32, y: f32, z: f32, interiorType: & CStr) -> Interior { invoke!(0xAAD6170AA33B13C0, x, y, z, interiorType) }
	pub fn GET_INTERIOR_AT_COORDS_WITH_TYPEHASH(x: f32, y: f32, z: f32, typeHash: Hash) -> Interior { invoke!(0x3543AEA1816D1D2B, x, y, z, typeHash) }
	pub fn IS_COLLISION_MARKED_OUTSIDE(x: f32, y: f32, z: f32) -> bool { invoke!(0xF291396B517E25B2, x, y, z) }
	pub fn GET_INTERIOR_FROM_COLLISION(x: f32, y: f32, z: f32) -> i32 { invoke!(0x5054D1A5218FA696, x, y, z) }
	pub fn ACTIVATE_INTERIOR_ENTITY_SET(interior: Interior, entitySetName: & CStr, p2: i32) { invoke_ignore!(0x174D0AAB11CED739, interior, entitySetName, p2) }
	pub fn DEACTIVATE_INTERIOR_ENTITY_SET(interior: Interior, entitySetName: & CStr, p2: bool) { invoke_ignore!(0x33B81A2C07A51FFF, interior, entitySetName, p2) }
	pub fn IS_INTERIOR_ENTITY_SET_ACTIVE(interior: Interior, entitySetName: & CStr) -> bool { invoke!(0x32810CA2125F5842, interior, entitySetName) }
	pub fn _IS_INTERIOR_ENTITY_SET_VALID(interior: Interior, entitySetName: & CStr) -> bool { invoke!(0xD56FF170710FC826, interior, entitySetName) }
	pub fn DISABLE_INTERIOR(interior: Interior, toggle: bool) { invoke_ignore!(0x3C2B92A1A07D4FCE, interior, toggle) }
	pub fn _0x2533F2AB0EB9C6F9(p0: Any, p1: Any) { invoke_ignore!(0x2533F2AB0EB9C6F9, p0, p1) }
	pub fn _0xFE2B3D5500B1B2E4(p0: Any, p1: Any) { invoke_ignore!(0xFE2B3D5500B1B2E4, p0, p1) }
}
pub mod INVENTORY {
	use std::ffi::CStr;
	use crate::core::scripthook::native::*;
	use crate::core::types::*;

	pub fn _INVENTORY_GET_INVENTORY_ID_FROM_PED(ped: Ped) -> i32 { invoke!(0x13D234A2A3F66E63, ped) }
	pub fn _GET_ITEM_ROLE_MAX_LEVEL_COUNT(inventoryId: i32, eRoleMaxLevel: Hash) -> i32 { invoke!(0xADDD1E7C0ECF7D95, inventoryId, eRoleMaxLevel) }
	pub fn _GET_ITEM_SLOT_MAX_COUNT(provision: Hash, slotId: Hash) -> i32 { invoke!(0xE80E50BEE276A54A, provision, slotId) }
	pub fn _0x112BCA290D2EB53C(inventoryId: i32, p1: Hash, year: &mut i32, month: &mut i32, day: &mut i32, hour: &mut i32, minute: &mut i32, second: &mut i32) -> bool { invoke!(0x112BCA290D2EB53C, inventoryId, p1, year, month, day, hour, minute, second) }
	pub fn _0x46743BBFEDBC859E(inventoryId: i32, eInventoryItem: Hash, p2: bool) { invoke_ignore!(0x46743BBFEDBC859E, inventoryId, eInventoryItem, p2) }
	pub fn _INVENTORY_SET_INVENTORY_ITEM_IN_USE(inventoryId: i32, guid: &mut Any, inUse: bool) { invoke_ignore!(0x65A5F70F4A292EBE, inventoryId, guid, inUse) }
	pub fn _INVENTORY_GET_INVENTORY_ITEM_IN_USE(inventoryId: i32, guid: &mut Any) -> bool { invoke!(0x70E3A884ED000A01, inventoryId, guid) }
	pub fn _INVENTORY_SET_INVENTORY_ITEM_HIDDEN(inventoryId: i32, guid: &mut Any, hidden: bool) { invoke_ignore!(0x9A113C660AEA3832, inventoryId, guid, hidden) }
	pub fn _INVENTORY_SET_INVENTORY_ITEM_HIDDEN_2(inventoryId: i32, guid: &mut Any, hidden: bool) { invoke_ignore!(0xD740F11FBC8AEF43, inventoryId, guid, hidden) }
	pub fn _INVENTORY_GET_INVENTORY_ITEM_HIDDEN(inventoryId: i32, guid: &mut Any) -> bool { invoke!(0xF9933164965533B7, inventoryId, guid) }
	pub fn _INVENTORY_GET_INVENTORY_ITEM_IS_ANIMAL_PELT(item: Hash) -> bool { invoke!(0x4AEF1FB5B9011D75, item) }
	pub fn _INVENTORY_GET_INVENTORY_ITEM_DESCRIPTION_HASH(item: Hash) -> Hash { invoke!(0xA4550FE9C512E3DD, item) }
	pub fn _INVENTORY_IS_INVENTORY_ITEM_FLAG_ENABLED(item: Hash, flag: i32) -> bool { invoke!(0x245D07651B1D183B, item, flag) }
	pub fn _INVENTORY_GET_IS_INVENTORY_ITEM_SOUND_VALID(item: Hash, soundType: i32) -> bool { invoke!(0x2BAE4880DCDD560B, item, soundType) }
	pub fn _INVENTORY_GET_INVENTORY_ITEM_SOUND(item: Hash, soundType: i32) -> Hash { invoke!(0x2E1CDC1FF3B8473E, item, soundType) }
	pub fn _INVENTORY_GET_INVENTORY_ITEM_INSPECTION_INFO(item: Hash, info: &mut Any) -> bool { invoke!(0x0C093C1787F18519, item, info) }
	pub fn _INVENTORY_GET_INVENTORY_ITEM_WEAPON_COPY_ID(inventoryId: i32, guid: &mut Any) -> i32 { invoke!(0xAB5F12746A099A0E, inventoryId, guid) }
	pub fn _INVENTORY_ARE_LOCAL_CHANGES_ALLOWED(inventoryId: i32) -> bool { invoke!(0x0FBBFFC891A97C81, inventoryId) }
	pub fn _INVENTORY_IS_GUID_VALID(guid: &mut Any) -> bool { invoke!(0xB881CA836CC4B6D4, guid) }
	pub fn _INVENTORY_COMPARE_GUIDS(guid1: &mut Any, guid2: &mut Any) -> bool { invoke!(0x4C543D5DFCD2DAFD, guid1, guid2) }
	pub fn INVENTORY_GET_GUID_FROM_ITEMID(inventoryId: i32, guid: &mut Any, p2: Hash, slotId: Hash, outGuid: &mut Any) -> bool { invoke!(0x886DFD3E185C8A89, inventoryId, guid, p2, slotId, outGuid) }
	pub fn _0x5D6182F3BCE1333B(inventoryId: i32, removeReason: Hash) -> bool { invoke!(0x5D6182F3BCE1333B, inventoryId, removeReason) }
	pub fn INVENTORY_GET_INVENTORY_ITEM(inventoryId: i32, inData: &mut Any, outData: &mut Any, p3: bool) -> bool { invoke!(0x9700E8EFC4AB9089, inventoryId, inData, outData, p3) }
	pub fn _INVENTORY_GET_FULL_INVENTORY_ITEM_DATA(inventoryId: i32, guid: &mut Any, p2: &mut Any, p3: i32, p4: i32) -> bool { invoke!(0x025A1B1FB03FBF61, inventoryId, guid, p2, p3, p4) }
	pub fn _INVENTORY_GET_INVENTORY_ITEM_CHILD(inventoryId: i32, parentGuid: &mut Any, childIndex: Any, outInventoryItem: &mut Any) -> bool { invoke!(0xCD9A485F2B383B44, inventoryId, parentGuid, childIndex, outInventoryItem) }
	pub fn _INVENTORY_ADD_ITEM_WITH_GUID(inventoryId: i32, guid1: &mut Any, guid2: &mut Any, item: Hash, inventoryItemSlot: Hash, p5: i32, addReason: Hash) -> bool { invoke!(0xCB5D11F9508A928D, inventoryId, guid1, guid2, item, inventoryItemSlot, p5, addReason) }
	pub fn _INVENTORY_UPDATE_INVENTORY_ITEM(inventoryId: i32, guid1: &mut Any, guid2: &mut Any, p3: i32) -> bool { invoke!(0xD80A8854DB5CFBA5, inventoryId, guid1, guid2, p3) }
	pub fn _INVENTORY_REMOVE_INVENTORY_ITEM_WITH_GUID(inventoryId: i32, guid: &mut Any, quantity: i32, removeReason: Hash) -> bool { invoke!(0x3E4E811480B3AE79, inventoryId, guid, quantity, removeReason) }
	pub fn _INVENTORY_REMOVE_INVENTORY_ITEM_WITH_ITEMID(inventoryId: i32, item: Hash, quantity: i32, removeReason: Hash) -> bool { invoke!(0xB4158C8C9A3B5DCE, inventoryId, item, quantity, removeReason) }
	pub fn _INVENTORY_MOVE_INVENTORY_ITEM(inventoryId: i32, guid1: &mut Any, guid2: &mut Any, slotId: Hash, quantity: i32, outGuid: &mut Any) -> bool { invoke!(0xDCCAA7C3BFD88862, inventoryId, guid1, guid2, slotId, quantity, outGuid) }
	pub fn _INVENTORY_SWAP_INVENTORY_ITEM(inventoryId: i32, guid1: &mut Any, guid2: &mut Any) -> bool { invoke!(0xF2753D691BCDA314, inventoryId, guid1, guid2) }
	pub fn _INVENTORY_CREATE_ITEM_COLLECTION(inventoryId: i32, filterName: & CStr, slotId: Hash, size: &mut i32) -> i32 { invoke!(0x80D78BDC9D88EF07, inventoryId, filterName, slotId, size) }
	pub fn _INVENTORY_CREATE_ITEM_COLLECTION_WITH_FILTER(inventoryId: i32, filter: &mut Any, numInCollection: &mut i32) -> i32 { invoke!(0x640F890C3E5A3FFD, inventoryId, filter, numInCollection) }
	pub fn _INVENTORY_GET_ITEM_FROM_COLLECTION_INDEX(collectionId: i32, itemIndex: i32, itemData: &mut Any) -> bool { invoke!(0x82FA24C3D3FCD9B7, collectionId, itemIndex, itemData) }
	pub fn _INVENTORY_RELEASE_ITEM_COLLECTION(collectionId: i32) -> bool { invoke!(0x42A2F33A1942E865, collectionId) }
	pub fn _INVENTORY_EQUIP_ITEM_WITH_GUID(inventoryId: i32, guid: &mut Any, bEquipped: bool) -> bool { invoke!(0x734311E2852760D0, inventoryId, guid, bEquipped) }
	pub fn _INVENTORY_GET_INVENTORY_ITEM_EQUIPPED_IN_SLOT_BY_REF(inventoryId: i32, guid: &mut Any, slotId: Hash, outGuid: &mut Any) -> bool { invoke!(0x22E590F108289A9D, inventoryId, guid, slotId, outGuid) }
	pub fn _INVENTORY_DOES_ITEM_OWN_EQUIPMENT(inventoryId: i32, guid: &mut Any, item: Hash) -> bool { invoke!(0x88B58B83A43A8CAB, inventoryId, guid, item) }
	pub fn _0xD08685BA892DBFAB(inventoryId: i32, guid: &mut Any, p2: &mut i32, p3: &mut i32) -> bool { invoke!(0xD08685BA892DBFAB, inventoryId, guid, p2, p3) }
	pub fn _0x0349404A22736740(p0: bool, inventoryId: i32, guid: &mut Any) { invoke_ignore!(0x0349404A22736740, p0, inventoryId, guid) }
	pub fn _INVENTORY_SET_INVENTORY_ITEM_WEATHER_EFFECTIVENESS(inventoryId: i32, guid: &mut Any, weatherEffectiveness: i32) { invoke_ignore!(0x6D2F987736A42D4C, inventoryId, guid, weatherEffectiveness) }
	pub fn _INVENTORY_DISABLE_ITEM(inventoryId: i32, item: Hash, gtxReason: Hash) { invoke_ignore!(0x766315A564594401, inventoryId, item, gtxReason) }
	pub fn _INVENTORY_ENABLE_ITEM(inventoryId: i32, item: Hash) { invoke_ignore!(0x6A564540FAC12211, inventoryId, item) }
	pub fn _INVENTORY_IS_INVENTORY_ITEM_EQUIPPED(inventoryId: i32, item: Hash, p2: bool) -> bool { invoke!(0x3D10D7179D7034AF, inventoryId, item, p2) }
	pub fn _INVENTORY_SET_INVENTORY_ITEM_INSPECTION_ENABLED(inventoryId: i32, p1: &mut Any, enabled: bool) -> bool { invoke!(0x227522FD59DDB7E8, inventoryId, p1, enabled) }
	pub fn _INVENTORY_IS_ITEM_EXPIRED(itemGUID: &mut Any) -> bool { invoke!(0x0137C77A2EC64536, itemGUID) }
	pub fn _INVENTORY_GET_ITEM_EXPIRY_TIME(itemGUID: &mut Any) -> i32 { invoke!(0x4A606C17276E1BCC, itemGUID) }
	pub fn _INVENTORY_GET_INVENTORY_ITEM_COUNT_WITH_GUID(inventoryId: i32, guid: &mut Any, p2: bool) -> i32 { invoke!(0xC97E0D2302382211, inventoryId, guid, p2) }
	pub fn _INVENTORY_GET_INVENTORY_ITEM_COUNT_WITH_ITEMID(inventoryId: i32, eInventoryItem: Hash, p2: bool) -> i32 { invoke!(0xE787F05DFC977BDE, inventoryId, eInventoryItem, p2) }
	pub fn _0xB1DD74A1F5536622(inventoryId: i32, itemGUID: &mut Any) -> bool { invoke!(0xB1DD74A1F5536622, inventoryId, itemGUID) }
	pub fn _INVENTORY_GET_CHILDREN_COUNT(inventoryId: i32, parentGuid: &mut Any) -> i32 { invoke!(0xE843D21A8E2498AA, inventoryId, parentGuid) }
	pub fn INVENTORY_GET_CHILDREN_IN_SLOT_COUNT(inventoryId: i32, guid: &mut Any, slotId: Hash) -> i32 { invoke!(0x033EE4B89F3AC545, inventoryId, guid, slotId) }
	pub fn _INVENTORY_GET_INVENTORY_ITEM_EQUIPPED_IN_SLOT(inventoryId: i32, guid: &mut Any, slotId: Hash, p3: i32, p4: &mut Any) -> i32 { invoke!(0xBE012571B25F5ACA, inventoryId, guid, slotId, p3, p4) }
	pub fn _INVENTORY_FITS_SLOT_ID(item: Hash, slotId: Hash) -> bool { invoke!(0x780C5B9AE2819807, item, slotId) }
	pub fn _GET_DEFAULT_ITEM_SLOT_INFO(item: Hash, p1: Hash) -> Hash { invoke!(0x6452B1D357D81742, item, p1) }
	pub fn _INVENTORY_GET_INVENTORY_ITEM_FIT_SLOT(p0: Hash, p1: &mut Any, p2: i32) -> bool { invoke!(0xB991FE166FAF84FD, p0, p1, p2) }
	pub fn _0x9AC53CB6907B4428(item: Hash, p1: &mut Any, p2: &mut Any) -> bool { invoke!(0x9AC53CB6907B4428, item, p1, p2) }
	pub fn _0x9D21B185ABC2DBC4(data: Any, effects: & CStr, p2: bool, p3: bool) -> Hash { invoke!(0x9D21B185ABC2DBC4, data, effects, p2, p3) }
	pub fn _0x75CFAC49301E134F(databindingEntryId: Hash, p1: bool, p2: bool) { invoke_ignore!(0x75CFAC49301E134F, databindingEntryId, p1, p2) }
	pub fn _0x9D21B185ABC2DBC5(data: Any, stats: & CStr, p2: i32, p3: i32) -> Hash { invoke!(0x9D21B185ABC2DBC5, data, stats, p2, p3) }
	pub fn _0x75CFAC49301E134E(data: Any, p1: bool, ped: Ped) { invoke_ignore!(0x75CFAC49301E134E, data, p1, ped) }
	pub fn _0x46DB71883EE9D5AF(data: Any, stats: & CStr, guid: &mut Any, ped: Ped) -> Hash { invoke!(0x46DB71883EE9D5AF, data, stats, guid, ped) }
	pub fn _0x951847CEF3D829FF(p0: Any, outGuid: &mut Any, ped: Ped) { invoke_ignore!(0x951847CEF3D829FF, p0, outGuid, ped) }
	pub fn _0x6862E4D93F64CF01(inventoryId: i32, guid: &mut Any, p2: Hash, p3: &mut Any) -> bool { invoke!(0x6862E4D93F64CF01, inventoryId, guid, p2, p3) }
	pub fn _SET_USE_MISSION_INVENTORY(toggle: bool) { invoke_ignore!(0x597F571DDEE3FFAC, toggle) }
	pub fn INVENTORY_DISABLE_MISSION_INVENTORY_PICKUPS() { invoke_ignore!(0xE1F389F03DC83673) }
	pub fn _INVENTORY_USE_MISSION_INVENTORY(enable: bool, mirrorTransactions: bool) { invoke_ignore!(0xA6AA9F56BC6CFF58, enable, mirrorTransactions) }
	pub fn INVENTORY_COPY_MP_INVENTORY_TO_MISSION_INVENTORY(p0: bool, p1: bool, bCopySatchelItems: bool, bCopyEmotes: bool, bCopyHorse: bool, p5: bool) { invoke_ignore!(0x644CCB76A76CFBD6, p0, p1, bCopySatchelItems, bCopyEmotes, bCopyHorse, p5) }
	pub fn _INVENTORY_COPY_ITEM_TO_MISSION_INVENTORY(guid: &mut Any, p1: bool) { invoke_ignore!(0x3112ADB9D5F3426B, guid, p1) }
	pub fn _0xE36D4A38D28D9CFB(p0: bool) { invoke_ignore!(0xE36D4A38D28D9CFB, p0) }
	pub fn _INVENTORY_USE_SP_BACKUP() -> bool { invoke!(0x7C7E4AB748EA3B07) }
	pub fn _INVENTORY_IS_PLAYER_INVENTORY_MIRRORING_TRANSACTIONS() -> bool { invoke!(0xFC7563F482781A3D) }
	pub fn _INVENTORY_COPY_ITEM_TO_INVENTORY(inventoryId: i32, inventoryIdCloned: i32, p2: &mut Any, p3: Any) { invoke_ignore!(0xC04F47D488EF9EBA, inventoryId, inventoryIdCloned, p2, p3) }
	pub fn _0x9E58207B194488AC(ped: Ped, p1: i32) { invoke_ignore!(0x9E58207B194488AC, ped, p1) }
	pub fn _SET_ITEM_PROMPT_INFO_REQUEST(p0: &mut Any) { invoke_ignore!(0xFD41D1D4350F6413, p0) }
	pub fn _0x9B4E793B1CB6550A() { invoke_ignore!(0x9B4E793B1CB6550A) }
	pub fn _SET_CARRIABLE_CARRY_ACTION_PROMPT_OVERRIDE(data: &mut Any) { invoke_ignore!(0xF666EF30F4F0AC4E, data) }
	pub fn _INVENTORY_ENABLE_WEAPONS(inventoryId: i32) { invoke_ignore!(0xD5D72F1624F3BA7C, inventoryId) }
	pub fn _INVENTORY_DISABLE_WEAPONS(inventoryId: i32, p1: Any) { invoke_ignore!(0xE3A46370F70F3607, inventoryId, p1) }
	pub fn _0xE1F45A67A9F0DCBC(inventoryId: i32) { invoke_ignore!(0xE1F45A67A9F0DCBC, inventoryId) }
	pub fn _0x6968CE7AC32F6788(inventoryId: i32) { invoke_ignore!(0x6968CE7AC32F6788, inventoryId) }
	pub fn _INVENTORY_CREATE_ITEM_COLLECTION_2(collectionSize: &mut i32) -> i32 { invoke!(0x97A3646645727F42, collectionSize) }
	pub fn _INVENTORY_CREATE_SORTED_COLLECTION(inventoryId: i32, p1: i32, size: &mut i32) -> i32 { invoke!(0xBB7F968675B34B0C, inventoryId, p1, size) }
}
pub mod ITEMDATABASE {
	use std::ffi::CStr;
	use crate::core::scripthook::native::*;
	use crate::core::types::*;

	pub fn _ITEMDATABASE_CAN_EQUIP_ITEM_ON_CATEGORY(key: Hash, category: Hash, slotId: Hash) -> bool { invoke!(0x856FF92C57742AE5, key, category, slotId) }
	pub fn _ITEMDATABASE_GET_FITS_SLOT_COUNT(category: Hash) -> i32 { invoke!(0x2970D1D6BFCF9B46, category) }
	pub fn _ITEMDATABASE_GET_FITS_SLOT_INFO(category: Hash, index: i32, outSlotId: &mut Hash) -> bool { invoke!(0x77210C146CED5261, category, index, outSlotId) }
	pub fn _ITEMDATABASE_GET_HAS_SLOT_COUNT(category: Hash) -> i32 { invoke!(0x44915068579D7710, category) }
	pub fn _ITEMDATABASE_GET_HAS_SLOT_INFO(category: Hash, index: i32, outSlotId: &mut Hash) -> bool { invoke!(0x8A9BD0DB7E8376CF, category, index, outSlotId) }
	pub fn ITEMDATABASE_FILLOUT_ITEM_BY_NAME(key: Hash, outData: &mut Any) -> bool { invoke!(0x2A610BEE7D341CC4, key, outData) }
	pub fn ITEMDATABASE_FILLOUT_ITEM_INFO(key: Hash, outData: &mut Any) -> bool { invoke!(0xFE90ABBCBFDC13B2, key, outData) }
	pub fn _ITEMDATABASE_FILLOUT_ACQUIRE_COST(key: Hash, costtype: Hash, outData: &mut Any) -> bool { invoke!(0x74F7928816E4E181, key, costtype, outData) }
	pub fn _ITEMDATABASE_FILLOUT_SELL_PRICE(key: Hash, sellType: Hash, outData: &mut Any) -> bool { invoke!(0x7A62A2EEDE1C3766, key, sellType, outData) }
	pub fn _ITEMDATABASE_FILLOUT_SATCHEL_DATA(key: Hash, outSatchelItemSize: &mut i32) -> bool { invoke!(0x4776EFD78F75C23F, key, outSatchelItemSize) }
	pub fn _ITEMDATABASE_FILLOUT_UI_DATA(key: Hash, outData: &mut Any) -> bool { invoke!(0xB86F7CC2DC67AC60, key, outData) }
	pub fn _ITEMDATABASE_FILLOUT_BUNDLE_UI_DATA(bundle: Hash, outData: &mut Any) -> bool { invoke!(0x74C3B1093728D263, bundle, outData) }
	pub fn _ITEMDATABASE_FILLOUT_ITEM_EFFECT_IDS(key: Hash, outData: &mut Any) -> bool { invoke!(0x9379BE60DC55BBE6, key, outData) }
	pub fn ITEMDATABASE_FILLOUT_ITEM_EFFECT_ID_INFO(key: Hash, outData: &mut Any) -> bool { invoke!(0xCF2D360D27FD1ABF, key, outData) }
	pub fn _ITEMDATABASE_FILLOUT_TAG_DATA(key: Hash, outData: &mut Any, outIndex: &mut i32, p3: i32) -> bool { invoke!(0x5A11D6EEA17165B0, key, outData, outIndex, p3) }
	pub fn _ITEMDATABASE_GET_ACQUIRE_COSTS_COUNT(key: Hash) -> i32 { invoke!(0x01FDDAD392D04144, key) }
	pub fn _ITEMDATABASE_GET_ACQUIRE_COST(key: Hash, index: i32, outData: &mut Any) -> bool { invoke!(0x6772A83C67A25775, key, index, outData) }
	pub fn _ITEMDATABASE_GET_ACQUIRE_COSTS_COUNT_FROM_COST_TYPE(key: Hash, costtype: Hash) -> i32 { invoke!(0xDEE7B3C76ED664BE, key, costtype) }
	pub fn _ITEMDATABASE_FILLOUT_ITEM(key: Hash, costtype: Hash, index: i32, outData: &mut Any) -> bool { invoke!(0xAD73B614DF26CF8A, key, costtype, index, outData) }
	pub fn _ITEMDATABASE_GET_AWARD_ACQUIRE_COST_COUNT(key: Hash) -> i32 { invoke!(0x12DF9C58201DD19A, key) }
	pub fn _0x1FC25AEB5F76B38D(award: Hash, index: i32, outData: &mut Any) -> bool { invoke!(0x1FC25AEB5F76B38D, award, index, outData) }
	pub fn _ITEMDATABASE_GET_AWARD_ACQUIRE_COST_COUNT_FROM_COST_TYPE(award: Hash, costtype: Hash) -> i32 { invoke!(0xF540239F9937033B, award, costtype) }
	pub fn _ITEMDATABASE_FILLOUT_AWARD_ACQUIRE_COST(award: Hash, costtype: Hash, index: i32, outData: &mut Any) -> bool { invoke!(0xF27F01BBF5ACD3F3, award, costtype, index, outData) }
	pub fn _ITEMDATABASE_CREATE_ITEM_COLLECTION(data: &mut Any, size: &mut i32, comparisonType: i32) -> i32 { invoke!(0x71EFA7999AE79408, data, size, comparisonType) }
	pub fn _ITEMDATABASE_GET_COLLECTION_SIZE(collectionId: i32) -> i32 { invoke!(0xD389A2549C4EFB30, collectionId) }
	pub fn _ITEMDATABASE_GET_COMPONENT_ITEM(collectionId: i32, index: i32, outKey: &mut Hash) -> bool { invoke!(0x8750F69A720C2E41, collectionId, index, outKey) }
	pub fn _ITEMDATABASE_RELEASE_ITEM_COLLECTION(collectionId: i32) -> bool { invoke!(0xCBB7B6EDFA933ADE, collectionId) }
	pub fn _ITEMDATABASE_IS_INTRINSIC_ITEM(key: Hash) -> bool { invoke!(0x337F88E3A063995E, key) }
	pub fn _ITEMDATABASE_IS_OVERPOWERED_ITEM(key: Hash) -> bool { invoke!(0x337F88E3A063995F, key) }
	pub fn _ITEMDATABASE_IS_KEY_VALID(key: Hash, mode: i32) -> bool { invoke!(0x6D5D51B188333FD1, key, mode) }
	pub fn _0x537A0555F62CA01A(key: Hash, p1: i32) -> bool { invoke!(0x537A0555F62CA01A, key, p1) }
	pub fn _ITEMDATABASE_GET_BUNDLE_ID(bundle: Hash) -> i32 { invoke!(0x891A45960B6B768A, bundle) }
	pub fn _ITEMDATABASE_GET_BUNDLE_ITEM_COUNT(bundleId: i32, data: &mut Any) -> i32 { invoke!(0x3332695B01015DF9, bundleId, data) }
	pub fn _ITEMDATABASE_GET_BUNDLE_ITEM_INFO(bundleId: i32, data: &mut Any, index: i32, outBundle: &mut Hash) -> bool { invoke!(0x5D48A77E4B668B57, bundleId, data, index, outBundle) }
	pub fn _ITEMDATABASE_IS_BUNDLE_VALID(bundle: Hash, mode: i32) -> bool { invoke!(0x4308812A6E9CA62E, bundle, mode) }
	pub fn _0x799FCD53358ED5FA(bundle: Any, p1: Any) -> i32 { invoke!(0x799FCD53358ED5FA, bundle, p1) }
	pub fn _0xC4146375D8A0B374(bundle: Any, p1: Any, index: i32, p3: Any) -> bool { invoke!(0xC4146375D8A0B374, bundle, p1, index, p3) }
	pub fn _ITEMDATABASE_GET_BUNDLE_ACQUIRE_COST_MODIFIERS(bundle: Hash, outData: &mut Any) -> bool { invoke!(0xA97EE5E4589FCF5A, bundle, outData) }
	pub fn _0x7A35A72A692BE9DB(p0: Any) -> i32 { invoke!(0x7A35A72A692BE9DB, p0) }
	pub fn _0x3A0B667ABFF87F6E(p0: Any, p1: Any, p2: Any) -> bool { invoke!(0x3A0B667ABFF87F6E, p0, p1, p2) }
	pub fn _0x388088BFF3681189(bundle: Hash, costtype: Hash) -> i32 { invoke!(0x388088BFF3681189, bundle, costtype) }
	pub fn _ITEMDATABASE_FILLOUT_BUNDLE(bundle: Hash, costtype: Hash, index: i32, outData: &mut Any) -> bool { invoke!(0xB542632693D53408, bundle, costtype, index, outData) }
	pub fn _ITEMDATABASE_IS_SHOP_KEY_VALID(shopType: Hash) -> bool { invoke!(0x00B9507D8E1D8716, shopType) }
	pub fn _ITEMDATABASE_GET_SHOP_INVENTORIES_ITEMS_COUNT(shopType: Hash) -> i32 { invoke!(0xC568B1A0F17C7025, shopType) }
	pub fn _ITEMDATABASE_GET_SHOP_INVENTORIES_ITEM_INFO(shopType: Hash, index: i32, outData: &mut Any) -> bool { invoke!(0x4A79B41B4EB91F4E, shopType, index, outData) }
	pub fn _ITEMDATABASE_GET_SHOP_INVENTORIES_ITEM_INFO_BY_KEY(shopType: Hash, key: Hash, outData: &mut Any) -> bool { invoke!(0xCFB06801F5099B25, shopType, key, outData) }
	pub fn _ITEMDATABASE_GET_SHOP_INVENTORIES_REQUIREMENT_GROUP_INFO(shopType: Hash, key: Hash, index: i32, outData: &mut Any) -> bool { invoke!(0x76C752D788A76813, shopType, key, index, outData) }
	pub fn _ITEMDATABASE_GET_SHOP_INVENTORIES_REQUIREMENT_INFO(shopType: Hash, key: Hash, groupIndex: i32, index: i32, outData: &mut Any) -> bool { invoke!(0xE0EA5C031AE5539F, shopType, key, groupIndex, index, outData) }
	pub fn _0x17721003A66C72BF(shopType: Hash, key: Hash, outData: &mut Any) -> bool { invoke!(0x17721003A66C72BF, shopType, key, outData) }
	pub fn _ITEMDATABASE_IS_SHOP_LAYOUT_KEY_VALID(layout: Hash) -> bool { invoke!(0x3AFE5182C45A84F6, layout) }
	pub fn _ITEMDATABASE_GET_SHOP_LAYOUT_INFO(layout: Hash, outData: &mut Any) -> bool { invoke!(0x66A6D76B6BB999B4, layout, outData) }
	pub fn _ITEMDATABASE_GET_SHOP_LAYOUT_ROOT_MENU_INFO(layout: Hash, index: i32, outData: &mut Any) -> bool { invoke!(0x86FCB565CCA0CFA7, layout, index, outData) }
	pub fn _ITEMDATABASE_GET_SHOP_LAYOUT_MENU_INFO_BY_ID(layout: Hash, menu: Hash, outData: &mut Any) -> bool { invoke!(0xD66114469978B55B, layout, menu, outData) }
	pub fn _ITEMDATABASE_GET_SHOP_LAYOUT_MENU_INFO_BY_INDEX(layout: Hash, menu: Hash, index: i32, outData: &mut Any) -> bool { invoke!(0xF04247092F193B75, layout, menu, index, outData) }
	pub fn _ITEMDATABASE_GET_SHOP_LAYOUT_MENU_PAGE_KEY(layout: Hash, menu: Hash, index: i32, outPageKey: &mut Hash) -> bool { invoke!(0x9A60570657A7B635, layout, menu, index, outPageKey) }
	pub fn _0xDBEADA0DF5F9AB9F(layout: Hash, index: i32, outMenuKey: &mut i32) -> bool { invoke!(0xDBEADA0DF5F9AB9F, layout, index, outMenuKey) }
	pub fn _ITEMDATABASE_GET_SHOP_LAYOUT_PAGE_INFO_BY_KEY(layout: Hash, pageKey: Hash, outData: &mut Any) -> bool { invoke!(0xB347C100DF0C9B7F, layout, pageKey, outData) }
	pub fn _ITEMDATABASE_GET_SHOP_LAYOUT_PAGE_ITEM_KEY(layout: Hash, pageKey: Hash, index: i32, outItemKey: &mut Hash, outMenuId: &mut i32, outLayout: &mut Hash) -> bool { invoke!(0xF32BEF578B3DBAE8, layout, pageKey, index, outItemKey, outMenuId, outLayout) }
	pub fn ITEMDATABASE_IS_BUYABLE_AWARD_VALID(award: Hash) -> bool { invoke!(0x4CE753203FA42214, award) }
	pub fn _ITEMDATABASE_FILLOUT_BUY_AWARD_ACQUIRE_COSTS(award: Hash, outData: &mut Any, outUnk: &mut i32, p3: i32) -> bool { invoke!(0xB52E20F6767A09A2, award, outData, outUnk, p3) }
	pub fn _ITEMDATABASE_FILLOUT_BUY_AWARD_UI_DATA(award: Hash, outData: &mut Any) -> bool { invoke!(0xF8D09EF8CE61D7BF, award, outData) }
	pub fn _ITEMDATABASE_GET_ITEM_PRICE_MODIFIERS(key: Hash, outData: &mut Any) -> bool { invoke!(0x4EB37AAB79AB0C48, key, outData) }
	pub fn _ITEMDATABASE_FILLOUT_PRICE_MODIFIER_BY_KEY(key: Hash, outData: &mut Any) -> bool { invoke!(0x40C5D95818823C94, key, outData) }
	pub fn _ITEMDATABASE_GET_NUMBER_OF_MODIFIED_PRICES(key: Hash) -> i32 { invoke!(0x5AAAF40E9B224F5E, key) }
	pub fn _ITEMDATABASE_GET_MODIFIED_PRICE(key: Hash, index: i32) -> Hash { invoke!(0xCB92EC9C004732B4, key, index) }
	pub fn _ITEMDATABASE_GET_NUMBER_OF_MODIFIERS(key: Hash) -> i32 { invoke!(0x1289D8315235856D, key) }
	pub fn _ITEMDATABASE_FILLOUT_MODIFIER(key: Hash, index: i32, outData: &mut Any) -> bool { invoke!(0x60614A0AB580A2B5, key, index, outData) }
	pub fn _ITEMDATABASE_GET_PRIORITY_ACCESS_AWARD(award: Hash) -> bool { invoke!(0xEF254F1A4C08B7E6, award) }
	pub fn _ITEMDATABASE_GET_AWARD_ITEM_COUNT(award: Hash) -> i32 { invoke!(0x3FAA928A79591761, award) }
	pub fn _0x48229CE0C7938237(award: Hash) -> i32 { invoke!(0x48229CE0C7938237, award) }
	pub fn _ITEMDATABASE_FILLOUT_AWARD_ITEM_INFO(award: Hash, index: i32, outData: &mut Hash) -> bool { invoke!(0x121D2005DD64496B, award, index, outData) }
	pub fn _0x8D029948CA29409B(award: Hash, index: i32, outData: &mut Any) -> bool { invoke!(0x8D029948CA29409B, award, index, outData) }
	pub fn _ITEMDATABASE_GET_AWARD_COST_MODIFIERS(award: Hash, outData: &mut Any) -> bool { invoke!(0xE81D0378A384E755, award, outData) }
	pub fn _0xD076DB9B96FAADF1(award: Hash, outData: &mut Any) -> bool { invoke!(0xD076DB9B96FAADF1, award, outData) }
	pub fn _ITEMDATABASE_DOES_ITEM_HAVE_TAG(item: Hash, tag: Hash, tagType: Hash) -> bool { invoke!(0xFF5FB5605AD56856, item, tag, tagType) }
	pub fn _ITEMDATABASE_DOES_BUNDLE_HAVE_TAG(bundle: Hash, tag: Hash, tagType: Hash) -> bool { invoke!(0x99C6EA66DFE73757, bundle, tag, tagType) }
	pub fn _ITEMDATABASE_GET_ITEM_TAG_TYPE(item: Hash, tag: Hash) -> Hash { invoke!(0x6111B8F9413F413A, item, tag) }
	pub fn _0x8870895BA5ED9385(key: Hash, tagType: Hash, outData: &mut Any) -> i32 { invoke!(0x8870895BA5ED9385, key, tagType, outData) }
	pub fn _ITEMDATABASE_LOCALIZATION_GET_NUM_LABEL_TYPES(p0: Any) -> i32 { invoke!(0xCEC6A41E8910486A, p0) }
	pub fn _ITEMDATABASE_LOCALIZATION_GET_NUM_VALUES(p0: Any, p1: Any) -> i32 { invoke!(0x49885D82A13EEAEA, p0, p1) }
	pub fn _ITEMDATABASE_LOCALIZATION_GET_TYPE(p0: Any, p1: Any) -> Any { invoke!(0xCABF5D41D0073D4A, p0, p1) }
	pub fn _ITEMDATABASE_LOCALIZATION_GET_VALUE(p0: Any, label: Hash, p2: Any) -> i32 { invoke!(0x9AE5610FDCED6EA7, p0, label, p2) }
	pub fn _ITEMDATABASE_GET_ITEM_PATHSET(key: Hash, defaultPathset: Hash) -> i32 { invoke!(0xF4452CE83118C738, key, defaultPathset) }
	pub fn _0xAA29A5F13B2C20B2(p0: Any, p1: Hash) -> Hash { invoke!(0xAA29A5F13B2C20B2, p0, p1) }
}
pub mod ITEMSET {
	use std::ffi::CStr;
	use crate::core::scripthook::native::*;
	use crate::core::types::*;

	pub fn CREATE_ITEMSET(p0: bool) -> ItemSet { invoke!(0xA1AF16083320065A, p0) }
	pub fn DESTROY_ITEMSET(itemset: ItemSet) { invoke_ignore!(0x712BC69F10549B92, itemset) }
	pub fn IS_ITEMSET_VALID(itemset: ItemSet) -> bool { invoke!(0xD30765D153EF5C76, itemset) }
	pub fn ADD_TO_ITEMSET(entity: Entity, itemset: ItemSet) -> bool { invoke!(0xABE74510883C7950, entity, itemset) }
	pub fn REMOVE_FROM_ITEMSET(entity: Entity, itemset: ItemSet) { invoke_ignore!(0xC5BAA432B429DC24, entity, itemset) }
	pub fn GET_ITEMSET_SIZE(itemset: ItemSet) -> i32 { invoke!(0x55F2E375AC6018A9, itemset) }
	pub fn GET_INDEXED_ITEM_IN_ITEMSET(index: i32, itemset: ItemSet) -> ScrHandle { invoke!(0x275A2E2C0FAB7612, index, itemset) }
	pub fn GET_INDEXED_SCENARIO_POINT_INDEX_IN_ITEMSET(index: i32, itemset: ItemSet) -> Any { invoke!(0x9FC3CDB5CE815901, index, itemset) }
	pub fn IS_IN_ITEMSET(entity: Entity, itemset: ItemSet) -> bool { invoke!(0xD1503C2EE2FE688C, entity, itemset) }
	pub fn CLEAN_ITEMSET(itemset: ItemSet) { invoke_ignore!(0x85F3A86CA9021FB0, itemset) }
	pub fn _CLEAR_ITEMSET(itemset: ItemSet) { invoke_ignore!(0x20A4BF0E09BEE146, itemset) }
}
pub mod LAW {
	use std::ffi::CStr;
	use crate::core::scripthook::native::*;
	use crate::core::types::*;

	pub fn _REPORT_CRIME(player: Player, crimeType: Hash, bounty: i32, entity: Entity, isKnownSuspect: bool) { invoke_ignore!(0xF60386770878A98F, player, crimeType, bounty, entity, isKnownSuspect) }
	pub fn SUPPRESS_CRIME_THIS_FRAME(player: Player, crimeType: Hash, p2: i32, p3: i32, p4: i32) { invoke_ignore!(0x785177E4D57D7389, player, crimeType, p2, p3, p4) }
	pub fn NUM_CRIMES_SUPPRESSED(player: Player, crimeType: Hash) -> i32 { invoke!(0xC08E804C91F47C80, player, crimeType) }
	pub fn _0xFFEBE5AA96BC2E4E(ped: Ped, crimeType: Hash, p2: bool) -> Any { invoke!(0xFFEBE5AA96BC2E4E, ped, crimeType, p2) }
	pub fn _0x15ABD5004CAD2D99(p0: i32) { invoke_ignore!(0x15ABD5004CAD2D99, p0) }
	pub fn _0xF611DE44AEB36A1D(crimeType: Hash, p1: bool) { invoke_ignore!(0xF611DE44AEB36A1D, crimeType, p1) }
	pub fn _0x3D2674828A4E6B3C() -> bool { invoke!(0x3D2674828A4E6B3C) }
	pub fn _0xC5EB2755FA25F1E9(p0: bool) { invoke_ignore!(0xC5EB2755FA25F1E9, p0) }
	pub fn GET_BOUNTY(player: Player) -> i32 { invoke!(0x54310AAB97B92816, player) }
	pub fn ADD_BOUNTY(player: Player, itemValueAmount: i32) { invoke_ignore!(0x0E3BDEED21BEB945, player, itemValueAmount) }
	pub fn SET_BOUNTY(player: Player, amount: i32) { invoke_ignore!(0x093A9D1F72DF0D19, player, amount) }
	pub fn CLEAR_BOUNTY(player: Player) { invoke_ignore!(0xC76F252371150D9A, player) }
	pub fn GET_WANTED_SCORE(player: Player) -> i32 { invoke!(0xDD5FD601481F648B, player) }
	pub fn SET_WANTED_SCORE(player: Player, intensity: i32) { invoke_ignore!(0xA80FF73F772ACF6A, player, intensity) }
	pub fn CLEAR_WANTED_SCORE(player: Player) { invoke_ignore!(0x062B4A4A3396351D, player) }
	pub fn _0x331D349E0380B097(p0: Any) { invoke_ignore!(0x331D349E0380B097, p0) }
	pub fn _0x292AD61A33A7A485() { invoke_ignore!(0x292AD61A33A7A485) }
	pub fn _0x07E8B8B20570271C(player: Player) { invoke_ignore!(0x07E8B8B20570271C, player) }
	pub fn _0x22741652985C84D0(player: Player, lawRegionHash: Hash) { invoke_ignore!(0x22741652985C84D0, player, lawRegionHash) }
	pub fn _REPORT_PLAYER_LAW_DISPATCH_RESPONSE_OVERRIDE(player: Player, dispatchResponseHash: Hash) { invoke_ignore!(0x9C4352134B2835FB, player, dispatchResponseHash) }
	pub fn _0x9EF07CFBB19A9733() -> bool { invoke!(0x9EF07CFBB19A9733) }
	pub fn _SET_LAW_DISABLED(toggle: bool) { invoke_ignore!(0x8DE82BC774F3B862, toggle) }
	pub fn _FORCE_LAW_ON_LOCAL_PLAYER_IMMEDIATELY() { invoke_ignore!(0x956510F8C36B5C64) }
	pub fn _SET_LAW_REGION(player: Player, lawRegionHash: Hash, stateHash: Hash) { invoke_ignore!(0x4752F68EB7F2D280, player, lawRegionHash, stateHash) }
	pub fn _0x5E6F375CA101C108(player: Player, p1: bool) { invoke_ignore!(0x5E6F375CA101C108, player, p1) }
	pub fn _SET_LAW_RBS_VOLUME(player: Player, p1: Hash) { invoke_ignore!(0x9BBDCB8DF789EBC1, player, p1) }
	pub fn IS_LAW_INCIDENT_ACTIVE(player: Player) -> bool { invoke!(0xAD401C63158ACBAA, player) }
	pub fn _0x148E7AC8141C9E64(player: Player) -> Hash { invoke!(0x148E7AC8141C9E64, player) }
	pub fn _0xEDFC6C1FD1C964F5(player: Player, crimeType: Hash, bounty: i32, p3: f32, p4: f32, p5: bool, p6: f32, p7: f32, p8: Any) { invoke_ignore!(0xEDFC6C1FD1C964F5, player, crimeType, bounty, p3, p4, p5, p6, p7, p8) }
	pub fn _LAW_WITNESS_RESPONSE_TASK(pedGroup1: Ped, ped: Ped, pedGroup2: Ped, x: f32, y: f32, z: f32, crimeType: Hash) -> bool { invoke!(0xF0B67BAD53C35BD9, pedGroup1, ped, pedGroup2, x, y, z, crimeType) }
	pub fn _0x018F30D762E62DF8(ped: Ped, p1: &mut Any) -> Any { invoke!(0x018F30D762E62DF8, ped, p1) }
	pub fn _0x318F0F9A4426CFA2(ped: Ped, p1: &mut Any) -> Any { invoke!(0x318F0F9A4426CFA2, ped, p1) }
	pub fn _0x95878B13E272EF1F(entity: Entity, ped: Ped, p2: bool, x: f32, y: f32, z: f32, crimeType: Hash) -> Any { invoke!(0x95878B13E272EF1F, entity, ped, p2, x, y, z, crimeType) }
	pub fn _ADD_WITNESS_RESPONSE(player: Player, crimeType: Hash, pedGroup: Ped) { invoke_ignore!(0x10827B5A0AAC56A7, player, crimeType, pedGroup) }
	pub fn _0xD7494DED50C6EF52(player: Player, crimeType: Hash, p2: i32) { invoke_ignore!(0xD7494DED50C6EF52, player, crimeType, p2) }
	pub fn ARE_WITNESSES_ACTIVE(player: Player) -> bool { invoke!(0x69E181772886F48B, player) }
	pub fn _ARE_WITNESSES_PENDING(player: Player) -> bool { invoke!(0x0BB6DE7D23C60626, player) }
	pub fn _ARE_INVESTIGATORS_ACTIVE(player: Player, areInvestigatorsActive: bool, p2: Any) -> bool { invoke!(0xF0FBFB9AB15F7734, player, areInvestigatorsActive, p2) }
	pub fn _0x522F74636DF10201(player: Player, itemSet: ItemSet) { invoke_ignore!(0x522F74636DF10201, player, itemSet) }
	pub fn _0xDA1A9ADC4E3D4B16(itemSet: ItemSet, p1: bool, p2: bool) { invoke_ignore!(0xDA1A9ADC4E3D4B16, itemSet, p1, p2) }
	pub fn _ENABLE_DISPATCH_LAW(toggle: bool) { invoke_ignore!(0xC805EB785824F712, toggle) }
	pub fn _ENABLE_DISPATCH_LAW_2(toggle: bool) { invoke_ignore!(0x710448D44A64C213, toggle) }
	pub fn _ARE_ANY_LAW_PEDS_INVESTIGATING() -> bool { invoke!(0xECE3C34B270428D5) }
	pub fn _ARE_LAW_PEDS_ENABLED_FOR_TRAIN() -> bool { invoke!(0xA22C46F16359471C) }
	pub fn _0x82F11E1296996574(p0: i32) { invoke_ignore!(0x82F11E1296996574, p0) }
	pub fn _0x3852237A3D9DF145(p0: i32) { invoke_ignore!(0x3852237A3D9DF145, p0) }
	pub fn _0x0EAF918F751F27BA(ped: Ped) -> bool { invoke!(0x0EAF918F751F27BA, ped) }
	pub fn _0xC0DF161950FB101E(ped: Ped) -> bool { invoke!(0xC0DF161950FB101E, ped) }
	pub fn _0xE4D6E45F491A66CB(player: Player, p1: i32) -> Any { invoke!(0xE4D6E45F491A66CB, player, p1) }
	pub fn _0xE9EB79CBF9C0F58A(player: Player) -> i32 { invoke!(0xE9EB79CBF9C0F58A, player) }
	pub fn _0x21213B833EF4DAE7(player: Player, ped: Ped, outCoords: &mut Vector3) { invoke_ignore!(0x21213B833EF4DAE7, player, ped, outCoords) }
	pub fn _0x61B98367D93F012F(player: Player) { invoke_ignore!(0x61B98367D93F012F, player) }
	pub fn _0x6ABC50979655BEE7(player: Player, crimeType: &mut Hash, p2: Any) { invoke_ignore!(0x6ABC50979655BEE7, player, crimeType, p2) }
	pub fn _0x390710D2DAFA6BFF(player: Player, p1: bool) { invoke_ignore!(0x390710D2DAFA6BFF, player, p1) }
	pub fn _GET_CRIME_BOUNTY_AMOUNT_BY_TYPE(crimeType: Hash) -> i32 { invoke!(0x35E5E21F9159849C, crimeType) }
	pub fn _0xDAEFDFDB2AEECE37(crimeType: Hash, p1: Any) -> i32 { invoke!(0xDAEFDFDB2AEECE37, crimeType, p1) }
	pub fn _GET_HUD_PLAYER_CRIME_TYPE(player: Player) -> Hash { invoke!(0x259CE340A8738814, player) }
	pub fn _0xE083BEDA81709891(player: Player) -> i32 { invoke!(0xE083BEDA81709891, player) }
	pub fn _0x89E005B1662F6E48(player: Player, p1: i32, p2: i32) -> bool { invoke!(0x89E005B1662F6E48, player, p1, p2) }
	pub fn _0x3738B784DDD35CC6(player: Player, p1: i32, p2: i32) -> bool { invoke!(0x3738B784DDD35CC6, player, p1, p2) }
	pub fn _0x0BDFEBCF40A5F7E3(crimeType: Hash) -> i32 { invoke!(0x0BDFEBCF40A5F7E3, crimeType) }
	pub fn _SET_CUSTOM_LAW_DISPATCH_RESPONSE(dispatchResponseHash: Hash) { invoke_ignore!(0x009CF9A29972C298, dispatchResponseHash) }
	pub fn _CREATE_LAW_DISPATCH_RESPONSE_FOR_COORDS(x: f32, y: f32, z: f32, dispatchResponseHash: Hash) -> Any { invoke!(0x75CBF20BA47E4F89, x, y, z, dispatchResponseHash) }
	pub fn _SET_BOUNTY_HUNTER_PURSUIT_CLEARED() { invoke_ignore!(0x55F37F5F3F2475E1) }
	pub fn _0xBD944A3D36E992DE() { invoke_ignore!(0xBD944A3D36E992DE) }
	pub fn _0x987BE590FB9D41E5(p0: bool) { invoke_ignore!(0x987BE590FB9D41E5, p0) }
	pub fn _0xDCF12B89624AAC96(p0: bool) { invoke_ignore!(0xDCF12B89624AAC96, p0) }
	pub fn _0xDDCE8E960D1DE240(p0: bool) { invoke_ignore!(0xDDCE8E960D1DE240, p0) }
	pub fn _0xDEA083C16BB91345() { invoke_ignore!(0xDEA083C16BB91345) }
	pub fn _0x9C8A2BF37E966464(player: Player, itemSet: ItemSet) { invoke_ignore!(0x9C8A2BF37E966464, player, itemSet) }
	pub fn _0x9D5C9A5A3321B128(player: Player) -> bool { invoke!(0x9D5C9A5A3321B128, player) }
	pub fn _0x0F230DE0DDBE3649(player: Player) -> bool { invoke!(0x0F230DE0DDBE3649, player) }
	pub fn _0x9945A3E2528A02E8(player: Player) -> bool { invoke!(0x9945A3E2528A02E8, player) }
	pub fn _GET_TIME_SINCE_LAST_SEEN_BY_LAW(player: Player) -> f32 { invoke!(0x717DA2281DF90855, player) }
	pub fn _0x9B4C564BFA7CFF37(p0: f32) { invoke_ignore!(0x9B4C564BFA7CFF37, p0) }
	pub fn _SET_ALLOW_DISABLED_LAW_RESPONSES(toggle: bool) { invoke_ignore!(0x4B52BF96E225D230, toggle) }
	pub fn RESET_WANTED_FOR_NEW_INCIDENT(player: Player) { invoke_ignore!(0x2728C77FBC4B9796, player) }
	pub fn _0x856CE8FDE2416602(ped: Ped) -> bool { invoke!(0x856CE8FDE2416602, ped) }
	pub fn _0x7351DA734F989F4E(entity: Entity) -> bool { invoke!(0x7351DA734F989F4E, entity) }
	pub fn _GET_BOUNTY_HUNTER_GLOBAL_COOLDOWN(p0: Hash) -> i32 { invoke!(0x76CF93D4B416B288, p0) }
	pub fn _SET_BOUNTY_HUNTER_GLOBAL_COOLDOWN(p0: Hash, p1: i32) { invoke_ignore!(0xF19706B1F8FFA88F, p0, p1) }
	pub fn _0x2001687F9562FD9D(p0: Any) { invoke_ignore!(0x2001687F9562FD9D, p0) }
	pub fn _PAUSE_BOUNTY_HUNTER_COOLDOWN(p0: Hash, p1: bool, p2: Any) { invoke_ignore!(0xC61EDEBF16CD9668, p0, p1, p2) }
	pub fn _0xE9AC8466ABE484BB(p0: bool, p1: Any) { invoke_ignore!(0xE9AC8466ABE484BB, p0, p1) }
	pub fn _0x40851BCC33ACD9AB(ped: Ped) -> bool { invoke!(0x40851BCC33ACD9AB, ped) }
	pub fn _0xF46108C50A22B029() -> bool { invoke!(0xF46108C50A22B029) }
	pub fn _0x7803436E68C32B26() { invoke_ignore!(0x7803436E68C32B26) }
	pub fn _0xC310239ACCCF5579() { invoke_ignore!(0xC310239ACCCF5579) }
	pub fn _0x29CD4896ECB66C12() { invoke_ignore!(0x29CD4896ECB66C12) }
	pub fn _0xE94B5E938619712E() { invoke_ignore!(0xE94B5E938619712E) }
	pub fn _0x7FC667F6DDFBCDCC(player: Player) -> i32 { invoke!(0x7FC667F6DDFBCDCC, player) }
	pub fn _0x9C5BD8C562565CE6(crimeType: &mut Hash) { invoke_ignore!(0x9C5BD8C562565CE6, crimeType) }
	pub fn _0xCBFB4951F2E3934C(player: Player, data: &mut Any) { invoke_ignore!(0xCBFB4951F2E3934C, player, data) }
	pub fn _SET_PED_LAW_BEHAVIOUR(ped: Ped, behaviour: i32) { invoke_ignore!(0x819ADD5EF1742F47, ped, behaviour) }
	pub fn _0x00DB0BC05E3FAA4E(ped: Ped, bitset: i32) { invoke_ignore!(0x00DB0BC05E3FAA4E, ped, bitset) }
	pub fn _0x0C392DB374655176(x: f32, y: f32, z: f32, p3: f32, itemSet: ItemSet) { invoke_ignore!(0x0C392DB374655176, x, y, z, p3, itemSet) }
	pub fn _0xC687A23E166DCF68(p0: &mut Any) -> Any { invoke!(0xC687A23E166DCF68, p0) }
	pub fn _SET_DISPATCH_MULTIPLIER_OVERRIDE(multiplier: f32) { invoke_ignore!(0x002BABE0B7D53136, multiplier) }
	pub fn _0x26934083D3F2579C(player: Player) -> bool { invoke!(0x26934083D3F2579C, player) }
	pub fn GET_PLAYER_REGISTERED_CRIME(player: Player, p1: i32, crimeType: &mut Hash) -> bool { invoke!(0x532C5FDDB986EE5C, player, p1, crimeType) }
	pub fn _0xB527099D1E1EED49(player: Player, p1: i32, crimeType: &mut Hash) -> bool { invoke!(0xB527099D1E1EED49, player, p1, crimeType) }
	pub fn CLEAR_PLAYER_PAST_CRIMES(player: Player) { invoke_ignore!(0xBCC6DC59E32A2BDC, player) }
	pub fn SET_PLAYER_ARRESTED_IN_REGION(player: Player, lawRegionHash: Hash) { invoke_ignore!(0xE0FA74AA3CCE650B, player, lawRegionHash) }
	pub fn SET_PLAYER_TURNED_IN_BOUNTY_IN_REGION(player: Player, lawRegionHash: Hash) { invoke_ignore!(0x73BAD7B2F2DB50DE, player, lawRegionHash) }
	pub fn _0xD6C0A8C7C0B2F82C(player: Player, p1: bool) { invoke_ignore!(0xD6C0A8C7C0B2F82C, player, p1) }
	pub fn SET_LAW_SENSE_RANGE_MODIFIER(player: Player, range: f32) { invoke_ignore!(0xFEC85339AACA2A35, player, range) }
	pub fn _0x7EF2A2FE38D74456(flag: i32, p1: bool) { invoke_ignore!(0x7EF2A2FE38D74456, flag, p1) }
	pub fn _0xC7DC5A0A7DF608CB(flag: i32) -> bool { invoke!(0xC7DC5A0A7DF608CB, flag) }
	pub fn SET_DISABLE_DISTURBANCE_CRIMES(player: Player, p1: bool) { invoke_ignore!(0xDE5FAA741A781F73, player, p1) }
	pub fn SET_POSTPONE_DISTURBANCE_CRIMES_DURING_COMBAT(player: Player, p1: bool) { invoke_ignore!(0x362086B911657B1A, player, p1) }
	pub fn _CREATE_GUARD_ZONE(name: & CStr) { invoke_ignore!(0x8F9DE75680275C9F, name) }
	pub fn _REMOVE_GUARD_ZONE(name: & CStr) { invoke_ignore!(0x67EBDD958835956C, name) }
	pub fn _0x9772395CC73E8D1F(ped: Ped, name: & CStr) { invoke_ignore!(0x9772395CC73E8D1F, ped, name) }
	pub fn _DISABLE_GUARD_ZONE(name: & CStr) { invoke_ignore!(0x26D558692B25DD95, name) }
	pub fn _0x0DBACA9C38C9A686(name: & CStr) -> bool { invoke!(0x0DBACA9C38C9A686, name) }
	pub fn _IS_GUARD_PED_INVESTIGATING(ped: Ped) -> bool { invoke!(0xD743C4293F47AFAD, ped) }
	pub fn _CREATE_GUARD_ZONE_FOR_ENTITY(guardZoneName: & CStr, entity: Entity, x: f32, y: f32, z: f32) -> bool { invoke!(0x0D4B77E862475ED3, guardZoneName, entity, x, y, z) }
	pub fn _SET_GUARD_ZONE_VOLUME_REGISTRATION_START(name: & CStr, volume: Volume) { invoke_ignore!(0x8C598A930F471938, name, volume) }
	pub fn _SET_GUARD_ZONE_VOLUME_RESTRICTED(name: & CStr, volume: Volume) { invoke_ignore!(0x35815F372D43E1E5, name, volume) }
	pub fn _SET_GUARD_ZONE_VOLUME_THREAT(name: & CStr, volume: Volume) { invoke_ignore!(0xA1B0E6301E2E02A6, name, volume) }
	pub fn _SET_GUARD_ZONE_VOLUME_WARNING(name: & CStr, volume: Volume) { invoke_ignore!(0xAD3E07C37A7C1ADC, name, volume) }
	pub fn _SET_GUARD_ZONE_VOLUME_REGISTRATION_END(name: & CStr, volume: Volume) { invoke_ignore!(0xA8A74AA79FB67159, name, volume) }
	pub fn _SET_GUARD_ZONE_POSITION(name: & CStr, x: f32, y: f32, z: f32) { invoke_ignore!(0x7E7BF59F89FC6C6D, name, x, y, z) }
	pub fn _SET_GUARD_ZONE_POSITION_2(name: & CStr, x: f32, y: f32, z: f32) { invoke_ignore!(0x2F9005E2EA4E5EE4, name, x, y, z) }
}
pub mod LOCALIZATION {
	use std::ffi::CStr;
	use crate::core::scripthook::native::*;
	use crate::core::types::*;

	pub fn LOCALIZATION_GET_SYSTEM_LANGUAGE() -> i32 { invoke!(0x3C1A05F86AE6ACB5) }
	pub fn GET_CURRENT_LANGUAGE() -> i32 { invoke!(0xDB917DA5C6835FCC) }
	pub fn _DOES_CURRENT_LANGUAGE_SUPPORT_CONDENSED_STYLE() -> bool { invoke!(0x45D50415E4D885FF) }
	pub fn LOCALIZATION_GET_SYSTEM_DATE_TYPE() -> i32 { invoke!(0x76E30B799EBEEA0F) }
}
pub mod MAP {
	use std::ffi::CStr;
	use crate::core::scripthook::native::*;
	use crate::core::types::*;

	pub fn GET_BLIP_FROM_ENTITY(entity: Entity) -> Blip { invoke!(0x6D2C41A8BD6D6FD0, entity) }
	pub fn GET_MAIN_PLAYER_BLIP_ID() -> Blip { invoke!(0x5CD2889B2B381D45) }
	pub fn _BLIP_ADD_FOR_STYLE(styleHash: Hash) -> Blip { invoke!(0x3E593DF9C2962EC6, styleHash) }
	pub fn BLIP_ADD_FOR_COORDS(blipHash: Hash, x: f32, y: f32, z: f32) -> Blip { invoke!(0x554D9D53F696D002, blipHash, x, y, z) }
	pub fn BLIP_ADD_FOR_ENTITY(blipHash: Hash, entity: Entity) -> Blip { invoke!(0x23F74C2FDA6E7C61, blipHash, entity) }
	pub fn BLIP_ADD_FOR_PICKUP_PLACEMENT(blipHash: Hash, pickup: Pickup) -> Blip { invoke!(0xA486008892065FB9, blipHash, pickup) }
	pub fn BLIP_ADD_FOR_RADIUS(blipHash: Hash, x: f32, y: f32, z: f32, radius: f32) -> Blip { invoke!(0x45F13B7E0A15C880, blipHash, x, y, z, radius) }
	pub fn _BLIP_ADD_FOR_AREA(blipHash: Hash, x: f32, y: f32, z: f32, scaleX: f32, scaleY: f32, scaleZ: f32, p7: i32) -> Blip { invoke!(0xEC174ADBCB611ECC, blipHash, x, y, z, scaleX, scaleY, scaleZ, p7) }
	pub fn _BLIP_ADD_FOR_VOLUME(blipHash: Hash, volume: Volume) -> Blip { invoke!(0xA6EF0C54A3443E70, blipHash, volume) }
	pub fn _BLIP_SET_STYLE(blip: Blip, styleHash: Hash) -> bool { invoke!(0xEDD964B7984AC291, blip, styleHash) }
	pub fn _BLIP_ADD_STYLE(blip: Blip, styleHash: Hash) -> bool { invoke!(0xBD62D98799A3DAF0, blip, styleHash) }
	pub fn BLIP_ADD_MODIFIER(blip: Blip, modifierHash: Hash) -> bool { invoke!(0x662D364ABF16DE2F, blip, modifierHash) }
	pub fn BLIP_REMOVE_MODIFIER(blip: Blip, modifierHash: Hash) -> bool { invoke!(0xB059D7BD3D78C16F, blip, modifierHash) }
	pub fn _0x250C75EB1728CC0D(blip: Blip) { invoke_ignore!(0x250C75EB1728CC0D, blip) }
	pub fn SET_BLIP_FLASH_TIMER(blip: Blip, blipType: i32, blipHash: Hash) { invoke_ignore!(0x02FF4CF43B7209D1, blip, blipType, blipHash) }
	pub fn SET_BLIP_FLASHES(blip: Blip, p1: &mut i32, p2: &mut Hash) -> bool { invoke!(0x0DF2B55F717DDB10, blip, p1, p2) }
	pub fn TRIGGER_SONAR_BLIP(typeHash: Hash, x: f32, y: f32, z: f32) { invoke_ignore!(0x72DD432F3CDFC0EE, typeHash, x, y, z) }
	pub fn _TRIGGER_SONAR_BLIP_ON_ENTITY(typeHash: Hash, entity: Entity) { invoke_ignore!(0x0C7A2289A5C4D7C9, typeHash, entity) }
	pub fn ALLOW_SONAR_BLIPS(toggle: bool) { invoke_ignore!(0x6E6E64788C07D2E0, toggle) }
	pub fn SET_BLIP_COORDS(blip: Blip, posX: f32, posY: f32, posZ: f32) { invoke_ignore!(0x4FF674F5E23D49CE, blip, posX, posY, posZ) }
	pub fn GET_BLIP_COORDS(blip: Blip) -> Vector3 { invoke!(0x201C319797BDA603, blip) }
	pub fn SET_BLIP_SPRITE(blip: Blip, hash: Hash, p2: bool) { invoke_ignore!(0x74F74D3207ED525C, blip, hash, p2) }
	pub fn _0x01B928CA2E198B01(p0: Any) -> Any { invoke!(0x01B928CA2E198B01, p0) }
	pub fn SET_BLIP_NAME_FROM_TEXT_FILE(blip: Blip, textLabel: & CStr) { invoke_ignore!(0x0A062D6D7C0B2C2C, blip, textLabel) }
	pub fn _SET_BLIP_NAME(blip: Blip, name: & CStr) { invoke_ignore!(0x9CB1A1623062F402, blip, name) }
	pub fn SET_BLIP_NAME_TO_PLAYER_NAME(blip: Blip, player: Player) { invoke_ignore!(0x093DD5A31BC2B459, blip, player) }
	pub fn SET_BLIP_ROTATION(blip: Blip, rotation: i32) { invoke_ignore!(0x6049966A94FBE706, blip, rotation) }
	pub fn SET_BLIP_SCALE(blip: Blip, scale: f32) { invoke_ignore!(0xD38744167B2FA257, blip, scale) }
	pub fn REMOVE_BLIP(blip: &mut Blip) { invoke_ignore!(0xF2C3C9DA47AAA54A, blip) }
	pub fn DOES_BLIP_EXIST(blip: Blip) -> bool { invoke!(0xCD82FA174080B3B1, blip) }
	pub fn _DOES_ENTITY_HAVE_BLIP(entity: Entity) -> bool { invoke!(0x9FA00E2FC134A9D0, entity) }
	pub fn SET_RADAR_ZOOM(zoomLevel: i32) { invoke_ignore!(0xCAF6489DA2C8DD9E, zoomLevel) }
	pub fn _ABANDON_BLIP(blip: Blip) { invoke_ignore!(0xDEEDE7C41742E011, blip) }
	pub fn _IS_BLIP_ATTACHED_TO_ANY_ENTITY(blip: Blip) -> bool { invoke!(0xE9F676788F8D5E1E, blip) }
	pub fn IS_BLIP_ON_MINIMAP(blip: Blip) -> bool { invoke!(0x46534526B9CD2D17, blip) }
	pub fn FORCE_SONAR_BLIPS_THIS_FRAME() -> Any { invoke!(0xEE1C7BA69BB74B08) }
	pub fn SET_GPS_FLAGS(p0: i32, p1: f32) { invoke_ignore!(0x5DE61C90DDECFA2D, p0, p1) }
	pub fn CLEAR_GPS_FLAGS() { invoke_ignore!(0x4D3771237C79FF41) }
	pub fn SET_GPS_CUSTOM_ROUTE_RENDER(p0: bool, p1: i32, p2: i32) { invoke_ignore!(0xF6CEF599FC470B33, p0, p1, p2) }
	pub fn CLEAR_GPS_CUSTOM_ROUTE() { invoke_ignore!(0x1EAA5674B4D181C5) }
	pub fn START_GPS_MULTI_ROUTE(colorNameHash: Hash, onFoot: bool, inVehicle: bool) { invoke_ignore!(0x3D3D15AF7BCAAF83, colorNameHash, onFoot, inVehicle) }
	pub fn ADD_POINT_TO_GPS_MULTI_ROUTE(x: f32, y: f32, z: f32, p3: bool) { invoke_ignore!(0x64C59DD6834FA942, x, y, z, p3) }
	pub fn SET_GPS_MULTI_ROUTE_RENDER(toggle: bool) { invoke_ignore!(0x4426D65E029A4DC0, toggle) }
	pub fn CLEAR_GPS_MULTI_ROUTE() { invoke_ignore!(0x9E0AB9AAEE87CE28) }
	pub fn _START_GPS_CUSTOM_ROUTE_FROM_WAYPOINT_RECORDING_ROUTE(waypointRecording: & CStr, point: i32, numPoints: i32, colorNameHash: Hash, p4: bool, p5: bool) { invoke_ignore!(0x6B44F13D888F770D, waypointRecording, point, numPoints, colorNameHash, p4, p5) }
	pub fn CLEAR_GPS_PLAYER_WAYPOINT() { invoke_ignore!(0x08FDC6F796E350D1) }
	pub fn _0xD3F58E9316B7FC2A(p0: Any) { invoke_ignore!(0xD3F58E9316B7FC2A, p0) }
	pub fn SET_WAYPOINT_OFF() { invoke_ignore!(0xFA8C41E8020D3439) }
	pub fn IS_WAYPOINT_ACTIVE() -> bool { invoke!(0x202B1BBFC6AB5EE4) }
	pub fn _GET_WAYPOINT_COORDS() -> Vector3 { invoke!(0x29B30D07C3F7873B) }
	pub fn _0xF08E42BFA46BDFF8(p0: Any, p1: Any) -> bool { invoke!(0xF08E42BFA46BDFF8, p0, p1) }
	pub fn _HIDE_ACTIVE_POINTS_OF_INTEREST() { invoke_ignore!(0xA1B4052C2A3DCC1E) }
	pub fn _SHOW_ACTIVE_POINTS_OF_INTEREST() { invoke_ignore!(0x3FBB838AEA30C1D8) }
	pub fn _0xF47A1EB2A538A3A3() -> Any { invoke!(0xF47A1EB2A538A3A3) }
	pub fn _FIND_CLOSEST_GPS_POSITION(x: f32, y: f32, z: f32, outPosition: &mut Vector3) -> bool { invoke!(0x3FDA2B79AEEE351C, x, y, z, outPosition) }
	pub fn DISPLAY_RADAR(toggle: bool) { invoke_ignore!(0x1B3DA717B9AFF828, toggle) }
	pub fn _SET_RADAR_CONFIG_TYPE(configHash: Hash, p1: Hash) { invoke_ignore!(0x9C113883487FD53C, configHash, p1) }
	pub fn _ADD_PROP_TO_MINIMAP(minimapProp: Hash, x: f32, y: f32, rotation: f32, p4: i32) { invoke_ignore!(0x1392105DA88BBFFB, minimapProp, x, y, rotation, p4) }
	pub fn _REMOVE_PROP_FROM_MINIMAP(minimapProp: Hash) { invoke_ignore!(0xE057FEA9A22EB3EE, minimapProp) }
	pub fn SET_RADAR_AS_EXTERIOR_THIS_FRAME() { invoke_ignore!(0xA8EBBAE986FB5457) }
	pub fn SET_MINIMAP_HIDE_FOW(toggle: bool) { invoke_ignore!(0x4B8F743A4A6D2FF8, toggle) }
	pub fn _SET_FOW_UPDATE_PLAYER_OVERRIDE(toggle: bool, p1: Hash) { invoke_ignore!(0x63E7279D04160477, toggle, p1) }
	pub fn _SET_MINIMAP_FOW_OVERRIDE_REVEAL_SCALE(scale: f32, p1: Hash) { invoke_ignore!(0xE5A7F70B7C0F3271, scale, p1) }
	pub fn _SET_MINIMAP_FOW_SHOULD_UPDATE(toggle: bool, p1: Hash) { invoke_ignore!(0x632AA10BF7EA53D3, toggle, p1) }
	pub fn SET_MINIMAP_FOW_REVEAL_COORDINATE(x: f32, y: f32, z: f32, p3: Hash) { invoke_ignore!(0x73348402566ECB6E, x, y, z, p3) }
	pub fn SET_MINIMAP_FOW_REVEAL_VOLUME(volume: Volume, p1: Hash) { invoke_ignore!(0x63CBBD6CA6F321F9, volume, p1) }
	pub fn RESET_MINIMAP_FOW(hash: Hash) { invoke_ignore!(0xEB3CB3386C775D72, hash) }
	pub fn _REVEAL_MINIMAP_FOW(hash: Hash) { invoke_ignore!(0xF8096DF9B87246E3, hash) }
	pub fn _SET_MINIMAP_ZONE(zone: Hash) { invoke_ignore!(0xA657EC9DBC6CC900, zone) }
	pub fn LOCK_MINIMAP_ANGLE(angle: i32) { invoke_ignore!(0x0BFD145EF819FB3A, angle) }
	pub fn UNLOCK_MINIMAP_ANGLE() { invoke_ignore!(0x5373DE8E179BC2A0) }
	pub fn _MAP_ENABLE_REGION_BLIP(regionHash: Hash, styleHash: Hash) { invoke_ignore!(0x563FCB6620523917, regionHash, styleHash) }
	pub fn _MAP_DISABLE_REGION_BLIP(regionHash: Hash) { invoke_ignore!(0x6786D7AFAC3162B3, regionHash) }
	pub fn _MAP_IS_REGION_HIGHLIGHTED_WITH_STYLE(regionHash: Hash, styleHash: Hash) -> bool { invoke!(0xE38450DBCBC70E3D, regionHash, styleHash) }
	pub fn _MAP_DISCOVER_REGION(discoveryHash: Hash) { invoke_ignore!(0xD8C7162AB2E2AF45, discoveryHash) }
	pub fn _MAP_DISCOVERY_SET_ENABLED(discoveryHash: Hash) { invoke_ignore!(0xDA98246C7A3C2189, discoveryHash) }
	pub fn _MAP_IS_DISCOVERY_ACTIVE(discoveryHash: Hash) -> bool { invoke!(0x3F81EA4275D39D6F, discoveryHash) }
	pub fn _SET_PAUSEMAP_COORDS_WITH_RADIUS(x: f32, y: f32, z: f32, radius: f32) { invoke_ignore!(0xE0884C184728C75B, x, y, z, radius) }
	pub fn _0x7C9F4CDF402CA82A() { invoke_ignore!(0x7C9F4CDF402CA82A) }
	pub fn _0x44813684F72B563C(entity: Entity, p1: Any) { invoke_ignore!(0x44813684F72B563C, entity, p1) }
	pub fn _0x97F6F158CC5B5CA2(entity: Entity, p1: Any) { invoke_ignore!(0x97F6F158CC5B5CA2, entity, p1) }
	pub fn _0xBB68D4D3CA3DE402(p0: Any, p1: Any) { invoke_ignore!(0xBB68D4D3CA3DE402, p0, p1) }
	pub fn _0x3CB8859F04763C78(p0: Any, p1: Any) -> Any { invoke!(0x3CB8859F04763C78, p0, p1) }
	pub fn _0x7563CBCA99253D1A(entity: Entity, blip: Hash) { invoke_ignore!(0x7563CBCA99253D1A, entity, blip) }
	pub fn _0x1726963E6049DB53(p0: Any) { invoke_ignore!(0x1726963E6049DB53, p0) }
}
pub mod MINIGAME {
	use std::ffi::CStr;
	use crate::core::scripthook::native::*;
	use crate::core::types::*;

	pub fn _0x6480723D3BE535B6(p0: Any) { invoke_ignore!(0x6480723D3BE535B6, p0) }
	pub fn _0x3DF7EE3A76185108() { invoke_ignore!(0x3DF7EE3A76185108) }
	pub fn _0xE1F365C4C8F259D8(p0: Any, p1: Any, p2: Any) -> Any { invoke!(0xE1F365C4C8F259D8, p0, p1, p2) }
	pub fn _0xE53A308AC35877A8() -> Any { invoke!(0xE53A308AC35877A8) }
	pub fn _0x580F34C726387226(p0: Any, p1: Any) -> Any { invoke!(0x580F34C726387226, p0, p1) }
	pub fn _0x0876326238914A3F() { invoke_ignore!(0x0876326238914A3F) }
	pub fn _0x3FFE60DD8A936551(p0: Any, p1: Any) -> Any { invoke!(0x3FFE60DD8A936551, p0, p1) }
	pub fn _0x18A0D48DF9211C07() { invoke_ignore!(0x18A0D48DF9211C07) }
	pub fn _MINIGAME_IS_CONNECTED_TO_SERVER(p0: Any) -> Any { invoke!(0x2A0C4736AC5AF0CE, p0) }
	pub fn _0x39654E1F68B78287() -> Any { invoke!(0x39654E1F68B78287) }
	pub fn _0x3EECAADAB0D9FE29() -> Any { invoke!(0x3EECAADAB0D9FE29) }
	pub fn _0xD39D32EB3B52DD83(p0: Any) -> Any { invoke!(0xD39D32EB3B52DD83, p0) }
	pub fn _MINIGAME_IS_SEAT_OCCUPIED(p0: Any) -> bool { invoke!(0x8593A8CB0ED2C3B4, p0) }
	pub fn _MINIGAME_REQUEST_SEAT_AT_TABLE(data: &mut Any) -> bool { invoke!(0xF6AC6085D8D6C004, data) }
	pub fn _MINIGAME_LEAVE_TABLE(p0: Any) -> Any { invoke!(0xF5446E47941E654C, p0) }
	pub fn _MINIGAME_GET_NEXT_EVENT_TYPE() -> Any { invoke!(0x578907F59BA01B6C) }
	pub fn _MINIGAME_GET_NEXT_EVENT(p0: Any, p1: Any) -> Any { invoke!(0xDF728C5AE137FC13, p0, p1) }
	pub fn _MINIGAME_POP_NEXT_EVENT() { invoke_ignore!(0x833E03BAEBADC4B0) }
	pub fn _0x3B31732FADE5BAF3() -> Any { invoke!(0x3B31732FADE5BAF3) }
	pub fn _0x578907F59BA01B6D(p0: Any) -> Any { invoke!(0x578907F59BA01B6D, p0) }
	pub fn _0xDF728C5AE137FC14(p0: Any, p1: Any, p2: Any) -> Any { invoke!(0xDF728C5AE137FC14, p0, p1, p2) }
	pub fn _MINIGAME_IS_REQUEST_PENDING(p0: Any) -> Any { invoke!(0x9105A4A2556FA937, p0) }
	pub fn _0x644439B5387EE57E(p0: Any, p1: Any) -> Any { invoke!(0x644439B5387EE57E, p0, p1) }
	pub fn _0x15E90B6A993017AA() -> Any { invoke!(0x15E90B6A993017AA) }
	pub fn _0x10342CC82E8356E9(p0: Any, p1: Any) -> Any { invoke!(0x10342CC82E8356E9, p0, p1) }
	pub fn _0x32A7C216344D623B(p0: Any, p1: Any, p2: Any) -> Any { invoke!(0x32A7C216344D623B, p0, p1, p2) }
	pub fn _0xEC819D612038EF4B(p0: Any, p1: Any, p2: Any, p3: Any, p4: Any, p5: Any, p6: Any, p7: Any) -> Any { invoke!(0xEC819D612038EF4B, p0, p1, p2, p3, p4, p5, p6, p7) }
	pub fn _POKER_GET_GAME_SETTINGS_FOR_ID(p0: Any) -> Any { invoke!(0x2D20E12E1990D584, p0) }
	pub fn _POKER_BUY_IN(p0: Any, p1: Any) -> Any { invoke!(0xB4D610EA5A1FDE74, p0, p1) }
	pub fn _POKER_CALL(p0: Any, p1: Any) -> Any { invoke!(0x8DED681B161EBD78, p0, p1) }
	pub fn _POKER_CHECK(p0: Any, p1: Any) -> Any { invoke!(0x49A045628D9B1B86, p0, p1) }
	pub fn _POKER_FOLD(p0: Any) -> Any { invoke!(0x3DFAB7D9BB45B5BE, p0) }
	pub fn _POKER_RAISE(p0: Any, p1: Any) -> Any { invoke!(0xECCF45A79A17BB96, p0, p1) }
	pub fn _POKER_REVEAL(p0: Any) -> Any { invoke!(0x2F2131DB0A8B02DC, p0) }
	pub fn _0x58521E6DCDE97D74(p0: Any, p1: Any, p2: Any) { invoke_ignore!(0x58521E6DCDE97D74, p0, p1, p2) }
	pub fn _0xF6DE98516FD3AC9B() { invoke_ignore!(0xF6DE98516FD3AC9B) }
	pub fn _0x3F4FD4BED07AB8C4(p0: Any) -> Any { invoke!(0x3F4FD4BED07AB8C4, p0) }
	pub fn _0x3AE451860F03CA8A(p0: Any, p1: Any) -> Any { invoke!(0x3AE451860F03CA8A, p0, p1) }
	pub fn _0x012027C28F421F46(p0: Any, p1: Any) -> Any { invoke!(0x012027C28F421F46, p0, p1) }
	pub fn _0x455ECCA0715C507F() { invoke_ignore!(0x455ECCA0715C507F) }
	pub fn _DOMINOES_BUY_IN(p0: Any) -> Any { invoke!(0x399E6CD12FC8CA89, p0) }
	pub fn _DOMINOES_PLACE_DOMINO(p0: Any, p1: Any) -> Any { invoke!(0xB79A29B33BF29BA5, p0, p1) }
	pub fn _DOMINOES_REQUEST_VALID_PLACEMENTS(p0: Any) -> Any { invoke!(0xE26AEE7E67D9E21D, p0) }
	pub fn _0xA2DB3C6270C122E3(p0: Any, p1: Any) -> Any { invoke!(0xA2DB3C6270C122E3, p0, p1) }
	pub fn _0x398066F893149856(p0: Any, p1: Any, p2: Any) -> Any { invoke!(0x398066F893149856, p0, p1, p2) }
	pub fn _0xBEA7D3CB47E1479C() -> Any { invoke!(0xBEA7D3CB47E1479C) }
	pub fn _0x910B088E51A511AC() -> Any { invoke!(0x910B088E51A511AC) }
	pub fn _0x9DD95B405AB4983E(p0: Any, p1: Any) -> Any { invoke!(0x9DD95B405AB4983E, p0, p1) }
}
pub mod MISC {
	use std::ffi::{CStr, CString};
	use crate::core::scripthook::native::*;
	use crate::core::types::*;

	pub fn GET_NUMBER_OF_FREE_STACKS_OF_THIS_SIZE(stackSize: i32) -> i32 { invoke!(0x40DC2907A9697EF7, stackSize) }
	pub fn _GET_NUMBER_OF_INSTRUCTIONS() -> i32 { invoke!(0x72904D3D62AF5839) }
	pub fn _GET_MAX_NUM_INSTRUCTIONS() -> i32 { invoke!(0xC43CD2668B204419) }
	pub fn IS_MAG_DEMO_1_ACTIVE() -> bool { invoke!(0x5FC9357C26DAEFCE) }
	pub fn SET_RANDOM_SEED(seed: i32) { invoke_ignore!(0x5CD7A49104AFCB6B, seed) }
	pub fn SET_TIME_SCALE(timeScale: f32) { invoke_ignore!(0x9682AF6050854856, timeScale) }
	pub fn SET_MISSION_FLAG(toggle: bool) { invoke_ignore!(0x36694B456BE80D0A, toggle) }
	pub fn GET_MISSION_FLAG() -> bool { invoke!(0xB15CD1CF58771DE1) }
	pub fn _0x5801BE2DF2AF07EC(p0: Any) { invoke_ignore!(0x5801BE2DF2AF07EC, p0) }
	pub fn SET_RANDOM_EVENT_FLAG(toggle: bool) { invoke_ignore!(0xB1ADCCC4150C6473, toggle) }
	pub fn GET_RANDOM_EVENT_FLAG() -> bool { invoke!(0x924D54E5698AE3E0) }
	pub fn _0x9BF2C0C568C61641(p0: Any) { invoke_ignore!(0x9BF2C0C568C61641, p0) }
	pub fn ACTIVITY_FEED_CREATE(p0: & CStr, p1: & CStr) { invoke_ignore!(0xCC7FC854B956A128, p0, p1) }
	pub fn ACTIVITY_FEED_ADD_SUBSTRING_TO_CAPTION(p0: & CStr) { invoke_ignore!(0x9935F76407C32539, p0) }
	pub fn _0xFF252E2BAFB7330F(p0: Any) { invoke_ignore!(0xFF252E2BAFB7330F, p0) }
	pub fn ACTIVITY_FEED_ACTION_START_WITH_COMMAND_LINE(p0: & CStr, p1: & CStr) { invoke_ignore!(0x91D657230BC208D2, p0, p1) }
	pub fn ACTIVITY_FEED_ACTION_START_WITH_COMMAND_LINE_ADD(p0: & CStr) { invoke_ignore!(0x1694A053DFB61A34, p0) }
	pub fn _0xAF530E56505D1BD6(p0: Any) -> Any { invoke!(0xAF530E56505D1BD6, p0) }
	pub fn ACTIVITY_FEED_POST() { invoke_ignore!(0xB16FC7B364D86585) }
	pub fn _0xF81C53561D15F330() -> *const char { invoke!(0xF81C53561D15F330) }
	pub fn _0x1096603B519C905F(name: & CStr) { invoke_ignore!(0x1096603B519C905F, name) }
	pub fn _0xCC3EDC5614B03F61(p0: i32) { invoke_ignore!(0xCC3EDC5614B03F61, p0) }
	pub fn INFORM_CODE_OF_CONTENT_ID_OF_CURRENT_UGC_MISSION(p0: & CStr) { invoke_ignore!(0x708DF841B8F27AA2, p0) }
	pub fn _0xDA4D8EB04E8E2928(p0: Any) { invoke_ignore!(0xDA4D8EB04E8E2928, p0) }
	pub fn _0xB08C4FA25BC29DB9(p0: Any) { invoke_ignore!(0xB08C4FA25BC29DB9, p0) }
	pub fn _GET_PREV_WEATHER_TYPE_HASH_NAME() -> Hash { invoke!(0x4BEB42AEBCA732E9) }
	pub fn _GET_NEXT_WEATHER_TYPE_HASH_NAME() -> Hash { invoke!(0x51021D36F62AAA83) }
	pub fn _0x0730E518486DEEC3(p0: Any) { invoke_ignore!(0x0730E518486DEEC3, p0) }
	pub fn SET_WEATHER_TYPE(weatherType: Hash, p1: bool, p2: bool, transition: bool, transitionTime: f32, p5: bool) { invoke_ignore!(0x59174F1AFE095B5A, weatherType, p1, p2, transition, transitionTime, p5) }
	pub fn _SET_WEATHER_TYPE_2(weatherType: Hash, p1: i32, p2: i32, p3: i32, p4: bool) { invoke_ignore!(0x2C6A07AF9AEDABD8, weatherType, p1, p2, p3, p4) }
	pub fn _GET_RANDOM_WEATHER_TYPE_INDEX() -> i32 { invoke!(0x7F4CE164D9A11DFE) }
	pub fn _GET_RANDOM_WEATHER_TYPE() -> Hash { invoke!(0x1359C181BC625503) }
	pub fn SET_RANDOM_WEATHER_TYPE(p0: bool, p1: bool) { invoke_ignore!(0x6E5A7FBEECAB3C72, p0, p1) }
	pub fn CLEAR_WEATHER_TYPE_PERSIST() { invoke_ignore!(0xD85DFE5C131E4AE9) }
	pub fn _CLEAR_WEATHER_TYPE_PERSIST_OVERTIME(milliseconds: i32) { invoke_ignore!(0xCE7690C0A0D1C36D, milliseconds) }
	pub fn GET_CURR_WEATHER_STATE(weatherType1: &mut Hash, weatherType2: &mut Hash, percentWeather2: &mut f32) { invoke_ignore!(0x0AC679B2342F14F2, weatherType1, weatherType2, percentWeather2) }
	pub fn SET_CURR_WEATHER_STATE(weatherType1: Hash, weatherType2: Hash, percentWeather2: f32, enabled: bool) { invoke_ignore!(0xFA3E3CA8A1DE6D5D, weatherType1, weatherType2, percentWeather2, enabled) }
	pub fn _SET_WEATHER_TYPE_FROZEN(toggle: bool) { invoke_ignore!(0xD74ACDF7DB8114AF, toggle) }
	pub fn _GET_FORCED_WEATHER(weather: &mut Hash, p1: &mut Hash) { invoke_ignore!(0xDD560ABEF5D3784C, weather, p1) }
	pub fn _0x2916B30DC6C41179(weatherType: Hash) { invoke_ignore!(0x2916B30DC6C41179, weatherType) }
	pub fn _0xD3F943B88F55376A(weatherType: Hash) { invoke_ignore!(0xD3F943B88F55376A, weatherType) }
	pub fn _0x243CEDE8F916B994() { invoke_ignore!(0x243CEDE8F916B994) }
	pub fn _SET_OVERRIDE_WEATHER(weatherType: Hash) { invoke_ignore!(0xBE83CAE8ED77A94F, weatherType) }
	pub fn CLEAR_OVERRIDE_WEATHER() { invoke_ignore!(0x80A398F16FFE3CC3) }
	pub fn _SET_WEATHER_VARIATION(weatherType: & CStr, variation: & CStr) { invoke_ignore!(0x3373779BAF7CAF48, weatherType, variation) }
	pub fn _CLEAR_WEATHER_VARIATION(weatherType: & CStr, p1: bool) { invoke_ignore!(0x0E71C80FA4EC8147, weatherType, p1) }
	pub fn WATER_OVERRIDE_SET_SHOREWAVEAMPLITUDE(amplitude: f32) { invoke_ignore!(0x55123D5A7D9D3C42, amplitude) }
	pub fn WATER_OVERRIDE_SET_OCEANWAVEMAXAMPLITUDE(maxAmplitude: f32) { invoke_ignore!(0xF06C5B66DE20B2B8, maxAmplitude) }
	pub fn _GET_TEMPERATURE_AT_COORDS(x: f32, y: f32, z: f32) -> f32 { invoke!(0xB98B78C3768AF6E0, x, y, z) }
	pub fn SET_WIND_SPEED(speed: f32) { invoke_ignore!(0xD00C2D82DC04A99F, speed) }
	pub fn GET_WIND_SPEED() -> f32 { invoke!(0xFFB7E74E041150A4) }
	pub fn SET_WIND_DIRECTION(direction: f32) { invoke_ignore!(0xB56C4F5F57A45600, direction) }
	pub fn GET_WIND_DIRECTION() -> Vector3 { invoke!(0xF703E82F3FE14A5F) }
	pub fn SET_RAIN(intensity: f32) { invoke_ignore!(0x193DFC0526830FD6, intensity) }
	pub fn GET_RAIN_LEVEL() -> f32 { invoke!(0x931B5F4CC130224B) }
	pub fn _0x745808BB01CEC6B9(p0: f32) { invoke_ignore!(0x745808BB01CEC6B9, p0) }
	pub fn _SET_SNOW_LEVEL(level: f32) { invoke_ignore!(0xF6BEE7E80EC5CA40, level) }
	pub fn GET_SNOW_LEVEL() -> f32 { invoke!(0x1E5D727041BE1709) }
	pub fn FORCE_LIGHTNING_FLASH() { invoke_ignore!(0x369DB5B2510FA080) }
	pub fn _FORCE_LIGHTNING_FLASH_AT_COORDS(x: f32, y: f32, z: f32, p3: f32) { invoke_ignore!(0x67943537D179597C, x, y, z, p3) }
	pub fn _0xA9342743B634A462(p0: Any) { invoke_ignore!(0xA9342743B634A462, p0) }
	pub fn GET_GAME_TIMER() -> i32 { invoke!(0x4F67E8ECA7D3F667) }
	pub fn _GET_GAME_TIMER_NON_SCALED_CLIPPED() -> i32 { invoke!(0x483B8C542103AD72) }
	pub fn GET_SYSTEM_TIME() -> i32 { invoke!(0xBE7F225417E35A7C) }
	pub fn GET_NUMBER_OF_MICROSECONDS_SINCE_LAST_CALL() -> i32 { invoke!(0xB0CE5E5ED8BB3581) }
	pub fn GET_SCRIPT_TIME_WITHIN_FRAME_IN_MICROSECONDS() -> i32 { invoke!(0x63219768C586667C) }
	pub fn RESET_SCRIPT_TIME_WITHIN_FRAME() { invoke_ignore!(0x1411A7CBC3A6EB7B) }
	pub fn GET_FRAME_TIME() -> f32 { invoke!(0x5E72022914CE3C38) }
	pub fn GET_SYSTEM_TIME_STEP() -> f32 { invoke!(0x3F3172FEAE3AFE1C) }
	pub fn GET_FRAME_COUNT() -> i32 { invoke!(0x77DFA958FCF100C1) }
	pub fn _0x6BED40493A1AFDB8(p0: &mut Any, p1: f32) { invoke_ignore!(0x6BED40493A1AFDB8, p0, p1) }
	pub fn _READ_INT_AS_FLOAT(value: i32) -> f32 { invoke!(0xD2C9126410DFA1B2, value) }
	pub fn GET_RANDOM_FLOAT_IN_RANGE(startRange: f32, endRange: f32) -> f32 { invoke!(0xE29F927A961F8AAA, startRange, endRange) }
	pub fn GET_RANDOM_INT_IN_RANGE(startRange: i32, endRange: i32) -> i32 { invoke!(0xD53343AA4FB7DD28, startRange, endRange) }
	pub fn GET_GROUND_Z_FOR_3D_COORD(x: f32, y: f32, z: f32, groundZ: &mut f32, p4: bool) -> bool { invoke!(0x24FA4267BB8D2431, x, y, z, groundZ, p4) }
	pub fn GET_GROUND_Z_AND_NORMAL_FOR_3D_COORD(x: f32, y: f32, z: f32, groundZ: &mut f32, normal: &mut Vector3) -> bool { invoke!(0x2A29CA9A6319E6AB, x, y, z, groundZ, normal) }
	pub fn _0xBBE5B63EFFB08E68(p0: Any, p1: Any, p2: Any, p3: Any, p4: Any, p5: Any, p6: Any) -> Any { invoke!(0xBBE5B63EFFB08E68, p0, p1, p2, p3, p4, p5, p6) }
	pub fn ASIN(p0: f32) -> f32 { invoke!(0x6E3C15D296C15583, p0) }
	pub fn ACOS(p0: f32) -> f32 { invoke!(0x586690F0176DC575, p0) }
	pub fn TAN(p0: f32) -> f32 { invoke!(0x8C13DB96497B7ABF, p0) }
	pub fn ATAN(p0: f32) -> f32 { invoke!(0x503054DED0B78027, p0) }
	pub fn ATAN2(p0: f32, p1: f32) -> f32 { invoke!(0x965B220A066E3F07, p0, p1) }
	pub fn GET_DISTANCE_BETWEEN_COORDS(x1: f32, y1: f32, z1: f32, x2: f32, y2: f32, z2: f32, useZ: bool) -> f32 { invoke!(0x0BE7F4E3CDBAFB28, x1, y1, z1, x2, y2, z2, useZ) }
	pub fn GET_ANGLE_BETWEEN_2D_VECTORS(x1: f32, y1: f32, x2: f32, y2: f32) -> f32 { invoke!(0xD0DFE1C486097BBB, x1, y1, x2, y2) }
	pub fn GET_HEADING_FROM_VECTOR_2D(dx: f32, dy: f32) -> f32 { invoke!(0x38D5202FF9271C62, dx, dy) }
	pub fn GET_CLOSEST_POINT_ON_LINE(p0: f32, p1: f32, p2: f32, p3: f32, p4: f32, p5: f32, p6: f32, p7: f32, p8: f32, p9: bool) -> Vector3 { invoke!(0x83ACC65D9ACEC5EF, p0, p1, p2, p3, p4, p5, p6, p7, p8, p9) }
	pub fn GET_LINE_PLANE_INTERSECTION(p0: f32, p1: f32, p2: f32, p3: f32, p4: f32, p5: f32, p6: f32, p7: f32, p8: f32, p9: f32, p10: f32, p11: f32, p12: &mut f32) -> bool { invoke!(0xAB6A04CEC428258B, p0, p1, p2, p3, p4, p5, p6, p7, p8, p9, p10, p11, p12) }
	pub fn SET_BIT(address: &mut i32, offset: i32) { invoke_ignore!(0xF73FBE4845C43B5B, address, offset) }
	pub fn CLEAR_BIT(address: &mut i32, offset: i32) { invoke_ignore!(0x7D1D4A3602B6AD4E, address, offset) }
	pub fn _IS_BIT_FLAG_SET(bitFlags: &mut Any, flag: i32) -> bool { invoke!(0x8F4F050054005C27, bitFlags, flag) }
	pub fn _IS_ANY_BIT_FLAG_SET(bitFlags: &mut Any) -> bool { invoke!(0x80E9C316EF84DD81, bitFlags) }
	pub fn _COUNT_BIT_FLAGS(bitFlags: &mut Any) -> i32 { invoke!(0xE704838F36F93B7B, bitFlags) }
	pub fn _SET_BIT_FLAG(bitFlags: &mut Any, flag: i32) { invoke_ignore!(0xE84AAC1B22A73E99, bitFlags, flag) }
	pub fn _CLEAR_BIT_FLAG(bitFlags: &mut Any, flag: i32) { invoke_ignore!(0xB909149F2BB5F6DA, bitFlags, flag) }
	pub fn _CLEAR_ALL_BIT_FLAGS(bitFlags: &mut Any) { invoke_ignore!(0xD2D74F89DF844A50, bitFlags) }
	pub fn GET_HASH_KEY(string: & CStr) -> Hash { invoke!(0xFD340785ADF8CFB7, string) }
	pub fn _GET_EASING_CURVE_VALUE(t: f32, b: f32, d: f32, easingCurveType: i32) -> f32 { invoke!(0xEF50E344A8F93784, t, b, d, easingCurveType) }
	pub fn IS_POSITION_OCCUPIED(x: f32, y: f32, z: f32, range: f32, p4: bool, p5: bool, p6: bool, p7: bool, p8: bool, p9: Any, p10: bool) -> bool { invoke!(0x825CA3ED43831015, x, y, z, range, p4, p5, p6, p7, p8, p9, p10) }
	pub fn CLEAR_AREA(x: f32, y: f32, z: f32, radius: f32, flag: i32) { invoke_ignore!(0x3B882A96EA77D5B1, x, y, z, radius, flag) }
	pub fn _CLEAR_VOLUME_AREA(volume: Volume, flag: i32) { invoke_ignore!(0x2FCD528A397E5C88, volume, flag) }
	pub fn CLEAR_ANGLED_AREA_OF_VEHICLES(p0: Any, p1: Any, p2: Any, p3: Any, p4: Any, p5: Any, p6: Any, p7: Any) { invoke_ignore!(0xA4D83115C1E02F8A, p0, p1, p2, p3, p4, p5, p6, p7) }
	pub fn SET_CREDITS_ACTIVE(toggle: bool) { invoke_ignore!(0xD37BECF862DA726F, toggle) }
	pub fn NETWORK_SET_SCRIPT_IS_SAFE_FOR_NETWORK_GAME() { invoke_ignore!(0x3D0EAC6385DD6100) }
	pub fn PAUSE_DEATH_ARREST_RESTART(toggle: bool) { invoke_ignore!(0x66AB6B6C7E72F393, toggle) }
	pub fn IGNORE_NEXT_RESTART(toggle: bool) { invoke_ignore!(0x6C9FF40FF1B69F8F, toggle) }
	pub fn SET_FADE_IN_AFTER_DEATH_ARREST(toggle: bool) { invoke_ignore!(0xDF3B5846DE5904AF, toggle) }
	pub fn SET_FADE_IN_AFTER_LOAD(toggle: bool) { invoke_ignore!(0xAC806C4CAB973517, toggle) }
	pub fn OVERRIDE_SAVE_HOUSE(override_: bool, x: f32, y: f32, z: f32, heading: f32, isAutosave: bool, returnCoords: &mut Vector3, returnHeading: &mut f32) -> bool { invoke!(0xB2C69E11A37B5AF0, override_, x, y, z, heading, isAutosave, returnCoords, returnHeading) }
	pub fn SHOOT_SINGLE_BULLET_BETWEEN_COORDS(x1: f32, y1: f32, z1: f32, x2: f32, y2: f32, z2: f32, damage: i32, p7: bool, weaponHash: Hash, ownerPed: Ped, isAudible: bool, isInvisible: bool, speed: f32, p13: bool) { invoke_ignore!(0x867654CBC7606F2C, x1, y1, z1, x2, y2, z2, damage, p7, weaponHash, ownerPed, isAudible, isInvisible, speed, p13) }
	pub fn FIRE_SINGLE_BULLET(args: &mut Any) { invoke_ignore!(0xCBC9A21F6A2A679C, args) }
	pub fn GET_MODEL_DIMENSIONS(modelHash: Hash, minimum: &mut Vector3, maximum: &mut Vector3) { invoke_ignore!(0xDCB8DDD5D054A7E7, modelHash, minimum, maximum) }
	pub fn IS_BIT_SET(address: i32, offset: i32) -> bool { invoke!(0x4ED6CFDFE8D4131A, address, offset) }
	pub fn _0x0A487CC74A517FB5(p0: Any) { invoke_ignore!(0x0A487CC74A517FB5, p0) }
	pub fn IS_MINIGAME_IN_PROGRESS() -> bool { invoke!(0xF4D8BCD052E7EA1B) }
	pub fn SHOULD_USE_METRIC_MEASUREMENTS() -> bool { invoke!(0x4FB556ACEFA93098) }
	pub fn _SHOULD_USE_METRIC_MEASUREMENTS_2() -> bool { invoke!(0x58BCDC75BA52110A) }
	pub fn _SHOULD_USE_METRIC_TEMPERATURE() -> bool { invoke!(0xFF4AAF3275BAAB4F) }
	pub fn _SHOULD_USE_METRIC_WEIGHT() -> bool { invoke!(0x8F24157FEDB85EA2) }
	pub fn _SHOULD_USE_24_HOUR_CLOCK() -> bool { invoke!(0x0177CF20345F44DD) }
	pub fn COMPARE_STRINGS(str1: & CStr, str2: & CStr, matchCase: bool, maxLength: i32) -> i32 { invoke!(0xBFBB74A15EFC149B, str1, str2, matchCase, maxLength) }
	pub fn ABSI(value: i32) -> i32 { invoke!(0x0C214D5B8A38C828, value) }
	pub fn ABSF(value: f32) -> f32 { invoke!(0x134549B388167CBF, value) }
	pub fn IS_PROJECTILE_IN_AREA(x1: f32, y1: f32, z1: f32, x2: f32, y2: f32, z2: f32, ownedByPlayer: bool) -> bool { invoke!(0x05B0061EFDFC8941, x1, y1, z1, x2, y2, z2, ownedByPlayer) }
	pub fn IS_PROJECTILE_TYPE_IN_AREA(xMin: f32, yMin: f32, zMin: f32, xMax: f32, yMax: f32, zMax: f32, weaponType: Hash, isPlayer: bool) -> bool { invoke!(0x04965FB9E14235C7, xMin, yMin, zMin, xMax, yMax, zMax, weaponType, isPlayer) }
	pub fn IS_PROJECTILE_TYPE_IN_ANGLED_AREA(p0: f32, p1: f32, p2: f32, p3: f32, p4: f32, p5: f32, p6: f32, p7: Any, p8: bool) -> bool { invoke!(0x928431F4133CD3D4, p0, p1, p2, p3, p4, p5, p6, p7, p8) }
	pub fn IS_PROJECTILE_TYPE_WITHIN_DISTANCE(p0: f32, p1: f32, p2: f32, p3: Any, p4: f32, p5: bool) -> bool { invoke!(0xF51C9BAAD9ED64C4, p0, p1, p2, p3, p4, p5) }
	pub fn GET_COORDS_OF_PROJECTILE_TYPE_WITHIN_DISTANCE(ped: Ped, weaponHash: Hash, distance: f32, outCoords: &mut Vector3, p4: bool, mustBeOwnedByThisPed: bool) -> bool { invoke!(0xD73C960A681052DF, ped, weaponHash, distance, outCoords, p4, mustBeOwnedByThisPed) }
	pub fn GET_PROJECTILE_OF_PROJECTILE_TYPE_WITHIN_DISTANCE(ped: Ped, weaponHash: Hash, distance: f32, outCoords: &mut Vector3, outProjectile: &mut Object, p5: bool, mustBeOwnedByThisPed: bool) -> bool { invoke!(0x9578986A6105A6AD, ped, weaponHash, distance, outCoords, outProjectile, p5, mustBeOwnedByThisPed) }
	pub fn IS_BULLET_IN_ANGLED_AREA(p0: f32, p1: f32, p2: f32, p3: f32, p4: f32, p5: f32, p6: f32, p7: bool) -> bool { invoke!(0x9D09D8493747CF02, p0, p1, p2, p3, p4, p5, p6, p7) }
	pub fn IS_BULLET_IN_AREA(p0: f32, p1: f32, p2: f32, p3: f32, p4: bool) -> bool { invoke!(0xC652FD308772D79E, p0, p1, p2, p3, p4) }
	pub fn IS_BULLET_IN_BOX(p0: f32, p1: f32, p2: f32, p3: f32, p4: f32, p5: f32, p6: bool) -> bool { invoke!(0xC128137C52152741, p0, p1, p2, p3, p4, p5, p6) }
	pub fn HAS_BULLET_IMPACTED_IN_AREA(x: f32, y: f32, z: f32, p3: f32, p4: bool, p5: bool) -> bool { invoke!(0xC153E5BCCF411814, x, y, z, p3, p4, p5) }
	pub fn HAS_BULLET_IMPACTED_IN_BOX(p0: f32, p1: f32, p2: f32, p3: f32, p4: f32, p5: f32, p6: bool, p7: bool) -> bool { invoke!(0x3B6A4C05FB2B33AC, p0, p1, p2, p3, p4, p5, p6, p7) }
	pub fn _0x7A76104CC2CC69E8(entity: Entity, p1: i32, p2: i32) -> Any { invoke!(0x7A76104CC2CC69E8, entity, p1, p2) }
	pub fn _0xDC416CA762BC4F43(p0: Any, p1: Any, p2: Any, p3: Any, p4: Any, p5: Any) -> Any { invoke!(0xDC416CA762BC4F43, p0, p1, p2, p3, p4, p5) }
	pub fn _0x970339EFA4FDE518(p0: Any, p1: Any, p2: Any) -> Any { invoke!(0x970339EFA4FDE518, p0, p1, p2) }
	pub fn IS_ORBIS_VERSION() -> bool { invoke!(0x88CFAE250D3E0C71) }
	pub fn IS_DURANGO_VERSION() -> bool { invoke!(0xD1CCC2A2639D325F) }
	pub fn IS_PC_VERSION() -> bool { invoke!(0xB0FB6CFAA5A1C833) }
	pub fn IS_STADIA_VERSION() -> bool { invoke!(0x268AB8420A9E4ED7) }
	pub fn IS_STRING_NULL(string: & CStr) -> bool { invoke!(0x602102324604D96B, string) }
	pub fn IS_STRING_NULL_OR_EMPTY(string: & CStr) -> bool { invoke!(0x2CF12F9ACF18F048, string) }
	pub fn IS_STRING_NULL_OR_EMPTY_OR_SPACES(string: & CStr) -> bool { invoke!(0x375F5870A7B8BEC1, string) }
	pub fn ARE_STRINGS_EQUAL(string1: & CStr, string2: & CStr) -> bool { invoke!(0xD3852F22AB713A1F, string1, string2) }
	pub fn _DOES_STRING_EXIST_IN_STRING(string1: & CStr, string2: & CStr) -> bool { invoke!(0x9382D5D43D2AA6FF, string1, string2) }
	pub fn _0x3C3C7B1B5EC08764() { invoke_ignore!(0x3C3C7B1B5EC08764) }
	pub fn _0x94E8CA3DEE952789(p0: Any, p1: Any) -> Any { invoke!(0x94E8CA3DEE952789, p0, p1) }
	pub fn _0x5B4A8121A47D844D(p0: Any) -> Any { invoke!(0x5B4A8121A47D844D, p0) }
	pub fn STRING_TO_INT(string: & CStr, outInteger: &mut i32) -> bool { invoke!(0xF2DD2298B3AF23E2, string, outInteger) }
	pub fn _INT_TO_STRING(value: i32, format: & CStr, buffer: &mut CStr) { invoke_ignore!(0xCF11C0CEB40C401B, value, format, buffer) }
	pub fn _0x74ACA66484CEBAF0(p0: Any) { invoke_ignore!(0x74ACA66484CEBAF0, p0) }
	pub fn _0x49C44FE78A135A1D(p0: Any) { invoke_ignore!(0x49C44FE78A135A1D, p0) }
	pub fn _0xF650DCF5D6F312C1(p0: Any) { invoke_ignore!(0xF650DCF5D6F312C1, p0) }
	pub fn _GET_STRING_FROM_FLOAT(value: f32, digits: i32) -> *const char { invoke!(0x2B6846401D68E563, value, digits) }
	pub fn _GET_STRING_FROM_VECTOR(x: f32, y: f32, z: f32) -> *const char { invoke!(0x6C4DBF553885F9EB, x, y, z) }
	pub fn _GET_STRING_FROM_BOOL(value: bool) -> *const char { invoke!(0xF216F74101968DB0, value) }
	pub fn VAR_STRING<'a>(flags: i32, string_type: &CStr, string: &CStr) -> &'a CStr { invoke!(0xFA925AC00EB830B9, flags) }
	pub fn _CREATE_COLOR_STRING(rgb: i32) -> *const char { invoke!(0xBCC2CFADEA1AEA6C, rgb) }
	pub fn SET_BITS_IN_RANGE(var: &mut i32, rangeStart: i32, rangeEnd: i32, p3: i32) { invoke_ignore!(0x324DC1CEF57F31E6, var, rangeStart, rangeEnd, p3) }
	pub fn GET_BITS_IN_RANGE(var: i32, rangeStart: i32, rangeEnd: i32) -> i32 { invoke!(0x68E1352AF48F905D, var, rangeStart, rangeEnd) }
	pub fn SET_GAME_PAUSED(toggle: bool) { invoke_ignore!(0xFAEC088D28B1DE4A, toggle) }
	pub fn SET_THIS_SCRIPT_CAN_BE_PAUSED(toggle: bool) { invoke_ignore!(0x3215376E79F6EA18, toggle) }
	pub fn SET_THIS_SCRIPT_CAN_REMOVE_BLIPS_CREATED_BY_ANY_SCRIPT(toggle: bool) { invoke_ignore!(0x8ABD939C2E5D00ED, toggle) }
	pub fn SET_CHEAT_ACTIVE(cheatId: i32) { invoke_ignore!(0xD4958E8CF0DE0DD0, cheatId) }
	pub fn _0xB711EB4BC8D06013() { invoke_ignore!(0xB711EB4BC8D06013) }
	pub fn POPULATE_NOW() { invoke_ignore!(0xEA6DC3A8ADD2005F) }
	pub fn IS_GAME_SESSION_STATE_MACHINE_IDLE() -> bool { invoke!(0xF9E7DBB39080640B) }
	pub fn _QUEUE_SAVEGAME_OPERATION(p0: i32) -> bool { invoke!(0x279B0696DA4657EB, p0) }
	pub fn _GET_STATUS_OF_SAVEGAME_OPERATION(p0: i32) -> i32 { invoke!(0x1B065A2BF7953815, p0) }
	pub fn _0x6C7B68D3CE60E8DE(p0: Any) -> Any { invoke!(0x6C7B68D3CE60E8DE, p0) }
	pub fn _0x627B68D9CE6EE8DE(p0: Any) -> Any { invoke!(0x627B68D9CE6EE8DE, p0) }
	pub fn _0x7CF96F1250EF3221(p0: Any) -> Any { invoke!(0x7CF96F1250EF3221, p0) }
	pub fn COPY_SCRIPT_STRUCT(dst: &mut Any, src: &mut Any, size: i32) { invoke_ignore!(0xF7AC7DC0DEE7C9BE, dst, src, size) }
	pub fn ENABLE_DISPATCH_SERVICE(dispatchService: i32, toggle: bool) { invoke_ignore!(0x50E52637EF70EF77, dispatchService, toggle) }
	pub fn BLOCK_DISPATCH_SERVICE_RESOURCE_CREATION(dispatchService: i32, toggle: bool) { invoke_ignore!(0x66947E61A44DE2C6, dispatchService, toggle) }
	pub fn CREATE_INCIDENT(dispatchService: i32, x: f32, y: f32, z: f32, numUnits: i32, radius: f32, outIncidentID: &mut i32, p7: Any, p8: Any) -> bool { invoke!(0x3F892CAF67444AE7, dispatchService, x, y, z, numUnits, radius, outIncidentID, p7, p8) }
	pub fn _CREATE_INCIDENT_WITH_ENTITIES(dispatchService: i32, x: f32, y: f32, z: f32, itemSet: ItemSet, radius: f32, outIncidentID: &mut i32) -> bool { invoke!(0xAB3D3F45436DB1D8, dispatchService, x, y, z, itemSet, radius, outIncidentID) }
	pub fn DELETE_INCIDENT(incidentId: i32) { invoke_ignore!(0x5CFD0F0D6AAE0AEE, incidentId) }
	pub fn IS_INCIDENT_VALID(incidentId: i32) -> bool { invoke!(0x39F2B1BAD412246A, incidentId) }
	pub fn _SET_INCIDENT_UNK(incidentId: i32) { invoke_ignore!(0x9617B6E5F6537B63, incidentId) }
	pub fn ADD_POP_MULTIPLIER_AREA(x1: f32, y1: f32, z1: f32, x2: f32, y2: f32, z2: f32, pedDensity: f32, trafficDensity: f32, p8: bool, p9: bool) -> i32 { invoke!(0x5EBDA1A3B8CB5EF7, x1, y1, z1, x2, y2, z2, pedDensity, trafficDensity, p8, p9) }
	pub fn DOES_POP_MULTIPLIER_AREA_EXIST(id: i32) -> bool { invoke!(0x03BA619C81A646B3, id) }
	pub fn REMOVE_POP_MULTIPLIER_AREA(id: i32, p1: bool) { invoke_ignore!(0x88CB484364EFB37A, id, p1) }
	pub fn _ADD_POP_MULTIPLIER_VOLUME(volume: Volume, pedDensity: f32, vehicleDensity: f32, p3: bool, p4: bool) -> i32 { invoke!(0x3233C4EC0514C7EC, volume, pedDensity, vehicleDensity, p3, p4) }
	pub fn _DOES_POP_MULTIPLIER_AREA_EXIST_FOR_VOLUME(volume: Volume) -> bool { invoke!(0x39D6DACE323A20B6, volume) }
	pub fn _REMOVE_POP_MULTIPLIER_AREA_FOR_VOLUME(volume: Volume, p1: i32) { invoke_ignore!(0xBD090F5B1DB82189, volume, p1) }
	pub fn _0xF569E33FB72ED28E() { invoke_ignore!(0xF569E33FB72ED28E) }
	pub fn RESET_DISPATCH_IDEAL_SPAWN_DISTANCE() { invoke_ignore!(0xC7817264BC4B6377) }
	pub fn SET_DISPATCH_IDEAL_SPAWN_DISTANCE(fIdealSpawnDistance: f32) { invoke_ignore!(0xEAB6823B82FBD283, fIdealSpawnDistance) }
	pub fn _RESET_DISPATCH_MIN_SPAWN_DISTANCE() { invoke_ignore!(0x96498D922D8D0D0A) }
	pub fn _SET_DISPATCH_MIN_SPAWN_DISTANCE(minSpawnDistance: f32) { invoke_ignore!(0x27A1B170AA8AF84C, minSpawnDistance) }
	pub fn _RESET_DISPATCH_MAX_SPAWN_DISTANCE() { invoke_ignore!(0x54EC7B6BC72BAD69) }
	pub fn _SET_DISPATCH_MAX_SPAWN_DISTANCE(maxSpawnDistance: f32) { invoke_ignore!(0x89314FB3463E28DE, maxSpawnDistance) }
	pub fn _0x4B0501A468B749F8() { invoke_ignore!(0x4B0501A468B749F8) }
	pub fn _0x6BCF7B5CD338281A(p0: Any, p1: Any, p2: Any) { invoke_ignore!(0x6BCF7B5CD338281A, p0, p1, p2) }
	pub fn _ADD_DISPATCH_SPAWN_BLOCKING_AREA(volume: Volume) -> Any { invoke!(0xA2D5A26208421426, volume) }
	pub fn REMOVE_DISPATCH_SPAWN_BLOCKING_AREA(p0: Any) { invoke_ignore!(0x49F751F6868DDC5B, p0) }
	pub fn RESET_WANTED_RESPONSE_NUM_PEDS_TO_SPAWN() { invoke_ignore!(0xEF42F56F69877125) }
	pub fn ADD_TACTICAL_NAV_MESH_POINT(x: f32, y: f32, z: f32, p3: i32) { invoke_ignore!(0xE4EE55E63FA9AF45, x, y, z, p3) }
	pub fn CLEAR_TACTICAL_NAV_MESH_POINTS() { invoke_ignore!(0xD93B6516C6878267) }
	pub fn DISPLAY_ONSCREEN_KEYBOARD(textType: i32, windowTitle: & CStr, p2: & CStr, defaultText: & CStr, defaultConcat1: & CStr, defaultConcat2: & CStr, defaultConcat3: & CStr, maxInputLength: i32) { invoke_ignore!(0x044131118D8DB3CD, textType, windowTitle, p2, defaultText, defaultConcat1, defaultConcat2, defaultConcat3, maxInputLength) }
	pub fn UPDATE_ONSCREEN_KEYBOARD() -> i32 { invoke!(0x37DF360F235A3893) }
	pub fn GET_ONSCREEN_KEYBOARD_RESULT() -> *const char { invoke!(0xAFB4CF58A4A292B1) }
	pub fn CANCEL_ONSCREEN_KEYBOARD() { invoke_ignore!(0x58A39BE597CE99CD) }
	pub fn NEXT_ONSCREEN_KEYBOARD_RESULT_WILL_DISPLAY_USING_THESE_FONTS(fontBitField: i32) { invoke_ignore!(0x5CB71EAA1429A358, fontBitField) }
	pub fn ACTION_MANAGER_ENABLE_ACTION(hash: Hash, enable: bool) { invoke_ignore!(0x7ACF124C12A2B045, hash, enable) }
	pub fn ACTION_MANAGER_IS_ACTION_ENABLED(hash: Hash) -> bool { invoke!(0xFD0759658268FD8E, hash) }
	pub fn GET_REAL_WORLD_TIME() -> i32 { invoke!(0x2E036F0480B8BF02) }
	pub fn SET_SUPER_JUMP_THIS_FRAME(player: Player) { invoke_ignore!(0xB3E9BE963F10C445, player) }
	pub fn SCRIPT_RACE_INIT(numCheckpoints: i32, numLaps: i32, numPlayers: i32, p3: Any) { invoke_ignore!(0x8AE059F47158417E, numCheckpoints, numLaps, numPlayers, p3) }
	pub fn SCRIPT_RACE_SHUTDOWN() { invoke_ignore!(0x334CE0DA4FAF330C) }
	pub fn SCRIPT_RACE_PLAYER_HIT_CHECKPOINT(part: i32, checkpoint: i32, lap: i32, time: i32) { invoke_ignore!(0xBA62B4D80FA66BD6, part, checkpoint, lap, time) }
	pub fn SCRIPT_RACE_GET_PLAYER_SPLIT_TIME(p0: Any, p1: &mut Any, p2: &mut Any) -> bool { invoke!(0x769E848C66E3C2BB, p0, p1, p2) }
	pub fn START_END_USER_BENCHMARK() { invoke_ignore!(0x29D1F6DF864A094E) }
	pub fn STOP_END_USER_BENCHMARK() { invoke_ignore!(0xB89AEC71AFF2B599) }
	pub fn RESET_END_USER_BENCHMARK() { invoke_ignore!(0xECBABD0307FB216F) }
	pub fn SAVE_END_USER_BENCHMARK() { invoke_ignore!(0xF4743E2ECC02B3DA) }
	pub fn UI_STARTED_END_USER_BENCHMARK() -> bool { invoke!(0x4FFA0386A6216113) }
	pub fn GET_BENCHMARK_ITERATIONS() -> i32 { invoke!(0x22FC52CF470CC98D) }
	pub fn GET_BENCHMARK_PASS() -> i32 { invoke!(0x9297DACF3A2CDFF7) }
	pub fn _0xDC057B86FC157031() -> Any { invoke!(0xDC057B86FC157031) }
	pub fn _0x9A252AA23D7098F2() { invoke_ignore!(0x9A252AA23D7098F2) }
	pub fn _DOES_ITEM_HAVE_VALID_BASE(item: ScrHandle) -> bool { invoke!(0xBDC6E364C9C78178, item) }
	pub fn _GET_ITEM_TYPE(handle: ScrHandle) -> i32 { invoke!(0xDC8D2FF478DF9553, handle) }
	pub fn _IS_BASE_A_PERSISTENT_CHARACTER(handle: ScrHandle) -> bool { invoke!(0x716F17F8A0419F95, handle) }
	pub fn _IS_BASE_A_COVER_POINT(handle: ScrHandle) -> bool { invoke!(0xFEC1D4B5C82C176F, handle) }
	pub fn _0x553D67295DDD2309(entity: Entity) { invoke_ignore!(0x553D67295DDD2309, entity) }
	pub fn _GET_VOLUME_FROM_INDEXED_ITEM(item: ScrHandle) -> Volume { invoke!(0xF18AF483DF70BBDE, item) }
	pub fn _GET_ENTITY_FROM_ITEM(item: ScrHandle) -> Entity { invoke!(0xEE04C0AFD4EFAF0E, item) }
	pub fn _GET_PED_FROM_INDEXED_ITEM(item: ScrHandle) -> Ped { invoke!(0x3FFB15534067DCD4, item) }
	pub fn _GET_VEHICLE_FROM_INDEXED_ITEM(item: ScrHandle) -> Vehicle { invoke!(0xE578C8AE173719B3, item) }
	pub fn _GET_OBJECT_FROM_INDEXED_ITEM(item: ScrHandle) -> Object { invoke!(0x18013392501CE5DC, item) }
	pub fn _0x33982467B1E349EF(p0: Any, p1: Any, p2: Any, p3: Any, p4: Any, p5: Any, p6: Any) -> Any { invoke!(0x33982467B1E349EF, p0, p1, p2, p3, p4, p5, p6) }
	pub fn _0x8314FC2013ECE2DA(p0: Any, p1: Any, p2: Any) -> Any { invoke!(0x8314FC2013ECE2DA, p0, p1, p2) }
	pub fn _0x4D5C9CC7E7E23E09() { invoke_ignore!(0x4D5C9CC7E7E23E09) }
	pub fn REGISTER_INTERACTION_LOCKON_PROMPT(entity: Entity, text: & CStr, radius: f32, p3: f32, flag: i32, p5: f32, p6: f32, prompt: Prompt, p8: bool, p9: i32) -> bool { invoke!(0x870708A6E147A9AD, entity, text, radius, p3, flag, p5, p6, prompt, p8, p9) }
	pub fn UNREGISTER_INTERACTION_LOCKON_PROMPT(entity: Entity) -> bool { invoke!(0xE98D55C5983F2509, entity) }
	pub fn _0xFC6ECB9170145ECE() { invoke_ignore!(0xFC6ECB9170145ECE) }
	pub fn _0x35165C658077CD0B() -> Any { invoke!(0x35165C658077CD0B) }
	pub fn _0x4B101DBCC9482F2D(ped: Ped) -> bool { invoke!(0x4B101DBCC9482F2D, ped) }
	pub fn _IS_MISSION_CREATOR_ACTIVE() -> bool { invoke!(0xF236C84C6ADFCB2F) }
	pub fn _0xA3A8926951471C82() { invoke_ignore!(0xA3A8926951471C82) }
	pub fn _CREATE_AI_MEMORY(args: &mut Any, aiMemoryType: i32) { invoke_ignore!(0x88BC5F4AEF77FC4E, args, aiMemoryType) }
	pub fn _GET_AI_PED_DOES_HAVE_EVENT_MEMORY(args: &mut Any, p1: i32) -> bool { invoke!(0xFDF38E2B711BF78E, args, p1) }
	pub fn _0xEB946B9E579729AD(ped: Ped, p1: Any) { invoke_ignore!(0xEB946B9E579729AD, ped, p1) }
	pub fn _SET_AI_MEMORY_REACTIONS_ENABLED(enabled: bool) { invoke_ignore!(0x6AC4AF46A6B8DFB2, enabled) }
	pub fn _0x8DB104CCEBCD58C5(p0: Any, p1: Any) -> Any { invoke!(0x8DB104CCEBCD58C5, p0, p1) }
	pub fn _0x68319452C5064ABA(p0: Any, p1: Any) { invoke_ignore!(0x68319452C5064ABA, p0, p1) }
	pub fn _0xDE2C3B74D2B3705C(p0: Any, p1: Any) { invoke_ignore!(0xDE2C3B74D2B3705C, p0, p1) }
	pub fn _0x49F3241C28EBBFBC(p0: i32) { invoke_ignore!(0x49F3241C28EBBFBC, p0) }
	pub fn _0x183672FE838A661B(data: &mut Any) { invoke_ignore!(0x183672FE838A661B, data) }
	pub fn _0x38C0C9CAE1544500(p0: Hash) { invoke_ignore!(0x38C0C9CAE1544500, p0) }
	pub fn _0x154340E87D8CC178(p0: Any) { invoke_ignore!(0x154340E87D8CC178, p0) }
	pub fn _0x94FCADCF9F0C368E(p0: Any) { invoke_ignore!(0x94FCADCF9F0C368E, p0) }
	pub fn _0x0D0AE5081F88CFE1(p0: Hash) -> bool { invoke!(0x0D0AE5081F88CFE1, p0) }
	pub fn _0xAF3A84C7DE6A1DC5(p0: Any, p1: Any) { invoke_ignore!(0xAF3A84C7DE6A1DC5, p0, p1) }
	pub fn _LOOT_TABLES_GET_INFO(ped: Ped, p1: bool, p2: bool, lootTableKey: Hash, p4: &mut Any, p5: Any) { invoke_ignore!(0x48E4D50F87A96AA5, ped, p1, p2, lootTableKey, p4, p5) }
	pub fn _0xB1F6665AA54DCD5C(p0: Any) -> Any { invoke!(0xB1F6665AA54DCD5C, p0) }
	pub fn _0x8BB99B85444544D9(p0: Any, p1: Any) -> Any { invoke!(0x8BB99B85444544D9, p0, p1) }
	pub fn _0x6F02B5E50511721E(p0: Any) -> Any { invoke!(0x6F02B5E50511721E, p0) }
	pub fn _0xCC1BAF72D571DB8D(p0: Any, p1: Any, p2: Any) -> Any { invoke!(0xCC1BAF72D571DB8D, p0, p1, p2) }
	pub fn _IS_PED_DECOMPOSED(ped: Ped) -> bool { invoke!(0x5170DDA6D63ACAAA, ped) }
	pub fn SET_PED_DECOMPOSED(ped: Ped, toggle: bool) { invoke_ignore!(0x674B90BE1115846D, ped, toggle) }
	pub fn _0xAB26DEEE120FD3FD(p0: Any, p1: Any) { invoke_ignore!(0xAB26DEEE120FD3FD, p0, p1) }
	pub fn _0x082C043C7AFC3747(compositeId: i32, p1: bool) { invoke_ignore!(0x082C043C7AFC3747, compositeId, p1) }
	pub fn DISABLE_LOOTING_COMPOSITE_LOOTABLE_THIS_FRAME(compositeId: i32, p1: bool) { invoke_ignore!(0x40D72189F46D2E15, compositeId, p1) }
	pub fn _0xBB282CF5D2333FB8(p0: Any, p1: Any) { invoke_ignore!(0xBB282CF5D2333FB8, p0, p1) }
	pub fn _0xAD44856A1CD29635(p0: Any, p1: Any, p2: Any) { invoke_ignore!(0xAD44856A1CD29635, p0, p1, p2) }
	pub fn _0x38C2BF94D15F464D(p0: Any) -> Any { invoke!(0x38C2BF94D15F464D, p0) }
	pub fn _0x3A87FDA8F1B6CDFB(p0: Any, p1: Any, p2: Any) { invoke_ignore!(0x3A87FDA8F1B6CDFB, p0, p1, p2) }
	pub fn _0x4647842FE8F31C1E(p0: Any, p1: Any) { invoke_ignore!(0x4647842FE8F31C1E, p0, p1) }
	pub fn _GET_LOOTING_EVENT_HAS_FIRED(ped: Ped, eventName: & CStr) -> bool { invoke!(0xF9B91C5129EABC08, ped, eventName) }
	pub fn _SET_LOOT_PELT_SATCHEL_ITEM(ped: Ped, item: Any) { invoke_ignore!(0x9B47971234169990, ped, item) }
	pub fn _0x96282005C5C6801F(p0: Any, p1: Any) { invoke_ignore!(0x96282005C5C6801F, p0, p1) }
	pub fn _0xF63FA29D4A9ACA86(p0: Any, p1: Any) { invoke_ignore!(0xF63FA29D4A9ACA86, p0, p1) }
	pub fn _0x8C0F6A3D7236DEEB(p0: Any, p1: Any) { invoke_ignore!(0x8C0F6A3D7236DEEB, p0, p1) }
	pub fn _0x7FA58CED69405F9A(p0: Any, p1: Any) { invoke_ignore!(0x7FA58CED69405F9A, p0, p1) }
	pub fn _0xA08111B053D84B4D(p0: Any) { invoke_ignore!(0xA08111B053D84B4D, p0) }
	pub fn _IS_GLOBAL_BLOCK_VALID(index: i32) -> bool { invoke!(0xACB7E1418A8B6E32, index) }
	pub fn _SET_GLOBAL_BLOCK_IS_LOADED(index: i32, toggle: bool) { invoke_ignore!(0xE97240065406CB80, index, toggle) }
	pub fn GAME_FRAMEWORK_MANAGER_INIT(transitionMode: Hash) -> bool { invoke!(0x4CABE596D632E4B0, transitionMode) }
	pub fn _GAME_FRAMEWORK_MANAGER_SHUTDOWN() { invoke_ignore!(0xAFF2FD8ADD927585) }
	pub fn _GAME_FRAMEWORK_MANAGER_GET_MODE() -> Hash { invoke!(0xFAED234C7F53ABEB) }
	pub fn _IS_PLAYER_OWNING_STANDALONE_SP() -> bool { invoke!(0x36040772DF5E59A0) }
	pub fn _0x0358B8A41916C613(p0: Any, p1: Any, p2: Any, p3: Any) -> Any { invoke!(0x0358B8A41916C613, p0, p1, p2, p3) }
	pub fn _SET_GAME_LOGIC_PAUSED() { invoke_ignore!(0x550F05CFFBD63C8C) }
	pub fn STOP_CURRENT_LOADING_PROGRESS_TIMER() { invoke_ignore!(0xA565FAC215CBC77D) }
	pub fn _0xDBDA48EC456ED908() { invoke_ignore!(0xDBDA48EC456ED908) }
}
pub mod MISSIONDATA {
	use std::ffi::CStr;
	use crate::core::scripthook::native::*;
	use crate::core::types::*;

	pub fn MISSIONDATA_IS_VALID(p0: Any) -> bool { invoke!(0xE54DC27571D5EDC5, p0) }
	pub fn MISSIONDATA_GET_CATAGORY(missionId: Hash) -> Hash { invoke!(0x57E798B65C45EE17, missionId) }
	pub fn MISSIONDATA_GET_TEXTURE_NAME(missionId: Hash) -> Hash { invoke!(0x57E798B56C45EE15, missionId) }
	pub fn MISSIONDATA_GET_TEXTURE_TXD(missionId: Hash) -> Hash { invoke!(0x57E798B57C45EE16, missionId) }
	pub fn MISSIONDATA_WAS_COMPLETED(missionId: Hash) -> bool { invoke!(0xE54DC27571D5EDC4, missionId) }
	pub fn MISSIONDATA_GET_RATING(missionId: Hash) -> i32 { invoke!(0x57E798B54C45EE1A, missionId) }
	pub fn _MISSIONDATA_SET_MISSION_RATING(missionId: Hash, rating: i32) { invoke_ignore!(0xE824CE7D13FCB300, missionId, rating) }
	pub fn MISSIONDATA_IS_REQUIRED_STORY_MISSION(missionId: Hash) -> bool { invoke!(0xE824CE7D13FCB35E, missionId) }
	pub fn _MISSIONDATA_SET_REPLAY_STATE_LOCKED(missionId: Hash, replayState: i32) { invoke_ignore!(0xE4E2C581F127A11C, missionId, replayState) }
	pub fn MISSIONDATA_GET_REPLAY_STATE(p0: Any) -> i32 { invoke!(0x8C32D86E9556ED86, p0) }
	pub fn MISSIONDATA_SET_RATING_SCORES(missionId: Hash, bronzeScore: i32, silverScore: i32, goldScore: i32) { invoke_ignore!(0x12F65317708749A5, missionId, bronzeScore, silverScore, goldScore) }
	pub fn MISSIONDATA_SET_HIGH_SCORE(missionId: Hash, score: i32) { invoke_ignore!(0x3A04F0169DA87A9D, missionId, score) }
	pub fn MISSIONDATA_GET_HIGH_SCORE(missionId: Hash) -> i32 { invoke!(0x9AABABF8313C3516, missionId) }
	pub fn _MISSIONDATA_IS_REPLAY_CATEGORY_LOCKED(category: Hash) -> bool { invoke!(0xE145864DECC34219, category) }
	pub fn MISSIONDATA_SET_REPLAY_LOCKED_FOR_CATEGORY(category: Hash, locked: bool) { invoke_ignore!(0x957A830C9B4B99EA, category, locked) }
	pub fn _MISSIONDATA_TIMECYCLE_BOX_SET_MODIFIER(timecycleName: & CStr) { invoke_ignore!(0x25855B1574BF8CD5, timecycleName) }
	pub fn _MISSIONDATA_TIMECYCLE_BOX_DELETE() { invoke_ignore!(0x7F89E15A8FB8DE97) }
	pub fn _MISSIONDATA_TIMECYCLE_BOX_EXISTS() -> bool { invoke!(0x7E8F86A4FA33033C) }
}
pub mod MONEY {
	use std::ffi::CStr;
	use crate::core::scripthook::native::*;
	use crate::core::types::*;

	pub fn _MONEY_GET_CASH_BALANCE() -> i32 { invoke!(0x0C02DABFA3B98176) }
	pub fn _MONEY_DECREMENT_CASH_BALANCE(amount: i32) -> bool { invoke!(0x466BC8769CF26A7A, amount) }
	pub fn _MONEY_INCREMENT_CASH_BALANCE(amount: i32, addReason: Hash) -> bool { invoke!(0xBC3422DC91667621, amount, addReason) }
	pub fn _NETWORK_GET_STRING_CASH_BALANCE() -> *const char { invoke!(0x282D36FF103D78DF) }
	pub fn _NETWORK_IS_MONEY_BALANCE_NOT_LESS_THAN(cashBalance: i32, goldBarBalance: i32) -> bool { invoke!(0xAEC5F0119867E457, cashBalance, goldBarBalance) }
	pub fn _0xA46FD001D1BE896C() -> *const char { invoke!(0xA46FD001D1BE896C) }
	pub fn _0x07AD9E43FD478527(p0: Any, p1: Any) -> bool { invoke!(0x07AD9E43FD478527, p0, p1) }
	pub fn _NETWORK_GET_CASH_BALANCE() -> i32 { invoke!(0x8A67120DBC299525) }
}
pub mod NETSHOPPING {
	use std::ffi::CStr;
	use crate::core::scripthook::native::*;
	use crate::core::types::*;

	pub fn CASHINVENTORY_INIT_SESSION_STATUS(p0: &mut i32, p1: &mut i32) -> bool { invoke!(0xC019112F8995DC1C, p0, p1) }
	pub fn _CASHINVENTORY_IS_SESSION_READY() -> bool { invoke!(0xFCC24220FDDAC929) }
	pub fn CASHINVENTORY_IS_CONNECTION_FAULTED() -> bool { invoke!(0x6CE9FB6332B5E46E) }
	pub fn _CASHINVENTORY_INIT_SESSION_IS_FAULTED() -> bool { invoke!(0xD1CE92D1D9BE170A) }
	pub fn _CASHINVENTORY_TRANSACTION_FIRE_AND_FORGET_ITEM(actionHash: Hash, id: &mut i32, item: &mut Any, p3: i32) -> bool { invoke!(0xFFEA09CCEC4AF32F, actionHash, id, item, p3) }
	pub fn _CASHINVENTORY_TRANSACTION_START(id: &mut i32, type_: Hash, actionHash: Hash) -> bool { invoke!(0xF039EC27F4490E96, id, type_, actionHash) }
	pub fn _CASHINVENTORY_TRANSACTION_GET_BASKET_IS_VALID(id: i32) -> bool { invoke!(0x52A226ADF4A270D2, id) }
	pub fn _CASHINVENTORY_TRANSACTION_DELETE(id: i32) -> bool { invoke!(0x59EF5D516E2D96B9, id) }
	pub fn _CASHINVENTORY_TRANSACTION_VALIDATE_ITEM(p0: Hash, p1: &mut Any) -> i32 { invoke!(0x6C9F12700BCE69F4, p0, p1) }
	pub fn _0x38640A8C2DEF011B(p0: i32) -> i32 { invoke!(0x38640A8C2DEF011B, p0) }
	pub fn _0xA3B8D31C13CB4239(p0: i32, p1: Hash, p2: &mut Any, p3: i32, p4: &mut Any, p5: i32) -> bool { invoke!(0xA3B8D31C13CB4239, p0, p1, p2, p3, p4, p5) }
	pub fn _CASHINVENTORY_TRANSACTION_ADD_AWARD(id: i32, hash: Hash, p2: &mut Any, p3: &mut Any) -> bool { invoke!(0x52BDE32F21BA3B6D, id, hash, p2, p3) }
	pub fn _CASHINVENTORY_TRANSACTION_GET_ITEM_INFO(id: i32, index: i32, itemInfo: &mut Any) -> bool { invoke!(0x7616B5F0895C2D99, id, index, itemInfo) }
	pub fn _CASHINVENTORY_TRANSACTION_GET_NUM_OF_ITEMS(id: i32) -> i32 { invoke!(0xCF2D04D076847478, id) }
	pub fn _CASHINVENTORY_TRANSACTION_GET_ACTION(id: i32) -> Hash { invoke!(0xBD2D520C51CCFF52, id) }
	pub fn _CASHINVENTORY_TRANSACTION_CHECKOUT(id: i32) -> bool { invoke!(0x592BC00BF6629BE7, id) }
	pub fn _CASHINVENTORY_TRANSACTION_CHECKOUT_STATUS(id: i32, status: &mut i32) -> bool { invoke!(0x26C008791D066F37, id, status) }
	pub fn _0xB6F4557060EF0FB4(p0: i32, p1: i32) -> i32 { invoke!(0xB6F4557060EF0FB4, p0, p1) }
	pub fn _CASHINVENTORY_TRANSACTION_RESPONSE_GET_ITEM_INFO(id: i32, index: i32, itemInfo: &mut Any) -> bool { invoke!(0x98412398BBE73F61, id, index, itemInfo) }
	pub fn _0xCE54C9ABE6FBC6DB(p0: Hash) -> bool { invoke!(0xCE54C9ABE6FBC6DB, p0) }
	pub fn _0xA0B7094629724974(p0: Hash, p1: Any) -> bool { invoke!(0xA0B7094629724974, p0, p1) }
	pub fn _0x92A32BA29622763F(id: i32, index: i32, p2: &mut Any) -> bool { invoke!(0x92A32BA29622763F, id, index, p2) }
	pub fn _0x3FA09DD57B93C0DE(p0: Hash, p1: i32, p2: i32, p3: Any, p4: i32) -> bool { invoke!(0x3FA09DD57B93C0DE, p0, p1, p2, p3, p4) }
	pub fn _0xD1555FBC96C88444(p0: Hash, p1: i32, p2: i32, p3: Any, p4: i32) -> bool { invoke!(0xD1555FBC96C88444, p0, p1, p2, p3, p4) }
}
pub mod NETWORK {
	use std::ffi::CStr;
	use crate::core::scripthook::native::*;
	use crate::core::types::*;

	pub fn NETWORK_IS_SIGNED_ONLINE() -> bool { invoke!(0x1077788E268557C2) }
	pub fn NETWORK_GET_NP_UNAVAILABLE_REASON() -> i32 { invoke!(0x74FB3E29E6D10FA9) }
	pub fn NETWORK_HAS_VALID_ROS_CREDENTIALS() -> bool { invoke!(0x85443FF4C328F53B) }
	pub fn NETWORK_IS_CLOUD_AVAILABLE() -> bool { invoke!(0x9A4CF4F48AD77302) }
	pub fn NETWORK_HAS_SOCIAL_CLUB_ACCOUNT() -> bool { invoke!(0x67A5589628E0CFF6) }
	pub fn NETWORK_IS_HOST() -> bool { invoke!(0x8DB296B814EDDA07) }
	pub fn NETWORK_HAVE_ONLINE_PRIVILEGES() -> bool { invoke!(0x25CB5A9F37BFD063) }
	pub fn NETWORK_CHECK_USER_CONTENT_PRIVILEGES(p0: i32) -> bool { invoke!(0x595F028698072DD9, p0) }
	pub fn NETWORK_CHECK_COMMUNICATION_PRIVILEGES(p0: i32) -> bool { invoke!(0x83F28CE49FBBFFBA, p0) }
	pub fn NETWORK_CAN_VIEW_GAMER_USER_CONTENT(gamerHandle: &mut Any) -> bool { invoke!(0x246545C37C27A717, gamerHandle) }
	pub fn _0xF23A6D6C11D8EC15(gamerHandle: &mut Any) -> bool { invoke!(0xF23A6D6C11D8EC15, gamerHandle) }
	pub fn _0x3E8CCE6769DB5F34(p0: i32) -> i32 { invoke!(0x3E8CCE6769DB5F34, p0) }
	pub fn _0xDBDF80673BBA3D65(p0: i32) -> bool { invoke!(0xDBDF80673BBA3D65, p0) }
	pub fn NETWORK_SHOW_ACCOUNT_UPGRADE_UI() { invoke_ignore!(0x83FE8D7229593017) }
	pub fn NETWORK_IS_PROMOTION_ENABLED() -> bool { invoke!(0x8FF6059DA26E688A) }
	pub fn NETWORK_IS_CUSTOM_UPSELL_ENABLED() -> bool { invoke!(0x78A9535AF83715C6) }
	pub fn NETWORK_SHOULD_SHOW_PROMOTION_DLG() -> bool { invoke!(0xDA4B1A479C414FB2) }
	pub fn _0xFC6FCF4C03F1BBF6() { invoke_ignore!(0xFC6FCF4C03F1BBF6) }
	pub fn _0x160F0CE6D76A39C9() -> Any { invoke!(0x160F0CE6D76A39C9) }
	pub fn NETWORK_GET_PROMOTION_DLG_SEEN_COUNT() -> i32 { invoke!(0x2FB53C631A49BE92) }
	pub fn _0xE5FF65CFF5160752() { invoke_ignore!(0xE5FF65CFF5160752) }
	pub fn NETWORK_CAN_ACCESS_MULTIPLAYER(loadingState: &mut i32) -> bool { invoke!(0xAF50DA1A3F8B1BA4, loadingState) }
	pub fn NETWORK_CHECK_ACCESS_AND_ALERT_IF_FAIL() -> bool { invoke!(0x2A8112A974DE1EF6) }
	pub fn _NETWORK_GET_GLOBAL_ENTITY_FLAGS(entity: Entity) -> i32 { invoke!(0xDD7806FD0543BC3D, entity) }
	pub fn _0xA95470DA137587F5(p0: bool) { invoke_ignore!(0xA95470DA137587F5, p0) }
	pub fn _0xBB697756309D77EE(p0: bool) -> Any { invoke!(0xBB697756309D77EE, p0) }
	pub fn _NETWORK_HAS_COMPLETED_MP_INTRO_FLOW_ON_CURRENT_SLOT() -> bool { invoke!(0xDD73C9838CE7181D) }
	pub fn NETWORK_SET_COMPLETED_MP_INTRO_FLOW_ON_CURRENT_SLOT(completed: bool) -> bool { invoke!(0x2C5BD9A43987AA27, completed) }
	pub fn _0xD7D0DF27CB1765B5(p0: i32) -> bool { invoke!(0xD7D0DF27CB1765B5, p0) }
	pub fn NETWORK_SET_MP_MISSION_FLAG_ON_CURRENT_SLOT(enabled: bool, flagIndex: i32) -> bool { invoke!(0x86FD10251A7118A4, enabled, flagIndex) }
	pub fn _0x3E74A687A73979C6(p0: bool) { invoke_ignore!(0x3E74A687A73979C6, p0) }
	pub fn NETWORK_SESSION_IS_PRIVATE() -> bool { invoke!(0xCEF70AA5B3F89BA1) }
	pub fn _NETWORK_SESSION_GET_SESSION_ID(sessionId: &mut Any) { invoke_ignore!(0xE9B356C330C0A806, sessionId) }
	pub fn _NETWORK_SESSION_ARE_SESSION_IDS_EQUAL(sessionId1: &mut Any, sessionId2: &mut Any) -> bool { invoke!(0x4DEC5000F7B508F0, sessionId1, sessionId2) }
	pub fn NETWORK_REQUEST_SESSION_SEAMLESS(flags: i32, seamlessType: i32, sessionRequestId: &mut Any) -> bool { invoke!(0x04019AE4956D4393, flags, seamlessType, sessionRequestId) }
	pub fn NETWORK_SESSION_REQUEST_SESSION_SEAMLESS(flags: i32, seamlessType: i32, userHash: i32, sessionRequestId: &mut Any) -> bool { invoke!(0x2989E131FDE37E97, flags, seamlessType, userHash, sessionRequestId) }
	pub fn NETWORK_SESSION_REQUEST_SESSION_COMPETITIVE(flags: i32, matchType: i32, userHash: i32, p3: i32, sessionRequestId: &mut Any) -> bool { invoke!(0x309BBEBEA8A3986C, flags, matchType, userHash, p3, sessionRequestId) }
	pub fn NETWORK_SESSION_REQUEST_SESSION_PRIVATE(flags: i32, numPlayers: i32, userHash: i32, sessionRequestId: &mut Any) -> bool { invoke!(0x39A8EF7AF29A192C, flags, numPlayers, userHash, sessionRequestId) }
	pub fn _NETWORK_SESSION_REQUEST_SESSION_ON_CALL(flags: i32, category: i32, p2: &mut Any, userHash: i32, sessionRequestId: &mut Any) -> bool { invoke!(0x23D9C1F2E4098EDC, flags, category, p2, userHash, sessionRequestId) }
	pub fn _NETWORK_SESSION_REQUEST_SESSION_NOMINATED(flags: i32, userHash: i32, p2: i32, sessionRequestId: &mut Any) -> bool { invoke!(0x4F4672457FF597D1, flags, userHash, p2, sessionRequestId) }
	pub fn NETWORK_SESSION_IS_SESSION_REQUEST_ID_VALID(sessionRequestId: &mut Any) -> bool { invoke!(0x2F54B146D3EDCE4D, sessionRequestId) }
	pub fn NETWORK_SESSION_GET_SESSION_TYPE() -> i32 { invoke!(0xF0C0C94B404206FA) }
	pub fn _0x1413B6BF27AB7A95() -> i32 { invoke!(0x1413B6BF27AB7A95) }
	pub fn NETWORK_SESSION_IS_ANY_REQUEST_IN_PROGRESS() -> bool { invoke!(0xBAFFDE5F953720D9) }
	pub fn _0xAFA14F98327791CE(sessionRequestId: &mut Any) -> bool { invoke!(0xAFA14F98327791CE, sessionRequestId) }
	pub fn NETWORK_SESSION_IS_REQUEST_IN_PROGRESS(sessionRequestId: &mut Any) -> bool { invoke!(0x8FB7C254CFCBF78E, sessionRequestId) }
	pub fn _NETWORK_SESSION_IS_REQUEST_IN_PROGRESS_BY_QUEUE_GROUP(queueGroup: i32) -> bool { invoke!(0x9E762A595CF88E4A, queueGroup) }
	pub fn _NETWORK_SESSION_CANCEL_REQUEST(sessionRequestId: &mut Any) -> bool { invoke!(0xE72E5C1289BD1F40, sessionRequestId) }
	pub fn _0xA6F1BAABFF6AD7B9(p0: &mut Any) { invoke_ignore!(0xA6F1BAABFF6AD7B9, p0) }
	pub fn _NETWORK_SESSION_GET_SESSION_REQUEST_RESULT(sessionRequestId: &mut Any, p1: &mut i32) -> i32 { invoke!(0x0DD051B1BF4B8BD6, sessionRequestId, p1) }
	pub fn NETWORK_SESSION_IS_REQUEST_PENDING_TRANSITION(sessionRequestId: &mut Any) -> bool { invoke!(0xCCF878D50F8AB10D, sessionRequestId) }
	pub fn _NETWORK_SESSION_TRANSITION_TO_SESSION(sessionRequestId: &mut Any) -> bool { invoke!(0xF20B18A330E6DB5C, sessionRequestId) }
	pub fn _NETWORK_SESSION_IS_NSRR_SUCCESS(sessionRequestId: &mut Any) -> bool { invoke!(0x0F44A5C78D114922, sessionRequestId) }
	pub fn NETWORK_SESSION_LEFT_QUEUE_OR_REQUESTED_SESSION(sessionRequestId: &mut Any) -> bool { invoke!(0xECE6A0C1B59CD8BE, sessionRequestId) }
	pub fn NETWORK_SESSION_LEAVE_SESSION() -> bool { invoke!(0x17C21B7319A05047) }
	pub fn NETWORK_SESSION_IS_TRANSITIONING() -> bool { invoke!(0xF2CBC969C4F090C7) }
	pub fn _0xFD4272A137703449() { invoke_ignore!(0xFD4272A137703449) }
	pub fn _NETWORK_SESSION_PLAYLIST_GO_TO_NEXT_CONTENT() { invoke_ignore!(0xBDE605F925B07127) }
	pub fn _NETWORK_SESSION_PLAYLIST_GET_UPCOMING_CONTENT() { invoke_ignore!(0x8F9DB6CD03B42B58) }
	pub fn NETWORK_DISABLE_REALTIME_MULTIPLAYER() { invoke_ignore!(0x236905C700FDB54D) }
	pub fn _0x71FA2D1880C48032(p0: bool) { invoke_ignore!(0x71FA2D1880C48032, p0) }
	pub fn NETWORK_GET_GLOBAL_MULTIPLAYER_CLOCK(hours: &mut i32, minutes: &mut i32, seconds: &mut i32) { invoke_ignore!(0x6D03BFBD643B2A02, hours, minutes, seconds) }
	pub fn NETWORK_CLEAR_CLOCK_TIME_OVERRIDE() { invoke_ignore!(0xD972DF67326F966E) }
	pub fn NETWORK_IS_CLOCK_TIME_OVERRIDDEN() -> bool { invoke!(0xD7C95D322FF57522) }
	pub fn NETWORK_GET_GLOBAL_CLOCK(hour: &mut i32, minute: &mut i32, second: &mut i32) -> bool { invoke!(0x11A7ADCD629E170F, hour, minute, second) }
	pub fn _NETWORK_CLOCK_TIME_OVERRIDE(hour: i32, minute: i32, second: i32, transitionTime: i32, pauseClock: bool) { invoke_ignore!(0x669E223E64B1903C, hour, minute, second, transitionTime, pauseClock) }
	pub fn _NETWORK_CLOCK_TIME_OVERRIDE_2(hour: i32, minute: i32, second: i32, transitionTime: i32, pauseClock: bool, clockwise: bool) { invoke_ignore!(0xE28C13ECC36FF14E, hour, minute, second, transitionTime, pauseClock, clockwise) }
	pub fn _NETWORK_CLEAR_CLOCK_OVERRIDE_OVERTIME(milliseconds: i32) { invoke_ignore!(0x65F040D91001ED4B, milliseconds) }
	pub fn _0x0E54D4DA6018FF8E() -> bool { invoke!(0x0E54D4DA6018FF8E) }
	pub fn NETWORK_IS_FINDING_GAMERS() -> bool { invoke!(0xDDDF64C91BFCF0AA) }
	pub fn NETWORK_DID_FIND_GAMERS_SUCCEED() -> bool { invoke!(0xF9B83B77929D8863) }
	pub fn _0x7BCA0A3972708436(outData: &mut Any, p1: i32) -> i32 { invoke!(0x7BCA0A3972708436, outData, p1) }
	pub fn NETWORK_CLEAR_FOUND_GAMERS() { invoke_ignore!(0x6D14CCEE1B40381A) }
	pub fn _NETWORK_GET_GAMER_SESSION_FROM_HANDLE(data: &mut Any, count: i32) -> bool { invoke!(0xFBDFE1C1356E12E8, data, count) }
	pub fn _NETWORK_HAS_CURRENT_GET_GAMER_STATUS_STARTED() -> bool { invoke!(0x25189F9908E9CD65) }
	pub fn NETWORK_DID_GET_GAMER_STATUS_SUCCEED() -> bool { invoke!(0x5AE17C6B0134B7F1) }
	pub fn _NETWORK_GET_GAMER_STATUS(gamerHandle: &mut Any, p1: i32) -> i32 { invoke!(0xDDAEB478E58F8DEA, gamerHandle, p1) }
	pub fn NETWORK_CLEAR_GET_GAMER_STATUS() { invoke_ignore!(0x86E0660E4F5C956D) }
	pub fn NETWORK_SET_SCRIPT_READY_FOR_EVENTS(toggle: bool) { invoke_ignore!(0x7AC752103856FB20, toggle) }
	pub fn _0x316FD416C432C761() -> bool { invoke!(0x316FD416C432C761) }
	pub fn _0x062842D61D0D53FD() -> bool { invoke!(0x062842D61D0D53FD) }
	pub fn NETWORK_IS_GAME_IN_PROGRESS() -> bool { invoke!(0x10FAB35428CCC9D7) }
	pub fn NETWORK_IS_SESSION_ACTIVE() -> bool { invoke!(0xD83C2B94E7508980) }
	pub fn NETWORK_IS_IN_SESSION() -> bool { invoke!(0xCA97246103B63917) }
	pub fn _NETWORK_IS_IN_SESSION_LOBBY() -> bool { invoke!(0xC5196C42DE19F646) }
	pub fn NETWORK_IS_SESSION_STARTED() -> bool { invoke!(0x9DE624D2FC4B603F) }
	pub fn NETWORK_CAN_SESSION_END() -> bool { invoke!(0x4EEBC3694E49C572) }
	pub fn NETWORK_GET_GAME_MODE() -> i32 { invoke!(0x225640E09EFFDC3F) }
	pub fn _NETWORK_SESSION_ADD_SESSION_FLAGS(flags: i32) -> bool { invoke!(0xE546BDA1B3E288EE, flags) }
	pub fn NETWORK_SESSION_REMOVE_SESSION_FLAGS(flags: i32) -> bool { invoke!(0x78335E12DB0BF961, flags) }
	pub fn NETWORK_SESSION_GET_SESSION_FLAGS() -> i32 { invoke!(0x51F33DBC1A41CBFD) }
	pub fn _NETWORK_SESSION_SET_PLAYER_FLAGS(flags: i32) -> bool { invoke!(0x0AE241A4A9ADEEEC, flags) }
	pub fn _NETWORK_SESSION_REMOVE_PLAYER_FLAGS(flags: i32) -> bool { invoke!(0x3215BBE34D3418C5, flags) }
	pub fn _NETWORK_GET_SESSION_HOST() -> Player { invoke!(0x8DC9AA3B508B1A85) }
	pub fn _0xD3A3C8B9F3BDEF81() -> Any { invoke!(0xD3A3C8B9F3BDEF81) }
	pub fn _0x18B94666CF610AEB() -> bool { invoke!(0x18B94666CF610AEB) }
	pub fn _0x981146E5C9CE9250(inviteIndex: i32) -> bool { invoke!(0x981146E5C9CE9250, inviteIndex) }
	pub fn _0xBF8276E51761F9DA() -> i32 { invoke!(0xBF8276E51761F9DA) }
	pub fn _0xDCA4A74135E1DEA5(p0: Any) -> bool { invoke!(0xDCA4A74135E1DEA5, p0) }
	pub fn NETWORK_HAS_PENDING_INVITE_FAILURE() -> bool { invoke!(0xD0498AD30E16B6BD) }
	pub fn _NETWORK_CAN_RECEIVE_INVITE_FROM_HANDLE(gamerHandle: &mut Any) -> bool { invoke!(0xF23D6475640D29EB, gamerHandle) }
	pub fn _0x704F92B3AF20D857(setting: bool) { invoke_ignore!(0x704F92B3AF20D857, setting) }
	pub fn _0xF342F6BD0A8287D5(p0: Any) { invoke_ignore!(0xF342F6BD0A8287D5, p0) }
	pub fn _0xD39A72AE5EBD57E5() { invoke_ignore!(0xD39A72AE5EBD57E5) }
	pub fn _NETWORK_SEND_SESSION_INVITE(gamerHandle: &mut Any, contentId: & CStr, data: &mut Any, dataSize: i32, p4: i32, flags: i32) -> bool { invoke!(0xE47001B7CB8B98AE, gamerHandle, contentId, data, dataSize, p4, flags) }
	pub fn _0xD1FFB246F4E088AC(p0: i32) -> bool { invoke!(0xD1FFB246F4E088AC, p0) }
	pub fn _0x27B1AE4D8C652F08(p0: i32) -> i32 { invoke!(0x27B1AE4D8C652F08, p0) }
	pub fn _0x6C27442A225A241A(p0: i32) -> i32 { invoke!(0x6C27442A225A241A, p0) }
	pub fn _0xE59F4924BD3A718D(p0: i32) -> *const char { invoke!(0xE59F4924BD3A718D, p0) }
	pub fn _0x78271BC02AE9AF83(p0: i32) -> i32 { invoke!(0x78271BC02AE9AF83, p0) }
	pub fn _0x16EFB123C4451032(p0: i32, gamerHandle: &mut Any) -> bool { invoke!(0x16EFB123C4451032, p0, gamerHandle) }
	pub fn _0xE79BA3BC265895DA(p0: i32) -> *const char { invoke!(0xE79BA3BC265895DA, p0) }
	pub fn _0xC0CFFDA87C2C163D(p0: i32, p1: Any, p2: i32) -> Any { invoke!(0xC0CFFDA87C2C163D, p0, p1, p2) }
	pub fn _0x5ED39DA62BEB1330(p0: i32) -> Any { invoke!(0x5ED39DA62BEB1330, p0) }
	pub fn NETWORK_ACCEPT_RS_INVITE(p0: i32) -> bool { invoke!(0xB2CEA5105AAC8DDE, p0) }
	pub fn _0x3AA0CDC63696166D(p0: i32) -> bool { invoke!(0x3AA0CDC63696166D, p0) }
	pub fn NETWORK_REQUEST_JOIN(p0: Any) -> i32 { invoke!(0xE483BB6BE686F632, p0) }
	pub fn _0xE8E633215471BB5D(p0: Any) -> i32 { invoke!(0xE8E633215471BB5D, p0) }
	pub fn _0xA2837A5E21FB5A58(p0: Any) -> bool { invoke!(0xA2837A5E21FB5A58, p0) }
	pub fn _0xE39600E50D608693(p0: Any, p1: Any) -> bool { invoke!(0xE39600E50D608693, p0, p1) }
	pub fn _0xD7BAD4062074B9C1(p0: Any) -> bool { invoke!(0xD7BAD4062074B9C1, p0) }
	pub fn _0xCA58D4FD20D70F24(p0: Any) -> i32 { invoke!(0xCA58D4FD20D70F24, p0) }
	pub fn _0xC028B3F52C707C49(p0: Any) -> bool { invoke!(0xC028B3F52C707C49, p0) }
	pub fn NETWORK_IS_PLATFORM_INVITE_PENDING() -> bool { invoke!(0xFC4165C9165C166F) }
	pub fn _0x5B9C6AC118FD4774() { invoke_ignore!(0x5B9C6AC118FD4774) }
	pub fn _NETWORK_GET_PLATFORM_INVITE_ID() -> i32 { invoke!(0x9BCF28FB5D65A9BE) }
	pub fn NETWORK_ACTION_PLATFORM_INVITE() -> bool { invoke!(0x3B82ACC3F4B6240C) }
	pub fn NETWORK_CLEAR_PLATFORM_INVITE() { invoke_ignore!(0xA4484173759749B1) }
	pub fn _0x603469298A4308AF(p0: bool) { invoke_ignore!(0x603469298A4308AF, p0) }
	pub fn _NETWORK_ARE_ONLINE_NOTIFICATIONS_SHOWN_IN_STORY_MODE() -> bool { invoke!(0xF5C5929E07512F80) }
	pub fn NETWORK_IS_IN_PLATFORM_PARTY() -> bool { invoke!(0x2FC5650B0271CB57) }
	pub fn _NETWORK_ARE_PLAYERS_IN_SAME_PLATFORM_PARTY(gamerHandle1: &mut Any, gamerHandle2: &mut Any) -> bool { invoke!(0x11820D1AE80DEA39, gamerHandle1, gamerHandle2) }
	pub fn NETWORK_IS_IN_PLATFORM_PARTY_CHAT() -> bool { invoke!(0xFD8B834A8BA05048) }
	pub fn NETWORK_SEED_RANDOM_NUMBER_GENERATOR(seed: i32) { invoke_ignore!(0xF1B84178F8674195, seed) }
	pub fn NETWORK_GET_RANDOM_INT_RANGED(rangeStart: i32, rangeEnd: i32) -> i32 { invoke!(0xE30CF56F1EFA5F43, rangeStart, rangeEnd) }
	pub fn NETWORK_SET_THIS_SCRIPT_IS_NETWORK_SCRIPT(maxNumMissionParticipants: i32, p1: bool, instanceId: i32) { invoke_ignore!(0x1CA59E306ECB80A5, maxNumMissionParticipants, p1, instanceId) }
	pub fn NETWORK_GET_THIS_SCRIPT_IS_NETWORK_SCRIPT() -> bool { invoke!(0x2910669969E9535E) }
	pub fn NETWORK_GET_MAX_NUM_PARTICIPANTS() -> i32 { invoke!(0xA6C90FBC38E395EE) }
	pub fn NETWORK_GET_NUM_PARTICIPANTS() -> i32 { invoke!(0x18D0456E86604654) }
	pub fn NETWORK_GET_SCRIPT_STATUS() -> i32 { invoke!(0x57D158647A6BFABF) }
	pub fn NETWORK_REGISTER_HOST_BROADCAST_VARIABLES(p0: Any, p1: Any, p2: Any) { invoke_ignore!(0x3E9B2F01C50DF595, p0, p1, p2) }
	pub fn _NETWORK_GET_SIZE_OF_HOST_BROADCAST_DATA_STORAGE(p0: &mut i32) -> i32 { invoke!(0xBA24095EA96DFE17, p0) }
	pub fn NETWORK_REGISTER_PLAYER_BROADCAST_VARIABLES(p0: Any, p1: Any, p2: Any) { invoke_ignore!(0x3364AA97340CA215, p0, p1, p2) }
	pub fn _NETWORK_GET_SIZE_OF_PLAYER_BROADCAST_DATA_STORAGE(p0: &mut i32) -> i32 { invoke!(0x690806BC83BC8CA2, p0) }
	pub fn NETWORK_HAS_RECEIVED_HOST_BROADCAST_DATA() -> bool { invoke!(0x5D10B3795F3FC886) }
	pub fn NETWORK_GET_PLAYER_INDEX(player: Player) -> i32 { invoke!(0x24FB80D107371267, player) }
	pub fn NETWORK_GET_PARTICIPANT_INDEX(index: i32) -> i32 { invoke!(0x1B84DF6AF2A46938, index) }
	pub fn NETWORK_GET_PLAYER_INDEX_FROM_PED(ped: Ped) -> Player { invoke!(0x6C0E2E0125610278, ped) }
	pub fn NETWORK_GET_NUM_CONNECTED_PLAYERS() -> i32 { invoke!(0xA4A79DD2D9600654) }
	pub fn NETWORK_IS_PLAYER_CONNECTED(player: Player) -> bool { invoke!(0x93DC1BE4E1ABE9D1, player) }
	pub fn NETWORK_GET_TOTAL_NUM_PLAYERS() -> i32 { invoke!(0xCF61D4B4702EE9EB) }
	pub fn NETWORK_IS_PARTICIPANT_ACTIVE(p0: i32) -> bool { invoke!(0x6FF8FF40B6357D45, p0) }
	pub fn NETWORK_IS_PLAYER_ACTIVE(player: Player) -> bool { invoke!(0xB8DFD30D6973E135, player) }
	pub fn NETWORK_IS_PLAYER_A_PARTICIPANT(player: Player) -> bool { invoke!(0x3CA58F6CB7CBD784, player) }
	pub fn NETWORK_IS_HOST_OF_THIS_SCRIPT() -> bool { invoke!(0x83CD99A1E6061AB5) }
	pub fn NETWORK_GET_HOST_OF_THIS_SCRIPT() -> Player { invoke!(0xC7B4D79B01FA7A5C) }
	pub fn NETWORK_GET_HOST_OF_SCRIPT(scriptName: & CStr, p1: i32, p2: i32) -> Player { invoke!(0x1D6A14F1F9A736FC, scriptName, p1, p2) }
	pub fn NETWORK_GET_HOST_OF_THREAD(threadId: i32) -> Player { invoke!(0xB4A25351D79B444C, threadId) }
	pub fn NETWORK_SET_MISSION_FINISHED() { invoke_ignore!(0x3B3D11CD9FFCDFC9) }
	pub fn NETWORK_IS_SCRIPT_ACTIVE(scriptName: & CStr, p1: i32, p2: bool, p3: i32) -> bool { invoke!(0x9D40DF90FAD26098, scriptName, p1, p2, p3) }
	pub fn NETWORK_IS_SCRIPT_ACTIVE_BY_HASH(scriptHash: Hash, p1: i32, p2: bool, p3: i32) -> bool { invoke!(0x1B89BC43B6E69107, scriptHash, p1, p2, p3) }
	pub fn _NETWORK_IS_THREAD_ACTIVE(threadId: i32) -> bool { invoke!(0x31DAD2CD6D49546E, threadId) }
	pub fn _NETWORK_GET_INSTANCE_ID_OF_THREAD(threadId: i32) -> i32 { invoke!(0xFB9ECED5B68F3B78, threadId) }
	pub fn NETWORK_GET_NUM_SCRIPT_PARTICIPANTS(scriptName: & CStr, instanceId: i32, position: Hash) -> i32 { invoke!(0x3658E8CD94FC121A, scriptName, instanceId, position) }
	pub fn NETWORK_GET_INSTANCE_ID_OF_THIS_SCRIPT() -> i32 { invoke!(0x638A3A81733086DB) }
	pub fn NETWORK_IS_PLAYER_A_PARTICIPANT_ON_SCRIPT(p0: Player, p1: &mut Any, p2: Any) -> bool { invoke!(0x1AD5B71586B94820, p0, p1, p2) }
	pub fn NETWORK_PREVENT_SCRIPT_HOST_MIGRATION() { invoke_ignore!(0x2302C0264EA58D31) }
	pub fn NETWORK_IS_FEATURE_SUPPORTED(featureId: i32) -> bool { invoke!(0x9C725D149622BFDE, featureId) }
	pub fn PARTICIPANT_ID() -> Player { invoke!(0x90986E8876CE0A83) }
	pub fn PARTICIPANT_ID_TO_INT() -> i32 { invoke!(0x57A3BDDAD8E5AA0A) }
	pub fn NETWORK_GET_DESTROYER_OF_NETWORK_ID(netId: i32, weaponHash: &mut Hash) -> i32 { invoke!(0x7A1ADEEF01740A24, netId, weaponHash) }
	pub fn _0x6CF82A7F65A5AD5F(ped: Ped, p1: &mut Any) -> Player { invoke!(0x6CF82A7F65A5AD5F, ped, p1) }
	pub fn _0x236321F1178A5446(player: Player, ped: Ped, p2: &mut Any) -> bool { invoke!(0x236321F1178A5446, player, ped, p2) }
	pub fn NETWORK_GET_ASSISTED_DAMAGE_OF_ENTITY(player: Player, entity: Entity, p2: &mut i32) -> bool { invoke!(0x4CACA84440FA26F6, player, entity, p2) }
	pub fn NETWORK_GET_ENTITY_KILLER_OF_PLAYER(player: Player, weaponHash: &mut Hash) -> Entity { invoke!(0x42B2DAA6B596F5F8, player, weaponHash) }
	pub fn NETWORK_RESURRECT_LOCAL_PLAYER(x: f32, y: f32, z: f32, heading: f32, p4: i32, p5: bool, p6: Any, p7: bool) { invoke_ignore!(0xEA23C49EAA83ACFB, x, y, z, heading, p4, p5, p6, p7) }
	pub fn _NETWORK_RESURRECT_LOCAL_PLAYER_2(args: &mut Any) { invoke_ignore!(0x4154B7D8C75E5DCF, args) }
	pub fn NETWORK_SET_LOCAL_PLAYER_INVINCIBLE_TIME(time: i32) { invoke_ignore!(0x2D95C7E2D7E07307, time) }
	pub fn NETWORK_SET_LOCAL_PLAYER_SYNC_LOOK_AT(toggle: bool) { invoke_ignore!(0x524FF0AEFF9C3973, toggle) }
	pub fn NETWORK_HAS_ENTITY_BEEN_REGISTERED_WITH_THIS_THREAD(entity: Entity) -> bool { invoke!(0xB07D3185E11657A5, entity) }
	pub fn NETWORK_GET_NETWORK_ID_FROM_ENTITY(entity: Entity) -> i32 { invoke!(0xA11700682F3AD45C, entity) }
	pub fn NETWORK_GET_ENTITY_FROM_NETWORK_ID(netId: i32) -> Entity { invoke!(0xCE4E5D9B0A4FF560, netId) }
	pub fn NETWORK_GET_ENTITY_IS_NETWORKED(entity: Entity) -> bool { invoke!(0xC7827959479DCC78, entity) }
	pub fn NETWORK_REGISTER_ENTITY_AS_NETWORKED(entity: Entity) { invoke_ignore!(0x06FAACD625D80CAA, entity) }
	pub fn NETWORK_DOES_NETWORK_ID_EXIST(netID: i32) -> bool { invoke!(0x38CE16C96BD11344, netID) }
	pub fn _0x950ACD8F05B7B9DF(p0: Any) -> Any { invoke!(0x950ACD8F05B7B9DF, p0) }
	pub fn NETWORK_REQUEST_CONTROL_OF_NETWORK_ID(netId: i32) -> bool { invoke!(0xA670B3662FAFFBD0, netId) }
	pub fn NETWORK_HAS_CONTROL_OF_NETWORK_ID(netId: i32) -> bool { invoke!(0x4D36070FE0215186, netId) }
	pub fn NETWORK_REQUEST_CONTROL_OF_ENTITY(entity: Entity) -> bool { invoke!(0xB69317BF5E782347, entity) }
	pub fn _NETWORK_REQUEST_CONTROL_OF_ANIM_SCENE(animScene: AnimScene) -> bool { invoke!(0xAAA92B631B13F614, animScene) }
	pub fn NETWORK_REQUEST_CONTROL_OF_PICKUP_PLACEMENT(p0: Any) -> bool { invoke!(0x56ED2C48558DAB78, p0) }
	pub fn NETWORK_HAS_CONTROL_OF_ENTITY(entity: Entity) -> bool { invoke!(0x01BF60A500E28887, entity) }
	pub fn NETWORK_HAS_CONTROL_OF_PICKUP(pickup: Pickup) -> bool { invoke!(0x5BC9495F0B3B6FA6, pickup) }
	pub fn _NETWORK_HAS_CONTROL_OF_ANIM_SCENE(animScene: AnimScene) -> bool { invoke!(0x26A5C12FACFF8724, animScene) }
	pub fn NETWORK_HAS_CONTROL_OF_PICKUP_PLACEMENT(p0: Any) -> bool { invoke!(0x51EABCF2786515AB, p0) }
	pub fn _0xF260AF6F43953316(handle: ScrHandle) -> i32 { invoke!(0xF260AF6F43953316, handle) }
	pub fn VEH_TO_NET(vehicle: Vehicle) -> i32 { invoke!(0xB4C94523F023419C, vehicle) }
	pub fn PED_TO_NET(ped: Ped) -> i32 { invoke!(0x0EDEC3C276198689, ped) }
	pub fn OBJ_TO_NET(object: Object) -> i32 { invoke!(0x99BFDC94A603E541, object) }
	pub fn _ANIM_SCENE_TO_NET(animScene: AnimScene) -> i32 { invoke!(0xE0D73CDDEA79DDCD, animScene) }
	pub fn NET_TO_VEH(netHandle: i32) -> Vehicle { invoke!(0x367B936610BA360C, netHandle) }
	pub fn NET_TO_PED(netHandle: i32) -> Ped { invoke!(0xBDCD95FC216A8B3E, netHandle) }
	pub fn NET_TO_OBJ(netHandle: i32) -> Object { invoke!(0xD8515F5FEA14CB3F, netHandle) }
	pub fn NET_TO_ENT(netHandle: i32) -> Entity { invoke!(0xBFFEAB45A9A9094A, netHandle) }
	pub fn _NET_TO_ANIM_SCENE(netId: i32) -> AnimScene { invoke!(0xD7F6781A0ABAF6FB, netId) }
	pub fn _PROPSET_TO_NET(propSet: PropSet) -> i32 { invoke!(0x74F99EF7EF503398, propSet) }
	pub fn _NET_TO_PROPSET(netId: i32) -> PropSet { invoke!(0xD08066E00D26C448, netId) }
	pub fn _0x0CC28C08613BA9E5(p0: i32) { invoke_ignore!(0x0CC28C08613BA9E5, p0) }
	pub fn NETWORK_GET_LOCAL_HANDLE(gamerHandle: &mut Any) { invoke_ignore!(0xE86051786B66CD8E, gamerHandle) }
	pub fn NETWORK_HANDLE_FROM_PLAYER(player: Player, gamerHandle: &mut Any) { invoke_ignore!(0x388EB2B86C73B6B3, player, gamerHandle) }
	pub fn NETWORK_HASH_FROM_PLAYER_HANDLE(player: Player) -> Hash { invoke!(0xBC1D768F2F5D6C05, player) }
	pub fn NETWORK_HANDLE_FROM_FRIEND(friendIndex: i32, gamerHandle: &mut Any) { invoke_ignore!(0xD45CB817D7E177D2, friendIndex, gamerHandle) }
	pub fn NETWORK_GET_GAMERTAG_FROM_HANDLE(gamerHandle: &mut Any) -> *const char { invoke!(0x426141162EBE5CDB, gamerHandle) }
	pub fn NETWORK_DISPLAYNAMES_FROM_HANDLES_START(p0: &mut Any, p1: Any) -> i32 { invoke!(0xD66C9E72B3CC4982, p0, p1) }
	pub fn NETWORK_GET_DISPLAYNAMES_FROM_HANDLES(p0: Any, p1: Any, p2: Any) -> i32 { invoke!(0x58CC181719256197, p0, p1, p2) }
	pub fn _NETWORK_GET_DISPLAY_NAME_FROM_HANDLE(gamerHandle: &mut Any, displayName: &mut CStr) -> bool { invoke!(0x7FEE4F07C54B6B3C, gamerHandle, displayName) }
	pub fn NETWORK_ARE_HANDLES_THE_SAME(gamerHandle1: &mut Any, gamerHandle2: &mut Any) -> bool { invoke!(0x57DBA049E110F217, gamerHandle1, gamerHandle2) }
	pub fn NETWORK_IS_HANDLE_VALID(gamerHandle: &mut Any) -> bool { invoke!(0x6F79B93B0A8E4133, gamerHandle) }
	pub fn NETWORK_GET_PLAYER_FROM_GAMER_HANDLE(gamerHandle: &mut Any) -> Player { invoke!(0xCE5F689CF5A0A49D, gamerHandle) }
	pub fn NETWORK_IS_GAMER_IN_MY_SESSION(gamerHandle: &mut Any) -> bool { invoke!(0x0F10B05DDF8D16E9, gamerHandle) }
	pub fn NETWORK_SHOW_PROFILE_UI(gamerHandle: &mut Any) { invoke_ignore!(0x859ED1CEA343FCA8, gamerHandle) }
	pub fn _0x5759160AC17C13CE(gamerHandle: &mut Any, message: & CStr) { invoke_ignore!(0x5759160AC17C13CE, gamerHandle, message) }
	pub fn _0xF302AB9D978352EE(entity: Entity) -> i32 { invoke!(0xF302AB9D978352EE, entity) }
	pub fn _0x4538EE7C321590BC(networkId: i32) -> Entity { invoke!(0x4538EE7C321590BC, networkId) }
	pub fn _0xA47D48D06AA5A188() -> bool { invoke!(0xA47D48D06AA5A188) }
	pub fn NETWORK_GET_TOTAL_NUM_FRIENDS() -> i32 { invoke!(0xDB7ABDD203FA3704) }
	pub fn _0xA94ECE191D90637A() -> i32 { invoke!(0xA94ECE191D90637A) }
	pub fn _0x5CB8B0C846D0F30B(p0: Any) { invoke_ignore!(0x5CB8B0C846D0F30B, p0) }
	pub fn _0xFF36F36B07E69059(p0: Any) { invoke_ignore!(0xFF36F36B07E69059, p0) }
	pub fn _NETWORK_GET_CURRENT_FRIEND_PAGE_DATA(p0: &mut Any) -> bool { invoke!(0xA3EEC0A5AFF3FC5B, p0) }
	pub fn _0xB389289F031F059A() -> i32 { invoke!(0xB389289F031F059A) }
	pub fn NETWORK_CAN_REFRESH_FRIEND_PAGE() -> bool { invoke!(0x1AF5E28E64A76A9F) }
	pub fn NETWORK_REFRESH_CURRENT_FRIEND_PAGE() -> bool { invoke!(0x1F51F367B710A832) }
	pub fn _0xDA1BFED8582F61F0() -> bool { invoke!(0xDA1BFED8582F61F0) }
	pub fn _0x232E1EB23CDB313C() -> bool { invoke!(0x232E1EB23CDB313C) }
	pub fn _0x3E4A16BC669E71B3() -> bool { invoke!(0x3E4A16BC669E71B3) }
	pub fn _NETWORK_IS_FRIEND_HANDLE_ONLINE(gamerHandle: &mut Any) -> bool { invoke!(0xE348D1404BD80146, gamerHandle) }
	pub fn _NETWORK_IS_FRIEND_HANDLE_IN_SAME_TITLE(gamerHandle: &mut Any) -> bool { invoke!(0x665161D250850A9F, gamerHandle) }
	pub fn _NETWORK_GET_GAMERTAG_FROM_FRIEND(gamerHandle: &mut Any) -> *const char { invoke!(0x5659D87BE674AB17, gamerHandle) }
	pub fn NETWORK_IS_FRIEND(gamerHandle: &mut Any) -> bool { invoke!(0x1A24A179F9B31654, gamerHandle) }
	pub fn NETWORK_IS_PENDING_FRIEND(gamerHandle: &mut Any) -> bool { invoke!(0x0BE73DA6984A6E33, gamerHandle) }
	pub fn NETWORK_ADD_FRIEND(gamerHandle: &mut Any, message: & CStr) -> bool { invoke!(0x8E02D73914064223, gamerHandle, message) }
	pub fn _NETWORK_REMOVE_FRIEND(gamerHandle: &mut Any) -> bool { invoke!(0x55F618F68AB854D3, gamerHandle) }
	pub fn _NETWORK_CAN_ADD_FRIEND(gamerHandle: &mut Any) -> bool { invoke!(0x99ABE9BF9DADA162, gamerHandle) }
	pub fn NETWORK_SET_PLAYER_IS_PASSIVE(toggle: bool) { invoke_ignore!(0x9C25E8EC4C535FBD, toggle) }
	pub fn NETWORK_SET_FRIENDLY_FIRE_OPTION(toggle: bool) { invoke_ignore!(0xF808475FA571D823, toggle) }
	pub fn NETWORK_SET_RICH_PRESENCE(p0: i32, p1: &mut Any, p2: i32, p3: i32) { invoke_ignore!(0x1DCCACDCFC569362, p0, p1, p2, p3) }
	pub fn NETWORK_GET_TIMEOUT_TIME() -> i32 { invoke!(0x5ED0356A0CE3A34F) }
	pub fn _0xBC7D36946D19E60E(p0: bool) { invoke_ignore!(0xBC7D36946D19E60E, p0) }
	pub fn _0x880A7202301E282B(p0: &mut Any, p1: &mut Any, x: f32, y: f32, z: f32, p5: f32, p6: Any) -> bool { invoke!(0x880A7202301E282B, p0, p1, x, y, z, p5, p6) }
	pub fn _0xC964FCD3D1720697() -> Any { invoke!(0xC964FCD3D1720697) }
	pub fn _0xEC089F84A9C16C62() -> Any { invoke!(0xEC089F84A9C16C62) }
	pub fn PREVENT_NETWORK_ID_MIGRATION(netId: i32) { invoke_ignore!(0x7182EDDA1EE7DB5A, netId) }
	pub fn KEEP_NETWORK_ID_IN_FAST_INSTANCE(netId: i32, p1: bool, p2: i32) { invoke_ignore!(0xE1BC73D6815BA361, netId, p1, p2) }
	pub fn _0x02C4C6C2900D84DF(player: Player, p1: Any) { invoke_ignore!(0x02C4C6C2900D84DF, player, p1) }
	pub fn _0xD78A26024BB13E08(player: Player) { invoke_ignore!(0xD78A26024BB13E08, player) }
	pub fn SET_NETWORK_ID_EXISTS_ON_ALL_MACHINES(netId: i32, toggle: bool) { invoke_ignore!(0xE05E81A888FA63C8, netId, toggle) }
	pub fn SET_NETWORK_ID_ALWAYS_EXISTS_FOR_PLAYER(netId: i32, player: Player, toggle: bool) { invoke_ignore!(0xA8A024587329F36A, netId, player, toggle) }
	pub fn SET_NETWORK_ID_STOP_CLONING(networkId: i32, bStopCloning: bool) { invoke_ignore!(0x9ED3108D6847760A, networkId, bStopCloning) }
	pub fn NETWORK_SET_ENTITY_REMAINS_WHEN_UNNETWORKED(entity: Entity, toggle: bool) { invoke_ignore!(0xD785864798258032, entity, toggle) }
	pub fn NETWORK_SET_ENTITY_ONLY_EXISTS_FOR_PARTICIPANTS(entity: Entity, toggle: bool) { invoke_ignore!(0xF1CA12B18AEF5298, entity, toggle) }
	pub fn _0xE31A04513237DC89(entity: Entity) { invoke_ignore!(0xE31A04513237DC89, entity) }
	pub fn SET_NETWORK_ID_VISIBLE_IN_CUTSCENE(p0: Any, p1: Any, p2: Any, p3: Any) { invoke_ignore!(0xA6928482543022B4, p0, p1, p2, p3) }
	pub fn IS_NETWORK_ID_OWNED_BY_PARTICIPANT(netId: i32) -> bool { invoke!(0xA1607996431332DF, netId) }
	pub fn _NETWORK_GET_PLAYER_OWNER_OF_NETWORK_ID(netId: i32) -> Player { invoke!(0xA6C0787443C9583E, netId) }
	pub fn SET_LOCAL_PLAYER_VISIBLE_IN_CUTSCENE(local: bool, remote: bool, instanceId: i32) { invoke_ignore!(0xD1065D68947E7B6E, local, remote, instanceId) }
	pub fn PREVENT_MIGRATION_OF_ENTITIES_IN_FAST_INSTANCE_FOR_LOCAL_PLAYER(toggle: bool) { invoke_ignore!(0x89D803CD48622150, toggle) }
	pub fn SET_LOCAL_PLAYER_INVISIBLE_LOCALLY(p0: bool) { invoke_ignore!(0xE5F773C1A1D9D168, p0) }
	pub fn SET_PLAYER_INVISIBLE_LOCALLY(player: Player, toggle: bool) { invoke_ignore!(0x12B37D54667DB0B8, player, toggle) }
	pub fn SET_PLAYER_VISIBLE_LOCALLY(player: Player, toggle: bool) { invoke_ignore!(0xFAA10F1FAFB11AF2, player, toggle) }
	pub fn SET_ENTITY_VISIBLE_IN_CUTSCENE(entity: Entity, p1: bool, p2: bool, p3: i32) { invoke_ignore!(0xE0031D3C8F36AB82, entity, p1, p2, p3) }
	pub fn _SET_DOOR_NETWORKED(doorHash: Hash) { invoke_ignore!(0x51D99497ABF3F451, doorHash) }
	pub fn _SET_DOOR_UNNETWORKED(p0: Any, toggle: bool) { invoke_ignore!(0xC1E1A3D5ED7617B8, p0, toggle) }
	pub fn IS_DAMAGE_TRACKER_ACTIVE_ON_NETWORK_ID(netID: i32) -> bool { invoke!(0x6E192E33AD436366, netID) }
	pub fn ACTIVATE_DAMAGE_TRACKER_ON_NETWORK_ID(netID: i32, toggle: bool) { invoke_ignore!(0xD45B1FFCCD52FF19, netID, toggle) }
	pub fn IS_SPHERE_VISIBLE_TO_ANOTHER_MACHINE(p0: Any, p1: Any, p2: Any, p3: Any, p4: Any) -> bool { invoke!(0xD82CF8E64C8729D8, p0, p1, p2, p3, p4) }
	pub fn IS_SPHERE_VISIBLE_TO_PLAYER(p0: Any, p1: Any, p2: Any, p3: Any, p4: Any, p5: Any) -> bool { invoke!(0xDC3A310219E5DA62, p0, p1, p2, p3, p4, p5) }
	pub fn _0xD42C543F73233041(p0: bool) { invoke_ignore!(0xD42C543F73233041, p0) }
	pub fn RESERVE_NETWORK_MISSION_OBJECTS(amount: i32) { invoke_ignore!(0x4E5C93BD0C32FBF8, amount) }
	pub fn RESERVE_NETWORK_CLIENT_MISSION_OBJECTS(amount: i32) { invoke_ignore!(0xE7DDA8BD3BCF751C, amount) }
	pub fn RESERVE_NETWORK_MISSION_PEDS(amount: i32) { invoke_ignore!(0xB60FEBA45333D36F, amount) }
	pub fn RESERVE_NETWORK_CLIENT_MISSION_PEDS(amount: i32) { invoke_ignore!(0x807E119F80231732, amount) }
	pub fn RESERVE_NETWORK_MISSION_VEHICLES(amount: i32) { invoke_ignore!(0x76B02E21ED27A469, amount) }
	pub fn RESERVE_NETWORK_MISSION_PICKUPS(amount: i32) { invoke_ignore!(0x4D40E7D749BC6E6D, amount) }
	pub fn CAN_REGISTER_MISSION_OBJECTS(amount: i32) -> bool { invoke!(0x800DD4721A8B008B, amount) }
	pub fn CAN_REGISTER_MISSION_PEDS(amount: i32) -> bool { invoke!(0xBCBF4FEF9FA5D781, amount) }
	pub fn CAN_REGISTER_MISSION_VEHICLES(amount: i32) -> bool { invoke!(0x7277F1F2E085EE74, amount) }
	pub fn CAN_REGISTER_MISSION_PICKUPS(amount: i32) -> bool { invoke!(0xF0460C7BF80011EA, amount) }
	pub fn CAN_REGISTER_MISSION_ENTITIES(ped_amt: i32, vehicle_amt: i32, object_amt: i32, pickup_amt: i32) -> bool { invoke!(0x69778E7564BADE6D, ped_amt, vehicle_amt, object_amt, pickup_amt) }
	pub fn GET_NUM_RESERVED_MISSION_OBJECTS(p0: bool) -> i32 { invoke!(0xAA81B5F10BC43AC2, p0) }
	pub fn GET_NUM_RESERVED_MISSION_PEDS(p0: bool) -> i32 { invoke!(0x1F13D5AE5CB17E17, p0) }
	pub fn GET_NUM_RESERVED_MISSION_VEHICLES(p0: bool) -> i32 { invoke!(0xCF3A965906452031, p0) }
	pub fn _GET_NUM_RESERVED_MISSION_PICKUPS(p0: bool) -> i32 { invoke!(0x62BE3ECC79FBD004, p0) }
	pub fn GET_NUM_CREATED_MISSION_OBJECTS(p0: bool) -> i32 { invoke!(0x12B6281B6C6706C0, p0) }
	pub fn GET_NUM_CREATED_MISSION_PEDS(p0: bool) -> i32 { invoke!(0xCB215C4B56A7FAE7, p0) }
	pub fn GET_NUM_CREATED_MISSION_VEHICLES(p0: bool) -> i32 { invoke!(0x0CD9AB83489430EA, p0) }
	pub fn _GET_NUM_CREATED_MISSION_PICKUPS(p0: bool) -> i32 { invoke!(0xD2BA051B94CA9BCC, p0) }
	pub fn _GET_RESERVED_MISSION_ENTITIES_FOR_THREAD(threadId: i32, pedMax: &mut i32, vehicleMax: &mut i32, unkMax: &mut i32, pedMin: &mut i32, vehicleMin: &mut i32, unkMin: &mut i32) { invoke_ignore!(0x99AAC89C510DEB0D, threadId, pedMax, vehicleMax, unkMax, pedMin, vehicleMin, unkMin) }
	pub fn GET_RESERVED_MISSION_ENTITIES_IN_AREA(x: f32, y: f32, z: f32, p3: bool, peds: &mut i32, vehicles: &mut i32, objects: &mut i32, pickups: &mut i32) { invoke_ignore!(0x5E71E72A94985214, x, y, z, p3, peds, vehicles, objects, pickups) }
	pub fn _0x5F328FC909F0E0FF(p0: i32, p1: i32, p2: i32, p3: i32) -> bool { invoke!(0x5F328FC909F0E0FF, p0, p1, p2, p3) }
	pub fn GET_MAX_NUM_NETWORK_OBJECTS() -> i32 { invoke!(0xC7BE335216B5EC7C) }
	pub fn GET_MAX_NUM_NETWORK_PEDS() -> i32 { invoke!(0x0C1F7D49C39D2289) }
	pub fn GET_MAX_NUM_NETWORK_VEHICLES() -> i32 { invoke!(0x0AFCE529F69B21FF) }
	pub fn GET_MAX_NUM_NETWORK_PICKUPS() -> i32 { invoke!(0xA72835064DD63E4C) }
	pub fn _0x75FC34A2BA345BD1(entity: Entity, player: Player, p2: &mut Any) -> bool { invoke!(0x75FC34A2BA345BD1, entity, player, p2) }
	pub fn _0x979765465A6F25FC(entity: Entity, p1: bool) { invoke_ignore!(0x979765465A6F25FC, entity, p1) }
	pub fn _0x5133CF81924F1129() -> i32 { invoke!(0x5133CF81924F1129) }
	pub fn _0x1E4E097D71D449FB(p0: bool) -> i32 { invoke!(0x1E4E097D71D449FB, p0) }
	pub fn _0x982D7AD755B8F62C(p0: bool) -> i32 { invoke!(0x982D7AD755B8F62C, p0) }
	pub fn _0x917AD74BDCF8B6E9(p0: bool) -> i32 { invoke!(0x917AD74BDCF8B6E9, p0) }
	pub fn _0xF8DC69DC1AD19072(p0: bool) -> i32 { invoke!(0xF8DC69DC1AD19072, p0) }
	pub fn _0x744BFBB0CA908161(p0: bool) -> i32 { invoke!(0x744BFBB0CA908161, p0) }
	pub fn _0x106CBDD5077DEDE1(p0: bool) -> i32 { invoke!(0x106CBDD5077DEDE1, p0) }
	pub fn _0xBAF7E2979442B29F(p0: bool) -> i32 { invoke!(0xBAF7E2979442B29F, p0) }
	pub fn _0x039B692B3318FAB6(p0: bool) -> i32 { invoke!(0x039B692B3318FAB6, p0) }
	pub fn _0x4835413EA6F9C9CD(p0: bool) -> i32 { invoke!(0x4835413EA6F9C9CD, p0) }
	pub fn GET_NETWORK_TIME() -> i32 { invoke!(0x7A5487FE9FAA6B48) }
	pub fn GET_NETWORK_TIME_ACCURATE() -> i32 { invoke!(0x89023FBBF9200E9F) }
	pub fn HAS_NETWORK_TIME_STARTED() -> bool { invoke!(0x46718ACEEDEAFC84) }
	pub fn GET_TIME_OFFSET(timeA: i32, timeB: i32) -> i32 { invoke!(0x017008CCDAD48503, timeA, timeB) }
	pub fn IS_TIME_LESS_THAN(timeA: i32, timeB: i32) -> bool { invoke!(0xCB2CF5148012C8D0, timeA, timeB) }
	pub fn IS_TIME_MORE_THAN(timeA: i32, timeB: i32) -> bool { invoke!(0xDE350F8651E4346C, timeA, timeB) }
	pub fn GET_TIME_DIFFERENCE(timeA: i32, timeB: i32) -> i32 { invoke!(0xA2C6FC031D46FFF0, timeA, timeB) }
	pub fn GET_CLOUD_TIME_AS_INT() -> i32 { invoke!(0x9A73240B49945C76) }
	pub fn CONVERT_POSIX_TIME(posixTime: i32, timeStructure: &mut Any) { invoke_ignore!(0xAC97AF97FA68E5D5, posixTime, timeStructure) }
	pub fn NETWORK_SET_IN_SPECTATOR_MODE(toggle: bool, playerPed: Ped) { invoke_ignore!(0x423DE3854BB50894, toggle, playerPed) }
	pub fn _NETWORK_SET_IN_STATIC_SPECTATOR_MODE(toggle: bool, x: f32, y: f32, z: f32) { invoke_ignore!(0xFBF1ECFB39A77B5F, toggle, x, y, z) }
	pub fn NETWORK_IS_IN_SPECTATOR_MODE() -> bool { invoke!(0x048746E388762E11) }
	pub fn _NETWORK_IS_PLAYER_IN_SPECTATOR_MODE(player: Player) -> bool { invoke!(0x5B709519997ECF0F, player) }
	pub fn NETWORK_SET_IN_MP_CUTSCENE(p0: bool, p1: bool, p2: i32, p3: bool) { invoke_ignore!(0x9CA5DE655269FEC4, p0, p1, p2, p3) }
	pub fn NETWORK_IS_IN_MP_CUTSCENE() -> bool { invoke!(0x6CC27C9FA2040220) }
	pub fn NETWORK_IS_PLAYER_IN_MP_CUTSCENE(player: Player) -> bool { invoke!(0x63F9EE203C3619F2, player) }
	pub fn _0x34BC1E79546BA543(p0: bool) { invoke_ignore!(0x34BC1E79546BA543, p0) }
	pub fn _SET_NETWORK_RESPOT_TIMER(entity: Entity, timer: i32, p2: bool) { invoke_ignore!(0x442B4347B6EC36E8, entity, timer, p2) }
	pub fn _0x26A867C0B7A456D1(entity: Entity) -> bool { invoke!(0x26A867C0B7A456D1, entity) }
	pub fn SET_LOCAL_PLAYER_AS_GHOST(toggle: bool) { invoke_ignore!(0x5FFE9B4144F9712F, toggle) }
	pub fn IS_ENTITY_A_GHOST(entity: Entity) -> bool { invoke!(0x21D04D7BC538C146, entity) }
	pub fn _SET_PLAYER_VISIBILITY_TO_LOCAL_PLAYER_DISABLED(player: Player, disabled: bool) { invoke_ignore!(0xDCA6ABDB9288FBE4, player, disabled) }
	pub fn _SET_ENTITY_GHOSTED_TO_LOCAL_PLAYER(entity: Entity, toggle: bool) { invoke_ignore!(0xEE5AE9956743BA20, entity, toggle) }
	pub fn IS_OBJECT_REASSIGNMENT_IN_PROGRESS() -> bool { invoke!(0x8FE9EB11EC9CC23A) }
	pub fn _0x039AD6B57D5179FF() -> i32 { invoke!(0x039AD6B57D5179FF) }
	pub fn _0x02B3CDD652B3CDD6() -> i32 { invoke!(0x02B3CDD652B3CDD6) }
	pub fn _GET_NUM_PEER_NEGOTIATION_RESPONSES() -> i32 { invoke!(0x4FE932E84FE932E8) }
	pub fn _NETWORK_DEBUG_REQUEST_ENTITY_POSITION(p0: &mut Any) { invoke_ignore!(0xFA38B52F91B59075, p0) }
	pub fn NETWORK_GET_NETWORK_ID_FROM_ROPE_ID(ropeId: i32) -> i32 { invoke!(0x42871327315EDAE8, ropeId) }
	pub fn NETWORK_GET_ROPE_ID_FROM_NETWORK_ID(netId: i32) -> i32 { invoke!(0xEB1A4DD8352EC828, netId) }
	pub fn _NETWORK_SPAWN_CONFIG_ADD_SPAWN_POINT(x: f32, y: f32, z: f32, heading: f32) { invoke_ignore!(0xFD1AC0B3858F224C, x, y, z, heading) }
	pub fn _0xA63E4F050F20021F() { invoke_ignore!(0xA63E4F050F20021F) }
	pub fn _NETWORK_SPAWN_CONFIG_ADD_EXCLUSION_VOLUME(volume: Volume) { invoke_ignore!(0xEEB7818B1D307212, volume) }
	pub fn _NETWORK_SPAWN_CONFIG_REMOVE_EXCLUSION_VOLUME(volume: Volume) { invoke_ignore!(0xA35E7BF20FA269E0, volume) }
	pub fn _0x0BF90CBB6B72977B() { invoke_ignore!(0x0BF90CBB6B72977B) }
	pub fn _0x7B3FF2D193628126(player: Player) { invoke_ignore!(0x7B3FF2D193628126, player) }
	pub fn _0x19B52C20B5C4757C() { invoke_ignore!(0x19B52C20B5C4757C) }
	pub fn NETWORK_SPAWN_CONFIG_SET_FLAGS(flags: i32) { invoke_ignore!(0xF94A0D5B254375DF, flags) }
	pub fn _0x6CEE2E30021DAEC6() { invoke_ignore!(0x6CEE2E30021DAEC6) }
	pub fn _NETWORK_SPAWN_CONFIG_ADD_PROPERTY_SCRIPTED(configProperty: i32, include: bool) { invoke_ignore!(0x44D59EC597BBF348, configProperty, include) }
	pub fn _0xB131E686BD97B3F8() { invoke_ignore!(0xB131E686BD97B3F8) }
	pub fn _NETWORK_SPAWN_CONFIG_ADD_PROPERTY_PREFERENCE(configProperty: i32, include: bool, weight: f32) { invoke_ignore!(0xEB6027FD1B4600D5, configProperty, include, weight) }
	pub fn _0x405DDEFB1F531B18(volume: Volume, p1: bool, p2: Any, p3: Any) { invoke_ignore!(0x405DDEFB1F531B18, volume, p1, p2, p3) }
	pub fn _0x43CF999205084B4B() { invoke_ignore!(0x43CF999205084B4B) }
	pub fn _0x13F592FC3BF0EA84(volume: Volume, p1: bool, originalWeight: f32, p3: Any, p4: Any) { invoke_ignore!(0x13F592FC3BF0EA84, volume, p1, originalWeight, p3, p4) }
	pub fn _0xCF23AB5BD47B384D(p0: Any) { invoke_ignore!(0xCF23AB5BD47B384D, p0) }
	pub fn _0xE5634491A58C2703(p0: f32) { invoke_ignore!(0xE5634491A58C2703, p0) }
	pub fn NETWORK_SPAWN_CONFIG_SET_GROUND_TO_ROOT_OFFSET(offset: f32) { invoke_ignore!(0x59577799F6AE2F34, offset) }
	pub fn _NETWORK_SPAWN_CONFIG_SET_LEVEL_WATER_DEPTH(waterDepthLevel: i32) { invoke_ignore!(0xBDCC671B911040F9, waterDepthLevel) }
	pub fn NETWORK_SPAWN_CONFIG_SET_TUNING_FLOAT(p0: Hash, p1: f32) { invoke_ignore!(0x0608326F7B98C08D, p0, p1) }
	pub fn _0x5D3C528B7A7DF836(nsctf: Hash) { invoke_ignore!(0x5D3C528B7A7DF836, nsctf) }
	pub fn _0x2686BD9566B65EDA(x: f32, y: f32, z: f32) { invoke_ignore!(0x2686BD9566B65EDA, x, y, z) }
	pub fn _0xBB1EC8C2EEF33BAA(entity: Entity) { invoke_ignore!(0xBB1EC8C2EEF33BAA, entity) }
	pub fn _0x67CCDF74C4DF7169() -> bool { invoke!(0x67CCDF74C4DF7169) }
	pub fn _0xC8B6D18E22484643() { invoke_ignore!(0xC8B6D18E22484643) }
	pub fn _0x97BCE4C4B3191228() { invoke_ignore!(0x97BCE4C4B3191228) }
	pub fn _0x41452E8A3B9C0C4B() -> i32 { invoke!(0x41452E8A3B9C0C4B) }
	pub fn _NETWORK_SPAWN_CONFIG_SEARCH_IN_PROGRESS() -> bool { invoke!(0x89EC2FC89ECB1005) }
	pub fn _0x61BFBAA795E712AD() { invoke_ignore!(0x61BFBAA795E712AD) }
	pub fn _NETWORK_SPAWN_CONFIG_SET_CANCEL_SEARCH() { invoke_ignore!(0x765E60A1DCB8B1CE) }
	pub fn _0x691E4DE5309EAEFC(p0: Any, p1: &mut Any) { invoke_ignore!(0x691E4DE5309EAEFC, p0, p1) }
	pub fn NETWORK_START_SOLO_TUTORIAL_SESSION() { invoke_ignore!(0x17E0198B3882C2CB) }
	pub fn NETWORK_END_TUTORIAL_SESSION() { invoke_ignore!(0xD0AFAFF5A51D72F7) }
	pub fn NETWORK_IS_IN_TUTORIAL_SESSION() -> bool { invoke!(0xADA24309FE08DACF) }
	pub fn NETWORK_IS_TUTORIAL_SESSION_CHANGE_PENDING() -> bool { invoke!(0x35F0B98A8387274D) }
	pub fn NETWORK_ARE_PLAYERS_IN_SAME_TUTORIAL_SESSION(player: Player, index: i32) -> bool { invoke!(0x9DE986FC9A87C474, player, index) }
	pub fn NETWORK_ALLOW_ALL_ENTITY_FADING_FOR_INSTANCES(toggle: bool) { invoke_ignore!(0x4B05B97BA46F419D, toggle) }
	pub fn NETWORK_ALLOW_ENTITY_FADING_FOR_INSTANCES(entity: Entity, toggle: bool) { invoke_ignore!(0xF3354D6CA46F419D, entity, toggle) }
	pub fn _0xDC6AD5C046F33AB4(p0: bool, p1: bool) { invoke_ignore!(0xDC6AD5C046F33AB4, p0, p1) }
	pub fn _0x6C7E04E9DE451789() { invoke_ignore!(0x6C7E04E9DE451789) }
	pub fn NETWORK_SET_LOCAL_PLAYER_PENDING_FAST_INSTANCE_ID(instanceId: i32) { invoke_ignore!(0x007FF852DCF49DA4, instanceId) }
	pub fn _NETWORK_GET_PLAYER_FAST_INSTANCE_ID(player: Player) -> i32 { invoke!(0xD9267375834C5EAB, player) }
	pub fn NETWORK_CONCEAL_PLAYER(player: Player, toggle: bool) { invoke_ignore!(0xBBDF066252829606, player, toggle) }
	pub fn NETWORK_IS_PLAYER_CONCEALED(player: Player) -> bool { invoke!(0x919B3C98ED8292F9, player) }
	pub fn _0x40FEDB13870042F1() { invoke_ignore!(0x40FEDB13870042F1) }
	pub fn _0x422F9D6D6C7BC290(p0: i32) { invoke_ignore!(0x422F9D6D6C7BC290, p0) }
	pub fn NETWORK_REQUEST_CLOUD_TUNABLES() { invoke_ignore!(0x42FB3B532D526E6C) }
	pub fn NETWORK_IS_TUNABLE_CLOUD_REQUEST_PENDING() -> bool { invoke!(0x0467C11ED88B7D28) }
	pub fn NETWORK_GET_TUNABLE_CLOUD_CRC() -> i32 { invoke!(0x10BD227A753B0D84) }
	pub fn NETWORK_DOES_TUNABLE_EXIST(tunableContext: Hash, tunableName: Hash) -> bool { invoke!(0x85E5F8B9B898B20A, tunableContext, tunableName) }
	pub fn NETWORK_ACCESS_TUNABLE_INT(tunableContext: Hash, tunableName: Hash, value: &mut i32) -> bool { invoke!(0x8BE1146DFD5D4468, tunableContext, tunableName, value) }
	pub fn NETWORK_ACCESS_TUNABLE_BOOL(tunableContext: Hash, tunableName: Hash) -> bool { invoke!(0xAA6A47A573ABB75A, tunableContext, tunableName) }
	pub fn NETWORK_TRY_ACCESS_TUNABLE_INT_HASH(tunableContext: Hash, tunableName: Hash, defaultValue: i32) -> i32 { invoke!(0xA25E006B36719774, tunableContext, tunableName, defaultValue) }
	pub fn NETWORK_TRY_ACCESS_TUNABLE_FLOAT_HASH(tunableContext: Hash, tunableName: Hash, defaultValue: f32) -> f32 { invoke!(0xA18393089C05E49C, tunableContext, tunableName, defaultValue) }
	pub fn NETWORK_TRY_ACCESS_TUNABLE_BOOL_HASH(tunableContext: Hash, tunableName: Hash, defaultValue: bool) -> bool { invoke!(0xB2AD5D29A99D4B26, tunableContext, tunableName, defaultValue) }
	pub fn _0x894B5ECAB45D2342(netHandle: i32, p1: Any) { invoke_ignore!(0x894B5ECAB45D2342, netHandle, p1) }
	pub fn NETWORK_DISABLE_PROXIMITY_MIGRATION(netID: i32) { invoke_ignore!(0x407091CF6037118E, netID) }
	pub fn _COMMERCE_STORE_IS_OPEN() -> bool { invoke!(0xCE5E79D9E303628E) }
	pub fn _COMMERCE_STORE_IS_ENABLED() -> bool { invoke!(0xDBC754CB6CCB9378) }
	pub fn CLOUD_HAS_REQUEST_COMPLETED(id: i32) -> bool { invoke!(0x4C61B39930D045DA, id) }
	pub fn CLOUD_DID_REQUEST_SUCCEED(id: i32) -> bool { invoke!(0x3A3D5568AF297CD5, id) }
	pub fn _GET_LAUNCH_PARAM_EXISTS(paramName: & CStr) -> bool { invoke!(0x02E97CE283648CD9, paramName) }
	pub fn GET_LAUNCH_PARAM_VALUE(paramName: & CStr) -> *const char { invoke!(0x65E65CA6A0FE59D4, paramName) }
	pub fn _SET_LAUNCH_PARAM_VALUE(paramName: & CStr, value: & CStr) { invoke_ignore!(0x668AF6E4933AC13F, paramName, value) }
	pub fn _CLEAR_LAUNCH_PARAM(paramName: & CStr) { invoke_ignore!(0x782C94DB6469634D, paramName) }
	pub fn _GET_LAUNCH_PARAM_STRING() -> *const char { invoke!(0xC59AB6A04333C502) }
	pub fn _SET_LAUNCH_PARAM_STRING(params: & CStr) { invoke_ignore!(0xDFFC15AA63D04AAB, params) }
	pub fn CLEAR_SERVICE_EVENT_ARGUMENTS() { invoke_ignore!(0x966DD84FB6A46017) }
	pub fn UGC_IS_REQUEST_PENDING(ugcRequestId: i32) -> bool { invoke!(0xF4AC4FA844FD559A, ugcRequestId) }
	pub fn UGC_HAS_REQUEST_FINISHED(ugcRequestId: i32) -> bool { invoke!(0xA9EB4D606076615D, ugcRequestId) }
	pub fn UGC_DID_REQUEST_SUCCEED(ugcRequestId: i32) -> bool { invoke!(0x0B6009A90B8495F1, ugcRequestId) }
	pub fn _0xCD53E6CBF609C012(ugcRequestId: i32) -> bool { invoke!(0xCD53E6CBF609C012, ugcRequestId) }
	pub fn _UGC_QUERY_BY_CONTENT_TYPE(p0: i32, maxGet: i32, contentTypeName: & CStr, p3: i32, p4: i32, p5: i32) -> i32 { invoke!(0xF40EF49B3099E98E, p0, maxGet, contentTypeName, p3, p4, p5) }
	pub fn _UGC_QUERY_BY_CATEGORY(categoryType: i32, p1: i32, maxGet: i32, contentTypeName: & CStr, p4: i32, p5: bool) -> i32 { invoke!(0x8C109958C9BB559D, categoryType, p1, maxGet, contentTypeName, p4, p5) }
	pub fn _UGC_QUERY_BY_CONTENT_ID(contentId: & CStr, latestVersion: bool, contentTypeName: & CStr) -> i32 { invoke!(0x69D22E183580113F, contentId, latestVersion, contentTypeName) }
	pub fn _UGC_IS_BOOK_MARKED(contentId: & CStr) -> bool { invoke!(0xE42D1042F09865FE, contentId) }
	pub fn UGC_CLEAR_QUERY_RESULTS(ugcRequestId: i32) { invoke_ignore!(0xE931354FEA710038, ugcRequestId) }
	pub fn UGC_QUERY_WAS_FORCE_CANCELLED(ugcRequestId: i32) -> bool { invoke!(0xF8F0705E77A0E705, ugcRequestId) }
	pub fn UGC_QUERY_GET_CONTENT_NUM(ugcRequestId: i32) -> i32 { invoke!(0x76160E0396142765, ugcRequestId) }
	pub fn _UGC_QUERY_GET_CREATOR_HANDLE(p0: Any, index: i32, gamerHandle: &mut Any) -> Any { invoke!(0xADB56322EEDFBDC9, p0, index, gamerHandle) }
	pub fn _UGC_QUERY_GET_OWNER_ID(p0: Any, index: i32) -> *const char { invoke!(0xF9F0B3028431967B, p0, index) }
	pub fn _UGC_QUERY_GET_NAME(p0: Any, index: i32) -> *const char { invoke!(0x2D053EA815702DD1, p0, index) }
	pub fn _UGC_QUERY_GET_ROOT_CONTENT_ID(p0: Any, index: i32) -> *const char { invoke!(0x566CEB0542EF5ECF, p0, index) }
	pub fn _UGC_QUERY_GET_PLAYLIST_NAME(p0: Any, index: i32) -> *const char { invoke!(0xCAF50048C8D0FBA0, p0, index) }
	pub fn _UGC_QUERY_GET_MISSION_DESC_HASH(p0: Any, index: i32) -> Hash { invoke!(0xA6BF569956C60A60, p0, index) }
	pub fn _UGC_QUERY_GET_CREATOR_PHOTO(p0: Any, p1: i32, p2: Any) -> *const char { invoke!(0x409FE0CA6A4D1D49, p0, p1, p2) }
	pub fn _UGC_QUERY_GET_DATE(p0: Any, index: i32, p2: &mut Any) { invoke_ignore!(0xE0CB4AB15CB32710, p0, index, p2) }
	pub fn _UGC_QUERY_GET_POSIX_UPDATED_DATE(p0: Any, p1: Any) -> i32 { invoke!(0x21A99A72B00D8002, p0, p1) }
	pub fn _UGC_QUERY_GET_POSIX_PUBLISHED_DATE(p0: Any, p1: Any) -> i32 { invoke!(0x104080CA9E519B00, p0, p1) }
	pub fn _UGC_QUERY_GET_VERSION(p0: Any, index: i32, p2: i32) -> i32 { invoke!(0x63E9DCBC8B0931ED, p0, index, p2) }
	pub fn _UGC_QUERY_GET_LANGUAGE(p0: Any, index: i32) -> i32 { invoke!(0x97764E8AC6487A9A, p0, index) }
	pub fn _UGC_QUERY_GET_PUBLISHED(p0: Any, p1: Any) -> bool { invoke!(0x9993F1E11944A3DD, p0, p1) }
	pub fn _UGC_QUERY_GET_RATING(p0: Any, index: i32, p2: i32) -> f32 { invoke!(0x24CD8FAEA1368379, p0, index, p2) }
	pub fn _0x5F0E99071582DECA(p0: Any, index: i32, p2: i32) -> Any { invoke!(0x5F0E99071582DECA, p0, index, p2) }
	pub fn UGC_QUERY_GET_CONTENT_HAS_PLAYER_RECORD(p0: Any, index: i32) -> bool { invoke!(0xF794765390A6DCA5, p0, index) }
	pub fn _UGC_QUERY_GET_BOOK_MARKED(p0: Any, index: i32) -> bool { invoke!(0x98539FC453AEA639, p0, index) }
	pub fn _UGC_HAS_PRIVILEGE() -> bool { invoke!(0x6506BFA755FB209C) }
	pub fn UGC_REQUEST_CONTENT_DATA_FROM_PARAMS(contentTypeName: & CStr, contentId: & CStr, fileId: i32, fileVersion: i32, languageId: i32) -> i32 { invoke!(0x7FD2990AF016795E, contentTypeName, contentId, fileId, fileVersion, languageId) }
	pub fn UGC_REQUEST_CACHED_DESCRIPTION(description: Hash) -> i32 { invoke!(0x5E0165278F6339EE, description) }
	pub fn UGC_IS_DESCRIPTION_REQUEST_IN_PROGRESS(description: Hash) -> bool { invoke!(0x2D5DC831176D0114, description) }
	pub fn UGC_HAS_DESCRIPTION_REQUEST_FINISHED(description: Hash) -> bool { invoke!(0xEBFA8D50ADDC54C4, description) }
	pub fn UGC_DID_DESCRIPTION_REQUEST_SUCCEED(description: Hash) -> bool { invoke!(0x162C23CA83ED0A62, description) }
	pub fn UGC_GET_CACHED_DESCRIPTION(description: Hash, length: i32) -> *const char { invoke!(0x40F7E66472DF3E5C, description, length) }
	pub fn UGC_RELEASE_CACHED_DESCRIPTION(description: Hash) -> bool { invoke!(0x5A34CD9C3C5BEC44, description) }
	pub fn UGC_RELEASE_ALL_CACHED_DESCRIPTIONS() { invoke_ignore!(0x68103E2247887242) }
	pub fn UGC_SET_QUERY_DATA_FROM_OFFLINE(p0: bool) { invoke_ignore!(0xF98DDE0A8ED09323, p0) }
	pub fn UGC_IS_LANGUAGE_SUPPORTED(languageId: i32) -> bool { invoke!(0xF53E48461B71EECB, languageId) }
	pub fn _0xD4022C7286B0DFA2(p0: & CStr, p1: i32, p2: i32) -> Any { invoke!(0xD4022C7286B0DFA2, p0, p1, p2) }
	pub fn _NETWORK_PERSONA_PHOTO_WRITE_SC_PROFILE(texture: & CStr, personaPhotoType: i32, formatIndex: i32) -> bool { invoke!(0xB72999D3120599DF, texture, personaPhotoType, formatIndex) }
	pub fn _NETWORK_PERSONA_PHOTO_WRITE_LOCAL(texture: & CStr, playerSlot: i32, p2: i32, personaPhotoLocalCacheType: i32) -> bool { invoke!(0x2A48D9567940598F, texture, playerSlot, p2, personaPhotoLocalCacheType) }
	pub fn _NETWORK_IS_PREVIOUS_UPLOAD_PENDING() -> bool { invoke!(0xA21E3BAD0A42D199) }
	pub fn _0xCC4E72C339461ED1() -> Any { invoke!(0xCC4E72C339461ED1) }
	pub fn _REQUEST_PEDSHOT_TEXTURE_LOCAL_BACKUP_DOWNLOAD(player: i32, personaPhotoLocalCacheType: i32) -> *const char { invoke!(0x356F9FB0698C1FEB, player, personaPhotoLocalCacheType) }
	pub fn _REQUEST_PEDSHOT_TEXTURE_LOCAL_DOWNLOAD(gamerHandle: &mut Any, p1: i32) -> *const char { invoke!(0xCAF4CA2F87779F8F, gamerHandle, p1) }
	pub fn _REQUEST_PEDSHOT_TEXTURE_MULTIPLAYER_DOWNLOAD(gamerHandle: &mut Any, p1: i32) -> *const char { invoke!(0xB5C4B18B12A2AF23, gamerHandle, p1) }
	pub fn _TEXTURE_DOWNLOAD_TEXTURE_NAME_IS_VALID(name: & CStr) -> bool { invoke!(0xE2C3CEC3C0903A00, name) }
	pub fn TEXTURE_DOWNLOAD_REQUEST(gamerHandle: &mut Any, filePath: & CStr, name: & CStr, p3: bool) -> i32 { invoke!(0x16160DA74A8E74A2, gamerHandle, filePath, name, p3) }
	pub fn _MUGSHOT_TEXTURE_DOWNLOAD_REQUEST(gamerHandle: &mut Any, p1: i32, name: & CStr, p3: bool) -> i32 { invoke!(0x9B5DB6CEAFAA10BB, gamerHandle, p1, name, p3) }
	pub fn UGC_TEXTURE_DOWNLOAD_REQUEST(p0: &mut Any, p1: Any, p2: Any, p3: Any, p4: &mut Any, p5: bool) -> i32 { invoke!(0x308F96458B7087CC, p0, p1, p2, p3, p4, p5) }
	pub fn _LOCAL_PLAYER_PEDSHOT_TEXTURE_DOWNLOAD_REQUEST(playerSlot: i32, personaPhotoLocalCacheType: i32) -> i32 { invoke!(0x6E2FD8CF7EB10E53, playerSlot, personaPhotoLocalCacheType) }
	pub fn TEXTURE_DOWNLOAD_RELEASE(textureDownloadId: i32) { invoke_ignore!(0x487EB90B98E9FB19, textureDownloadId) }
	pub fn _TEXTURE_DOWNLOAD_RELEASE_BY_NAME(name: & CStr) { invoke_ignore!(0x7A17B7981560FFA5, name) }
	pub fn TEXTURE_DOWNLOAD_GET_NAME(textureDownloadId: i32) -> *const char { invoke!(0x3448505B6E35262D, textureDownloadId) }
	pub fn GET_STATUS_OF_TEXTURE_DOWNLOAD(textureDownloadId: i32) -> i32 { invoke!(0x8BD6C6DEA20E82C6, textureDownloadId) }
	pub fn _PEDMUGSHOT_GET_STATUS() -> i32 { invoke!(0xCBAC13F065C47596) }
	pub fn _PEDMUGSHOT_TAKE() -> bool { invoke!(0xCD954F330693F5F2) }
	pub fn _PEDMUGSHOT_REQUEST_SEND() -> Any { invoke!(0xFBC30B70B3CDB87E) }
	pub fn _0x814729078AED6D30() { invoke_ignore!(0x814729078AED6D30) }
	pub fn NETWORK_HAVE_ROS_BANNED_PRIV() -> bool { invoke!(0x8020A73847E0CA7D) }
	pub fn NETWORK_HAS_ROS_PRIVILEGE(index: i32) -> bool { invoke!(0xA699957E60D80214, index) }
	pub fn NETWORK_START_USER_CONTENT_PERMISSIONS_CHECK(gamerHandle: &mut Any) -> i32 { invoke!(0xDEB2B99A1AF1A2A6, gamerHandle) }
	pub fn _NETWORK_AUTO_SESSION_SET_ALLOWED_TO_SPLIT(toggle: bool) { invoke_ignore!(0x0A428058079EE65C, toggle) }
	pub fn NETWORK_AUTO_SESSION_IS_ALLOWED_TO_MERGE() -> bool { invoke!(0xAADED99A6B268A27) }
	pub fn _NETWORK_AUTO_SESSION_SET_ALLOWED_TO_MERGE(toggle: bool, p1: &mut Any, p2: i32) { invoke_ignore!(0x63246A24F5747510, toggle, p1, p2) }
	pub fn _NETWORK_AUTO_SESSION_IS_AUTO_WARP_DISABLED() -> bool { invoke!(0xE258570E0C116A66) }
	pub fn _NETWORK_AUTO_SESSION_SET_AUTO_WARP_ENABLED(toggle: bool) { invoke_ignore!(0x4440FEE3EFE78F54, toggle) }
	pub fn NETWORK_AUTO_SESSION_CAN_SPLIT_SESSION(p0: &mut i32) -> bool { invoke!(0xE404BFF0ABA23CDC, p0) }
	pub fn NETWORK_AUTO_SESSION_SPLIT_SESSION(playersToTake: i32, maxInstancePlayers: i32, sessionFlags: i32, bucketId: i32) -> bool { invoke!(0xC223D299C670413D, playersToTake, maxInstancePlayers, sessionFlags, bucketId) }
	pub fn _NETWORK_AUTO_SESSION_IS_PROCESSING_SESSION_SPLIT() -> bool { invoke!(0xA021095C983F20D8) }
	pub fn _NETWORK_AUTO_SESSION_SPLIT_SESSION_SUCCESSFUL() -> bool { invoke!(0x6D87BA8EF15226CD) }
	pub fn _NETWORK_AUTO_SESSION_IS_INSTANCED_SESSION() -> bool { invoke!(0x277865A734918AE6) }
	pub fn NETWORK_AUTO_SESSION_FINISH_INSTANCE() { invoke_ignore!(0xBB51299166B844F3) }
	pub fn _0xFD8112109A96877C() { invoke_ignore!(0xFD8112109A96877C) }
	pub fn _0x5A91BCEF74944E93(player: Player, p1: f32) { invoke_ignore!(0x5A91BCEF74944E93, player, p1) }
	pub fn _0xFB3205788F8AFA3F() -> i32 { invoke!(0xFB3205788F8AFA3F) }
	pub fn _0x335AF56613CA0F49(p0: i32) { invoke_ignore!(0x335AF56613CA0F49, p0) }
	pub fn _0x9E5A47744C0F0376(p0: i32) -> bool { invoke!(0x9E5A47744C0F0376, p0) }
	pub fn _0xD3B6EBC6C3D77D44(p0: i32) { invoke_ignore!(0xD3B6EBC6C3D77D44, p0) }
	pub fn _0xA7670F7991099680(p0: i32) { invoke_ignore!(0xA7670F7991099680, p0) }
	pub fn _0x7673C0D2C5CDAC55() { invoke_ignore!(0x7673C0D2C5CDAC55) }
	pub fn _0x3CBD6565D9C3B133(p0: i32, p1: i32, p2: f32) { invoke_ignore!(0x3CBD6565D9C3B133, p0, p1, p2) }
	pub fn _0x0D183D8490EE4366(p0: i32, p1: i32) { invoke_ignore!(0x0D183D8490EE4366, p0, p1) }
	pub fn _0xC1968045EEB563B7(p0: i32) { invoke_ignore!(0xC1968045EEB563B7, p0) }
	pub fn NETWORK_AUTO_SESSION_IS_OBJECT_CREATION_PAUSED() -> bool { invoke!(0x0E2C3AEE6CE603B7) }
	pub fn _0x0B6B4507AC5EA8B8() -> bool { invoke!(0x0B6B4507AC5EA8B8) }
	pub fn NETWORK_IS_RESETTING_POPULATION() -> bool { invoke!(0x1BB50CD340A996E6) }
	pub fn NETWORK_RESET_POPULATION(p0: bool, p1: i32) -> bool { invoke!(0x101F538C25ABB39A, p0, p1) }
	pub fn NETWORK_DISABLE_LEAVE_REMOTE_PED_BEHIND(toggle: bool) { invoke_ignore!(0xC505036A35AFD01B, toggle) }
	pub fn _0x3034C77C79A58880(p0: bool) { invoke_ignore!(0x3034C77C79A58880, p0) }
	pub fn NETWORK_ALLOW_REMOTE_ATTACHMENT_MODIFICATION(entity: Entity, toggle: bool) { invoke_ignore!(0x267C78C60E806B9A, entity, toggle) }
	pub fn NETWORK_SHOW_CHAT_RESTRICTION_MSC(player: Player) { invoke_ignore!(0x6BFF5F84102DF80A, player) }
	pub fn NETWORK_SHOW_PSN_UGC_RESTRICTION() { invoke_ignore!(0x5C497525F803486B) }
	pub fn NETWORK_IS_CONNECTED_VIA_RELAY(player: Player) -> bool { invoke!(0x16D3D49902F697BB, player) }
	pub fn NETWORK_GET_AVERAGE_LATENCY(player: Player) -> f32 { invoke!(0xD414BE129BB81B32, player) }
	pub fn NETWORK_GET_AVERAGE_PING(player: Player) -> f32 { invoke!(0x0E3A041ED6AC2B45, player) }
	pub fn NETWORK_GET_AVERAGE_PACKET_LOSS(player: Player) -> f32 { invoke!(0x350C23949E43686C, player) }
	pub fn NETWORK_GET_NUM_UNACKED_RELIABLES(player: Player) -> i32 { invoke!(0xFF8FCF9FFC458A1C, player) }
	pub fn NETWORK_GET_UNRELIABLE_RESEND_COUNT(player: Player) -> i32 { invoke!(0x3765C3A3E8192E10, player) }
	pub fn NETWORK_GET_HIGHEST_RELIABLE_RESEND_COUNT(player: Player) -> i32 { invoke!(0x52C1EADAF7B10302, player) }
	pub fn NETWORK_DUMP_NET_IF_CONFIG() { invoke_ignore!(0xAEDF1BC1C133D6E3) }
	pub fn NETWORK_GET_NET_STATISTICS_INFO() { invoke_ignore!(0x6FD992C4A1C1B986) }
	pub fn NETWORK_IS_PLAYER_INDEX_VALID(player: Player) -> bool { invoke!(0x255A5EF65EDA9167, player) }
	pub fn _GET_PLAYER_WAYPOINT_IS_ACTIVE(player: Player) -> bool { invoke!(0xDCC4B7F7112E8AB7, player) }
	pub fn _0x455156F47DC6B78C(p0: bool) { invoke_ignore!(0x455156F47DC6B78C, p0) }
	pub fn _SET_LOCAL_PLAYER_DAMAGE_MULTIPLIER_FOR_PLAYER(player: Player, damageMultiplier: f32) { invoke_ignore!(0xD041A32992A55F84, player, damageMultiplier) }
	pub fn NETWORK_TRIGGER_DAMAGE_EVENT_FOR_ZERO_DAMAGE(entity: Entity, p1: bool) { invoke_ignore!(0x0C8BC052AE87D744, entity, p1) }
	pub fn _0x5CD3AAD8FF9ED121(p0: Any) { invoke_ignore!(0x5CD3AAD8FF9ED121, p0) }
	pub fn _0x51951DE06C0D1C40(player: Player, type_: i32) { invoke_ignore!(0x51951DE06C0D1C40, player, type_) }
	pub fn _0xE3AB5EEFCB6671A2(setting: i32) { invoke_ignore!(0xE3AB5EEFCB6671A2, setting) }
	pub fn _0x9B39B0555CC692B5() { invoke_ignore!(0x9B39B0555CC692B5) }
	pub fn _0xFE53B1F8D43F19BF(player1: Player, player2: Player) -> i32 { invoke!(0xFE53B1F8D43F19BF, player1, player2) }
	pub fn _0x862C5040F4888741(player1: Player, player2: Player) -> bool { invoke!(0x862C5040F4888741, player1, player2) }
	pub fn _0x2CD41AC000E6F611() { invoke_ignore!(0x2CD41AC000E6F611) }
	pub fn _0xACC44768AF229042() { invoke_ignore!(0xACC44768AF229042) }
	pub fn _0x7E300B5B86AB1D1A(p0: &mut Any, p1: i32, p2: i32, p3: i32, p4: i32, p5: i32, p6: i32, p7: i32, p8: i32, p9: i32, p10: i32, p11: i32, p12: i32, p13: i32, p14: i32) { invoke_ignore!(0x7E300B5B86AB1D1A, p0, p1, p2, p3, p4, p5, p6, p7, p8, p9, p10, p11, p12, p13, p14) }
	pub fn GET_UNIQUE_INT_FOR_PLAYER(player: Player) -> i32 { invoke!(0x07F723401B9D921C, player) }
	pub fn _0x780A13F780A13F1B(toggle: bool) { invoke_ignore!(0x780A13F780A13F1B, toggle) }
	pub fn _NETWORK_IS_TRACKED_PLAYER_VISIBLE(player: Player, trackedPlayer: Player) -> bool { invoke!(0xE525878A35B9EEBD, player, trackedPlayer) }
	pub fn NETWORK_IS_AIM_CAM_ACTIVE(player: Player) -> bool { invoke!(0x8E7CE19219669AEB, player) }
	pub fn _NETWORK_ALERT(ctx: Hash, lh: Hash, ec: i32, h: i32) { invoke_ignore!(0x1BAA028F52EED310, ctx, lh, ec, h) }
	pub fn _0x19447FCAE97704DC(ctx: Hash, ec: i32, ex: bool, ro: bool) { invoke_ignore!(0x19447FCAE97704DC, ctx, ec, ex, ro) }
	pub fn _0x2C4E98DDA475364F(p0: & CStr) { invoke_ignore!(0x2C4E98DDA475364F, p0) }
	pub fn _0x3F0ABAE38A0515AD(p0: i32, p1: i32) { invoke_ignore!(0x3F0ABAE38A0515AD, p0, p1) }
	pub fn _0x3F2EE18A3E294801(p0: i32) -> i32 { invoke!(0x3F2EE18A3E294801, p0) }
	pub fn _SET_SOCIAL_MATCHMAKING_ALLOWED(toggle: bool) { invoke_ignore!(0x777D0571A466B520, toggle) }
	pub fn _GET_SOCIAL_MATCHMAKING_ALLOWED() -> bool { invoke!(0xD0541EF28E9C4783) }
	pub fn NETWORK_AWARD_HAS_REACHED_MAXCLAIM(p0: Any) -> bool { invoke!(0xFBE782B3165AC8EC, p0) }
	pub fn _0x271F95E55C663B8B(p0: Any, p1: Any) -> Any { invoke!(0x271F95E55C663B8B, p0, p1) }
	pub fn _0x64A36BA85CE01A81(p0: Any, p1: Any, p2: Any, p3: Any) -> Any { invoke!(0x64A36BA85CE01A81, p0, p1, p2, p3) }
	pub fn _0xE10F2D7715ABABEC(p0: Any) -> Any { invoke!(0xE10F2D7715ABABEC, p0) }
	pub fn _0x7A8E8DF782B47EB0(p0: Any, p1: Any, p2: Any) -> Any { invoke!(0x7A8E8DF782B47EB0, p0, p1, p2) }
	pub fn _0x77B299E8799B1332(p0: Any, p1: Any, p2: Any) -> Any { invoke!(0x77B299E8799B1332, p0, p1, p2) }
	pub fn _0x923346025512DFB7(p0: Any) -> Any { invoke!(0x923346025512DFB7, p0) }
	pub fn _NETWORK_GET_XP() -> i32 { invoke!(0xDB438CC9BC6F4022) }
	pub fn _NETWORK_GET_RANK() -> i32 { invoke!(0x32C90CDFAF40514C) }
	pub fn NETWORK_SET_RECENT_GAMERS_ENABLED(toggle: bool) { invoke_ignore!(0x29FE035D35B8589C, toggle) }
	pub fn _0x273E04A3A7AD1F2D() -> bool { invoke!(0x273E04A3A7AD1F2D) }
	pub fn _NETWORK_ADD_PLAYER_TO_RECENT_GAMERS_LIST(player: Player, p1: i32) { invoke_ignore!(0x157D8F3DE12B307F, player, p1) }
	pub fn NETWORK_REQUEST_RECENT_GAMER_NAMES(p0: i32, playerCount: i32) -> bool { invoke!(0x6D206D383BB5F6B1, p0, playerCount) }
	pub fn _NETWORK_IS_RECENT_GAMER_NAMES_REQUEST_IN_PROGRESS() -> bool { invoke!(0x4664D213A0CCAF40) }
	pub fn _NETWORK_DID_RECENT_GAMER_NAMES_REQUEST_SUCCEED() -> bool { invoke!(0x12AEB56B489415C5) }
	pub fn _NETWORK_GET_NUM_RECENT_GAMERS() -> i32 { invoke!(0x37A834AEC6A4F74A) }
	pub fn NETWORK_GET_RECENT_GAMER_NAMES(p0: i32, p1: i32, outData: &mut Any, dataSize: i32) -> bool { invoke!(0xFEFCC345CE357453, p0, p1, outData, dataSize) }
	pub fn _0x49CF17A564918E8D() { invoke_ignore!(0x49CF17A564918E8D) }
	pub fn _0xD637D327080CD86E(p0: i32) { invoke_ignore!(0xD637D327080CD86E, p0) }
	pub fn _0x564552C6AF1EEAB1() { invoke_ignore!(0x564552C6AF1EEAB1) }
	pub fn NETWORK_ACTIVITY_RESET_TO_IDLE() { invoke_ignore!(0x3FE141FDB990E3D1) }
	pub fn NETWORK_ACTIVITY_SET_CURRENT(netPlaylistActivity: i32) { invoke_ignore!(0x9ADAC065D9F6706F, netPlaylistActivity) }
	pub fn _NETWORK_GET_ROS_TITLE_NAME() -> *const char { invoke!(0xAC6153A0722F524C) }
	pub fn _REPORT_PLAYER(player: Player, reportType: i32, description: & CStr, horseName: & CStr) { invoke_ignore!(0xA197C35F73AC0F12, player, reportType, description, horseName) }
}
pub mod OBJECT {
	use std::ffi::CStr;
	use crate::core::scripthook::native::*;
	use crate::core::types::*;

	pub fn CREATE_OBJECT(modelHash: Hash, x: f32, y: f32, z: f32, isNetwork: bool, bScriptHostObj: bool, dynamic: bool, p7: bool, p8: bool) -> Object { invoke!(0x509D5878EB39E842, modelHash, x, y, z, isNetwork, bScriptHostObj, dynamic, p7, p8) }
	pub fn CREATE_OBJECT_NO_OFFSET(modelHash: Hash, x: f32, y: f32, z: f32, isNetwork: bool, bScriptHostObj: bool, dynamic: bool, p7: bool) -> Object { invoke!(0x9A294B2138ABB884, modelHash, x, y, z, isNetwork, bScriptHostObj, dynamic, p7) }
	pub fn DELETE_OBJECT(object: &mut Object) { invoke_ignore!(0x931914268722C263, object) }
	pub fn PLACE_OBJECT_ON_GROUND_PROPERLY(object: Object, p1: bool) -> bool { invoke!(0x58A850EAEE20FAA3, object, p1) }
	pub fn SLIDE_OBJECT(object: Object, toX: f32, toY: f32, toZ: f32, speedX: f32, speedY: f32, speedZ: f32, collision: bool) -> bool { invoke!(0x2FDFF4107B8C1147, object, toX, toY, toZ, speedX, speedY, speedZ, collision) }
	pub fn SET_OBJECT_TARGETTABLE(object: Object, targettable: bool) { invoke_ignore!(0x8A7391690F5AFD81, object, targettable) }
	pub fn _SET_OBJECT_TARGETTABLE_2(object: Object, targettable: bool) { invoke_ignore!(0x581EDBE56E8D62C9, object, targettable) }
	pub fn _SET_OBJECT_TARGETTABLE_FOCUS(object: Object, p1: bool, p2: bool) { invoke_ignore!(0xA22712E8471AA08E, object, p1, p2) }
	pub fn _0xF6E88489B4E6EBE5(p0: Any, p1: Any) { invoke_ignore!(0xF6E88489B4E6EBE5, p0, p1) }
	pub fn _0xE157A8A336C7F04A(p0: Any, p1: Any) { invoke_ignore!(0xE157A8A336C7F04A, p0, p1) }
	pub fn _0x46CBCF0E98A4E156(p0: Any, p1: Any) { invoke_ignore!(0x46CBCF0E98A4E156, p0, p1) }
	pub fn GET_CLOSEST_OBJECT_OF_TYPE(x: f32, y: f32, z: f32, radius: f32, modelHash: Hash, missionScriptObject: bool, scriptHostObject: bool, networkObject: bool) -> Object { invoke!(0xE143FA2249364369, x, y, z, radius, modelHash, missionScriptObject, scriptHostObject, networkObject) }
	pub fn HAS_OBJECT_BEEN_BROKEN(p0: Any) -> bool { invoke!(0x8ABFB70C49CC43E2, p0) }
	pub fn HAS_CLOSEST_OBJECT_OF_TYPE_BEEN_BROKEN(p0: f32, p1: f32, p2: f32, p3: f32, modelHash: Hash, p5: Any) -> bool { invoke!(0x761B0E69AC4D007E, p0, p1, p2, p3, modelHash, p5) }
	pub fn GET_OFFSET_FROM_COORD_AND_HEADING_IN_WORLD_COORDS(xPos: f32, yPos: f32, zPos: f32, heading: f32, xOffset: f32, yOffset: f32, zOffset: f32) -> Vector3 { invoke!(0x163E252DE035A133, xPos, yPos, zPos, heading, xOffset, yOffset, zOffset) }
	pub fn _ADD_DOOR_TO_SYSTEM_NEW(doorHash: Hash, p1: bool, p2: bool, p3: bool, threadId: i32, p5: i32, p6: bool) { invoke_ignore!(0xD99229FE93B46286, doorHash, p1, p2, p3, threadId, p5, p6) }
	pub fn _IS_DOOR_REGISTERED_WITH_NETWORK(doorHash: Hash) -> bool { invoke!(0xB5DED7B65C604FDF, doorHash) }
	pub fn REMOVE_DOOR_FROM_SYSTEM(doorHash: Hash) { invoke_ignore!(0x464D8E1427156FE4, doorHash) }
	pub fn DOOR_SYSTEM_SET_DOOR_STATE(doorHash: Hash, state: i32) { invoke_ignore!(0x6BAB9442830C7F53, doorHash, state) }
	pub fn DOOR_SYSTEM_GET_DOOR_STATE(doorHash: Hash) -> i32 { invoke!(0x160AA1B32F6139B8, doorHash) }
	pub fn DOOR_SYSTEM_SET_AUTOMATIC_RATE(doorHash: Hash, rate: f32) { invoke_ignore!(0x03C27E13B42A0E82, doorHash, rate) }
	pub fn DOOR_SYSTEM_SET_AUTOMATIC_DISTANCE(doorHash: Hash, distance: f32) { invoke_ignore!(0x9BA001CB45CBF627, doorHash, distance) }
	pub fn _0xB3B1546D23DF8DE1(p0: Any, p1: Any, p2: Any, p3: Any, p4: Any) { invoke_ignore!(0xB3B1546D23DF8DE1, p0, p1, p2, p3, p4) }
	pub fn DOOR_SYSTEM_SET_OPEN_RATIO(doorHash: Hash, ajar: f32, forceUpdate: bool) { invoke_ignore!(0xB6E6FBA95C7324AC, doorHash, ajar, forceUpdate) }
	pub fn _0x3A77DAE8B4FD7586(p0: Any, p1: Any) { invoke_ignore!(0x3A77DAE8B4FD7586, p0, p1) }
	pub fn _DOOR_SYSTEM_SET_ABLE_TO_CHANGE_OPEN_RATIO_WHILE_LOCKED(doorHash: Hash, p1: bool) { invoke_ignore!(0x1F1FABFE9B2A1254, doorHash, p1) }
	pub fn _IS_DOOR_REGISTERED_WITH_OWNER(doorHash: Hash) -> bool { invoke!(0x4F89DAD4156BA145, doorHash) }
	pub fn _DOOR_SYSTEM_CHANGE_SCRIPT_OWNER(doorHash: Hash) { invoke_ignore!(0x985767F5FA45BC44, doorHash) }
	pub fn _DOOR_SYSTEM_GET_AUTOMATIC_RATE(doorHash: Hash) -> f32 { invoke!(0x8433E1954BE323FC, doorHash) }
	pub fn _0x6E2AA80BB0C03728(p0: Any, p1: Any) -> Any { invoke!(0x6E2AA80BB0C03728, p0, p1) }
	pub fn _DOOR_SYSTEM_SET_AUTOMATIC_STATE(doorHash: Hash, disable: bool) { invoke_ignore!(0x1BC47A9DEDC8DF5D, doorHash, disable) }
	pub fn DOOR_SYSTEM_GET_OPEN_RATIO(doorHash: Hash) -> f32 { invoke!(0x65499865FCA6E5EC, doorHash) }
	pub fn _0x7F458B543006C8FE(p0: Any, p1: Any) { invoke_ignore!(0x7F458B543006C8FE, p0, p1) }
	pub fn _0xACD4F9831DFAD7F5(p0: Any) -> Any { invoke!(0xACD4F9831DFAD7F5, p0) }
	pub fn _0x0C0A373D181BF900(p0: Any) { invoke_ignore!(0x0C0A373D181BF900, p0) }
	pub fn _0xA93F925F1942E434(p0: Any, p1: Any) { invoke_ignore!(0xA93F925F1942E434, p0, p1) }
	pub fn _0x4D8611DFE1126478(p0: Any) -> Any { invoke!(0x4D8611DFE1126478, p0) }
	pub fn _0x57C242543B7B8FB9(p0: Any, p1: Any) { invoke_ignore!(0x57C242543B7B8FB9, p0, p1) }
	pub fn _0x4AE07EBA3462C5D5(p0: Any, p1: Any) { invoke_ignore!(0x4AE07EBA3462C5D5, p0, p1) }
	pub fn _0x22031584496CFB70(p0: Any, p1: Any) { invoke_ignore!(0x22031584496CFB70, p0, p1) }
	pub fn _0xC07B91B996C1DE89(p0: Any, p1: Any) { invoke_ignore!(0xC07B91B996C1DE89, p0, p1) }
	pub fn IS_DOOR_REGISTERED_WITH_SYSTEM(doorHash: Hash) -> bool { invoke!(0xC153C43EA202C8C1, doorHash) }
	pub fn IS_DOOR_CLOSED(doorHash: Hash) -> bool { invoke!(0xC531EE8A1145A149, doorHash) }
	pub fn _0x0943113E02322164(object: Object, p1: i32) -> Any { invoke!(0x0943113E02322164, object, p1) }
	pub fn _0x614D0B4533F842D3(p0: Any) -> Any { invoke!(0x614D0B4533F842D3, p0) }
	pub fn _DOOR_SYSTEM_FORCE_SHUT(doorHash: Hash, p1: bool) { invoke_ignore!(0x276AAF0F1C7F2494, doorHash, p1) }
	pub fn _0xEBA314768FB35D58(p0: Any) -> Any { invoke!(0xEBA314768FB35D58, p0) }
	pub fn _0x5230BF34EB0EC645(p0: Any) { invoke_ignore!(0x5230BF34EB0EC645, p0) }
	pub fn DOES_OBJECT_OF_TYPE_EXIST_AT_COORDS(x: f32, y: f32, z: f32, radius: f32, hash: Hash, p5: bool) -> bool { invoke!(0xBFA48E2FF417213F, x, y, z, radius, hash, p5) }
	pub fn IS_POINT_IN_ANGLED_AREA(p0: f32, p1: f32, p2: f32, p3: f32, p4: f32, p5: f32, p6: f32, p7: f32, p8: f32, p9: f32, p10: bool, p11: bool) -> bool { invoke!(0x2A70BAE8883E4C81, p0, p1, p2, p3, p4, p5, p6, p7, p8, p9, p10, p11) }
	pub fn SET_OBJECT_ALLOW_LOW_LOD_BUOYANCY(object: Object, toggle: bool) { invoke_ignore!(0x4D89D607CB3DD1D2, object, toggle) }
	pub fn SET_OBJECT_PHYSICS_PARAMS(object: Object, weight: f32, p2: f32, p3: f32, p4: f32, p5: f32, gravity: f32, p7: f32, p8: f32, p9: f32, p10: f32, buoyancy: f32) { invoke_ignore!(0xF6DF6E90DE7DF90F, object, weight, p2, p3, p4, p5, gravity, p7, p8, p9, p10, buoyancy) }
	pub fn GET_OBJECT_FRAGMENT_DAMAGE_HEALTH(p0: Any, p1: bool) -> f32 { invoke!(0xB6FBFD079B8D0596, p0, p1) }
	pub fn _0x235C863DA77BD88D(p0: Any, p1: Any, p2: Any) -> Any { invoke!(0x235C863DA77BD88D, p0, p1, p2) }
	pub fn SET_ACTIVATE_OBJECT_PHYSICS_AS_SOON_AS_IT_IS_UNFROZEN(object: Object, toggle: bool) { invoke_ignore!(0x406137F8EF90EAF5, object, toggle) }
	pub fn BREAK_OBJECT_FRAGMENT_CHILD(object: Object, p1: Any, p2: bool) { invoke_ignore!(0xE7E4C198B0185900, object, p1, p2) }
	pub fn BREAK_ALL_OBJECT_FRAGMENT_BONES(object: Object) { invoke_ignore!(0x8462BE2341A55B6F, object) }
	pub fn _0xAAACF33CBF9B990A(p0: Any, p1: Any) { invoke_ignore!(0xAAACF33CBF9B990A, p0, p1) }
	pub fn _DAMAGE_BONE_ON_PROP(object: Object, bone: i32) { invoke_ignore!(0xE4EFB315BCD2A838, object, bone) }
	pub fn FIX_OBJECT_FRAGMENT(object: Object) { invoke_ignore!(0xF9C1681347C8BD15, object) }
	pub fn _0x58DE624FA7FB0E7F(p0: Any) -> Any { invoke!(0x58DE624FA7FB0E7F, p0) }
	pub fn _0x491439AEF410A2FC(p0: Any) { invoke_ignore!(0x491439AEF410A2FC, p0) }
	pub fn _SET_OBJECT_BREAK_SCALE(object: Object, scale: f32) { invoke_ignore!(0xFFB99FFD17F65889, object, scale) }
	pub fn _0xCEAB54F4632C6EF6(p0: Any, p1: Any) { invoke_ignore!(0xCEAB54F4632C6EF6, p0, p1) }
	pub fn TRACK_OBJECT_VISIBILITY(object: Object) { invoke_ignore!(0xB252BC036B525623, object) }
	pub fn IS_OBJECT_VISIBLE(object: Object) -> bool { invoke!(0x8B32ACE6326A7546, object) }
	pub fn SET_OBJECT_TAKES_DAMAGE_FROM_COLLIDING_WITH_BUILDINGS(object: Object, enabled: bool) { invoke_ignore!(0xEB6F1A9B5510A5D2, object, enabled) }
	pub fn ALLOW_DAMAGE_EVENTS_FOR_NON_NETWORKED_OBJECTS(enabled: bool) { invoke_ignore!(0xE2B3B852B537C398, enabled) }
	pub fn _0x6579860A5558524A(p0: Any, p1: Any) { invoke_ignore!(0x6579860A5558524A, p0, p1) }
	pub fn _0xDFA1237F5228263F(p0: Any, p1: Any) { invoke_ignore!(0xDFA1237F5228263F, p0, p1) }
	pub fn _GET_LIGHT_INTENSITY_FROM_OBJECT(object: Object) -> f32 { invoke!(0xFA3B61EC249B4674, object) }
	pub fn _SET_LIGHT_INTENSITY_FOR_OBJECT(object: Object, lightIntensity: f32) { invoke_ignore!(0xF49574E2332A8F06, object, lightIntensity) }
	pub fn _SET_LIGHT_TRANSLUCENCY_FOR_OBJECT(object: Object, value: f32) { invoke_ignore!(0x63E39F09310F481F, object, value) }
	pub fn _SET_LIGHT_SCATTERING_DISABLED_FOR_OBJECT(object: Object, disable: bool) { invoke_ignore!(0x04D1D4E411CE52D0, object, disable) }
	pub fn _0x7FCD49388BC9B775(p0: Any, p1: Any) { invoke_ignore!(0x7FCD49388BC9B775, p0, p1) }
	pub fn _0xFA99E8E575F2FEF8(p0: Any) -> Any { invoke!(0xFA99E8E575F2FEF8, p0) }
	pub fn GET_RAYFIRE_MAP_OBJECT(x: f32, y: f32, z: f32, radius: f32, name: & CStr) -> Object { invoke!(0xB48FCED898292E52, x, y, z, radius, name) }
	pub fn SET_STATE_OF_RAYFIRE_MAP_OBJECT(object: Object, state: i32) { invoke_ignore!(0x5C29F698D404C5E1, object, state) }
	pub fn GET_STATE_OF_RAYFIRE_MAP_OBJECT(object: Object) -> i32 { invoke!(0x899BA936634A322E, object) }
	pub fn DOES_RAYFIRE_MAP_OBJECT_EXIST(object: Object) -> bool { invoke!(0x52AF537A0C5B8AAD, object) }
	pub fn GET_RAYFIRE_MAP_OBJECT_ANIM_PHASE(object: Object) -> f32 { invoke!(0x260EE4FDBDF4DB01, object) }
	pub fn CREATE_PICKUP(pickupHash: Hash, x: f32, y: f32, z: f32, flags: i32, p5: i32, p6: bool, modelHash: Hash, p8: i32, p9: f32, p10: Any) -> Pickup { invoke!(0xFBA08C503DD5FA58, pickupHash, x, y, z, flags, p5, p6, modelHash, p8, p9, p10) }
	pub fn CREATE_PICKUP_ROTATE(pickupHash: Hash, posX: f32, posY: f32, posZ: f32, rotX: f32, rotY: f32, rotZ: f32, flags: i32, p8: i32, p9: i32, p10: bool, modelHash: Hash, p12: i32, p13: f32, p14: Any) -> Pickup { invoke!(0x891804727E0A98B7, pickupHash, posX, posY, posZ, rotX, rotY, rotZ, flags, p8, p9, p10, modelHash, p12, p13, p14) }
	pub fn CREATE_AMBIENT_PICKUP(pickupHash: Hash, x: f32, y: f32, z: f32, flags: i32, value: i32, modelHash: Hash, p7: bool, p8: bool, p9: i32, p10: f32) -> Object { invoke!(0x673966A0C0FD7171, pickupHash, x, y, z, flags, value, modelHash, p7, p8, p9, p10) }
	pub fn CREATE_PORTABLE_PICKUP(pickupHash: Hash, x: f32, y: f32, z: f32, placeOnGround: bool, modelHash: Hash) -> Object { invoke!(0x2EAF1FDB2FB55698, pickupHash, x, y, z, placeOnGround, modelHash) }
	pub fn ATTACH_PORTABLE_PICKUP_TO_PED(pickupObject: Object, ped: Ped) { invoke_ignore!(0x8DC39368BDD57755, pickupObject, ped) }
	pub fn DETACH_PORTABLE_PICKUP_FROM_PED(pickupObject: Object) { invoke_ignore!(0xCF463D1E9A0AECB1, pickupObject) }
	pub fn _HIDE_PICKUP_OBJECT(pickupObject: Object, toggle: bool) { invoke_ignore!(0x2777150CC7D9365E, pickupObject, toggle) }
	pub fn SET_MAX_NUM_PORTABLE_PICKUPS_CARRIED_BY_PLAYER(modelHash: Hash, p1: i32) { invoke_ignore!(0x0BF3B3BD47D79C08, modelHash, p1) }
	pub fn _0x3E2616E7EA539480(p0: Any) -> Any { invoke!(0x3E2616E7EA539480, p0) }
	pub fn SET_LOCAL_PLAYER_CAN_COLLECT_PORTABLE_PICKUPS(toggle: bool) { invoke_ignore!(0x78857FC65CADB909, toggle) }
	pub fn GET_SAFE_PICKUP_COORDS(p0: Any, p1: Any, p2: Any, p3: Any, p4: Any, p5: Any) -> Vector3 { invoke!(0x6E16BC2503FF1FF0, p0, p1, p2, p3, p4, p5) }
	pub fn GET_PICKUP_COORDS(pickup: Pickup) -> Vector3 { invoke!(0x225B8B35C88029B3, pickup) }
	pub fn REMOVE_ALL_PICKUPS_OF_TYPE(pickupHash: Hash) { invoke_ignore!(0x27F9D613092159CF, pickupHash) }
	pub fn HAS_PICKUP_BEEN_COLLECTED(pickup: Pickup) -> bool { invoke!(0x80EC48E6679313F9, pickup) }
	pub fn REMOVE_PICKUP(pickup: Pickup) { invoke_ignore!(0x3288D8ACAECD2AB2, pickup) }
	pub fn SET_PICKUP_DO_NOT_AUTO_PLACE_ON_GROUND(pickupObject: Object) { invoke_ignore!(0x634C19521485AB25, pickupObject) }
	pub fn DOES_PICKUP_EXIST(pickup: Pickup) -> bool { invoke!(0xAFC1CA75AD4074D1, pickup) }
	pub fn DOES_PICKUP_OBJECT_EXIST(pickupObject: Object) -> bool { invoke!(0xD9EFB6DBF7DAAEA3, pickupObject) }
	pub fn GET_PICKUP_OBJECT(pickup: Pickup) -> Object { invoke!(0x5099BC55630B25AE, pickup) }
	pub fn IS_OBJECT_A_PORTABLE_PICKUP(object: Object) -> bool { invoke!(0x0378C08504160D0D, object) }
	pub fn _IS_PICKUP_TYPE_VALID(pickupHash: Hash) -> bool { invoke!(0x007BD043587F7C82, pickupHash) }
	pub fn DOES_PICKUP_OF_TYPE_EXIST_IN_AREA(pickupHash: Hash, x: f32, y: f32, z: f32, radius: f32) -> bool { invoke!(0xF9C36251F6E48E33, pickupHash, x, y, z, radius) }
	pub fn SET_PICKUP_REGENERATION_TIME(pickup: Pickup, duration: i32) { invoke_ignore!(0x78015C9B4B3ECC9D, pickup, duration) }
	pub fn FORCE_PICKUP_REGENERATE(p0: Any) { invoke_ignore!(0x758A5C1B3B1E1990, p0) }
	pub fn _SET_NETWORK_PICKUP_USABLE_FOR_PLAYER(player: Player, pickupHash: Hash, isUsable: bool) { invoke_ignore!(0x94F3D956BFAEAE18, player, pickupHash, isUsable) }
	pub fn SET_LOCAL_PLAYER_PERMITTED_TO_COLLECT_PICKUPS_WITH_MODEL(modelHash: Hash, toggle: bool) { invoke_ignore!(0x88EAEC617CD26926, modelHash, toggle) }
	pub fn BLOCK_PICKUP_FROM_PLAYER_COLLECTION(p0: Any, p1: Any) { invoke_ignore!(0xB8F5062070BB6DBD, p0, p1) }
	pub fn SET_PICKUP_NOT_LOOTABLE(p0: Any, p1: Any) { invoke_ignore!(0x92E87F60F21A0C3A, p0, p1) }
	pub fn _0x1F5E07E14A86FAFC(p0: bool) { invoke_ignore!(0x1F5E07E14A86FAFC, p0) }
	pub fn SET_TEAM_PICKUP_OBJECT(object: Object, p1: Any, p2: bool) { invoke_ignore!(0x53E0DF1A2A3CF0CA, object, p1, p2) }
	pub fn _0x9F52AD67D1A91BAD(p0: Any, p1: Any) -> Any { invoke!(0x9F52AD67D1A91BAD, p0, p1) }
	pub fn PREVENT_COLLECTION_OF_PORTABLE_PICKUP(object: Object, p1: bool, p2: bool) { invoke_ignore!(0x92AEFB5F6E294023, object, p1, p2) }
	pub fn SET_PICKUP_GENERATION_RANGE_MULTIPLIER(multiplier: f32) { invoke_ignore!(0x318516E02DE3ECE2, multiplier) }
	pub fn SET_PICKUP_UNCOLLECTABLE(p0: Any, p1: Any) { invoke_ignore!(0x4A8CB328CD6F1C9B, p0, p1) }
	pub fn SET_PICKUP_HIDDEN_WHEN_UNCOLLECTABLE(p0: Any, p1: Any) { invoke_ignore!(0x81218CE01B672219, p0, p1) }
	pub fn _SET_AMBIENT_PICKUP_LIFETIME(lifetime: i32) { invoke_ignore!(0xAC9AE68F0A463752, lifetime) }
	pub fn SET_PICKUP_PARTICLE_FX_SPAWN(p0: Any, p1: Any) { invoke_ignore!(0xEB9740A38FD6D634, p0, p1) }
	pub fn SET_PICKUP_PARTICLE_FX_HIGHLIGHT(p0: Any, p1: Any) { invoke_ignore!(0x1607C7D9B3021DF5, p0, p1) }
	pub fn SUPPRESS_PICKUP_REWARD_TYPE(rewardType: i32, suppress: bool) { invoke_ignore!(0xF92099527DB8E2A7, rewardType, suppress) }
	pub fn _0x20135AF9C10D2A3D(p0: Any) -> Any { invoke!(0x20135AF9C10D2A3D, p0) }
	pub fn _SET_PICKUP_COLLECTABLE_ON_MOUNT(object: Object) { invoke_ignore!(0x00EE08603EADEE92, object) }
	pub fn _0xDE116ECFFDD4B997(p0: Any, p1: Any) { invoke_ignore!(0xDE116ECFFDD4B997, p0, p1) }
	pub fn GET_WEAPON_TYPE_FROM_PICKUP_TYPE(pickupHash: Hash) -> Hash { invoke!(0x08F96CA6C551AD51, pickupHash) }
	pub fn _GET_AMMO_TYPE_FROM_PICKUP_TYPE(pickupHash: Hash) -> Hash { invoke!(0x44B09A23D728045A, pickupHash) }
	pub fn SET_OBJECT_TINT_INDEX(object: Object, textureVariation: i32) { invoke_ignore!(0x971DA0055324D033, object, textureVariation) }
	pub fn _GET_OBJECT_LIGHT_INTENSITY(object: Object) -> f32 { invoke!(0x3397CD4E0353DFBA, object) }
	pub fn _SET_OBJECT_BURN_OPACITY(object: Object, opacity: f32) { invoke_ignore!(0x7D7285EFEAB5AF15, object, opacity) }
	pub fn _SET_OBJECT_BURN_INTENSITY(object: Object, intensity: f32) { invoke_ignore!(0xC8E21C1677DC5E6F, object, intensity) }
	pub fn _SET_OBJECT_BURN_LEVEL(object: Object, burnLevel: f32, affectAsh: bool) { invoke_ignore!(0x2797C633DCDBBAC5, object, burnLevel, affectAsh) }
	pub fn _0x9A74A9CADFA8A598(p0: Any) { invoke_ignore!(0x9A74A9CADFA8A598, p0) }
	pub fn _RESET_OBJECT_VELOCITY(object: Object) { invoke_ignore!(0xF40AB58D83C35027, object) }
	pub fn _SET_OBJECT_BURN_SPEED(object: Object, speed: f32, p2: f32) { invoke_ignore!(0x646564A3B7DF68F8, object, speed, p2) }
	pub fn _0xCBFBD38F2E0A263B(p0: Any, p1: Any) { invoke_ignore!(0xCBFBD38F2E0A263B, p0, p1) }
	pub fn CONVERT_OLD_PICKUP_TYPE_TO_NEW(pickupHash: Hash) -> Hash { invoke!(0x5EAAD83F8CFB4575, pickupHash) }
	pub fn SET_FORCE_OBJECT_THIS_FRAME(x: f32, y: f32, z: f32, p3: f32) { invoke_ignore!(0xF538081986E49E9D, x, y, z, p3) }
	pub fn _0xD91E55B6C005EB09(p0: Any, p1: Any) -> Any { invoke!(0xD91E55B6C005EB09, p0, p1) }
	pub fn ONLY_CLEAN_UP_OBJECT_WHEN_OUT_OF_RANGE(object: Object) { invoke_ignore!(0xADBE4809F19F927A, object) }
	pub fn _0xCAAF2BCCFEF37F77(object: Object, p1: Any) { invoke_ignore!(0xCAAF2BCCFEF37F77, object, p1) }
	pub fn _0x08C5825A2932EA7B(p0: Any) -> Any { invoke!(0x08C5825A2932EA7B, p0) }
	pub fn _0x7D4411D6736CD295(p0: Any, p1: Any) -> Any { invoke!(0x7D4411D6736CD295, p0, p1) }
	pub fn _0x250EBB11E81A10BE(p0: Any) -> Any { invoke!(0x250EBB11E81A10BE, p0) }
	pub fn _0x2BF1953C0C21AC88(p0: Any) -> Any { invoke!(0x2BF1953C0C21AC88, p0) }
	pub fn CREATE_OBJECT_SKELETON(object: Object) -> bool { invoke!(0xB6CBD40F8EA69E8A, object) }
	pub fn _MAKE_ITEM_CARRIABLE(object: Object) { invoke_ignore!(0x1461DF6DB886BE3F, object) }
	pub fn _0xF65EDE5D02A7A760(p0: Any, p1: Any) { invoke_ignore!(0xF65EDE5D02A7A760, p0, p1) }
	pub fn _SET_AUTO_JUMPABLE_BY_HORSE(object: Object, p1: bool) { invoke_ignore!(0x98D2D9C053A1F449, object, p1) }
	pub fn _SET_NOT_JUMPABLE_BY_HORSE(object: Object, p1: bool) { invoke_ignore!(0xE1C708BA4885796B, object, p1) }
	pub fn _SET_OBJECT_KICKABLE(object: Object, kickable: bool) { invoke_ignore!(0xB7017DA4D498269F, object, kickable) }
	pub fn SET_CUSTOM_TEXTURES_ON_OBJECT(object: Object, txdHash: Hash, p2: Any, p3: Any) { invoke_ignore!(0xE124889AE0521FCF, object, txdHash, p2, p3) }
	pub fn _0xD503D6F0986D58BC(p0: Any, p1: Any) { invoke_ignore!(0xD503D6F0986D58BC, p0, p1) }
	pub fn _0xAEE6C800E124CFE1(p0: Any, p1: Any) { invoke_ignore!(0xAEE6C800E124CFE1, p0, p1) }
	pub fn _0x3DF1A0A58498E209(object: Object, p1: Any) { invoke_ignore!(0x3DF1A0A58498E209, object, p1) }
}
pub mod PAD {
	use std::ffi::CStr;
	use crate::core::scripthook::native::*;
	use crate::core::types::*;

	pub fn _SET_CONTROL_CONTEXT(control: i32, context: Hash) { invoke_ignore!(0x2804658EB7D8A50B, control, context) }
	pub fn _GET_CURRENT_CONTROL_CONTEXT(control: i32) -> Hash { invoke!(0xDDCEB0F26C89C00F, control) }
	pub fn _IS_CONTROL_ACTION_VALID(action: Hash, control: i32) -> bool { invoke!(0xBC0884BC590951C7, action, control) }
	pub fn IS_CONTROL_ENABLED(control: i32, action: Hash) -> bool { invoke!(0x1CEA6BFDF248E5D9, control, action) }
	pub fn IS_CONTROL_PRESSED(control: i32, action: Hash) -> bool { invoke!(0xF3A21BCD95725A4A, control, action) }
	pub fn IS_CONTROL_RELEASED(control: i32, action: Hash) -> bool { invoke!(0x648EE3E7F38877DD, control, action) }
	pub fn IS_CONTROL_JUST_PRESSED(control: i32, action: Hash) -> bool { invoke!(0x580417101DDB492F, control, action) }
	pub fn IS_CONTROL_JUST_RELEASED(control: i32, action: Hash) -> bool { invoke!(0x50F940259D3841E6, control, action) }
	pub fn GET_CONTROL_VALUE(control: i32, action: Hash) -> i32 { invoke!(0xD95E79E8686D2C27, control, action) }
	pub fn GET_CONTROL_NORMAL(control: i32, action: Hash) -> f32 { invoke!(0xEC3C9B8D5327B563, control, action) }
	pub fn GET_CONTROL_UNBOUND_NORMAL(control: i32, action: Hash) -> f32 { invoke!(0x5B84D09CEC5209C5, control, action) }
	pub fn SET_CONTROL_VALUE_NEXT_FRAME(control: i32, action: Hash, value: f32) -> bool { invoke!(0xE8A25867FBA3B05E, control, action, value) }
	pub fn IS_DISABLED_CONTROL_PRESSED(control: i32, action: Hash) -> bool { invoke!(0xE2587F8CBBD87B1D, control, action) }
	pub fn IS_DISABLED_CONTROL_JUST_PRESSED(control: i32, action: Hash) -> bool { invoke!(0x91AEF906BCA88877, control, action) }
	pub fn IS_DISABLED_CONTROL_JUST_RELEASED(control: i32, action: Hash) -> bool { invoke!(0x305C8DCD79DA8B0F, control, action) }
	pub fn GET_DISABLED_CONTROL_NORMAL(control: i32, action: Hash) -> f32 { invoke!(0x11E65974A982637C, control, action) }
	pub fn GET_DISABLED_CONTROL_UNBOUND_NORMAL(control: i32, action: Hash) -> f32 { invoke!(0x4F8A26A890FD62FB, control, action) }
	pub fn GET_CONTROL_HOW_LONG_AGO(control: i32) -> i32 { invoke!(0xD7D22F5592AED8BA, control) }
	pub fn _GET_DISABLED_CONTROL_HOW_LONG_AGO(control: i32) -> i32 { invoke!(0x771DFCB24D19C2F6, control) }
	pub fn IS_USING_KEYBOARD_AND_MOUSE(control: i32) -> bool { invoke!(0xA571D46727E2B718, control) }
	pub fn _0x43F35DDB2905D945(p0: Any, p1: Any) -> Any { invoke!(0x43F35DDB2905D945, p0, p1) }
	pub fn _0xBD629C1C4F501C80(p0: Any) -> Any { invoke!(0xBD629C1C4F501C80, p0) }
	pub fn HAVE_CONTROLS_CHANGED(control: i32) -> bool { invoke!(0x6CD79468A1E595C6, control) }
	pub fn SET_CONTROL_LIGHT_EFFECT_COLOR(control: i32, red: i32, green: i32, blue: i32) { invoke_ignore!(0x8290252FFF36ACB5, control, red, green, blue) }
	pub fn CLEAR_CONTROL_LIGHT_EFFECT(control: i32) { invoke_ignore!(0xCB0360EFEFB2580D, control) }
	pub fn SET_CONTROL_LIGHT_EFFECT_FLASHING_COLOR(control: i32, red: i32, green: i32, blue: i32) { invoke_ignore!(0xA45884DB10EC7EE3, control, red, green, blue) }
	pub fn SET_CONTROL_SHAKE(control: i32, duration: i32, frequency: i32) { invoke_ignore!(0x48B3886C1358D0D5, control, duration, frequency) }
	pub fn SET_CONTROL_TRIGGER_SHAKE(control: i32, leftDuration: i32, leftFrequency: i32, rightDuration: i32, rightFrequency: i32) { invoke_ignore!(0x14D29BB12D47F68C, control, leftDuration, leftFrequency, rightDuration, rightFrequency) }
	pub fn STOP_CONTROL_SHAKE(control: i32) { invoke_ignore!(0x38C16A305E8CDC8D, control) }
	pub fn SET_CONTROL_SHAKE_SUPPRESSED_ID(control: i32, uniqueId: i32) { invoke_ignore!(0xF239400E16C23E08, control, uniqueId) }
	pub fn CLEAR_CONTROL_SHAKE_SUPPRESSED_ID(control: i32) { invoke_ignore!(0xA0CEFCEA390AAB9B, control) }
	pub fn IS_LOOK_INVERTED() -> bool { invoke!(0x77B612531280010D) }
	pub fn SET_INPUT_EXCLUSIVE(control: i32, action: Hash) { invoke_ignore!(0xEDE476E5EE29EDB1, control, action) }
	pub fn DISABLE_CONTROL_ACTION(control: i32, action: Hash, disableRelatedActions: bool) { invoke_ignore!(0xFE99B66D079CF6BC, control, action, disableRelatedActions) }
	pub fn ENABLE_CONTROL_ACTION(control: i32, action: Hash, enableRelatedActions: bool) { invoke_ignore!(0x351220255D64C155, control, action, enableRelatedActions) }
	pub fn DISABLE_ALL_CONTROL_ACTIONS(control: i32) { invoke_ignore!(0x5F4B6931816E599B, control) }
	pub fn _0x5F217BC1190503D8(rumbleCurve: & CStr, p1: f32) { invoke_ignore!(0x5F217BC1190503D8, rumbleCurve, p1) }
	pub fn _0x709BA8C08C5C008D() { invoke_ignore!(0x709BA8C08C5C008D) }
	pub fn _0x1252C029FC8EBB4D() -> bool { invoke!(0x1252C029FC8EBB4D) }
	pub fn _0x52C68E92D6E23ADD(p0: Any) { invoke_ignore!(0x52C68E92D6E23ADD, p0) }
}
pub mod PATHFIND {
	use std::ffi::CStr;
	use crate::core::scripthook::native::*;
	use crate::core::types::*;

	pub fn SET_ROADS_IN_AREA(xMin: f32, yMin: f32, zMin: f32, xMax: f32, yMax: f32, zMax: f32, p6: Any, p7: Any, p8: Any) { invoke_ignore!(0xBF1A602B5BA52FEE, xMin, yMin, zMin, xMax, yMax, zMax, p6, p7, p8) }
	pub fn SET_ROADS_IN_ANGLED_AREA(p0: Any, p1: Any, p2: Any, p3: Any, p4: Any, p5: Any, p6: Any, p7: Any, p8: Any, p9: Any, p10: Any) { invoke_ignore!(0x1A5AA1208AF5DB59, p0, p1, p2, p3, p4, p5, p6, p7, p8, p9, p10) }
	pub fn SET_ROADS_IN_VOLUME(volume: Volume, p1: bool, p2: bool, p3: bool) { invoke_ignore!(0xC1799FAFD2FDF52B, volume, p1, p2, p3) }
	pub fn RESET_ROADS_IN_VOLUME(volume: Volume, p1: bool) { invoke_ignore!(0xD17672447692478E, volume, p1) }
	pub fn SET_ROADS_BACK_TO_ORIGINAL(xMin: f32, yMin: f32, zMin: f32, xMax: f32, yMax: f32, zMax: f32, p6: Any, p7: Any) { invoke_ignore!(0x1EE7063B80FFC77C, xMin, yMin, zMin, xMax, yMax, zMax, p6, p7) }
	pub fn SET_ROADS_BACK_TO_ORIGINAL_IN_ANGLED_AREA(p0: Any, p1: Any, p2: Any, p3: Any, p4: Any, p5: Any, p6: Any, p7: Any, p8: Any) { invoke_ignore!(0x0027501B9F3B407E, p0, p1, p2, p3, p4, p5, p6, p7, p8) }
	pub fn _0xAFE2AE66F6251C66(xMin: f32, yMin: f32, zMin: f32, xMax: f32, yMax: f32, zMax: f32, p6: i32, p7: Any) { invoke_ignore!(0xAFE2AE66F6251C66, xMin, yMin, zMin, xMax, yMax, zMax, p6, p7) }
	pub fn _0x4358BCF14C91761C(p0: Any, p1: Any, p2: Any, p3: Any, p4: Any, p5: Any, p6: Any, p7: Any, p8: Any, p9: Any) { invoke_ignore!(0x4358BCF14C91761C, p0, p1, p2, p3, p4, p5, p6, p7, p8, p9) }
	pub fn _0xB03944057FD735BA(p0: Any, p1: Any, p2: Any) { invoke_ignore!(0xB03944057FD735BA, p0, p1, p2) }
	pub fn _0x6C3F12ECEB6D2E2A(xMin: f32, yMin: f32, zMin: f32, xMax: f32, yMax: f32, zMax: f32, p6: Any, p7: Any) { invoke_ignore!(0x6C3F12ECEB6D2E2A, xMin, yMin, zMin, xMax, yMax, zMax, p6, p7) }
	pub fn _0x5A4E1A41E3A02AD0(p0: Any, p1: Any, p2: Any) { invoke_ignore!(0x5A4E1A41E3A02AD0, p0, p1, p2) }
	pub fn GET_CLOSEST_VEHICLE_NODE(x: f32, y: f32, z: f32, outPosition: &mut Vector3, nodeType: i32, p5: f32, p6: f32) -> bool { invoke!(0x240A18690AE96513, x, y, z, outPosition, nodeType, p5, p6) }
	pub fn _0xCA27A86CAA4E98ED(p0: Any, p1: Any, p2: Any, p3: Any, p4: Any, p5: Any, p6: Any) -> Any { invoke!(0xCA27A86CAA4E98ED, p0, p1, p2, p3, p4, p5, p6) }
	pub fn GET_CLOSEST_VEHICLE_NODE_WITH_HEADING(x: f32, y: f32, z: f32, outPosition: &mut Vector3, outHeading: &mut f32, nodeType: i32, p6: f32, p7: f32) -> bool { invoke!(0x23CFFD4CCB243354, x, y, z, outPosition, outHeading, nodeType, p6, p7) }
	pub fn GET_NTH_CLOSEST_VEHICLE_NODE(x: f32, y: f32, z: f32, nthClosest: i32, outPosition: &mut Vector3, unknown1: i32, unknown2: f32, unknown3: Any) -> bool { invoke!(0x5A6D8DF6FBC5D0C4, x, y, z, nthClosest, outPosition, unknown1, unknown2, unknown3) }
	pub fn GET_NTH_CLOSEST_VEHICLE_NODE_ID(x: f32, y: f32, z: f32, nth: i32, nodetype: i32, p5: f32, p6: f32) -> i32 { invoke!(0x116443008E5CEFC3, x, y, z, nth, nodetype, p5, p6) }
	pub fn GET_NTH_CLOSEST_VEHICLE_NODE_WITH_HEADING(x: f32, y: f32, z: f32, nthClosest: i32, outPosition: &mut Vector3, heading: &mut f32, unknown1: &mut Any, unknown2: i32, unknown3: f32, unknown4: f32) -> bool { invoke!(0x591B40D4390DB54A, x, y, z, nthClosest, outPosition, heading, unknown1, unknown2, unknown3, unknown4) }
	pub fn GET_NTH_CLOSEST_VEHICLE_NODE_ID_WITH_HEADING(x: f32, y: f32, z: f32, nthClosest: i32, returnHeading: &mut f32, returnNumLanes: &mut i32, nodeFlags: i32, zMeasureMult: f32, zTolerance: f32) -> i32 { invoke!(0x4114EAA8A7F7766D, x, y, z, nthClosest, returnHeading, returnNumLanes, nodeFlags, zMeasureMult, zTolerance) }
	pub fn GET_NTH_CLOSEST_VEHICLE_NODE_FAVOUR_DIRECTION(x: f32, y: f32, z: f32, desiredX: f32, desiredY: f32, desiredZ: f32, nthClosest: i32, outPosition: &mut Vector3, outHeading: &mut f32, nodetype: i32, p10: Any, p11: Any) -> bool { invoke!(0x2FAC235A6062F14A, x, y, z, desiredX, desiredY, desiredZ, nthClosest, outPosition, outHeading, nodetype, p10, p11) }
	pub fn IS_VEHICLE_NODE_ID_VALID(vehicleNodeId: i32) -> bool { invoke!(0x5829A02AF4F0B3CB, vehicleNodeId) }
	pub fn GET_VEHICLE_NODE_POSITION(nodeId: i32, outPosition: &mut Vector3) { invoke_ignore!(0x8E8D72FF24DEE1FB, nodeId, outPosition) }
	pub fn GET_VEHICLE_NODE_IS_SWITCHED_OFF(nodeID: i32) -> bool { invoke!(0x28533DBDDF7C2C97, nodeID) }
	pub fn GET_CLOSEST_ROAD(x: f32, y: f32, z: f32, p3: f32, p4: i32, p5: &mut Vector3, p6: &mut Vector3, p7: &mut Any, p8: &mut Any, p9: &mut f32, p10: bool) -> Any { invoke!(0x132F52BBA570FE92, x, y, z, p3, p4, p5, p6, p7, p8, p9, p10) }
	pub fn ARE_NODES_LOADED_FOR_AREA(x1: f32, y1: f32, x2: f32, y2: f32) -> bool { invoke!(0xF7B79A50B905A30D, x1, y1, x2, y2) }
	pub fn REQUEST_PATH_NODES_IN_AREA_THIS_FRAME(x1: f32, y1: f32, x2: f32, y2: f32) -> bool { invoke!(0x07FB139B592FA687, x1, y1, x2, y2) }
	pub fn GET_RANDOM_VEHICLE_NODE(x: f32, y: f32, z: f32, radius: f32, minLanes: i32, avoidDeadEnds: bool, avoidHighways: bool, outPosition: &mut Vector3, nodeId: &mut i32) -> bool { invoke!(0x93E0DB8440B73A7D, x, y, z, radius, minLanes, avoidDeadEnds, avoidHighways, outPosition, nodeId) }
	pub fn _GET_SPAWN_DATA_FOR_ROAD_NODE(nodeId: i32, x: f32, y: f32, z: f32, outCoords: &mut Vector3, heading: &mut f32) { invoke_ignore!(0xA3791B915B8B84C6, nodeId, x, y, z, outCoords, heading) }
	pub fn IS_POINT_ON_ROAD(x: f32, y: f32, z: f32, vehicle: Vehicle) -> bool { invoke!(0x125BF4ABFC536B09, x, y, z, vehicle) }
	pub fn SET_PED_PATHS_IN_AREA(x1: f32, y1: f32, z1: f32, x2: f32, y2: f32, z2: f32, unknown: bool, p7: Any) { invoke_ignore!(0x34F060F4BF92E018, x1, y1, z1, x2, y2, z2, unknown, p7) }
	pub fn _0xE5EF9DE716FF737E(p0: Any, p1: Any, p2: Any) { invoke_ignore!(0xE5EF9DE716FF737E, p0, p1, p2) }
	pub fn GET_SAFE_COORD_FOR_PED(x: f32, y: f32, z: f32, onGround: bool, outPosition: &mut Vector3, flags: i32) -> bool { invoke!(0xB61C8E878A4199CA, x, y, z, onGround, outPosition, flags) }
	pub fn SET_PED_PATHS_BACK_TO_ORIGINAL(p0: Any, p1: Any, p2: Any, p3: Any, p4: Any, p5: Any, p6: Any) { invoke_ignore!(0xE04B48F2CC926253, p0, p1, p2, p3, p4, p5, p6) }
	pub fn _0xCF213A5FC3ABFC08(p0: Any, p1: Any, p2: Any) { invoke_ignore!(0xCF213A5FC3ABFC08, p0, p1, p2) }
	pub fn ADD_NAVMESH_REQUIRED_REGION(x: f32, y: f32, radius: f32) { invoke_ignore!(0x387EAD7EE42F6685, x, y, radius) }
	pub fn IS_NAVMESH_LOADED_IN_AREA(x1: f32, y1: f32, z1: f32, x2: f32, y2: f32, z2: f32) -> bool { invoke!(0xF813C7E63F9062A5, x1, y1, z1, x2, y2, z2) }
	pub fn GET_NUM_NAVMESHES_EXISTING_IN_AREA(p0: f32, p1: f32, p2: f32, p3: f32, p4: f32, p5: f32) -> i32 { invoke!(0x01708E8DD3FF8C65, p0, p1, p2, p3, p4, p5) }
	pub fn _NAVMESH_ACTIVATE_SWAP(name: & CStr) -> bool { invoke!(0x7C334FF4D9215912, name) }
	pub fn _NAVMESH_DEACTIVATE_SWAP(name: & CStr) -> bool { invoke!(0x527B97C203BB8606, name) }
	pub fn _NAVMESH_IS_SWAP_ACTIVE(name: & CStr) -> bool { invoke!(0x5AC0944C156E5F44, name) }
	pub fn _NAVMESH_DOES_SWAP_EXIST(name: & CStr) -> bool { invoke!(0x495CFAB2924237C7, name) }
	pub fn _0x5A3B54ADDF5472A3(p0: & CStr) -> i32 { invoke!(0x5A3B54ADDF5472A3, p0) }
	pub fn _0xA33914B00CA55756(p0: & CStr, p1: i32) -> *mut Any { invoke!(0xA33914B00CA55756, p0, p1) }
	pub fn _NAVMESH_ASSIGN_NAVMESH_TO_VEHICLE(vehicle: Vehicle, navMeshName: & CStr) -> bool { invoke!(0x44026E3DB3CED602, vehicle, navMeshName) }
	pub fn ADD_NAVMESH_BLOCKING_OBJECT(p0: f32, p1: f32, p2: f32, p3: f32, p4: f32, p5: f32, p6: f32, p7: bool, p8: Any) -> Any { invoke!(0xFCD5C8E06E502F5A, p0, p1, p2, p3, p4, p5, p6, p7, p8) }
	pub fn REMOVE_NAVMESH_BLOCKING_OBJECT(p0: Any) { invoke_ignore!(0x46399A7895957C0E, p0) }
	pub fn DOES_NAVMESH_BLOCKING_OBJECT_EXIST(p0: Any) -> bool { invoke!(0x0EAEB0DB4B132399, p0) }
	pub fn _ADD_NAVMESH_BLOCKING_VOLUME(volume: Volume, flags: i32) -> bool { invoke!(0x19C7567D2F2287D6, volume, flags) }
	pub fn _REMOVE_NAVMESH_BLOCKING_VOLUME(volume: Volume) { invoke_ignore!(0x2C87C3E1C7B96EE2, volume) }
	pub fn _DOES_NAVMESH_BLOCKING_VOLUME_EXIST(volume: Volume) -> bool { invoke!(0xDE0EA444735C1368, volume) }
	pub fn _0x6DAD6630AE4A74CB(p0: Any, p1: Any) { invoke_ignore!(0x6DAD6630AE4A74CB, p0, p1) }
	pub fn NAVMESH_REQUEST_PATH(ped: Ped, x1: f32, y1: f32, z1: f32, x2: f32, y2: f32, z2: f32, bitFlag: i32) -> i32 { invoke!(0x348F211CA2404039, ped, x1, y1, z1, x2, y2, z2, bitFlag) }
	pub fn _NAVMESH_CLEAR_REQUESTED_PATH(path: i32) -> bool { invoke!(0x661BB1E1FF77742D, path) }
	pub fn _NAVMESH_REQUESTED_QUERY_STATUS(path: i32) -> i32 { invoke!(0x3A0F82F6EE2291C8, path) }
	pub fn _NAVMESH_REQUESTED_PATH_WAYPOINTS_FOUND(path: i32) -> bool { invoke!(0x8800776E410EB669, path) }
	pub fn _NAVMESH_REQUESTED_PATH_WAYPOINTS_TERRAIN(path: i32) -> i32 { invoke!(0xF61CFEDEAB627BFA, path) }
	pub fn _NAVMESH_REQUESTED_PATH_NUM_WAYPOINTS(path: i32) -> i32 { invoke!(0xD470725E0703D22F, path) }
	pub fn _NAVMESH_REQUESTED_PATH_WAYPOINT_BY_INDEX(path: i32, waypointIndex: i32) -> Vector3 { invoke!(0x430F8319AE56C8A9, path, waypointIndex) }
	pub fn GET_APPROX_FLOOR_FOR_POINT(x: f32, y: f32) -> f32 { invoke!(0x336511A34F2E5185, x, y) }
	pub fn SET_AMBIENT_PED_RANGE_MULTIPLIER_THIS_FRAME(multiplier: f32) { invoke_ignore!(0x0B919E1FB47CC4E0, multiplier) }
	pub fn SET_IGNORE_NO_GPS_FLAG(toggle: bool) { invoke_ignore!(0x72751156E7678833, toggle) }
	pub fn GET_GPS_BLIP_ROUTE_LENGTH() -> i32 { invoke!(0xBBB45C3CF5C8AA85) }
	pub fn _0xEFC535C9FAF563B3(p0: Any) -> Any { invoke!(0xEFC535C9FAF563B3, p0) }
	pub fn _0x665B21666351CB37(p0: Any, p1: Any, p2: Any) -> Any { invoke!(0x665B21666351CB37, p0, p1, p2) }
	pub fn GET_GPS_BLIP_ROUTE_FOUND() -> bool { invoke!(0x869DAACBBE9FA006) }
	pub fn _0x54F4D7B6670FBB5A(p0: Any, p1: Any, p2: Any, p3: Any, p4: Any) -> Any { invoke!(0x54F4D7B6670FBB5A, p0, p1, p2, p3, p4) }
	pub fn _0x34C9AF25649172D0(p0: Any) { invoke_ignore!(0x34C9AF25649172D0, p0) }
	pub fn _0xF2A2177AC848B3A8(volume: Volume, p1: i32, p2: i32) { invoke_ignore!(0xF2A2177AC848B3A8, volume, p1, p2) }
	pub fn _0x4BDEBEA5702B97A9(p0: Any, p1: Any, p2: Any, p3: Any, p4: Any, p5: Any) { invoke_ignore!(0x4BDEBEA5702B97A9, p0, p1, p2, p3, p4, p5) }
	pub fn _0x264E9A5CD78C338F(p0: Any) { invoke_ignore!(0x264E9A5CD78C338F, p0) }
	pub fn _0x869A7015BD4606E9(p0: Any) { invoke_ignore!(0x869A7015BD4606E9, p0) }
	pub fn _SIMULATED_ROUTE_CREATE(x1: f32, y1: f32, z1: f32, x2: f32, y2: f32, z2: f32, p6: i32) -> Any { invoke!(0xFD5BB35AAB83FD48, x1, y1, z1, x2, y2, z2, p6) }
	pub fn _SIMULATED_ROUTE_DELETE(p0: Any) { invoke_ignore!(0x4907D0E4FB26EE65, p0) }
	pub fn _SIMULATED_ROUTE_EXISTS(p0: Any) -> bool { invoke!(0x65A8196B8D7F5E0B, p0) }
	pub fn SIMULATED_ROUTE_IS_LOADED(p0: Any) -> bool { invoke!(0x240915043CB799D7, p0) }
	pub fn SIMULATED_ROUTE_TRAVEL_TO_POINT(p0: Any, p1: f32, p2: f32) { invoke_ignore!(0xA1A3DE1C215C7394, p0, p1, p2) }
	pub fn SIMULATED_ROUTE_GET_ETA(p0: Any) -> f32 { invoke!(0x2DD5F78D73B24172, p0) }
}
pub mod PED {
	use std::ffi::CStr;
	use crate::core::scripthook::native::*;
	use crate::core::types::*;

	pub fn CREATE_PED(modelHash: Hash, x: f32, y: f32, z: f32, heading: f32, isNetwork: bool, bScriptHostPed: bool, p7: bool, p8: bool) -> Ped { invoke!(0xD49F9B0955C367DE, modelHash, x, y, z, heading, isNetwork, bScriptHostPed, p7, p8) }
	pub fn DELETE_PED(ped: &mut Ped) { invoke_ignore!(0xCC0EF140F99365C5, ped) }
	pub fn _SET_REMOVE_PED_NETWORKED(ped: Ped, p1: i32) { invoke_ignore!(0x39A2FC5AF55A52B1, ped, p1) }
	pub fn _0x7C08E7CB8D951B70(ped: Ped, p1: f32) { invoke_ignore!(0x7C08E7CB8D951B70, ped, p1) }
	pub fn _SET_PED_TO_BE_REMOVED(ped: Ped, p1: f32, p2: f32, p3: i32, p4: Any) { invoke_ignore!(0x36E4B61DC56DE77C, ped, p1, p2, p3, p4) }
	pub fn CLONE_PED(ped: Ped, isNetwork: bool, bScriptHostPed: bool, copyHeadBlendFlag: bool) -> Ped { invoke!(0xEF29A16337FACADB, ped, isNetwork, bScriptHostPed, copyHeadBlendFlag) }
	pub fn CLONE_PED_TO_TARGET(ped: Ped, targetPed: Ped) { invoke_ignore!(0xE952D6431689AD9A, ped, targetPed) }
	pub fn IS_PED_IN_VEHICLE(ped: Ped, vehicle: Vehicle, atGetIn: bool) -> bool { invoke!(0xA3EE4A07279BB9DB, ped, vehicle, atGetIn) }
	pub fn IS_PED_IN_MODEL(ped: Ped, modelHash: Hash) -> bool { invoke!(0x796D90EFB19AA332, ped, modelHash) }
	pub fn IS_PED_IN_ANY_VEHICLE(ped: Ped, atGetIn: bool) -> bool { invoke!(0x997ABD671D25CA0B, ped, atGetIn) }
	pub fn _0x9851DE7AEC10B4E1(x: f32, y: f32, z: f32, p3: f32, p4: i32, p5: Any) { invoke_ignore!(0x9851DE7AEC10B4E1, x, y, z, p3, p4, p5) }
	pub fn IS_PED_INJURED(ped: Ped) -> bool { invoke!(0x84A2DD9AC37C35C1, ped) }
	pub fn IS_PED_FATALLY_INJURED(ped: Ped) -> bool { invoke!(0xD839450756ED5A80, ped) }
	pub fn IS_PED_DEAD_OR_DYING(ped: Ped, p1: bool) -> bool { invoke!(0x3317DEDB88C95038, ped, p1) }
	pub fn _SET_PED_BLEEDOUT_PROFILE(ped: Ped, bleedoutProfile: Hash) { invoke_ignore!(0x66C047719B0E80E1, ped, bleedoutProfile) }
	pub fn IS_PED_AIMING_FROM_COVER(ped: Ped) -> bool { invoke!(0x3998B1276A3300E5, ped) }
	pub fn IS_PED_RELOADING(ped: Ped) -> bool { invoke!(0x24B100C68C645951, ped) }
	pub fn IS_PED_A_PLAYER(ped: Ped) -> bool { invoke!(0x12534C348C6CB68B, ped) }
	pub fn _IS_ANIMAL_CONTROLLED_BY_A_PLAYER(ped: Ped) -> bool { invoke!(0x0E2F43516F998269, ped) }
	pub fn CREATE_PED_INSIDE_VEHICLE(vehicle: Vehicle, modelHash: Hash, seatIndex: i32, p3: bool, p4: bool, p5: bool) -> Ped { invoke!(0x7DD959874C1FD534, vehicle, modelHash, seatIndex, p3, p4, p5) }
	pub fn SET_PED_DESIRED_HEADING(ped: Ped, heading: f32) { invoke_ignore!(0xAA5A7ECE2AA8FE70, ped, heading) }
	pub fn FORCE_ALL_HEADING_VALUES_TO_ALIGN(ped: Ped) { invoke_ignore!(0xFF287323B0E2C69A, ped) }
	pub fn IS_PED_FACING_PED(ped: Ped, otherPed: Ped, angle: f32) -> bool { invoke!(0xD71649DB0A545AA3, ped, otherPed, angle) }
	pub fn IS_PED_IN_MELEE_COMBAT(ped: Ped) -> bool { invoke!(0x4E209B2C1EAD5159, ped) }
	pub fn _0x6DB875AFC584FA32(ped: Ped, p1: i32) -> Any { invoke!(0x6DB875AFC584FA32, ped, p1) }
	pub fn IS_PED_STOPPED(ped: Ped) -> bool { invoke!(0x530944F6F4B8A214, ped) }
	pub fn IS_PED_SITTING(ped: Ped) -> bool { invoke!(0x84D0BF2B21862059, ped) }
	pub fn _0x09D7AFD3716DA8E1(ped: Ped, p1: i32) -> bool { invoke!(0x09D7AFD3716DA8E1, ped, p1) }
	pub fn _0x9C81338B2E62CE0A(player: Player, ped: Ped, shotNearRecentlyTime: i32) -> bool { invoke!(0x9C81338B2E62CE0A, player, ped, shotNearRecentlyTime) }
	pub fn _0xB7DBB2986B87E230(ped: Ped, p1: f32) -> bool { invoke!(0xB7DBB2986B87E230, ped, p1) }
	pub fn _0xD355E2F1BB41087E(ped: Ped, p1: f32) -> bool { invoke!(0xD355E2F1BB41087E, ped, p1) }
	pub fn IS_ANY_PED_SHOOTING_IN_AREA(x1: f32, y1: f32, z1: f32, x2: f32, y2: f32, z2: f32, p6: bool, p7: bool) -> bool { invoke!(0xA0D3D71EA1086C55, x1, y1, z1, x2, y2, z2, p6, p7) }
	pub fn IS_PED_SHOOTING(ped: Ped) -> bool { invoke!(0x34616828CD07F1A1, ped) }
	pub fn TIME_SINCE_PED_LAST_SHOT(ped: Ped) -> f32 { invoke!(0x285D36C5C72B0569, ped) }
	pub fn SET_PED_ACCURACY(ped: Ped, accuracy: i32) { invoke_ignore!(0x7AEFB85C1D49DEB6, ped, accuracy) }
	pub fn GET_PED_ACCURACY(ped: Ped) -> i32 { invoke!(0x37F4AD56ECBC0CD6, ped) }
	pub fn IS_PED_MODEL(ped: Ped, modelHash: Hash) -> bool { invoke!(0xC9D55B1A358A5BF7, ped, modelHash) }
	pub fn EXPLODE_PED_HEAD(ped: Ped, weaponHash: Hash) { invoke_ignore!(0x2D05CED3A38D0F3A, ped, weaponHash) }
	pub fn ADD_ARMOUR_TO_PED(ped: Ped, amount: i32) { invoke_ignore!(0x5BA652A0CD14DF2F, ped, amount) }
	pub fn _0x5CA20FBE49891BBD(ped: Ped, p1: i32) { invoke_ignore!(0x5CA20FBE49891BBD, ped, p1) }
	pub fn SET_PED_INTO_VEHICLE(ped: Ped, vehicle: Vehicle, seatIndex: i32) { invoke_ignore!(0xF75B0D629E1C063D, ped, vehicle, seatIndex) }
	pub fn SET_PED_MOVE_ANIMS_BLEND_OUT(ped: Ped) { invoke_ignore!(0x9E8C908F41584ECD, ped) }
	pub fn _0x606D529DADA3C940(ped: Ped, p1: Any) { invoke_ignore!(0x606D529DADA3C940, ped, p1) }
	pub fn IS_PED_MALE(ped: Ped) -> bool { invoke!(0x6D9F5FAA7488BA46, ped) }
	pub fn IS_PED_HUMAN(ped: Ped) -> bool { invoke!(0xB980061DA992779D, ped) }
	pub fn GET_VEHICLE_PED_IS_IN(ped: Ped, lastVehicle: bool) -> Vehicle { invoke!(0x9A9112A0FE9A4713, ped, lastVehicle) }
	pub fn RESET_PED_LAST_VEHICLE(ped: Ped) { invoke_ignore!(0xBB8DE8CF6A8DD8BB, ped) }
	pub fn _0xC6136B40FFFB778B(p0: bool) { invoke_ignore!(0xC6136B40FFFB778B, p0) }
	pub fn _0x6E8B87139854022D(ped: Ped, clipset: & CStr) { invoke_ignore!(0x6E8B87139854022D, ped, clipset) }
	pub fn _GET_NUM_FREE_SLOTS_IN_PED_POOL() -> i32 { invoke!(0x313778EDCA9158E2) }
	pub fn _RESERVE_AMBIENT_PEDS(numPeds: i32) { invoke_ignore!(0xED9582B3DA8F02B4, numPeds) }
	pub fn _RESERVE_AMBIENT_PEDS_TOTAL(numPeds: i32) { invoke_ignore!(0xF008E0BA1FE1D644, numPeds) }
	pub fn _UNRESERVE_AMBIENT_PEDS(numPeds: i32) { invoke_ignore!(0x7D4E70A67A651C71, numPeds) }
	pub fn _GET_NUM_RESERVED_AMBIENT_PEDS_DESIRED() -> i32 { invoke!(0x62DE46F061CAA468) }
	pub fn _GET_NUM_RESERVED_AMBIENT_PEDS_READY() -> i32 { invoke!(0x5C16855277819BBF) }
	pub fn _ARE_ALL_AMBIENT_PED_RESERVATIONS_READY() -> bool { invoke!(0x5E420FF293EE5472) }
	pub fn _SET_SCENARIO_PED_RANGE_MULTIPLIER_THIS_FRAME(multiplier: f32) { invoke_ignore!(0xA77FA7BE9312F8C0, multiplier) }
	pub fn _0x9E3842E5DAD69F80(volume: Volume) { invoke_ignore!(0x9E3842E5DAD69F80, volume) }
	pub fn _SET_AMBIENT_PED_DENSITY_MULTIPLIER_THIS_FRAME(multiplier: f32) { invoke_ignore!(0xAB0D553FE20A6E25, multiplier) }
	pub fn SET_SCENARIO_PED_DENSITY_MULTIPLIER_THIS_FRAME(multiplier: f32) { invoke_ignore!(0x7A556143A1C03898, multiplier) }
	pub fn _SET_AMBIENT_ANIMAL_DENSITY_MULTIPLIER_THIS_FRAME(multiplier: f32) { invoke_ignore!(0xC0258742B034DFAF, multiplier) }
	pub fn _SET_AMBIENT_HUMAN_DENSITY_MULTIPLIER_THIS_FRAME(multiplier: f32) { invoke_ignore!(0xBA0980B5C0A11924, multiplier) }
	pub fn _SET_SCENARIO_ANIMAL_DENSITY_MULTIPLIER_THIS_FRAME(multiplier: f32) { invoke_ignore!(0xDB48E99F8E064E56, multiplier) }
	pub fn _SET_SCENARIO_HUMAN_DENSITY_MULTIPLIER_THIS_FRAME(multiplier: f32) { invoke_ignore!(0x28CB6391ACEDD9DB, multiplier) }
	pub fn _SET_SCENARIO_PED_DENSITY_THIS_FRAME(configHash: Hash) { invoke_ignore!(0x95423627A9CA598E, configHash) }
	pub fn INSTANTLY_FILL_PED_POPULATION() { invoke_ignore!(0x4759CC730F947C81) }
	pub fn _0xBFA6B7731C3BAF02() { invoke_ignore!(0xBFA6B7731C3BAF02) }
	pub fn IS_INSTANTLY_FILL_PED_POPULATION_FINISHED() -> bool { invoke!(0x0EE3F0D7FECCC54F) }
	pub fn SET_PED_NON_CREATION_AREA(x1: f32, y1: f32, z1: f32, x2: f32, y2: f32, z2: f32) { invoke_ignore!(0xEE01041D559983EA, x1, y1, z1, x2, y2, z2) }
	pub fn CLEAR_PED_NON_CREATION_AREA() { invoke_ignore!(0x2E05208086BA0651) }
	pub fn _ATTACH_VOLUME_TO_ENTITY(volume: Volume, entity: Entity, offsetX: f32, offsetY: f32, offsetZ: f32, rotX: f32, rotY: f32, rotZ: f32, p8: i32, p9: bool) { invoke_ignore!(0x7C00CFC48A782DC0, volume, entity, offsetX, offsetY, offsetZ, rotX, rotY, rotZ, p8, p9) }
	pub fn _DETACH_VOLUME_FROM_ENTITY(volume: Volume, entity: Entity) { invoke_ignore!(0x19C975B81BE53C28, volume, entity) }
	pub fn SET_PED_ONTO_MOUNT(ped: Ped, mount: Ped, seatIndex: i32, p3: bool) { invoke_ignore!(0x028F76B6E78246EB, ped, mount, seatIndex, p3) }
	pub fn _REMOVE_PED_FROM_MOUNT(ped: Ped, p1: bool, p2: bool) { invoke_ignore!(0x5337B721C51883A9, ped, p1, p2) }
	pub fn CREATE_PED_ON_MOUNT(mount: Ped, modelHash: Hash, index: i32, p3: bool, p4: bool, p5: bool, p6: bool) -> Ped { invoke!(0xF89AA2BD01FC06B7, mount, modelHash, index, p3, p4, p5, p6) }
	pub fn _IS_MOUNT_SEAT_FREE(mount: Ped, seat: i32) -> bool { invoke!(0xAAB0FE202E9FC9F0, mount, seat) }
	pub fn IS_PED_ON_MOUNT(ped: Ped) -> bool { invoke!(0x460BC76A0E10655E, ped) }
	pub fn IS_PED_FULLY_ON_MOUNT(ped: Ped, p1: bool) -> bool { invoke!(0x95CBC65780DE7EB1, ped, p1) }
	pub fn GET_MOUNT(ped: Ped) -> Ped { invoke!(0xE7E11B8DCBED1058, ped) }
	pub fn _GET_LAST_LED_MOUNT(ped: Ped) -> Ped { invoke!(0x693126B5D0457D0D, ped) }
	pub fn _GET_LAST_MOUNT(ped: Ped) -> Ped { invoke!(0x4C8B59171957BCF7, ped) }
	pub fn _0xED1C764997A86D5A(ped1: Ped, ped2: Ped) { invoke_ignore!(0xED1C764997A86D5A, ped1, ped2) }
	pub fn _0xB8AB265426CFE6DD(ped: Ped, p1: bool) { invoke_ignore!(0xB8AB265426CFE6DD, ped, p1) }
	pub fn _0xE8D1CCB9375C101B(mount: Ped, player: Player) -> i32 { invoke!(0xE8D1CCB9375C101B, mount, player) }
	pub fn _0xA691C10054275290(mount: Ped, player: Player, dismountedTimestamp: i32) { invoke_ignore!(0xA691C10054275290, mount, player, dismountedTimestamp) }
	pub fn _0x6734F0A6A52C371C(player: Player, horseSlot: i32) { invoke_ignore!(0x6734F0A6A52C371C, player, horseSlot) }
	pub fn _0x024EC9B649111915(ped: Ped, p1: bool) { invoke_ignore!(0x024EC9B649111915, ped, p1) }
	pub fn SET_PED_OWNS_ANIMAL(ped: Ped, animal: Ped, p2: bool) { invoke_ignore!(0x931B241409216C1F, ped, animal, p2) }
	pub fn _GET_ACTIVE_ANIMAL_OWNER(animal: Ped) -> Ped { invoke!(0xF103823FFE72BB49, animal) }
	pub fn _CLEAR_ACTIVE_ANIMAL_OWNER(horse: Ped, clear: bool) { invoke_ignore!(0xBCC76708E5677E1D, horse, clear) }
	pub fn _GET_HORSE_TAMING_STATE(horse: Ped) -> i32 { invoke!(0x454AD4DA6C41B5BD, horse) }
	pub fn _0x54D3CD482742C482(animal: Ped, p2: f32) { invoke_ignore!(0x54D3CD482742C482, animal, p2) }
	pub fn _0x5CB2EBB467BE3ED6(animal: Ped, p2: f32) { invoke_ignore!(0x5CB2EBB467BE3ED6, animal, p2) }
	pub fn _0x9F0F28B42C4EE80A(animal: Ped, p2: f32) { invoke_ignore!(0x9F0F28B42C4EE80A, animal, p2) }
	pub fn _SET_MOUNT_BONDING_LEVEL(ped: Ped, bondingLevel: i32) { invoke_ignore!(0xA69899995997A63B, ped, bondingLevel) }
	pub fn _SET_MOUNT_SECURITY_ENABLED(ped: Ped, toggle: bool) { invoke_ignore!(0x11E6B9629C46D6EC, ped, toggle) }
	pub fn _0x9B65444C07B782BF(ped: Ped, p1: & CStr) { invoke_ignore!(0x9B65444C07B782BF, ped, p1) }
	pub fn IS_ANIMAL_INTERACTION_POSSIBLE(ped: Ped, animal: Ped) -> bool { invoke!(0xD543D3A8FDE4F185, ped, animal) }
	pub fn _IS_ANIMAL_INTERACTION_RUNNING(ped: Ped) -> bool { invoke!(0x7FC84E85D98F063D, ped) }
	pub fn _0x8BE24D74D74C6E9B(ped: Ped) -> Ped { invoke!(0x8BE24D74D74C6E9B, ped) }
	pub fn _0x77243ED4F7CAAA55(ped: Ped) -> bool { invoke!(0x77243ED4F7CAAA55, ped) }
	pub fn IS_PED_ON_VEHICLE(ped: Ped, p1: bool) -> bool { invoke!(0x67722AEB798E5FAB, ped, p1) }
	pub fn IS_PED_ON_SPECIFIC_VEHICLE(ped: Ped, vehicle: Vehicle) -> bool { invoke!(0xEC5F66E459AF3BB2, ped, vehicle) }
	pub fn _WARP_PED_OUT_OF_VEHICLE(ped: Ped) { invoke_ignore!(0xE0B61ED8BB37712F, ped) }
	pub fn SET_PED_MONEY(ped: Ped, amount: i32) { invoke_ignore!(0xA9C8960E8684C1B5, ped, amount) }
	pub fn GET_PED_MONEY(ped: Ped) -> i32 { invoke!(0x3F69145BBA87BAE7, ped) }
	pub fn SET_BLOCKING_OF_NON_TEMPORARY_EVENTS_FOR_AMBIENT_PEDS_THIS_FRAME(p0: bool) { invoke_ignore!(0x9911F4A24485F653, p0) }
	pub fn _0x34EDDD59364AD74A(ped: Ped, p1: &mut Any) { invoke_ignore!(0x34EDDD59364AD74A, ped, p1) }
	pub fn _0x2D976DBDC731DF80(ped: Ped) { invoke_ignore!(0x2D976DBDC731DF80, ped) }
	pub fn IS_PED_SITTING_IN_VEHICLE(ped: Ped, vehicle: Vehicle) -> bool { invoke!(0xA808AA1D79230FC2, ped, vehicle) }
	pub fn IS_PED_SITTING_IN_ANY_VEHICLE(ped: Ped) -> bool { invoke!(0x826AA586EDB9FEF8, ped) }
	pub fn IS_PED_ON_FOOT(ped: Ped) -> bool { invoke!(0x01FEE67DB37F59B2, ped) }
	pub fn IS_PED_PLANTING_BOMB(ped: Ped) -> bool { invoke!(0xC70B5FAE151982D8, ped) }
	pub fn GET_DEAD_PED_PICKUP_COORDS(ped: Ped, p1: f32, p2: f32) -> Vector3 { invoke!(0xCD5003B097200F36, ped, p1, p2) }
	pub fn IS_PED_IN_ANY_BOAT(ped: Ped) -> bool { invoke!(0x2E0E1C2B4F6CB339, ped) }
	pub fn IS_PED_IN_ANY_HELI(ped: Ped) -> bool { invoke!(0x298B91AE825E5705, ped) }
	pub fn IS_PED_IN_ANY_PLANE(ped: Ped) -> bool { invoke!(0x5FFF4CFC74D8FB80, ped) }
	pub fn IS_PED_IN_FLYING_VEHICLE(ped: Ped) -> bool { invoke!(0x9134873537FA419C, ped) }
	pub fn _0x256EDD55C6BE1482(ped: Ped) -> bool { invoke!(0x256EDD55C6BE1482, ped) }
	pub fn GET_PED_LAST_DAMAGE_BONE(ped: Ped, outBone: &mut i32) -> bool { invoke!(0xD75960F6BD9EA49C, ped, outBone) }
	pub fn CLEAR_PED_LAST_DAMAGE_BONE(ped: Ped) { invoke_ignore!(0x8EF6B7AC68E2F01B, ped) }
	pub fn _GET_PED_DAMAGE_CLEANLINESS(ped: Ped) -> i32 { invoke!(0x88EFFED5FE8B0B4A, ped) }
	pub fn _SET_PED_DAMAGE_CLEANLINESS(ped: Ped, damageCleanliness: i32) { invoke_ignore!(0x7528720101A807A5, ped, damageCleanliness) }
	pub fn _GET_PED_DAMAGED(ped: Ped) -> bool { invoke!(0x6CFC373008A1EDAF, ped) }
	pub fn _SET_PED_DAMAGED(ped: Ped, damaged: bool) { invoke_ignore!(0xDACE03C65C6666DB, ped, damaged) }
	pub fn _SET_PLAYER_CURRENT_ANIMAL_DAMAGE_MODIFIER(player: Player, modifier: f32, p2: i32, p3: i32) -> Any { invoke!(0x9EFF3C91DF38304F, player, modifier, p2, p3) }
	pub fn _UPDATE_ANIMAL_DAMAGE_MODIFIER(player: Player) { invoke_ignore!(0x0F9E754EBE8FDBFA, player) }
	pub fn _GET_PLAYER_CURRENT_ANIMAL_DAMAGE_MODIFIER(player: Player) -> f32 { invoke!(0xEE2D5C819A65BF26, player) }
	pub fn _0xB29C553BA582D09E(p0: &mut Any, model: Hash, damageCleanliness: i32, p3: i32) -> Any { invoke!(0xB29C553BA582D09E, p0, model, damageCleanliness, p3) }
	pub fn _0x101B45C5F56D970F(p0: &mut Any, ped: Ped, damageCleanliness: i32, p3: i32) -> bool { invoke!(0x101B45C5F56D970F, p0, ped, damageCleanliness, p3) }
	pub fn _COMPUTE_SATCHEL_ITEM_FOR_PED_CARCASS(outInventoryItemArray: &mut Any, ped: Ped, damageCleanliness: i32, skinningQuality: i32) -> i32 { invoke!(0x6B89FAA36FC909A3, outInventoryItemArray, ped, damageCleanliness, skinningQuality) }
	pub fn COMPUTE_SATCHEL_ITEM_FOR_PED_DAMAGE(p0: Any, pedAttached: Ped, damageCleanliness: i32) -> bool { invoke!(0x9E7738B291706746, p0, pedAttached, damageCleanliness) }
	pub fn SET_AI_WEAPON_DAMAGE_MODIFIER(value: f32) { invoke_ignore!(0x1B1E2A40A65B8521, value) }
	pub fn RESET_AI_WEAPON_DAMAGE_MODIFIER() { invoke_ignore!(0xEA16670E7BA4743C) }
	pub fn _SET_TOTAL_PED_DAMAGE_FROM_AI(ped: Ped, totalDamage: f32) { invoke_ignore!(0x73B6F907B913C860, ped, totalDamage) }
	pub fn _0xA6D6F03095C88F59(ped: Ped) { invoke_ignore!(0xA6D6F03095C88F59, ped) }
	pub fn _GET_TOTAL_PED_DAMAGE_FROM_AI(ped: Ped) -> f32 { invoke!(0x92C8EACA29F6BED6, ped) }
	pub fn SET_PED_TO_PLAYER_WEAPON_DAMAGE_MODIFIER(ped: Ped, damageModifier: f32) { invoke_ignore!(0xD77AE48611B7B10A, ped, damageModifier) }
	pub fn GET_PED_TO_PLAYER_WEAPON_DAMAGE_MODIFIER(ped: Ped) -> f32 { invoke!(0x936E7CAD0AE2EE14, ped) }
	pub fn _SET_CURRENT_DEFENSE_AGAINST_PLAYERS_MODIFIER(horse: Ped, modifier: f32) { invoke_ignore!(0x069EDDF1FD4DEB0A, horse, modifier) }
	pub fn _0x763FA8A9D76EE3A7(ped: Ped) -> f32 { invoke!(0x763FA8A9D76EE3A7, ped) }
	pub fn SET_AI_MELEE_WEAPON_DAMAGE_MODIFIER(modifier: f32) { invoke_ignore!(0x66460DEDDD417254, modifier) }
	pub fn _SET_ACCURACY_AGAINST_LOCAL_PLAYER_MODIFIER(ped: Ped, modifier: f32) { invoke_ignore!(0xC2266AA617668AD3, ped, modifier) }
	pub fn _GET_ACCURACY_AGAINST_LOCAL_PLAYER_MODIFIER(ped: Ped) -> f32 { invoke!(0xDC9273D95976BA22, ped) }
	pub fn _SET_PED_HEADSHOT_DAMAGE_MULTIPLIER(ped: Ped, multiplier: f32) { invoke_ignore!(0x2BA918C823B8BA56, ped, multiplier) }
	pub fn _0xDEE8D30AA5C2E28D(ped: Ped, p1: Hash, p2: bool) { invoke_ignore!(0xDEE8D30AA5C2E28D, ped, p1, p2) }
	pub fn _0xE1B3BE07D3AADDED(ped: Ped, p1: i32, p2: bool) { invoke_ignore!(0xE1B3BE07D3AADDED, ped, p1, p2) }
	pub fn _0x32CEDA9A0AB4CEF7(ped: Ped, p1: Hash, p2: bool) { invoke_ignore!(0x32CEDA9A0AB4CEF7, ped, p1, p2) }
	pub fn _0x52A24D8A1DA89658(ped: Ped, p1: i32, p2: bool) { invoke_ignore!(0x52A24D8A1DA89658, ped, p1, p2) }
	pub fn _0x34B5CEAC180A5D6E(ped: Ped, p1: Hash, p2: bool) { invoke_ignore!(0x34B5CEAC180A5D6E, ped, p1, p2) }
	pub fn _0x4F27603E44A8E4C0(ped: Ped, p1: i32, p2: bool) { invoke_ignore!(0x4F27603E44A8E4C0, ped, p1, p2) }
	pub fn _0x3FDCC1F8C17E303E(ped: Ped, p1: i32, p2: f32) { invoke_ignore!(0x3FDCC1F8C17E303E, ped, p1, p2) }
	pub fn _0xE50C9816B3F22D8B(ped: Ped, p1: Hash, p2: f32) { invoke_ignore!(0xE50C9816B3F22D8B, ped, p1, p2) }
	pub fn _SET_DEFENSE_MODIFIER_FOR_PED(ped: Ped, modifier: f32) { invoke_ignore!(0x9B6808EC46BE849B, ped, modifier) }
	pub fn _SET_MIN_PED_HEALTH_THRESHOLD(ped: Ped, healthAmount: f32) { invoke_ignore!(0x7883AA809DF43D98, ped, healthAmount) }
	pub fn _0xC5B78E41DCF8227C(ped: Ped, p1: bool) { invoke_ignore!(0xC5B78E41DCF8227C, ped, p1) }
	pub fn SET_PED_CAN_BE_TARGETTED(ped: Ped, toggle: bool) { invoke_ignore!(0x63F58F7C80513AAD, ped, toggle) }
	pub fn SET_PED_CAN_BE_TARGETTED_BY_TEAM(ped: Ped, team: i32, toggle: bool) { invoke_ignore!(0xBF1CA77833E58F2C, ped, team, toggle) }
	pub fn SET_PED_CAN_BE_TARGETTED_BY_PLAYER(ped: Ped, player: Player, toggle: bool) { invoke_ignore!(0x66B57B72E0836A76, ped, player, toggle) }
	pub fn _SET_INTERACTION_LOCKON_FLAG(ped: Ped, player: Player, flag: i32, enable: bool) { invoke_ignore!(0xFECA2081F61ED2CD, ped, player, flag, enable) }
	pub fn IS_PED_FALLING(ped: Ped) -> bool { invoke!(0xFB92A102F1C4DFA3, ped) }
	pub fn _IS_PED_SLIDING(ped: Ped) -> bool { invoke!(0xD6740E14E4CEFC0B, ped) }
	pub fn IS_PED_JUMPING(ped: Ped) -> bool { invoke!(0xCEDABC5900A0BF97, ped) }
	pub fn IS_PED_CLIMBING(ped: Ped) -> bool { invoke!(0x53E8CB4F48BFE623, ped) }
	pub fn _IS_PED_CLIMBING_LADDER(ped: Ped) -> bool { invoke!(0x59643424B68D52B5, ped) }
	pub fn _0x577C60BA06D0EA64(ped: Ped) -> bool { invoke!(0x577C60BA06D0EA64, ped) }
	pub fn IS_PED_VAULTING(ped: Ped) -> bool { invoke!(0x117C70D1F5730B5E, ped) }
	pub fn IS_PED_DIVING(ped: Ped) -> bool { invoke!(0x5527B8246FEF9B11, ped) }
	pub fn IS_PED_OPENING_DOOR(ped: Ped) -> bool { invoke!(0x26AF0E8E30BD2A2C, ped) }
	pub fn _0x5C6C7C70CA302801(ped: Ped) -> bool { invoke!(0x5C6C7C70CA302801, ped) }
	pub fn _0xB91AB3BE7F655D49(ped: Ped) -> bool { invoke!(0xB91AB3BE7F655D49, ped) }
	pub fn IS_PED_IN_ANY_TAXI(ped: Ped) -> bool { invoke!(0x6E575D6A898AB852, ped) }
	pub fn SET_PED_ID_RANGE(ped: Ped, value: f32) { invoke_ignore!(0xF107E836A70DCE05, ped, value) }
	pub fn _GET_PED_ID_RANGE(ped: Ped) -> f32 { invoke!(0x31167ED4324B758D, ped) }
	pub fn SET_PED_HIGHLY_PERCEPTIVE(ped: Ped, toggle: bool) { invoke_ignore!(0x52D59AB61DDC05DD, ped, toggle) }
	pub fn SET_PED_INJURED_ON_GROUND_BEHAVIOUR(ped: Ped, unk: f32) { invoke_ignore!(0xEC4B4B3B9908052A, ped, unk) }
	pub fn DISABLE_PED_INJURED_ON_GROUND_BEHAVIOUR(ped: Ped) { invoke_ignore!(0x733C87D4CE22BEA2, ped) }
	pub fn _0x028E7B3BBA0BD2FC(ped: Ped) { invoke_ignore!(0x028E7B3BBA0BD2FC, ped) }
	pub fn _0xFA8C10DCE0706D43(ped: Ped) -> bool { invoke!(0xFA8C10DCE0706D43, ped) }
	pub fn _SET_PED_ANIMAL_DETECTION_MODIFIER(ped: Ped, modifier: f32) { invoke_ignore!(0x43CA928E892CFDB8, ped, modifier) }
	pub fn _0x2BA9D7BF629F920C(ped: Ped) -> f32 { invoke!(0x2BA9D7BF629F920C, ped) }
	pub fn SET_PED_SEEING_RANGE(ped: Ped, value: f32) { invoke_ignore!(0xF29CF591C4BF6CEE, ped, value) }
	pub fn _0x900CA00CE703E1E2(ped: Ped) -> f32 { invoke!(0x900CA00CE703E1E2, ped) }
	pub fn SET_PED_HEARING_RANGE(ped: Ped, value: f32) { invoke_ignore!(0x33A8F7F7D5F7F33C, ped, value) }
	pub fn SET_PED_VISUAL_FIELD_MIN_ANGLE(ped: Ped, value: f32) { invoke_ignore!(0x2DB492222FB21E26, ped, value) }
	pub fn SET_PED_VISUAL_FIELD_MAX_ANGLE(ped: Ped, value: f32) { invoke_ignore!(0x70793BDCA1E854D4, ped, value) }
	pub fn SET_PED_VISUAL_FIELD_PERIPHERAL_RANGE(ped: Ped, range: f32) { invoke_ignore!(0x9C74B0BC831B753A, ped, range) }
	pub fn SET_PED_VISUAL_FIELD_CENTER_ANGLE(ped: Ped, angle: f32) { invoke_ignore!(0x3B6405E8AB34A907, ped, angle) }
	pub fn _0x9AB33CB5834885B3(ped: Ped, p1: f32, p2: f32, p3: f32, p4: f32) { invoke_ignore!(0x9AB33CB5834885B3, ped, p1, p2, p3, p4) }
	pub fn _0x899DFA0009AC93DE(ped: Ped, p1: f32) { invoke_ignore!(0x899DFA0009AC93DE, ped, p1) }
	pub fn _0x3A5697B80FED5EBE(ped: Ped, p1: f32, p2: f32, p3: f32, p4: f32) { invoke_ignore!(0x3A5697B80FED5EBE, ped, p1, p2, p3, p4) }
	pub fn SET_PED_STEALTH_MOVEMENT(ped: Ped, toggle: bool, p2: Any, p3: Any) { invoke_ignore!(0x88CBB5CEB96B7BD2, ped, toggle, p2, p3) }
	pub fn GET_PED_STEALTH_MOVEMENT(ped: Ped) -> bool { invoke!(0x7C2AC9CA66575FBF, ped) }
	pub fn _SET_PED_CROUCH_MOVEMENT(ped: Ped, state: bool, p2: i32, immediately: bool) { invoke_ignore!(0x7DE9692C6F64CFE8, ped, state, p2, immediately) }
	pub fn GET_PED_CROUCH_MOVEMENT(ped: Ped) -> bool { invoke!(0xD5FE956C70FF370B, ped) }
	pub fn GET_PED_IS_DOING_COMBAT_ROLL(ped: Ped) -> bool { invoke!(0xC48A9EB0D499B3E5, ped) }
	pub fn CREATE_GROUP(taskAllocator: i32) -> i32 { invoke!(0x90370EBE0FEE1A3D, taskAllocator) }
	pub fn SET_PED_AS_GROUP_LEADER(ped: Ped, groupId: i32, p2: bool) { invoke_ignore!(0x2A7819605465FBCE, ped, groupId, p2) }
	pub fn SET_PED_AS_GROUP_MEMBER(ped: Ped, groupId: i32) { invoke_ignore!(0x9F3480FE65DB31B5, ped, groupId) }
	pub fn SET_PED_CAN_TELEPORT_TO_GROUP_LEADER(pedHandle: Ped, groupId: i32, toggle: bool) { invoke_ignore!(0x2E2F4240B3F24647, pedHandle, groupId, toggle) }
	pub fn REMOVE_GROUP(groupId: i32) { invoke_ignore!(0x8EB2F69076AF7053, groupId) }
	pub fn REMOVE_PED_FROM_GROUP(ped: Ped) { invoke_ignore!(0xED74007FFB146BC2, ped) }
	pub fn IS_PED_GROUP_MEMBER(ped: Ped, groupId: i32, p2: bool) -> bool { invoke!(0x9BB01E3834671191, ped, groupId, p2) }
	pub fn _IS_PED_GROUP_LEADER(ped: Ped, groupId: i32) -> bool { invoke!(0x878B68960C1C2A35, ped, groupId) }
	pub fn IS_PED_HANGING_ON_TO_VEHICLE(ped: Ped) -> bool { invoke!(0x1C86D8AEF8254B78, ped) }
	pub fn SET_GROUP_SEPARATION_RANGE(groupId: i32, separationRange: f32) { invoke_ignore!(0x4102C7858CFEE4E4, groupId, separationRange) }
	pub fn _0x89E59DBD15E21177(groupId: i32, p1: i32) { invoke_ignore!(0x89E59DBD15E21177, groupId, p1) }
	pub fn IS_PED_PRONE(ped: Ped) -> bool { invoke!(0xD6A86331A537A7B9, ped) }
	pub fn _IS_PED_INVESTIGATING(ped: Ped) -> bool { invoke!(0x7583A9D35248B83F, ped) }
	pub fn IS_PED_IN_COMBAT(ped: Ped, target: Ped) -> bool { invoke!(0x4859F1FC66A6278E, ped, target) }
	pub fn CAN_PED_IN_COMBAT_SEE_TARGET(ped: Ped, target: Ped) -> bool { invoke!(0xEAD42DE3610D0721, ped, target) }
	pub fn IS_PED_JACKING(ped: Ped) -> bool { invoke!(0x4AE4FF911DFB61DA, ped) }
	pub fn IS_PED_BEING_JACKED(ped: Ped) -> bool { invoke!(0x9A497FE2DF198913, ped) }
	pub fn IS_PED_BEING_STUNNED(ped: Ped, weaponType: Hash) -> bool { invoke!(0x4FBACCE3B4138EE8, ped, weaponType) }
	pub fn GET_PEDS_JACKER(ped: Ped) -> Ped { invoke!(0x9B128DC36C1E04CF, ped) }
	pub fn GET_JACK_TARGET(ped: Ped) -> Ped { invoke!(0x5486A79D9FBD342D, ped) }
	pub fn IS_PED_FLEEING(ped: Ped) -> bool { invoke!(0xBBCCE00B381F8482, ped) }
	pub fn IS_PED_IN_COVER(ped: Ped, p1: bool, p2: bool) -> bool { invoke!(0x60DFD0691A170B88, ped, p1, p2) }
	pub fn _0x2DD4E0E26DFAD97D(ped1: Ped, ped2: Ped, p2: f32) -> bool { invoke!(0x2DD4E0E26DFAD97D, ped1, ped2, p2) }
	pub fn _PED_WAS_KILLED_BY_HEADSHOT(ped: Ped) -> bool { invoke!(0x06FA94C835787C64, ped) }
	pub fn _PED_DUELING_DID_PLAYER_HEADSHOT_OPPONENT(ped: Ped) -> bool { invoke!(0xBD6B242B8BD5543A, ped) }
	pub fn IS_PED_IN_COVER_FACING_LEFT(ped: Ped) -> bool { invoke!(0x845333B3150583AB, ped) }
	pub fn IS_PED_GOING_INTO_COVER(ped: Ped) -> bool { invoke!(0x9F65DBC537E59AD5, ped) }
	pub fn IS_PED_RESPONDING_TO_THREAT(ped: Ped) -> bool { invoke!(0x77525BBF433F2CD6, ped) }
	pub fn _GET_ACTIVE_DYNAMIC_SCENARIO_2(ped: Ped) -> Hash { invoke!(0xC22AA08A8ADB87D4, ped) }
	pub fn _GET_ACTIVE_DYNAMIC_SCENARIO(ped: Ped) -> Hash { invoke!(0x569F1E1237508DEB, ped) }
	pub fn _GIVE_PED_SCENARIO_PROP(ped: Ped, object: Object, conditionalAnim: & CStr, p3: & CStr, p4: & CStr, p5: bool) -> bool { invoke!(0x3BBDD6143FF16F98, ped, object, conditionalAnim, p3, p4, p5) }
	pub fn GIVE_PED_HASH_SCENARIO_PROP(ped: Ped, object: Object, conditionalAnim: & CStr, scenarioType: Hash, p4: Hash, p5: bool) -> bool { invoke!(0x2B02DB082258625F, ped, object, conditionalAnim, scenarioType, p4, p5) }
	pub fn _GIVE_PED_SCENARIO_PROP_DYNAMIC(ped: Ped, object: Object, p2: & CStr, p3: & CStr, p4: bool) -> bool { invoke!(0xA0774E388CE4A679, ped, object, p2, p3, p4) }
	pub fn _REQUEST_PROP_SCENARIO_PED(ped: Ped, object: Object, p2: & CStr, p3: & CStr, p4: & CStr, p5: bool) -> Any { invoke!(0xBEC65C6049B3219D, ped, object, p2, p3, p4, p5) }
	pub fn _REQUEST_PED_FOR_SCENARIO_TYPE(ped: Ped, object: Object, p2: & CStr, scenarioType: Hash, p4: & CStr, p5: bool) -> Any { invoke!(0xBDED916A9F9B0604, ped, object, p2, scenarioType, p4, p5) }
	pub fn _GET_PED_REGISTER_PROP(ped: Ped, propName: & CStr, detachProp: bool) -> Entity { invoke!(0x4D0D2E3D8BC000EB, ped, propName, detachProp) }
	pub fn GET_SEAT_PED_IS_TRYING_TO_ENTER(ped: Ped) -> i32 { invoke!(0x6F4C85ACD641BCD2, ped) }
	pub fn GET_PED_SOURCE_OF_DEATH(ped: Ped) -> Entity { invoke!(0x93C8B64DEB84728C, ped) }
	pub fn GET_PED_CAUSE_OF_DEATH(ped: Ped) -> Hash { invoke!(0x16FFE42AB2D2DC59, ped) }
	pub fn GET_PED_TIME_OF_DEATH(ped: Ped) -> i32 { invoke!(0x1E98817B311AE98A, ped) }
	pub fn COUNT_PEDS_IN_COMBAT_WITH_TARGET(ped: Ped, flag: i32) -> i32 { invoke!(0x5407B7288D0478B7, ped, flag) }
	pub fn _GET_PEDS_IN_COMBAT_WITH_TARGET(ped: Ped, itemset: ItemSet, flag: i32) -> i32 { invoke!(0x7BE607DAFF382FD2, ped, itemset, flag) }
	pub fn COUNT_PEDS_IN_COMBAT_WITH_TARGET_WITHIN_RADIUS(ped: Ped, x: f32, y: f32, z: f32, radius: f32, flag: i32) -> i32 { invoke!(0x336B3D200AB007CB, ped, x, y, z, radius, flag) }
	pub fn GET_CURRENT_TARGET_FOR_PED(ped: Ped) -> Entity { invoke!(0xCD66FEA29400A0B5, ped) }
	pub fn SET_PED_RELATIONSHIP_GROUP_DEFAULT_HASH(ped: Ped, hash: Hash) { invoke_ignore!(0xADB3F206518799E8, ped, hash) }
	pub fn _GET_DEFAULT_RELATIONSHIP_GROUP_HASH(modelHash: Hash) -> Hash { invoke!(0x3CC4A718C258BDD0, modelHash) }
	pub fn SET_PED_RELATIONSHIP_GROUP_HASH(ped: Ped, relationshipGroup: Hash) { invoke_ignore!(0xC80A74AC829DDD92, ped, relationshipGroup) }
	pub fn SET_RELATIONSHIP_BETWEEN_GROUPS(relationship: i32, group1: Hash, group2: Hash) { invoke_ignore!(0xBF25EB89375A37AD, relationship, group1, group2) }
	pub fn CLEAR_RELATIONSHIP_BETWEEN_GROUPS(relationship: i32, group1: Hash, group2: Hash) { invoke_ignore!(0x5E29243FB56FC6D4, relationship, group1, group2) }
	pub fn ADD_RELATIONSHIP_GROUP(name: & CStr, groupHash: &mut Hash) -> bool { invoke!(0xF372BC22FCB88606, name, groupHash) }
	pub fn REMOVE_RELATIONSHIP_GROUP(groupHash: Hash) { invoke_ignore!(0xB6BA2444AB393DA2, groupHash) }
	pub fn GET_RELATIONSHIP_BETWEEN_PEDS(ped1: Ped, ped2: Ped) -> i32 { invoke!(0xEBA5AD3A0EAF7121, ped1, ped2) }
	pub fn GET_PED_RELATIONSHIP_GROUP_DEFAULT_HASH(ped: Ped) -> Hash { invoke!(0x42FDD0F017B1E38E, ped) }
	pub fn GET_PED_RELATIONSHIP_GROUP_HASH(ped: Ped) -> Hash { invoke!(0x7DBDD04862D95F04, ped) }
	pub fn GET_RELATIONSHIP_BETWEEN_GROUPS(group1: Hash, group2: Hash) -> i32 { invoke!(0x9E6B70061662AE5C, group1, group2) }
	pub fn _0xDC91F22F09BC6C2F(group: Hash, p1: bool) { invoke_ignore!(0xDC91F22F09BC6C2F, group, p1) }
	pub fn _0x9629FAF6460D35CB(group: Hash, p1: bool) { invoke_ignore!(0x9629FAF6460D35CB, group, p1) }
	pub fn _0x4E68C7EF706DF35D(ped: Ped, x: f32, y: f32, z: f32, p4: f32, relationshipGroup: Hash) { invoke_ignore!(0x4E68C7EF706DF35D, ped, x, y, z, p4, relationshipGroup) }
	pub fn _0x3ACCE14DFA6BA8C2(ped: Ped, p1: i32, x: f32, y: f32, z: f32, p5: f32, itemset: ItemSet) -> i32 { invoke!(0x3ACCE14DFA6BA8C2, ped, p1, x, y, z, p5, itemset) }
	pub fn SET_PED_TO_INFORM_RESPECTED_FRIENDS(ped: Ped, radius: f32, maxFriends: i32) { invoke_ignore!(0x112942C6E708F70B, ped, radius, maxFriends) }
	pub fn _0x40C9155AF8BC13F3(ped: Ped) -> bool { invoke!(0x40C9155AF8BC13F3, ped) }
	pub fn _0xF4860514AD354226(shockingEvent: ScrHandle, x: f32, y: f32, z: f32, p4: f32, p5: &mut i32) -> i32 { invoke!(0xF4860514AD354226, shockingEvent, x, y, z, p4, p5) }
	pub fn IS_PED_RESPONDING_TO_EVENT(ped: Ped, eventType: Hash) -> bool { invoke!(0x625B774D75C87068, ped, eventType) }
	pub fn _0x5E9FAF6C513347B4(ped: Ped, eventType: Hash) -> Entity { invoke!(0x5E9FAF6C513347B4, ped, eventType) }
	pub fn _0x326F7951EF0D7F75(ped: Ped, eventType: Hash) -> Any { invoke!(0x326F7951EF0D7F75, ped, eventType) }
	pub fn _0xE76687023D8C8505(perscharModel: Hash, p1: i32) -> Entity { invoke!(0xE76687023D8C8505, perscharModel, p1) }
	pub fn _0xCB8F4C9343EBE240(ped: Ped, eventType: Hash, coords: &mut Vector3) -> bool { invoke!(0xCB8F4C9343EBE240, ped, eventType, coords) }
	pub fn SET_PED_FIRING_PATTERN(ped: Ped, patternHash: Hash) { invoke_ignore!(0x9AC577F5A12AD8A9, ped, patternHash) }
	pub fn _SET_PED_FIRING_PATTERN_2(ped: Ped, patternHash: Hash) { invoke_ignore!(0x20E54854DEF6A54A, ped, patternHash) }
	pub fn _SET_PED_FIRING_PATTERN_3(ped: Ped, patternHash: Hash) { invoke_ignore!(0x244E8C282188E40F, ped, patternHash) }
	pub fn SET_PED_SHOOT_RATE(ped: Ped, shootRate: i32) { invoke_ignore!(0x614DA022990752DC, ped, shootRate) }
	pub fn _0x3C529A827998F9B3(ped: Ped, p1: i32, p2: i32) { invoke_ignore!(0x3C529A827998F9B3, ped, p1, p2) }
	pub fn _0x1F44B7E283C09EDE(ped: Ped, p1: f32, p2: i32) { invoke_ignore!(0x1F44B7E283C09EDE, ped, p1, p2) }
	pub fn SET_COMBAT_FLOAT(ped: Ped, combatType: i32, newValue: f32) { invoke_ignore!(0xFF41B4B141ED981C, ped, combatType, newValue) }
	pub fn GET_COMBAT_FLOAT(ped: Ped, combatType: i32) -> f32 { invoke!(0x52DFF8A10508090A, ped, combatType) }
	pub fn GET_GROUP_SIZE(groupId: i32, hasLeader: &mut bool, numberOfFollowers: &mut i32) { invoke_ignore!(0x8DE69FE35CA09A45, groupId, hasLeader, numberOfFollowers) }
	pub fn DOES_GROUP_EXIST(groupId: i32) -> bool { invoke!(0x7C6B0C22F9F40BBE, groupId) }
	pub fn _0x0455546F23FF08E4(groupId: i32) -> bool { invoke!(0x0455546F23FF08E4, groupId) }
	pub fn IS_GROUP_LOCALLY_CONTROLLED(groupId: i32) -> bool { invoke!(0x909AD9E9A92A10DF, groupId) }
	pub fn GET_PED_GROUP_INDEX(ped: Ped) -> i32 { invoke!(0xF162E133B4E7A675, ped) }
	pub fn IS_PED_IN_GROUP(ped: Ped) -> bool { invoke!(0x5891CAC5D4ACFF74, ped) }
	pub fn _IS_PED_LEADING_ANY_GROUP(ped: Ped) -> bool { invoke!(0x917760CFE7A0E0F1, ped) }
	pub fn GET_PLAYER_PED_IS_FOLLOWING(ped: Ped) -> Player { invoke!(0x6A3975DEA89F9A17, ped) }
	pub fn SET_GROUP_FORMATION(groupId: i32, formationType: i32) { invoke_ignore!(0xCE2F5FC3AF7E8C1E, groupId, formationType) }
	pub fn _GET_GROUP_FORMATION(groupId: i32) -> i32 { invoke!(0x13A1B061007C906B, groupId) }
	pub fn SET_GROUP_FORMATION_SPACING(groupId: i32, p1: f32, p2: f32, p3: f32) { invoke_ignore!(0x1D9D45004C28C916, groupId, p1, p2, p3) }
	pub fn RESET_GROUP_FORMATION_DEFAULT_SPACING(groupId: i32) { invoke_ignore!(0x63DAB4CCB3273205, groupId) }
	pub fn _0xB05CC690CDE8A4A9(groupId: i32, p1: f32) -> bool { invoke!(0xB05CC690CDE8A4A9, groupId, p1) }
	pub fn ADD_CUSTOM_FORMATION_LOCATION(groupId: i32, x: f32, y: f32, z: f32, position: i32) { invoke_ignore!(0x4E23CD07BD161E06, groupId, x, y, z, position) }
	pub fn ADD_FORMATION_LOCATION(groupId: i32, p1: f32, p2: f32, p3: f32) -> bool { invoke!(0xB05945C1E9E60D91, groupId, p1, p2, p3) }
	pub fn SET_FORMATION_POSITIONS_TARGET_RADIUS(groupId: i32, radius: f32) -> bool { invoke!(0x7CC7D3B7AF7FB71F, groupId, radius) }
	pub fn _SET_FORMATION_AUTO_ASSIGN_POSITION(groupId: i32, toggle: bool) { invoke_ignore!(0x478F6B9920446CE2, groupId, toggle) }
	pub fn _SET_PED_FORMATION_POSITION(ped: Ped, position: i32, toggle: bool) { invoke_ignore!(0x0E9E95FDEDCC9D35, ped, position, toggle) }
	pub fn _0x8AF8E647D6B2A649(groupId: i32, ped: Ped) -> i32 { invoke!(0x8AF8E647D6B2A649, groupId, ped) }
	pub fn _0x87C2724A56F66020(ped: Ped) { invoke_ignore!(0x87C2724A56F66020, ped) }
	pub fn _0xD5BD1B5318A81994(groupId: i32, p1: bool) { invoke_ignore!(0xD5BD1B5318A81994, groupId, p1) }
	pub fn _0x9BBEAF8B0C007F1E(ped: Ped, p1: bool) { invoke_ignore!(0x9BBEAF8B0C007F1E, ped, p1) }
	pub fn _0xC99F104BDF8C7F5A(ped: Ped, p1: bool) { invoke_ignore!(0xC99F104BDF8C7F5A, ped, p1) }
	pub fn _0x02E741E19E39628C(ped: Ped, p1: f32) { invoke_ignore!(0x02E741E19E39628C, ped, p1) }
	pub fn _0x97C475212B327666(groupId: i32, p1: bool) { invoke_ignore!(0x97C475212B327666, groupId, p1) }
	pub fn _0x154B7E841AC7412F(groupId: i32, p1: bool) { invoke_ignore!(0x154B7E841AC7412F, groupId, p1) }
	pub fn _0x8AFCCC0F18D70018(groupId: i32, p1: bool) { invoke_ignore!(0x8AFCCC0F18D70018, groupId, p1) }
	pub fn _0xE1103300F3456DE7(groupId: i32, p1: f32, p2: f32) { invoke_ignore!(0xE1103300F3456DE7, groupId, p1, p2) }
	pub fn _0xA8A95CECB1906EA2(groupId: i32, p1: bool) { invoke_ignore!(0xA8A95CECB1906EA2, groupId, p1) }
	pub fn _0xDDFAD4DEAA7FA362(groupId: i32, p1: f32, p2: f32, p3: f32, p4: f32) { invoke_ignore!(0xDDFAD4DEAA7FA362, groupId, p1, p2, p3, p4) }
	pub fn _0x966DE09688A1DE39(groupId: i32, p1: f32, p2: f32, p3: f32, p4: f32) { invoke_ignore!(0x966DE09688A1DE39, groupId, p1, p2, p3, p4) }
	pub fn _0x7E5185B979706210(groupId: i32, p1: i32) { invoke_ignore!(0x7E5185B979706210, groupId, p1) }
	pub fn _0x40C3524D4ED83554(groupId: i32, p1: bool) { invoke_ignore!(0x40C3524D4ED83554, groupId, p1) }
	pub fn _0x86FAFC18E3D4380C(groupId: i32, p1: bool) { invoke_ignore!(0x86FAFC18E3D4380C, groupId, p1) }
	pub fn _0x07EA5B053FA60AC7(groupId: i32, p1: bool) { invoke_ignore!(0x07EA5B053FA60AC7, groupId, p1) }
	pub fn _0xF9CBD46433E36713(ped: Ped, targetEntity: Entity, p2: f32, p3: f32, p4: f32, p5: f32, p6: f32, p7: f32, p8: f32, p9: & CStr) { invoke_ignore!(0xF9CBD46433E36713, ped, targetEntity, p2, p3, p4, p5, p6, p7, p8, p9) }
	pub fn GET_VEHICLE_PED_IS_USING(ped: Ped) -> Vehicle { invoke!(0x6094AD011A2EA87D, ped) }
	pub fn GET_VEHICLE_PED_IS_ENTERING(ped: Ped) -> Vehicle { invoke!(0xF92691AED837A5FC, ped) }
	pub fn _GET_VEHICLE_DRAFT_HORSE_IS_ATTACHED_TO(horse: Ped) -> Vehicle { invoke!(0xE4770DA1B8FF4FD1, horse) }
	pub fn _GET_LAST_VEHICLE_DRAFT_HORSE_WAS_ATTACHED_TO(horse: Ped) -> Vehicle { invoke!(0x5064DB5083C29921, horse) }
	pub fn GET_SEAT_PED_IS_USING(ped: Ped) -> i32 { invoke!(0x4E76CB57222A00E5, ped) }
	pub fn _GET_TRANSPORT_PED_IS_SEATED_ON(ped: Ped) -> Entity { invoke!(0x849BD6C6314793D0, ped) }
	pub fn IS_PED_ENTERING_ANY_TRANSPORT(ped: Ped) -> bool { invoke!(0x1D46B417F926D34D, ped) }
	pub fn SET_PED_GRAVITY(ped: Ped, toggle: bool) { invoke_ignore!(0x9FF447B6B6AD960A, ped, toggle) }
	pub fn _0x96595B36D6A2279B(animal: Ped, toggle: bool) { invoke_ignore!(0x96595B36D6A2279B, animal, toggle) }
	pub fn _SET_PED_IMMERSION_FLAG(ped: Ped, toggle: bool) { invoke_ignore!(0x7FB0088E8769CDDB, ped, toggle) }
	pub fn _0xA90684ED185CCB4B(animal: Ped, p1: bool, p2: f32, p3: f32) { invoke_ignore!(0xA90684ED185CCB4B, animal, p1, p2, p3) }
	pub fn _FAKE_SET_PED_LOCO_INJURED(ped: Ped, enabled: bool) { invoke_ignore!(0x8B3CB08158E98481, ped, enabled) }
	pub fn _FORCE_PED_DEATH(ped: Ped, pedKiller: Ped, weapon: Hash) { invoke_ignore!(0x1CE875505D45338A, ped, pedKiller, weapon) }
	pub fn APPLY_DAMAGE_TO_PED(ped: Ped, damageAmount: i32, damageArmour: bool, boneId: i32, pedKiller: Ped) { invoke_ignore!(0x697157CED63F18D4, ped, damageAmount, damageArmour, boneId, pedKiller) }
	pub fn _0xBAD2A311667A50D7(ped: Ped, p1: bool) { invoke_ignore!(0xBAD2A311667A50D7, ped, p1) }
	pub fn GET_PED_TYPE(ped: Ped) -> i32 { invoke!(0xFF059E1E4C01E63C, ped) }
	pub fn SET_PED_AS_COP(ped: Ped, toggle: bool) { invoke_ignore!(0xBB03C38DD3FB7FFD, ped, toggle) }
	pub fn _0x405180B14DA5A935(ped: Ped, p1: bool) { invoke_ignore!(0x405180B14DA5A935, ped, p1) }
	pub fn _SET_PED_INTERACTION_PERSONALITY(ped: Ped, personality: Hash) { invoke_ignore!(0x24C82EF607105FAA, ped, personality) }
	pub fn _GET_PED_INTERACTION_PERSONALITY(ped: Ped) -> Hash { invoke!(0xD7AD3C7EBAF88C92, ped) }
	pub fn _SET_PED_PERSONALITY(ped: Ped, personality: Hash) { invoke_ignore!(0xB8B6430EAD2D2437, ped, personality) }
	pub fn _0x329772C47DBB2FBC(ped: Ped) { invoke_ignore!(0x329772C47DBB2FBC, ped) }
	pub fn _GET_IS_PED_IN_DISPUTE_WITH_PED(ped: Ped, pedInDisputeWith: Ped) -> bool { invoke!(0x331550B212014B92, ped, pedInDisputeWith) }
	pub fn _0x94132D7C8D3575C4(ped: Ped) -> bool { invoke!(0x94132D7C8D3575C4, ped) }
	pub fn _0x8AF46E5159A5B620(ped: Ped, speechParams: Hash) { invoke_ignore!(0x8AF46E5159A5B620, ped, speechParams) }
	pub fn _0x45FEA6D5539BD474(ped: Ped, p1: & CStr) { invoke_ignore!(0x45FEA6D5539BD474, ped, p1) }
	pub fn _SET_PED_INTERACTION_POSITIVE_RESPONSE(ped: Ped, speech: & CStr) { invoke_ignore!(0x20C5459379D75C1C, ped, speech) }
	pub fn _0xE37ACEE15AC50C7E(ped: Ped, p1: & CStr) { invoke_ignore!(0xE37ACEE15AC50C7E, ped, p1) }
	pub fn _SET_PED_INTERACTION_NEGATIVE_RESPONSE(ped: Ped, speech: & CStr) { invoke_ignore!(0xA3C53CDE922BC78B, ped, speech) }
	pub fn _0x41C23A8E6B344867(ped: Ped, p1: & CStr) { invoke_ignore!(0x41C23A8E6B344867, ped, p1) }
	pub fn GET_IS_PED_RESPONDING_TO_POSITIVE_INTERACTION(ped: Ped, player: Player) -> bool { invoke!(0x9337183FDA2E9035, ped, player) }
	pub fn GET_IS_PED_RESPONDING_TO_NEGATIVE_INTERACTION(ped: Ped, player: Player) -> bool { invoke!(0xA454D234E45BB6E5, ped, player) }
	pub fn _0xA7DC9266ED6A4E51(ped: Ped) { invoke_ignore!(0xA7DC9266ED6A4E51, ped) }
	pub fn _0x89816B58C3466262(ped: Ped) -> Any { invoke!(0x89816B58C3466262, ped) }
	pub fn _0x97B06669AC569003(ped1: Ped, ped2: Ped) { invoke_ignore!(0x97B06669AC569003, ped1, ped2) }
	pub fn _0x85F500F4E24CA43E(ped: Ped, p1: f32) { invoke_ignore!(0x85F500F4E24CA43E, ped, p1) }
	pub fn _0x9B9B9FA0EA283E3D(ped: Ped, p1: f32) { invoke_ignore!(0x9B9B9FA0EA283E3D, ped, p1) }
	pub fn _0xEC60D1D225BC50AA(ped: Ped, p1: f32) { invoke_ignore!(0xEC60D1D225BC50AA, ped, p1) }
	pub fn _0x12F2D161BF4031FC(ped: Ped, p1: f32) { invoke_ignore!(0x12F2D161BF4031FC, ped, p1) }
	pub fn _0x0ADA3EC589E1736E() { invoke_ignore!(0x0ADA3EC589E1736E) }
	pub fn _GET_IS_PED_BEING_ROBBED(ped: Ped, player: Player, trueUntilPlayerPocketsItem: bool) -> bool { invoke!(0xE33F98BD76490ABC, ped, player, trueUntilPlayerPocketsItem) }
	pub fn _0xD55DB4466D00A258(legendaryAnimal: Ped) -> bool { invoke!(0xD55DB4466D00A258, legendaryAnimal) }
	pub fn SET_PED_MAX_HEALTH(ped: Ped, value: i32) { invoke_ignore!(0xF5F6378C4F3419D3, ped, value) }
	pub fn GET_PED_MAX_HEALTH(ped: Ped) -> i32 { invoke!(0x4700A416E8324EF3, ped) }
	pub fn _SET_PED_HEALTH_CONFIG(ped: Ped, configHash: Hash) { invoke_ignore!(0xF6B82FCE03B43A37, ped, configHash) }
	pub fn INIT_PED_DEFAULT_HEALTH(ped: Ped) { invoke_ignore!(0x7DD7FB3480D8083E, ped) }
	pub fn SET_PED_MAX_TIME_IN_WATER(ped: Ped, value: f32) { invoke_ignore!(0x43C851690662113D, ped, value) }
	pub fn SET_PED_MAX_TIME_UNDERWATER(ped: Ped, value: f32) { invoke_ignore!(0x6BA428C528D9E522, ped, value) }
	pub fn _0xAF041C10756C30FB(ped: Ped, p1: bool, p2: bool, p3: bool) { invoke_ignore!(0xAF041C10756C30FB, ped, p1, p2, p3) }
	pub fn _0x5AF24CA9C974E51A(ped1: Ped, ped2: Ped) { invoke_ignore!(0x5AF24CA9C974E51A, ped1, ped2) }
	pub fn SET_PED_CAN_BE_KNOCKED_OFF_VEHICLE(ped: Ped, state: i32) { invoke_ignore!(0x7A6535691B477C48, ped, state) }
	pub fn CAN_KNOCK_PED_OFF_VEHICLE(ped: Ped) -> bool { invoke!(0x51AC07A44D4F5B8A, ped) }
	pub fn KNOCK_PED_OFF_VEHICLE(ped: Ped) { invoke_ignore!(0x45BBCBA77C29A841, ped) }
	pub fn GET_PED_AS_GROUP_MEMBER(groupID: i32, memberNumber: i32) -> Ped { invoke!(0x51455483CF23ED97, groupID, memberNumber) }
	pub fn GET_PED_AS_GROUP_LEADER(groupID: i32) -> Ped { invoke!(0x5CCE68DBD5FE93EC, groupID) }
	pub fn SET_PED_KEEP_TASK(ped: Ped, toggle: bool) { invoke_ignore!(0x971D38760FBC02EF, ped, toggle) }
	pub fn IS_PED_SWIMMING(ped: Ped) -> bool { invoke!(0x9DE327631295B4C2, ped) }
	pub fn IS_PED_SWIMMING_UNDER_WATER(ped: Ped) -> bool { invoke!(0xC024869A53992F34, ped) }
	pub fn _0xDC88D06719070C39(ped: Ped) -> bool { invoke!(0xDC88D06719070C39, ped) }
	pub fn SET_CREATE_RANDOM_COPS(toggle: bool) { invoke_ignore!(0x102E68B2024D536D, toggle) }
	pub fn IS_PED_IN_ANY_TRAIN(ped: Ped) -> bool { invoke!(0x6F972C1AB75A1ED0, ped) }
	pub fn IS_PED_GETTING_INTO_A_VEHICLE(ped: Ped) -> bool { invoke!(0xBB062B2B5722478E, ped) }
	pub fn _0x550CB89DD7F4FA3D(ped1: Ped, ped2: Ped) -> bool { invoke!(0x550CB89DD7F4FA3D, ped1, ped2) }
	pub fn SET_ENABLE_HANDCUFFS(ped: Ped, p1: bool, p2: bool) { invoke_ignore!(0xDF1AF8B5D56542FA, ped, p1, p2) }
	pub fn SET_ENABLE_BOUND_ANKLES(ped: Ped, toggle: bool) { invoke_ignore!(0xC52E0F855C58FC2E, ped, toggle) }
	pub fn _0x8822F139408B8D0A(ped: Ped) -> bool { invoke!(0x8822F139408B8D0A, ped) }
	pub fn _0x8822F124788B8D0A(ped: Ped, p1: bool) { invoke_ignore!(0x8822F124788B8D0A, ped, p1) }
	pub fn RESET_PED_WEAPON_MOVEMENT_CLIPSET(ped: Ped) { invoke_ignore!(0x97B0DB5B4AA74E77, ped) }
	pub fn _SET_PED_GETUP_ANIMATION(ped: Ped, animName: & CStr, p2: bool) { invoke_ignore!(0x3AE3552E7C207CC5, ped, animName, p2) }
	pub fn _0x88A95BB640FC186F(ped: Ped) { invoke_ignore!(0x88A95BB640FC186F, ped) }
	pub fn RESET_PED_IN_VEHICLE_CONTEXT(ped: Ped) { invoke_ignore!(0x22EF8FF8778030EB, ped) }
	pub fn _0x878E8104FA27CDAE(vehicle: Vehicle, p1: Hash) { invoke_ignore!(0x878E8104FA27CDAE, vehicle, p1) }
	pub fn SET_PED_GESTURE_GROUP(ped: Ped, gesture: & CStr, p2: i32) { invoke_ignore!(0xDDF803377F94AAA8, ped, gesture, p2) }
	pub fn _SET_PED_DESIRED_LOCO_FOR_MODEL(ped: Ped, locomotionArchetype: & CStr) { invoke_ignore!(0x923583741DC87BCE, ped, locomotionArchetype) }
	pub fn _CLEAR_PED_DESIRED_LOCO_FOR_MODEL(ped: Ped) { invoke_ignore!(0x4FD80C3DD84B817B, ped) }
	pub fn _SET_PED_DESIRED_LOCO_MOTION_TYPE(ped: Ped, locoMotionType: & CStr) { invoke_ignore!(0x89F5E7ADECCCB49C, ped, locoMotionType) }
	pub fn _CLEAR_PED_DESIRED_LOCO_MOTION_TYPE(ped: Ped) { invoke_ignore!(0x58F7DB5BD8FA2288, ped) }
	pub fn _0x2371C39D4F91C288(ped: Ped) { invoke_ignore!(0x2371C39D4F91C288, ped) }
	pub fn _REQUEST_PED_GETUP_ANIMATION(ped: Ped, getUpType: & CStr) { invoke_ignore!(0xEAA8242C8479C27D, ped, getUpType) }
	pub fn PED_COWER_IN_PLACE(ped: Ped, ped2: Ped) { invoke_ignore!(0xF6E1E9F47A7686F8, ped, ped2) }
	pub fn PED_COWER_MOVE_TO_POINT(ped: Ped, p1: f32, p2: f32, p3: f32, ped2: Ped, p5: f32) { invoke_ignore!(0x1E4C940233FC0C6F, ped, p1, p2, p3, ped2, p5) }
	pub fn _0x16F798A05BB9E3B5(ped: Ped) { invoke_ignore!(0x16F798A05BB9E3B5, ped) }
	pub fn _PED_EMOTIONAL_PRESET_LOCO_MOTION(ped: Ped, presetName: & CStr, targetPed: Ped, duration: i32, flag: i32) { invoke_ignore!(0xAAB050DA48B57978, ped, presetName, targetPed, duration, flag) }
	pub fn _PED_CLEAR_LOCO_MOTION(ped: Ped) { invoke_ignore!(0x935CF6E42BAF7F4D, ped) }
	pub fn _0x32CCAD8A981B53D3(ped: Ped) { invoke_ignore!(0x32CCAD8A981B53D3, ped) }
	pub fn _SET_PED_DRUNKNESS(ped: Ped, enabled: bool, drunknessLevel: f32) { invoke_ignore!(0x406CCF555B04FAD3, ped, enabled, drunknessLevel) }
	pub fn _IS_PED_DRUNK(ped: Ped) -> bool { invoke!(0x50F124E6EF188B22, ped) }
	pub fn _GET_PED_DRUNKNESS(ped: Ped) -> f32 { invoke!(0x6FB76442469ABD68, ped) }
	pub fn GET_ANIM_INITIAL_OFFSET_POSITION(animDict: & CStr, animName: & CStr, x: f32, y: f32, z: f32, xRot: f32, yRot: f32, zRot: f32, p8: f32, p9: i32) -> Vector3 { invoke!(0xBE22B26DD764C040, animDict, animName, x, y, z, xRot, yRot, zRot, p8, p9) }
	pub fn GET_ANIM_INITIAL_OFFSET_ROTATION(animDict: & CStr, animName: & CStr, x: f32, y: f32, z: f32, xRot: f32, yRot: f32, zRot: f32, p8: f32, p9: i32) -> Vector3 { invoke!(0x4B805E6046EE9E47, animDict, animName, x, y, z, xRot, yRot, zRot, p8, p9) }
	pub fn SET_PED_RANDOM_COMPONENT_VARIATION(ped: Ped, p1: i32) { invoke_ignore!(0xC8A9481A01E63C28, ped, p1) }
	pub fn KNOCK_OFF_PED_PROP(ped: Ped, p1: bool, p2: bool, p3: bool, p4: bool) { invoke_ignore!(0x6FD7816A36615F48, ped, p1, p2, p3, p4) }
	pub fn SET_BLOCKING_OF_NON_TEMPORARY_EVENTS(ped: Ped, toggle: bool) { invoke_ignore!(0x9F8AA94D6D97DBF4, ped, toggle) }
	pub fn _GET_BLOCKING_OF_NON_TEMPORARY_EVENTS(ped: Ped) -> bool { invoke!(0x268B3AEBF032A88D, ped) }
	pub fn _0xC17A94CC8FC3C61A(entity: Entity, boneId: i32, p2: f32, p3: f32, p4: f32) { invoke_ignore!(0xC17A94CC8FC3C61A, entity, boneId, p2, p3, p4) }
	pub fn _SET_PED_SCALE(ped: Ped, scale: f32) { invoke_ignore!(0x25ACFC650B65C538, ped, scale) }
	pub fn _0x134775B093AD5C38(ped: Ped) -> f32 { invoke!(0x134775B093AD5C38, ped) }
	pub fn _GET_PED_HEIGHT(ped: Ped) -> f32 { invoke!(0x1D491CCF7211FB74, ped) }
	pub fn _GET_PED_MODEL_SIZE_FROM_HASH(modelHash: Hash) -> i32 { invoke!(0xA65AA1ACE81E5A77, modelHash) }
	pub fn REGISTER_TARGET(ped: Ped, targetPed: Ped, p2: bool) { invoke_ignore!(0x2F25D9AEFA34FBA2, ped, targetPed, p2) }
	pub fn _REGISTER_HATED_TARGETS_IN_AREA(ped: Ped, x: f32, y: f32, z: f32, radius: f32) { invoke_ignore!(0xD8736EFDA38EDC5C, ped, x, y, z, radius) }
	pub fn REGISTER_HATED_TARGETS_AROUND_PED(ped: Ped, radius: f32) { invoke_ignore!(0x9222F300BF8354FE, ped, radius) }
	pub fn _IS_TARGET(ped: Ped, targetPed: Ped) -> bool { invoke!(0x6E5CBCB3941D7D08, ped, targetPed) }
	pub fn _REMOVE_TARGET(ped: Ped, targetPed: Ped) { invoke_ignore!(0x4707E9C23D8CA3FE, ped, targetPed) }
	pub fn GET_CLOSEST_PED(x: f32, y: f32, z: f32, radius: f32, p4: bool, p5: bool, outPed: &mut Ped, p7: bool, p8: bool, p9: bool, pedType: i32) -> bool { invoke!(0xC33AB876A77F8164, x, y, z, radius, p4, p5, outPed, p7, p8, p9, pedType) }
	pub fn CAN_PED_RAGDOLL(ped: Ped) -> bool { invoke!(0x128F79EDCECE4FD5, ped) }
	pub fn SET_PED_TO_RAGDOLL(ped: Ped, timeMin: i32, timeMax: i32, ragdollType: i32, abortIfInjured: bool, abortIfDead: bool, nmTaskMessageParameterName: & CStr) -> bool { invoke!(0xAE99FB955581844A, ped, timeMin, timeMax, ragdollType, abortIfInjured, abortIfDead, nmTaskMessageParameterName) }
	pub fn SET_PED_TO_RAGDOLL_WITH_FALL(ped: Ped, timeMin: i32, timeMax: i32, ragdollType: i32, falldirX: f32, falldirY: f32, falldirZ: f32, p7: f32, p8: f32, p9: f32, p10: f32, p11: f32, p12: f32, p13: f32) -> bool { invoke!(0xD76632D99E4966C8, ped, timeMin, timeMax, ragdollType, falldirX, falldirY, falldirZ, p7, p8, p9, p10, p11, p12, p13) }
	pub fn SET_PED_RAGDOLL_ON_COLLISION(ped: Ped, toggle: bool, p2: bool) { invoke_ignore!(0xF0A4F1BBF4FA7497, ped, toggle, p2) }
	pub fn _SET_PED_TO_DISABLE_RAGDOLL(ped: Ped, toggle: bool) { invoke_ignore!(0x221F4D9912B7FE86, ped, toggle) }
	pub fn IS_PED_RAGDOLL(ped: Ped) -> bool { invoke!(0x47E4E977581C5B55, ped) }
	pub fn IS_PED_RUNNING_RAGDOLL_TASK(ped: Ped) -> bool { invoke!(0xE3B6097CC25AA69E, ped) }
	pub fn SET_PED_RAGDOLL_FORCE_FALL(ped: Ped) { invoke_ignore!(0x01F6594B923B9251, ped) }
	pub fn _0x8CB2553C559102C1(ped: Ped, p1: i32, p2: bool) { invoke_ignore!(0x8CB2553C559102C1, ped, p1, p2) }
	pub fn _0xFD3C31A2E45671E7(ped: Ped, p1: i32) { invoke_ignore!(0xFD3C31A2E45671E7, ped, p1) }
	pub fn RESET_PED_RAGDOLL_TIMER(ped: Ped) { invoke_ignore!(0x9FA4664CF62E47E8, ped) }
	pub fn SET_PED_CAN_RAGDOLL(ped: Ped, toggle: bool) { invoke_ignore!(0xB128377056A54E2A, ped, toggle) }
	pub fn _0x3AEC4A410ECAF30D(ped: Ped) -> bool { invoke!(0x3AEC4A410ECAF30D, ped) }
	pub fn IS_PED_RUNNING_MOBILE_PHONE_TASK(ped: Ped) -> bool { invoke!(0x2AFE52F782F25775, ped) }
	pub fn SET_RAGDOLL_BLOCKING_FLAGS(ped: Ped, flags: i32) { invoke_ignore!(0x26695EC767728D84, ped, flags) }
	pub fn CLEAR_RAGDOLL_BLOCKING_FLAGS(ped: Ped, flags: i32) { invoke_ignore!(0xD86D101FCFD00A4B, ped, flags) }
	pub fn _0x9F933E0985E12C51(ped: Ped, p1: f32, p2: f32, p3: f32) { invoke_ignore!(0x9F933E0985E12C51, ped, p1, p2, p3) }
	pub fn _0x88B2026A3B0BE33D(ped: Ped, p1: f32) { invoke_ignore!(0x88B2026A3B0BE33D, ped, p1) }
	pub fn SET_PED_DEFENSIVE_AREA_VOLUME(ped: Ped, volume: Volume, p2: bool, p3: bool, p4: bool) { invoke_ignore!(0xFC3DB99C8144CD81, ped, volume, p2, p3, p4) }
	pub fn SET_PED_SPHERE_DEFENSIVE_AREA(ped: Ped, x: f32, y: f32, z: f32, radius: f32, p5: bool, p6: bool, p7: bool) { invoke_ignore!(0x9D3151A373974804, ped, x, y, z, radius, p5, p6, p7) }
	pub fn _SET_PED_DEFENSIVE_SPHERE_ATTACHED_TO_ENTITY(ped: Ped, entity: Entity, x: f32, y: f32, z: f32, radius: f32, p6: i32, p7: bool) { invoke_ignore!(0x1854217C640B39EC, ped, entity, x, y, z, radius, p6, p7) }
	pub fn _SET_PED_DEFENSIVE_AREA_TO_ANGLED_AREA(ped: Ped, x1: f32, y1: f32, z1: f32, x2: f32, y2: f32, z2: f32, p7: Any, p8: bool, p9: bool, entity: Entity, p11: bool) { invoke_ignore!(0xEB2BFE5D009F0331, ped, x1, y1, z1, x2, y2, z2, p7, p8, p9, entity, p11) }
	pub fn SET_PED_DEFENSIVE_AREA_DIRECTION(ped: Ped, p1: f32, p2: f32, p3: f32, p4: bool) { invoke_ignore!(0x413C6C763A4AFFAD, ped, p1, p2, p3, p4) }
	pub fn REMOVE_PED_DEFENSIVE_AREA(ped: Ped, toggle: bool) { invoke_ignore!(0x74D4E028107450A9, ped, toggle) }
	pub fn GET_PED_DEFENSIVE_AREA_POSITION(ped: Ped, p1: bool) -> Vector3 { invoke!(0x3C06B8786DD94CD1, ped, p1) }
	pub fn IS_PED_DEFENSIVE_AREA_ACTIVE(ped: Ped, p1: bool) -> bool { invoke!(0xBA63D9FE45412247, ped, p1) }
	pub fn _GET_PED_DEFENSIVE_VOLUME(ped: Ped, p1: Any) -> Volume { invoke!(0xEF2E6F870783369B, ped, p1) }
	pub fn _0x4EC4EA2F72B36358(ped: Ped, p1: bool) { invoke_ignore!(0x4EC4EA2F72B36358, ped, p1) }
	pub fn _0xCF0B19806473D324(ped: Ped, x: f32, y: f32, z: f32) { invoke_ignore!(0xCF0B19806473D324, ped, x, y, z) }
	pub fn _0xB4B7C92FCE7347B7(ped: Ped) { invoke_ignore!(0xB4B7C92FCE7347B7, ped) }
	pub fn REVIVE_INJURED_PED(ped: Ped) { invoke_ignore!(0x8D8ACD8388CD99CE, ped) }
	pub fn RESURRECT_PED(ped: Ped) { invoke_ignore!(0x71BC8E838B9C6035, ped) }
	pub fn SET_PED_NAME_DEBUG(ped: Ped, name: & CStr) { invoke_ignore!(0x98EFA132A4117BE1, ped, name) }
	pub fn SPECIAL_FUNCTION_DO_NOT_USE(ped: Ped, p1: bool) { invoke_ignore!(0xF9ACF4A08098EA25, ped, p1) }
	pub fn _0x7020839C7302D8AC(ped: Ped) -> bool { invoke!(0x7020839C7302D8AC, ped) }
	pub fn _0xE1AADD0055D76603(ped: Ped, entity: Entity, boneIndex1: i32, boneIndex2: i32, x: f32, y: f32, z: f32, p7: f32, p8: bool, p9: bool, p10: i32) { invoke_ignore!(0xE1AADD0055D76603, ped, entity, boneIndex1, boneIndex2, x, y, z, p7, p8, p9, p10) }
	pub fn _0x5A1A929C8B729B4A(ped: Ped) { invoke_ignore!(0x5A1A929C8B729B4A, ped) }
	pub fn _0x97A38B65EBDA3D50(ped: Ped, p1: bool) { invoke_ignore!(0x97A38B65EBDA3D50, ped, p1) }
	pub fn _0x06A10B4D7F50B0C3(ped: Ped) -> bool { invoke!(0x06A10B4D7F50B0C3, ped) }
	pub fn _0x88A5564B19C15391(ped: Ped) -> bool { invoke!(0x88A5564B19C15391, ped) }
	pub fn _0x354CA4DDDEEC397A(ped: Ped) -> i32 { invoke!(0x354CA4DDDEEC397A, ped) }
	pub fn _0xFEA6126C34DF2532(ped: Ped, p1: bool) { invoke_ignore!(0xFEA6126C34DF2532, ped, p1) }
	pub fn _0xA967D6A8ED2D713B(ped: Ped, p1: bool) { invoke_ignore!(0xA967D6A8ED2D713B, ped, p1) }
	pub fn APPLY_PED_BLOOD_SPECIFIC(ped: Ped, p1: Any, p2: f32, p3: f32, p4: f32, p5: f32, p6: Any, p7: f32, p8: &mut Any) { invoke_ignore!(0xEF0D582CBF2D9B0F, ped, p1, p2, p3, p4, p5, p6, p7, p8) }
	pub fn _0x58D32261AE0F0843(ped: Ped, boneId: i32, p2: f32, p3: f32, p4: f32, p5: f32, p6: f32, p7: f32, p8: & CStr) { invoke_ignore!(0x58D32261AE0F0843, ped, boneId, p2, p3, p4, p5, p6, p7, p8) }
	pub fn _0x735662994E60A710(ped: Ped, p1: bool) { invoke_ignore!(0x735662994E60A710, ped, p1) }
	pub fn _0x91BAB9E064F036CD(p0: Any, p1: Any) { invoke_ignore!(0x91BAB9E064F036CD, p0, p1) }
	pub fn _0x897934E868EDDD6C(ped: Ped, p1: i32, p2: f32, p3: f32, p4: f32) { invoke_ignore!(0x897934E868EDDD6C, ped, p1, p2, p3, p4) }
	pub fn _SET_PED_ACTIVATE_WOUND_EFFECT(ped: Ped, p1: i32, boneId: i32, moveWoundLeftRight: f32, bloodFountainPressure: f32, yaw: f32, bloodFountainDirection: f32, bloodFountainPulse: f32, p8: f32, p9: f32) { invoke_ignore!(0xFFD54D9FE71B966A, ped, p1, boneId, moveWoundLeftRight, bloodFountainPressure, yaw, bloodFountainDirection, bloodFountainPulse, p8, p9) }
	pub fn _UPDATE_PED_WOUND_EFFECT(ped: Ped, value: f32) { invoke_ignore!(0x66B1CB778D911F49, ped, value) }
	pub fn APPLY_PED_DAMAGE_PACK(ped: Ped, damagePack: & CStr, damage: f32, mult: f32) { invoke_ignore!(0x46DF918788CB093F, ped, damagePack, damage, mult) }
	pub fn CLEAR_PED_BLOOD_DAMAGE(ped: Ped) { invoke_ignore!(0x8FE22675A5A45817, ped) }
	pub fn CLEAR_PED_BLOOD_DAMAGE_BY_ZONE(ped: Ped, p1: i32) { invoke_ignore!(0x56E3B78C5408D9F4, ped, p1) }
	pub fn CLEAR_PED_DAMAGE_DECAL_BY_ZONE(ped: Ped, p1: i32, p2: & CStr) { invoke_ignore!(0x523C79AEEFCC4A2A, ped, p1, p2) }
	pub fn _CLEAR_PED_BLOOD_DAMAGE_FACIAL(ped: Ped, p1: i32) { invoke_ignore!(0x7F5D88333EE8A86F, ped, p1) }
	pub fn _0x34C11114887150FD(p0: Any, p1: Any) { invoke_ignore!(0x34C11114887150FD, p0, p1) }
	pub fn _0xD8544F6260F5F01E(ped: Ped, p1: i32) { invoke_ignore!(0xD8544F6260F5F01E, ped, p1) }
	pub fn _0xEB8886E1065654CD(ped: Ped, p1: i32, p2: & CStr, p3: f32) { invoke_ignore!(0xEB8886E1065654CD, ped, p1, p2, p3) }
	pub fn FADE_AND_DESTROY_PED(ped: &mut Ped) { invoke_ignore!(0x7043D0681285BA2D, ped) }
	pub fn _IS_PED_QUEUED_FOR_DELETION(ped: Ped) -> bool { invoke!(0x8D9BFCE3352DE47F, ped) }
	pub fn CLEAR_PED_WETNESS(ped: Ped) { invoke_ignore!(0x9C720776DAA43E7E, ped) }
	pub fn SET_PED_WETNESS_HEIGHT(ped: Ped, height: f32) { invoke_ignore!(0x44CB6447D2571AA0, ped, height) }
	pub fn _0xF9CFF5BB70E8A2CB(ped: Ped, p1: f32) { invoke_ignore!(0xF9CFF5BB70E8A2CB, ped, p1) }
	pub fn SET_PED_WETNESS_ENABLED_THIS_FRAME(ped: Ped) { invoke_ignore!(0xB5485E4907B53019, ped) }
	pub fn _0xA7A806677F8DE138(ped: Ped) { invoke_ignore!(0xA7A806677F8DE138, ped) }
	pub fn _0xA064BBABB064446F(p0: Any) { invoke_ignore!(0xA064BBABB064446F, p0) }
	pub fn CLEAR_PED_ENV_DIRT(ped: Ped) { invoke_ignore!(0x6585D955A68452A5, ped) }
	pub fn SET_PED_SWEAT(ped: Ped, sweat: f32) { invoke_ignore!(0x27B0405F59637D1F, ped, sweat) }
	pub fn CLEAR_PED_DECORATIONS(ped: Ped) { invoke_ignore!(0x0E5173C163976E38, ped) }
	pub fn WAS_PED_SKELETON_UPDATED(ped: Ped) -> bool { invoke!(0x11B499C1E0FF8559, ped) }
	pub fn GET_PED_BONE_COORDS(ped: Ped, boneId: i32, offsetX: f32, offsetY: f32, offsetZ: f32) -> Vector3 { invoke!(0x17C07FC640E86B4E, ped, boneId, offsetX, offsetY, offsetZ) }
	pub fn ADD_SCENARIO_BLOCKING_AREA(x1: f32, y1: f32, z1: f32, x2: f32, y2: f32, z2: f32, p6: bool, blockingFlags: i32) -> i32 { invoke!(0x1B5C85C612E5256E, x1, y1, z1, x2, y2, z2, p6, blockingFlags) }
	pub fn REMOVE_SCENARIO_BLOCKING_AREAS() { invoke_ignore!(0xD37401D78A929A49) }
	pub fn REMOVE_SCENARIO_BLOCKING_AREA(p0: Any, p1: bool) { invoke_ignore!(0x31D16B74C6E29D66, p0, p1) }
	pub fn _ADD_SCENARIO_BLOCKING_VOLUME(volume: Volume, p1: bool, flag: i32) -> Any { invoke!(0x4C39C95AE5DB1329, volume, p1, flag) }
	pub fn _0x6F46F8ACB44C4FC1(p0: Any) -> Any { invoke!(0x6F46F8ACB44C4FC1, p0) }
	pub fn _IS_SCENARIO_BLOCKING_AREA_VALID(p0: Any) -> bool { invoke!(0x91A5F9CBEBB9D936, p0) }
	pub fn IS_PED_USING_SCENARIO_HASH(ped: Ped, scenarioHash: Hash) -> bool { invoke!(0x34D6AC1157C8226C, ped, scenarioHash) }
	pub fn IS_PED_USING_ANY_SCENARIO(ped: Ped) -> bool { invoke!(0x57AB4A3080F85143, ped) }
	pub fn IS_PED_USING_THIS_SCENARIO(ped: Ped, scenario: i32) -> bool { invoke!(0x9C54041BB66BCF9E, ped, scenario) }
	pub fn _CAN_PED_USE_SCENARIO_POINT(ped: Ped, scenario: i32, p2: Any, p3: Any, p4: Any) -> bool { invoke!(0xAB643407D0B26F07, ped, scenario, p2, p3, p4) }
	pub fn _0x1148F706CF4EBDDA(ped: Ped, p1: Hash, p2: i32) -> bool { invoke!(0x1148F706CF4EBDDA, ped, p1, p2) }
	pub fn SET_PED_PANIC_EXIT_SCENARIO(ped: Ped, x: f32, y: f32, z: f32) -> bool { invoke!(0xFE07FF6495D52E2A, ped, x, y, z) }
	pub fn TOGGLE_SCENARIO_PED_COWER_IN_PLACE(ped: Ped, toggle: bool) { invoke_ignore!(0x9A77DFD295E29B09, ped, toggle) }
	pub fn _0xD8CEEED54C672B5D(p0: Any, p1: Any, p2: Any, p3: Any, p4: Any, p5: Any, p6: Any) { invoke_ignore!(0xD8CEEED54C672B5D, p0, p1, p2, p3, p4, p5, p6) }
	pub fn SET_PED_SHOULD_PLAY_DIRECTED_NORMAL_SCENARIO_EXIT(ped: Ped, x: f32, y: f32, z: f32) -> bool { invoke!(0xEC6935EBE0847B90, ped, x, y, z) }
	pub fn SET_PED_SHOULD_PLAY_NORMAL_SCENARIO_EXIT(ped: Ped) { invoke_ignore!(0xA3A9299C4F2ADB98, ped) }
	pub fn SET_PED_SHOULD_PLAY_IMMEDIATE_SCENARIO_EXIT(ped: Ped) { invoke_ignore!(0xF1C03A5352243A30, ped) }
	pub fn SET_PED_SHOULD_PLAY_FLEE_SCENARIO_EXIT(ped: Ped, x: f32, y: f32, z: f32, lookIntensity: i32) -> bool { invoke!(0xEEED8FAFEC331A70, ped, x, y, z, lookIntensity) }
	pub fn SET_PED_SHOULD_PLAY_COMBAT_SCENARIO_EXIT(ped: Ped, x: f32, y: f32, z: f32, lookIntensity: i32) -> bool { invoke!(0x802092B07E3B1EEA, ped, x, y, z, lookIntensity) }
	pub fn SET_PED_SHOULD_PLAY_EMOTIONAL_SCENARIO_EXIT(ped: Ped, x: f32, y: f32, z: f32, lookIntensity: i32, p5: bool) -> bool { invoke!(0x62FDAD5E01D2DD47, ped, x, y, z, lookIntensity, p5) }
	pub fn SET_PED_SHOULD_PLAY_QUICK_SCENARIO_EXIT(ped: Ped, x: f32, y: f32, z: f32, lookIntensity: i32, p5: bool) -> bool { invoke!(0x463803429297117C, ped, x, y, z, lookIntensity, p5) }
	pub fn _0xF9331B3A314EB49D(ped: Ped) -> bool { invoke!(0xF9331B3A314EB49D, ped) }
	pub fn _0xE735A7DA22E88359(p0: Any) { invoke_ignore!(0xE735A7DA22E88359, p0) }
	pub fn _0x82CB0F3F0C7785E5(p0: Any) -> Any { invoke!(0x82CB0F3F0C7785E5, p0) }
	pub fn _0xCA95C156C14B2054(p0: Any, p1: Any) { invoke_ignore!(0xCA95C156C14B2054, p0, p1) }
	pub fn SET_FACIAL_IDLE_ANIM_OVERRIDE(ped: Ped, animName: & CStr, animDict: & CStr) { invoke_ignore!(0xFFC24B988B938B38, ped, animName, animDict) }
	pub fn CLEAR_FACIAL_IDLE_ANIM_OVERRIDE(ped: Ped) { invoke_ignore!(0x726256CC1EEB182F, ped) }
	pub fn _REQUEST_PED_FACIAL_MOOD_THIS_FRAME(ped: Ped, mood: Hash, p2: i32) { invoke_ignore!(0x8B3B71C80A29A4BB, ped, mood, p2) }
	pub fn _0xD2F0FE8805D91647(p0: Any, p1: Any) { invoke_ignore!(0xD2F0FE8805D91647, p0, p1) }
	pub fn SET_PED_CAN_PLAY_GESTURE_ANIMS(ped: Ped, p1: Any, p2: Any) { invoke_ignore!(0xBAF20C5432058024, ped, p1, p2) }
	pub fn _0x7EDB3C766B0D073F(ped: Ped) { invoke_ignore!(0x7EDB3C766B0D073F, ped) }
	pub fn SET_PED_CAN_PLAY_AMBIENT_ANIMS(ped: Ped, toggle: bool) { invoke_ignore!(0x6373D1349925A70E, ped, toggle) }
	pub fn SET_PED_CAN_PLAY_AMBIENT_BASE_ANIMS(ped: Ped, toggle: bool) { invoke_ignore!(0x0EB0585D15254740, ped, toggle) }
	pub fn _0x4F63433CE3C08230(ped: Ped, p1: bool) { invoke_ignore!(0x4F63433CE3C08230, ped, p1) }
	pub fn SET_PED_CAN_ARM_IK(ped: Ped, toggle: bool) { invoke_ignore!(0x6C3B4D6D13B4C841, ped, toggle) }
	pub fn _SET_PED_CAN_UNK_BODYPART_IK(ped: Ped, toggle: bool) { invoke_ignore!(0xEE9DF765990E8D1D, ped, toggle) }
	pub fn SET_PED_CAN_HEAD_IK(ped: Ped, toggle: bool) { invoke_ignore!(0xC11C18092C5530DC, ped, toggle) }
	pub fn SET_PED_CAN_LEG_IK(ped: Ped, toggle: bool) { invoke_ignore!(0x73518ECE2485412B, ped, toggle) }
	pub fn SET_PED_CAN_TORSO_IK(ped: Ped, toggle: bool) { invoke_ignore!(0xF2B7106D37947CE0, ped, toggle) }
	pub fn SET_PED_CAN_TORSO_REACT_IK(ped: Ped, toggle: bool) { invoke_ignore!(0xF5846EDB26A98A24, ped, toggle) }
	pub fn SET_PED_CAN_TORSO_VEHICLE_IK(ped: Ped, toggle: bool) { invoke_ignore!(0x6647C5F6F5792496, ped, toggle) }
	pub fn SET_PED_CAN_USE_AUTO_CONVERSATION_LOOKAT(ped: Ped, toggle: bool) { invoke_ignore!(0xEC4686EC06434678, ped, toggle) }
	pub fn IS_PED_HEADTRACKING_PED(ped1: Ped, ped2: Ped) -> bool { invoke!(0x5CD3CB88A7F8850D, ped1, ped2) }
	pub fn IS_PED_HEADTRACKING_ENTITY(ped: Ped, entity: Entity) -> bool { invoke!(0x813A0A7C9D2E831F, ped, entity) }
	pub fn _DISABLE_AMBIENT_LOOK_AT_REQUESTS(p0: Any, p1: Any) { invoke_ignore!(0x80038740C96AD17F, p0, p1) }
	pub fn _DISABLE_ALL_LOOK_AT_REQUESTS(ped: Ped, p1: i32) { invoke_ignore!(0xE1965A380342BE1F, ped, p1) }
	pub fn _0xCD9E5F94A2F38683(ped: Ped, p1: bool) { invoke_ignore!(0xCD9E5F94A2F38683, ped, p1) }
	pub fn SET_PED_CLOTH_PIN_FRAMES(ped: Ped, p1: bool) { invoke_ignore!(0x78C4E9961DB3EB5B, ped, p1) }
	pub fn _0x1D4636C90BBEFACB(ped: Ped, p1: i32) { invoke_ignore!(0x1D4636C90BBEFACB, ped, p1) }
	pub fn _0xEF371232BC6053E1(ped: Ped) { invoke_ignore!(0xEF371232BC6053E1, ped) }
	pub fn _0x86F0B6730C32AC14(ped: Ped, p1: bool) { invoke_ignore!(0x86F0B6730C32AC14, ped, p1) }
	pub fn _0x8101BA1C0B462412(ped: Ped, ropeId: i32) { invoke_ignore!(0x8101BA1C0B462412, ped, ropeId) }
	pub fn SET_PED_CONFIG_FLAG(ped: Ped, flagId: i32, value: bool) { invoke_ignore!(0x1913FE4CBF41C463, ped, flagId, value) }
	pub fn SET_PED_RESET_FLAG(ped: Ped, flagId: i32, doReset: bool) { invoke_ignore!(0xC1E8A365BF3B29F2, ped, flagId, doReset) }
	pub fn GET_PED_CONFIG_FLAG(ped: Ped, flagId: i32, p2: bool) -> bool { invoke!(0x7EE53118C892B513, ped, flagId, p2) }
	pub fn GET_PED_RESET_FLAG(ped: Ped, flagId: i32) -> bool { invoke!(0xAF9E59B1B1FBF2A0, ped, flagId) }
	pub fn _0xC6981AFF6D2A71C2(p0: Any) { invoke_ignore!(0xC6981AFF6D2A71C2, p0) }
	pub fn _0xE0FE107AB174D64A(p0: Any, p1: Any) { invoke_ignore!(0xE0FE107AB174D64A, p0, p1) }
	pub fn SET_PED_GROUP_MEMBER_PASSENGER_INDEX(ped: Ped, index: i32) { invoke_ignore!(0x0BDDB8D9EC6BCF3C, ped, index) }
	pub fn IS_PED_EVASIVE_DIVING(ped: Ped, evadingEntity: &mut Entity) -> bool { invoke!(0x414641C26E105898, ped, evadingEntity) }
	pub fn _SHOOT_TRIGGER_AT_COORDS(ped: Ped, x: f32, y: f32, z: f32, p4: i32, p5: f32, p6: i32, p7: f32) -> Any { invoke!(0x4C57F27D1554E6B0, ped, x, y, z, p4, p5, p6, p7) }
	pub fn _IS_THIS_MODEL_A_HORSE(model: Hash) -> bool { invoke!(0x772A1969F649E902, model) }
	pub fn SET_PED_MODEL_IS_SUPPRESSED(model: Hash, toggle: bool) { invoke_ignore!(0xE163A4BCE4DE6F11, model, toggle) }
	pub fn _IS_PED_MODEL_SUPPRESSED(model: Hash) -> bool { invoke!(0xAA9F048DCF69B6DC, model) }
	pub fn _0x7ABBD9E449E0DB00(ped: Ped, p1: bool) { invoke_ignore!(0x7ABBD9E449E0DB00, ped, p1) }
	pub fn _SET_PED_DISABLE_KICK_MOVE(ped: Ped, disable: bool) { invoke_ignore!(0xADD31A5C7A5FAA73, ped, disable) }
	pub fn SET_PED_CAN_RAGDOLL_FROM_PLAYER_IMPACT(ped: Ped, toggle: bool) { invoke_ignore!(0xDF993EE5E90ABA25, ped, toggle) }
	pub fn _0xE6CB36F43A95D75F(p0: Any) { invoke_ignore!(0xE6CB36F43A95D75F, p0) }
	pub fn SET_PED_LEG_IK_MODE(ped: Ped, mode: i32) { invoke_ignore!(0xC396F5B86FF9FEBD, ped, mode) }
	pub fn _IS_PED_IN_POINT(ped: Ped, x: f32, y: f32, z: f32, radius: f32, p5: bool) -> bool { invoke!(0x078076AB50FB117F, ped, x, y, z, radius, p5) }
	pub fn _SET_PED_CAN_BE_LASSOED(ped: Ped, toggle: bool) { invoke_ignore!(0xFD6943B6DF77E449, ped, toggle) }
	pub fn SET_PED_COMBAT_MOVEMENT(ped: Ped, combatMovement: i32) { invoke_ignore!(0x4D9CA1009AFBD057, ped, combatMovement) }
	pub fn GET_PED_COMBAT_MOVEMENT(ped: Ped) -> i32 { invoke!(0xDEA92412FCAEB3F5, ped) }
	pub fn _0x815C0074A1BC0D93(ped: Ped, p1: i32) { invoke_ignore!(0x815C0074A1BC0D93, ped, p1) }
	pub fn _0xFFDE295662405B25(ped: Ped) -> i32 { invoke!(0xFFDE295662405B25, ped) }
	pub fn SET_PED_COMBAT_ABILITY(ped: Ped, abilityLevel: i32) { invoke_ignore!(0xC7622C0D36B2FDA8, ped, abilityLevel) }
	pub fn SET_PED_COMBAT_RANGE(ped: Ped, range: i32) { invoke_ignore!(0x3C606747B23E497B, ped, range) }
	pub fn SET_PED_COMBAT_ATTRIBUTES(ped: Ped, attributeIndex: i32, enabled: bool) { invoke_ignore!(0x9F7794730795E019, ped, attributeIndex, enabled) }
	pub fn _GET_PED_COMBAT_ATTRIBUTE(ped: Ped, attributeIndex: i32) -> bool { invoke!(0xCC2B20596E29E4E3, ped, attributeIndex) }
	pub fn _SET_PED_COMBAT_ATTRIBUTE_HASH(ped: Ped, p1: Hash) { invoke_ignore!(0xBD75500141E4725C, ped, p1) }
	pub fn SET_PED_TARGET_LOSS_RESPONSE(ped: Ped, responseType: i32) { invoke_ignore!(0x0703B9079823DA4A, ped, responseType) }
	pub fn _0x0A4618FFD517E24D(p0: Any, p1: Any) { invoke_ignore!(0x0A4618FFD517E24D, p0, p1) }
	pub fn _0x712B2C2B2471B493(ped: Ped, p1: Hash) { invoke_ignore!(0x712B2C2B2471B493, ped, p1) }
	pub fn _0x00B380FF2DF6AB7A(p0: Any, p1: Any) { invoke_ignore!(0x00B380FF2DF6AB7A, p0, p1) }
	pub fn _SET_PED_COMBAT_STYLE(ped: Ped, combatStyleHash: Hash, p2: i32, duration: f32) { invoke_ignore!(0x8ACC0506743A8A5C, ped, combatStyleHash, p2, duration) }
	pub fn _CLEAR_PED_COMBAT_STYLE(ped: Ped, p1: i32) { invoke_ignore!(0x78815FC52832B690, ped, p1) }
	pub fn _SET_PED_COMBAT_STYLE_MOD(ped: Ped, combatStyleModHash: Hash, duration: f32) { invoke_ignore!(0x8B1E8E35A6E814EA, ped, combatStyleModHash, duration) }
	pub fn _CLEAR_PED_COMBAT_STYLE_MOD(ped: Ped, combatStyleModHash: Hash) { invoke_ignore!(0x1FA132CBCD7CB239, ped, combatStyleModHash) }
	pub fn _0x5BF0B9D9A8E227A0(ped: Ped) -> bool { invoke!(0x5BF0B9D9A8E227A0, ped) }
	pub fn _0x642720D8D69328B6(ped: Ped, p1: Hash) { invoke_ignore!(0x642720D8D69328B6, ped, p1) }
	pub fn IS_PED_PERFORMING_MELEE_ACTION(ped: Ped, p1: i32, p2: Hash) -> bool { invoke!(0xDCCA191DF9980FD7, ped, p1, p2) }
	pub fn _0x99DF2639DA76C1DC(ped1: Ped, ped2: Ped, p2: i32) -> bool { invoke!(0x99DF2639DA76C1DC, ped1, ped2, p2) }
	pub fn _GET_PED_MELEE_ACTION_PHASE(ped: Ped) -> f32 { invoke!(0x6127F25ED21C533C, ped) }
	pub fn IS_PED_BEING_STEALTH_KILLED(ped: Ped) -> bool { invoke!(0x863B23EFDE9C5DF2, ped) }
	pub fn GET_MELEE_TARGET_FOR_PED(ped: Ped) -> Ped { invoke!(0x18A3E9EE1297FD39, ped) }
	pub fn _0xDEDBED3020DA49DC(p0: Any) { invoke_ignore!(0xDEDBED3020DA49DC, p0) }
	pub fn _0xA405BF9F01960C16(p0: Any) { invoke_ignore!(0xA405BF9F01960C16, p0) }
	pub fn _GET_PED_BRAWLING_STYLE(ped: Ped) -> Hash { invoke!(0xEC6B59BE445FEC51, ped) }
	pub fn _SET_PED_BRAWLING_STYLE(ped: Ped, brawlingStyle: Hash) { invoke_ignore!(0x8BA83CC4288CD56D, ped, brawlingStyle) }
	pub fn _0x9D8DFE2DE9CB4DFC(ped: Ped) { invoke_ignore!(0x9D8DFE2DE9CB4DFC, ped) }
	pub fn _0xC48AF420371C7407(ped: Ped, grapple: Hash) -> Any { invoke!(0xC48AF420371C7407, ped, grapple) }
	pub fn _0x5EFA8A3D8A60D662(p0: Any, p1: Any) -> Any { invoke!(0x5EFA8A3D8A60D662, p0, p1) }
	pub fn _0x242EDF85D4E87B65(p0: Any) -> Any { invoke!(0x242EDF85D4E87B65, p0) }
	pub fn _SET_PED_COMBAT_BEHAVIOUR(ped: Ped, behaviour: Hash) { invoke_ignore!(0x9238A3D970BBB0A9, ped, behaviour) }
	pub fn _0x9A4AC116CC1EEE14(p0: Any) { invoke_ignore!(0x9A4AC116CC1EEE14, p0) }
	pub fn _0xE20027B414BFE6C7(p0: Any, p1: Any) { invoke_ignore!(0xE20027B414BFE6C7, p0, p1) }
	pub fn _SET_PED_BEAT_MULTIPLIER(ped: Ped, p1: f32) { invoke_ignore!(0x6DBF2D78709AD70B, ped, p1) }
	pub fn _SET_PED_ACTION_DISABLE_FLAG(ped: Ped, actionDisableFlag: i32) { invoke_ignore!(0xB8DE69D9473B7593, ped, actionDisableFlag) }
	pub fn _CLEAR_PED_ACTION_DISABLE_FLAG(ped: Ped, actionDisableFlag: i32) { invoke_ignore!(0x949B2F9ED2917F5D, ped, actionDisableFlag) }
	pub fn _IS_PED_ACTION_DISABLE_FLAG_ENABLED(ped: Ped, actionDisableFlag: i32) -> bool { invoke!(0xB346C85D49CC998E, ped, actionDisableFlag) }
	pub fn _SET_PED_TARGET_ACTION_DISABLE_FLAG(ped: Ped, actionDisableFlag: i32) { invoke_ignore!(0xC163DAC52AC975D3, ped, actionDisableFlag) }
	pub fn _CLEAR_PED_TARGET_ACTION_DISABLE_FLAG(ped: Ped, actionDisableFlag: i32) { invoke_ignore!(0xBBF6D1D07C02D00A, ped, actionDisableFlag) }
	pub fn _IS_PED_TARGET_ACTION_DISABLE_FLAG_ENABLED(ped: Ped, actionDisableFlag: i32) -> bool { invoke!(0x02AA2096FE00F3E1, ped, actionDisableFlag) }
	pub fn _0x57F35552E771BE9D(ped: Ped, p1: i32) { invoke_ignore!(0x57F35552E771BE9D, ped, p1) }
	pub fn _0x7C10221CE718AA72(ped: Ped, p1: i32) { invoke_ignore!(0x7C10221CE718AA72, ped, p1) }
	pub fn _0x0D3B1568917EBDA0(ped: Ped, p1: i32) -> bool { invoke!(0x0D3B1568917EBDA0, ped, p1) }
	pub fn _0x29F3539189D3E277(p0: Any, p1: Any) { invoke_ignore!(0x29F3539189D3E277, p0, p1) }
	pub fn _0xE9E06EA514A69061(p0: Any, p1: Any) { invoke_ignore!(0xE9E06EA514A69061, p0, p1) }
	pub fn _SET_PED_WRITHING_DURATION(ped: Ped, writhingDuration1: f32, writhingDuration2: f32, p3: i32) { invoke_ignore!(0x4DB9D03AC4E1FA84, ped, writhingDuration1, writhingDuration2, p3) }
	pub fn _GET_PED_REMAINING_REVIVAL_TIME(ped: Ped, normalized: bool) -> f32 { invoke!(0xEBE89623EB861271, ped, normalized) }
	pub fn SET_PAUSE_PED_WRITHE_BLEEDOUT(ped: Ped, toggle: bool) { invoke_ignore!(0x925A160133003AC6, ped, toggle) }
	pub fn _0x12EB4E31F092C9B3(ped: Ped) -> bool { invoke!(0x12EB4E31F092C9B3, ped) }
	pub fn GET_PED_IS_GRAPPLING(ped: Ped) -> bool { invoke!(0x0E99E3BF11BB6367, ped) }
	pub fn GET_PED_IS_BEING_GRAPPLED(ped: Ped) -> bool { invoke!(0x3BDFCF25B58B0415, ped) }
	pub fn _GET_PED_GRAPPLER(ped: Ped) -> Ped { invoke!(0xD0B7AEB56229D317, ped) }
	pub fn GET_PED_GRAPPLE_STATE(ped: Ped) -> i32 { invoke!(0x2311F15D971AA680, ped) }
	pub fn _GET_PED_GRAPPLE_STYLE(ped: Ped) -> Hash { invoke!(0x753B15AD0FD6F3E3, ped) }
	pub fn _SET_PED_GRAPPLE_STYLE(ped: Ped, style: Hash) -> Any { invoke!(0x630E7B01F091A197, ped, style) }
	pub fn _SET_PED_GRAPPLE_SEQUENCE(ped: Ped, grappleSequence: & CStr) { invoke_ignore!(0x604190F0CF0DF158, ped, grappleSequence) }
	pub fn _SET_PED_GRAPPLE_FLAG(ped: Ped, flag: i32, enable: bool) { invoke_ignore!(0x789DABD18E9024DB, ped, flag, enable) }
	pub fn _CLEAR_PED_GRAPPLE_FLAG(ped: Ped, flag: i32) { invoke_ignore!(0xEAE3B5B019C8D23F, ped, flag) }
	pub fn _GET_PED_GRAPPLE_FLAG(ped: Ped) -> i32 { invoke!(0xF3C873ED0C595109, ped) }
	pub fn _SET_PED_GRAPPLE_ACTION(ped: Ped, grappleAction: Hash) { invoke_ignore!(0x8301D87B1B89E219, ped, grappleAction) }
	pub fn _SET_PED_GRAPPLE_EFFECT_MULTIPLIER(ped: Ped, multiplier: f32) -> Any { invoke!(0x99A6E246C315BF60, ped, multiplier) }
	pub fn _SET_PED_GRAPPLE_ANIMATION(ped: Ped, grappleAnim: Hash) { invoke_ignore!(0x56E9C26CD29D1ED6, ped, grappleAnim) }
	pub fn SET_PED_FLEE_ATTRIBUTES(ped: Ped, attributeFlags: i32, enable: bool) { invoke_ignore!(0x70A2D1137C8ED7C9, ped, attributeFlags, enable) }
	pub fn _IS_PED_COWERING(ped: Ped) -> bool { invoke!(0xB086C8C0F5701D14, ped) }
	pub fn IS_ANY_PED_NEAR_POINT(x: f32, y: f32, z: f32, radius: f32) -> bool { invoke!(0x083961498679DC9F, x, y, z, radius) }
	pub fn FORCE_PED_AI_AND_ANIMATION_UPDATE(ped: Ped, p1: bool, p2: bool) { invoke_ignore!(0x2208438012482A1A, ped, p1, p2) }
	pub fn _0xC2722B252C79E641(ped: Ped, p1: Any, p2: Any, p3: bool) { invoke_ignore!(0xC2722B252C79E641, ped, p1, p2, p3) }
	pub fn _IS_PED_DOING_SCENARIO_TRANSITION(ped: Ped) -> bool { invoke!(0xC488B8C0E52560D8, ped) }
	pub fn _0x2DC0E8DCBD3546E9(ped: Ped) -> bool { invoke!(0x2DC0E8DCBD3546E9, ped) }
	pub fn _ADD_SCENARIO_TRANSITION(ped: Ped) { invoke_ignore!(0x6D07B371E9439019, ped) }
	pub fn _GIVE_PED_HASH_COMMAND(ped: Ped, commandHash: Hash, activationDuration: f32) { invoke_ignore!(0xD65FDC686A031C83, ped, commandHash, activationDuration) }
	pub fn _GET_IS_PED_COMMAND_HASH_PRESENT(ped: Ped, commandHash: Hash) -> bool { invoke!(0x68821369A2CEADD5, ped, commandHash) }
	pub fn IS_PED_HEADING_TOWARDS_POSITION(ped: Ped, x: f32, y: f32, z: f32, p4: f32) -> bool { invoke!(0xFCF37A457CB96DC0, ped, x, y, z, p4) }
	pub fn _0x600BBDD29820370C(ped: Ped) { invoke_ignore!(0x600BBDD29820370C, ped) }
	pub fn REQUEST_PED_VISIBILITY_TRACKING(ped: Ped) { invoke_ignore!(0x7D7A2E43E74E2EB8, ped) }
	pub fn RELEASE_PED_VISIBILITY_TRACKING(ped: Ped) { invoke_ignore!(0x3088634CF8C819CF, ped) }
	pub fn REQUEST_PED_VEHICLE_VISIBILITY_TRACKING(ped: Ped, p1: bool) { invoke_ignore!(0x2BC338A7B21F4608, ped, p1) }
	pub fn REQUEST_PED_USE_SMALL_BBOX_VISIBILITY_TRACKING(ped: Ped, p1: bool) { invoke_ignore!(0x75BA1CB3B7D40CAF, ped, p1) }
	pub fn GET_TRACKED_PED_PIXELCOUNT(ped: Ped) -> i32 { invoke!(0x511F1A683387C7E2, ped) }
	pub fn IS_TRACKED_PED_VISIBLE(ped: Ped) -> bool { invoke!(0x91C8E617F64188AC, ped) }
	pub fn _IS_TRACKED_PED_VISIBILITY_PERCENTAGE_NOT_LESS_THAN(ped: Ped, percent: f32) -> bool { invoke!(0x164CECC59E70DF86, ped, percent) }
	pub fn _IS_PED_VISIBILITY_TRACKED(ped: Ped) -> bool { invoke!(0x5102307CE88798EB, ped) }
	pub fn CAN_PED_BE_MOUNTED(ped: Ped) -> bool { invoke!(0x2D64376CF437363E, ped) }
	pub fn _IS_META_PED_FISH(ped: Ped) -> bool { invoke!(0x118D476A6F1A13F1, ped) }
	pub fn IS_EVENT_IN_QUEUE(ped: Ped, eventType: Hash) -> bool { invoke!(0xC8D523BF5BBD3808, ped, eventType) }
	pub fn CAN_PED_SEE_ENTITY(ped: Ped, targetEntity: Entity, p2: bool, p3: bool) -> i32 { invoke!(0x7F9B9791D4CB71F6, ped, targetEntity, p2, p3) }
	pub fn CAN_PED_SEE_PED_CACHED(ped: Ped, targetPed: Ped, p2: bool) -> i32 { invoke!(0x9D9473CB82D83A30, ped, targetPed, p2) }
	pub fn _0x0EA9EACBA3B01601(ped1: Ped, ped2: Ped, p2: bool) -> f32 { invoke!(0x0EA9EACBA3B01601, ped1, ped2, p2) }
	pub fn GET_PED_BONE_INDEX(ped: Ped, boneId: i32) -> i32 { invoke!(0x3F428D08BE5AAE31, ped, boneId) }
	pub fn _0xC5303F460A40D21D(ped: Ped, p1: i32) -> i32 { invoke!(0xC5303F460A40D21D, ped, p1) }
	pub fn _0xE29D8CD66553DBAA(horse: Ped) { invoke_ignore!(0xE29D8CD66553DBAA, horse) }
	pub fn _0xB06F5F1DEF417216(p0: Any, p1: Any, p2: Any, p3: Any) { invoke_ignore!(0xB06F5F1DEF417216, p0, p1, p2, p3) }
	pub fn _0xFC23348F0F4E245F(p0: Any, p1: Any, p2: Any, p3: Any) { invoke_ignore!(0xFC23348F0F4E245F, p0, p1, p2, p3) }
	pub fn _0x9184788BFF1EDAD7(p0: Any, p1: Any) { invoke_ignore!(0x9184788BFF1EDAD7, p0, p1) }
	pub fn _SET_PED_DIRT_CLEANED(ped: Ped, p1: f32, p2: i32, p3: bool, p4: bool) { invoke_ignore!(0xE3144B932DFDFF65, ped, p1, p2, p3, p4) }
	pub fn _0x0105FEE8F9091255(p0: Any, p1: Any) -> Any { invoke!(0x0105FEE8F9091255, p0, p1) }
	pub fn _0xD049920CD29F6CC8(p0: Any, p1: Any, p2: Any, p3: Any, p4: Any) { invoke_ignore!(0xD049920CD29F6CC8, p0, p1, p2, p3, p4) }
	pub fn _0xBB3E5370EBB6BE28(p0: Any, p1: Any) -> Any { invoke!(0xBB3E5370EBB6BE28, p0, p1) }
	pub fn _0x2FA568BFA725F8D6(p0: Any, p1: Any, p2: Any, p3: Any) { invoke_ignore!(0x2FA568BFA725F8D6, p0, p1, p2, p3) }
	pub fn _0x56E4BAD93D33453C(p0: Any, p1: Any) -> Any { invoke!(0x56E4BAD93D33453C, p0, p1) }
	pub fn _0x75A082563B4452E5(p0: Any, p1: Any, p2: Any, p3: Any) { invoke_ignore!(0x75A082563B4452E5, p0, p1, p2, p3) }
	pub fn _0x16802C32B2FCA06B(p0: Any, p1: Any, p2: Any, p3: Any) { invoke_ignore!(0x16802C32B2FCA06B, p0, p1, p2, p3) }
	pub fn _0x8BA0C65AC15A7D33(p0: Any, p1: Any, p2: Any, p3: Any) { invoke_ignore!(0x8BA0C65AC15A7D33, p0, p1, p2, p3) }
	pub fn _0x0FFDF937E5C11382(p0: Any, p1: Any, p2: Any, p3: Any, p4: Any, p5: Any) { invoke_ignore!(0x0FFDF937E5C11382, p0, p1, p2, p3, p4, p5) }
	pub fn _0x5FCF25D584065BFD(p0: Any, p1: Any, p2: Any, p3: Any) { invoke_ignore!(0x5FCF25D584065BFD, p0, p1, p2, p3) }
	pub fn _0xA2116C1E4ED85C24(ped: Ped, inverted: bool) { invoke_ignore!(0xA2116C1E4ED85C24, ped, inverted) }
	pub fn FORCE_PED_MOTION_STATE(ped: Ped, motionStateHash: Hash, p2: bool, p3: i32, p4: bool) -> bool { invoke!(0xF28965D04F570DCA, ped, motionStateHash, p2, p3, p4) }
	pub fn _0x75D3333409CD33CE(p0: Any, p1: Any, p2: Any) { invoke_ignore!(0x75D3333409CD33CE, p0, p1, p2) }
	pub fn GET_PED_CURRENT_MOVE_BLEND_RATIO(ped: Ped, speedX: &mut f32, speedY: &mut f32) -> bool { invoke!(0xF60165E1D2C5370B, ped, speedX, speedY) }
	pub fn SET_PED_MAX_MOVE_BLEND_RATIO(ped: Ped, value: f32) { invoke_ignore!(0x433083750C5E064A, ped, value) }
	pub fn SET_PED_MIN_MOVE_BLEND_RATIO(ped: Ped, value: f32) { invoke_ignore!(0x01A898D26E2333DD, ped, value) }
	pub fn _0xBC1DC48270468444(p0: Any) { invoke_ignore!(0xBC1DC48270468444, p0) }
	pub fn _0x46BF2A810679D6E6(ped: Ped, maxMoveBlendRatio: f32) -> f32 { invoke!(0x46BF2A810679D6E6, ped, maxMoveBlendRatio) }
	pub fn _0xCA95924C893A0C91(ped: Ped, p1: f32) -> f32 { invoke!(0xCA95924C893A0C91, ped, p1) }
	pub fn _HORSE_AGITATE(mount: Ped, kickOffRider: bool) { invoke_ignore!(0xBAE08F00021BFFB2, mount, kickOffRider) }
	pub fn _0x413697EC260AABBF(p0: Any, p1: Any, p2: Any) { invoke_ignore!(0x413697EC260AABBF, p0, p1, p2) }
	pub fn _0xCAC43D060099EA72(ped: Ped) { invoke_ignore!(0xCAC43D060099EA72, ped) }
	pub fn _0xC9151483CC06A414(ped: Ped) { invoke_ignore!(0xC9151483CC06A414, ped) }
	pub fn _0xAD3330E3C3E98007(p0: Any, p1: Any) { invoke_ignore!(0xAD3330E3C3E98007, p0, p1) }
	pub fn _0xB8E2D655E1D5BD39(p0: Any) -> Any { invoke!(0xB8E2D655E1D5BD39, p0) }
	pub fn _0x7BB810E8B343AC7B(p0: Any) -> Any { invoke!(0x7BB810E8B343AC7B, p0) }
	pub fn SET_PED_MOVE_RATE_OVERRIDE(ped: Ped, value: f32) { invoke_ignore!(0x085BF80FA50A39D1, ped, value) }
	pub fn GET_PED_NEARBY_VEHICLES(ped: Ped, sizeAndVehs: &mut Any) -> i32 { invoke!(0xCFF869CBFA210D82, ped, sizeAndVehs) }
	pub fn GET_PED_NEARBY_PEDS(ped: Ped, sizeAndPeds: &mut Any, ignoredPedType: i32, p3: i32) -> i32 { invoke!(0x23F8F5FC7E8C4A6B, ped, sizeAndPeds, ignoredPedType, p3) }
	pub fn IS_PED_READY_TO_RENDER(ped: Ped) -> bool { invoke!(0xA0BC8FAED8CFEB3C, ped) }
	pub fn _0x6A489892E813951A(p0: Any) { invoke_ignore!(0x6A489892E813951A, p0) }
	pub fn IS_PED_USING_ACTION_MODE(ped: Ped) -> bool { invoke!(0x00E73468D085F745, ped) }
	pub fn _IS_PED_USING_ACTION_MODE_2(ped: Ped) -> bool { invoke!(0xEBB208D6AE712C03, ped) }
	pub fn SET_PED_USING_ACTION_MODE(ped: Ped, bActionModeEnabled: bool, p2: i32, action: & CStr) { invoke_ignore!(0xD75ACCF5E0FB5367, ped, bActionModeEnabled, p2, action) }
	pub fn SET_PED_CAPSULE(ped: Ped, value: f32) { invoke_ignore!(0x364DF566EC833DE2, ped, value) }
	pub fn _GET_RIDER_OF_MOUNT(mount: Ped, p1: bool) -> Ped { invoke!(0xB676EFDA03DADA52, mount, p1) }
	pub fn SPAWNPOINTS_START_SEARCH(x: f32, y: f32, z: f32, width: f32, p4: f32, spawnpointsFlag: i32, p6: f32, duration: i32, p8: f32) { invoke_ignore!(0x2DF9038C90AD5264, x, y, z, width, p4, spawnpointsFlag, p6, duration, p8) }
	pub fn SPAWNPOINTS_START_SEARCH_IN_ANGLED_AREA(x1: f32, y1: f32, z1: f32, x2: f32, y2: f32, z2: f32, width: f32, spawnpointsFlag: i32, p8: f32, duration: i32, p10: f32) { invoke_ignore!(0xB2AFF10216DEFA2F, x1, y1, z1, x2, y2, z2, width, spawnpointsFlag, p8, duration, p10) }
	pub fn _SPAWNPOINTS_START_SEARCH_WITH_VOLUME(volume: Volume, spawnpointsFlag: i32, p2: f32, duration: i32, p4: f32) { invoke_ignore!(0x83ED1FC9DF3411F5, volume, spawnpointsFlag, p2, duration, p4) }
	pub fn SPAWNPOINTS_CANCEL_SEARCH() { invoke_ignore!(0xFEE4A5459472A9F8) }
	pub fn SPAWNPOINTS_IS_SEARCH_ACTIVE() -> bool { invoke!(0x3C67506996001F5E) }
	pub fn SPAWNPOINTS_IS_SEARCH_COMPLETE() -> bool { invoke!(0xA586FBEB32A53DBB) }
	pub fn SPAWNPOINTS_IS_SEARCH_FAILED() -> bool { invoke!(0xF445DE8DA80A1792) }
	pub fn SPAWNPOINTS_GET_NUM_SEARCH_RESULTS() -> i32 { invoke!(0xA635C11B8C44AFC2) }
	pub fn SPAWNPOINTS_GET_SEARCH_RESULT(randomInt: i32, x: &mut f32, y: &mut Any, z: &mut f32) { invoke_ignore!(0x280C7E3AC7F56E90, randomInt, x, y, z) }
	pub fn SPAWNPOINTS_GET_SEARCH_RESULT_FLAGS(p0: Any, p1: &mut Any) { invoke_ignore!(0xB782F8238512BAD5, p0, p1) }
	pub fn SET_IK_TARGET(ped: Ped, ikIndex: i32, entityLookAt: Entity, boneLookAt: i32, offsetX: f32, offsetY: f32, offsetZ: f32, p7: Any, blendInDuration: i32, blendOutDuration: i32) { invoke_ignore!(0xC32779C16FCEECD9, ped, ikIndex, entityLookAt, boneLookAt, offsetX, offsetY, offsetZ, p7, blendInDuration, blendOutDuration) }
	pub fn _REQUEST_PED_EMOTIONAL_PRESET(ped: Ped, name: & CStr) { invoke_ignore!(0x5C3C55EAAD19915F, ped, name) }
	pub fn _HAS_PED_EMOTIONAL_PRESET_LOADED(ped: Ped, name: & CStr) -> bool { invoke!(0xDE3904B22695D9F9, ped, name) }
	pub fn _REMOVE_PED_EMOTIONAL_PRESET(ped: Ped, name: & CStr) { invoke_ignore!(0xFC3BAB1801A8255A, ped, name) }
	pub fn _REQUEST_MOTION_TYPE_ASSET(nameHash: Hash, ped: Ped) { invoke_ignore!(0xF7EA250B9A919E03, nameHash, ped) }
	pub fn HAS_MOTION_TYPE_ASSET_LOADED(nameHash: Hash, ped: Ped) -> bool { invoke!(0x854BC9B1A1CCD034, nameHash, ped) }
	pub fn _REMOVE_MOTION_TYPE_ASSET(nameHash: Hash, ped: Ped) { invoke_ignore!(0xDE7B2B4144906CDF, nameHash, ped) }
	pub fn _0x290B2E6CCDE532E1(ped: Ped) -> bool { invoke!(0x290B2E6CCDE532E1, ped) }
	pub fn _0x0EEF7A81C17679DB(ped: Ped) -> bool { invoke!(0x0EEF7A81C17679DB, ped) }
	pub fn SET_PED_LOD_MULTIPLIER(ped: Ped, multiplier: f32) { invoke_ignore!(0xDC2C5C242AAC342B, ped, multiplier) }
	pub fn _GET_PED_LOD_MULTIPLIER(ped: Ped) -> f32 { invoke!(0x1B710E6F4AB69341, ped) }
	pub fn _0xA218D2BBCAA7388C(p0: Any, p1: Any) -> Any { invoke!(0xA218D2BBCAA7388C, p0, p1) }
	pub fn IS_ANY_HOSTILE_PED_NEAR_POINT(ped: Ped, x: f32, y: f32, z: f32, radius: f32) -> bool { invoke!(0x68772DB2B2526F9F, ped, x, y, z, radius) }
	pub fn _0xCBDE59C48F2B06F5(p0: Any, p1: Any, p2: Any) { invoke_ignore!(0xCBDE59C48F2B06F5, p0, p1, p2) }
	pub fn _0x6A190B94C2541A99(p0: Any) { invoke_ignore!(0x6A190B94C2541A99, p0) }
	pub fn IS_TARGET_PED_IN_PERCEPTION_AREA(ped: Ped, targetPed: Ped, p2: f32, customDistance: f32, p4: f32, p5: f32) -> bool { invoke!(0x06087579E7AA85A9, ped, targetPed, p2, customDistance, p4, p5) }
	pub fn SET_POP_CONTROL_SPHERE_THIS_FRAME(p0: Any, p1: Any, p2: Any, p3: Any, p4: Any) { invoke_ignore!(0xD8C3BE3EE94CAF2D, p0, p1, p2, p3, p4) }
	pub fn IS_PED_HOGTIED(ped: Ped) -> bool { invoke!(0x3AA24CCC0D451379, ped) }
	pub fn IS_PED_BEING_HOGTIED(ped: Ped) -> bool { invoke!(0xD453BB601D4A606E, ped) }
	pub fn _0x3D9F958834AB9C30(ped: Ped) -> Ped { invoke!(0x3D9F958834AB9C30, ped) }
	pub fn _0x913D04A5176F84C9(ped: Ped) -> bool { invoke!(0x913D04A5176F84C9, ped) }
	pub fn IS_PED_HOGTYING(ped: Ped) -> bool { invoke!(0x42429C674B61238B, ped) }
	pub fn _GET_PED_LASSO_HOGTIE_FLAG(ped: Ped, flagId: i32) -> bool { invoke!(0x2C76FA0E01681F8D, ped, flagId) }
	pub fn SET_PED_LASSO_HOGTIE_FLAG(ped: Ped, flagId: i32, value: bool) { invoke_ignore!(0xAE6004120C18DF97, ped, flagId, value) }
	pub fn _0x4642182A298187D0(ped: Ped, p1: i32, p2: &mut Any, p3: i32, p4: i32) -> i32 { invoke!(0x4642182A298187D0, ped, p1, p2, p3, p4) }
	pub fn _0x6B67320E0D57856A(ped: Ped, p1: &mut Any, p2: i32, p3: bool) { invoke_ignore!(0x6B67320E0D57856A, ped, p1, p2, p3) }
	pub fn _GET_FIRST_ENTITY_PED_IS_CARRYING(ped: Ped) -> Entity { invoke!(0xD806CD2A4F2C2996, ped) }
	pub fn _0xAA6C49AE90A32299(ped: Ped, p1: Hash) { invoke_ignore!(0xAA6C49AE90A32299, ped, p1) }
	pub fn _GET_CARRIER_AS_PED(entity: Entity) -> Ped { invoke!(0x09B83E68DE004CD4, entity) }
	pub fn _GET_CARRIER_AS_MOUNT(entity: Entity) -> Ped { invoke!(0xA033D7E4BBF9844D, entity) }
	pub fn _GET_CARRIER_AS_HUMAN(entity: Entity) -> Ped { invoke!(0x79443D56C8DF45EE, entity) }
	pub fn GET_CARRIED_ATTACHED_INFO_FOR_SLOT(p0: Any, p1: Any, p2: Any, p3: Any) -> Any { invoke!(0x608BC6A6AACD5036, p0, p1, p2, p3) }
	pub fn DETACH_CARRIABLE_ENTITY(entity: Entity, p1: bool, p2: bool) { invoke_ignore!(0xED00D72F81CF7278, entity, p1, p2) }
	pub fn FIND_ALL_ATTACHED_CARRIABLE_ENTITIES(ped: Ped, itemset: ItemSet) { invoke_ignore!(0xB5ACE8B23A438EC0, ped, itemset) }
	pub fn IS_PED_CARRYING_SOMETHING(ped: Ped) -> bool { invoke!(0xA911EE21EDF69DAF, ped) }
	pub fn _0xB65927F861E7AE39(ped: Ped, p1: i32) -> bool { invoke!(0xB65927F861E7AE39, ped, p1) }
	pub fn _0xA1FBAC56D38563E2(volume: Volume) -> bool { invoke!(0xA1FBAC56D38563E2, volume) }
	pub fn _0x6F43C351A5D51E2F(ped: Ped, p1: &mut Any) -> Any { invoke!(0x6F43C351A5D51E2F, ped, p1) }
	pub fn IS_PED_LASSOED(ped: Ped) -> bool { invoke!(0x9682F850056C9ADE, ped) }
	pub fn _GET_LASSOER_OF_PED(ped: Ped) -> Entity { invoke!(0x833F0053340EF413, ped) }
	pub fn _GET_LASSO_TARGET(ped: Ped) -> Entity { invoke!(0xB65A4DAB460A19BD, ped) }
	pub fn _GET_LASSOED_LASSOER(ped: Ped) -> Ped { invoke!(0x0C31C51168E80365, ped) }
	pub fn SET_LOOTING_FLAG(ped: Ped, lootFlag: i32, enabled: bool) { invoke_ignore!(0x6569F31A01B4C097, ped, lootFlag, enabled) }
	pub fn _GET_LOOTING_FLAG(ped: Ped, lootFlag: i32) -> bool { invoke!(0xE4C11F104620DDCE, ped, lootFlag) }
	pub fn _REFRESH_LOOT_STATE_FOR_PED(ped: Ped, p1: i32, lootTarget: &mut Ped, p3: i32, p4: i32) -> i32 { invoke!(0x5463C962BC7777C3, ped, p1, lootTarget, p3, p4) }
	pub fn GET_PED_LOOT_STATUS_MP(ped: Ped) -> i32 { invoke!(0xC737697C41628340, ped) }
	pub fn _0x4B19F171450E0D4F(ped: Ped) -> Ped { invoke!(0x4B19F171450E0D4F, ped) }
	pub fn _0x758F081DB204DDDE(ped: Ped) -> bool { invoke!(0x758F081DB204DDDE, ped) }
	pub fn GET_LOOTING_PICKUP_TARGET_ENTITY(ped: Ped) -> Entity { invoke!(0x14169FA823679E41, ped) }
	pub fn _0x7B5C293238EE4F20(p0: Any) -> Any { invoke!(0x7B5C293238EE4F20, p0) }
	pub fn _0xAE6B68A83ABBE7C0(p0: Any) { invoke_ignore!(0xAE6B68A83ABBE7C0, p0) }
	pub fn _0xA4B6432E3880F2F9(ped: Ped) -> bool { invoke!(0xA4B6432E3880F2F9, ped) }
	pub fn _ADD_PED_SUBSCRIBE_TO_LEGENDARY_BLIPS(ped: Ped) -> bool { invoke!(0xE37287EE358939C3, ped) }
	pub fn _REMOVE_PED_SUBSCRIBE_TO_LEGENDARY_BLIPS(ped: Ped) -> bool { invoke!(0x011A42FD923D41CA, ped) }
	pub fn IS_PED_INCAPACITATED(ped: Ped) -> bool { invoke!(0xB655DB7582AEC805, ped) }
	pub fn _SET_PED_INCAPACITATION_MODIFIERS(ped: Ped, canBeIncapacitated: bool, threshold: i32, bleedoutTime: i32, p4: i32) { invoke_ignore!(0x39ED303390DDEAC7, ped, canBeIncapacitated, threshold, bleedoutTime, p4) }
	pub fn _GET_PED_CAN_BE_INCAPACITATED_THIS_FRAME(ped: Ped) -> bool { invoke!(0x7A4E00364B5D727B, ped) }
	pub fn SET_PED_CAN_BE_INCAPACITATED(ped: Ped, toggle: bool) { invoke_ignore!(0x5240864E847C691C, ped, toggle) }
	pub fn _GET_PED_INCAPACITATION_HEALTH(ped: Ped) -> i32 { invoke!(0x89BFDF6D53145545, ped) }
	pub fn _SET_PED_INCAPACITATION_TOTAL_BLEED_OUT_DURATION(ped: Ped, duration: f32) { invoke_ignore!(0x2890418B39BC8FFF, ped, duration) }
	pub fn _RESET_PED_INCAPACITATION_BLEED_OUT_DURATION(ped: Ped) { invoke_ignore!(0x4B9668DB91DC39B8, ped) }
	pub fn _SET_PED_INCAPACITATION_FLAGS(ped: Ped, flags: i32) { invoke_ignore!(0xD67B6F3BCF81BA47, ped, flags) }
	pub fn _0x92A1B55A59720395(p0: Any, p1: Any) { invoke_ignore!(0x92A1B55A59720395, p0, p1) }
	pub fn _INCAPACITATED_REVIVE(ped: Ped, ped2: Ped) { invoke_ignore!(0xF6262491C7704A63, ped, ped2) }
	pub fn _GET_INCAPACITATION_TIME_REMAINING(ped: Ped) -> i32 { invoke!(0x88D9D76D78065487, ped) }
	pub fn _SET_PED_KNOCKED_BY_ONE_HIT(ped: Ped, p1: f32) { invoke_ignore!(0x5BCF0B79D4F5DBA3, ped, p1) }
	pub fn _0x2E5B5D1F1453E08E(ped: Ped, p1: i32) { invoke_ignore!(0x2E5B5D1F1453E08E, ped, p1) }
	pub fn _0x29924EB8EE9DB926(ped: Ped, p1: f32) { invoke_ignore!(0x29924EB8EE9DB926, ped, p1) }
	pub fn _SET_PED_ACTIVE_PLAYER_TYPE(ped: Ped, playerType: Hash) { invoke_ignore!(0xB285AD0EC870B2DF, ped, playerType) }
	pub fn _0xCB86D3E3E3708901(p0: Any, p1: Any, p2: Any, p3: Any, p4: Any) -> Any { invoke!(0xCB86D3E3E3708901, p0, p1, p2, p3, p4) }
	pub fn _0x633F83B301C87994(p0: Any, p1: Any) { invoke_ignore!(0x633F83B301C87994, p0, p1) }
	pub fn _0x5203038FF8BAE577(ped: Ped, p1: i32, p2: i32) -> bool { invoke!(0x5203038FF8BAE577, ped, p1, p2) }
	pub fn _0x7F090958AE95B61B(ped: Ped, p1: i32) -> bool { invoke!(0x7F090958AE95B61B, ped, p1) }
	pub fn _0xC494C76A34266E82(ped: Ped, p1: i32) { invoke_ignore!(0xC494C76A34266E82, ped, p1) }
	pub fn _0xC3995D396F1D97B6(ped: Ped, p1: i32, p2: i32) -> bool { invoke!(0xC3995D396F1D97B6, ped, p1, p2) }
	pub fn _0x15F4732C357B1D6D(p0: Any, p1: Any, p2: Any) { invoke_ignore!(0x15F4732C357B1D6D, p0, p1, p2) }
	pub fn _0x947E43F544B6AB34(ped: Ped, player: Player, flag: i32, ms: i32) -> bool { invoke!(0x947E43F544B6AB34, ped, player, flag, ms) }
	pub fn _0x32417CB860A3BDC4(p0: Any, p1: Any) -> Any { invoke!(0x32417CB860A3BDC4, p0, p1) }
	pub fn _0xE737D5F14304A2EC(ped: Ped, player: Player, p2: i32) { invoke_ignore!(0xE737D5F14304A2EC, ped, player, p2) }
	pub fn _0xEBD49472BCCF7642(p0: Any, p1: Any) { invoke_ignore!(0xEBD49472BCCF7642, p0, p1) }
	pub fn _SET_PLAYER_GREET_DISABLED_FOR_PED(ped: Ped, player: Player, duration: i32) { invoke_ignore!(0x19173C3F15367B54, ped, player, duration) }
	pub fn _SET_PLAYER_ANTAGONIZE_DISABLED_FOR_PED(ped: Ped, player: Player, duration: i32) { invoke_ignore!(0x5708EDD71B50C008, ped, player, duration) }
	pub fn _0xB9BDFAE609DFB7C5(p0: Any, p1: Any, p2: Any) { invoke_ignore!(0xB9BDFAE609DFB7C5, p0, p1, p2) }
	pub fn _0xF7327ACC7A89AEF1(p0: Any, p1: Any, p2: Any) -> Any { invoke!(0xF7327ACC7A89AEF1, p0, p1, p2) }
	pub fn _GET_PED_ATTITUDE(ped: Ped, player: Player) -> i32 { invoke!(0x7CC2186C32D3540A, ped, player) }
	pub fn _0x1E017404784AA6A3(ped: Ped, p1: Hash) -> bool { invoke!(0x1E017404784AA6A3, ped, p1) }
	pub fn _0x2B4CE170DE09F346(ped: Ped, p1: Hash) { invoke_ignore!(0x2B4CE170DE09F346, ped, p1) }
	pub fn _0x7C8AA850617651D9(p0: Any, p1: Any) -> Any { invoke!(0x7C8AA850617651D9, p0, p1) }
	pub fn _GET_PED_MOTIVATION(ped: Ped, motivationState: i32, targetPed: Ped) -> f32 { invoke!(0x42688E94E96FD9B4, ped, motivationState, targetPed) }
	pub fn _SET_PED_MOTIVATION(ped: Ped, motivationState: i32, threshold: f32, targetPed: Ped) { invoke_ignore!(0x06D26A96CA1BCA75, ped, motivationState, threshold, targetPed) }
	pub fn _0x23BDE06596A22CEC(ped: Ped, p1: i32, p2: f32, p3: Any) { invoke_ignore!(0x23BDE06596A22CEC, ped, p1, p2, p3) }
	pub fn _0xCDFB8C04D4C95D9B(p0: Any, p1: Any, p2: Any, p3: Any) { invoke_ignore!(0xCDFB8C04D4C95D9B, p0, p1, p2, p3) }
	pub fn _GET_IS_PED_MOTIVATION_STATE_ENABLED(ped: Ped, motivationState: i32) -> bool { invoke!(0x33FA048675821DA7, ped, motivationState) }
	pub fn _SET_PED_MOTIVATION_STATE_OVERRIDE(ped: Ped, motivationState: i32, enabled: bool) { invoke_ignore!(0x2EB75FB86C41F026, ped, motivationState, enabled) }
	pub fn _SET_PED_MOTIVATION_MODIFIER(ped: Ped, motivationState: i32, modifier: f32) { invoke_ignore!(0xA1EB5D029E0191D3, ped, motivationState, modifier) }
	pub fn _0xFD8E853F0BC2E942(p0: Any, p1: Any) { invoke_ignore!(0xFD8E853F0BC2E942, p0, p1) }
	pub fn _SET_PED_SCENT(ped: Ped, scent: f32) { invoke_ignore!(0x01B21B81865E2A1F, ped, scent) }
	pub fn _SET_PED_LADDER_MOVEMENT_SPEED_MODIFIER(ped: Ped, p1: f32) { invoke_ignore!(0x05CE6AF4DF071D23, ped, p1) }
	pub fn _RESET_PED_LADDER_MOVEMENT_SPEED_MODIFIER(ped: Ped) { invoke_ignore!(0x801917E7D7BCE418, ped) }
	pub fn _0xC6C4E15CF7D52FEA(p0: Any, p1: Any) { invoke_ignore!(0xC6C4E15CF7D52FEA, p0, p1) }
	pub fn _SET_PED_VOICE_VOLUME(ped: Ped, volume: f32) { invoke_ignore!(0xD05AD61F242C626B, ped, volume) }
	pub fn _0x0F967019CC853BCC(p0: Any, p1: Any) { invoke_ignore!(0x0F967019CC853BCC, p0, p1) }
	pub fn _IS_PED_DRAGGING(ped: Ped) -> bool { invoke!(0x226CF9B159E38F42, ped) }
	pub fn IS_PED_BEING_DRAGGED(ped: Ped) -> bool { invoke!(0xEF3A8772F085B4AA, ped) }
	pub fn _0x070A3841406C43D5(p0: Any, p1: Any) { invoke_ignore!(0x070A3841406C43D5, p0, p1) }
	pub fn _ADD_PED_STAY_OUT_VOLUME(ped: Ped, volume: Volume) -> bool { invoke!(0xE9B168527B337BF0, ped, volume) }
	pub fn _REMOVE_PED_STAY_OUT_VOLUME(ped: Ped, volume: Volume) -> bool { invoke!(0x0CAB404CD2DB41F5, ped, volume) }
	pub fn _0x9E66708B2B41F14A(p0: Any, p1: Any) { invoke_ignore!(0x9E66708B2B41F14A, p0, p1) }
	pub fn _0xF634E2892220EF34(ped: Ped, p1: Any) { invoke_ignore!(0xF634E2892220EF34, ped, p1) }
	pub fn _0xAAC0EE3B4999ABB5(ped: Ped, targetPed: Ped) { invoke_ignore!(0xAAC0EE3B4999ABB5, ped, targetPed) }
	pub fn GET_PED_MOTION_FOCUS_ENTITY(ped: Ped) -> Entity { invoke!(0x243E1B4607040057, ped) }
	pub fn _SET_CHAR_EXPRESSION(ped: Ped, index: i32, value: f32) { invoke_ignore!(0x5653AB26C82938CF, ped, index, value) }
	pub fn _GET_CHAR_EXPRESSION(ped: Ped, index: i32) -> f32 { invoke!(0xFD1BA1EEF7985BB8, ped, index) }
	pub fn _0x5BB04BC74A474B47(p0: Any, p1: Any) { invoke_ignore!(0x5BB04BC74A474B47, p0, p1) }
	pub fn _0x9078FB0557364099(p0: Any) { invoke_ignore!(0x9078FB0557364099, p0) }
	pub fn IS_PED_FALLING_OVER(ped: Ped) -> bool { invoke!(0x3E592D0486DEC0F6, ped) }
	pub fn _HAS_PED_BEEN_SHOVED_RECENTLY(ped: Ped, ms: i32) -> bool { invoke!(0x29FCE825613FEFCA, ped, ms) }
	pub fn _GET_PED_TRANQUILIZER(ped: Ped) -> Ped { invoke!(0x65C75FDCCAC86464, ped) }
	pub fn _0x0D497AA69059FE40(p0: Any, p1: Any) { invoke_ignore!(0x0D497AA69059FE40, p0, p1) }
	pub fn _0xD7D2F45C56A4F4DF(p0: Any, p1: Any, p2: Any) { invoke_ignore!(0xD7D2F45C56A4F4DF, p0, p1, p2) }
	pub fn _SET_PED_CULL_RANGE(ped: Ped, p1: f32, p2: f32) { invoke_ignore!(0x8AC1D721B2097B6E, ped, p1, p2) }
	pub fn _0x1D23D3F70606D788(p0: Any, p1: Any) { invoke_ignore!(0x1D23D3F70606D788, p0, p1) }
	pub fn _GET_META_PED_TYPE(ped: Ped) -> i32 { invoke!(0xEC9A1261BF0CE510, ped) }
	pub fn _IS_META_PED_USING_COMPONENT(ped: Ped, component: Hash) -> bool { invoke!(0xFB4891BD7578CDC1, ped, component) }
	pub fn _0xBD0E4F52F6D95242(ped: Ped) -> bool { invoke!(0xBD0E4F52F6D95242, ped) }
	pub fn _IS_PED_CHILD(ped: Ped) -> bool { invoke!(0x137772000DAF42C5, ped) }
	pub fn _0xFFA1594703ED27CA(ped: Ped, p1: i32) { invoke_ignore!(0xFFA1594703ED27CA, ped, p1) }
	pub fn _SET_META_PED_TAG(ped: Ped, drawable: Hash, albedo: Hash, normal: Hash, material: Hash, palette: Hash, tint0: i32, tint1: i32, tint2: i32) { invoke_ignore!(0xBC6DF00D7A4A6819, ped, drawable, albedo, normal, material, palette, tint0, tint1, tint2) }
	pub fn REMOVE_TAG_FROM_META_PED(ped: Ped, component: Hash, p2: i32) { invoke_ignore!(0xD710A5007C2AC539, ped, component, p2) }
	pub fn _0xA2B8E47442C76CEC(p0: Any, p1: Any) { invoke_ignore!(0xA2B8E47442C76CEC, p0, p1) }
	pub fn _GET_NUM_COMPONENT_CATEGORIES_IN_PED(ped: Ped) -> i32 { invoke!(0xA622E66EEE92A08D, ped) }
	pub fn _GET_PED_COMPONENT_CATEGORY_BY_INDEX(ped: Ped, index: i32) -> Hash { invoke!(0xCCB97B51893C662F, ped, index) }
	pub fn _GET_NUM_COMPONENTS_IN_PED(ped: Ped) -> i32 { invoke!(0x90403E8107B60E81, ped) }
	pub fn _0x9B90842304C938A7(p0: Any, p1: Any, p2: Any) -> Any { invoke!(0x9B90842304C938A7, p0, p1, p2) }
	pub fn _UPDATE_PED_VARIATION(ped: Ped, p1: bool, p2: bool, p3: bool, p4: bool, p5: bool) { invoke_ignore!(0xCC8CA3E88256E58F, ped, p1, p2, p3, p4, p5) }
	pub fn _0xCB1A3864C524F784(p0: Any, p1: Any) { invoke_ignore!(0xCB1A3864C524F784, p0, p1) }
	pub fn _0xFA0D206B489A6846(p0: Any, p1: Any, p2: Any, p3: Any, p4: Any) { invoke_ignore!(0xFA0D206B489A6846, p0, p1, p2, p3, p4) }
	pub fn _0xA4AC05B1A364EBC5(p0: Any, p1: Any, p2: Any) -> Any { invoke!(0xA4AC05B1A364EBC5, p0, p1, p2) }
	pub fn _0x1298B3D8E4C2409F(p0: Any) { invoke_ignore!(0x1298B3D8E4C2409F, p0) }
	pub fn _0xA274F51EF7E34B95(p0: Any, p1: Any) -> Any { invoke!(0xA274F51EF7E34B95, p0, p1) }
	pub fn _0xC2EF407645BEECDC(p0: Any) -> Any { invoke!(0xC2EF407645BEECDC, p0) }
	pub fn _EQUIP_META_PED_OUTFIT(ped: Ped, hash: Hash) { invoke_ignore!(0x1902C4CFCC5BE57C, ped, hash) }
	pub fn _0xFA742B82D093D848(p0: Any, p1: Any, p2: Any) { invoke_ignore!(0xFA742B82D093D848, p0, p1, p2) }
	pub fn _GET_PED_META_OUTFIT_HASH(ped: Ped) -> Hash { invoke!(0x30569F348D126A5A, ped) }
	pub fn _0xA2F8B3B5FEDFC100(p0: Any, p1: Any) { invoke_ignore!(0xA2F8B3B5FEDFC100, p0, p1) }
	pub fn _EQUIP_META_PED_SUBOUTFIT(ped: Ped, suboutfit: Hash, p2: i32) { invoke_ignore!(0x66FF395445A88A6E, ped, suboutfit, p2) }
	pub fn _DOES_META_PED_OUTFIT_EXIST_FOR_PED_MODEL(outfit: Hash, model: Hash) -> bool { invoke!(0xC0E880B7A441164D, outfit, model) }
	pub fn _DOES_META_PED_SUBOUTFIT_EXIST_FOR_PED_MODEL(outfit: Hash, suboutfit: Hash, model: Hash) -> bool { invoke!(0x4FF3C2B4E6A196C1, outfit, suboutfit, model) }
	pub fn _0x62FDF4E678E40CC6(entity: Entity, p1: Any) -> Any { invoke!(0x62FDF4E678E40CC6, entity, p1) }
	pub fn _RESET_PED_COMPONENTS(ped: Ped) { invoke_ignore!(0x0BFA1BD465CDFEFD, ped) }
	pub fn _0x370A973252741AC4(ped: Ped, p1: bool) { invoke_ignore!(0x370A973252741AC4, ped, p1) }
	pub fn _SET_RANDOM_OUTFIT_VARIATION(ped: Ped, p1: bool) { invoke_ignore!(0x283978A15512B2FE, ped, p1) }
	pub fn GET_NUM_META_PED_OUTFITS(ped: Ped) -> i32 { invoke!(0x10C70A515BC03707, ped) }
	pub fn _EQUIP_META_PED_OUTFIT_PRESET(ped: Ped, presetId: i32, p2: bool) { invoke_ignore!(0x77FF8D35EEC6BBC4, ped, presetId, p2) }
	pub fn _EQUIP_META_PED_OUTFIT_EXTRA(ped: Ped, component: i32, p2: Any, p3: Any) { invoke_ignore!(0xA5BAE410B03E7371, ped, component, p2, p3) }
	pub fn _IS_META_PED_OUTFIT_EQUIPPED(ped: Ped, outfit: Hash) -> bool { invoke!(0x98082246107A6ACF, ped, outfit) }
	pub fn _0x851966E1E35AF491(p0: Any, p1: Any) { invoke_ignore!(0x851966E1E35AF491, p0, p1) }
	pub fn _SET_TEXTURE_OUTFIT_TINTS(ped: Ped, componentCategory: Hash, palette: Hash, tint0: i32, tint1: i32, tint2: i32) { invoke_ignore!(0x4EFC1F8FF1AD94DE, ped, componentCategory, palette, tint0, tint1, tint2) }
	pub fn _REQUEST_META_PED(model: Hash, p1: i32) -> i32 { invoke!(0xF97C34C33487D569, model, p1) }
	pub fn _HAS_META_PED_REQUEST_LOADED(requestId: i32) -> bool { invoke!(0xC0940AC858C1E126, requestId) }
	pub fn _IS_META_PED_REQUEST_VALID(requestId: i32) -> bool { invoke!(0x43E4DA469541A9C9, requestId) }
	pub fn _CREATE_META_PED(requestId: i32, x: f32, y: f32, z: f32, heading: f32, p5: bool, p6: bool, p7: bool, p8: bool, p9: bool) -> Ped { invoke!(0x0BCD4091C8EABA42, requestId, x, y, z, heading, p5, p6, p7, p8, p9) }
	pub fn _RELEASE_META_PED_REQUEST(requestId: i32) { invoke_ignore!(0x3972F78A78B5D9DF, requestId) }
	pub fn _REQUEST_META_PED_OUTFIT(model: Hash, outfit: Hash) -> i32 { invoke!(0x13154A76CE0CF9AB, model, outfit) }
	pub fn _0x27E8A84C12B0B7D1(p0: Any, p1: Any, p2: Any) -> Any { invoke!(0x27E8A84C12B0B7D1, p0, p1, p2) }
	pub fn _0x273915CE30780986(p0: Any, p1: Any) -> Any { invoke!(0x273915CE30780986, p0, p1) }
	pub fn _RELEASE_META_PED_OUTFIT_REQUEST(requestId: i32) { invoke_ignore!(0x4592B8B9B0EF5F48, requestId) }
	pub fn _0x3FCBB5FCFD968698(drawable: Hash, albedo: Hash, normal: Hash, material: Hash, p4: Any) -> i32 { invoke!(0x3FCBB5FCFD968698, drawable, albedo, normal, material, p4) }
	pub fn _REQUEST_META_PED_ASSET_BUNDLE(asset: Hash, p1: i32) -> i32 { invoke!(0x91FE941F9FCFB702, asset, p1) }
	pub fn _REQUEST_META_PED_COMPONENT(metaPedType: i32, p1: Any, p2: i32, p3: i32, p4: i32) -> Any { invoke!(0xF6D9E1F3560CBF8E, metaPedType, p1, p2, p3, p4) }
	pub fn _RELEASE_META_PED_ASSET_REQUEST(requestId: i32) { invoke_ignore!(0x13E7320C762F0477, requestId) }
	pub fn _HAS_META_PED_ASSET_LOADED(requestId: i32) -> bool { invoke!(0xB0B2C6D170B0E8E5, requestId) }
	pub fn _IS_META_PED_ASSET_VALID(requestId: i32) -> bool { invoke!(0x93FFD92F05EC32FD, requestId) }
	pub fn _CREATE_META_PED_ASSET(asset: Hash, posX: f32, posY: f32, posZ: f32, rotX: f32, rotY: f32, rotZ: f32, p7: bool, p8: bool, p9: bool) -> Entity { invoke!(0x9641A9A20310F6B8, asset, posX, posY, posZ, rotX, rotY, rotZ, p7, p8, p9) }
	pub fn _HAS_META_PED_OUTFIT_LOADED(requestId: i32) -> bool { invoke!(0x610438375E5D1801, requestId) }
	pub fn _IS_META_PED_OUTFIT_REQUEST_VALID(requestId: i32) -> bool { invoke!(0xB25E57FC8E37114D, requestId) }
	pub fn _CREATE_META_PED_OUTFIT_PED(requestId: i32, x: f32, y: f32, z: f32, heading: f32, p5: bool, p6: bool, p7: bool, p8: bool) -> Ped { invoke!(0xEAF682A14F8E5F53, requestId, x, y, z, heading, p5, p6, p7, p8) }
	pub fn _APPLY_PED_META_PED_OUTFIT(requestId: i32, ped: Ped, p2: bool, p3: bool) -> bool { invoke!(0x74F512E29CB717E2, requestId, ped, p2, p3) }
	pub fn _SET_META_PED_WEARINESS(ped: Ped, weariness: f32) { invoke_ignore!(0x314C5465195F3B30, ped, weariness) }
	pub fn _0xF47D54B986F0A346(ped: Ped, danceIntensity: i32) { invoke_ignore!(0xF47D54B986F0A346, ped, danceIntensity) }
	pub fn _0x3EFED081B4834BA1(p0: Any) { invoke_ignore!(0x3EFED081B4834BA1, p0) }
	pub fn _0x0FB1BA7FF73B41E1(p0: Any, p1: Any, p2: Any) { invoke_ignore!(0x0FB1BA7FF73B41E1, p0, p1, p2) }
	pub fn _0xB292203008EBBAAC(p0: Any) -> Any { invoke!(0xB292203008EBBAAC, p0) }
	pub fn _0xD4D403EA031F351C(ped: Ped) -> bool { invoke!(0xD4D403EA031F351C, ped) }
	pub fn _GET_PELT_FROM_HORSE(horse: Ped, index: i32) -> i32 { invoke!(0x0CEEB6F4780B1F2F, horse, index) }
	pub fn _SET_PELT_FOR_HORSE(horse: Ped, peltId: i32) { invoke_ignore!(0xA73F50E8796150D5, horse, peltId) }
	pub fn _0xC412AA1C73111FE0(p0: Any, p1: Any, p2: Any, p3: Any, p4: Any) { invoke_ignore!(0xC412AA1C73111FE0, p0, p1, p2, p3, p4) }
	pub fn _CLEAR_PELT_FROM_HORSE(horse: Ped, peltId: i32) { invoke_ignore!(0x627F7F3A0C4C51FF, horse, peltId) }
	pub fn _SET_PED_LIGHTS(ped: Ped, toggle: bool) { invoke_ignore!(0x13A210949FCBD92B, ped, toggle) }
	pub fn _0xD049FDAF089FDDB0(ped: Ped, p1: Hash, p2: f32) { invoke_ignore!(0xD049FDAF089FDDB0, ped, p1, p2) }
	pub fn _0xDD9540E7B1C9714F(ped: Ped, p1: Hash, r: f32, g: f32, b: f32) { invoke_ignore!(0xDD9540E7B1C9714F, ped, p1, r, g, b) }
	pub fn _0x55546004A244302A(p0: Any, p1: Any) { invoke_ignore!(0x55546004A244302A, p0, p1) }
	pub fn _SET_HEALTH_RECHARGE_MULTIPLIER(ped: Ped, multiplier: i32) { invoke_ignore!(0xDE1B1907A83A1550, ped, multiplier) }
	pub fn _GET_HEALTH_RECHARGE_MULTIPLIER(ped: Ped) -> f32 { invoke!(0x95B8E397B8F4360F, ped) }
	pub fn _SET_STAMINA_DEPLETION_MULTIPLIER(ped: Ped, multiplier: i32) { invoke_ignore!(0xEF5A3D2285D8924B, ped, multiplier) }
	pub fn _GET_STAMINA_DEPLETION_MULTIPLIER(ped: Ped) -> f32 { invoke!(0x825F6DD559A0895B, ped) }
	pub fn _SET_STAMINA_RECHARGE_MULTIPLIER(ped: Ped, multiplier: i32) { invoke_ignore!(0x345C9F993A8AB4A4, ped, multiplier) }
	pub fn _GET_STAMINA_RECHARGE_MULTIPLIER(ped: Ped) -> f32 { invoke!(0xE7687EB2F634ABF0, ped) }
	pub fn _CHANGE_PED_STAMINA(ped: Ped, amount: f32) -> bool { invoke!(0xC3D4B754C0E86B9E, ped, amount) }
	pub fn _GET_PED_STAMINA(ped: Ped) -> f32 { invoke!(0x775A1CA7893AA8B5, ped) }
	pub fn _GET_PED_STAMINA_NORMALIZED(ped: Ped) -> f32 { invoke!(0x22F2A386D43048A9, ped) }
	pub fn _GET_PED_MAX_STAMINA(ped: Ped) -> f32 { invoke!(0xCB42AFE2B613EE55, ped) }
	pub fn _0x36513AFFC703C60D(p0: Any) { invoke_ignore!(0x36513AFFC703C60D, p0) }
	pub fn _RESTORE_PED_STAMINA(ped: Ped, stamina: f32) { invoke_ignore!(0x675680D089BFA21F, ped, stamina) }
	pub fn _0xFC3B580C4380B5B7(ped: Ped) -> i32 { invoke!(0xFC3B580C4380B5B7, ped) }
	pub fn _0xEA8763E505AFD49A(p0: Any, p1: Any, p2: Any) { invoke_ignore!(0xEA8763E505AFD49A, p0, p1, p2) }
	pub fn _0xE4EF4382E22C780C(p0: Any) { invoke_ignore!(0xE4EF4382E22C780C, p0) }
	pub fn _0xD61FCF9FCFD515B7(p0: Any, p1: Any, p2: Any) { invoke_ignore!(0xD61FCF9FCFD515B7, p0, p1, p2) }
	pub fn _0x8D9DB115FBA8E23D(p0: Any) { invoke_ignore!(0x8D9DB115FBA8E23D, p0) }
	pub fn _GET_NUM_RESERVED_HEALTH(ped: Ped) -> Any { invoke!(0x16F2C8C084AB2092, ped) }
	pub fn _0xD97BC27AC039F681(p0: Any, p1: Any, p2: Any, p3: Any) -> Any { invoke!(0xD97BC27AC039F681, p0, p1, p2, p3) }
	pub fn _0xF6A8C4B4A11AE89C(p0: Any, p1: Any, p2: Any, p3: Any, p4: Any, p5: Any) -> Any { invoke!(0xF6A8C4B4A11AE89C, p0, p1, p2, p3, p4, p5) }
	pub fn _0xE4C95E0AE31C6512(ped: Ped, p1: Any) { invoke_ignore!(0xE4C95E0AE31C6512, ped, p1) }
	pub fn _GET_PED_LAST_DROPPED_HAT(ped: Ped) -> Object { invoke!(0x1F714E7A9DADFC42, ped) }
	pub fn _0x5D4CD22A8C82A81A(ped: Ped, p1: bool) { invoke_ignore!(0x5D4CD22A8C82A81A, ped, p1) }
	pub fn _0xBF567DF2BEF211A6(p0: Any, p1: Any) { invoke_ignore!(0xBF567DF2BEF211A6, p0, p1) }
	pub fn _CREATE_GRAVITY_WELL(xPos: f32, yPos: f32, zPos: f32, heading: f32, radius: f32, p5: f32, p6: f32, p7: f32, stopAtDestination: bool) -> i32 { invoke!(0x4F5EBE70081E5A20, xPos, yPos, zPos, heading, radius, p5, p6, p7, stopAtDestination) }
	pub fn _REMOVE_GRAVITY_WELL(handle: i32) { invoke_ignore!(0x87247BC60B60BED8, handle) }
	pub fn _IS_PED_INTIMIDATED(ped: Ped) -> bool { invoke!(0x57779B55B83E2BEA, ped) }
	pub fn _0x7EE3A8660F38797E(ped: Ped) -> bool { invoke!(0x7EE3A8660F38797E, ped) }
	pub fn _0xA180FBD502A03125(p0: Any, p1: Any, p2: Any) -> Any { invoke!(0xA180FBD502A03125, p0, p1, p2) }
	pub fn _REQUEST_TEXTURE(albedoHash: Hash, normalHash: Hash, materialHash: Hash) -> i32 { invoke!(0xC5E7204F322E49EB, albedoHash, normalHash, materialHash) }
	pub fn _APPLY_TEXTURE_ON_PED(ped: Ped, componentHash: Hash, textureId: i32) { invoke_ignore!(0x0B46E25761519058, ped, componentHash, textureId) }
	pub fn _RELEASE_TEXTURE(textureId: i32) { invoke_ignore!(0x6BEFAA907B076859, textureId) }
	pub fn _UPDATE_PED_TEXTURE(textureId: i32) { invoke_ignore!(0x92DAABA2C1C10B0E, textureId) }
	pub fn _RESET_PED_TEXTURE(textureId: i32) { invoke_ignore!(0x8472A1789478F82F, textureId) }
	pub fn _IS_TEXTURE_VALID(textureId: i32) -> bool { invoke!(0x31DC8D3F216D8509, textureId) }
	pub fn _ADD_TEXTURE_LAYER(textureId: i32, albedoHash: Hash, normalHash: Hash, materialHash: Hash, blendType: i32, texAlpha: f32, sheetGridIndex: i32) -> i32 { invoke!(0x86BB5FF45F193A02, textureId, albedoHash, normalHash, materialHash, blendType, texAlpha, sheetGridIndex) }
	pub fn _REMOVE_PED_OVERLAY(textureId: i32, overlayId: i32) { invoke_ignore!(0x96C349DE04C49011, textureId, overlayId) }
	pub fn _SET_TEXTURE_LAYER_SHEET_GRID_INDEX(textureId: i32, layerId: i32, sheetGridIndex: i32) { invoke_ignore!(0x3329AAE2882FC8E4, textureId, layerId, sheetGridIndex) }
	pub fn _SET_TEXTURE_LAYER_ALPHA(textureId: i32, layerId: i32, texAlpha: f32) { invoke_ignore!(0x6C76BC24F8BB709A, textureId, layerId, texAlpha) }
	pub fn _SET_TEXTURE_LAYER_ROUGHNESS(textureId: i32, layerId: i32, texRough: f32) { invoke_ignore!(0x057C4F092E2298BE, textureId, layerId, texRough) }
	pub fn _SET_TEXTURE_LAYER_PALLETE(textureId: i32, layerId: i32, paletteHash: Hash) { invoke_ignore!(0x1ED8588524AC9BE1, textureId, layerId, paletteHash) }
	pub fn _SET_TEXTURE_LAYER_TINT(textureId: i32, layerId: i32, tint0: i32, tint1: i32, tint2: i32) { invoke_ignore!(0x2DF59FFE6FFD6044, textureId, layerId, tint0, tint1, tint2) }
	pub fn _SET_TEXTURE_LAYER_MOD(textureId: i32, layerId: i32, modTextureHash: Hash, modAlpha: f32, modChannel: i32) { invoke_ignore!(0xF2EA041F1146D75B, textureId, layerId, modTextureHash, modAlpha, modChannel) }
	pub fn _SET_TEXTURE_LAYER_TEXTURE_MAP(textureId: i32, layerId: i32, albedoHash: Hash, normalHash: Hash, materialHash: Hash) { invoke_ignore!(0x253A63B5BADBC398, textureId, layerId, albedoHash, normalHash, materialHash) }
	pub fn _CLEAR_PED_TEXTURE(textureId: i32) { invoke_ignore!(0xB63B9178D0F58D82, textureId) }
	pub fn _0xC991EF46FE323867(ped: Ped, p1: Any) { invoke_ignore!(0xC991EF46FE323867, ped, p1) }
	pub fn _0x1F8215D0E446F593(p0: Any, p1: Any, p2: Any) { invoke_ignore!(0x1F8215D0E446F593, p0, p1, p2) }
	pub fn IS_LOCATION_SPAWN_SAFE(ped: Ped, p1: f32) -> bool { invoke!(0xFB1E7998B8595825, ped, p1) }
	pub fn _0x53BA7D96B9A421D9(p0: Any, p1: Any) { invoke_ignore!(0x53BA7D96B9A421D9, p0, p1) }
	pub fn _0x96C7B659854DE629(p0: Any, p1: Any) { invoke_ignore!(0x96C7B659854DE629, p0, p1) }
	pub fn SET_HORSE_AVOIDANCE_LEVEL(horse: Ped, avoidanceLevel: i32) { invoke_ignore!(0xDDCF6FEA5D7ACC17, horse, avoidanceLevel) }
	pub fn RESET_HORSE_AVOIDANCE_LEVEL_TO_DEFAULT(horse: Ped) { invoke_ignore!(0x2A5AFD2B8381A6E1, horse) }
	pub fn _SET_PED_USE_HORSE_MAP_COLLISION(ped: Ped, toggle: bool) -> Any { invoke!(0xEB72453B6F5B45B0, ped, toggle) }
	pub fn _0x5B73975B4F12F7F3(p0: Any, p1: Any, p2: Any, p3: Any, p4: Any) { invoke_ignore!(0x5B73975B4F12F7F3, p0, p1, p2, p3, p4) }
	pub fn GET_META_PED_ASSET_GUIDS(ped: Ped, index: i32, drawable: &mut Hash, albedo: &mut Hash, normal: &mut Hash, material: &mut Hash) -> bool { invoke!(0xA9C28516A6DC9D56, ped, index, drawable, albedo, normal, material) }
	pub fn GET_META_PED_ASSET_TINT(ped: Ped, index: i32, pallete: &mut Hash, tint0: &mut i32, tint1: &mut i32, tint2: &mut i32) -> bool { invoke!(0xE7998FEC53A33BBE, ped, index, pallete, tint0, tint1, tint2) }
	pub fn _SET_PED_BLACKBOARD_INT(ped: Ped, variableName: & CStr, value: i32, removeTimer: i32) { invoke_ignore!(0x5F53010C4C3F6BAF, ped, variableName, value, removeTimer) }
	pub fn _SET_PED_BLACKBOARD_BOOL(ped: Ped, variableName: & CStr, value: bool, removeTimer: i32) { invoke_ignore!(0xCB9401F918CB0F75, ped, variableName, value, removeTimer) }
	pub fn _SET_PED_BLACKBOARD_FLOAT(ped: Ped, variableName: & CStr, value: f32, removeTimer: i32) { invoke_ignore!(0x437C08DB4FEBE2BD, ped, variableName, value, removeTimer) }
	pub fn _SET_PED_BLACKBOARD_HASH(ped: Ped, variableName: & CStr, value: & CStr, removeTimer: i32) { invoke_ignore!(0xA762C9D6CF165E0D, ped, variableName, value, removeTimer) }
	pub fn _REMOVE_PED_BLACKBOARD_INT(ped: Ped, variableName: & CStr) { invoke_ignore!(0x81B75428A7813E67, ped, variableName) }
	pub fn _REMOVE_PED_BLACKBOARD_BOOL(ped: Ped, variableName: & CStr) { invoke_ignore!(0xA6F67BEC53379A32, ped, variableName) }
	pub fn _REMOVE_PED_BLACKBOARD_FLOAT(ped: Ped, variableName: & CStr) { invoke_ignore!(0x411189E51B8020BA, ped, variableName) }
	pub fn _REMOVE_PED_BLACKBOARD_HASH(ped: Ped, variableName: & CStr) { invoke_ignore!(0x0E17378642156790, ped, variableName) }
	pub fn _GET_PED_BLACKBOARD_BOOL(ped: Ped, variableName: & CStr) -> bool { invoke!(0x498F2E77982D6945, ped, variableName) }
	pub fn _GET_PED_BLACKBOARD_FLOAT(ped: Ped, variableName: & CStr) -> f32 { invoke!(0x56E58D4D118FB45E, ped, variableName) }
	pub fn _GET_PED_BLACKBOARD_HASH(ped: Ped, variableName: & CStr) -> Hash { invoke!(0xBF5E791BBBF90A3C, ped, variableName) }
	pub fn GET_PED_BLACKBOARD_SCRIPT_INT(ped: Ped, variableName: & CStr) -> i32 { invoke!(0xB71B91B398F8F067, ped, variableName) }
	pub fn GET_PED_BLACKBOARD_SCRIPT_BOOL(ped: Ped, variableName: & CStr) -> bool { invoke!(0x4912DFE492DB98CD, ped, variableName) }
	pub fn GET_PED_BLACKBOARD_SCRIPT_FLOAT(ped: Ped, variableName: & CStr) -> f32 { invoke!(0xA29FD00D45311EB7, ped, variableName) }
	pub fn _SET_TANK_ATTRIBUTE_SIZE(ped: Ped, attributeIndex: i32, size: f32) { invoke_ignore!(0x7FF72DE061DF55E2, ped, attributeIndex, size) }
	pub fn _0xA31D350D66FA1855(p0: Any) -> Any { invoke!(0xA31D350D66FA1855, p0) }
	pub fn _IS_USING_SLIPSTREAM(ped: Ped) -> bool { invoke!(0xAF61B3CD8C3B82C3, ped) }
	pub fn _0xEEDC9B29314B2733(p0: Any, p1: Any, p2: Any, p3: Any, p4: Any, p5: Any, p6: Any, p7: Any, p8: Any, p9: Any) { invoke_ignore!(0xEEDC9B29314B2733, p0, p1, p2, p3, p4, p5, p6, p7, p8, p9) }
	pub fn _0x5C90E20C25E6D83C(p0: Any) { invoke_ignore!(0x5C90E20C25E6D83C, p0) }
	pub fn _0x10F96086123B939F(legendaryPed: Ped, preyPed: Ped, p2: f32) { invoke_ignore!(0x10F96086123B939F, legendaryPed, preyPed, p2) }
	pub fn _0x3FDBB99EFD8CE4AF(p0: Any, p1: Any, p2: Any) { invoke_ignore!(0x3FDBB99EFD8CE4AF, p0, p1, p2) }
	pub fn _0x7E8F9949B7AABBF0(p0: Any, p1: Any, p2: Any) { invoke_ignore!(0x7E8F9949B7AABBF0, p0, p1, p2) }
	pub fn _0x5DA36CCCB63C0895(p0: Any, p1: Any, p2: Any) -> Any { invoke!(0x5DA36CCCB63C0895, p0, p1, p2) }
	pub fn _0x56076667E7C2DCD6(p0: Any, p1: Any) { invoke_ignore!(0x56076667E7C2DCD6, p0, p1) }
	pub fn _APPLY_SHOP_ITEM_TO_PED(ped: Ped, componentHash: Hash, immediately: bool, isMp: bool, p4: bool) { invoke_ignore!(0xD3A7B003ED343FD9, ped, componentHash, immediately, isMp, p4) }
	pub fn _REMOVE_SHOP_ITEM_FROM_PED(ped: Ped, componentHash: Hash, p2: i32, p3: bool) { invoke_ignore!(0x0D7FFA1B2F69ED82, ped, componentHash, p2, p3) }
	pub fn REMOVE_SHOP_ITEM_FROM_PED_BY_CATEGORY(ped: Ped, componentCategory: Hash, p2: i32, p3: bool) { invoke_ignore!(0xDF631E4BCE1B1FC4, ped, componentCategory, p2, p3) }
	pub fn _UPDATE_SHOP_ITEM_WEARABLE_STATE(ped: Ped, componentHash: Hash, wearableState: Hash, p3: i32, p4: bool, p5: i32) { invoke_ignore!(0x66B957AAC2EAAEAB, ped, componentHash, wearableState, p3, p4, p5) }
	pub fn _GET_SHOP_ITEM_COMPONENT_AT_INDEX(ped: Ped, index: i32, p2: bool, argStruct: &mut Any, argStruct2: &mut Any) -> Hash { invoke!(0x77BA37622E22023B, ped, index, p2, argStruct, argStruct2) }
	pub fn _GET_SHOP_ITEM_NUM_WEARABLE_STATES(componentHash: Hash, isMpFemale: bool, p2: bool) -> i32 { invoke!(0xFFCC2DB2D9953401, componentHash, isMpFemale, p2) }
	pub fn _GET_SHOP_ITEM_WEARABLE_STATE_BY_INDEX(componentHash: Hash, wearableStateIndex: i32, isMpFemale: bool, p3: bool) -> Hash { invoke!(0x6243635AF2F1B826, componentHash, wearableStateIndex, isMpFemale, p3) }
	pub fn _GET_SHOP_ITEM_COMPONENT_CATEGORY(componentHash: Hash, metapedType: i32, isMP: bool) -> Hash { invoke!(0x5FF9A878C3D115B8, componentHash, metapedType, isMP) }
	pub fn _0x31B2E7F2E3C58B89(p0: Any, p1: Any, p2: Any, p3: Any) -> Any { invoke!(0x31B2E7F2E3C58B89, p0, p1, p2, p3) }
	pub fn _GET_SHOP_ITEM_BASE_LAYERS(shopItem: Hash, p1: Any, ped: Ped, metapedType: i32, p4: bool, drawable: &mut Hash, albedo: &mut Hash, normal: &mut Hash, material: &mut Hash, p9: &mut Hash, p10: &mut Hash, p11: &mut Hash, p12: &mut Hash) -> bool { invoke!(0x63342C50EC115CE8, shopItem, p1, ped, metapedType, p4, drawable, albedo, normal, material, p9, p10, p11, p12) }
	pub fn _0xAAB86462966168CE(ped: Ped, isMP: bool) -> Any { invoke!(0xAAB86462966168CE, ped, isMP) }
	pub fn _GET_SHOP_ITEM_HAT_COMPONENT(ped: Ped, metapedType: i32, p2: bool) -> Any { invoke!(0x7E02E4218D916B94, ped, metapedType, p2) }
	pub fn _REFRESH_META_PED_SHOP_ITEMS(ped: Ped, p1: i32) { invoke_ignore!(0x59BD177A1A48600A, ped, p1) }
	pub fn _0xD103F6DBB5442BE8(ped: Ped, p1: i32) { invoke_ignore!(0xD103F6DBB5442BE8, ped, p1) }
	pub fn _SET_PED_PROMPT_NAME_FROM_GXT_ENTRY(ped: Ped, gxtEntryHash: Hash) { invoke_ignore!(0xFCA8FB9E15FA80D3, ped, gxtEntryHash) }
	pub fn _SET_PED_PROMPT_NAME(ped: Ped, name: & CStr) { invoke_ignore!(0x4A48B6E03BABB4AC, ped, name) }
	pub fn _SET_PED_PROMPT_NAME_FROM_GXT_ENTRY_2(ped: Ped, gxtEntryHash: Hash) { invoke_ignore!(0xC2745D9261664901, ped, gxtEntryHash) }
	pub fn _SET_PED_PROMPT_NAME_2(ped: Ped, name: & CStr) { invoke_ignore!(0x19B14E04B009E28B, ped, name) }
	pub fn _0xF917F92BF22ECBAB(p0: Any) { invoke_ignore!(0xF917F92BF22ECBAB, p0) }
	pub fn _0x49DADFC4CD808B0A(p0: Any, p1: Any, p2: Any) { invoke_ignore!(0x49DADFC4CD808B0A, p0, p1, p2) }
	pub fn _HAS_PED_TAKEN_GORE_DAMAGE(ped: Ped, limb: i32) -> bool { invoke!(0xBA208A8D6399A3AC, ped, limb) }
	pub fn _0x704C908E9C405136(ped: Ped) { invoke_ignore!(0x704C908E9C405136, ped) }
	pub fn _0x7406C71F4AC2FFCC(p0: Any) { invoke_ignore!(0x7406C71F4AC2FFCC, p0) }
	pub fn _0x28508173C6A7CC18(p0: Any) { invoke_ignore!(0x28508173C6A7CC18, p0) }
	pub fn _0x52250B92EA70BE3D(p0: Any) -> Any { invoke!(0x52250B92EA70BE3D, p0) }
	pub fn _SET_PED_QUALITY(ped: Ped, quality: i32) { invoke_ignore!(0xCE6B874286D640BB, ped, quality) }
	pub fn _GET_PED_QUALITY(ped: Ped) -> i32 { invoke!(0x7BCC6087D130312A, ped) }
	pub fn _0xCE7A6C1D5CDE1F9D(ped: Ped, object: Object, propName: & CStr, animName: & CStr) { invoke_ignore!(0xCE7A6C1D5CDE1F9D, ped, object, propName, animName) }
	pub fn _0x604E1010E3162E86(p0: Any, p1: Any, p2: Any) { invoke_ignore!(0x604E1010E3162E86, p0, p1, p2) }
	pub fn _0xE8ABE3B73FC7FE17(p0: Any, p1: Any, p2: Any, p3: Any) { invoke_ignore!(0xE8ABE3B73FC7FE17, p0, p1, p2, p3) }
	pub fn _REMOVE_PED_PROP(ped: Ped, propName: & CStr) { invoke_ignore!(0x3A50753042B6891B, ped, propName) }
	pub fn _SET_TOTAL_PED_DAMAGE_FALLOFF_BONUS(ped: Ped, bonus: f32) { invoke_ignore!(0x932786CE3C76477C, ped, bonus) }
	pub fn _0x095C2277FED731DB(p0: Any) -> Any { invoke!(0x095C2277FED731DB, p0) }
	pub fn _0x09171A6F8FDE5DC1(p0: Any, p1: Any, p2: Any, p3: Any, p4: Any) { invoke_ignore!(0x09171A6F8FDE5DC1, p0, p1, p2, p3, p4) }
	pub fn _0x09E378C52B1433B5(p0: Any, p1: Any, p2: Any, p3: Any, p4: Any) { invoke_ignore!(0x09E378C52B1433B5, p0, p1, p2, p3, p4) }
	pub fn _0x6507AC3BD7C99009(x: f32, y: f32, z: f32, p3: f32) -> bool { invoke!(0x6507AC3BD7C99009, x, y, z, p3) }
	pub fn _GET_PLAYER_PED_WATER_DEPTH(ped: Ped) -> f32 { invoke!(0x2942457417A5FD24, ped) }
	pub fn _SET_PED_TRAIL_EFFECT(ped: Ped, p1: bool, duration: f32) { invoke_ignore!(0xA5950E16B8F31052, ped, p1, duration) }
	pub fn _0xEBAAC9A750E7563B(ped: Ped) -> bool { invoke!(0xEBAAC9A750E7563B, ped) }
	pub fn _0x992187D975635DF5(p0: Any, p1: Any) { invoke_ignore!(0x992187D975635DF5, p0, p1) }
	pub fn _0x0B787A37EEDD226F(p0: Any, p1: Any) { invoke_ignore!(0x0B787A37EEDD226F, p0, p1) }
}
pub mod PERSCHAR {
	use std::ffi::CStr;
	use crate::core::scripthook::native::*;
	use crate::core::types::*;

	pub fn _0x63AA2B8EB087886A(p0: Any, p1: Any) { invoke_ignore!(0x63AA2B8EB087886A, p0, p1) }
	pub fn _GET_PERSCHAR_MODEL_NAME(persCharHash: Hash) -> Hash { invoke!(0xA00DF706C60173D1, persCharHash) }
	pub fn _0x8BC555034A5A5E8C(p0: Any, p1: Any) { invoke_ignore!(0x8BC555034A5A5E8C, p0, p1) }
	pub fn _0x70605812ABC9FF0F(p0: Any, p1: Any) { invoke_ignore!(0x70605812ABC9FF0F, p0, p1) }
	pub fn _0xDC9655D47DEC0353(p0: Any) -> Any { invoke!(0xDC9655D47DEC0353, p0) }
	pub fn _0x2DF89CD2ED1D0BDE(p0: Any, p1: Any) { invoke_ignore!(0x2DF89CD2ED1D0BDE, p0, p1) }
	pub fn _0x535A66AAD2BF68F9(p0: Any, p1: Any) { invoke_ignore!(0x535A66AAD2BF68F9, p0, p1) }
	pub fn _0xCEB40B678E403759(p0: Any) -> Any { invoke!(0xCEB40B678E403759, p0) }
	pub fn _SET_PERSCHAR_SCHEDULE(persCharHash: Hash, schedule: & CStr) { invoke_ignore!(0x187D65F3AEC5D679, persCharHash, schedule) }
	pub fn _0x8B44273A92CD406C(p0: Any) { invoke_ignore!(0x8B44273A92CD406C, p0) }
	pub fn _0xE0E65E0D261F7507(p0: Any) { invoke_ignore!(0xE0E65E0D261F7507, p0) }
	pub fn _0x112DDF56300BC6E5(p0: Any) -> Any { invoke!(0x112DDF56300BC6E5, p0) }
	pub fn _CREATE_PERSISTENT_CHARACTER(hash: Hash) -> PersChar { invoke!(0x4F76E3676583D951, hash) }
	pub fn _IS_PERSISTENT_CHARACTER_VALID(persChar: PersChar) -> bool { invoke!(0x800DF3FC913355F3, persChar) }
	pub fn _DELETE_PERSCHAR(persChar: PersChar) { invoke_ignore!(0xFC77C5B44D5FF7C0, persChar) }
	pub fn _0x5EE6FCCC9C832CA2(p0: Any) -> Vector3 { invoke!(0x5EE6FCCC9C832CA2, p0) }
	pub fn _0x59C7AD6FEA2AC449(p0: Any, p1: Any, p2: Any, p3: Any) { invoke_ignore!(0x59C7AD6FEA2AC449, p0, p1, p2, p3) }
	pub fn _0xBB68908CD11AEBDC(persChar: PersChar) { invoke_ignore!(0xBB68908CD11AEBDC, persChar) }
	pub fn _0x94995829ED15A598(p0: Any) -> Vector3 { invoke!(0x94995829ED15A598, p0) }
	pub fn _GET_PERSCHAR_PED_INDEX(persChar: PersChar) -> Ped { invoke!(0x31C70A716CAE1FEE, persChar) }
	pub fn _GET_PERSCHAR_INDEX_FROM_PED_INDEX(ped: Ped) -> PersChar { invoke!(0x32A1E3B83D501096, ped) }
	pub fn _0xF8DE7154F7D1458F(p0: Any) -> Any { invoke!(0xF8DE7154F7D1458F, p0) }
	pub fn _0x669C25840C6F7AE2(p0: Any, p1: Any) { invoke_ignore!(0x669C25840C6F7AE2, p0, p1) }
	pub fn _RETASK_PERSISTENT_CHARACTER(persChar: PersChar) { invoke_ignore!(0x631CD2D77FDC0316, persChar) }
	pub fn _0x0B3A99AB6713AA52(p0: Any) { invoke_ignore!(0x0B3A99AB6713AA52, p0) }
	pub fn _IS_PERSISTENT_CHARACTER_DEAD(persChar: PersChar) -> bool { invoke!(0xEB98B38CA60742D7, persChar) }
	pub fn _REVIVE_PERSCHAR(persChar: PersChar) -> bool { invoke!(0x49A8C2CD97815215, persChar) }
	pub fn _0xD4B614179BCD0654(p0: Any) { invoke_ignore!(0xD4B614179BCD0654, p0) }
	pub fn _0x406808610220405B(p0: Any) { invoke_ignore!(0x406808610220405B, p0) }
	pub fn _0xA2B18FF8D39F6D87(p0: Any) { invoke_ignore!(0xA2B18FF8D39F6D87, p0) }
	pub fn _0xE4C51A8A3BD1664C(p0: Any) -> Any { invoke!(0xE4C51A8A3BD1664C, p0) }
	pub fn _0x8AE4EFA464DAE42D(p0: Any, p1: Any) { invoke_ignore!(0x8AE4EFA464DAE42D, p0, p1) }
	pub fn _0xA4DCB3F0DD7488BD(p0: Any, p1: Any, p2: Any, p3: Any, p4: Any) { invoke_ignore!(0xA4DCB3F0DD7488BD, p0, p1, p2, p3, p4) }
	pub fn _0xD95D777F828B2BBB(p0: Any) { invoke_ignore!(0xD95D777F828B2BBB, p0) }
	pub fn _0x92690B0822493CE0() { invoke_ignore!(0x92690B0822493CE0) }
	pub fn _0xEFC5C6670E0B99BA() { invoke_ignore!(0xEFC5C6670E0B99BA) }
	pub fn _0x2E957AA81F2C61C9() { invoke_ignore!(0x2E957AA81F2C61C9) }
	pub fn _0xB173599D61FAEB31() { invoke_ignore!(0xB173599D61FAEB31) }
	pub fn _0x08FC896D2CB31FCC(p0: Any, p1: bool) -> Entity { invoke!(0x08FC896D2CB31FCC, p0, p1) }
	pub fn _FORCE_SPAWN_PERSCHAR(persChar: PersChar, p1: bool) -> Entity { invoke!(0x0CADC3A977997472, persChar, p1) }
	pub fn _FORCE_DESPAWN_PERSCHAR(persChar: PersChar) { invoke_ignore!(0x7B204F88F6C3D287, persChar) }
	pub fn _0xFCC6DB8DBE709BC8(persChar: PersChar) { invoke_ignore!(0xFCC6DB8DBE709BC8, persChar) }
	pub fn _0xA8C406C2A56EDC16(persChar: PersChar) { invoke_ignore!(0xA8C406C2A56EDC16, persChar) }
	pub fn _0x4F81EAD1DE8FA19B(persChar: PersChar) { invoke_ignore!(0x4F81EAD1DE8FA19B, persChar) }
	pub fn _0x6759BEE6762E140B(persChar: PersChar) { invoke_ignore!(0x6759BEE6762E140B, persChar) }
	pub fn _0xB65E7F733956CF25(persChar: PersChar) { invoke_ignore!(0xB65E7F733956CF25, persChar) }
	pub fn _0x4AFC7288C77238B3(p0: Any) -> Any { invoke!(0x4AFC7288C77238B3, p0) }
	pub fn _0xA8120EBEAF290C7A(p0: Any) -> Any { invoke!(0xA8120EBEAF290C7A, p0) }
	pub fn _0x69786495C92A3044(p0: Any) -> Any { invoke!(0x69786495C92A3044, p0) }
	pub fn _0xEC254C2C9B0F08F1(p0: Any, p1: Any) -> Any { invoke!(0xEC254C2C9B0F08F1, p0, p1) }
	pub fn _0x9C7F95946E304778(p0: Any, p1: Any) -> Any { invoke!(0x9C7F95946E304778, p0, p1) }
}
pub mod PERSISTENCE {
	use std::ffi::CStr;
	use crate::core::scripthook::native::*;
	use crate::core::types::*;

	pub fn _0x7A1BD123E5CDB6E5() { invoke_ignore!(0x7A1BD123E5CDB6E5) }
	pub fn PERSISTENCE_REMOVE_ALL_ENTITIES_IN_AREA(x: f32, y: f32, z: f32, radius: f32) { invoke_ignore!(0x9D16896F0DBE78A2, x, y, z, radius) }
	pub fn _0x065887B694359799(p0: Any) { invoke_ignore!(0x065887B694359799, p0) }
	pub fn _0xFC9806DA9A460093(x1: f32, y1: f32, z1: f32, x2: f32, y2: f32, z2: f32) { invoke_ignore!(0xFC9806DA9A460093, x1, y1, z1, x2, y2, z2) }
	pub fn _0xB03140014ACA6C40(p0: Any, p1: Any) { invoke_ignore!(0xB03140014ACA6C40, p0, p1) }
	pub fn _0xE225CEF1901F6108(p0: Any, p1: Any) { invoke_ignore!(0xE225CEF1901F6108, p0, p1) }
	pub fn _0x8DE104BEC243A73B(p0: Any) { invoke_ignore!(0x8DE104BEC243A73B, p0) }
	pub fn _PERSISTENCE_REFRESH_TOWN_VOLUME(volume: Volume) { invoke_ignore!(0xEFB5F34CC0953B27, volume) }
	pub fn _0xBA2C49EA6A8D24FF(p0: Any, p1: Any, p2: Any, p3: Any, p4: Any, p5: Any, p6: Any) -> Any { invoke!(0xBA2C49EA6A8D24FF, p0, p1, p2, p3, p4, p5, p6) }
	pub fn _0x2E545965DF98D476(p0: Any) -> Any { invoke!(0x2E545965DF98D476, p0) }
	pub fn _0xF5622FA6ACFCA7DB(p0: Any, p1: Any) { invoke_ignore!(0xF5622FA6ACFCA7DB, p0, p1) }
	pub fn _0x3CA5E58C9731A16B(p0: Any, p1: Any) { invoke_ignore!(0x3CA5E58C9731A16B, p0, p1) }
	pub fn _0xDC0A1F0ECEC9F0C0(p0: Any, p1: Any) { invoke_ignore!(0xDC0A1F0ECEC9F0C0, p0, p1) }
	pub fn _0x5A79220F6D38D7C3(p0: Any) -> Any { invoke!(0x5A79220F6D38D7C3, p0) }
	pub fn _0xCFDA2518F322D836(p0: Any) -> Any { invoke!(0xCFDA2518F322D836, p0) }
	pub fn _0x1F56FB3FDB4EAF65(p0: Any) -> Any { invoke!(0x1F56FB3FDB4EAF65, p0) }
	pub fn _0x291CC21D1FB6790E(p0: Any) { invoke_ignore!(0x291CC21D1FB6790E, p0) }
	pub fn PERSISTENCE_ADD_SCENARIO_LOOTED(scenario: i32) { invoke_ignore!(0x8245C1F3262F4AC2, scenario) }
	pub fn _PERSISTENCE_IS_SCENARIO_MARKED_AS_LOOTED(scenario: i32) -> bool { invoke!(0xFB7CF1DE938A3E22, scenario) }
	pub fn _PERSISTENCE_IS_SCENARIO_MARKED_AS_LOOTED_AT_COORDS(x: f32, y: f32, z: f32) -> bool { invoke!(0xB6E1A185C2B9319A, x, y, z) }
	pub fn _PERSISTENCE_IS_SCENARIO_MARKED_AS_LOOTED_AT_COORDS_WITH_MODEL(x: f32, y: f32, z: f32, model: Hash) -> bool { invoke!(0x188313616D184213, x, y, z, model) }
	pub fn _0x66DAA3A9274E8E82() { invoke_ignore!(0x66DAA3A9274E8E82) }
}
pub mod PHYSICS {
	use std::ffi::CStr;
	use crate::core::scripthook::native::*;
	use crate::core::types::*;

	pub fn ADD_ROPE(x: f32, y: f32, z: f32, rotX: f32, rotY: f32, rotZ: f32, length: f32, ropeType: i32, maxLength: f32, minLength: f32, p10: f32, p11: bool, p12: bool, rigid: bool, p14: f32, breakWhenShot: bool, unkPtr: &mut Any, p17: bool) -> i32 { invoke!(0xE832D760399EB220, x, y, z, rotX, rotY, rotZ, length, ropeType, maxLength, minLength, p10, p11, p12, rigid, p14, breakWhenShot, unkPtr, p17) }
	pub fn _ADD_ROPE_2(x: f32, y: f32, z: f32, rotX: f32, rotY: f32, rotZ: f32, length: f32, ropeType: i32, isNetworked: bool, p9: i32, p10: f32) -> i32 { invoke!(0xE9C59F6809373A99, x, y, z, rotX, rotY, rotZ, length, ropeType, isNetworked, p9, p10) }
	pub fn DELETE_ROPE(ropeId: &mut i32) { invoke_ignore!(0x52B4829281364649, ropeId) }
	pub fn _RELEASE_ROPE(ropeId: i32) { invoke_ignore!(0x6076213101A47B3B, ropeId) }
	pub fn DELETE_CHILD_ROPE(ropeId: i32) { invoke_ignore!(0xAA5D6B1888E4DB20, ropeId) }
	pub fn _BREAK_ROPE(ropeId: &mut i32, ropeTop: &mut i32, ropeBottom: &mut i32, offsetX: f32, offsetY: f32, offsetZ: f32, p6: i32) { invoke_ignore!(0x4CFA2B7FAE115ECB, ropeId, ropeTop, ropeBottom, offsetX, offsetY, offsetZ, p6) }
	pub fn DOES_ROPE_EXIST(ropeId: i32) -> bool { invoke!(0xFD5448BE3111ED96, ropeId) }
	pub fn _IS_ROPE_BROKEN(ropeId: i32) -> bool { invoke!(0x79C2BEC82CFD7F7F, ropeId) }
	pub fn _ROPE_CHANGE_VISIBILITY(ropeId: &mut i32, visible: bool) { invoke_ignore!(0x7A54D82227A139DB, ropeId, visible) }
	pub fn ROPE_DRAW_SHADOW_ENABLED(ropeId: &mut i32, toggle: bool) { invoke_ignore!(0xF159A63806BB5BA8, ropeId, toggle) }
	pub fn GET_ROPE_VERTEX_COUNT(ropeId: i32) -> i32 { invoke!(0x3655F544CD30F0B5, ropeId) }
	pub fn _0xE54BF2CE6C7D23A9(ropeId: i32, p1: i32, x: f32, y: f32, z: f32) { invoke_ignore!(0xE54BF2CE6C7D23A9, ropeId, p1, x, y, z) }
	pub fn _0x9C24846D0A4A2776(p0: Any) { invoke_ignore!(0x9C24846D0A4A2776, p0) }
	pub fn _0x0CB16D05E03FB525(p0: Any) { invoke_ignore!(0x0CB16D05E03FB525, p0) }
	pub fn _0xF27F1A8DE4F50A1B(p0: Any, p1: Any, p2: Any, p3: Any, p4: Any, p5: Any, p6: Any) { invoke_ignore!(0xF27F1A8DE4F50A1B, p0, p1, p2, p3, p4, p5, p6) }
	pub fn _0x21D0890D88DFB0B0(ropeId: i32, p1: bool, p2: f32, p3: f32, p4: f32, p5: f32, p6: f32, p7: f32, p8: f32, p9: f32, p10: i32) { invoke_ignore!(0x21D0890D88DFB0B0, ropeId, p1, p2, p3, p4, p5, p6, p7, p8, p9, p10) }
	pub fn ATTACH_ENTITIES_TO_ROPE(ropeId: i32, entity1: Entity, entity2: Entity, ent1X: f32, ent1Y: f32, ent1Z: f32, ent2X: f32, ent2Y: f32, ent2Z: f32, length: f32, alwaysZero1: i32, alwaysZero2: i32, boneName1: & CStr, boneName2: & CStr, p14: bool, boneId1: i32, boneId2: i32, alwaysZero3: i32, alwaysZero4: i32, p19: bool, p20: bool) { invoke_ignore!(0x3D95EC8B6D940AC3, ropeId, entity1, entity2, ent1X, ent1Y, ent1Z, ent2X, ent2Y, ent2Z, length, alwaysZero1, alwaysZero2, boneName1, boneName2, p14, boneId1, boneId2, alwaysZero3, alwaysZero4, p19, p20) }
	pub fn _ATTACH_ENTITIES_TO_ROPE_2(ropeId: i32, entity1: Entity, entity2: Entity, ent1X: f32, ent1Y: f32, ent1Z: f32, ent2X: f32, ent2Y: f32, ent2Z: f32, boneName1: & CStr, boneName2: & CStr) { invoke_ignore!(0x462FF2A432733A44, ropeId, entity1, entity2, ent1X, ent1Y, ent1Z, ent2X, ent2Y, ent2Z, boneName1, boneName2) }
	pub fn _ATTACH_ENTITES_TO_ROPE_3(ropeId: i32, entity1: Entity, entity2: Entity, p3: f32, p4: f32, p5: f32, p6: f32, p7: f32, p8: f32, p9: Any, p10: Any) { invoke_ignore!(0xE9CD9A67834985A7, ropeId, entity1, entity2, p3, p4, p5, p6, p7, p8, p9, p10) }
	pub fn _0x69C810B72291D831(p0: Any, p1: Any, p2: Any, p3: Any, p4: Any, p5: Any, p6: Any) { invoke_ignore!(0x69C810B72291D831, p0, p1, p2, p3, p4, p5, p6) }
	pub fn _0xB7469CB9AC3C0FD4(p0: Any, p1: Any, p2: Any, p3: Any, p4: Any, p5: Any, p6: Any, p7: Any) { invoke_ignore!(0xB7469CB9AC3C0FD4, p0, p1, p2, p3, p4, p5, p6, p7) }
	pub fn _0xC64E7A62632AD2FE(ropeId: i32, p1: Any, p2: Any, p3: Any, p4: Any, p5: Any, p6: Any, p7: Any) { invoke_ignore!(0xC64E7A62632AD2FE, ropeId, p1, p2, p3, p4, p5, p6, p7) }
	pub fn _IS_ROPE_ATTACHED_TO_ENTITY(ropeId: i32, entity: Entity) -> bool { invoke!(0x9B4F7E3E4F9C77B3, ropeId, entity) }
	pub fn DETACH_ROPE_FROM_ENTITY(ropeId: i32, entity: Entity) { invoke_ignore!(0xBCF3026912A8647D, ropeId, entity) }
	pub fn _HITCH_HORSE(horse: Ped, x: f32, y: f32, z: f32) { invoke_ignore!(0x06AADE17334F7A40, horse, x, y, z) }
	pub fn _UNHITCH_HORSE(horse: Ped) { invoke_ignore!(0x0348469DAA17576C, horse) }
	pub fn _0x6EA0E93CFFA472CC(p0: Any) { invoke_ignore!(0x6EA0E93CFFA472CC, p0) }
	pub fn _0xBDDA142759307528(p0: Any) { invoke_ignore!(0xBDDA142759307528, p0) }
	pub fn _0x32F4DBFDFCCCC735(p0: Any, p1: Any, p2: Any) { invoke_ignore!(0x32F4DBFDFCCCC735, p0, p1, p2) }
	pub fn _0xF8CA39D5C0D1D9A1(p0: Any, p1: Any) { invoke_ignore!(0xF8CA39D5C0D1D9A1, p0, p1) }
	pub fn _0xEAF529446488EB18(p0: Any) { invoke_ignore!(0xEAF529446488EB18, p0) }
	pub fn _0x31160EC47E7C9549(p0: Any, p1: Any) { invoke_ignore!(0x31160EC47E7C9549, p0, p1) }
	pub fn _0x5E981C764DF33117(p0: Any, p1: Any) { invoke_ignore!(0x5E981C764DF33117, p0, p1) }
	pub fn ROPE_SET_UPDATE_ORDER(ropeId: i32, p1: Any) { invoke_ignore!(0xDC57A637A20006ED, ropeId, p1) }
	pub fn _0xFB9153A54AC713E8(ropeId: i32, p1: bool) { invoke_ignore!(0xFB9153A54AC713E8, ropeId, p1) }
	pub fn _0xD699E688B49C0FD2(ropeId: i32, p1: f32, p2: f32, p3: f32, p4: bool) { invoke_ignore!(0xD699E688B49C0FD2, ropeId, p1, p2, p3, p4) }
	pub fn _0xBB3E9B073E66C3C9(ropeId: i32, p1: bool, p2: bool, p3: bool, p4: bool) { invoke_ignore!(0xBB3E9B073E66C3C9, ropeId, p1, p2, p3, p4) }
	pub fn _0x522FA3F490E2F7AC(ropeId: i32, p1: Any, p2: Any) { invoke_ignore!(0x522FA3F490E2F7AC, ropeId, p1, p2) }
	pub fn _0x3900491C0D61ED4B(p0: Any, p1: Any) { invoke_ignore!(0x3900491C0D61ED4B, p0, p1) }
	pub fn _0xC89E7410A93AC19A(ropeId: i32, p1: f32) { invoke_ignore!(0xC89E7410A93AC19A, ropeId, p1) }
	pub fn _0x1D97DA8ACB5D2582(ropeId: i32, p1: i32) { invoke_ignore!(0x1D97DA8ACB5D2582, ropeId, p1) }
	pub fn _CREATE_ROPE_WINDING_ABILITY(ropeId: i32, p1: & CStr, ropeModelType: & CStr, length: f32, p4: bool) { invoke_ignore!(0x3C6490D940FF5D0B, ropeId, p1, ropeModelType, length, p4) }
	pub fn GET_ROPE_LAST_VERTEX_COORD(ropeId: i32) -> Vector3 { invoke!(0x21BB0FBD3E217C2D, ropeId) }
	pub fn GET_ROPE_VERTEX_COORD(ropeId: i32, vertex: i32) -> Vector3 { invoke!(0xEA61CA8E80F09E4D, ropeId, vertex) }
	pub fn START_ROPE_WINDING(ropeId: i32) { invoke_ignore!(0x1461C72C889E343E, ropeId) }
	pub fn STOP_ROPE_WINDING(ropeId: i32) { invoke_ignore!(0xCB2D4AB84A19AA7C, ropeId) }
	pub fn START_ROPE_UNWINDING_FRONT(ropeId: i32) { invoke_ignore!(0x538D1179EC1AA9A9, ropeId) }
	pub fn STOP_ROPE_UNWINDING_FRONT(ropeId: i32) { invoke_ignore!(0xFFF3A50779EFBBB3, ropeId) }
	pub fn _START_ROPE_UNWINDING_BACK(ropeId: i32) { invoke_ignore!(0x00F611A794A3C36E, ropeId) }
	pub fn _STOP_ROPE_UNWINDING_BACK(ropeId: i32) { invoke_ignore!(0x10DAA76CB8A201A1, ropeId) }
	pub fn _0x461FCBDEB4D06717(ropeId: i32, p1: bool) { invoke_ignore!(0x461FCBDEB4D06717, ropeId, p1) }
	pub fn _0x423C6B1F3786D28B(p0: Any, p1: Any) { invoke_ignore!(0x423C6B1F3786D28B, p0, p1) }
	pub fn _0x76BAD9D538BCA1AA(ropeId: i32, p1: f32) { invoke_ignore!(0x76BAD9D538BCA1AA, ropeId, p1) }
	pub fn _0xB40EA9E0D2E2F7F3(ropeId: i32, p1: f32) { invoke_ignore!(0xB40EA9E0D2E2F7F3, ropeId, p1) }
	pub fn _ROPE_GET_FORCED_LENGTH(ropeId: i32) -> f32 { invoke!(0x3D69537039F8D824, ropeId) }
	pub fn _0x751DF00EEFF122E3(p0: Any) { invoke_ignore!(0x751DF00EEFF122E3, p0) }
	pub fn ROPE_FORCE_LENGTH(ropeId: i32, length: f32) { invoke_ignore!(0xD009F759A723DB1B, ropeId, length) }
	pub fn _0x8D59079C37C21D78(ropeId: i32, p1: f32) { invoke_ignore!(0x8D59079C37C21D78, ropeId, p1) }
	pub fn _0x814D453FCFDF119F(p0: Any, p1: Any, p2: Any) { invoke_ignore!(0x814D453FCFDF119F, p0, p1, p2) }
	pub fn _0x1FC92BDBA1106BD2(ropeId: i32, p1: f32) { invoke_ignore!(0x1FC92BDBA1106BD2, ropeId, p1) }
	pub fn _0xDEDE679ED29DD4E7(ropeId: i32, p1: bool) { invoke_ignore!(0xDEDE679ED29DD4E7, ropeId, p1) }
	pub fn _0xF1EA2A881EB7F2CD(ropeId: i32, p1: bool) { invoke_ignore!(0xF1EA2A881EB7F2CD, ropeId, p1) }
	pub fn _0x5A989B7EE3672A56(p0: Any, p1: Any) { invoke_ignore!(0x5A989B7EE3672A56, p0, p1) }
	pub fn _0x483D4E917B0D35A9(p0: Any, p1: Any) { invoke_ignore!(0x483D4E917B0D35A9, p0, p1) }
	pub fn _ROPE_GET_BREAKER_OF_ROPE(ropeId: i32) -> Player { invoke!(0xEE360CFC80C8B2BC, ropeId) }
	pub fn SET_DAMPING(entity: Entity, vertex: i32, value: f32) { invoke_ignore!(0xEEA3B200A6FEB65B, entity, vertex, value) }
	pub fn ACTIVATE_PHYSICS(entity: Entity) { invoke_ignore!(0x710311ADF0E20730, entity) }
	pub fn BREAK_ENTITY_GLASS(entity: Entity, p1: f32, p2: f32, p3: f32, p4: f32, p5: f32, p6: f32, p7: f32, p8: f32, p9: Any, p10: bool) { invoke_ignore!(0x2E648D16F6E308F3, entity, p1, p2, p3, p4, p5, p6, p7, p8, p9, p10) }
	pub fn _0x8EEDFD8921389928(p0: Any, p1: Any, p2: Any, p3: Any, p4: Any, p5: Any, p6: Any, p7: Any, p8: Any, p9: Any) { invoke_ignore!(0x8EEDFD8921389928, p0, p1, p2, p3, p4, p5, p6, p7, p8, p9) }
	pub fn SET_DISABLE_BREAKING(object: Object, toggle: bool) { invoke_ignore!(0x5CEC1A84620E7D5B, object, toggle) }
	pub fn SET_DISABLE_FRAG_DAMAGE(object: Object, toggle: bool) { invoke_ignore!(0x01BA3AED21C16CFB, object, toggle) }
	pub fn _0x5BD7457221CC5FF4(p0: Any, p1: Any) { invoke_ignore!(0x5BD7457221CC5FF4, p0, p1) }
}
pub mod PLAYER {
	use std::ffi::CStr;
	use crate::core::scripthook::native::*;
	use crate::core::types::*;

	pub fn GET_PLAYER_PED(player: Player) -> Ped { invoke!(0x275F255ED201B937, player) }
	pub fn _GET_PLAYER_PED_2(player: Player) -> Ped { invoke!(0x5EBE38A20BC51C27, player) }
	pub fn _0x325434C68358D282(toggle: bool) { invoke_ignore!(0x325434C68358D282, toggle) }
	pub fn GET_PLAYER_PED_SCRIPT_INDEX(player: Player) -> Ped { invoke!(0x5C880F9056D784C8, player) }
	pub fn SET_PLAYER_MODEL(player: Player, modelHash: Hash, p2: bool) { invoke_ignore!(0xED40380076A31506, player, modelHash, p2) }
	pub fn _NETWORK_HAS_PLAYER_VALID_PED(player: Player) -> bool { invoke!(0x0760D6F70EBCC05C, player) }
	pub fn GET_PLAYER_TEAM(player: Player) -> i32 { invoke!(0xB464EB6A40C7975B, player) }
	pub fn SET_PLAYER_TEAM(player: Player, team: i32, bRestrictToThisScript: bool) { invoke_ignore!(0xE8DD8536F01DE600, player, team, bRestrictToThisScript) }
	pub fn GET_PLAYER_NAME(player: Player) -> *const char { invoke!(0x7124FD9AC0E01BA0, player) }
	pub fn _FORMAT_PLAYER_NAME_STRING(string: & CStr) -> *const char { invoke!(0x5B6193813E03E4E9, string) }
	pub fn GET_WANTED_LEVEL_RADIUS(p0: i32) -> f32 { invoke!(0x80B00EB26D9521C7, p0) }
	pub fn GET_WANTED_LEVEL_THRESHOLD(wantedLevel: i32) -> i32 { invoke!(0x1B1A3B358F7D8F07, wantedLevel) }
	pub fn SET_PLAYER_WANTED_LEVEL(player: Player, wantedLevel: i32, disableNoMission: bool) { invoke_ignore!(0x384D4765395E006C, player, wantedLevel, disableNoMission) }
	pub fn IS_PLAYER_WANTED_LEVEL_GREATER(player: Player, wantedLevel: i32) -> bool { invoke!(0xE1C0AD4C24324C36, player, wantedLevel) }
	pub fn CLEAR_PLAYER_WANTED_LEVEL(player: Player) { invoke_ignore!(0x4E4B996C928C7AA6, player) }
	pub fn IS_PLAYER_DEAD(player: Player) -> bool { invoke!(0x2E9C3FCB6798F397, player) }
	pub fn SET_PLAYER_CONTROL(player: Player, toggle: bool, flags: i32, bPreventHeadingChange: bool) { invoke_ignore!(0x4D51E59243281D80, player, toggle, flags, bPreventHeadingChange) }
	pub fn GET_PLAYER_WANTED_LEVEL(player: Player) -> i32 { invoke!(0xABC532F9098BFD9D, player) }
	pub fn SET_MAX_WANTED_LEVEL(maxWantedLevel: i32) { invoke_ignore!(0x28A4BD2CEE236E19, maxWantedLevel) }
	pub fn _SET_MAX_WANTED_LEVEL_2(maxWantedLevel: i32) { invoke_ignore!(0xEA6DE0CD15AECBE2, maxWantedLevel) }
	pub fn SET_POLICE_RADAR_BLIPS(toggle: bool) { invoke_ignore!(0x6FD7DD6B63F2820E, toggle) }
	pub fn IS_PLAYER_PLAYING(player: Player) -> bool { invoke!(0xBFFB35986CAAE58C, player) }
	pub fn SET_EVERYONE_IGNORE_PLAYER(player: Player, toggle: bool) { invoke_ignore!(0x34630A768925B852, player, toggle) }
	pub fn GET_IS_PLAYER_UI_PROMPT_ACTIVE(player: Player, p1: i32) -> bool { invoke!(0x51BEA356B1C60225, player, p1) }
	pub fn _MODIFY_PLAYER_UI_PROMPT(player: Player, promptType: i32, promptMode: i32, disabled: bool) { invoke_ignore!(0x0751D461F06E41CE, player, promptType, promptMode, disabled) }
	pub fn _GET_PLAYER_UI_PROMPT_IS_DISABLED(player: Player, promptType: i32, promptMode: i32) -> bool { invoke!(0x6614F9039BD31931, player, promptType, promptMode) }
	pub fn _MODIFY_PLAYER_UI_PROMPT_FOR_PED(player: Player, ped: Ped, promptType: i32, promptMode: i32, enabled: bool) { invoke_ignore!(0xA3DB37EDF9A74635, player, ped, promptType, promptMode, enabled) }
	pub fn _GET_PLAYER_UI_PROMPT_FOR_PED_IS_ENABLED(player: Player, ped: Ped, promptType: i32, promptMode: i32) -> bool { invoke!(0xEA8F168A76A0B9BC, player, ped, promptType, promptMode) }
	pub fn _0x93624B36E8851B42(player: Player) { invoke_ignore!(0x93624B36E8851B42, player) }
	pub fn _0x9073EC5456651A90(p0: Any, p1: Any) { invoke_ignore!(0x9073EC5456651A90, p0, p1) }
	pub fn _0x2E67707BEC52CA4B(p0: Any) { invoke_ignore!(0x2E67707BEC52CA4B, p0) }
	pub fn SET_ALL_RANDOM_PEDS_FLEE(player: Player, toggle: bool) { invoke_ignore!(0xE705309B8C6445A4, player, toggle) }
	pub fn SET_ALL_RANDOM_PEDS_FLEE_THIS_FRAME(player: Player) { invoke_ignore!(0xD5C198A62F1DEB0A, player) }
	pub fn SET_ALL_NEUTRAL_RANDOM_PEDS_FLEE_THIS_FRAME(player: Player) { invoke_ignore!(0x16752DAA7E6D3F72, player) }
	pub fn _0x1D256EED194F5B58(p0: Any) { invoke_ignore!(0x1D256EED194F5B58, p0) }
	pub fn _0x5B7B97E99F84138B(p0: Any) -> Any { invoke!(0x5B7B97E99F84138B, p0) }
	pub fn _SET_DISABLE_PLAYER_WANTED_LEVEL(player: Player, disable: bool) { invoke_ignore!(0x8674D138391FFB1B, player, disable) }
	pub fn _GET_WANTED_LEVEL_MULTIPLIER() -> f32 { invoke!(0xA82964B9D8D6A983) }
	pub fn SET_WANTED_LEVEL_MULTIPLIER(multiplier: f32) { invoke_ignore!(0xD7FA719CB54866C2, multiplier) }
	pub fn RESET_WANTED_LEVEL_DIFFICULTY(player: Player) { invoke_ignore!(0x062D14F18E8B0CAE, player) }
	pub fn UPDATE_WANTED_POSITION_THIS_FRAME(player: Player) { invoke_ignore!(0xD0B0B044112BF424, player) }
	pub fn SUPPRESS_WITNESSES_CALLING_POLICE_THIS_FRAME(player: Player) { invoke_ignore!(0x96722257E5381E00, player) }
	pub fn REPORT_POLICE_SPOTTED_PLAYER(player: Player) { invoke_ignore!(0xCBCCF73FFA69CC6B, player) }
	pub fn SET_LAW_RESPONSE_DELAY_OVERRIDE(p0: f32) { invoke_ignore!(0xD2DFC9CCA5596A11, p0) }
	pub fn RESET_LAW_RESPONSE_DELAY_OVERRIDE() { invoke_ignore!(0x5CE5CACC01D0F985) }
	pub fn CAN_PLAYER_START_MISSION(player: Player) -> bool { invoke!(0x2DF170B1185AF777, player) }
	pub fn IS_PLAYER_READY_FOR_CUTSCENE(player: Player) -> bool { invoke!(0xAA67BCB0097F2FA3, player) }
	pub fn IS_PLAYER_TARGETTING_ENTITY(player: Player, entity: Entity, p2: bool) -> bool { invoke!(0x27F89FDC16688A7A, player, entity, p2) }
	pub fn GET_PLAYER_TARGET_ENTITY(player: Player, entity: &mut Entity) -> bool { invoke!(0xAE663DDD99C8A670, player, entity) }
	pub fn _0x927861B2C08DBEA5(player: Player) -> bool { invoke!(0x927861B2C08DBEA5, player) }
	pub fn _IS_PLAYER_FREE_FOCUSING(player: Player) -> bool { invoke!(0x1A51BFE60708E482, player) }
	pub fn GET_PLAYER_INTERACTION_TARGET_ENTITY(player: Player, outEntity: &mut Entity, p2: bool, p3: bool) -> bool { invoke!(0x3EE1F7A8C32F24E1, player, outEntity, p2, p3) }
	pub fn _0xBEA3A6E5F5F79A6F(p0: Any, p1: Any) -> Any { invoke!(0xBEA3A6E5F5F79A6F, p0, p1) }
	pub fn _IS_PLAYER_IN_SCOPE(player: Player) -> bool { invoke!(0x04D7F33640662FA2, player) }
	pub fn IS_PLAYER_FREE_AIMING(player: Player) -> bool { invoke!(0x936F967D4BE1CE9D, player) }
	pub fn IS_PLAYER_FREE_AIMING_AT_ENTITY(player: Player, entity: Entity) -> bool { invoke!(0x8C67C11C68713D25, player, entity) }
	pub fn GET_ENTITY_PLAYER_IS_FREE_AIMING_AT(player: Player, entity: &mut Entity) -> bool { invoke!(0xA6817C110B830EAD, player, entity) }
	pub fn _0x3DAABE78A23694BC(p0: Any, p1: Any) { invoke_ignore!(0x3DAABE78A23694BC, p0, p1) }
	pub fn _0x7AE93C45EC14A166(player: Player, ped: &mut Ped) -> bool { invoke!(0x7AE93C45EC14A166, player, ped) }
	pub fn SET_PLAYER_LOCKON_RANGE_OVERRIDE(player: Player, range: f32) { invoke_ignore!(0x3A3CD06597388322, player, range) }
	pub fn SET_PLAYER_CAN_BE_HASSLED_BY_GANGS(player: Player, toggle: bool) { invoke_ignore!(0xC7FE774412046825, player, toggle) }
	pub fn SET_PLAYER_CAN_USE_COVER(player: Player, toggle: bool) { invoke_ignore!(0x5EDA520F7A3BAF4E, player, toggle) }
	pub fn _0xD1A70C1E8D1031FE(p0: Any, p1: Any) { invoke_ignore!(0xD1A70C1E8D1031FE, p0, p1) }
	pub fn _0xACA45DDCEF6071C4(player: Player, p1: bool) { invoke_ignore!(0xACA45DDCEF6071C4, player, p1) }
	pub fn _0xA0C683284DF027C7(player: Player, p1: i32, enable: bool) { invoke_ignore!(0xA0C683284DF027C7, player, p1, enable) }
	pub fn _SET_PLAYER_CAN_MERCY_KILL(player: Player, toggle: bool) { invoke_ignore!(0x39363DFD04E91496, player, toggle) }
	pub fn _0x4EC8BE63B8A5D4EF(player: Player, p1: i32) { invoke_ignore!(0x4EC8BE63B8A5D4EF, player, p1) }
	pub fn GET_MAX_WANTED_LEVEL() -> i32 { invoke!(0xD04CFAD1E2B7984A) }
	pub fn IS_PLAYER_TARGETTING_ANYTHING(player: Player) -> bool { invoke!(0x4605C66E0F935F83, player) }
	pub fn RESTORE_PLAYER_STAMINA(player: Player, p1: f32) { invoke_ignore!(0xC41F4B6E23FE6A4A, player, p1) }
	pub fn _0x8591EE69CC3ED257(player: Player, toggle: bool) { invoke_ignore!(0x8591EE69CC3ED257, player, toggle) }
	pub fn GET_PLAYER_GROUP(player: Player) -> i32 { invoke!(0x9BAB31815159ABCF, player) }
	pub fn _0x3D9DA5C9EFD20D88(p0: Any, p1: Any) { invoke_ignore!(0x3D9DA5C9EFD20D88, p0, p1) }
	pub fn _0x2BEED53B912537D0(p0: Any, p1: Any, p2: Any) { invoke_ignore!(0x2BEED53B912537D0, p0, p1, p2) }
	pub fn _0x908D4B72854C8F62(p0: Any) { invoke_ignore!(0x908D4B72854C8F62, p0) }
	pub fn _0xD1F6B912785BFD35(p0: Any) -> Any { invoke!(0xD1F6B912785BFD35, p0) }
	pub fn _0xC4873B053054C04B(p0: Any, p1: Any, p2: Any, p3: Any, p4: Any, p5: Any, p6: Any, p7: Any) { invoke_ignore!(0xC4873B053054C04B, p0, p1, p2, p3, p4, p5, p6, p7) }
	pub fn _0xCA59808E51FD67C4(p0: Any, p1: Any) { invoke_ignore!(0xCA59808E51FD67C4, p0, p1) }
	pub fn _0xBA5CA1FEB5DE0DF6(p0: Any, p1: Any, p2: Any, p3: Any, p4: Any, p5: Any) { invoke_ignore!(0xBA5CA1FEB5DE0DF6, p0, p1, p2, p3, p4, p5) }
	pub fn _0x0869D499A7848309(p0: Any, p1: Any, p2: Any, p3: Any, p4: Any, p5: Any, p6: Any, p7: Any) { invoke_ignore!(0x0869D499A7848309, p0, p1, p2, p3, p4, p5, p6, p7) }
	pub fn _0xB331D8A73F9D2BDF(player: Player, p1: &mut Any) -> bool { invoke!(0xB331D8A73F9D2BDF, player, p1) }
	pub fn _ADD_PLAYER_AS_FOLLOW_TARGET(player: Player, ped: Ped, p2: f32, p3: f32, followMode: i32, followPriority: i32, p6: bool) { invoke_ignore!(0xAC22AA6DF4D1C1DE, player, ped, p2, p3, followMode, followPriority, p6) }
	pub fn _REMOVE_PLAYER_AS_FOLLOW_TARGET(player: Player, ped: Ped) { invoke_ignore!(0x0C6B89876262A99D, player, ped) }
	pub fn _0x12E09E278C6C29B7(p0: Any) { invoke_ignore!(0x12E09E278C6C29B7, p0) }
	pub fn _0xDD33A82352C4652F(player: Player, ped: Ped, p2: i32) { invoke_ignore!(0xDD33A82352C4652F, player, ped, p2) }
	pub fn _0x1FDA57E8908F2609(player: Player, ped: Ped, useSteerassist: bool) { invoke_ignore!(0x1FDA57E8908F2609, player, ped, useSteerassist) }
	pub fn _0x84481018E668E1B8(player: Player, ped: Ped, p2: Any) { invoke_ignore!(0x84481018E668E1B8, player, ped, p2) }
	pub fn _0x2009F8AB7A5E9D6D(player: Player) -> bool { invoke!(0x2009F8AB7A5E9D6D, player) }
	pub fn _IS_PLAYER_FOLLOWING_TARGET(player: Player, ped: Ped) -> bool { invoke!(0xE24C64D9ADED2EF5, player, ped) }
	pub fn _0xE7F8707269544B29(player: Player, ped: Ped) -> bool { invoke!(0xE7F8707269544B29, player, ped) }
	pub fn _0xE631EAF35828FA67(p0: Any) -> Any { invoke!(0xE631EAF35828FA67, p0) }
	pub fn _0x086549F3B0381CB1(p0: Any, p1: Any) { invoke_ignore!(0x086549F3B0381CB1, p0, p1) }
	pub fn IS_PLAYER_CONTROL_ON(player: Player) -> bool { invoke!(0x7964097FCE4C244B, player) }
	pub fn IS_PLAYER_SCRIPT_CONTROL_ON(player: Player) -> bool { invoke!(0xB78350754157C00F, player) }
	pub fn IS_PLAYER_CLIMBING(player: Player) -> bool { invoke!(0xB8A70C22FD48197A, player) }
	pub fn _0xEBB6E27AC2FF32DA(p0: Any, p1: Any, p2: Any, p3: Any, p4: Any) { invoke_ignore!(0xEBB6E27AC2FF32DA, p0, p1, p2, p3, p4) }
	pub fn _0xB15CD2F9932C9AB5(p0: Any) -> Any { invoke!(0xB15CD2F9932C9AB5, p0) }
	pub fn _0x621D1B289CAF5978(player: Player) -> bool { invoke!(0x621D1B289CAF5978, player) }
	pub fn IS_PLAYER_BEING_ARRESTED(player: Player, atArresting: bool) -> bool { invoke!(0xC8183AE963C58374, player, atArresting) }
	pub fn RESET_PLAYER_ARREST_STATE(player: Player) { invoke_ignore!(0x12917931C31F1750, player) }
	pub fn _0xCBB54CC7FFFFAB86(p0: Any, p1: Any, p2: Any, p3: Any) { invoke_ignore!(0xCBB54CC7FFFFAB86, p0, p1, p2, p3) }
	pub fn _0xBED386157F65942C(p0: Any, p1: Any) { invoke_ignore!(0xBED386157F65942C, p0, p1) }
	pub fn _0xDAB6A2FC56B7DE65(p0: Any) -> Any { invoke!(0xDAB6A2FC56B7DE65, p0) }
	pub fn _0x0F4EAF69DA41AF43(p0: Any) -> Any { invoke!(0x0F4EAF69DA41AF43, p0) }
	pub fn _SET_BOUNTY_TARGET(player: Player, target: Player) { invoke_ignore!(0x6ADF821FBF21920E, player, target) }
	pub fn _CLEAR_BOUNTY_TARGET(player: Player) { invoke_ignore!(0x8F2A81C09DA9124A, player) }
	pub fn GET_PLAYERS_LAST_VEHICLE() -> Vehicle { invoke!(0x2F96E7720B0B19EA) }
	pub fn GET_PLAYER_INDEX() -> Player { invoke!(0x47E385B0D957C8D4) }
	pub fn INT_TO_PLAYERINDEX(value: i32) -> Player { invoke!(0x748B3A65C2604C33, value) }
	pub fn INT_TO_PARTICIPANTINDEX(value: i32) -> i32 { invoke!(0x58FF971FC8F2702C, value) }
	pub fn PLAYER_ID() -> Player { invoke!(0x217E9DC48139933D) }
	pub fn PLAYER_PED_ID() -> Ped { invoke!(0x096275889B8E0EE0) }
	pub fn NETWORK_PLAYER_ID_TO_INT() -> i32 { invoke!(0x8A9386F0749A17FA) }
	pub fn HAS_FORCE_CLEANUP_OCCURRED(cleanupFlags: i32) -> bool { invoke!(0xC11469DCA6FC3BB5, cleanupFlags) }
	pub fn FORCE_CLEANUP(cleanupFlags: i32) { invoke_ignore!(0x768C017FB878E4F4, cleanupFlags) }
	pub fn FORCE_CLEANUP_FOR_ALL_THREADS_WITH_THIS_NAME(name: & CStr, cleanupFlags: i32) { invoke_ignore!(0xDAACAF8B687F2353, name, cleanupFlags) }
	pub fn FORCE_CLEANUP_FOR_THREAD_WITH_THIS_ID(id: i32, cleanupFlags: i32) { invoke_ignore!(0xF4C9512A2F0A3031, id, cleanupFlags) }
	pub fn GET_CAUSE_OF_MOST_RECENT_FORCE_CLEANUP() -> i32 { invoke!(0x84E8E29EBD4A46D2) }
	pub fn _SET_PLAYER_MOOD(player: Player, mood: i32) { invoke_ignore!(0x39BED552DB46FFA9, player, mood) }
	pub fn _GET_PLAYER_MOOD(player: Player) -> i32 { invoke!(0x054473164C012699, player) }
	pub fn SET_PLAYER_MAY_ONLY_ENTER_THIS_VEHICLE(player: Player, vehicle: Vehicle) { invoke_ignore!(0xDA35A134038557EC, player, vehicle) }
	pub fn _0xC71D07C96946E263(p0: Any, p1: Any) { invoke_ignore!(0xC71D07C96946E263, p0, p1) }
	pub fn SET_PLAYER_MAY_NOT_ENTER_ANY_VEHICLE(player: Player) { invoke_ignore!(0xBEC463B3A11C909E, player) }
	pub fn IS_SYSTEM_UI_BEING_DISPLAYED() -> bool { invoke!(0x908258B6209E71F7) }
	pub fn _0xD48227263E3D06AE(p0: Any, p1: Any, p2: Any, p3: Any, p4: Any, p5: Any, p6: Any, p7: Any, p8: Any, p9: Any) { invoke_ignore!(0xD48227263E3D06AE, p0, p1, p2, p3, p4, p5, p6, p7, p8, p9) }
	pub fn _0x3946FC742AC305CD(player: Player, ped: Ped, p2: & CStr, x: f32, y: f32, z: f32, targetEntity: Entity, p7: & CStr) { invoke_ignore!(0x3946FC742AC305CD, player, ped, p2, x, y, z, targetEntity, p7) }
	pub fn _0xA28056CD1B04B250(p0: Any, p1: Any, p2: Any, p3: Any, p4: Any, p5: Any, p6: Any, p7: Any, p8: Any, p9: Any, p10: Any) { invoke_ignore!(0xA28056CD1B04B250, p0, p1, p2, p3, p4, p5, p6, p7, p8, p9, p10) }
	pub fn _0xC67A4910425F11F1(player: Player, name: & CStr) { invoke_ignore!(0xC67A4910425F11F1, player, name) }
	pub fn SET_PLAYER_INVINCIBLE(player: Player, toggle: bool) { invoke_ignore!(0xFEBEEBC9CBDF4B12, player, toggle) }
	pub fn GET_PLAYER_INVINCIBLE(player: Player) -> bool { invoke!(0x0CBBCB2CCFA7DC4E, player) }
	pub fn SET_PLAYER_LOCKON(player: Player, toggle: bool) { invoke_ignore!(0x462AA1973CBBA75E, player, toggle) }
	pub fn SET_LOCKON_TO_FRIENDLY_PLAYERS(player: Player, toggle: bool) { invoke_ignore!(0x4A056257802DD3E5, player, toggle) }
	pub fn SET_PLAYER_TARGETING_MODE(targetMode: i32) { invoke_ignore!(0xD66A941F401E7302, targetMode) }
	pub fn _SET_PLAYER_IN_VEHICLE_TARGETING_MODE(targetMode: i32) { invoke_ignore!(0x19B4F71703902238, targetMode) }
	pub fn _0x747257807B8721CE(p0: Any, p1: Any) -> Any { invoke!(0x747257807B8721CE, p0, p1) }
	pub fn _0x8702D9150D9FBB3D(p0: Any, p1: Any) -> Any { invoke!(0x8702D9150D9FBB3D, p0, p1) }
	pub fn _0xCB0B9506BC91E441(p0: Any, p1: Any) { invoke_ignore!(0xCB0B9506BC91E441, p0, p1) }
	pub fn CLEAR_PLAYER_HAS_DAMAGED_AT_LEAST_ONE_PED(player: Player) { invoke_ignore!(0x270B63A641BE32F2, player) }
	pub fn HAS_PLAYER_DAMAGED_AT_LEAST_ONE_PED(player: Player) -> bool { invoke!(0xDA4A4B9B96E20092, player) }
	pub fn CLEAR_PLAYER_HAS_DAMAGED_AT_LEAST_ONE_NON_ANIMAL_PED(player: Player) { invoke_ignore!(0x0361096D6CE4372C, player) }
	pub fn HAS_PLAYER_DAMAGED_AT_LEAST_ONE_NON_ANIMAL_PED(player: Player) -> bool { invoke!(0x16C8D205DD5A2E90, player) }
	pub fn _0xEACEBAAE0A33FB3F(p0: Any) { invoke_ignore!(0xEACEBAAE0A33FB3F, p0) }
	pub fn _0x72AD59F7B7FB6E24(player: Player, p1: i32) -> bool { invoke!(0x72AD59F7B7FB6E24, player, p1) }
	pub fn _0x1A6E84F13C952094(player: Player, p1: i32, p2: &mut Any) -> bool { invoke!(0x1A6E84F13C952094, player, p1, p2) }
	pub fn _SET_PLAYER_DAMAGE_INFO_OVERRIDE(player: Player, damageInfo: & CStr) { invoke_ignore!(0x78B3D19AF6391A55, player, damageInfo) }
	pub fn _0x1F488807BC8E0630(player: Player) { invoke_ignore!(0x1F488807BC8E0630, player) }
	pub fn SET_AIR_DRAG_MULTIPLIER_FOR_PLAYERS_VEHICLE(player: Player, multiplier: f32) { invoke_ignore!(0x5DA6500FE849DA16, player, multiplier) }
	pub fn SET_SWIM_MULTIPLIER_FOR_PLAYER(player: Player, multiplier: f32) { invoke_ignore!(0xBFCEABDE34DA5085, player, multiplier) }
	pub fn _0x73EB2EF2E92D23BF() -> bool { invoke!(0x73EB2EF2E92D23BF) }
	pub fn SET_PLAYER_FORCED_AIM(player: Player, toggle: bool, ped: Ped, p3: i32, p4: bool) { invoke_ignore!(0xD5FCC166AEB2FD0F, player, toggle, ped, p3, p4) }
	pub fn _0x310CE349E0C0EC4B(player: Player, ped: Ped, p2: i32) { invoke_ignore!(0x310CE349E0C0EC4B, player, ped, p2) }
	pub fn DISABLE_PLAYER_FIRING(player: Player, toggle: bool) { invoke_ignore!(0x2970929FD5F9FC89, player, toggle) }
	pub fn _0xEBFF94328FF7A18A(p0: Any, p1: Any) { invoke_ignore!(0xEBFF94328FF7A18A, p0, p1) }
	pub fn _0xF993373285053D77(p0: Any, p1: Any, p2: Any) { invoke_ignore!(0xF993373285053D77, p0, p1, p2) }
	pub fn _0xE956C2340A76272E(p0: Any) -> Any { invoke!(0xE956C2340A76272E, p0) }
	pub fn _ENABLE_CUSTOM_DEADEYE_ABILITY(player: Player, enable: bool) { invoke_ignore!(0x95EE1DEE1DCD9070, player, enable) }
	pub fn _0xDE6C85975F9D4894(p0: Any) -> Any { invoke!(0xDE6C85975F9D4894, p0) }
	pub fn _0xBBA140062B15A8AC(player: Player) { invoke_ignore!(0xBBA140062B15A8AC, player) }
	pub fn _SPECIAL_ABILITY_SET_DISABLED(player: Player, disabled: bool) { invoke_ignore!(0xAE637BB8EF017875, player, disabled) }
	pub fn _IS_SPECIAL_ABILITY_ACTIVE(player: Player) -> bool { invoke!(0xB16223CB7DA965F0, player) }
	pub fn _MODIFY_INFINITE_TRAIL_VISION(player: Player, toggle: bool) { invoke_ignore!(0x28A13BF6B05C3D83, player, toggle) }
	pub fn _SPECIAL_ABILITY_SET_EAGLE_EYE_DISABLED(player: Player) { invoke_ignore!(0xC0B21F235C02139C, player) }
	pub fn _SPECIAL_ABILITY_RESTORE_BY_AMOUNT(player: Player, amount: f32, p2: i32, p3: i32, p4: i32) { invoke_ignore!(0x51345AE20F22C261, player, amount, p2, p3, p4) }
	pub fn _0xFA437FA0738C370C(player: Player, p1: f32, p2: i32, p3: i32, p4: i32) { invoke_ignore!(0xFA437FA0738C370C, player, p1, p2, p3, p4) }
	pub fn _SPECIAL_ABILITY_RESTORE_OUTER_RING(player: Player, amount: f32) { invoke_ignore!(0x2498035289B5688F, player, amount) }
	pub fn _GET_PLAYER_REQUIRED_DEAD_EYE_AMOUNT(player: Player) -> f32 { invoke!(0x811A748B1BE231BA, player) }
	pub fn _SPECIAL_ABILITY_GET_AMOUNT_CACHED(player: Player) -> f32 { invoke!(0x029884FB65821B07, player) }
	pub fn _SPECIAL_ABILITY_DRAIN_BY_AMOUNT(player: Player, amount: f32, p2: Any) { invoke_ignore!(0x200114E99552462B, player, amount, p2) }
	pub fn _SPECIAL_ABILITY_START_RESTORE(player: Player, p1: i32, p2: bool) { invoke_ignore!(0x1D77B47AFA584E90, player, p1, p2) }
	pub fn _SET_SPECIAL_ABILITY_MULTIPLIER(player: Player, multiplier: f32) { invoke_ignore!(0x5A498FCA232F71E1, player, multiplier) }
	pub fn _GET_PLAYER_SPECIAL_ABILITY_MULTIPLIER(player: Player) -> f32 { invoke!(0xAB3773E7AA1E9DCC, player) }
	pub fn _SET_SPECIAL_ABILITY_TYPE(player: Player, type_: i32) { invoke_ignore!(0x00BA333DA05ADC23, player, type_) }
	pub fn _0x22B3CABEDDB538B2(player: Player, p1: f32) { invoke_ignore!(0x22B3CABEDDB538B2, player, p1) }
	pub fn _SET_SPECIAL_ABILITY_DURATION_COST(player: Player, durationCost: f32) { invoke_ignore!(0xB783F75940B23014, player, durationCost) }
	pub fn _SET_SPECIAL_ABILITY_DISABLE_TIMER(player: Player, timer: f32) { invoke_ignore!(0xC0B1C05B313693D1, player, timer) }
	pub fn _0x57D9991DC1334151(p0: Any) -> Any { invoke!(0x57D9991DC1334151, p0) }
	pub fn _0x21091B4BEB6376EE(p0: Any) -> Any { invoke!(0x21091B4BEB6376EE, p0) }
	pub fn _SET_SPECIAL_ABILITY_ACTIVATION_COST(player: Player, activationCost: f32, p2: i32) { invoke_ignore!(0xAE4BCC79C587EBBF, player, activationCost, p2) }
	pub fn _0x4D1699543B1C023C(player: Player, p1: f32) { invoke_ignore!(0x4D1699543B1C023C, player, p1) }
	pub fn _GET_PLAYER_DEAD_EYE_METER_LEVEL(player: Player, p1: bool) -> f32 { invoke!(0x3A6AE4EEE30370FE, player, p1) }
	pub fn _GET_PLAYER_DEAD_EYE(player: Player) -> f32 { invoke!(0xA81D24AE0AF99A5E, player) }
	pub fn _GET_PLAYER_CACHED_DEAD_EYE_AMOUNT(player: Player) -> f32 { invoke!(0xDF66A37936D5F3D9, player) }
	pub fn _GET_PLAYER_MAX_DEAD_EYE(player: Player, p1: Any) -> f32 { invoke!(0x592F58BC4D2A2CF3, player, p1) }
	pub fn _GET_PLAYER_HEALTH(player: Player) -> f32 { invoke!(0x0317C947D062854E, player) }
	pub fn _GET_PLAYER_STAMINA(player: Player) -> f32 { invoke!(0x0FF421E467373FCF, player) }
	pub fn _SET_PLAYER_STAT_FLAG_HASH(player: Player, p1: Hash) { invoke_ignore!(0x768E81AE285A4B67, player, p1) }
	pub fn _SET_USED_ITEM_EFFECT(health: f32, stamina: f32, deadeye: f32, healthCore: i32, staminaCore: i32, deadeyeCore: i32) { invoke_ignore!(0x0E1DB1F8F5B561DC, health, stamina, deadeye, healthCore, staminaCore, deadeyeCore) }
	pub fn _0x08E22898A6AF4905(p0: Any, p1: Any) { invoke_ignore!(0x08E22898A6AF4905, p0, p1) }
	pub fn _0xBEFED69CE8317F91(p0: Any) -> Any { invoke!(0xBEFED69CE8317F91, p0) }
	pub fn _ENABLE_EAGLEEYE(player: Player, enable: bool) { invoke_ignore!(0xA63FCAD3A6FEC6D2, player, enable) }
	pub fn _IS_SECONDARY_SPECIAL_ABILITY_ENABLED(player: Player) -> bool { invoke!(0xE022CC1B545F1D9F, player) }
	pub fn _SECONDARY_SPECIAL_ABILITY_SET_ACTIVE(player: Player) { invoke_ignore!(0x1710BC33CFB83634, player) }
	pub fn _SECONDARY_SPECIAL_ABILITY_SET_DISABLED(player: Player, disabled: bool) { invoke_ignore!(0x64FF4BF9AF59E139, player, disabled) }
	pub fn _IS_SECONDARY_SPECIAL_ABILITY_ACTIVE(player: Player) -> bool { invoke!(0x45AB66D02B601FA7, player) }
	pub fn _0x107F2A66E1C4C83A(p0: Any) { invoke_ignore!(0x107F2A66E1C4C83A, p0) }
	pub fn START_PLAYER_TELEPORT(player: Player, x: f32, y: f32, z: f32, heading: f32, p5: bool, p6: bool, p7: bool, p8: bool) { invoke_ignore!(0xDF8822C55EDDA65B, player, x, y, z, heading, p5, p6, p7, p8) }
	pub fn _0x2C2D287748E8E9B7(p0: bool) { invoke_ignore!(0x2C2D287748E8E9B7, p0) }
	pub fn UPDATE_PLAYER_TELEPORT(player: Player) -> bool { invoke!(0xC39DCE4672CBFBC1, player) }
	pub fn STOP_PLAYER_TELEPORT() { invoke_ignore!(0x0858B86146601BE8) }
	pub fn IS_PLAYER_TELEPORT_ACTIVE() -> bool { invoke!(0x085EEAEB8783FEB5) }
	pub fn GET_PLAYER_CURRENT_STEALTH_NOISE(player: Player) -> f32 { invoke!(0xD7ECC25E176ECBA5, player) }
	pub fn SET_PLAYER_HEALTH_RECHARGE_MULTIPLIER(player: Player, regenRate: f32) { invoke_ignore!(0x8899C244EBCF70DE, player, regenRate) }
	pub fn _GET_PLAYER_HEALTH_RECHARGE_MULTIPLIER(player: Player) -> f32 { invoke!(0x22CD23BB0C45E0CD, player) }
	pub fn _SET_PLAYER_HEALTH_RECHARGE_TIME_MODIFIER(player: Player, modifier: f32) { invoke_ignore!(0x535ED4605F89AB6E, player, modifier) }
	pub fn SET_PLAYER_STAMINA_RECHARGE_MULTIPLIER(player: Player, multiplier: f32) { invoke_ignore!(0xFECA17CF3343694B, player, multiplier) }
	pub fn _GET_PLAYER_STAMINA_RECHARGE_MULTIPLIER(player: Player) -> f32 { invoke!(0x617D3494AD58200F, player) }
	pub fn _SET_PLAYER_STAMINA_SPRINT_DEPLETION_MULTIPLIER(player: Player, multiplier: f32) { invoke_ignore!(0xBBADFB5E5E5766FB, player, multiplier) }
	pub fn _GET_PLAYER_STAMINA_DEPLETION_MULTIPLIER(player: Player) -> f32 { invoke!(0x68A0389E0718AC8F, player) }
	pub fn _SET_PED_ACTIVE_PLAYER_HORSE(player: Player, horse: Ped) { invoke_ignore!(0x8FBF9EDB378CCB8C, player, horse) }
	pub fn _GET_ACTIVE_HORSE_FOR_PLAYER(player: Player) -> Ped { invoke!(0x46FA0AE18F4C7FA9, player) }
	pub fn _SET_PED_AS_SADDLE_HORSE_FOR_PLAYER(player: Player, mount: Ped) { invoke_ignore!(0xD2CB0FB0FDCB473D, player, mount) }
	pub fn _GET_SADDLE_HORSE_FOR_PLAYER(player: Player) -> Ped { invoke!(0xB48050D326E9A2F3, player) }
	pub fn SET_PED_AS_TEMP_PLAYER_HORSE(player: Player, horse: Ped) -> bool { invoke!(0x227B06324234FB09, player, horse) }
	pub fn _GET_TEMP_PLAYER_HORSE(player: Player) -> Ped { invoke!(0xD3F7445CEA2E5035, player) }
	pub fn _0x77B0B6D17A3AC9AA(p0: Any, p1: Any) { invoke_ignore!(0x77B0B6D17A3AC9AA, p0, p1) }
	pub fn _SET_PLAYER_MOUNT_STATE_ACTIVE(player: Player, active: bool) { invoke_ignore!(0xDF93973251FB2CA5, player, active) }
	pub fn _0x694FFA4308060CD1(p0: Any, p1: Any) { invoke_ignore!(0x694FFA4308060CD1, p0, p1) }
	pub fn BOOST_PLAYER_HORSE_SPEED_FOR_TIME(player: Player, speedBoost: f32, duration: i32) { invoke_ignore!(0x09C28F828EE674FA, player, speedBoost, duration) }
	pub fn SET_PLAYER_WEAPON_DAMAGE_MODIFIER(player: Player, modifier: f32) { invoke_ignore!(0x94D529F7B73D7A85, player, modifier) }
	pub fn SET_PLAYER_WEAPON_DEFENSE_MODIFIER(player: Player, modifier: f32) { invoke_ignore!(0xD15CC2D493160BE3, player, modifier) }
	pub fn _0x818241B3EDA84191(player: Player, p1: bool) { invoke_ignore!(0x818241B3EDA84191, player, p1) }
	pub fn SET_PLAYER_MELEE_WEAPON_DAMAGE_MODIFIER(player: Player, modifier: f32) { invoke_ignore!(0xE4CB5A3F18170381, player, modifier) }
	pub fn _SET_PLAYER_EXPLOSIVE_WEAPON_DAMAGE_MODIFIER(player: Player, modifier: f32) { invoke_ignore!(0x2D3ACE3DE0A2B622, player, modifier) }
	pub fn _0x83C989D5B5B5B466(p0: Any, p1: Any) { invoke_ignore!(0x83C989D5B5B5B466, p0, p1) }
	pub fn _0x03B4B759A8990505(p0: Any) -> Any { invoke!(0x03B4B759A8990505, p0) }
	pub fn _0x67659A8F248E0141(p0: Any, p1: Any) { invoke_ignore!(0x67659A8F248E0141, p0, p1) }
	pub fn _SET_RECEIVED_HORSEBACK_DAMAGE_DECREASE(player: Player, damageDecrease: f32) { invoke_ignore!(0xB427911EA6DFFEF3, player, damageDecrease) }
	pub fn _SET_AI_PLAYER_DEFENSE_MODIFIER_AGAINST_AI(player: Player, modifier: f32) { invoke_ignore!(0x914071FF93AF2692, player, modifier) }
	pub fn _GET_AI_PLAYER_DEFENSE_MODIFIER_AGAINST_AI(player: Player) -> f32 { invoke!(0x2E78D822208E740A, player) }
	pub fn _0x19B2C7A6C34FAD54(p0: Any, p1: Any) -> Any { invoke!(0x19B2C7A6C34FAD54, p0, p1) }
	pub fn _0x9422743A5BA50E10(p0: Any) -> Any { invoke!(0x9422743A5BA50E10, p0) }
	pub fn _SET_PLAYER_DEFENSE_MODIFIER(player: Player, weaponDefenseMod: f32, meleeDefenseMod: f32) { invoke_ignore!(0x497A6539BB0E8787, player, weaponDefenseMod, meleeDefenseMod) }
	pub fn _SET_PLAYER_DEFENSE_TYPE_MODIFIER(player: Player, type_: i32, defenseModifier: f32) { invoke_ignore!(0x93F499CAE53FCD05, player, type_, defenseModifier) }
	pub fn SET_PLAYER_WEAPON_TYPE_DAMAGE_MODIFIER(player: Player, weaponHash: Hash, damageModifier: f32) { invoke_ignore!(0xD04AD186CE8BB129, player, weaponHash, damageModifier) }
	pub fn _GET_PLAYER_WEAPON_DAMAGE(player: Player, weaponHash: Hash) -> f32 { invoke!(0xFE0304050261442C, player, weaponHash) }
	pub fn _0x5C2E5E3CAEEB1F58(p0: Any, p1: Any, p2: Any) { invoke_ignore!(0x5C2E5E3CAEEB1F58, p0, p1, p2) }
	pub fn _SET_PLAYER_WEAPON_GROUP_DAMAGE_MODIFIER(player: Player, weaponGroup: Hash, modifier: f32) { invoke_ignore!(0xFC79DCC94D0A5897, player, weaponGroup, modifier) }
	pub fn _SET_PLAYER_WEAPON_GROUP_AS_INSTANT_KILL(player: Player, weaponGroup: Hash, toggle: bool) { invoke_ignore!(0x59F0AFF3E0A1B019, player, weaponGroup, toggle) }
	pub fn _SET_PLAYER_TRAMPLE_DAMAGE_MODIFIER(player: Player, modifier: f32) { invoke_ignore!(0xAF341032E97FB061, player, modifier) }
	pub fn _SET_PLAYER_LASSO_DAMAGE_PER_SECOND(player: Player, damage: f32) { invoke_ignore!(0x43F50A7CD2482156, player, damage) }
	pub fn _SET_PLAYER_TOTAL_ACCURACY_MODIFIER(player: Player, accuracy: f32) { invoke_ignore!(0x967FF5BC0CFE6D26, player, accuracy) }
	pub fn _SET_PLAYER_LOCAL_ACCURACY_FLOOR_MODIFIER(player: Player, accuracy: f32) { invoke_ignore!(0x4EA69188FBCE6A7D, player, accuracy) }
	pub fn _SET_PLAYER_REMOTE_ACCURACY_FLOOR_MODIFIER(player: Player, accuracy: f32) { invoke_ignore!(0xDEE80FEDFDD43C9B, player, accuracy) }
	pub fn _0x3AD212429E095EFB(p0: Any, p1: Any) { invoke_ignore!(0x3AD212429E095EFB, p0, p1) }
	pub fn SET_PLAYER_NOISE_MULTIPLIER(player: Player, multiplier: f32) { invoke_ignore!(0xB5EC6BDAEBCA454C, player, multiplier) }
	pub fn _0x113EF458AB6CDA67(p0: Any, p1: Any) { invoke_ignore!(0x113EF458AB6CDA67, p0, p1) }
	pub fn SET_PLAYER_SNEAKING_NOISE_MULTIPLIER(player: Player, multiplier: f32) { invoke_ignore!(0x4DE44FA389DCA565, player, multiplier) }
	pub fn SIMULATE_PLAYER_INPUT_GAIT(player: Player, speed: f32, duration: i32, heading: f32, p4: bool, p5: bool) { invoke_ignore!(0xFA0C063C422C4355, player, speed, duration, heading, p4, p5) }
	pub fn RESET_PLAYER_INPUT_GAIT(player: Player) { invoke_ignore!(0x61A2EECAB274829B, player) }
	pub fn SET_PLAYER_SIMULATE_AIMING(player: Player, toggle: bool) { invoke_ignore!(0xE0447DEF81CCDFD2, player, toggle) }
	pub fn SET_PLAYER_CLOTH_PIN_FRAMES(ped: Ped, p1: i32) { invoke_ignore!(0xD0D9317DFEEF9A66, ped, p1) }
	pub fn HAS_PLAYER_BEEN_SPOTTED_IN_STOLEN_VEHICLE(player: Player) -> bool { invoke!(0xC932F57F31EA9152, player) }
	pub fn GET_PLAYER_RECEIVED_BATTLE_EVENT_RECENTLY(player: Player, p1: i32, p2: bool) -> bool { invoke!(0xFB6EB8785F808551, player, p1, p2) }
	pub fn _SET_MOUNT_PROMPT_DISABLED(disabled: bool) { invoke_ignore!(0x5B9813ECF7633FE8, disabled) }
	pub fn IS_PLAYER_RIDING_TRAIN(player: Player) -> bool { invoke!(0x2FB0ACADA6A238DD, player) }
	pub fn _0x9AFCF9FE1884BF62(p0: Any, p1: Any) { invoke_ignore!(0x9AFCF9FE1884BF62, p0, p1) }
	pub fn _0x1E8099F449ABB0BA(p0: Any) -> Any { invoke!(0x1E8099F449ABB0BA, p0) }
	pub fn _GET_DEADEYE_ABILITY_LEVEL(player: Player) -> i32 { invoke!(0xCCE7C695C164C35F, player) }
	pub fn _SET_DEADEYE_ABILITY_LEVEL(player: Player, level: i32) { invoke_ignore!(0xF0FE8E790BFEB5BB, player, level) }
	pub fn _IS_DEADEYE_ABILITY_LOCKED(player: Player, abilityType: i32) -> bool { invoke!(0x8A0643B0B4CA276B, player, abilityType) }
	pub fn _SET_DEADEYE_ABILITY_LOCKED(player: Player, abilityType: i32, toggle: bool) { invoke_ignore!(0x2797B8D66DD0EBB8, player, abilityType, toggle) }
	pub fn _SET_DEADEYE_TAGGING_ENABLED(player: Player, toggle: bool) { invoke_ignore!(0x6B5DDFB967E5073D, player, toggle) }
	pub fn _GET_IS_DEADEYE_TAGGING_ENABLED(player: Player) -> bool { invoke!(0x32348719DCED2969, player) }
	pub fn _0x3C4AE8506638C7E2(p0: Any, p1: Any) { invoke_ignore!(0x3C4AE8506638C7E2, p0, p1) }
	pub fn _0x51139D8C17B16FBC(p0: Any) -> Any { invoke!(0x51139D8C17B16FBC, p0) }
	pub fn _0x8F44EBB3BA8F6D44(p0: Any, p1: Any) { invoke_ignore!(0x8F44EBB3BA8F6D44, p0, p1) }
	pub fn _SET_DEADEYE_TAGGING_CONFIG(player: Player, filter: i32) { invoke_ignore!(0x83FCD6921FC8FD05, player, filter) }
	pub fn _0xE92261BD28C0878F(p0: Any) -> Any { invoke!(0xE92261BD28C0878F, p0) }
	pub fn _SET_DEADEYE_ABILITY_DEPLETION_DELAY(player: Player, delay: f32) { invoke_ignore!(0x870634493CB4372C, player, delay) }
	pub fn _0xA54000D4BFD90BDE(p0: Any) -> Any { invoke!(0xA54000D4BFD90BDE, p0) }
	pub fn _0x6EDB5D08CB03E763(p0: Any, p1: Any) { invoke_ignore!(0x6EDB5D08CB03E763, p0, p1) }
	pub fn _0x27AD7162D3FED01E(p0: Any, p1: Any) -> Any { invoke!(0x27AD7162D3FED01E, p0, p1) }
	pub fn _GET_NUM_MARKED_DEADEYE_TARGETS(player: Player) -> i32 { invoke!(0xCCD9B77F70D31C9D, player) }
	pub fn _0xC93A9A45430D484E(p0: Any) -> Any { invoke!(0xC93A9A45430D484E, p0) }
	pub fn _0x570A13A4CA2799BB(player: Player, p1: bool) { invoke_ignore!(0x570A13A4CA2799BB, player, p1) }
	pub fn _0x3ACAC8832E77BC93(player: Player, p1: bool) { invoke_ignore!(0x3ACAC8832E77BC93, player, p1) }
	pub fn _0x2B12B6FC8B8772AB(player: Player, p1: i32) { invoke_ignore!(0x2B12B6FC8B8772AB, player, p1) }
	pub fn _0xE910932F4B30BE23(player: Player) { invoke_ignore!(0xE910932F4B30BE23, player) }
	pub fn _0x131E294EF60160DF(player: Player, p1: f32, p2: f32, p3: f32, p4: f32, p5: Any) { invoke_ignore!(0x131E294EF60160DF, player, p1, p2, p3, p4, p5) }
	pub fn _0x0E9057A9DA78D0F8(player: Player, bitflag: i32) { invoke_ignore!(0x0E9057A9DA78D0F8, player, bitflag) }
	pub fn _0x263D69767F76059C(player: Player, p1: i32) { invoke_ignore!(0x263D69767F76059C, player, p1) }
	pub fn _REGISTER_EAGLE_EYE_FOR_ENTITY(player: Player, entity: Entity, p2: bool) { invoke_ignore!(0x543DFE14BE720027, player, entity, p2) }
	pub fn _REGISTER_EAGLE_EYE_TRAILS_FOR_ENTITY(player: Player, entity: Entity, p2: Any) { invoke_ignore!(0xAC67098A1E54ABB0, player, entity, p2) }
	pub fn _UNREGISTER_EAGLE_EYE_FOR_ENTITY(player: Player, entity: Entity) { invoke_ignore!(0x9DAE1380CC5C6451, player, entity) }
	pub fn _UNREGISTER_EAGLE_EYE_TRAILS_FOR_ENTITY(player: Player, entity: Entity, p2: Any) { invoke_ignore!(0x9A957912CE2EABD1, player, entity, p2) }
	pub fn _0xE5D3EB37ABC1EB03(player: Player) { invoke_ignore!(0xE5D3EB37ABC1EB03, player) }
	pub fn _IS_EAGLE_EYE_REGISTERED_FOR_ENTITY(player: Player, entity: Entity) -> bool { invoke!(0x0E6846476906C9DD, player, entity) }
	pub fn _0x6852288340B43239(p0: Any, p1: Any) -> Any { invoke!(0x6852288340B43239, p0, p1) }
	pub fn _0xE50A67C33514A390(p0: Any, p1: Any) -> Any { invoke!(0xE50A67C33514A390, p0, p1) }
	pub fn _0xD288E02E364972D2(p0: Any, p1: Any, p2: Any) { invoke_ignore!(0xD288E02E364972D2, p0, p1, p2) }
	pub fn _EAGLE_EYE_DISABLE_TRACKING_TRAIL(entity: Entity, trail: & CStr, p2: Any, p3: Any) { invoke_ignore!(0x40AB73092C95B5F5, entity, trail, p2, p3) }
	pub fn _0x6ECFC621A168424C(entity1: Entity, entity2: Entity, p2: Any, p3: f32) { invoke_ignore!(0x6ECFC621A168424C, entity1, entity2, p2, p3) }
	pub fn _0xDC5E09D012D759C4(entity1: Entity, entity2: Entity, p2: Any) { invoke_ignore!(0xDC5E09D012D759C4, entity1, entity2, p2) }
	pub fn _0x00B156AFEBCC5AE0(p0: Any) { invoke_ignore!(0x00B156AFEBCC5AE0, p0) }
	pub fn _0xC58CE6824E604DEC(p0: Any) { invoke_ignore!(0xC58CE6824E604DEC, p0) }
	pub fn _0x330CA55A3647FA1C(p0: Any, p1: Any) { invoke_ignore!(0x330CA55A3647FA1C, p0, p1) }
	pub fn _0xA62BBAAE67A05BB0(p0: Any) -> Any { invoke!(0xA62BBAAE67A05BB0, p0) }
	pub fn _EAGLE_EYE_SET_COLOR(player: Player, p1: bool, p2: &mut Any) { invoke_ignore!(0x2C41D93F550D5E37, player, p1, p2) }
	pub fn _0x22C8B10802301381(p0: Any, p1: Any) { invoke_ignore!(0x22C8B10802301381, p0, p1) }
	pub fn _EAGLE_EYE_SET_DRAIN_RATE_MODIFIER(player: Player, modifier: f32) { invoke_ignore!(0xE0D6C2A146A5C993, player, modifier) }
	pub fn _0x06E1FB78B1E59CA5(ped: Ped, p1: bool) { invoke_ignore!(0x06E1FB78B1E59CA5, ped, p1) }
	pub fn _EAGLE_EYE_SET_PLUS_FLAG_DISABLED(ped: Ped, disabled: bool) { invoke_ignore!(0xCE285A4413B00B7F, ped, disabled) }
	pub fn _0x3813E11A378958A5(p0: Any) -> Any { invoke!(0x3813E11A378958A5, p0) }
	pub fn _EAGLE_EYE_SET_FOCUS_ON_ASSOCIATED_CLUE_TRAIL(player: Player, linkedWaypointPed: Entity) { invoke_ignore!(0x2AF423D6ECB2C485, player, linkedWaypointPed) }
	pub fn _0x0F9CF06986300875(p0: Any) { invoke_ignore!(0x0F9CF06986300875, p0) }
	pub fn _EAGLE_EYE_SET_TRACKING_UPGRADE(player: Player, p1: f32) { invoke_ignore!(0xDFC85C5199045026, player, p1) }
	pub fn _EAGLE_EYE_SET_TRACKING_UPGRADE_2(player: Player, p1: f32) { invoke_ignore!(0x6FA957D1B55941C1, player, p1) }
	pub fn _0x1DA5C5B0923E1B85(p0: Any) -> Any { invoke!(0x1DA5C5B0923E1B85, p0) }
	pub fn _0xAAED694CE814817F(p0: Any) -> Any { invoke!(0xAAED694CE814817F, p0) }
	pub fn EAGLE_EYE_SET_CUSTOM_ENTITY_TINT(entity: Entity, red: i32, green: i32, blue: i32) { invoke_ignore!(0x62ED71E133B6C9F1, entity, red, green, blue) }
	pub fn _0xBC02B3D151D3859F(entity: Entity, p1: Any) { invoke_ignore!(0xBC02B3D151D3859F, entity, p1) }
	pub fn _EAGLE_EYE_SET_CUSTOM_DISTANCE(entity: Entity, distance: f32) { invoke_ignore!(0x907B16B3834C69E2, entity, distance) }
	pub fn _0xF21C7A3F3FFBA629(player: Player) { invoke_ignore!(0xF21C7A3F3FFBA629, player) }
	pub fn _SET_PLAYER_MANAGE_BUFF_SUPER_JUMP(player: Player, p1: f32) { invoke_ignore!(0x292F0B6EDC82E3A4, player, p1) }
	pub fn _SET_LOCAL_PLAYER_PERSONA_ABILITY_FLAG(flagId: i32, toggle: bool) { invoke_ignore!(0x7146CF430965927C, flagId, toggle) }
	pub fn _SET_PLAYER_MAX_AMMO_OVERRIDE_FOR_AMMO_TYPE(player: Player, ammoType: Hash, amount: i32) { invoke_ignore!(0xE133C1EC5300F740, player, ammoType, amount) }
	pub fn _0xC900A465364A85D6(player: Player) { invoke_ignore!(0xC900A465364A85D6, player) }
	pub fn _0xCFB2EED4FCB7BD77(p0: Any, p1: Any, p2: Any) { invoke_ignore!(0xCFB2EED4FCB7BD77, p0, p1, p2) }
	pub fn _0x2BB8D58E88777499(p0: Any) { invoke_ignore!(0x2BB8D58E88777499, p0) }
	pub fn _0x00EB5A760638DB55(p0: Any, p1: Any, p2: Any) { invoke_ignore!(0x00EB5A760638DB55, p0, p1, p2) }
	pub fn _0x65887EAC535A0B0C(p0: Any) { invoke_ignore!(0x65887EAC535A0B0C, p0) }
	pub fn _SET_WEAPON_DEGRADATION_MODIFIER(player: Player, modifier: f32) { invoke_ignore!(0x11A7FF918EF6BC66, player, modifier) }
	pub fn _SET_BOW_DRAW_REDUCTION_TIME_IN_DEADEYE(player: Player, drawReductionTime: f32) { invoke_ignore!(0xBE0C524970892D41, player, drawReductionTime) }
	pub fn _SET_BOW_STAMINA_DRAIN_SPEED(player: Player, staminaDrain: f32) { invoke_ignore!(0xFE7C9CF376D23342, player, staminaDrain) }
	pub fn _SET_DAMAGE_CLOSE_DISTANCE_BONUS(player: Player, closeRangeLowerBound: f32, closeRangeUpperBound: f32) { invoke_ignore!(0x7761A30432C91297, player, closeRangeLowerBound, closeRangeUpperBound) }
	pub fn _SET_DAMAGE_CLOSE_DISTANCE_BONUS_TOTAL(player: Player, closeDamageBonus: f32) { invoke_ignore!(0x5006C36652D6EC56, player, closeDamageBonus) }
	pub fn _SET_DAMAGE_FAR_DISTANCE_BONUS(player: Player, farRangeLowerBound: f32, farRangeUpperBound: f32) { invoke_ignore!(0xED591CB17C8BA216, player, farRangeLowerBound, farRangeUpperBound) }
	pub fn _SET_DAMAGE_FAR_DISTANCE_BONUS_TOTAL(player: Player, farDamageBonus: f32) { invoke_ignore!(0x1F0E3A4434565F8F, player, farDamageBonus) }
	pub fn _0x6C54E69516CC56BD(p0: Any) -> Any { invoke!(0x6C54E69516CC56BD, p0) }
	pub fn _0x3A8611BD7BDE84F7(p0: Any, p1: Any) { invoke_ignore!(0x3A8611BD7BDE84F7, p0, p1) }
	pub fn _0xC177C827CEFC0AA4(p0: Any, p1: Any) { invoke_ignore!(0xC177C827CEFC0AA4, p0, p1) }
	pub fn _0xBD96185264DDAAEA(p0: Any, p1: Any) { invoke_ignore!(0xBD96185264DDAAEA, p0, p1) }
	pub fn _0x628E742FE1F79C4A(p0: Any, p1: Any) { invoke_ignore!(0x628E742FE1F79C4A, p0, p1) }
	pub fn _SET_PLAYER_INTERACTION_POSITIVE_RESPONSE(player: Player, speech: & CStr) { invoke_ignore!(0xC6366A585659D15C, player, speech) }
	pub fn _SET_PLAYER_INTERACTION_NEGATIVE_RESPONSE(player: Player, speech: & CStr) { invoke_ignore!(0x98CD760DE43B612E, player, speech) }
	pub fn _0x216BC0D3D2E413D2(player: Player, p1: Any) { invoke_ignore!(0x216BC0D3D2E413D2, player, p1) }
	pub fn _0x45EF176B532CA851(p0: Any, p1: Any) { invoke_ignore!(0x45EF176B532CA851, p0, p1) }
	pub fn _0xA342495F93B7B838(p0: Any, p1: Any) { invoke_ignore!(0xA342495F93B7B838, p0, p1) }
	pub fn _0x3BB84F812E052C90(p0: Any) { invoke_ignore!(0x3BB84F812E052C90, p0) }
	pub fn _0x9FC5A003FB76EDBD(p0: Any, p1: Any) { invoke_ignore!(0x9FC5A003FB76EDBD, p0, p1) }
	pub fn _0x0FAF95D71ED67ADE(player: Player, p1: & CStr) { invoke_ignore!(0x0FAF95D71ED67ADE, player, p1) }
	pub fn _0x988C9045531B9FCE(player: Player, p1: & CStr) { invoke_ignore!(0x988C9045531B9FCE, player, p1) }
	pub fn _0x06C3DB00B69D5435(player: Player, p1: & CStr) { invoke_ignore!(0x06C3DB00B69D5435, player, p1) }
	pub fn _0xBB6EA5D59E926095(category: i32, emote: Hash) { invoke_ignore!(0xBB6EA5D59E926095, category, emote) }
	pub fn _0xE1D356F5A66D0FFA(emote: Hash) -> bool { invoke!(0xE1D356F5A66D0FFA, emote) }
	pub fn _0x929DDD5538F3DF1F(p0: Any, p1: Any) { invoke_ignore!(0x929DDD5538F3DF1F, p0, p1) }
	pub fn _0xFA7DAAE3959E6C7B(p0: Any, p1: Any) { invoke_ignore!(0xFA7DAAE3959E6C7B, p0, p1) }
	pub fn _0x9461A8FAB0378E5B(p0: Any, p1: Any) { invoke_ignore!(0x9461A8FAB0378E5B, p0, p1) }
	pub fn _0xCB61A63AA53D7D22(p0: Any, p1: Any) { invoke_ignore!(0xCB61A63AA53D7D22, p0, p1) }
	pub fn _0xCFFC3ECCD7A5CCEB(player: Player, weapon: Hash, p2: bool) { invoke_ignore!(0xCFFC3ECCD7A5CCEB, player, weapon, p2) }
	pub fn _0x76F7E1BCD623A429(p0: Any) { invoke_ignore!(0x76F7E1BCD623A429, p0) }
	pub fn _0x585CE159DB46FADB(p0: Any, p1: Any) { invoke_ignore!(0x585CE159DB46FADB, p0, p1) }
	pub fn _SET_PLAYER_RESET_FLAG(player: Player, playerResetFlag: i32, p2: bool) { invoke_ignore!(0x9F9A829C6751F3C7, player, playerResetFlag, p2) }
	pub fn _GET_PLAYER_RESET_FLAG(player: Player, playerResetFlag: i32) -> bool { invoke!(0xFE691E89C08937B6, player, playerResetFlag) }
	pub fn GET_MOUNT_OWNED_BY_PLAYER(player: Player) -> Ped { invoke!(0xF49F14462F0AE27C, player) }
	pub fn _GET_PLAYER_OWNER_OF_MOUNT(mount: Ped) -> Player { invoke!(0xAD03B03737CE6810, mount) }
	pub fn _SET_PLAYER_OWNS_MOUNT(player: Player, mount: Ped) { invoke_ignore!(0xE6D4E435B56D5BD0, player, mount) }
	pub fn _GET_VEHICLE_OWNED_BY_PLAYER(player: Player) -> Vehicle { invoke!(0xB9050A97594C8832, player) }
	pub fn _GET_PLAYER_OWNER_OF_VEHICLE(vehicle: Vehicle) -> Player { invoke!(0x7C803BDC8343228D, vehicle) }
	pub fn _SET_PLAYER_OWNS_VEHICLE(player: Player, vehicle: Vehicle) { invoke_ignore!(0xD0E02AA618020D17, player, vehicle) }
	pub fn _GET_PLAYER_HUNTING_WAGON(player: Player) -> Vehicle { invoke!(0x5CA6BBD4A7D8145E, player) }
	pub fn _SET_PLAYER_HUNTING_WAGON(player: Player, wagon: Vehicle) { invoke_ignore!(0x6A4404BDFA62CE2C, player, wagon) }
	pub fn _0x9044835BE9D9DBFE(p0: Any, p1: Any) { invoke_ignore!(0x9044835BE9D9DBFE, p0, p1) }
	pub fn GET_DISCOVERABLE_NAME_HASH_AND_TYPE_FOR_ENTITY(entity: Entity, type_: &mut Hash) -> Hash { invoke!(0x0139637A3BFF8B6D, entity, type_) }
	pub fn _GET_CONSTRUCTED_DISCOVERED_CHARACTER_NAME(p0: Hash, model: bool, outfit: bool) -> Hash { invoke!(0x8E84119A23C16623, p0, model, outfit) }
	pub fn GET_TARGET_CHARACTER_NAME_SCRIPT_OVERRIDE_HASH(ped: Ped) -> Hash { invoke!(0x0335106F3ACABBED, ped) }
	pub fn GET_TARGET_CHARACTER_NAME_SCRIPT_OVERRIDE_RAW_STRING(ped: Ped) -> *const char { invoke!(0x755E08680F21EF30, ped) }
	pub fn GET_TARGET_CHARACTER_NAME_FOR_LOCAL_PLAYER(ped: Ped) -> Hash { invoke!(0x36E3D8B5A6552FE8, ped) }
	pub fn _0xDA9D7BE231FE865F(p0: Any, p1: Any, p2: Any) -> Any { invoke!(0xDA9D7BE231FE865F, p0, p1, p2) }
	pub fn _SET_PLAYER_HAS_DISCOVERED_CHARACTER_NAME_SP(player: Player, p1: i32, discoveryHash: Hash) { invoke_ignore!(0x946D46CD6DFB9742, player, p1, discoveryHash) }
	pub fn _GET_HAS_PLAYER_DISCOVERED_CHARACTER_NAME_SP(player: Player, p1: i32, discoveryHash: Hash) -> bool { invoke!(0x0772F87D7B07719A, player, p1, discoveryHash) }
	pub fn _0xCDDD4B74660E2335(p0: Any, p1: Any, p2: Any) { invoke_ignore!(0xCDDD4B74660E2335, p0, p1, p2) }
	pub fn _0x77E83C315A3B31CA(p0: Any) { invoke_ignore!(0x77E83C315A3B31CA, p0) }
	pub fn _SET_PLAYER_HAS_DISCOVERED_CHARACTER_NAME_MP(discoveryHash: Hash) { invoke_ignore!(0x7C32191D9FB2BDEA, discoveryHash) }
	pub fn GET_HAS_PLAYER_DISCOVERED_CHARACTER_NAME_MP(discoveryHash: Hash) -> bool { invoke!(0x354F689C4FFAAB37, discoveryHash) }
	pub fn _MODIFY_PLAYER_DISCOVERED_CHARACTER_NAME_MP_SET_UNDISCOVERED(discoveryHash: Hash) { invoke_ignore!(0xFB0E622B401884D3, discoveryHash) }
	pub fn _SET_SHOW_INFO_CARD(player: Player, showingInfoCard: bool) { invoke_ignore!(0xDC68829BB3F37023, player, showingInfoCard) }
	pub fn _0x4DBC4873707E8FD6(p0: Any, p1: Any, p2: Any, p3: Any) { invoke_ignore!(0x4DBC4873707E8FD6, p0, p1, p2, p3) }
	pub fn _0xCEDC16930526F728(p0: Any) { invoke_ignore!(0xCEDC16930526F728, p0) }
	pub fn _0x14E57F88BA0A07FC(location: Hash) { invoke_ignore!(0x14E57F88BA0A07FC, location) }
	pub fn _0x2E1ABE627C95ED9B() -> Any { invoke!(0x2E1ABE627C95ED9B) }
	pub fn _0x497A18F8F88AA9D8() { invoke_ignore!(0x497A18F8F88AA9D8) }
	pub fn _0x4F0D2256AAE94EDA(p0: i32) { invoke_ignore!(0x4F0D2256AAE94EDA, p0) }
	pub fn _SET_LOCKON_FOCUS_FIRE_VFX(player: Player, p1: & CStr) { invoke_ignore!(0x5F8E0303C229C84B, player, p1) }
	pub fn _0x0B7803F6F7BB43E0() -> Any { invoke!(0x0B7803F6F7BB43E0) }
	pub fn _0xC74EB3F2EC169F6B(p0: Any) -> Any { invoke!(0xC74EB3F2EC169F6B, p0) }
	pub fn _0x3B296934DB026873(p0: Any, p1: Any) { invoke_ignore!(0x3B296934DB026873, p0, p1) }
	pub fn SET_MIN_TIME_BEFORE_HORSE_BUCKING(mount: Ped, iMinBuckTime: i32) { invoke_ignore!(0x506CE71FB6E8CF5E, mount, iMinBuckTime) }
	pub fn _0xF4CB347D7B5EB0FD() -> Any { invoke!(0xF4CB347D7B5EB0FD) }
	pub fn _0xCD7CA3013FD12749(p0: Any, p1: Any) { invoke_ignore!(0xCD7CA3013FD12749, p0, p1) }
	pub fn _FORCE_REST_SCENARIO(toggle: bool) { invoke_ignore!(0xE5A3DD2FF84E1A4B, toggle) }
	pub fn _0x57028FD99886F6F9() -> bool { invoke!(0x57028FD99886F6F9) }
	pub fn _0x35A33783EC3C3448(p0: Any) { invoke_ignore!(0x35A33783EC3C3448, p0) }
	pub fn _0x39D8D7082BC34B72(p0: Any) { invoke_ignore!(0x39D8D7082BC34B72, p0) }
	pub fn _0x1AD8AD999C27F44A(p0: Any) { invoke_ignore!(0x1AD8AD999C27F44A, p0) }
}
pub mod POPULATION {
	use std::ffi::CStr;
	use crate::core::scripthook::native::*;
	use crate::core::types::*;

	pub fn GET_NUM_MODELS_IN_POPULATION_SET(popSetHash: Hash) -> i32 { invoke!(0xA1E3171ED0E47564, popSetHash) }
	pub fn GET_PED_MODEL_NAME_IN_POPULATION_SET(popSetHash: Hash, index: i32) -> Hash { invoke!(0x3EAFA1C533B7139E, popSetHash, index) }
	pub fn GET_RANDOM_MODEL_FROM_POPULATION_SET(popSetHash: Hash, flags: i32, p2: Hash, p3: bool, p4: bool, x: f32, y: f32, z: f32) -> Hash { invoke!(0x6B12ED8C77E8567B, popSetHash, flags, p2, p3, p4, x, y, z) }
	pub fn _CREATE_POPZONE_FROM_VOLUME(volume: Volume) -> PopZone { invoke!(0x9AC1C64FE46B6D09, volume) }
	pub fn _DELETE_SCRIPT_POPZONE(popZone: PopZone) { invoke_ignore!(0xA6E6A66FC4CA4224, popZone) }
	pub fn _IS_POPZONE_VALID(popZone: PopZone) -> bool { invoke!(0xA5BD585005EFCAD4, popZone) }
	pub fn SET_POPZONE_POPULATION_SET(popZone: PopZone, populationSetHash: Hash) { invoke_ignore!(0x3E6A49D9B519E85C, popZone, populationSetHash) }
	pub fn _0x7E6BC0B94F5928F0(popZone: PopZone, p1: i32, p2: i32) { invoke_ignore!(0x7E6BC0B94F5928F0, popZone, p1, p2) }
	pub fn _0x578E2FA64E847C60(popZone: PopZone, p1: i32) { invoke_ignore!(0x578E2FA64E847C60, popZone, p1) }
	pub fn _0x08892122769770D5(popZone: PopZone, p1: bool) { invoke_ignore!(0x08892122769770D5, popZone, p1) }
	pub fn _0x0F1861101C9A9944(popZone: PopZone, p1: bool) { invoke_ignore!(0x0F1861101C9A9944, popZone, p1) }
	pub fn SET_SPAWNER_INFO_PRIORITY(p0: Hash, p1: Hash, priority: i32) { invoke_ignore!(0x60CDE717A6D47769, p0, p1, priority) }
	pub fn CLEAR_SPAWNER_INFO_PRIORITY(p0: Hash, p1: Hash) { invoke_ignore!(0x217A54DE2D200305, p0, p1) }
	pub fn _0x638FCFC6042A9473(p0: Any, p1: Any) { invoke_ignore!(0x638FCFC6042A9473, p0, p1) }
	pub fn _ADD_AMBIENT_AVOIDANCE_RESTRICTION(volume: Volume, includeFlags: i32, excludeFlags: i32, p3: Hash, p4: Hash, p5: Hash, p6: i32) { invoke_ignore!(0xB56D41A694E42E86, volume, includeFlags, excludeFlags, p3, p4, p5, p6) }
	pub fn _REMOVE_AMBIENT_AVOIDANCE_RESTRICTION(volume: Volume) { invoke_ignore!(0x74C2B3DC0B294102, volume) }
	pub fn _ADD_AMBIENT_SPAWN_RESTRICTION(volume: Volume, includeFlags: i32, excludeFlags: i32, p3: Hash, p4: Hash, p5: Hash, p6: i32) { invoke_ignore!(0x18262CAFEBB5FBE1, volume, includeFlags, excludeFlags, p3, p4, p5, p6) }
	pub fn _REMOVE_AMBIENT_SPAWN_RESTRICTION(volume: Volume) { invoke_ignore!(0xA1CFB35069D23C23, volume) }
	pub fn _0x2161278C6322F740(includeFlags: i32, excludeFlags: i32, p2: i32, p3: Hash, p4: i32, volume: Volume) { invoke_ignore!(0x2161278C6322F740, includeFlags, excludeFlags, p2, p3, p4, volume) }
	pub fn _0xF45E46DEECF7DF6E(bitFlag: i32, p1: Any, p2: Any, p3: Any, p4: Any) { invoke_ignore!(0xF45E46DEECF7DF6E, bitFlag, p1, p2, p3, p4) }
	pub fn _0x8EC7CD701F872F87(p0: Any, p1: Any, p2: Any, p3: Any, p4: Any, p5: Any) { invoke_ignore!(0x8EC7CD701F872F87, p0, p1, p2, p3, p4, p5) }
	pub fn _0xC4533E3E87125C9E(p0: Any) { invoke_ignore!(0xC4533E3E87125C9E, p0) }
	pub fn _SET_PED_SHOULD_IGNORE_AVOIDANCE_VOLUMES(ped: Ped, p1: i32) { invoke_ignore!(0xF74E134F40192884, ped, p1) }
	pub fn _0xDBBF12EA7C1029B2(p0: Any, p1: Any) { invoke_ignore!(0xDBBF12EA7C1029B2, p0, p1) }
	pub fn _0x247F86595D396344(p0: Any) { invoke_ignore!(0x247F86595D396344, p0) }
	pub fn _0x324AB2A68AD8AEE5() { invoke_ignore!(0x324AB2A68AD8AEE5) }
	pub fn DISABLE_AMBIENT_ROAD_POPULATION(unk: bool) { invoke_ignore!(0xC6DCC2A3A8825C85, unk) }
	pub fn ENABLE_AMBIENT_ROAD_POPULATION() { invoke_ignore!(0xBC90BDF4E5228EA1) }
	pub fn _0x2660E7720EDC4BD0(p0: Any, p1: Any, p2: Any) { invoke_ignore!(0x2660E7720EDC4BD0, p0, p1, p2) }
	pub fn _GET_RANDOM_FISH_TYPE_FOR_LOCATION() -> Hash { invoke!(0x595478B3BBC3076D) }
	pub fn _0xEC116EDB683AD479(p0: bool) { invoke_ignore!(0xEC116EDB683AD479, p0) }
}
pub mod POSSE {
	use std::ffi::CStr;
	use crate::core::scripthook::native::*;
	use crate::core::types::*;

	pub fn _0xC086FF658B2E51DB() -> Any { invoke!(0xC086FF658B2E51DB) }
	pub fn _0xC086FF658B2E51DA(p0: Any) -> Any { invoke!(0xC086FF658B2E51DA, p0) }
	pub fn _0xC087FF658B2E51DA(p0: Any, p1: Any) -> Any { invoke!(0xC087FF658B2E51DA, p0, p1) }
	pub fn POSSE_GET_POSSE_MEMBERSHIP_COUNT() -> i32 { invoke!(0xC088FF658B2E51DA) }
	pub fn _0xC089FF658B2E51DA(p0: Any, p1: Any) -> Any { invoke!(0xC089FF658B2E51DA, p0, p1) }
	pub fn _0xC08AFF658B2E51DA(p0: Any) { invoke_ignore!(0xC08AFF658B2E51DA, p0) }
	pub fn _0xC08BFF658B2E51DA(p0: Any) -> Any { invoke!(0xC08BFF658B2E51DA, p0) }
	pub fn _0xC08AFF658B2E51DB(p0: Any) { invoke_ignore!(0xC08AFF658B2E51DB, p0) }
	pub fn _0xC08CFF658B2E51DA(p0: Any, p1: Any) -> Any { invoke!(0xC08CFF658B2E51DA, p0, p1) }
	pub fn _0xC09CFF658B2E51DA(p0: Any, p1: Any, p2: Any) -> Any { invoke!(0xC09CFF658B2E51DA, p0, p1, p2) }
	pub fn _0xC08DEF658B2E51DA(p0: Any) -> Any { invoke!(0xC08DEF658B2E51DA, p0) }
	pub fn _0xC08DFF658B2E51DA() -> Any { invoke!(0xC08DFF658B2E51DA) }
	pub fn _0xC08DFF658B2E51DB(p0: Any) -> Any { invoke!(0xC08DFF658B2E51DB, p0) }
	pub fn _0xC08EFF658B2E51DB(p0: Any, p1: Any) -> Any { invoke!(0xC08EFF658B2E51DB, p0, p1) }
	pub fn _0xC08FFF658B2E51DA() -> Any { invoke!(0xC08FFF658B2E51DA) }
	pub fn _0xC08FFF658B2E51DB(p0: Any) -> Any { invoke!(0xC08FFF658B2E51DB, p0) }
	pub fn _0xC084FF658B2E61DA(p0: Any) -> Any { invoke!(0xC084FF658B2E61DA, p0) }
	pub fn _0xC084FF658B2E71DA(p0: Any, p1: Any, p2: Any) -> Any { invoke!(0xC084FF658B2E71DA, p0, p1, p2) }
	pub fn _0xC084FF658B2E81DA(p0: Any, p1: Any, p2: Any) -> Any { invoke!(0xC084FF658B2E81DA, p0, p1, p2) }
	pub fn _0xC084FF658B2E52DA(p0: Any) -> Any { invoke!(0xC084FF658B2E52DA, p0) }
	pub fn _0xC084FF658B2E53DA() -> Any { invoke!(0xC084FF658B2E53DA) }
	pub fn _0xC084FF658B2E54DA(p0: Any) -> Any { invoke!(0xC084FF658B2E54DA, p0) }
	pub fn _0xC084FF658B2E55DA(p0: Any, p1: Any) -> Any { invoke!(0xC084FF658B2E55DA, p0, p1) }
	pub fn _0xC484FF658B2E55DA(p0: Any) { invoke_ignore!(0xC484FF658B2E55DA, p0) }
	pub fn _0xC584FF658B2E55DA(p0: Any) { invoke_ignore!(0xC584FF658B2E55DA, p0) }
	pub fn _0xC684FF658B2E55DA(p0: Any) { invoke_ignore!(0xC684FF658B2E55DA, p0) }
	pub fn _0xC184FF658B2E55DA(p0: Any, p1: Any) -> Any { invoke!(0xC184FF658B2E55DA, p0, p1) }
	pub fn _0xC284FF658B2E55DA(p0: Any, p1: Any, p2: Any) -> Any { invoke!(0xC284FF658B2E55DA, p0, p1, p2) }
	pub fn _0xC394FF658B2E55DA(p0: Any, p1: Any, p2: Any, p3: Any) -> Any { invoke!(0xC394FF658B2E55DA, p0, p1, p2, p3) }
	pub fn _0xC07CFF658B2E51DA(p0: Any, p1: Any) -> Any { invoke!(0xC07CFF658B2E51DA, p0, p1) }
	pub fn _0xC06CFF658B2E51DA(p0: Any, p1: Any, p2: Any) -> Any { invoke!(0xC06CFF658B2E51DA, p0, p1, p2) }
}
pub mod PROPSET {
	use std::ffi::CStr;
	use crate::core::scripthook::native::*;
	use crate::core::types::*;

	pub fn _REQUEST_PROP_SET(hash: Hash) -> bool { invoke!(0xF3DE57A46D5585E9, hash) }
	pub fn _REQUEST_PROP_SET_2(hash: Hash) -> bool { invoke!(0xE72F591958F3ACAB, hash) }
	pub fn _HAS_PROP_SET_LOADED(hash: Hash) -> bool { invoke!(0x48A88FC684C55FDC, hash) }
	pub fn _HAS_PROP_SET_LOADED_2(hash: Hash) -> bool { invoke!(0xD090ABEF4D6A7D96, hash) }
	pub fn _SET_PROP_SET_AS_NO_LONGER_NEEDED(propSet: PropSet) { invoke_ignore!(0x909E3C7FAE539FB1, propSet) }
	pub fn _DELETE_PROP_SET(propSet: PropSet, p1: bool, p2: bool) { invoke_ignore!(0x58AC173A55D9D7B4, propSet, p1, p2) }
	pub fn _RELEASE_PROP_SET(hash: Hash) -> bool { invoke!(0xB1964A83B345B4AB, hash) }
	pub fn _CREATE_PROP_SET(propsetType: Hash, x: f32, y: f32, z: f32, placementType: i32, heading: f32, zProbe: f32, p7: bool, useVegMod: bool) -> PropSet { invoke!(0xE65C5CBA95F0E510, propsetType, x, y, z, placementType, heading, zProbe, p7, useVegMod) }
	pub fn _CREATE_PROP_SET_2(propsetType: Hash, x: f32, y: f32, z: f32, placementType: i32, heading: f32, zProbe: f32, p7: bool, useVegMod: bool) -> PropSet { invoke!(0x899C97A1CCE7D483, propsetType, x, y, z, placementType, heading, zProbe, p7, useVegMod) }
	pub fn CREATE_PROP_SET_INSTANCE_ATTACHED_TO_ENTITY(hash: Hash, x: f32, y: f32, z: f32, entity: Entity, p5: f32, p6: bool, p7: i32, p8: bool) -> PropSet { invoke!(0x9609DBDDE18FAD8C, hash, x, y, z, entity, p5, p6, p7, p8) }
	pub fn _CREATE_PROP_SET_INSTANCE_ATTACHED_TO_ENTITY_2(hash: Hash, x: f32, y: f32, z: f32, entity: Entity, p5: f32, p6: bool, p7: i32, p8: bool) -> PropSet { invoke!(0xACA7FB30269096D4, hash, x, y, z, entity, p5, p6, p7, p8) }
	pub fn DOES_PROP_SET_EXIST(propSet: PropSet) -> bool { invoke!(0x7DDDCF815E650FF5, propSet) }
	pub fn _DOES_PROP_SET_OF_TYPE_EXIST_NEAR_COORDS(propsetHash: Hash, x: f32, y: f32, z: f32) -> bool { invoke!(0x72068021F498E6E3, propsetHash, x, y, z) }
	pub fn IS_PROP_SET_FULLY_LOADED(propSet: PropSet) -> bool { invoke!(0xF42DB680A8B2A4D9, propSet) }
	pub fn _SET_PROP_SET_VISIBLE(propSet: PropSet, toggle: bool) { invoke_ignore!(0x9D096A5BD02F953E, propSet, toggle) }
	pub fn _IS_PROP_SET_VISIBLE(propSet: PropSet) -> bool { invoke!(0x0CE8AAFE9E433A23, propSet) }
	pub fn _GET_PROP_SET_MODEL(propSet: PropSet) -> Hash { invoke!(0xA6A9712955F53D9C, propSet) }
	pub fn _GET_VEHICLE_PROP_SET_HASH(vehicle: Vehicle) -> Hash { invoke!(0x36F69E7A22655653, vehicle) }
	pub fn _GET_ENTITIES_FROM_PROP_SET(propSet: PropSet, itemSet: ItemSet, model: Hash, p3: bool, p4: bool) -> i32 { invoke!(0x738271B660FE0695, propSet, itemSet, model, p3, p4) }
	pub fn _0xC4B67EF3FD65622D(p0: Any, p1: Any, p2: Any, p3: Any, p4: Any, p5: Any) { invoke_ignore!(0xC4B67EF3FD65622D, p0, p1, p2, p3, p4, p5) }
	pub fn _0x58E0B01D45CA7357(p0: Any) { invoke_ignore!(0x58E0B01D45CA7357, p0) }
	pub fn _SET_PROP_SET_FLAG(propSet: PropSet, flag: i32) { invoke_ignore!(0xC1AB7EEFD3E6EE49, propSet, flag) }
	pub fn _GET_VEHICLE_PROP_SET(vehicle: Vehicle) -> PropSet { invoke!(0xCE2ACD6F602803E5, vehicle) }
	pub fn _DOES_VEHICLE_HAVE_ANY_PROP_SET(vehicle: Vehicle) -> bool { invoke!(0x53784CEA0159439B, vehicle) }
	pub fn _ADD_PROP_SET_FOR_VEHICLE(vehicle: Vehicle, propset: Hash) { invoke_ignore!(0xD80FAF919A2E56EA, vehicle, propset) }
	pub fn _REMOVE_VEHICLE_PROP_SETS(vehicle: Vehicle) { invoke_ignore!(0x3BCF32FF37EA9F1D, vehicle) }
	pub fn _IS_VEHICLE_PROP_SET_LOADED(vehicle: Vehicle) -> bool { invoke!(0x155B2FBE72D7D1D0, vehicle) }
	pub fn _ADD_ADDITIONAL_PROP_SET_FOR_VEHICLE(vehicle: Vehicle, propset: Hash) { invoke_ignore!(0x75F90E4051CC084C, vehicle, propset) }
	pub fn _IS_VEHICLE_PROP_SET_LOADED_ADDITIONAL(vehicle: Vehicle) -> bool { invoke!(0x7264F9CA87A9830B, vehicle) }
	pub fn _GET_VEHICLE_LIGHT_PROP_SET(vehicle: Vehicle) -> PropSet { invoke!(0xA079300AF757FB1A, vehicle) }
	pub fn _DOES_VEHICLE_HAVE_ANY_LIGHT_PROP_SET(vehicle: Vehicle) -> bool { invoke!(0xC9B4B3A36F81FD75, vehicle) }
	pub fn _ADD_LIGHT_PROP_SET_TO_VEHICLE(vehicle: Vehicle, lightPropset: Hash) { invoke_ignore!(0xC0F0417A90402742, vehicle, lightPropset) }
	pub fn _REMOVE_VEHICLE_LIGHT_PROP_SETS(vehicle: Vehicle) { invoke_ignore!(0xE31C0CB1C3186D40, vehicle) }
	pub fn _IS_VEHICLE_LIGHT_PROP_SET_LOADED(vehicle: Vehicle) -> bool { invoke!(0x0790473EEE1977D3, vehicle) }
	pub fn _GET_TRAIN_CARRIAGE_PROP_SET(trainCarriage: Entity) -> PropSet { invoke!(0xCFC0BD09BB1B73FF, trainCarriage) }
	pub fn _HAS_VEHICLE_TRAILER_PROP_SET_LOADED(vehicle: Vehicle, wagonIndex: i32) -> bool { invoke!(0x8F3333F0A6900B3C, vehicle, wagonIndex) }
	pub fn _GET_PROP_SET_AT_COORDS(propsetHash: Hash, x: f32, y: f32, z: f32) -> PropSet { invoke!(0xC061E50F8D299F95, propsetHash, x, y, z) }
}
pub mod QUEUE {
	use std::ffi::CStr;
	use crate::core::scripthook::native::*;
	use crate::core::types::*;

	pub fn _EVENT_QUEUE_IS_EMPTY(hash: Hash) -> bool { invoke!(0x402B5D7D269FF796, hash) }
	pub fn _EVENT_QUEUE_POP(hash: Hash) { invoke_ignore!(0xD87DF294B049211D, hash) }
}
pub mod RECORDING {
	use std::ffi::CStr;
	use crate::core::scripthook::native::*;
	use crate::core::types::*;

	pub fn REPLAY_PREVENT_RECORDING_THIS_FRAME() { invoke_ignore!(0xA8C44C13419634F2) }
}
pub mod REPLAY {
	use std::ffi::CStr;
	use crate::core::scripthook::native::*;
	use crate::core::types::*;

	pub fn REPLAY_SYSTEM_HAS_REQUESTED_A_SCRIPT_CLEANUP() -> bool { invoke!(0x0F838D47DE58EDB2) }
	pub fn SET_SCRIPTS_HAVE_CLEANED_UP_FOR_REPLAY_SYSTEM() { invoke_ignore!(0x57C6525034E76EB0) }
	pub fn OPEN_VIDEO_EDITOR() -> bool { invoke!(0xB3F2829907403C13) }
	pub fn CLOSE_VIDEO_EDITOR(p0: Any) -> bool { invoke!(0xCEEC64BD27A59312, p0) }
	pub fn IS_VIDEO_EDITOR_RUNNING() -> bool { invoke!(0x9EEB007317FA3B9C) }
}
pub mod SCRIPTS {
	use std::ffi::CStr;
	use crate::core::scripthook::native::*;
	use crate::core::types::*;

	pub fn _SET_PLAYER_BIT_AT_INDEX(value: &mut Any, bitIndex: i32) { invoke_ignore!(0x31010318BA9897AC, value, bitIndex) }
	pub fn _CLEAR_PLAYER_BIT_AT_INDEX(value: &mut Any, bitIndex: i32) { invoke_ignore!(0xD426E2E3288469D6, value, bitIndex) }
	pub fn _0xE4ABE20DCE7C7CFE(p0: Any, p1: Any, p2: Any) { invoke_ignore!(0xE4ABE20DCE7C7CFE, p0, p1, p2) }
	pub fn _0xFFDDF802279BE128(p0: Any, p1: Any, p2: Any) { invoke_ignore!(0xFFDDF802279BE128, p0, p1, p2) }
	pub fn _0x64F765D9A1F8F02C(p0: &mut Any, p1: &mut Any, p2: &mut Any) { invoke_ignore!(0x64F765D9A1F8F02C, p0, p1, p2) }
	pub fn _SET_ALL_PLAYER_BITS(value: &mut Any) { invoke_ignore!(0x20F4CB76689ACDBC, value) }
	pub fn _CLEAR_ALL_PLAYER_BITS(value: &mut Any) { invoke_ignore!(0xDE544B7EC0C187CC, value) }
	pub fn _IS_PLAYER_BIT_SET_AT_INDEX(value: &mut Any, bitIndex: i32) -> bool { invoke!(0x72B2E00C9BAC6789, value, bitIndex) }
	pub fn _IS_ANY_PLAYER_BIT_SET(playerBits: &mut i32) -> bool { invoke!(0x179A6F0EE2E79026, playerBits) }
	pub fn GET_BLOCK_OF_PLAYER_BITS(value: &mut Any, p1: i32) -> i32 { invoke!(0xFA3B530A5CC693D5, value, p1) }
	pub fn SET_BLOCK_OF_PLAYER_BITS(value: &mut Any, p1: i32, p2: i32) { invoke_ignore!(0xC6DFB8C04C86D5A5, value, p1, p2) }
	pub fn COUNT_PLAYER_BITS(value: &mut Any) -> i32 { invoke!(0x462C687BEA254BD9, value) }
	pub fn _0x1BDB5A07307F6929(p0: Any, p1: Any) { invoke_ignore!(0x1BDB5A07307F6929, p0, p1) }
	pub fn _0x1C5EB3C27F7508CB(p0: Any, p1: Any) { invoke_ignore!(0x1C5EB3C27F7508CB, p0, p1) }
	pub fn _0x42A429CDFED6D99D(p0: Any, p1: Any, p2: Any) { invoke_ignore!(0x42A429CDFED6D99D, p0, p1, p2) }
	pub fn _0x5827BE85A87B073D(p0: Any) { invoke_ignore!(0x5827BE85A87B073D, p0) }
	pub fn _0x0A79C81C418F5D38(p0: Any, p1: Any) -> Any { invoke!(0x0A79C81C418F5D38, p0, p1) }
	pub fn _0xA88E1D7FA1E20080(p0: Any) -> Any { invoke!(0xA88E1D7FA1E20080, p0) }
	pub fn COUNT_PARTICIPANT_BITS(value: &mut Any) -> i32 { invoke!(0x2F050A3FF8738245, value) }
	pub fn REQUEST_SCRIPT(scriptName: & CStr) { invoke_ignore!(0x46ED607DDD40D7FE, scriptName) }
	pub fn SET_SCRIPT_AS_NO_LONGER_NEEDED(scriptName: & CStr) { invoke_ignore!(0x0086D3067E1CFD1C, scriptName) }
	pub fn HAS_SCRIPT_LOADED(scriptName: & CStr) -> bool { invoke!(0xE97BD36574F8B0A6, scriptName) }
	pub fn DOES_SCRIPT_EXIST(scriptName: & CStr) -> bool { invoke!(0x552B171E3F69E5AE, scriptName) }
	pub fn REQUEST_SCRIPT_WITH_NAME_HASH(scriptHash: Hash) { invoke_ignore!(0xF6B9CE3F8D5B9B74, scriptHash) }
	pub fn SET_SCRIPT_WITH_NAME_HASH_AS_NO_LONGER_NEEDED(scriptHash: Hash) { invoke_ignore!(0x50723A1567C8361E, scriptHash) }
	pub fn HAS_SCRIPT_WITH_NAME_HASH_LOADED(scriptHash: Hash) -> bool { invoke!(0xA5D8E0C2F3C7EEBC, scriptHash) }
	pub fn DOES_SCRIPT_WITH_NAME_HASH_EXIST(scriptHash: Hash) -> bool { invoke!(0xA34E89749F628284, scriptHash) }
	pub fn TERMINATE_THREAD(threadId: i32) { invoke_ignore!(0x87ED52AE40EA1A52, threadId) }
	pub fn IS_THREAD_ACTIVE(threadId: i32, ignoreKilledState: bool) -> bool { invoke!(0x46E9AE36D8FA6417, threadId, ignoreKilledState) }
	pub fn DOES_THREAD_EXIST(threadId: i32) -> bool { invoke!(0xFF975BC4435A0FA3, threadId) }
	pub fn GET_THREAD_EXISTENCE_DETAILS(threadId: i32, threadExists: &mut bool, hasScriptHandler: &mut bool) { invoke_ignore!(0xD92FA81B64920E85, threadId, threadExists, hasScriptHandler) }
	pub fn _GET_HASH_OF_THREAD(threadId: i32) -> Hash { invoke!(0x724CB89D35B283D0, threadId) }
	pub fn SCRIPT_THREAD_ITERATOR_RESET() { invoke_ignore!(0x39382EB8DCD8684D) }
	pub fn SCRIPT_THREAD_ITERATOR_GET_NEXT_THREAD_ID() -> i32 { invoke!(0x3CE3FB167E837D7C) }
	pub fn _IS_BACKGROUND_SCRIPT(threadId: i32) -> bool { invoke!(0x20B7F69B40C6B755, threadId) }
	pub fn GET_ID_OF_THIS_THREAD() -> i32 { invoke!(0x55525C346BEF6960) }
	pub fn TERMINATE_THIS_THREAD() { invoke_ignore!(0x5E8B6D17FF91CD59) }
	pub fn GET_NUMBER_OF_THREADS_RUNNING_THE_SCRIPT_WITH_THIS_HASH(scriptHash: Hash) -> i32 { invoke!(0x8E34C953364A76DD, scriptHash) }
	pub fn _REQUEST_THREAD_EXIT(threadId: i32) { invoke_ignore!(0x7DE4643157AD646C, threadId) }
	pub fn _REQUEST_THREAD_EXIT_FOR_ALL_THREADS_WITH_THIS_NAME(nameHash: Hash) { invoke_ignore!(0x7423F7835770F619, nameHash) }
	pub fn IS_THREAD_EXIT_REQUESTED() -> bool { invoke!(0x9E4EF615E307FBBE) }
	pub fn _IS_THREAD_EXIT_REQUESTED_FOR_THREAD_WITH_THIS_ID(threadId: i32) -> bool { invoke!(0x30BED53646C86D11, threadId) }
	pub fn _GET_THREAD_EXIT_REASON() -> i32 { invoke!(0x54AE4FDEEFEAB77E) }
	pub fn GET_HASH_OF_THIS_SCRIPT_NAME() -> Hash { invoke!(0xBC2C927F5C264960) }
	pub fn GET_NUMBER_OF_EVENTS(eventGroup: i32) -> i32 { invoke!(0x5CE8DE5909565748, eventGroup) }
	pub fn GET_EVENT_EXISTS(eventGroup: i32, eventType: Hash) -> bool { invoke!(0xC9F59C0A710ECD34, eventGroup, eventType) }
	pub fn GET_EVENT_AT_INDEX(eventGroup: i32, eventIndex: i32) -> Hash { invoke!(0xA85E614430EFF816, eventGroup, eventIndex) }
	pub fn GET_EVENT_DATA(eventGroup: i32, eventIndex: i32, eventData: &mut Any, eventDataSize: i32) -> bool { invoke!(0x57EC5FA4D4D6AFCA, eventGroup, eventIndex, eventData, eventDataSize) }
	pub fn SET_EVENT_FLAG_FOR_DELETION(eventGroup: i32, eventIndex: i32, p2: bool) { invoke_ignore!(0x4768D5252EAEB76F, eventGroup, eventIndex, p2) }
	pub fn TRIGGER_SCRIPT_EVENT(eventGroup: i32, eventData: &mut Any, eventDataSize: i32, scriptMetadataIndex: i32, playerBits: &mut i32) { invoke_ignore!(0x5AE99C571D5BBE5D, eventGroup, eventData, eventDataSize, scriptMetadataIndex, playerBits) }
	pub fn _TRIGGER_SCRIPT_EVENT_2(eventData: &mut Any, eventDataSize: i32, scriptMetadataIndex: i32, threadId: i32) { invoke_ignore!(0x8B61C950A148FFA2, eventData, eventDataSize, scriptMetadataIndex, threadId) }
	pub fn _0xE7282390542F570D(p0: Any) -> Any { invoke!(0xE7282390542F570D, p0) }
	pub fn _0x11B0A0B282FA9B10(p0: bool) { invoke_ignore!(0x11B0A0B282FA9B10, p0) }
	pub fn _0x6F700A4BF7C3331B(p0: bool) { invoke_ignore!(0x6F700A4BF7C3331B, p0) }
	pub fn _0xF9E951A1E5517C06() { invoke_ignore!(0xF9E951A1E5517C06) }
	pub fn _0x76CBCD9EADC00955() { invoke_ignore!(0x76CBCD9EADC00955) }
	pub fn SHUTDOWN_LOADING_SCREEN() { invoke_ignore!(0xFC179D7E8886DADF) }
	pub fn SET_NO_LOADING_SCREEN(toggle: bool) { invoke_ignore!(0x5CB83156AA038F95, toggle) }
	pub fn GET_NO_LOADING_SCREEN() -> bool { invoke!(0x323DAF00687E0F28) }
	pub fn _DISPLAY_LOADING_SCREENS(p0: Hash, p1: Hash, p2: Hash, gamemodeName: & CStr, title: & CStr, subtitle: & CStr) { invoke_ignore!(0x1E5B70E53DB661E5, p0, p1, p2, gamemodeName, title, subtitle) }
	pub fn _0x29FB4CE89472C3CB(p0: Any, p1: Any, p2: f32, p3: f32, p4: & CStr, p5: & CStr, p6: & CStr, p7: i32) { invoke_ignore!(0x29FB4CE89472C3CB, p0, p1, p2, p3, p4, p5, p6, p7) }
	pub fn STOP_DISPLAYING_MP_TRANSITION_LOADING_SCREENS(p0: Any) { invoke_ignore!(0x778D4733E0F2F265, p0) }
	pub fn IS_LOADING_SCREEN_VISIBLE() -> bool { invoke!(0xB54ADBE65D528FCB) }
	pub fn BAIL_TO_LANDING_PAGE(bailCode: i32) { invoke_ignore!(0xBC2C927F5C264243, bailCode) }
	pub fn BAIL_WITH_PASS_THROUGH_PARAMS(params: & CStr) { invoke_ignore!(0xE98204D3C25AE14C, params) }
	pub fn BG_IS_EXITFLAG_SET() -> bool { invoke!(0x2238EC3EC631AB1F) }
	pub fn BG_SET_EXITFLAG_RESPONSE() { invoke_ignore!(0x4858148E3B8A75D0) }
	pub fn BG_START_CONTEXT_HASH(contextHash: Hash) { invoke_ignore!(0x2EB67D564DCC09D5, contextHash) }
	pub fn BG_END_CONTEXT_HASH(contextHash: Hash) { invoke_ignore!(0x6D1431744182CDE8, contextHash) }
	pub fn BG_START_CONTEXT(contextName: & CStr) { invoke_ignore!(0x49BA5678BA040CA7, contextName) }
	pub fn BG_END_CONTEXT(contextName: & CStr) { invoke_ignore!(0x3ABF7BA1C3E2C8CF, contextName) }
	pub fn BG_DOES_LAUNCH_PARAM_EXIST(scriptIndex: i32, p1: & CStr) -> bool { invoke!(0x4AE1DFF337A86FDE, scriptIndex, p1) }
	pub fn BG_GET_LAUNCH_PARAM_VALUE(scriptIndex: i32, p1: & CStr) -> i32 { invoke!(0x55C40B7592BAD213, scriptIndex, p1) }
	pub fn BG_GET_SCRIPT_ID_FROM_NAME_HASH(p0: Hash) -> i32 { invoke!(0x829CD22E043A2577, p0) }
	pub fn _BG_RELOAD_ALL_BACKGROUND_SCRIPTS() { invoke_ignore!(0xBE7D814CFA181B56) }
	pub fn _ACTIVATE_GOAL_CONTEXT(goalContext: Hash) { invoke_ignore!(0x7D654266025E921B, goalContext) }
	pub fn _DEACTIVATE_GOAL_CONTEXT(goalContext: Hash) { invoke_ignore!(0x50B72A754EE64A71, goalContext) }
	pub fn _IS_GOAL_CONTEXT_ACTIVE(goalContext: Hash) -> bool { invoke!(0x7409669C5ED50144, goalContext) }
	pub fn _NET_RPC_GUID_TO_STRING(netRpcGuid: &mut Any) -> *const char { invoke!(0xAC9FF854BD4BA9B5, netRpcGuid) }
	pub fn AWARDS_GET_RESULT_ITEM(rpcGuid: &mut Any, awardHash: Hash, itemIndex: i32, outResultItem: &mut Any) -> bool { invoke!(0xAC8FAB22A914AE34, rpcGuid, awardHash, itemIndex, outResultItem) }
	pub fn _AWARDS_GET_UNLOCK_CLAIM_DATA(rpcGuid: &mut Any, awardHash: Hash, dataIndex: i32, outUnlockData: &mut Any) -> bool { invoke!(0xB9467E41DAB1CF2C, rpcGuid, awardHash, dataIndex, outUnlockData) }
	pub fn _LOOT_GET_RESULT_ITEM(rpcGuid: &mut Any, itemIndex: i32, outResultItem: &mut Any) -> bool { invoke!(0x4293B44A855F82CC, rpcGuid, itemIndex, outResultItem) }
	pub fn _LOOT_GET_LOOT_CLAIM_DATA(rpcGuid: &mut Any, dataIndex: i32, outLootData: &mut Any) -> bool { invoke!(0xF1E9045F5AA9E428, rpcGuid, dataIndex, outLootData) }
	pub fn _STORE_GLOBAL_BLOCK(index: i32) -> bool { invoke!(0xB952A3AC41D58F2F, index) }
	pub fn _RESTORE_GLOBAL_BLOCK(index: i32) -> bool { invoke!(0xDC3914A99B4A5FDF, index) }
	pub fn _DOES_COMPRESSED_GLOBAL_BLOCK_BUFFER_EXIST(index: i32) -> bool { invoke!(0x66EE5B93C308F734, index) }
	pub fn _SET_GLOBAL_BLOCK_CAN_BE_ACCESSED(index: i32, toggle: bool) { invoke_ignore!(0xE66F392BFCE734AF, index, toggle) }
	pub fn _GET_GLOBAL_BLOCK_CAN_BE_ACCESSED(index: i32) -> bool { invoke!(0x42A7EB5C814C2DE0, index) }
	pub fn _SET_ALL_GLOBAL_BLOCKS_HAVE_BEEN_LOADED(toggle: bool) { invoke_ignore!(0x11986B05885564D2, toggle) }
	pub fn HAVE_ALL_CHILD_SCRIPTS_TERMINATED(p0: i32) -> bool { invoke!(0x380FFA15B72408FB, p0) }
	pub fn START_NEW_SCRIPT(scriptName: & CStr, stackSize: i32) -> i32 { invoke!(0xE81651AD79516E48, scriptName, stackSize) }
	pub fn START_NEW_SCRIPT_WITH_ARGS(scriptName: & CStr, args: &mut Any, argCount: i32, stackSize: i32) -> i32 { invoke!(0xB8BA7F44DF1575E1, scriptName, args, argCount, stackSize) }
	pub fn START_NEW_SCRIPT_WITH_NAME_HASH(scriptHash: Hash, stackSize: i32) -> i32 { invoke!(0xEB1C67C3A5333A92, scriptHash, stackSize) }
	pub fn START_NEW_SCRIPT_WITH_NAME_HASH_AND_ARGS(scriptHash: Hash, args: &mut Any, argCount: i32, stackSize: i32) -> i32 { invoke!(0xC4BB298BD441BE78, scriptHash, args, argCount, stackSize) }
}
pub mod SAVE {
	use std::ffi::CStr;
	use crate::core::scripthook::native::*;
	use crate::core::types::*;

	pub fn _0x4FB5869E2B37FC00() { invoke_ignore!(0x4FB5869E2B37FC00) }
	pub fn SAVEGAME_SAVE_SP(savegameType: Hash) -> bool { invoke!(0x62C9EB51656D68CE, savegameType) }
	pub fn SAVEGAME_SAVE_MP(savegameType: Hash) -> bool { invoke!(0x1840F3B30ED0105F, savegameType) }
	pub fn SAVEGAME_IS_SAVE_PENDING() -> bool { invoke!(0x3CF46F55C6585590) }
	pub fn _0x1431540BCA1A1BD2() -> Any { invoke!(0x1431540BCA1A1BD2) }
	pub fn _0xA7ECEBAFBAF997A5(savegameType: Hash) -> Any { invoke!(0xA7ECEBAFBAF997A5, savegameType) }
	pub fn _0xED4B0C1057892B2E(p0: Any, p1: Any, p2: Any, p3: Any) { invoke_ignore!(0xED4B0C1057892B2E, p0, p1, p2, p3) }
	pub fn _0x9BB83C4DD7BE0802(p0: Any, p1: Any, p2: Any, p3: Any, p4: Any) { invoke_ignore!(0x9BB83C4DD7BE0802, p0, p1, p2, p3, p4) }
	pub fn _0xE8346E62FD7FB962() { invoke_ignore!(0xE8346E62FD7FB962) }
	pub fn _0xC0ABF784590798A9(p0: Any) { invoke_ignore!(0xC0ABF784590798A9, p0) }
	pub fn _0xB00CE33465B5406D(p0: Any, p1: Any) -> Any { invoke!(0xB00CE33465B5406D, p0, p1) }
	pub fn _SAVEGAME_GET_INT_2(p0: &mut Any, variableName: & CStr) { invoke_ignore!(0x529B9CCD0972AF4D, p0, variableName) }
	pub fn _SAVEGAME_GET_INT(p0: &mut Any, variableName: & CStr) { invoke_ignore!(0x529B9CCD0972AF4E, p0, variableName) }
	pub fn _SAVEGAME_GET_INT_3(p0: &mut Any, variableName: & CStr) { invoke_ignore!(0xB25B5A375BE5BE26, p0, variableName) }
	pub fn _SAVEGAME_GET_FLOAT(p0: &mut Any, variableName: & CStr) { invoke_ignore!(0x35DEFECAE36D4FAE, p0, variableName) }
	pub fn _SAVEGAME_GET_BOOL(p0: &mut Any, variableName: & CStr) { invoke_ignore!(0xBB7F4273C186BC4B, p0, variableName) }
	pub fn _SAVEGAME_GET_TEXT_LABEL_23(p0: &mut Any, variableName: & CStr) { invoke_ignore!(0x5A10D6506B2F2C63, p0, variableName) }
	pub fn _SAVEGAME_GET_TEXT_LABEL_31(p0: &mut Any, variableName: & CStr) { invoke_ignore!(0x4845E7E7643A908C, p0, variableName) }
	pub fn _SAVEGAME_GET_TEXT_LABEL_63(p0: &mut Any, variableName: & CStr) { invoke_ignore!(0x186608A2AC6F9E88, p0, variableName) }
	pub fn _0x443174C20B8B9E7F(p0: Any, p1: Any, p2: Any) { invoke_ignore!(0x443174C20B8B9E7F, p0, p1, p2) }
	pub fn _0x8E8FFB9E4AD051D2(p0: Any, p1: Any, p2: Any, p3: Any) { invoke_ignore!(0x8E8FFB9E4AD051D2, p0, p1, p2, p3) }
	pub fn _0xE0B45E983BFC0768() { invoke_ignore!(0xE0B45E983BFC0768) }
	pub fn _0x81F4E92BE3958364(p0: Any, p1: Any, p2: Any) { invoke_ignore!(0x81F4E92BE3958364, p0, p1, p2) }
	pub fn _0xA844FEB5C22C2C74() { invoke_ignore!(0xA844FEB5C22C2C74) }
}
pub mod SHAPETEST {
	use std::ffi::CStr;
	use crate::core::scripthook::native::*;
	use crate::core::types::*;

	pub fn START_SHAPE_TEST_LOS_PROBE(x1: f32, y1: f32, z1: f32, x2: f32, y2: f32, z2: f32, flags: i32, entity: Entity, p8: i32) -> ScrHandle { invoke!(0x7EE9F5D83DD4F90E, x1, y1, z1, x2, y2, z2, flags, entity, p8) }
	pub fn _0x04AA59CA40571C2E(p0: Any, p1: Any) -> Any { invoke!(0x04AA59CA40571C2E, p0, p1) }
	pub fn START_EXPENSIVE_SYNCHRONOUS_SHAPE_TEST_LOS_PROBE(x1: f32, y1: f32, z1: f32, x2: f32, y2: f32, z2: f32, flags: i32, entityToIgnore: Entity, p8: i32) -> ScrHandle { invoke!(0x377906D8A31E5586, x1, y1, z1, x2, y2, z2, flags, entityToIgnore, p8) }
	pub fn START_SHAPE_TEST_BOX(posX: f32, posY: f32, posZ: f32, dimensionsX: f32, dimensionsY: f32, dimensionsZ: f32, rotX: f32, rotY: f32, rotZ: f32, rotationOrder: i32, flags: i32, entityToIgnore: Entity, options: i32) -> ScrHandle { invoke!(0xFE466162C4401D18, posX, posY, posZ, dimensionsX, dimensionsY, dimensionsZ, rotX, rotY, rotZ, rotationOrder, flags, entityToIgnore, options) }
	pub fn START_SHAPE_TEST_CAPSULE(x1: f32, y1: f32, z1: f32, x2: f32, y2: f32, z2: f32, radius: f32, flags: i32, entityToIgnore: Entity, p9: i32) -> ScrHandle { invoke!(0x28579D1B8F8AAC80, x1, y1, z1, x2, y2, z2, radius, flags, entityToIgnore, p9) }
	pub fn START_SHAPE_TEST_SWEPT_SPHERE(x1: f32, y1: f32, z1: f32, x2: f32, y2: f32, z2: f32, radius: f32, flags: i32, entity: Entity, p9: Any) -> ScrHandle { invoke!(0xAA5B7C8309F73230, x1, y1, z1, x2, y2, z2, radius, flags, entity, p9) }
	pub fn START_SHAPE_TEST_MOUSE_CURSOR_LOS_PROBE(pVec1: &mut Vector3, pVec2: &mut Vector3, flag: i32, entity: Entity, flag2: i32) -> ScrHandle { invoke!(0x9839013D8B6014F1, pVec1, pVec2, flag, entity, flag2) }
	pub fn GET_SHAPE_TEST_RESULT(shapeTestHandle: ScrHandle, hit: &mut bool, endCoords: &mut Vector3, surfaceNormal: &mut Vector3, entityHit: &mut Entity) -> i32 { invoke!(0xEDE8AC7C5108FB1D, shapeTestHandle, hit, endCoords, surfaceNormal, entityHit) }
}
pub mod SOCIALCLUB {
	use std::ffi::CStr;
	use crate::core::scripthook::native::*;
	use crate::core::types::*;

	pub fn SC_INBOX_GET_TOTAL_NUM_MESSAGES() -> i32 { invoke!(0x8EF0F633280C0663) }
	pub fn SC_INBOX_GET_MESSAGE_TYPE_AT_INDEX(msgIndex: i32) -> Hash { invoke!(0xFF92537C4DDC1241, msgIndex) }
	pub fn SC_INBOX_GET_MESSAGE_IS_READ_AT_INDEX(msgIndex: i32) -> bool { invoke!(0x74CF39E030A382C4, msgIndex) }
	pub fn SC_INBOX_SET_MESSAGE_AS_READ_AT_INDEX(msgIndex: i32) -> bool { invoke!(0x63CAC501FFF66DC4, msgIndex) }
	pub fn SC_INBOX_MESSAGE_GET_DATA_INT(p0: i32, context: & CStr, out: &mut i32) -> bool { invoke!(0x95BB39C4DA99F348, p0, context, out) }
	pub fn SC_INBOX_MESSAGE_GET_DATA_STRING(p0: i32, context: & CStr, out: &mut CStr) -> bool { invoke!(0x66F77FD58506FF6B, p0, context, out) }
	pub fn SC_INBOX_MESSAGE_GET_RAW_TYPE_AT_INDEX(p0: i32) -> *const char { invoke!(0x176D077685CD83E4, p0) }
	pub fn SC_PRESENCE_ATTR_SET_FLOAT(attrHash: Hash, value: f32) -> bool { invoke!(0xA31DAFCDC33775E9, attrHash, value) }
	pub fn SC_PRESENCE_ATTR_SET_INT_EX(attrName: & CStr, value: i32, p2: bool) -> bool { invoke!(0x0000000085488C49, attrName, value, p2) }
	pub fn SC_PRESENCE_ATTR_SET_FLOAT_EX(attrName: & CStr, value: f32, p2: bool) -> bool { invoke!(0x00000000467F4CAA, attrName, value, p2) }
	pub fn SC_PRESENCE_ATTR_SET_STRING_EX(attrName: & CStr, value: & CStr, p2: bool) -> bool { invoke!(0x00000000EB2D93B3, attrName, value, p2) }
	pub fn SC_PROFANITY_CHECK_STRING(string: & CStr, token: &mut i32) -> bool { invoke!(0x9C74AC9D87B3FFF4, string, token) }
	pub fn SC_PROFANITY_GET_CHECK_IS_VALID(token: i32) -> bool { invoke!(0x08C8052AF40C4247, token) }
	pub fn SC_PROFANITY_GET_CHECK_IS_PENDING(token: i32) -> bool { invoke!(0x3A10BCD0C8AA0B82, token) }
	pub fn SC_PROFANITY_GET_STRING_PASSED(token: i32) -> bool { invoke!(0xF302973BB8BE70E6, token) }
	pub fn SC_PROFANITY_GET_STRING_STATUS(token: i32) -> i32 { invoke!(0x0CF3BFB99EBBE5B1, token) }
	pub fn SC_COMMUNITY_EVENT_IS_ACTIVE() -> bool { invoke!(0xCBF743C984695CF3) }
	pub fn SC_COMMUNITY_EVENT_GET_EVENT_ID() -> i32 { invoke!(0xD635DF6BAA5A6017) }
	pub fn SC_COMMUNITY_EVENT_GET_EXTRA_DATA_INT(p0: & CStr, p1: &mut i32) -> bool { invoke!(0xB4411D4D6B81438E, p0, p1) }
	pub fn SC_COMMUNITY_EVENT_GET_EXTRA_DATA_FLOAT(p0: & CStr, p1: &mut f32) -> bool { invoke!(0x060BBAD634C2B44B, p0, p1) }
	pub fn SC_COMMUNITY_EVENT_GET_EXTRA_DATA_STRING(p0: & CStr, p1: &mut CStr) -> bool { invoke!(0x9F6DCD0C939C71E9, p0, p1) }
	pub fn SC_COMMUNITY_EVENT_GET_DISPLAY_NAME(p0: &mut CStr) -> bool { invoke!(0x89D9BDE7334B110F, p0) }
	pub fn SC_COMMUNITY_EVENT_IS_ACTIVE_FOR_TYPE(p0: & CStr) -> bool { invoke!(0x09937EB0CEBC2F9F, p0) }
	pub fn SC_COMMUNITY_EVENT_GET_EVENT_ID_FOR_TYPE(p0: & CStr) -> i32 { invoke!(0x03C03ABBABBEF752, p0) }
	pub fn SC_COMMUNITY_EVENT_GET_EXTRA_DATA_INT_FOR_TYPE(p0: & CStr, p1: &mut i32, p2: & CStr) -> bool { invoke!(0x3519CC3525319A96, p0, p1, p2) }
	pub fn SC_COMMUNITY_EVENT_GET_EXTRA_DATA_FLOAT_FOR_TYPE(p0: & CStr, p1: &mut f32, p2: & CStr) -> bool { invoke!(0x1BDB56DB258F052D, p0, p1, p2) }
	pub fn SC_COMMUNITY_EVENT_GET_EXTRA_DATA_STRING_FOR_TYPE(p0: & CStr, p1: &mut CStr, p2: & CStr) -> bool { invoke!(0xC8FC3B2432E8229D, p0, p1, p2) }
	pub fn SC_COMMUNITY_EVENT_GET_DISPLAY_NAME_FOR_TYPE(p0: &mut CStr, p1: & CStr) -> bool { invoke!(0x85EA0BEC7B1F7622, p0, p1) }
	pub fn SC_COMMUNITY_EVENT_IS_ACTIVE_BY_ID(p0: i32) -> bool { invoke!(0x62B384FEFDE06817, p0) }
	pub fn SC_COMMUNITY_EVENT_GET_EXTRA_DATA_INT_BY_ID(p0: i32, p1: & CStr, p2: &mut i32) -> bool { invoke!(0x7C981DE05A7403A0, p0, p1, p2) }
	pub fn SC_COMMUNITY_EVENT_GET_EXTRA_DATA_FLOAT_BY_ID(p0: i32, p1: & CStr, p2: &mut f32) -> bool { invoke!(0x91C9E2A0F9DD6DD4, p0, p1, p2) }
	pub fn SC_COMMUNITY_EVENT_GET_EXTRA_DATA_STRING_BY_ID(p0: i32, p1: & CStr, p2: &mut CStr) -> bool { invoke!(0x049D2196D9D11184, p0, p1, p2) }
	pub fn SC_COMMUNITY_EVENT_GET_DISPLAY_NAME_BY_ID(p0: i32, p1: &mut CStr) -> bool { invoke!(0x11EA52CAD1B55910, p0, p1) }
}
pub mod SOCIALCLUBFEED {
	use std::ffi::CStr;
	use crate::core::scripthook::native::*;
	use crate::core::types::*;

	pub fn _SC_FEED_SUBMIT_PRESET_MESSAGE(type_: i32, subType: i32) -> i32 { invoke!(0xEFB64240F6B17817, type_, subType) }
	pub fn SC_FEED_HUB_HAS_NEW_DATA() -> bool { invoke!(0x068332D20CB6F897) }
}
pub mod SPACTIONPROXY {
	use std::ffi::CStr;
	use crate::core::scripthook::native::*;
	use crate::core::types::*;

	pub fn _SPACTIONPROXY_START_MANAGER() -> bool { invoke!(0x1F471B79ACC91BEE) }
	pub fn _SPACTIONPROXY_MANAGER_IS_READY() -> bool { invoke!(0x1F471B79ACC91BED) }
	pub fn _SPACTIONPROXY_MANAGER_IS_FAILED() -> bool { invoke!(0x1F471B79ACC91BEC) }
	pub fn _SPACTIONPROXY_GET_NEXT_PENDING_CRAFTING_ACTION(data: &mut Any) -> bool { invoke!(0x1F471B79ACC97BEF, data) }
	pub fn _SPACTIONPROXY_GET_NEXT_PENDING_BUY_ACTION(data: &mut Any) -> bool { invoke!(0x1F471B79ACC98BEF, data) }
	pub fn _SPACTIONPROXY_PROCESS_ACTION(p0: Any, p1: bool) -> bool { invoke!(0x1F471B79ACC94BEF, p0, p1) }
}
pub mod STATS {
	use std::ffi::CStr;
	use crate::core::scripthook::native::*;
	use crate::core::types::*;

	pub fn STAT_ID_IS_VALID(statId: &mut Any) -> bool { invoke!(0xC48FE1971C9743FF, statId) }
	pub fn STAT_ID_SET_INT(statId: &mut Any, value: i32, p2: bool) -> bool { invoke!(0xA4DDF5DF95E65EEE, statId, value, p2) }
	pub fn STAT_ID_SET_FLOAT(statId: &mut Any, value: f32, p2: bool) -> bool { invoke!(0x481BDF6A10C5EF68, statId, value, p2) }
	pub fn STAT_ID_SET_BOOL(statId: &mut Any, value: bool, p2: bool) -> bool { invoke!(0x3B5107353267D7A1, statId, value, p2) }
	pub fn STAT_ID_SET_GXT_LABEL(statId: &mut Any, label: & CStr, p2: bool) -> bool { invoke!(0x05060A54834F2382, statId, label, p2) }
	pub fn STAT_ID_SET_DATE(statId: &mut Any, date: &mut Any, p2: bool) -> bool { invoke!(0x1FAE9B2FAA2DFE06, statId, date, p2) }
	pub fn STAT_ID_GET_INT(statId: &mut Any, p1: &mut i32) -> bool { invoke!(0x767FBC2AC802EF3E, statId, p1) }
	pub fn STAT_ID_GET_FLOAT(statId: &mut Any, value: &mut f32) -> bool { invoke!(0xD7AE6C9C9C6AC54D, statId, value) }
	pub fn STAT_ID_GET_BOOL(statId: &mut Any, value: &mut bool) -> bool { invoke!(0x11B5E6D2AE73F48F, statId, value) }
	pub fn STAT_ID_GET_DATE(statId: &mut Any, date: &mut Any) -> bool { invoke!(0x8B0FACEFC36C824C, statId, date) }
	pub fn _0x0FEE2561120F3333(statId: &mut Any) { invoke_ignore!(0x0FEE2561120F3333, statId) }
	pub fn _STAT_ID_INCREMENT_INT(statId: &mut Any, value: i32) { invoke_ignore!(0x6A0184E904CDF25E, statId, value) }
	pub fn _STAT_ID_INCREMENT_FLOAT(statId: &mut Any, value: f32) { invoke_ignore!(0x4A47E38EA3D60939, statId, value) }
	pub fn _STAT_ID_DECREMENT_INT(statId: &mut Any, value: i32) { invoke_ignore!(0xBD861AE8A5181ED7, statId, value) }
	pub fn _0x91A4F58E01ED5E4C(statId: &mut Any, value: i32) { invoke_ignore!(0x91A4F58E01ED5E4C, statId, value) }
	pub fn _0xE141F6B40B1E3683(statId: &mut Any, value: f32) { invoke_ignore!(0xE141F6B40B1E3683, statId, value) }
	pub fn STAT_ID_SET_TO_POSSE_ID(statId: &mut Any) { invoke_ignore!(0x34B22DE38477EDB4, statId) }
	pub fn _STAT_CALCULATE_COOLDOWN(value: i32) -> i32 { invoke!(0x1E7384AB5D4F4581, value) }
	pub fn _STAT_PHEROMONE_COOLDOWN_LEGENDARY_ANIMAL(entity: Entity, statId: &mut Any) -> bool { invoke!(0x5420D398A42917FC, entity, statId) }
	pub fn _STAT_ITEM_FISH_CAUGHT(fish: Ped, weight: f32, category: Hash, subcategory: Hash) { invoke_ignore!(0xDA26263C87CCE9C1, fish, weight, category, subcategory) }
	pub fn _STAT_CARRIED_SATCHEL_ITEM_FROM_PED(ped: Ped) { invoke_ignore!(0x831BF01C56149A8A, ped) }
	pub fn _STAT_DONATE_INCREMENT_ITEM(item: i32, slot: i32, p2: Any, p3: Any) { invoke_ignore!(0x7C2ABF6E556B21FC, item, slot, p2, p3) }
	pub fn _0x8312F09C56149A8A(animalType: Hash) { invoke_ignore!(0x8312F09C56149A8A, animalType) }
	pub fn _0x378D3B1B11D9385B(p0: i32) { invoke_ignore!(0x378D3B1B11D9385B, p0) }
	pub fn _0xDA26263C07CCE9C2(p0: i32) { invoke_ignore!(0xDA26263C07CCE9C2, p0) }
	pub fn _0xD64DBC8B0424135F(ped: Ped, animalType: Hash) { invoke_ignore!(0xD64DBC8B0424135F, ped, animalType) }
	pub fn _0xA59590050F80FF2E(p0: Any, p1: bool, p2: bool, p3: bool) { invoke_ignore!(0xA59590050F80FF2E, p0, p1, p2, p3) }
	pub fn _STAT_ADD_ANIMAL_SAMPLE_TARGET(animalType: Hash) { invoke_ignore!(0x90E9A5DADBABC918, animalType) }
	pub fn _0xF8181B5EF156862C(ped: Ped) { invoke_ignore!(0xF8181B5EF156862C, ped) }
	pub fn STAT_ADD_BOUNTY_TARGET(unlockHash: Hash, ped: Ped) { invoke_ignore!(0x6B1044FDC2B09101, unlockHash, ped) }
	pub fn _0x8C889E4CBB4B2356(p0: Any, ped: Ped) { invoke_ignore!(0x8C889E4CBB4B2356, p0, ped) }
	pub fn STAT_BOUNTY_CAPTURED(entity: Entity) { invoke_ignore!(0x262EF7CF49CF1EB9, entity) }
	pub fn _0xA596890CF55B5095(ped: Ped, p1: bool) { invoke_ignore!(0xA596890CF55B5095, ped, p1) }
	pub fn STAT_BOUNTY_ESCAPED(ped: Ped) { invoke_ignore!(0xB22F05732F72F70C, ped) }
	pub fn _0xF21A5D66874FCEDD(p0: Any, p1: Hash, p2: Hash) { invoke_ignore!(0xF21A5D66874FCEDD, p0, p1, p2) }
	pub fn _0x3EB2791A1FBC8A42(statItem: Hash, p1: i32) { invoke_ignore!(0x3EB2791A1FBC8A42, statItem, p1) }
	pub fn _0xDF95DF488A645CE7() { invoke_ignore!(0xDF95DF488A645CE7) }
	pub fn STAT_PHOTOGRAPH_TAKEN(itemset: ItemSet) { invoke_ignore!(0x4D31051A4CA83787, itemset) }
	pub fn _0xBE66B26B6529E943(unlockHash: Hash, ped: Ped, animalType: Hash) { invoke_ignore!(0xBE66B26B6529E943, unlockHash, ped, animalType) }
	pub fn _0xF2B5ABDE09958689(unlockHash: Hash, ped1: Ped, ped2: Ped) { invoke_ignore!(0xF2B5ABDE09958689, unlockHash, ped1, ped2) }
	pub fn STAT_REGISTER_LEGENDARY_ANIMAL_DEED(deedHash: Hash) { invoke_ignore!(0xCD0D69C65BB0E8B9, deedHash) }
	pub fn _0x302E71C1D9EE75B9(statId: &mut Any, p1: Hash, p2: &mut i32) -> bool { invoke!(0x302E71C1D9EE75B9, statId, p1, p2) }
	pub fn STATSTRACKER_IS_INITIALIZED(p0: Hash) -> bool { invoke!(0x01F4D242765C6B24, p0) }
	pub fn _0x6123E2832C34243D(p0: Any, p1: Any, p2: Any, p3: Any, p4: Any) { invoke_ignore!(0x6123E2832C34243D, p0, p1, p2, p3, p4) }
	pub fn _0xCA41E86545413B5B(p0: Any, p1: Any, p2: Any, p3: Any, p4: Any, p5: Any, p6: Any) { invoke_ignore!(0xCA41E86545413B5B, p0, p1, p2, p3, p4, p5, p6) }
	pub fn STATSTRACKER_DEED_STARTED(p0: Hash, p1: Any) { invoke_ignore!(0xB2A38826E5886E83, p0, p1) }
	pub fn _STATSTRACKER_DEED_STATUS(deedType: i32, deedHash: Hash, missionStatus: i32, data: &mut Any) { invoke_ignore!(0xD5910ECF81A2278C, deedType, deedHash, missionStatus, data) }
	pub fn _0x99230691875FC218(p0: Any, p1: Hash, x: f32, y: f32, z: f32) { invoke_ignore!(0x99230691875FC218, p0, p1, x, y, z) }
	pub fn _0x025E98E317652CDD(p0: i32) { invoke_ignore!(0x025E98E317652CDD, p0) }
	pub fn _0xE5A680A5D8B1F687(p0: i32) { invoke_ignore!(0xE5A680A5D8B1F687, p0) }
	pub fn _0x4DAC398297981B87(p0: i32) -> bool { invoke!(0x4DAC398297981B87, p0) }
	pub fn CHAL_IS_GOAL_ACTIVE(chalHash: Hash, goalHash: Hash) -> bool { invoke!(0x04DAC3929796EB87, chalHash, goalHash) }
	pub fn CHAL_SET_GOAL_DISABLED(chalHash: Hash, goalHash: Hash, disabled: bool) { invoke_ignore!(0xF63DF9EE16393343, chalHash, goalHash, disabled) }
	pub fn CHAL_GET_NUM_RANKS_COMPLETED(chalHash: Hash) -> i32 { invoke!(0x58CB53DB63F84DE9, chalHash) }
	pub fn CHAL_GET_MAX_RANKS(chalHash: Hash) -> i32 { invoke!(0x58CB53DB63F84DEA, chalHash) }
	pub fn CHAL_ADD_GOAL_PROGRESS_INT(chalHash: Hash, goalHash: Hash, value: i32) { invoke_ignore!(0xDDBD560745B1EE9A, chalHash, goalHash, value) }
	pub fn CHAL_ADD_GOAL_PROGRESS_FLOAT(chalHash: Hash, goalHash: Hash, value: f32) { invoke_ignore!(0x86922D8C02FB7703, chalHash, goalHash, value) }
	pub fn CHAL_SET_GOAL_PROGRESS_INT(chalHash: Hash, goalHash: Hash, value: i32) { invoke_ignore!(0xDDBD560745B1EE9B, chalHash, goalHash, value) }
	pub fn CHAL_ADD_GOAL_PROGRESS_INT_BY_SCORE_ID(p0: Hash, value: i32) { invoke_ignore!(0xDDBD560745B1EE9C, p0, value) }
	pub fn CHAL_ADD_GOAL_PROGRESS_FLOAT_BY_SCORE_ID(p0: Hash, value: f32) { invoke_ignore!(0x86922D8C02FB7705, p0, value) }
	pub fn CHAL_ACHIEVEMENT_IS_COMPLETE(p0: Hash, p1: Hash) -> bool { invoke!(0x77B97A827739D434, p0, p1) }
	pub fn CHAL_ACHIEVEMENT_GET_PROGRESS_INT(p0: Hash, p1: Hash) -> i32 { invoke!(0x808712E428F697B8, p0, p1) }
	pub fn CHAL_NET_START_CHAL(chalHash: Hash) { invoke_ignore!(0x4ABF7E4DB6279E8F, chalHash) }
	pub fn CHAL_NET_STOP_CHAL(chalHash: Hash) { invoke_ignore!(0x43B0163154A50C86, chalHash) }
	pub fn CHAL_NET_START_GOAL(chalHash: Hash, goalHash: Hash) { invoke_ignore!(0xC3FCB47344DCB638, chalHash, goalHash) }
	pub fn CHAL_NET_STOP_GOAL(chalHash: Hash, goalHash: Hash) { invoke_ignore!(0x00CE6A93324A590B, chalHash, goalHash) }
	pub fn _0xDDBD560745B1EE98(chalHash: Hash, goalHash: Hash, player: Player) -> i32 { invoke!(0xDDBD560745B1EE98, chalHash, goalHash, player) }
	pub fn _0xB112B9262EC29C20(p0: Hash, p1: i32) -> *const char { invoke!(0xB112B9262EC29C20, p0, p1) }
	pub fn _0x4FCBCC0584CD08E9(p0: Hash) { invoke_ignore!(0x4FCBCC0584CD08E9, p0) }
	pub fn _0xCA1F0B5103936891(p0: Hash) -> bool { invoke!(0xCA1F0B5103936891, p0) }
	pub fn _0x8BA3D7B1E83EF803(p0: Hash) -> Hash { invoke!(0x8BA3D7B1E83EF803, p0) }
	pub fn CHAL_MISSION_GET_NUM_GOALS(missionHash: Hash) -> i32 { invoke!(0x0B0576DD3A75E58D, missionHash) }
	pub fn CHAL_MISSION_GET_NUM_GOALS_COMPLETE(missionHash: Hash) -> i32 { invoke!(0xA785A52B59B7E7B2, missionHash) }
	pub fn CHAL_MISSION_IS_GOAL_COMPLETE(missionHash: Hash, goalHash: Hash) -> bool { invoke!(0xC0BB774787BBF301, missionHash, goalHash) }
	pub fn CHAL_MISSION_ADD_GOAL_PROGRESS_INT(missionHash: Hash, goalHash: Hash, value: i32) { invoke_ignore!(0x97E18E7C098626DE, missionHash, goalHash, value) }
	pub fn _0x9D0F5D2E1951CD84() -> f32 { invoke!(0x9D0F5D2E1951CD84) }
	pub fn _0x218F7710A139D012() { invoke_ignore!(0x218F7710A139D012) }
	pub fn _0x3AEABAE3F3C7600C() -> bool { invoke!(0x3AEABAE3F3C7600C) }
	pub fn _0x3F6FD87D2030ADC6() -> *const char { invoke!(0x3F6FD87D2030ADC6) }
	pub fn _0xA2E2BEA4E83F6270(p0: Hash) -> Any { invoke!(0xA2E2BEA4E83F6270, p0) }
	pub fn _0xB5E2EDA2135E0FA1(p0: Hash, p1: i32, scheduleLocation: &mut Hash) -> bool { invoke!(0xB5E2EDA2135E0FA1, p0, p1, scheduleLocation) }
	pub fn WEEKLY_COLLECTIBLE_GET_NUM_SETS(chalHash: Hash) -> i32 { invoke!(0x8F5317729F791D10, chalHash) }
	pub fn WEEKLY_COLLECTIBLE_GET_ITEM_SET_BUY_AWARD(chalHash: Hash, index: i32) -> Hash { invoke!(0x610783F646894D25, chalHash, index) }
	pub fn WEEKLY_COLLECTIBLE_GET_ITEM_SET_LABEL(chalHash: Hash, index: i32) -> Hash { invoke!(0xBFFA88522FF0F730, chalHash, index) }
	pub fn WEEKLY_COLLECTIBLE_GET_NUM_ITEMS_IN_SET(chalHash: Hash, index: i32) -> i32 { invoke!(0x7D675C9DDDB365BE, chalHash, index) }
	pub fn WEEKLY_COLLECTIBLE_GET_ITEM_IN_SET(chalHash: Hash, setIndex: i32, itemIndex: i32, p3: &mut Hash, p4: &mut i32) -> bool { invoke!(0xBA61BA6205A3F5A8, chalHash, setIndex, itemIndex, p3, p4) }
	pub fn _0x4F2D5FA23DB992DE() { invoke_ignore!(0x4F2D5FA23DB992DE) }
	pub fn _0x4E463A3CDEFFFE96() { invoke_ignore!(0x4E463A3CDEFFFE96) }
}
pub mod STREAMING {
	use std::ffi::CStr;
	use crate::core::scripthook::native::*;
	use crate::core::types::*;

	pub fn REQUEST_MODEL(model: Hash, p1: bool) { invoke_ignore!(0xFA28FE3A6246FC30, model, p1) }
	pub fn HAS_MODEL_LOADED(model: Hash) -> bool { invoke!(0x1283B8B89DD5D1B6, model) }
	pub fn SET_MODEL_AS_NO_LONGER_NEEDED(model: Hash) { invoke_ignore!(0x4AD96EF928BD4F9A, model) }
	pub fn IS_MODEL_IN_CDIMAGE(model: Hash) -> bool { invoke!(0xD6F3B6D7716CFF8E, model) }
	pub fn IS_MODEL_VALID(model: Hash) -> bool { invoke!(0x392C8D8E07B70EFC, model) }
	pub fn IS_MODEL_A_PED(model: Hash) -> bool { invoke!(0xC3F09DE9D6D17DDA, model) }
	pub fn IS_MODEL_A_VEHICLE(model: Hash) -> bool { invoke!(0x354F62672DE7DB0A, model) }
	pub fn _IS_MODEL_AN_OBJECT(model: Hash) -> bool { invoke!(0x274EE1B90CFA669E, model) }
	pub fn _HAS_COLLISION_LOADED_AT_COORD(x: f32, y: f32, z: f32) -> bool { invoke!(0xDA8B2EAF29E872E2, x, y, z) }
	pub fn _0x80B3E0597366ADF1() { invoke_ignore!(0x80B3E0597366ADF1) }
	pub fn REQUEST_COLLISION_AT_COORD(x: f32, y: f32, z: f32) { invoke_ignore!(0x0A3720F162A033C9, x, y, z) }
	pub fn _REQUEST_METADATA_AT_COORD(x: f32, y: f32, z: f32) { invoke_ignore!(0xA8432A14D4DC2101, x, y, z) }
	pub fn REQUEST_COLLISION_FOR_MODEL(model: Hash) { invoke_ignore!(0xF1767BE37F661551, model) }
	pub fn HAS_COLLISION_FOR_MODEL_LOADED(model: Hash) -> bool { invoke!(0x210A79C9EC89778F, model) }
	pub fn REQUEST_ADDITIONAL_COLLISION_AT_COORD(x: f32, y: f32, z: f32) { invoke_ignore!(0x83A8D71650D1894F, x, y, z) }
	pub fn DOES_ANIM_DICT_EXIST(animDict: & CStr) -> bool { invoke!(0x537F44CB0D7F150D, animDict) }
	pub fn REQUEST_ANIM_DICT(animDict: & CStr) { invoke_ignore!(0xA862A2AD321F94B4, animDict) }
	pub fn HAS_ANIM_DICT_LOADED(animDict: & CStr) -> bool { invoke!(0x27FF6FE8009B40CA, animDict) }
	pub fn REMOVE_ANIM_DICT(animDict: & CStr) { invoke_ignore!(0x4763145053A33D46, animDict) }
	pub fn REQUEST_MOVE_NETWORK_DEF(name: & CStr) { invoke_ignore!(0x2B6529C54D29037A, name) }
	pub fn HAS_MOVE_NETWORK_DEF_LOADED(name: & CStr) -> bool { invoke!(0x2C04D89A0FB4E244, name) }
	pub fn REMOVE_MOVE_NETWORK_DEF(name: & CStr) { invoke_ignore!(0x57A197AD83F66BBF, name) }
	pub fn REQUEST_CLIP_SET(clipSet: & CStr) { invoke_ignore!(0xEF7611B57A820126, clipSet) }
	pub fn HAS_CLIP_SET_LOADED(clipSet: & CStr) -> bool { invoke!(0x1F23D6B6DA1CC3B2, clipSet) }
	pub fn REMOVE_CLIP_SET(clipSet: & CStr) { invoke_ignore!(0x817FA1B1EE7CD6F0, clipSet) }
	pub fn _REQUEST_CLIP_SET_BY_HASH(clipSetHash: Hash) { invoke_ignore!(0xAC37644A538F7524, clipSetHash) }
	pub fn _0x03DDBF2D73799F9E(p0: Any) { invoke_ignore!(0x03DDBF2D73799F9E, p0) }
	pub fn _0x85B8F04555AB49B8(p0: Any) -> Any { invoke!(0x85B8F04555AB49B8, p0) }
	pub fn _0x9F348DE670423460(p0: Any) { invoke_ignore!(0x9F348DE670423460, p0) }
	pub fn _0x5288B7F0690F7C1F(p0: Any) -> Any { invoke!(0x5288B7F0690F7C1F, p0) }
	pub fn _REQUEST_SCENARIO_TYPE(scenarioType: Hash, p1: i32, p2: Any, p3: Any) -> i32 { invoke!(0x19A6BE7D9C6884D3, scenarioType, p1, p2, p3) }
	pub fn _HAS_SCENARIO_TYPE_LOADED(scenarioType: Hash, p1: bool) -> bool { invoke!(0x9427C94D2E4094A4, scenarioType, p1) }
	pub fn _REMOVE_SCENARIO_ASSET(scenarioType: Hash) -> Any { invoke!(0x4EDDD9E9CA5AF985, scenarioType) }
	pub fn _0xB223249B7798EEED(p0: Any, p1: Any, p2: Any, p3: Any) -> Any { invoke!(0xB223249B7798EEED, p0, p1, p2, p3) }
	pub fn _0xA0AE7653E8181725(p0: Any) -> Any { invoke!(0xA0AE7653E8181725, p0) }
	pub fn _0x66BC28E50E85270E(p0: Any) -> Any { invoke!(0x66BC28E50E85270E, p0) }
	pub fn _GET_IPL_BOUNDING_SPHERE(iplHash: Hash, position: &mut Vector3, radius: &mut f32) -> bool { invoke!(0x9C77964B0E07B633, iplHash, position, radius) }
	pub fn REQUEST_IPL_HASH(iplHash: Hash) { invoke_ignore!(0x59767C5A7A9AE6DA, iplHash) }
	pub fn REQUEST_IPL_BY_HASH(iplHash: Hash) { invoke_ignore!(0x9E211A378F95C97C, iplHash) }
	pub fn REMOVE_IPL_HASH(iplHash: Hash) { invoke_ignore!(0x5A3E5CF7B4014B96, iplHash) }
	pub fn REMOVE_IPL_BY_HASH(iplHash: Hash) { invoke_ignore!(0x431E3AB760629B34, iplHash) }
	pub fn IS_IPL_ACTIVE_HASH(iplHash: Hash) -> bool { invoke!(0xD779B9B910BD3B7C, iplHash) }
	pub fn IS_IPL_ACTIVE_BY_HASH(iplHash: Hash) -> bool { invoke!(0x93AC1B91CB6D9913, iplHash) }
	pub fn _IS_POSITION_INSIDE_IPL_STREAMING_EXTENTS(iplHash: Hash, x: f32, y: f32, z: f32) -> bool { invoke!(0x73B40D97D7BAAD77, iplHash, x, y, z) }
	pub fn _0xDEEE1F265B7ECEF5() { invoke_ignore!(0xDEEE1F265B7ECEF5) }
	pub fn SET_GAME_PAUSES_FOR_STREAMING(toggle: bool) { invoke_ignore!(0xB3BC8250F4FE8B63, toggle) }
	pub fn GET_NUMBER_OF_STREAMING_REQUESTS() -> i32 { invoke!(0x30CCCC4D88E654CA) }
	pub fn REQUEST_PTFX_ASSET() { invoke_ignore!(0x001FF43843028E0C) }
	pub fn HAS_PTFX_ASSET_LOADED() -> bool { invoke!(0x13A3F30A9ED0BC31) }
	pub fn REMOVE_PTFX_ASSET() { invoke_ignore!(0x042F9049EA419E86) }
	pub fn REQUEST_NAMED_PTFX_ASSET(fxNameHash: Hash) { invoke_ignore!(0xF2B2353BBC0D4E8F, fxNameHash) }
	pub fn HAS_NAMED_PTFX_ASSET_LOADED(fxNameHash: Hash) -> bool { invoke!(0x65BB72F29138F5D6, fxNameHash) }
	pub fn REMOVE_NAMED_PTFX_ASSET(fxNameHash: Hash) { invoke_ignore!(0xF20866829E1C81A2, fxNameHash) }
	pub fn SET_POPULATION_BUDGET_MULTIPLIER(fBudgetMultiplier: f32) { invoke_ignore!(0x2F9AC754FE179D58, fBudgetMultiplier) }
	pub fn GET_POPULATION_BUDGET_MULTIPLIER() -> f32 { invoke!(0x8A3945405B31048F) }
	pub fn _0x071769BCB24379E5() -> Any { invoke!(0x071769BCB24379E5) }
	pub fn CLEAR_FOCUS() { invoke_ignore!(0x86CCAF7CE493EFBE) }
	pub fn SET_FOCUS_POS_AND_VEL(x: f32, y: f32, z: f32, offsetX: f32, offsetY: f32, offsetZ: f32) { invoke_ignore!(0x25F6EF88664540E2, x, y, z, offsetX, offsetY, offsetZ) }
	pub fn SET_FOCUS_ENTITY(entity: Entity) { invoke_ignore!(0x955AEDD58F4BD309, entity) }
	pub fn IS_ENTITY_FOCUS(entity: Entity) -> bool { invoke!(0xF87DE697E9A06EC6, entity) }
	pub fn SET_MAPDATACULLBOX_ENABLED(name: & CStr, toggle: bool) { invoke_ignore!(0x3CACC83F6FED837C, name, toggle) }
	pub fn SET_ALL_MAPDATA_CULLED(p0: Any) { invoke_ignore!(0x19ABCC581D28E6F9, p0) }
	pub fn _0xF01D21DF39554115(p0: Any) { invoke_ignore!(0xF01D21DF39554115, p0) }
	pub fn LOAD_SCENE_START(posX: f32, posY: f32, posZ: f32, offsetX: f32, offsetY: f32, offsetZ: f32, radius: f32, p7: i32) -> bool { invoke!(0x387AD749E3B69B70, posX, posY, posZ, offsetX, offsetY, offsetZ, radius, p7) }
	pub fn LOAD_SCENE_START_SPHERE(x: f32, y: f32, z: f32, radius: f32, p4: Any) -> bool { invoke!(0x513F8AA5BF2F17CF, x, y, z, radius, p4) }
	pub fn LOAD_SCENE_STOP() { invoke_ignore!(0x5A8B01199C3E79C3) }
	pub fn IS_LOAD_SCENE_ACTIVE() -> bool { invoke!(0xCF45DF50C7775F2A) }
	pub fn IS_LOAD_SCENE_LOADED() -> bool { invoke!(0x0909F71B5C070797) }
	pub fn IS_RENDERED_SCENE_LOADED() -> bool { invoke!(0x45BF3A6239A576B7) }
	pub fn IS_PLAYER_SWITCH_IN_PROGRESS() -> bool { invoke!(0xED20CB1F5297791D) }
	pub fn SET_SCENE_STREAMING_TRACKS_CAM_POS_THIS_FRAME() { invoke_ignore!(0xA03A6812529AD9C8) }
	pub fn IPL_GROUP_SWAP_START(iplName1: & CStr, iplName2: & CStr) { invoke_ignore!(0x20D504994FDC4412, iplName1, iplName2) }
	pub fn IPL_GROUP_SWAP_CANCEL() { invoke_ignore!(0x31108BB5715D035F) }
	pub fn IPL_GROUP_SWAP_IS_READY() -> bool { invoke!(0xC2C05DEFE85A0B64) }
	pub fn IPL_GROUP_SWAP_FINISH() { invoke_ignore!(0x040EE319EFD1D3B5) }
	pub fn IPL_GROUP_SWAP_IS_ACTIVE() -> bool { invoke!(0xFC464598F6EE97B0) }
	pub fn PREFETCH_SRL(srl: & CStr) { invoke_ignore!(0x354837E5A5BAA5AF, srl) }
	pub fn _0xAE00387E53B1E9FC() { invoke_ignore!(0xAE00387E53B1E9FC) }
	pub fn _0xEF1A8A484118735E() { invoke_ignore!(0xEF1A8A484118735E) }
	pub fn _0xD9F2FF4AF394D926() { invoke_ignore!(0xD9F2FF4AF394D926) }
	pub fn IS_SRL_LOADED() -> bool { invoke!(0x5C2C88512CF6DAFB) }
	pub fn BEGIN_SRL() { invoke_ignore!(0x0360710033BE60D9) }
	pub fn END_SRL() { invoke_ignore!(0x1CE71FB33CA079FE) }
	pub fn SET_SRL_TIME(p0: f32) { invoke_ignore!(0x18231AEF458BCFF2, p0) }
	pub fn SET_SRL_READAHEAD_TIMES(p0: i32, p1: i32, p2: i32, p3: i32) { invoke_ignore!(0xD346248C1DCE0D76, p0, p1, p2, p3) }
	pub fn SET_SRL_LONG_JUMP_MODE(p0: bool) { invoke_ignore!(0x7C907E8A725E5FD2, p0) }
	pub fn SET_HD_AREA(x: f32, y: f32, z: f32, radius: f32) { invoke_ignore!(0xB88B905AFA35CB4D, x, y, z, radius) }
	pub fn CLEAR_HD_AREA() { invoke_ignore!(0xD83B22434E52728D) }
	pub fn _0x09FBF15D73EFC900() { invoke_ignore!(0x09FBF15D73EFC900) }
	pub fn _0xF11D7CB962FCD747(p0: Any) { invoke_ignore!(0xF11D7CB962FCD747, p0) }
	pub fn _0xB9B9E47EDB9D63DB() { invoke_ignore!(0xB9B9E47EDB9D63DB) }
	pub fn _0xBE8DAA9D8D01DA6A(p0: Any, p1: Any, p2: Any) { invoke_ignore!(0xBE8DAA9D8D01DA6A, p0, p1, p2) }
	pub fn _0x53764309C4618087(p0: Any) -> Any { invoke!(0x53764309C4618087, p0) }
	pub fn _0x032A14D082A9B269(p0: Hash) { invoke_ignore!(0x032A14D082A9B269, p0) }
	pub fn _0xAFA87A7D41EE346A(p0: Any) { invoke_ignore!(0xAFA87A7D41EE346A, p0) }
	pub fn _0x6A6E79FBE8678C98() { invoke_ignore!(0x6A6E79FBE8678C98) }
	pub fn _0xCC61D8D6C19D9F14(p0: Any) { invoke_ignore!(0xCC61D8D6C19D9F14, p0) }
	pub fn _0xDA7FDEFF4DE86839() -> Any { invoke!(0xDA7FDEFF4DE86839) }
	pub fn _0x5D5E2102B174B8D2() -> Any { invoke!(0x5D5E2102B174B8D2) }
	pub fn _0x7B8C2B846C05E5AD() -> Any { invoke!(0x7B8C2B846C05E5AD) }
	pub fn _0x62D5F0588915B277() { invoke_ignore!(0x62D5F0588915B277) }
	pub fn _0x2F4D53023F826FF0() -> Any { invoke!(0x2F4D53023F826FF0) }
	pub fn _0xDABFE48BA0D457AA() -> Any { invoke!(0xDABFE48BA0D457AA) }
	pub fn _0xE5B76E5B56CD77DD() -> Any { invoke!(0xE5B76E5B56CD77DD) }
	pub fn _0x27AF48C62B281341() -> Any { invoke!(0x27AF48C62B281341) }
	pub fn _0x99F92061EFE908BA() -> Any { invoke!(0x99F92061EFE908BA) }
	pub fn _0x05DD384F39DE1C8C(p0: Any, p1: Any) -> Any { invoke!(0x05DD384F39DE1C8C, p0, p1) }
	pub fn _0x198B85CC3C7A4593(p0: Any, p1: Any) -> Any { invoke!(0x198B85CC3C7A4593, p0, p1) }
	pub fn _0x2A6D1DAAB9EBB262(p0: Any) -> Any { invoke!(0x2A6D1DAAB9EBB262, p0) }
	pub fn _0x07559B29950301FF(p0: Any, p1: Any) { invoke_ignore!(0x07559B29950301FF, p0, p1) }
	pub fn _0xD6E39DC5D46DF4AB(p0: Any) -> Any { invoke!(0xD6E39DC5D46DF4AB, p0) }
	pub fn _0x8D56BDA343D9519F(p0: Any) -> Any { invoke!(0x8D56BDA343D9519F, p0) }
	pub fn _0xD840C130D7AACFA5(p0: Any, p1: Any, p2: Any) { invoke_ignore!(0xD840C130D7AACFA5, p0, p1, p2) }
	pub fn _0x2E24C27B112B5B12(p0: Any) { invoke_ignore!(0x2E24C27B112B5B12, p0) }
	pub fn _SET_GUARMA_WORLDHORIZON_ACTIVE(toggle: bool) { invoke_ignore!(0x74E2261D2A66849A, toggle) }
}
pub mod TASK {
	use std::ffi::CStr;
	use crate::core::scripthook::native::*;
	use crate::core::types::*;

	pub fn TASK_PAUSE(ped: Ped, ms: i32) { invoke_ignore!(0xE73A266DB0CA9042, ped, ms) }
	pub fn TASK_STAND_STILL(ped: Ped, time: i32) { invoke_ignore!(0x919BE13EED931959, ped, time) }
	pub fn TASK_JUMP(ped: Ped, unused: bool) { invoke_ignore!(0x0AE4086104E067B1, ped, unused) }
	pub fn _TASK_JUMP_2(ped: Ped, x: f32, y: f32, z: f32, entity: Entity) { invoke_ignore!(0x91083103137D7254, ped, x, y, z, entity) }
	pub fn TASK_COWER(ped: Ped, duration: i32, pedToCowerFrom: Ped, p3: & CStr) { invoke_ignore!(0x3EB1FE9E8E908E15, ped, duration, pedToCowerFrom, p3) }
	pub fn TASK_HANDS_UP(ped: Ped, duration: i32, facingPed: Ped, timeToFacePed: i32, flags: i32) { invoke_ignore!(0xF2EAB31979A7F910, ped, duration, facingPed, timeToFacePed, flags) }
	pub fn TASK_KNOCKED_OUT(ped: Ped, p1: f32, permanently: bool) { invoke_ignore!(0xF90427F00A495A28, ped, p1, permanently) }
	pub fn TASK_KNOCKED_OUT_AND_HOGTIED(ped: Ped, p1: f32, p2: i32) { invoke_ignore!(0x42AC6401ABB8C7E5, ped, p1, p2) }
	pub fn _0xFFB520A3E16F7B7B(ped: Ped, p1: f32) { invoke_ignore!(0xFFB520A3E16F7B7B, ped, p1) }
	pub fn _0x8B1FDF63C3193EDA(ped: Ped, p1: f32) { invoke_ignore!(0x8B1FDF63C3193EDA, ped, p1) }
	pub fn UPDATE_TASK_HANDS_UP_DURATION(ped: Ped, duration: i32) { invoke_ignore!(0xA98FCAFD7893C834, ped, duration) }
	pub fn _0x28EF780BDEA8A639(ped: Ped, p1: i32) { invoke_ignore!(0x28EF780BDEA8A639, ped, p1) }
	pub fn TASK_DUCK(ped: Ped, p1: i32) { invoke_ignore!(0xA14B5FBF986BAC23, ped, p1) }
	pub fn _TASK_BOARD_VEHICLE(ped: Ped, vehicle: Vehicle, p2: Any, p3: Any, p4: Any, p5: Any) { invoke_ignore!(0xE53D17AD837CBF7C, ped, vehicle, p2, p3, p4, p5) }
	pub fn _TASK_DISEMBARK_VEHICLE(p0: Any, vehicle: Vehicle, p2: i32, p3: Any, p4: f32, p5: Any) { invoke_ignore!(0xA7C6854BB5A4192A, p0, vehicle, p2, p3, p4, p5) }
	pub fn _TASK_BOARD_VEHICLE_2(ped: Ped, p1: Any, p2: Any, p3: f32, flags: i32) { invoke_ignore!(0xE41A09C8DDFF7AA4, ped, p1, p2, p3, flags) }
	pub fn TASK_DISEMBARK_NEAREST_TRAIN_CARRIAGE(ped: Ped, p1: f32, flags: i32) { invoke_ignore!(0x0A11F3BDEC03ED5F, ped, p1, flags) }
	pub fn TASK_ENTER_VEHICLE(ped: Ped, vehicle: Vehicle, timeout: i32, seat: i32, speed: f32, flag: i32, p6: Any) { invoke_ignore!(0xC20E50AA46D09CA8, ped, vehicle, timeout, seat, speed, flag, p6) }
	pub fn TASK_LEAVE_VEHICLE(ped: Ped, vehicle: Vehicle, flags: i32, unkPed: Ped) { invoke_ignore!(0xD3DBCE61A490BE02, ped, vehicle, flags, unkPed) }
	pub fn TASK_MOUNT_ANIMAL(ped: Ped, mount: Ped, timer: i32, seatIndex: i32, pedSpeed: f32, mountStyle: i32, p6: Any, p7: Any) { invoke_ignore!(0x92DB0739813C5186, ped, mount, timer, seatIndex, pedSpeed, mountStyle, p6, p7) }
	pub fn TASK_DISMOUNT_ANIMAL(rider: Ped, taskFlag: i32, p2: Any, p3: Any, p4: Any, targetPed: Ped) { invoke_ignore!(0x48E92D3DDE23C23A, rider, taskFlag, p2, p3, p4, targetPed) }
	pub fn TASK_HITCH_ANIMAL(ped: Ped, scenarioPoint: i32, flag: i32) { invoke_ignore!(0x9030AD4B6207BFE8, ped, scenarioPoint, flag) }
	pub fn _0xE05A5D39BE6E93AF(p0: Any) { invoke_ignore!(0xE05A5D39BE6E93AF, p0) }
	pub fn TASK_VEHICLE_DRIVE_TO_COORD(ped: Ped, vehicle: Vehicle, x: f32, y: f32, z: f32, speed: f32, style: Any, vehicleModel: Hash, drivingMode: i32, stopRange: f32, straightLineDist: f32) { invoke_ignore!(0xE2A2AA2F659D77A7, ped, vehicle, x, y, z, speed, style, vehicleModel, drivingMode, stopRange, straightLineDist) }
	pub fn _TASK_VEHICLE_DRIVE_TO_COORD_2(ped: Ped, p1: Any, p2: Any, p3: Any, p4: Any, p5: Any, p6: Any, p7: Any, p8: Any) { invoke_ignore!(0xF0108F01FB105DA2, ped, p1, p2, p3, p4, p5, p6, p7, p8) }
	pub fn TASK_VEHICLE_DRIVE_WANDER(ped: Ped, vehicle: Vehicle, speed: f32, drivingStyle: i32) { invoke_ignore!(0x480142959D337D00, ped, vehicle, speed, drivingStyle) }
	pub fn TASK_FOLLOW_TO_OFFSET_OF_ENTITY(ped: Ped, entity: Entity, offsetX: f32, offsetY: f32, offsetZ: f32, movementSpeed: f32, timeout: i32, stoppingRange: f32, persistFollowing: bool, p9: bool, walkOnly: bool, p11: bool, p12: bool, p13: bool) { invoke_ignore!(0x304AE42E357B8C7E, ped, entity, offsetX, offsetY, offsetZ, movementSpeed, timeout, stoppingRange, persistFollowing, p9, walkOnly, p11, p12, p13) }
	pub fn TASK_FOLLOW_TO_OFFSET_OF_COORD(ped: Ped, p1: Any, p2: Any, p3: Any, p4: Any, p5: Any, p6: Any, p7: Any, p8: Any, p9: Any, p10: Any, p11: Any, p12: Any, p13: Any, p14: Any) { invoke_ignore!(0x2E3676282C18A692, ped, p1, p2, p3, p4, p5, p6, p7, p8, p9, p10, p11, p12, p13, p14) }
	pub fn _0x3FFCD7BBA074CC80(ped: Ped, p1: Any, p2: Any, p3: Any, p4: Any, p5: Any, p6: Any, p7: Any, p8: Any) { invoke_ignore!(0x3FFCD7BBA074CC80, ped, p1, p2, p3, p4, p5, p6, p7, p8) }
	pub fn TASK_GO_STRAIGHT_TO_COORD(ped: Ped, x: f32, y: f32, z: f32, moveBlendSpeedY: f32, p5: i32, p6: f32, p7: f32, p8: i32) { invoke_ignore!(0xD76B57B44F1E6F8B, ped, x, y, z, moveBlendSpeedY, p5, p6, p7, p8) }
	pub fn TASK_GO_STRAIGHT_TO_COORD_RELATIVE_TO_ENTITY(ped: Ped, p1: Any, p2: Any, p3: Any, p4: Any, p5: Any, p6: Any, p7: Any) { invoke_ignore!(0x61E360B7E040D12E, ped, p1, p2, p3, p4, p5, p6, p7) }
	pub fn TASK_MOVE_IN_TRAFFIC(ped: Ped, p1: Any, p2: Any, p3: Any) { invoke_ignore!(0x8AA1593AEC087A29, ped, p1, p2, p3) }
	pub fn TASK_MOVE_IN_TRAFFIC_TO_DESTINATION(ped: Ped, p1: Any, p2: Any, p3: Any, p4: Any, p5: Any, p6: Any, p7: Any) { invoke_ignore!(0xDCA3A13F7A45338B, ped, p1, p2, p3, p4, p5, p6, p7) }
	pub fn TASK_MOVE_IN_TRAFFIC_AWAY_FROM_ENTITY(ped: Ped, p1: Any, p2: Any, p3: Any, p4: Any) { invoke_ignore!(0x13DED0BC45600FE1, ped, p1, p2, p3, p4) }
	pub fn _0xBAAB791AA72C2821(p0: Any, p1: Any) { invoke_ignore!(0xBAAB791AA72C2821, p0, p1) }
	pub fn TASK_MOVE_FOLLOW_ROAD_USING_NAVMESH(ped: Ped, moveBlendRatio: f32, x: f32, y: f32, z: f32, p5: Any) { invoke_ignore!(0x79482C12482A860D, ped, moveBlendRatio, x, y, z, p5) }
	pub fn TASK_ACHIEVE_HEADING(ped: Ped, heading: f32, timeout: i32) { invoke_ignore!(0x93B93A37987F1F3D, ped, heading, timeout) }
	pub fn TASK_FLUSH_ROUTE() { invoke_ignore!(0x841142A1376E9006) }
	pub fn TASK_EXTEND_ROUTE(x: f32, y: f32, z: f32) { invoke_ignore!(0x1E7889778264843A, x, y, z) }
	pub fn TASK_FOLLOW_POINT_ROUTE(ped: Ped, p1: Any, p2: Any, p3: Any, p4: Any, p5: Any) { invoke_ignore!(0x0E14C5550DC3CD1D, ped, p1, p2, p3, p4, p5) }
	pub fn TASK_ENTER_ANIM_SCENE(ped: Ped, animScene: AnimScene, entityName: & CStr, playbackListName: & CStr, enterSpeed: f32, bAutoStart: bool, flag: i32, p7: i32, p8: f32) { invoke_ignore!(0xC2329B0206426644, ped, animScene, entityName, playbackListName, enterSpeed, bAutoStart, flag, p7, p8) }
	pub fn TASK_MOVE_BE_IN_FORMATION(ped: Ped, p1: Any, p2: Any, p3: Any, p4: Any, p5: Any, p6: Any) { invoke_ignore!(0x4AA5AA97C65E4A2F, ped, p1, p2, p3, p4, p5, p6) }
	pub fn TASK_GO_TO_ENTITY(ped: Ped, target: Entity, duration: i32, distance: f32, speed: f32, p5: f32, p6: i32) { invoke_ignore!(0x6A071245EB0D1882, ped, target, duration, distance, speed, p5, p6) }
	pub fn TASK_FOLLOW_AND_CONVERSE_WITH_PED(ped: Ped, targetPed: Ped, p2: Any, p3: Any, p4: f32, p5: f32, p6: i32, p7: Any, p8: Any, p9: f32, p10: f32) { invoke_ignore!(0x489FFCCCE7392B55, ped, targetPed, p2, p3, p4, p5, p6, p7, p8, p9, p10) }
	pub fn _0xA052608A12559BBB(p0: Any, p1: Any) { invoke_ignore!(0xA052608A12559BBB, p0, p1) }
	pub fn TASK_WANDER_AND_CONVERSE_WITH_PED(ped: Ped, p1: Any, p2: Any, p3: Any) { invoke_ignore!(0x8AC76D1408731732, ped, p1, p2, p3) }
	pub fn TASK_LEAD_AND_CONVERSE(ped: Ped, p1: Any, p2: Any, p3: Any, p4: Any, p5: Any, p6: Any, p7: Any, p8: Any) { invoke_ignore!(0xAA19711D33C6708C, ped, p1, p2, p3, p4, p5, p6, p7, p8) }
	pub fn _0xDE0C8B145EA466FF(p0: Any, p1: Any, p2: Any, p3: Any, p4: Any, p5: Any) { invoke_ignore!(0xDE0C8B145EA466FF, p0, p1, p2, p3, p4, p5) }
	pub fn TASK_SEEK_CLEAR_LOS_TO_ENTITY(ped: Ped, entity: Entity, p2: f32, p3: f32, p4: f32) { invoke_ignore!(0x8D7F2A63688C20A4, ped, entity, p2, p3, p4) }
	pub fn TASK_GO_TO_WHISTLE(ped: Ped, p1: Ped, whistleType: i32) { invoke_ignore!(0xBAD6545608CECA6E, ped, p1, whistleType) }
	pub fn _0xEB67D4E056C85A81(p0: Any) -> Any { invoke!(0xEB67D4E056C85A81, p0) }
	pub fn _0x78D8C1D4EB80C588(p0: Any) -> Any { invoke!(0x78D8C1D4EB80C588, p0) }
	pub fn TASK_LEAD_HORSE(ped: Ped, horse: Ped) { invoke_ignore!(0x9A7A4A54596FE09D, ped, horse) }
	pub fn TASK_STOP_LEADING_HORSE(ped: Ped) { invoke_ignore!(0xED27560703F37258, ped) }
	pub fn _TASK_FLEE_FROM_COORD(p0: Any, p1: Any, p2: Any, p3: Any, p4: Any, p5: Any, p6: Any, p7: Any, p8: Any, p9: Any, p10: Any, p11: Any) { invoke_ignore!(0x6879FF208ED87F2A, p0, p1, p2, p3, p4, p5, p6, p7, p8, p9, p10, p11) }
	pub fn _TASK_FLEE_FROM_PED(ped: Ped, fleeFromTarget: Ped, x: f32, y: f32, z: f32, distance: f32, p6: i32, p7: i32, p8: f32, targetPed: Ped) { invoke_ignore!(0x7B74D8EEDE9B5727, ped, fleeFromTarget, x, y, z, distance, p6, p7, p8, targetPed) }
	pub fn TASK_SMART_FLEE_COORD(ped: Ped, x: f32, y: f32, z: f32, distance: f32, time: i32, fleeType: i32, fleeSpeed: f32) { invoke_ignore!(0x94587F17E9C365D5, ped, x, y, z, distance, time, fleeType, fleeSpeed) }
	pub fn TASK_SMART_FLEE_PED(ped: Ped, fleeFromTarget: Ped, fleeDistance: f32, fleeTime: i32, fleeType: i32, fleeSpeed: f32, targetPed: Ped) { invoke_ignore!(0x22B0D0E37CCB840D, ped, fleeFromTarget, fleeDistance, fleeTime, fleeType, fleeSpeed, targetPed) }
	pub fn _0x673A8779D229BA5A(p0: Any, p1: Any, p2: Any, p3: Any, p4: Any, p5: Any) { invoke_ignore!(0x673A8779D229BA5A, p0, p1, p2, p3, p4, p5) }
	pub fn _0x2E1D6D87346BB7D2(p0: Any, p1: Any, p2: Any, p3: Any) { invoke_ignore!(0x2E1D6D87346BB7D2, p0, p1, p2, p3) }
	pub fn TASK_FLEE_COORD(ped: Ped, x: f32, y: f32, z: f32, fleeStyle: i32, p5: i32, p6: f32, duration: i32, p8: i32) { invoke_ignore!(0x58428248BF4B64E4, ped, x, y, z, fleeStyle, p5, p6, duration, p8) }
	pub fn TASK_FLEE_PED(ped: Ped, fleeFromTarget: Ped, fleeStyle: i32, flag: i32, p4: f32, p5: i32, p6: i32) { invoke_ignore!(0xFD45175A6DFD7CE9, ped, fleeFromTarget, fleeStyle, flag, p4, p5, p6) }
	pub fn TASK_FLEE_COORD_VIA(p0: Any, p1: Any, p2: Any, p3: Any, p4: Any, p5: Any, p6: Any, p7: Any, p8: Any, p9: Any, p10: Any, p11: Any) { invoke_ignore!(0x390E0B697D25EAF5, p0, p1, p2, p3, p4, p5, p6, p7, p8, p9, p10, p11) }
	pub fn TASK_FLEE_PED_VIA(p0: Any, p1: Any, p2: Any, p3: Any, p4: Any, p5: Any, p6: Any, p7: Any, p8: Any, p9: Any) { invoke_ignore!(0x5802E0F910E4CF1D, p0, p1, p2, p3, p4, p5, p6, p7, p8, p9) }
	pub fn _ADD_FLEE_TARGET_COORDS(ped: Ped, x: f32, y: f32, z: f32, p4: f32) { invoke_ignore!(0xE8F1A5B4CED3725A, ped, x, y, z, p4) }
	pub fn ADD_FLEE_TARGET_PED(ped: Ped, targetPed: Ped, p2: f32) { invoke_ignore!(0x3923EC958249657D, ped, targetPed, p2) }
	pub fn _0xA42DC7919159CCCF(p0: Any) { invoke_ignore!(0xA42DC7919159CCCF, p0) }
	pub fn TASK_FLY_AWAY(ped: Ped, fleeFromTarget: Ped) { invoke_ignore!(0xE86A537B5A3C297C, ped, fleeFromTarget) }
	pub fn TASK_FLY_TO_COORD(ped: Ped, travelMbr: f32, x: f32, y: f32, z: f32, p5: bool, p6: bool) { invoke_ignore!(0xD6CFC2D59DA72042, ped, travelMbr, x, y, z, p5, p6) }
	pub fn TASK_FLYING_CIRCLE(ped: Ped, p1: Any, p2: Any, p3: Any, p4: Any, p5: Any, p6: Any) { invoke_ignore!(0x72997893BFB8ECCC, ped, p1, p2, p3, p4, p5, p6) }
	pub fn TASK_WALK_AWAY(ped: Ped, entity: Entity) { invoke_ignore!(0x04ACFAC71E6858F9, ped, entity) }
	pub fn TASK_SHOCKING_EVENT_REACT(ped: Ped, p1: Any, p2: Any) { invoke_ignore!(0x452419CBD838065B, ped, p1, p2) }
	pub fn TASK_REACT(ped: Ped, reactingTo: Entity, x: f32, y: f32, z: f32, reactionName: & CStr, p6: f32, p7: f32, p8: i32) { invoke_ignore!(0xC4C32C31920E1B70, ped, reactingTo, x, y, z, reactionName, p6, p7, p8) }
	pub fn TASK_WANDER_IN_AREA(ped: Ped, x: f32, y: f32, z: f32, radius: f32, p5: f32, p6: f32, p7: i32) { invoke_ignore!(0xE054346CA3A0F315, ped, x, y, z, radius, p5, p6, p7) }
	pub fn TASK_WANDER_IN_VOLUME(ped: Ped, volume: Volume, p2: f32, p3: f32, p4: i32) { invoke_ignore!(0x9FDA168777B28424, ped, volume, p2, p3, p4) }
	pub fn TASK_WANDER_STANDARD(ped: Ped, p1: f32, p2: i32) { invoke_ignore!(0xBB9CE077274F6A1B, ped, p1, p2) }
	pub fn TASK_WANDER_SWIM(ped: Ped, p1: Any) { invoke_ignore!(0x527EA3DB8BC7F03B, ped, p1) }
	pub fn TASK_PLANT_BOMB(ped: Ped, x: f32, y: f32, z: f32, heading: f32) { invoke_ignore!(0x965FEC691D55E9BF, ped, x, y, z, heading) }
	pub fn TASK_HORSE_ACTION(ped: Ped, action: i32, targetPed: Ped, p3: Any) { invoke_ignore!(0xA09CFD29100F06C3, ped, action, targetPed, p3) }
	pub fn TASK_ANIMAL_INTERACTION(ped: Ped, targetPed: Ped, interactionType: Hash, interactionModel: Hash, skipIdleAnimationClip: bool) { invoke_ignore!(0xCD181A959CFDD7F4, ped, targetPed, interactionType, interactionModel, skipIdleAnimationClip) }
	pub fn TASK_COMBAT_ANIMAL_WARN(ped: Ped, p1: Any, p2: Any) { invoke_ignore!(0xF960F3D57B660E96, ped, p1, p2) }
	pub fn TASK_COMBAT_ANIMAL_CHARGE_PED(ped: Ped, targetPed: Ped, p2: bool, p3: Any, p4: Any, p5: Any, p6: Any) { invoke_ignore!(0xEE3AA414CF99F368, ped, targetPed, p2, p3, p4, p5, p6) }
	pub fn _0x76610D12A838EBDE(p0: Any) -> Any { invoke!(0x76610D12A838EBDE, p0) }
	pub fn TASK_AMBIENT_ANIMAL_STALK(ped: Ped, p1: Any, p2: Any) { invoke_ignore!(0x37C13863ABA1B4A3, ped, p1, p2) }
	pub fn TASK_AMBIENT_ANIMAL_HUNT(ped: Ped, p1: Any, p2: Any) { invoke_ignore!(0x4B39D8F9D0FE7749, ped, p1, p2) }
	pub fn TASK_ANIMAL_UNALERTED(ped: Ped, p1: Any, p2: Any, p3: Any, p4: Any) { invoke_ignore!(0x21FDF9A25CFE1CE5, ped, p1, p2, p3, p4) }
	pub fn TASK_ANIMAL_ALERTED(ped: Ped, p1: Any, p2: Any) { invoke_ignore!(0x979D93372FC8C565, ped, p1, p2) }
	pub fn TASK_ANIMAL_FLEE(ped: Ped, targetPed: Ped, p2: Any) { invoke_ignore!(0xA899B61C66F09134, ped, targetPed, p2) }
	pub fn _0x244430C13BA5258E(p0: Any, p1: Any, p2: Any, p3: Any) -> Any { invoke!(0x244430C13BA5258E, p0, p1, p2, p3) }
	pub fn TASK_EAT(ped: Ped, p1: Any, p2: Any) { invoke_ignore!(0xBD7949BD07299672, ped, p1, p2) }
	pub fn TASK_BARK(ped: Ped, barkAtTarget: Ped, mood: Hash) { invoke_ignore!(0x83BFC1F836B2F3F2, ped, barkAtTarget, mood) }
	pub fn TASK_FOLLOW_PAVEMENT_TO_COORD(ped: Ped, args: &mut Any) { invoke_ignore!(0x1B1475414E70DD8E, ped, args) }
	pub fn TASK_FOLLOW_NAV_MESH_TO_COORD(ped: Ped, x: f32, y: f32, z: f32, speedMultiplier: f32, timeout: i32, stoppingRange: f32, flags: i32, heading: f32) { invoke_ignore!(0x15D3A79D4E44B913, ped, x, y, z, speedMultiplier, timeout, stoppingRange, flags, heading) }
	pub fn TASK_FOLLOW_NAV_MESH_TO_COORD_ADVANCED(ped: Ped, x: f32, y: f32, z: f32, speedMultiplier: f32, timeout: i32, stoppingRange: f32, flags: i32, p8: f32, p9: f32, p10: f32, entity: Entity, unk: f32) { invoke_ignore!(0x17F58B88D085DBAC, ped, x, y, z, speedMultiplier, timeout, stoppingRange, flags, p8, p9, p10, entity, unk) }
	pub fn SET_PED_PATH_CAN_USE_CLIMBOVERS(ped: Ped, toggle: bool) { invoke_ignore!(0x8E06A6FE76C9EFF4, ped, toggle) }
	pub fn SET_PED_PATH_CAN_USE_LADDERS(ped: Ped, toggle: bool) { invoke_ignore!(0x77A5B103C87F476E, ped, toggle) }
	pub fn SET_PED_PATH_CAN_DROP_FROM_HEIGHT(ped: Ped, toggle: bool) { invoke_ignore!(0xE361C5C71C431A4F, ped, toggle) }
	pub fn _0xE6A151364C600B24(p0: Any) -> Any { invoke!(0xE6A151364C600B24, p0) }
	pub fn _0x1632EB9386CDBE64(p0: Any, p1: Any) { invoke_ignore!(0x1632EB9386CDBE64, p0, p1) }
	pub fn SET_PED_PATH_CLIMB_COST_MODIFIER(ped: Ped, modifier: f32) { invoke_ignore!(0x88E32DB8C1A4AA4B, ped, modifier) }
	pub fn SET_PED_PATH_DEEP_SNOW_COST_MODIFIER(ped: Ped, modifier: f32) { invoke_ignore!(0xE8C296B75EACC357, ped, modifier) }
	pub fn SET_PED_PATH_FOLIAGE_COST_MODIFIER(ped: Ped, modifier: f32) { invoke_ignore!(0x3AD8EFF9703BE657, ped, modifier) }
	pub fn _0x8798CF6815B8FE0F(p0: Any, p1: Any) { invoke_ignore!(0x8798CF6815B8FE0F, p0, p1) }
	pub fn _0x5B68D0007D9C92EB(p0: Any, p1: Any) { invoke_ignore!(0x5B68D0007D9C92EB, p0, p1) }
	pub fn _0x82ED59F095056550(p0: Any, p1: Any) { invoke_ignore!(0x82ED59F095056550, p0, p1) }
	pub fn _0xE01C8DC8EDD28D31(p0: Any, p1: Any) { invoke_ignore!(0xE01C8DC8EDD28D31, p0, p1) }
	pub fn _0x098CAA6DBE7D8D82(p0: Any, p1: Any) { invoke_ignore!(0x098CAA6DBE7D8D82, p0, p1) }
	pub fn _GET_PED_IS_IGNORING_DEAD_BODIES(ped: Ped) -> bool { invoke!(0x1948BBE561A2375A, ped) }
	pub fn _SET_PED_IGNORE_DEAD_BODIES(ped: Ped, toggle: bool) { invoke_ignore!(0x013A7BA5015C1372, ped, toggle) }
	pub fn _SET_PED_PATH_LADDER_COST_MODIFIER(ped: Ped, modifier: f32) { invoke_ignore!(0x70F7A1EAB1AE3AA8, ped, modifier) }
	pub fn SET_PED_PATH_MAY_ENTER_WATER(ped: Ped, mayEnterWater: bool) { invoke_ignore!(0xF35425A4204367EC, ped, mayEnterWater) }
	pub fn _SET_PED_PATH_MAY_ENTER_DEEP_WATER(ped: Ped, mayEnterDeepWater: bool) { invoke_ignore!(0x9DE63896B176EA94, ped, mayEnterDeepWater) }
	pub fn _0xC6170856E54557B2(p0: Any, p1: Any, p2: Any) { invoke_ignore!(0xC6170856E54557B2, p0, p1, p2) }
	pub fn _0xF948F4356F010F11(p0: Any, p1: Any, p2: Any) { invoke_ignore!(0xF948F4356F010F11, p0, p1, p2) }
	pub fn SET_PED_PATH_PREFER_TO_AVOID_WATER(ped: Ped, avoidWater: bool, p2: f32) { invoke_ignore!(0x38FE1EC73743793C, ped, avoidWater, p2) }
	pub fn _0x8BB283A7888AD1AD(p0: Any, p1: Any, p2: Any) { invoke_ignore!(0x8BB283A7888AD1AD, p0, p1, p2) }
	pub fn _0x12990818C1D35886(p0: Any, p1: Any, p2: Any) { invoke_ignore!(0x12990818C1D35886, p0, p1, p2) }
	pub fn _0x7C015D8BCEC72CF4(p0: Any, p1: Any) { invoke_ignore!(0x7C015D8BCEC72CF4, p0, p1) }
	pub fn SET_PED_PATH_AVOID_FIRE(ped: Ped, avoidFire: bool) { invoke_ignore!(0x4455517B28441E60, ped, avoidFire) }
	pub fn _0x42CFD8FD8CC8DC69(p0: Any, p1: Any) { invoke_ignore!(0x42CFD8FD8CC8DC69, p0, p1) }
	pub fn _0x216343750545A486(p0: Any, p1: Any, p2: Any) { invoke_ignore!(0x216343750545A486, p0, p1, p2) }
	pub fn _0x06ECF3925BC2ABAE(p0: Any, p1: Any) { invoke_ignore!(0x06ECF3925BC2ABAE, p0, p1) }
	pub fn _0xFA30E2254461ADEB(p0: Any, p1: Any) { invoke_ignore!(0xFA30E2254461ADEB, p0, p1) }
	pub fn TASK_GO_TO_COORD_ANY_MEANS(ped: Ped, x: f32, y: f32, z: f32, speed: f32, entity: Entity, p6: bool, walkingStyle: i32, p8: f32) { invoke_ignore!(0x5BC448CB78FA3E88, ped, x, y, z, speed, entity, p6, walkingStyle, p8) }
	pub fn TASK_GO_TO_COORD_ANY_MEANS_EXTRA_PARAMS(ped: Ped, x: f32, y: f32, z: f32, speed: f32, p5: Any, p6: bool, walkingStyle: i32, p8: f32, p9: Any, p10: Any, p11: Any, p12: Any) { invoke_ignore!(0x1DD45F9ECFDB1BC9, ped, x, y, z, speed, p5, p6, walkingStyle, p8, p9, p10, p11, p12) }
	pub fn TASK_GO_TO_COORD_ANY_MEANS_EXTRA_PARAMS_WITH_CRUISE_SPEED(ped: Ped, p1: Any, p2: Any, p3: Any, p4: Any, p5: Any, p6: Any, p7: Any, p8: Any, p9: Any, p10: Any, p11: Any, p12: Any, p13: Any, p14: Any) { invoke_ignore!(0xB8ECD61F531A7B02, ped, p1, p2, p3, p4, p5, p6, p7, p8, p9, p10, p11, p12, p13, p14) }
	pub fn TASK_PLAY_ANIM(ped: Ped, animDict: & CStr, animName: & CStr, speed: f32, speedMultiplier: f32, duration: i32, flags: i32, playbackRate: f32, p8: bool, ikFlags: i32, p10: bool, taskFilter: & CStr, p12: bool) { invoke_ignore!(0xEA47FE3719165B94, ped, animDict, animName, speed, speedMultiplier, duration, flags, playbackRate, p8, ikFlags, p10, taskFilter, p12) }
	pub fn TASK_PLAY_ANIM_ADVANCED(ped: Ped, animDict: & CStr, animName: & CStr, posX: f32, posY: f32, posZ: f32, rotX: f32, rotY: f32, rotZ: f32, speed: f32, speedMultiplier: f32, duration: i32, flags: i32, p13: f32, p14: i32, p15: i32, p16: i32) { invoke_ignore!(0x83CDB10EA29B370B, ped, animDict, animName, posX, posY, posZ, rotX, rotY, rotZ, speed, speedMultiplier, duration, flags, p13, p14, p15, p16) }
	pub fn TASK_PLAY_UPPER_ANIM_FACING_ENTITY(ped: Ped, animDict: & CStr, animName: & CStr, entity: Entity, p4: i32, p5: f32, p6: f32, p7: i32, p8: f32, p9: bool, p10: bool, p11: f32, p12: & CStr, p13: i32, p14: f32) { invoke_ignore!(0xAD67214236AB1CFE, ped, animDict, animName, entity, p4, p5, p6, p7, p8, p9, p10, p11, p12, p13, p14) }
	pub fn STOP_ANIM_TASK(ped: Ped, animDictionary: & CStr, animationName: & CStr, p3: f32) { invoke_ignore!(0x97FF36A1D40EA00A, ped, animDictionary, animationName, p3) }
	pub fn TASK_SCRIPTED_ANIMATION(ped: Ped, args: &mut Any) { invoke_ignore!(0x126EF75F1E17ABE5, ped, args) }
	pub fn PLAY_ENTITY_SCRIPTED_ANIM(entity: Entity, args: &mut Any) { invoke_ignore!(0x77A1EEC547E7FCF1, entity, args) }
	pub fn STOP_ANIM_PLAYBACK(ped: Ped, p1: i32, p2: bool) { invoke_ignore!(0xEE08C992D238C5D1, ped, p1, p2) }
	pub fn SET_ANIM_FILTER(p0: Any, p1: Any, p2: Any, p3: Any) { invoke_ignore!(0x87B66D77D545DB66, p0, p1, p2, p3) }
	pub fn SET_ANIM_RATE(p0: Any, p1: f32, p2: Any, p3: bool) { invoke_ignore!(0x032D49C5E359C847, p0, p1, p2, p3) }
	pub fn CAN_START_ITEM_INTERACTION(ped: Ped, itemHash: Hash, interactionAnimHash: Hash, p3: i32) -> bool { invoke!(0x2D19BC4DF626CBE7, ped, itemHash, interactionAnimHash, p3) }
	pub fn START_TASK_ITEM_INTERACTION(ped: Ped, itemHash: Hash, interactionAnimHash: Hash, p3: i32, flag: i32, p5: f32) { invoke_ignore!(0xAE72E7DF013AAA61, ped, itemHash, interactionAnimHash, p3, flag, p5) }
	pub fn _TASK_ITEM_INTERACTION_2(ped: Ped, propNameGxt: Hash, prop: Object, propId: Hash, itemInteractionState: Hash, p5: i32, p6: Any, p7: f32) { invoke_ignore!(0x72F52AA2D2B172CC, ped, propNameGxt, prop, propId, itemInteractionState, p5, p6, p7) }
	pub fn _TASK_ITEM_INTERACTION_3(ped: Ped, item: Hash, guid: &mut Any, p3: Any, p4: Any, p5: Any, p6: f32) { invoke_ignore!(0xD61D5E1AD9876DEB, ped, item, guid, p3, p4, p5, p6) }
	pub fn _0xB35370D5353995CB(ped: Ped, item: Hash, p2: f32) { invoke_ignore!(0xB35370D5353995CB, ped, item, p2) }
	pub fn GET_ITEM_INTERACTION_STATE(ped: Ped) -> Hash { invoke!(0x6AA3DCA2C6F5EB6D, ped) }
	pub fn GET_ITEM_INTERACTION_ITEM_ID(ped: Ped) -> Hash { invoke!(0x804425C4BBD00883, ped) }
	pub fn IS_PED_RUNNING_INSPECTION_TASK(ped: Ped) -> bool { invoke!(0x038B1F1674F0E242, ped) }
	pub fn IS_PED_RUNNING_TASK_ITEM_INTERACTION(ped: Ped) -> bool { invoke!(0xEC7E480FF8BD0BED, ped) }
	pub fn _GET_ITEM_INTERACTION_ENTITY_FROM_PED(ped: Ped, item: Hash) -> Entity { invoke!(0x05A0100EA714DB68, ped, item) }
	pub fn GET_ITEM_INTERACTION_PROMPT_PROGRESS(ped: Ped, inputContext: Hash) -> f32 { invoke!(0xBC864A70AD55E0C1, ped, inputContext) }
	pub fn _0x678D3226CF70B9C8(ped: Ped, p1: bool) -> Object { invoke!(0x678D3226CF70B9C8, ped, p1) }
	pub fn TASK_EVASIVE_ANIM(ped1: Ped, ped2: Ped, p2: i32) { invoke_ignore!(0x5F22926E1BCE9B08, ped1, ped2, p2) }
	pub fn TASK_LOOK_AT_COORD(ped: Ped, x: f32, y: f32, z: f32, duration: i32, flags: i32, p6: i32, p7: bool) { invoke_ignore!(0x6FA46612594F7973, ped, x, y, z, duration, flags, p6, p7) }
	pub fn TASK_LOOK_AT_ENTITY(ped: Ped, lookAtTarget: Entity, duration: i32, p3: i32, p4: i32, p5: i32) { invoke_ignore!(0x69F4BE8C8CC4796C, ped, lookAtTarget, duration, p3, p4, p5) }
	pub fn TASK_CLEAR_LOOK_AT(ped: Ped) { invoke_ignore!(0x0F804F1DB19B9689, ped) }
	pub fn _0x508F5053E3F6F0C4(ped: Ped, x: f32, y: f32, z: f32, p4: f32) -> bool { invoke!(0x508F5053E3F6F0C4, ped, x, y, z, p4) }
	pub fn _0x23767D80C7EED7C6(p0: Any, p1: Any) { invoke_ignore!(0x23767D80C7EED7C6, p0, p1) }
	pub fn OPEN_SEQUENCE_TASK(taskSequenceId: &mut i32) { invoke_ignore!(0xE8854A4326B9E12B, taskSequenceId) }
	pub fn CLOSE_SEQUENCE_TASK(taskSequenceId: i32) { invoke_ignore!(0x39E72BC99E6360CB, taskSequenceId) }
	pub fn TASK_PERFORM_SEQUENCE(ped: Ped, taskSequenceId: i32) { invoke_ignore!(0x5ABA3986D90D8A3B, ped, taskSequenceId) }
	pub fn _TASK_PERFORM_SEQUENCE_2(p0: Any, p1: Any, p2: Any, p3: Any) { invoke_ignore!(0x4FC0AF869D6E309D, p0, p1, p2, p3) }
	pub fn CLEAR_SEQUENCE_TASK(taskSequenceId: &mut i32) { invoke_ignore!(0x3841422E9C488D8C, taskSequenceId) }
	pub fn SET_SEQUENCE_TO_REPEAT(taskSequenceId: i32, repeatMode: i32) { invoke_ignore!(0x58C70CF3A41E4AE7, taskSequenceId, repeatMode) }
	pub fn GET_SEQUENCE_PROGRESS(ped: Ped) -> i32 { invoke!(0x00A9010CFE1E3533, ped) }
	pub fn GET_IS_TASK_ACTIVE(ped: Ped, taskIndex: i32) -> bool { invoke!(0xB0760331C7AA4155, ped, taskIndex) }
	pub fn GET_SCRIPT_TASK_STATUS(ped: Ped, taskHash: Hash, p2: bool) -> i32 { invoke!(0x77F1BEB8863288D5, ped, taskHash, p2) }
	pub fn _0x9FF5F9B24E870748(p0: Any) -> Any { invoke!(0x9FF5F9B24E870748, p0) }
	pub fn _GET_SCRIPT_TASK_ACTION_TIME(ped: Ped, task: Hash) -> f32 { invoke!(0xA710DC5D25F8B942, ped, task) }
	pub fn REACT_LOOK_AT(ped: Ped, targetPed: Ped, lookIntensity: i32, exitAnimation: i32, duration: f32, p5: i32, targetPed2: Ped, p7: Any, p8: Any) { invoke_ignore!(0xE7FA07624574B79A, ped, targetPed, lookIntensity, exitAnimation, duration, p5, targetPed2, p7, p8) }
	pub fn REACT_LOOK_AT_END(ped: Ped, exitAnimation: i32, p2: bool) { invoke_ignore!(0x541E5B41DCA45828, ped, exitAnimation, p2) }
	pub fn IS_PED_SCENARIO_REACT_LOOKING(ped: Ped, p1: bool) -> bool { invoke!(0x916B8E075ABC8B4E, ped, p1) }
	pub fn GET_ACTIVE_VEHICLE_MISSION_TYPE(vehicle: Vehicle) -> i32 { invoke!(0x534AEBA6E5ED4CAB, vehicle) }
	pub fn TASK_LEAVE_ANY_VEHICLE(ped: Ped, p1: i32, taskFlag: i32) { invoke_ignore!(0x504D54DF3F6F2247, ped, p1, taskFlag) }
	pub fn _0xBD70108D01875299(p0: Any) -> Any { invoke!(0xBD70108D01875299, p0) }
	pub fn TASK_USE_RANDOM_SCENARIO_IN_GROUP(ped: Ped, p1: Any, p2: Any, p3: Any, p4: Any) { invoke_ignore!(0x14747F4A5971DE4E, ped, p1, p2, p3, p4) }
	pub fn TASK_AIM_GUN_AT_ENTITY(ped: Ped, targetEntity: Entity, duration: i32, p3: bool, p4: i32) { invoke_ignore!(0x9B53BB6E8943AF53, ped, targetEntity, duration, p3, p4) }
	pub fn TASK_TURN_PED_TO_FACE_ENTITY(ped: Ped, targetEntity: Entity, duration: i32, p3: f32, p4: f32, p5: f32) { invoke_ignore!(0x5AD23D40115353AC, ped, targetEntity, duration, p3, p4, p5) }
	pub fn TASK_AIM_GUN_AT_COORD(ped: Ped, x: f32, y: f32, z: f32, time: i32, p5: bool, p6: bool) { invoke_ignore!(0x6671F3EEC681BDA1, ped, x, y, z, time, p5, p6) }
	pub fn TASK_AIM_AT_COORD(ped: Ped, p1: Any, p2: Any, p3: Any, p4: Any, p5: Any, p6: Any) { invoke_ignore!(0x4AF1D73861212F52, ped, p1, p2, p3, p4, p5, p6) }
	pub fn TASK_AIM_AT_ENTITY(ped: Ped, p1: Any, p2: Any, p3: Any, p4: Any) { invoke_ignore!(0xCF7569BD0FB480A0, ped, p1, p2, p3, p4) }
	pub fn TASK_SHOOT_AT_COORD(ped: Ped, x: f32, y: f32, z: f32, duration: i32, firingPattern: Hash, p6: Any) { invoke_ignore!(0x46A6CC01E0826106, ped, x, y, z, duration, firingPattern, p6) }
	pub fn TASK_SHUFFLE_TO_NEXT_VEHICLE_SEAT(ped: Ped, vehicle: Vehicle) { invoke_ignore!(0x7AA80209BDA643EB, ped, vehicle) }
	pub fn CLEAR_PED_TASKS(ped: Ped, p1: bool, p2: bool) { invoke_ignore!(0xE1EF3C1216AFF2CD, ped, p1, p2) }
	pub fn _0x1A7D63CB1B0BB223(p0: Any) { invoke_ignore!(0x1A7D63CB1B0BB223, p0) }
	pub fn _0xB2D15D3551FE4FAE(p0: Any) { invoke_ignore!(0xB2D15D3551FE4FAE, p0) }
	pub fn _0xDF94844D474F31E5(ped: Ped) { invoke_ignore!(0xDF94844D474F31E5, ped) }
	pub fn _0xEBA2081E0A5F4D17(p0: Any) { invoke_ignore!(0xEBA2081E0A5F4D17, p0) }
	pub fn _0x141BC64C8D7C5529(vehicle: Vehicle) { invoke_ignore!(0x141BC64C8D7C5529, vehicle) }
	pub fn CLEAR_PED_SECONDARY_TASK(ped: Ped) { invoke_ignore!(0x176CECF6F920D707, ped) }
	pub fn TASK_EVERYONE_LEAVE_VEHICLE_IN_ORDER(vehicle: Vehicle, p1: bool) { invoke_ignore!(0x6F1C49F275BD25B3, vehicle, p1) }
	pub fn TASK_INVESTIGATE(ped: Ped, p1: Any, p2: Any, p3: Any, p4: Any, p5: Any) { invoke_ignore!(0x5C8514540D27FBFB, ped, p1, p2, p3, p4, p5) }
	pub fn TASK_GOTO_ENTITY_OFFSET(ped: Ped, entity: Entity, p2: Any, x: f32, y: f32, z: f32, duration: i32) { invoke_ignore!(0xE39B4FF4FDEBDE27, ped, entity, p2, x, y, z, duration) }
	pub fn TASK_GOTO_ENTITY_OFFSET_XY(ped: Ped, entity: Entity, duration: i32, targetRadius: f32, xOffset: f32, yOffset: f32, moveBlendRatio: f32, offsetFlags: i32) { invoke_ignore!(0x338E7EF52B6095A9, ped, entity, duration, targetRadius, xOffset, yOffset, moveBlendRatio, offsetFlags) }
	pub fn TASK_GOTO_ENTITY_OFFSET_XYZ(ped: Ped, p1: Any, p2: Any, p3: Any, p4: Any, p5: Any, p6: Any, p7: Any, p8: Any) { invoke_ignore!(0xFA6DA9D151769392, ped, p1, p2, p3, p4, p5, p6, p7, p8) }
	pub fn TASK_GOTO_ENTITY_OFFSET_XY_AIMING(ped: Ped, p1: Any, p2: Any, p3: Any, p4: Any, p5: Any, p6: Any, p7: Any, p8: Any) { invoke_ignore!(0x901BD69984400F62, ped, p1, p2, p3, p4, p5, p6, p7, p8) }
	pub fn TASK_GOTO_ENTITY_OFFSET_XYZ_AIMING(ped: Ped, p1: Any, p2: Any, p3: Any, p4: Any, p5: Any, p6: Any, p7: Any, p8: Any, p9: Any) { invoke_ignore!(0x41B0832CA96B5351, ped, p1, p2, p3, p4, p5, p6, p7, p8, p9) }
	pub fn TASK_FOLLOW_ENTITY_WHILE_AIMING_AT_ENTITY(ped: Ped, p1: Any, p2: Any, p3: Any, p4: Any, p5: Any, p6: Any, p7: Any) { invoke_ignore!(0x2D532EAA142CF83F, ped, p1, p2, p3, p4, p5, p6, p7) }
	pub fn TASK_TURN_PED_TO_FACE_COORD(ped: Ped, x: f32, y: f32, z: f32, duration: i32) { invoke_ignore!(0x1DDA930A0AC38571, ped, x, y, z, duration) }
	pub fn TASK_VEHICLE_TEMP_ACTION(driver: Ped, vehicle: Vehicle, action: i32, time: i32) { invoke_ignore!(0xC429DCEEB339E129, driver, vehicle, action, time) }
	pub fn TASK_VEHICLE_MISSION(driver: Ped, vehicle: Vehicle, vehicleTarget: Vehicle, missionType: i32, p4: f32, p5: Any, p6: f32, p7: f32, DriveAgainstTraffic: bool) { invoke_ignore!(0x659427E0EF36BCDE, driver, vehicle, vehicleTarget, missionType, p4, p5, p6, p7, DriveAgainstTraffic) }
	pub fn TASK_VEHICLE_DRIVE_TO_DESTINATION(driver: Ped, vehicle: Vehicle, x: f32, y: f32, z: f32, speed: f32, drivingFlags: i32, p7: i32, stoppingRange1: f32, stoppingRange2: f32, p10: bool) { invoke_ignore!(0x7F241A0D14354583, driver, vehicle, x, y, z, speed, drivingFlags, p7, stoppingRange1, stoppingRange2, p10) }
	pub fn _TASK_VEHICLE_DRIVE_TO_DESTINATION_2(vehicle: Vehicle, x: f32, y: f32, z: f32, speed: f32, p5: i32, p6: i32, p7: f32, p8: f32) { invoke_ignore!(0x391073B9D3CCE2BA, vehicle, x, y, z, speed, p5, p6, p7, p8) }
	pub fn _TASK_VEHICLE_FLEE_ON_CLEANUP(vehicle: Vehicle, p1: f32, p2: f32, p3: f32, speed: f32, type_: Hash) { invoke_ignore!(0x55CD5FDDD4335C1E, vehicle, p1, p2, p3, speed, type_) }
	pub fn TASK_VEHICLE_DRIVE_STRAIGHT_TO_POINT(driver: Ped, vehicle: Vehicle, x: f32, y: f32, z: f32, p5: f32, p6: f32, flag: i32) { invoke_ignore!(0x089FF2FB965F0A29, driver, vehicle, x, y, z, p5, p6, flag) }
	pub fn _TASK_VEHICLE_DRIVE_TO_POINT_2(vehicle: Vehicle, x: f32, y: f32, z: f32, p4: f32, p5: f32, p6: Any) { invoke_ignore!(0x6524A8981E8BE7C9, vehicle, x, y, z, p4, p5, p6) }
	pub fn _0x1D125814EBC517EB(p0: Any, p1: Any, p2: Any, p3: Any) { invoke_ignore!(0x1D125814EBC517EB, p0, p1, p2, p3) }
	pub fn _0x583AE9AF9CEE0958(vehicle: Vehicle, x: f32, y: f32, z: f32) -> bool { invoke!(0x583AE9AF9CEE0958, vehicle, x, y, z) }
	pub fn TASK_VEHICLE_MISSION_PED_TARGET(ped: Ped, vehicle: Vehicle, pedTarget: Ped, mode: i32, maxSpeed: f32, drivingStyle: i32, minDistance: f32, p7: f32, DriveAgainstTraffic: bool) { invoke_ignore!(0x9454528DF15D657A, ped, vehicle, pedTarget, mode, maxSpeed, drivingStyle, minDistance, p7, DriveAgainstTraffic) }
	pub fn _0xA263ADBBC8056214(p0: Any, p1: Any) { invoke_ignore!(0xA263ADBBC8056214, p0, p1) }
	pub fn TASK_VEHICLE_ESCORT(ped: Ped, vehicle: Vehicle, targetVehicle: Vehicle, mode: i32, speed: f32, drivingStyle: i32, minDistance: f32, p7: i32, noRoadsDistance: f32) { invoke_ignore!(0x0FA6E4B75F302400, ped, vehicle, targetVehicle, mode, speed, drivingStyle, minDistance, p7, noRoadsDistance) }
	pub fn TASK_BOAT_MISSION(pedDriver: Ped, boat: Vehicle, p2: Any, p3: Any, x: f32, y: f32, z: f32, p7: Any, maxSpeed: f32, drivingStyle: i32, p10: f32, p11: Any) { invoke_ignore!(0x15C86013127CE63F, pedDriver, boat, p2, p3, x, y, z, p7, maxSpeed, drivingStyle, p10, p11) }
	pub fn TASK_WEAPON(ped: Ped) { invoke_ignore!(0x7157B82D60E4BC46, ped) }
	pub fn TASK_DRIVE_BY(driverPed: Ped, targetPed: Ped, targetVehicle: Vehicle, targetX: f32, targetY: f32, targetZ: f32, distanceToShoot: f32, pedAccuracy: i32, p8: bool, firingPattern: Hash) { invoke_ignore!(0x2F8AF0E82773A171, driverPed, targetPed, targetVehicle, targetX, targetY, targetZ, distanceToShoot, pedAccuracy, p8, firingPattern) }
	pub fn SET_DRIVEBY_TASK_TARGET(shootingPed: Ped, targetPed: Ped, targetVehicle: Vehicle, x: f32, y: f32, z: f32) { invoke_ignore!(0xE5B302114D8162EE, shootingPed, targetPed, targetVehicle, x, y, z) }
	pub fn CLEAR_DRIVEBY_TASK_UNDERNEATH_DRIVING_TASK(ped: Ped) { invoke_ignore!(0xC35B5CDB2824CF69, ped) }
	pub fn IS_DRIVEBY_TASK_UNDERNEATH_DRIVING_TASK(ped: Ped) -> bool { invoke!(0x8785E6E40C7A8818, ped) }
	pub fn GET_IS_PED_AIMING_IN_THE_AIR(ped: Ped) -> bool { invoke!(0x8785E6E40C7A8819, ped) }
	pub fn _SET_PED_CLEAR_AIMING_IN_THE_AIR(ped: Ped, p1: Any) { invoke_ignore!(0x34C0010188D7C54A, ped, p1) }
	pub fn IS_MOUNTED_WEAPON_TASK_UNDERNEATH_DRIVING_TASK(ped: Ped) -> bool { invoke!(0xA320EF046186FA3B, ped) }
	pub fn TASK_WARP_PED_INTO_VEHICLE(ped: Ped, vehicle: Vehicle, seat: i32) { invoke_ignore!(0x9A7D091411C5F684, ped, vehicle, seat) }
	pub fn TASK_SHOOT_AT_ENTITY(entity: Entity, targetEntity: Entity, duration: i32, firingPattern: Hash, affectCockedState: bool) { invoke_ignore!(0x08DA95E8298AE772, entity, targetEntity, duration, firingPattern, affectCockedState) }
	pub fn TASK_SHOOT_WITH_WEAPON(ped: Ped, args: &mut Any) { invoke_ignore!(0x08AA95E8298AE772, ped, args) }
	pub fn _0x2416EC2F31F75266(entity: Entity, targetEntity: Entity, duration: i32, p3: Any, p4: Any) { invoke_ignore!(0x2416EC2F31F75266, entity, targetEntity, duration, p3, p4) }
	pub fn _0x41323F4E0C4AE94B(p0: Any, p1: Any, p2: Any, p3: Any, p4: Any, p5: Any, p6: Any) { invoke_ignore!(0x41323F4E0C4AE94B, p0, p1, p2, p3, p4, p5, p6) }
	pub fn _0x5EA655F01D93667A(p0: Any) -> Any { invoke!(0x5EA655F01D93667A, p0) }
	pub fn TASK_CLIMB(ped: Ped, unused: bool) { invoke_ignore!(0x89D9FCC2435112F1, ped, unused) }
	pub fn _TASK_CLIMB_2(ped: Ped, heading: f32) { invoke_ignore!(0xDF1D85BCAF60D537, ped, heading) }
	pub fn TASK_CLIMB_LADDER(ped: Ped, p1: f32, p2: bool, p3: bool) { invoke_ignore!(0xB6C987F9285A3814, ped, p1, p2, p3) }
	pub fn CLEAR_PED_TASKS_IMMEDIATELY(ped: Ped, p1: bool, resetCrouch: bool) { invoke_ignore!(0xAAA34F8A7CB32098, ped, p1, resetCrouch) }
	pub fn TASK_PERFORM_SEQUENCE_FROM_PROGRESS(ped: Ped, p1: Any, p2: Any, p3: Any) { invoke_ignore!(0x89221B16730234F0, ped, p1, p2, p3) }
	pub fn SET_PED_DESIRED_MOVE_BLEND_RATIO(ped: Ped, p1: f32) { invoke_ignore!(0x1E982AC8716912C5, ped, p1) }
	pub fn GET_PED_DESIRED_MOVE_BLEND_RATIO(ped: Ped) -> f32 { invoke!(0x8517D4A6CA8513ED, ped) }
	pub fn TASK_GOTO_ENTITY_AIMING(ped: Ped, target: Entity, distanceToStopAt: f32, StartAimingDist: f32) { invoke_ignore!(0xA9DA48FAB8A76C12, ped, target, distanceToStopAt, StartAimingDist) }
	pub fn TASK_SET_SPHERE_DEFENSIVE_AREA(ped: Ped, p1: f32, p2: f32, p3: f32, p4: f32) { invoke_ignore!(0x933C06518B52A9A4, ped, p1, p2, p3, p4) }
	pub fn TASK_CLEAR_DEFENSIVE_AREA(ped: Ped) { invoke_ignore!(0x95A6C46A31D1917D, ped) }
	pub fn TASK_PED_SLIDE_TO_COORD(ped: Ped, x: f32, y: f32, z: f32, heading: f32, p5: f32) { invoke_ignore!(0xD04FE6765D990A06, ped, x, y, z, heading, p5) }
	pub fn _0x9420FB11B8D77948(p0: Any) -> Any { invoke!(0x9420FB11B8D77948, p0) }
	pub fn _0x6BA606AB3A83BC4D(p0: Any) -> Any { invoke!(0x6BA606AB3A83BC4D, p0) }
	pub fn ADD_COVER_POINT(p0: f32, p1: f32, p2: f32, p3: f32, p4: Any, p5: Any, p6: Any, p7: bool) -> ScrHandle { invoke!(0xD5C12A75C7B9497F, p0, p1, p2, p3, p4, p5, p6, p7) }
	pub fn _0x59872EA4CBD11C56(p0: Any, p1: Any, p2: Any, p3: Any, p4: Any, p5: Any, p6: Any, p7: Any, p8: Any) -> Any { invoke!(0x59872EA4CBD11C56, p0, p1, p2, p3, p4, p5, p6, p7, p8) }
	pub fn REMOVE_COVER_POINT(coverpoint: ScrHandle) { invoke_ignore!(0xAE287C923D891715, coverpoint) }
	pub fn DOES_SCRIPTED_COVER_POINT_EXIST_AT_COORDS(p0: Any, p1: Any, p2: Any, p3: Any) -> bool { invoke!(0xA98B8E3C088E5A31, p0, p1, p2, p3) }
	pub fn GET_SCRIPTED_COVER_POINT_COORDS(coverpoint: ScrHandle) -> Vector3 { invoke!(0x594A1028FC2A3E85, coverpoint) }
	pub fn _0xE116F6F2DA2D777E(p0: Any) -> Vector3 { invoke!(0xE116F6F2DA2D777E, p0) }
	pub fn _0x50AA09A0DA64E73C(p0: Any, p1: Any, p2: Any, p3: Any, p4: Any, p5: Any, p6: Any) { invoke_ignore!(0x50AA09A0DA64E73C, p0, p1, p2, p3, p4, p5, p6) }
	pub fn _0xE5831AA1E2FD147C(p0: Any) { invoke_ignore!(0xE5831AA1E2FD147C, p0) }
	pub fn TASK_COMBAT_PED(ped: Ped, targetPed: Ped, p2: i32, p3: i32) { invoke_ignore!(0xF166E48407BAC484, ped, targetPed, p2, p3) }
	pub fn TASK_COMBAT_PED_TIMED(ped: Ped, targetPed: Ped, p2: i32, p3: Any) { invoke_ignore!(0x944F30DCB7096BDE, ped, targetPed, p2, p3) }
	pub fn _TASK_COMBAT_PED_3(p0: Any, p1: Any, p2: Any, p3: Any, p4: Any, p5: Any) { invoke_ignore!(0xC624414FA748B9BA, p0, p1, p2, p3, p4, p5) }
	pub fn TASK_SEEK_COVER_FROM_POS(ped: Ped, x: f32, y: f32, z: f32, duration: i32, p5: Any, p6: Any, p7: Any) { invoke_ignore!(0x75AC2B60386D89F2, ped, x, y, z, duration, p5, p6, p7) }
	pub fn TASK_SEEK_COVER_FROM_PED(ped: Ped, fromPed: Ped, duration: i32, p3: Any, p4: Any, p5: Any) { invoke_ignore!(0x84D32B3BEC531324, ped, fromPed, duration, p3, p4, p5) }
	pub fn TASK_SEEK_COVER_TO_COVER_POINT(ped: Ped, p1: Any, p2: Any, p3: Any, p4: Any, p5: Any, p6: Any, p7: Any, p8: Any) { invoke_ignore!(0xD43D95C7A869447F, ped, p1, p2, p3, p4, p5, p6, p7, p8) }
	pub fn TASK_SEEK_COVER_TO_COORDS(ped: Ped, p1: Any, p2: Any, p3: Any, p4: Any, p5: Any, p6: Any, p7: Any, p8: Any, p9: Any, p10: Any) { invoke_ignore!(0x39246A6958EF072C, ped, p1, p2, p3, p4, p5, p6, p7, p8, p9, p10) }
	pub fn TASK_PUT_PED_DIRECTLY_INTO_COVER(ped: Ped, x: f32, y: f32, z: f32, timeout: i32, p5: bool, p6: f32, p7: Any, p8: Any, coverpoint: ScrHandle, p10: bool, p11: bool, p12: Any) { invoke_ignore!(0x4172393E6BE1FECE, ped, x, y, z, timeout, p5, p6, p7, p8, coverpoint, p10, p11, p12) }
	pub fn _TASK_PUT_PED_DIRECTLY_INTO_COVER_FROM_COORDS(ped: Ped, x: f32, y: f32, z: f32, fromX: f32, fromY: f32, fromZ: f32, timeout: i32, p8: Any, p9: Any, p10: Any, p11: Any, p12: Any, p13: Any, p14: Any, p15: Any, p16: Any, p17: Any) { invoke_ignore!(0xDF8A5855B9F9A97B, ped, x, y, z, fromX, fromY, fromZ, timeout, p8, p9, p10, p11, p12, p13, p14, p15, p16, p17) }
	pub fn TASK_PUT_PED_DIRECTLY_INTO_MELEE(ped: Ped, meleeTarget: Ped, meleeStyle: Hash, p3: f32, animBlendRatio: f32, p5: bool, p6: i32) { invoke_ignore!(0x1C6CD14A876FFE39, ped, meleeTarget, meleeStyle, p3, animBlendRatio, p5, p6) }
	pub fn TASK_PUT_PED_DIRECTLY_INTO_GRAPPLE(ped: Ped, grappleTarget: Ped, grappleStyle: Hash, p3: f32, p4: f32, p5: bool, p6: i32) { invoke_ignore!(0xA05F3F20889D7A5B, ped, grappleTarget, grappleStyle, p3, p4, p5, p6) }
	pub fn TASK_COMPANION_AMBIENT(ped: Ped, p1: Any) { invoke_ignore!(0xE017CF6E2527FE4F, ped, p1) }
	pub fn _0x098036CAB8373D36(p0: Any) { invoke_ignore!(0x098036CAB8373D36, p0) }
	pub fn _0x10C44F633E2D6D9E(p0: Any) { invoke_ignore!(0x10C44F633E2D6D9E, p0) }
	pub fn _0x7FB78B2199C10E92(p0: Any) { invoke_ignore!(0x7FB78B2199C10E92, p0) }
	pub fn TASK_GUARD(ped: Ped, p1: Any, p2: Any) { invoke_ignore!(0xB9FB242EACCAF30F, ped, p1, p2) }
	pub fn TASK_GUARD_CURRENT_POSITION(ped: Ped, p1: f32, p2: f32, p3: bool) { invoke_ignore!(0x4A58A47A72E3FCB4, ped, p1, p2, p3) }
	pub fn _TASK_GUARD_ASSIGNED_DEFENSIVE_AREA_2(ped: Ped, p1: Any, p2: Any, p3: Any, p4: Any, p5: Any, p6: Any, p7: Any) { invoke_ignore!(0x1FC9B33976BACD6C, ped, p1, p2, p3, p4, p5, p6, p7) }
	pub fn TASK_GUARD_ASSIGNED_DEFENSIVE_AREA(ped: Ped, p1: f32, p2: f32, p3: f32, p4: f32, p5: f32, p6: Any) { invoke_ignore!(0xD2A207EEBDF9889B, ped, p1, p2, p3, p4, p5, p6) }
	pub fn TASK_STAND_GUARD(ped: Ped, x: f32, y: f32, z: f32, heading: f32, scenarioName: & CStr) { invoke_ignore!(0xAE032F8BBA959E90, ped, x, y, z, heading, scenarioName) }
	pub fn SET_DRIVE_TASK_CRUISE_SPEED(driver: Ped, cruiseSpeed: f32) { invoke_ignore!(0x5C9B84BD7D31D908, driver, cruiseSpeed) }
	pub fn SET_DRIVE_TASK_MAX_CRUISE_SPEED(ped: Ped, maxCruiseSpeed: f32) { invoke_ignore!(0x404A5AA9B9F0B746, ped, maxCruiseSpeed) }
	pub fn ADD_COVER_BLOCKING_AREA(playerX: f32, playerY: f32, playerZ: f32, radiusX: f32, radiusY: f32, radiusZ: f32, p6: bool, p7: bool, p8: bool, p9: bool) { invoke_ignore!(0x45C597097DD7CB81, playerX, playerY, playerZ, radiusX, radiusY, radiusZ, p6, p7, p8, p9) }
	pub fn _ADD_COVER_BLOCKING_VOLUME(volume: Volume, p1: bool, p2: bool, p3: bool, p4: bool) { invoke_ignore!(0xEB2ED1DC3AEC0654, volume, p1, p2, p3, p4) }
	pub fn REMOVE_ALL_COVER_BLOCKING_AREAS() { invoke_ignore!(0xDB6708C0B46F56D8) }
	pub fn _0x2A10538D0A005E81(p0: Any, p1: Any) { invoke_ignore!(0x2A10538D0A005E81, p0, p1) }
	pub fn _0x4F57397388E1DFF8() { invoke_ignore!(0x4F57397388E1DFF8) }
	pub fn TASK_ROB_PED(ped: Ped, p1: Any, p2: Any, p3: Any, p4: Any) { invoke_ignore!(0x7BB967F85D8CCBDB, ped, p1, p2, p3, p4) }
	pub fn _0xBEDBE39B5FD98FD6(ped: Ped) -> bool { invoke!(0xBEDBE39B5FD98FD6, ped) }
	pub fn CREATE_SCENARIO_POINT_HASH(scenarioHash: Hash, x: f32, y: f32, z: f32, heading: f32, p5: Any, p6: Any, p7: bool) -> i32 { invoke!(0x94B745CE41DB58A1, scenarioHash, x, y, z, heading, p5, p6, p7) }
	pub fn CREATE_SCENARIO_POINT_HASH_ATTACHED_TO_ENTITY(entity: Entity, scenarioHash: Hash, x: f32, y: f32, z: f32, heading: f32, p6: Any, p7: Any, p8: bool) -> i32 { invoke!(0x794AB1379A74064D, entity, scenarioHash, x, y, z, heading, p6, p7, p8) }
	pub fn _DOES_SCENARIO_POINT_HAVE_PROPS(scenario: i32) -> bool { invoke!(0xEA31F199A73801D3, scenario) }
	pub fn GET_PROP_FOR_SCENARIO_POINT(scenarioPoint: i32, name: & CStr) -> Entity { invoke!(0x295514F198EFD0CA, scenarioPoint, name) }
	pub fn _ASSOCIATE_PROP_WITH_SCENARIO(scenario: i32, entity: Entity, propName: & CStr, p3: bool) -> bool { invoke!(0x8360C47380B6F351, scenario, entity, propName, p3) }
	pub fn _SET_SCENARIO_POINT_FLAG(scenario: i32, flag: i32, value: bool) { invoke_ignore!(0x5AF19B6CC2115D34, scenario, flag, value) }
	pub fn _IS_SCENARIO_POINT_FLAG_SET(scenario: i32, flag: i32) -> bool { invoke!(0x8569C38D2FB80650, scenario, flag) }
	pub fn _0xADC45010BC17AF0E(p0: Any, p1: Any) { invoke_ignore!(0xADC45010BC17AF0E, p0, p1) }
	pub fn _0x974DA3408DEC4E79(p0: Any) -> Any { invoke!(0x974DA3408DEC4E79, p0) }
	pub fn _DISASSOCIATE_PROP_FROM_SCENARIO(scenario: i32, propName: & CStr) -> bool { invoke!(0x6EF4E31B4D5D2DA0, scenario, propName) }
	pub fn DOES_SCENARIO_POINT_EXIST(scenario: i32) -> bool { invoke!(0x841475AC96E794D1, scenario) }
	pub fn _0x22CD2C33ED4467A1(p0: Any) -> Any { invoke!(0x22CD2C33ED4467A1, p0) }
	pub fn _GET_SCENARIO_POINT_ENTITY(scenario: i32) -> Entity { invoke!(0x7467165EE97D3C68, scenario) }
	pub fn _GET_PED_USING_SCENARIO_POINT(scenario: i32) -> Ped { invoke!(0x5BA659955369B0E2, scenario) }
	pub fn _GET_SCENARIO_POINT_COORDS(scenario: i32, p1: bool) -> Vector3 { invoke!(0xA8452DD321607029, scenario, p1) }
	pub fn _0x91CB5E431F579BA1(p0: Any) -> Vector3 { invoke!(0x91CB5E431F579BA1, p0) }
	pub fn _0x370F57C47F68EBCA(p0: Any) -> Any { invoke!(0x370F57C47F68EBCA, p0) }
	pub fn _GET_SCENARIO_POINT_HEADING(scenario: i32, p1: bool) -> f32 { invoke!(0xB93EA7184BAA85C3, scenario, p1) }
	pub fn _GET_SCENARIO_POINT_RADIUS(scenario: i32) -> f32 { invoke!(0x6718F40313A2B5A6, scenario) }
	pub fn _SET_SCENARIO_POINT_COORDS(scenario: i32, xPos: f32, yPos: f32, zPos: f32, p4: bool) { invoke_ignore!(0x2056AB38DF06825C, scenario, xPos, yPos, zPos, p4) }
	pub fn _SET_SCENARIO_POINT_HEADING(scenario: i32, heading: f32, p2: bool) { invoke_ignore!(0xD3A0DA8F91612C6E, scenario, heading, p2) }
	pub fn _SET_SCENARIO_POINT_RADIUS(scenario: i32, radius: f32) { invoke_ignore!(0xC47D9080A9A8856A, scenario, radius) }
	pub fn _0xA7479FB665361EDB(p0: Any, p1: Any) { invoke_ignore!(0xA7479FB665361EDB, p0, p1) }
	pub fn _0xE69FDA40AAC3EFC0(p0: Any, p1: Any) { invoke_ignore!(0xE69FDA40AAC3EFC0, p0, p1) }
	pub fn GET_SCENARIO_POINTS_IN_AREA(posX: f32, posY: f32, posZ: f32, radius: f32, scenariosInRadius: &mut Any, size: i32) -> i32 { invoke!(0x345EC3B7EBDE1CB5, posX, posY, posZ, radius, scenariosInRadius, size) }
	pub fn _0xEFD875C2791EBEFD(p0: Any, p1: Any, p2: Any, p3: Any) -> Any { invoke!(0xEFD875C2791EBEFD, p0, p1, p2, p3) }
	pub fn _0x152664AA3188B193(p0: Any, p1: Any, p2: Any, p3: Any, p4: Any, p5: Any) -> Any { invoke!(0x152664AA3188B193, p0, p1, p2, p3, p4, p5) }
	pub fn _0xE7BBC4E56B989449(p0: Any, p1: Any, p2: Any) -> Any { invoke!(0xE7BBC4E56B989449, p0, p1, p2) }
	pub fn _GET_SCENARIO_POINT_PED_IS_USING(ped: Ped, p1: bool) -> i32 { invoke!(0xDF7993356F52359A, ped, p1) }
	pub fn GET_RANSACK_SCENARIO_POINT_PED_IS_USING(ped: Ped) -> Any { invoke!(0xD04241BBF6D03A5E, ped) }
	pub fn _SET_SCENARIO_CONTAINER_OPENING_STATE(entity: Entity, open: bool) { invoke_ignore!(0x188F8071F244B9B8, entity, open) }
	pub fn _0xA9E7672F8C6C6F74(p0: Any) -> Any { invoke!(0xA9E7672F8C6C6F74, p0) }
	pub fn _0x0A98A362C5A19A43(p0: Any) -> Any { invoke!(0x0A98A362C5A19A43, p0) }
	pub fn _0x849791EBBDBA0362(p0: Any) -> Any { invoke!(0x849791EBBDBA0362, p0) }
	pub fn _0x640A602946A8C972(p0: Any) -> Any { invoke!(0x640A602946A8C972, p0) }
	pub fn _0x01AF8A3729231A43(p0: Any) -> Any { invoke!(0x01AF8A3729231A43, p0) }
	pub fn _GET_SCENARIO_CONTAINER_OPENING_STATE(entity: Entity) -> bool { invoke!(0xB219612B5568E9EC, entity) }
	pub fn _RESET_SCENARIO_FOR_ENTITY(scenario: i32, entity: Entity) { invoke_ignore!(0x2E20878FD208A68E, scenario, entity) }
	pub fn _0x4161648394262FDF(p0: Any, p1: Any, p2: Any, p3: Any) { invoke_ignore!(0x4161648394262FDF, p0, p1, p2, p3) }
	pub fn _0x9C8F42A5D1859DC1(p0: Any) { invoke_ignore!(0x9C8F42A5D1859DC1, p0) }
	pub fn _DELETE_SCENARIO_POINT(scenario: i32) { invoke_ignore!(0x81948DFE4F5A0283, scenario) }
	pub fn TASK_USE_SCENARIO_POINT(ped: Ped, scenario: i32, conditionalAnim: & CStr, p3: i32, p4: bool, p5: bool, p6: Hash, p7: bool, p8: f32, p9: bool) { invoke_ignore!(0xCCDAE6324B6A821C, ped, scenario, conditionalAnim, p3, p4, p5, p6, p7, p8, p9) }
	pub fn _TASK_USE_SCENARIO_POINT_2(ped: Ped, ped2: Ped, p2: Any, p3: & CStr, p4: i32, p5: Hash, p6: f32, p7: bool) { invoke_ignore!(0x0F6641449DD86FBE, ped, ped2, p2, p3, p4, p5, p6, p7) }
	pub fn TASK_START_SCENARIO_IN_PLACE_HASH(ped: Ped, scenarioHash: Hash, duration: i32, playEnterAnim: bool, conditionalHash: Hash, heading: f32, p6: bool) { invoke_ignore!(0x524B54361229154F, ped, scenarioHash, duration, playEnterAnim, conditionalHash, heading, p6) }
	pub fn _TASK_START_SCENARIO_IN_PLACE_2(ped: Ped, p1: Any, p2: & CStr, p3: i32, p4: bool, p5: f32, p6: bool) { invoke_ignore!(0xA917E39F2CEFD215, ped, p1, p2, p3, p4, p5, p6) }
	pub fn TASK_START_SCENARIO_AT_POSITION(ped: Ped, scenarioHash: Hash, x: f32, y: f32, z: f32, heading: f32, duration: i32, sittingScenario: bool, teleport: bool, p9: & CStr, p10: f32, p11: bool) { invoke_ignore!(0x4D1F61FC34AF3CD1, ped, scenarioHash, x, y, z, heading, duration, sittingScenario, teleport, p9, p10, p11) }
	pub fn _0xF97F462779B31786(p0: Any) -> Any { invoke!(0xF97F462779B31786, p0) }
	pub fn _0x6C269F673C47031E(p0: Any) -> Any { invoke!(0x6C269F673C47031E, p0) }
	pub fn _0x9667CCE29BFA0780(p0: Any) { invoke_ignore!(0x9667CCE29BFA0780, p0) }
	pub fn _0x00FFE0F85253C572(p0: Any) -> Any { invoke!(0x00FFE0F85253C572, p0) }
	pub fn _TASK_USE_NEAREST_SCENARIO_TO_COORD(ped: Ped, x: f32, y: f32, z: f32, distance: f32, duration: i32, p6: bool, p7: bool, p8: bool, p9: bool) { invoke_ignore!(0x322BFDEA666E2B0E, ped, x, y, z, distance, duration, p6, p7, p8, p9) }
	pub fn TASK_USE_NEAREST_SCENARIO_TO_COORD_WARP(ped: Ped, x: f32, y: f32, z: f32, distance: f32, duration: i32, p6: bool, p7: bool, p8: bool, p9: bool) { invoke_ignore!(0x58E2E0F23F6B76C3, ped, x, y, z, distance, duration, p6, p7, p8, p9) }
	pub fn TASK_USE_NEAREST_TRAIN_SCENARIO_TO_COORD_WARP(ped: Ped, x: f32, y: f32, z: f32, distance: f32) { invoke_ignore!(0x3774B03456DD6106, ped, x, y, z, distance) }
	pub fn TASK_USE_NEAREST_SCENARIO_CHAIN_TO_COORD(ped: Ped, x: f32, y: f32, z: f32, distance: f32, p5: bool, p6: bool, p7: bool, p8: bool) { invoke_ignore!(0x9FDA1B3D7E7028B3, ped, x, y, z, distance, p5, p6, p7, p8) }
	pub fn TASK_USE_NEAREST_SCENARIO_CHAIN_TO_COORD_WARP(ped: Ped, x: f32, y: f32, z: f32, distance: f32, p5: bool, p6: bool, p7: bool, p8: bool) { invoke_ignore!(0x97A28E63F0BA5631, ped, x, y, z, distance, p5, p6, p7, p8) }
	pub fn _0xFDECCA06E8B81346(ped: Ped) -> Any { invoke!(0xFDECCA06E8B81346, ped) }
	pub fn _0x2D657B10F211C572(ped: Ped, p1: f32) -> Any { invoke!(0x2D657B10F211C572, ped, p1) }
	pub fn TASK_RIDE_TRAIN(ped: Ped, train: Vehicle, scenarioPoint: i32, scenarioHash: Hash) { invoke_ignore!(0x37FB1C870E2EC2C6, ped, train, scenarioPoint, scenarioHash) }
	pub fn _0x79197F7D2BB5E73A(p0: Any, p1: Any, p2: Any, p3: Any, p4: Any, p5: Any) -> Any { invoke!(0x79197F7D2BB5E73A, p0, p1, p2, p3, p4, p5) }
	pub fn DOES_SCENARIO_EXIST_IN_AREA(x: f32, y: f32, z: f32, radius: f32, p4: bool, p5: Any, p6: bool) -> bool { invoke!(0x5A59271FFADD33C1, x, y, z, radius, p4, p5, p6) }
	pub fn DOES_SCENARIO_OF_TYPE_EXIST_IN_AREA_HASH(x: f32, y: f32, z: f32, typeHash: Hash, radius: f32, p5: bool) -> bool { invoke!(0x6EEAD6AF637DA752, x, y, z, typeHash, radius, p5) }
	pub fn FIND_SCENARIO_OF_TYPE_HASH(xPos: f32, yPos: f32, zPos: f32, scenarioType: Hash, distance: f32, p5: Any, p6: bool) -> i32 { invoke!(0xF533D68FF970D190, xPos, yPos, zPos, scenarioType, distance, p5, p6) }
	pub fn _0x0D322AEF8878B8FE(p0: Any, p1: Any) { invoke_ignore!(0x0D322AEF8878B8FE, p0, p1) }
	pub fn _0xD508FA229F1C4900(p0: Any, p1: Any, p2: Any, p3: Any, p4: Any, p5: Any) -> Any { invoke!(0xD508FA229F1C4900, p0, p1, p2, p3, p4, p5) }
	pub fn _0xB8E213D02F37947D(p0: Any, p1: Any, p2: Any, p3: Any, p4: Any, p5: Any, p6: Any) { invoke_ignore!(0xB8E213D02F37947D, p0, p1, p2, p3, p4, p5, p6) }
	pub fn IS_SCENARIO_OCCUPIED(p0: f32, p1: f32, p2: f32, p3: f32, p4: bool) -> bool { invoke!(0x788756D73AC2E07C, p0, p1, p2, p3, p4) }
	pub fn _0x1ACBC313966C21F3(scenario: i32) -> Any { invoke!(0x1ACBC313966C21F3, scenario) }
	pub fn PED_HAS_USE_SCENARIO_TASK(ped: Ped) -> bool { invoke!(0x295E3CCEC879CCD7, ped) }
	pub fn _PED_IS_IN_SCENARIO_BASE(ped: Ped) -> bool { invoke!(0x02EBBB3989B7E695, ped) }
	pub fn _0x90703A8F75EE4ABD(p0: Any, p1: Any) -> Any { invoke!(0x90703A8F75EE4ABD, p0, p1) }
	pub fn _0xD999E379265A4501(p0: Any, p1: Any, p2: Any) { invoke_ignore!(0xD999E379265A4501, p0, p1, p2) }
	pub fn PLAY_ANIM_ON_RUNNING_SCENARIO(ped: Ped, animDict: & CStr, animName: & CStr) { invoke_ignore!(0x748040460F8DF5DC, ped, animDict, animName) }
	pub fn _0x74F0209674864CBD() -> Any { invoke!(0x74F0209674864CBD) }
	pub fn _0xE1C105E6BBA48270() -> Any { invoke!(0xE1C105E6BBA48270) }
	pub fn _0x1AC5A8AB50CFAA33(p0: Any) -> Any { invoke!(0x1AC5A8AB50CFAA33, p0) }
	pub fn _0xBEEFBB608D2AA68A(p0: Any) { invoke_ignore!(0xBEEFBB608D2AA68A, p0) }
	pub fn _0x19BC99C678FBA139(p0: Any, p1: Any, p2: Any) { invoke_ignore!(0x19BC99C678FBA139, p0, p1, p2) }
	pub fn _0x5D9B0BAAF04CF65B(p0: Any, p1: Any, p2: Any, p3: Any) { invoke_ignore!(0x5D9B0BAAF04CF65B, p0, p1, p2, p3) }
	pub fn _0x9B6A58FDB0024F12(p0: Any, p1: Any) { invoke_ignore!(0x9B6A58FDB0024F12, p0, p1) }
	pub fn _0xBC3F847AE2C3DC65(p0: Any, p1: Any) { invoke_ignore!(0xBC3F847AE2C3DC65, p0, p1) }
	pub fn _0x450080DDEDB91258(p0: Any, p1: Any) { invoke_ignore!(0x450080DDEDB91258, p0, p1) }
	pub fn _0x954451EA2D2120FB(p0: Any, p1: Any) { invoke_ignore!(0x954451EA2D2120FB, p0, p1) }
	pub fn _0x0F4F6C4CE471259D(p0: Any, p1: Any) { invoke_ignore!(0x0F4F6C4CE471259D, p0, p1) }
	pub fn _0xB8E3486D107F4194(p0: Any, p1: Any) { invoke_ignore!(0xB8E3486D107F4194, p0, p1) }
	pub fn _0x827A58CED9D4D5B4(p0: Any, p1: Any) { invoke_ignore!(0x827A58CED9D4D5B4, p0, p1) }
	pub fn _0x4A7D73989F52EB37(p0: Any, p1: Any) { invoke_ignore!(0x4A7D73989F52EB37, p0, p1) }
	pub fn _0xB79817DB31FF72B9(p0: Any, p1: Any) { invoke_ignore!(0xB79817DB31FF72B9, p0, p1) }
	pub fn _0x65D281985F2BDFC2(p0: Any, p1: Any) { invoke_ignore!(0x65D281985F2BDFC2, p0, p1) }
	pub fn _0x885D19AC2B6FBFF4(p0: Any, p1: Any) { invoke_ignore!(0x885D19AC2B6FBFF4, p0, p1) }
	pub fn _0x2064B33F6E6B92D4(p0: Any, p1: Any, p2: Any, p3: Any) { invoke_ignore!(0x2064B33F6E6B92D4, p0, p1, p2, p3) }
	pub fn _0xCE4E669400E5F8AA(p0: Any, p1: Any, p2: Any, p3: Any) { invoke_ignore!(0xCE4E669400E5F8AA, p0, p1, p2, p3) }
	pub fn _0x2EB977293923C723(p0: Any, p1: Any) { invoke_ignore!(0x2EB977293923C723, p0, p1) }
	pub fn _0xE9225354FB7437A7(p0: Any, p1: Any) { invoke_ignore!(0xE9225354FB7437A7, p0, p1) }
	pub fn _0x764DB5A48390FBAD(p0: Any, p1: Any) { invoke_ignore!(0x764DB5A48390FBAD, p0, p1) }
	pub fn _0x8F8C84363810691A(p0: Any, p1: Any) { invoke_ignore!(0x8F8C84363810691A, p0, p1) }
	pub fn _0xFF8AFCA532B500D4(p0: Any, p1: Any) { invoke_ignore!(0xFF8AFCA532B500D4, p0, p1) }
	pub fn _0xFE5D28B9B7837CC1(p0: Any, p1: Any, p2: Any, p3: Any) -> Any { invoke!(0xFE5D28B9B7837CC1, p0, p1, p2, p3) }
	pub fn _0x2B8AF29A78024BD3(p0: Any) { invoke_ignore!(0x2B8AF29A78024BD3, p0) }
	pub fn _0x0365000D8BF86531(p0: Any) -> Any { invoke!(0x0365000D8BF86531, p0) }
	pub fn _0x865732725536EE39(p0: Any) -> Vector3 { invoke!(0x865732725536EE39, p0) }
	pub fn _0x0E184495B27BB57D() { invoke_ignore!(0x0E184495B27BB57D) }
	pub fn DOES_SCENARIO_GROUP_EXIST(scenarioGroup: & CStr) -> bool { invoke!(0xF9034C136C9E00D3, scenarioGroup) }
	pub fn _DOES_SCENARIO_GROUP_EXIST_HASH(scenarioGroup: Hash) -> bool { invoke!(0x76E98B52369A289C, scenarioGroup) }
	pub fn IS_SCENARIO_GROUP_ENABLED(scenarioGroup: & CStr) -> bool { invoke!(0x367A09DED4E05B99, scenarioGroup) }
	pub fn _IS_SCENARIO_GROUP_ENABLED_HASH(scenarioGroup: Hash) -> bool { invoke!(0xDCC374913DE6AAA6, scenarioGroup) }
	pub fn SET_SCENARIO_GROUP_ENABLED(scenarioGroup: & CStr, toggle: bool) { invoke_ignore!(0x02C8E5B49848664E, scenarioGroup, toggle) }
	pub fn _SET_SCENARIO_GROUP_ENABLED_HASH(scenarioGroup: Hash, toggle: bool) { invoke_ignore!(0x9925EDDB6EAB88CD, scenarioGroup, toggle) }
	pub fn RESET_SCENARIO_GROUPS_ENABLED() { invoke_ignore!(0xDD902D0349AFAD3A) }
	pub fn _0x358A1A751B335A11(p0: Any) { invoke_ignore!(0x358A1A751B335A11, p0) }
	pub fn FORCE_SCENARIO_GROUP_PRIORITY(p0: Any, p1: Any) { invoke_ignore!(0x444C910A5058E568, p0, p1) }
	pub fn _0xE55478C5EDF70AC2(p0: Any) -> Any { invoke!(0xE55478C5EDF70AC2, p0) }
	pub fn _IS_SCENARIO_POINT_ACTIVE(scenario: i32) -> bool { invoke!(0x0CC36D4156006509, scenario) }
	pub fn _SET_SCENARIO_POINT_ACTIVE(scenario: i32, active: bool) { invoke_ignore!(0xEEE4829304F93EEE, scenario, active) }
	pub fn _RESET_SCENARIO_SCRIPT(scenario: i32) { invoke_ignore!(0x5A40040BB5AE3EA2, scenario) }
	pub fn IS_SCENARIO_TYPE_ENABLED(scenarioType: & CStr) -> bool { invoke!(0x3A815DB3EA088722, scenarioType) }
	pub fn SET_SCENARIO_TYPE_ENABLED(scenarioType: & CStr, toggle: bool) { invoke_ignore!(0xEB47EC4E34FB7EE1, scenarioType, toggle) }
	pub fn _SET_SCENARIO_TYPE_ENABLED_HASH(scenarioType: Hash, toggle: bool) { invoke_ignore!(0xD00E50E673802D71, scenarioType, toggle) }
	pub fn RESET_SCENARIO_TYPES_ENABLED() { invoke_ignore!(0x0D40EE2A7F2B2D6D) }
	pub fn _GET_SCENARIO_POINT_TYPE_PED_IS_USING(ped: Ped) -> i32 { invoke!(0x2D0571BB55879DA2, ped) }
	pub fn _GET_SCENARIO_POINT_TYPE(scenario: i32) -> Hash { invoke!(0xA92450B5AE687AAF, scenario) }
	pub fn IS_PED_ACTIVE_IN_SCENARIO(ped: Ped, scenario: i32) -> bool { invoke!(0xAA135F9482C82CC3, ped, scenario) }
	pub fn IS_PED_EXITING_SCENARIO(ped: Ped, p1: bool) -> bool { invoke!(0x0C3CB2E600C8977D, ped, p1) }
	pub fn _0x2C497BDEF897C6DF(p0: Any) -> Any { invoke!(0x2C497BDEF897C6DF, p0) }
	pub fn TASK_COMBAT_HATED_TARGETS_IN_AREA(ped: Ped, x: f32, y: f32, z: f32, radius: f32, flags: i32, p6: Any) { invoke_ignore!(0x4CF5F55DAC3280A0, ped, x, y, z, radius, flags, p6) }
	pub fn TASK_COMBAT_HATED_TARGETS_NO_LOS_TEST(ped: Ped, radius: f32) { invoke_ignore!(0xB5BC69D9C4060BC3, ped, radius) }
	pub fn TASK_COMBAT_HATED_TARGETS_AROUND_PED(ped: Ped, radius: f32, flags: i32, p3: Any) { invoke_ignore!(0x7BF835BB9E2698C8, ped, radius, flags, p3) }
	pub fn TASK_COMBAT_HATED_TARGETS_AROUND_PED_TIMED(ped: Ped, radius: f32, time: i32, flags: i32) { invoke_ignore!(0x2BBA30B854534A0C, ped, radius, time, flags) }
	pub fn TASK_COMBAT_HATED_TARGETS(ped: Ped, radius: f32) { invoke_ignore!(0x8182B561A29BD597, ped, radius) }
	pub fn _0x30B391915538EBE2(p0: Any) { invoke_ignore!(0x30B391915538EBE2, p0) }
	pub fn TASK_THROW_PROJECTILE(ped: Ped, p1: Any, p2: Any, p3: Any) { invoke_ignore!(0x7285951DBF6B5A51, ped, p1, p2, p3) }
	pub fn _TASK_THROW_PROJECTILE_2(p0: Any, p1: Any, p2: Any, p3: Any) { invoke_ignore!(0x7282356DFF6B5A51, p0, p1, p2, p3) }
	pub fn TASK_WHISTLE_ANIM(ped: Ped, audPedWhistleType: Hash, p2: Hash) { invoke_ignore!(0xD6401A1B2F63BED6, ped, audPedWhistleType, p2) }
	pub fn TASK_SWAP_WEAPON(ped: Ped, p1: Any, p2: Any, p3: Any, p4: Any) { invoke_ignore!(0xA21C51255B205245, ped, p1, p2, p3, p4) }
	pub fn _0x0000A8ACDC2E1B6A(p0: Any, p1: Any) { invoke_ignore!(0x0000A8ACDC2E1B6A, p0, p1) }
	pub fn TASK_RELOAD_WEAPON(ped: Ped, unused: bool) { invoke_ignore!(0x62D2916F56B9CD2D, ped, unused) }
	pub fn TASK_PICK_UP_WEAPON(ped: Ped, p1: Any) { invoke_ignore!(0x55B0ECFD98596624, ped, p1) }
	pub fn IS_PED_GETTING_UP(ped: Ped) -> bool { invoke!(0x2A74E1D5F2F00EEC, ped) }
	pub fn TASK_ANIMAL_WRITHE(ped: Ped, p1: Any, p2: Any) { invoke_ignore!(0x8C038A39C4A4B6D6, ped, p1, p2) }
	pub fn _TASK_ANIMAL_BLEED_OUT(ped: Ped, killer: Ped, p2: bool, weaponHash: Hash, p4: i32, p5: i32) { invoke_ignore!(0x30A768C30D385EC5, ped, killer, p2, weaponHash, p4, p5) }
	pub fn IS_PED_IN_WRITHE(ped: Ped) -> bool { invoke!(0xDEB6D52126E7D640, ped) }
	pub fn _0x3F8387DB1B9F31B7(p0: Any, p1: Any) -> Any { invoke!(0x3F8387DB1B9F31B7, p0, p1) }
	pub fn _0x756C7B4C43DF0422(p0: Any) -> Any { invoke!(0x756C7B4C43DF0422, p0) }
	pub fn _0x351F74ED6177EBE7() -> Any { invoke!(0x351F74ED6177EBE7) }
	pub fn _0x6C50B9DCCCA70023(p0: Any) -> Any { invoke!(0x6C50B9DCCCA70023, p0) }
	pub fn TASK_REVIVE_TARGET(ped: Ped, reviver: Ped, tool: Hash) { invoke_ignore!(0x356088527D9EBAAD, ped, reviver, tool) }
	pub fn OPEN_PATROL_ROUTE(patrolRoute: & CStr) { invoke_ignore!(0xA36BFB5EE89F3D82, patrolRoute) }
	pub fn CLOSE_PATROL_ROUTE() { invoke_ignore!(0xB043ECA801B8CBC1) }
	pub fn ADD_PATROL_ROUTE_NODE(nodeId: i32, scenarioName: & CStr, x: f32, y: f32, z: f32, lookPosX: f32, lookPosY: f32, lookPosZ: f32, duration: i32, p9: bool) { invoke_ignore!(0x8EDF950167586B7C, nodeId, scenarioName, x, y, z, lookPosX, lookPosY, lookPosZ, duration, p9) }
	pub fn ADD_PATROL_ROUTE_LINK(node1: i32, node2: i32) { invoke_ignore!(0x23083260DEC3A551, node1, node2) }
	pub fn CREATE_PATROL_ROUTE() { invoke_ignore!(0xAF8A443CCC8018DC) }
	pub fn DELETE_PATROL_ROUTE(patrolRoute: & CStr) { invoke_ignore!(0x7767DD9D65E91319, patrolRoute) }
	pub fn _0x643FD1556F621772(p0: Any, p1: Any, p2: Any) -> Any { invoke!(0x643FD1556F621772, p0, p1, p2) }
	pub fn TASK_PATROL(ped: Ped, patrolRoute: & CStr, p2: Any, p3: bool, p4: bool) { invoke_ignore!(0xBDA5DF49D080FE4E, ped, patrolRoute, p2, p3, p4) }
	pub fn _TASK_PATROL_2(p0: Any, p1: Any, p2: Any, p3: Any, p4: Any, p5: Any, p6: Any, p7: Any) { invoke_ignore!(0x964B06C88E4C86DB, p0, p1, p2, p3, p4, p5, p6, p7) }
	pub fn TASK_STAY_IN_COVER(ped: Ped) { invoke_ignore!(0xE5DA8615A6180789, ped) }
	pub fn TASK_VEHICLE_SHOOT_AT_PED(ped: Ped, target: Ped, p2: f32) { invoke_ignore!(0x10AB107B887214D8, ped, target, p2) }
	pub fn TASK_VEHICLE_AIM_AT_PED(ped: Ped, target: Ped) { invoke_ignore!(0xE41885592B08B097, ped, target) }
	pub fn TASK_VEHICLE_SHOOT_AT_COORD(ped: Ped, x: f32, y: f32, z: f32, p4: f32) { invoke_ignore!(0x5190796ED39C9B6D, ped, x, y, z, p4) }
	pub fn _0xAF2EF28CE3084505(p0: Any, p1: Any, p2: Any, p3: Any) { invoke_ignore!(0xAF2EF28CE3084505, p0, p1, p2, p3) }
	pub fn TASK_VEHICLE_AIM_AT_COORD(ped: Ped, x: f32, y: f32, z: f32) { invoke_ignore!(0x447C1E9EF844BC0F, ped, x, y, z) }
	pub fn TASK_VEHICLE_GOTO_NAVMESH(ped: Ped, vehicle: Vehicle, x: f32, y: f32, z: f32, speed: f32, behaviorFlag: i32, stoppingRange: f32) { invoke_ignore!(0x195AEEB13CEFE2EE, ped, vehicle, x, y, z, speed, behaviorFlag, stoppingRange) }
	pub fn TASK_GO_TO_COORD_WHILE_AIMING_AT_COORD(ped: Ped, p1: Any, p2: Any, p3: Any, p4: Any, p5: Any, p6: Any, p7: Any, p8: Any, p9: Any, p10: Any, p11: Any, p12: Any, p13: Any, p14: Any, p15: Any) { invoke_ignore!(0x11315AB3385B8AC0, ped, p1, p2, p3, p4, p5, p6, p7, p8, p9, p10, p11, p12, p13, p14, p15) }
	pub fn TASK_GO_TO_COORD_WHILE_AIMING_AT_COORD_USING_COMBAT_STYLE(ped: Ped, p1: Any, p2: Any, p3: Any, p4: Any, p5: Any, p6: Any, p7: Any, p8: Any, p9: Any, p10: Any, p11: Any, p12: Any, p13: Any, p14: Any, p15: Any) { invoke_ignore!(0x639C0425A0B4E77E, ped, p1, p2, p3, p4, p5, p6, p7, p8, p9, p10, p11, p12, p13, p14, p15) }
	pub fn TASK_GO_TO_COORD_WHILE_AIMING_AT_ENTITY(ped1: Ped, x: f32, y: f32, z: f32, ped2: Ped, p5: f32, p6: Any, p7: f32, p8: f32, p9: Any, p10: Any, p11: Any, firingPattern: Hash, p13: i32, p14: Any) { invoke_ignore!(0xB2A16444EAD9AE47, ped1, x, y, z, ped2, p5, p6, p7, p8, p9, p10, p11, firingPattern, p13, p14) }
	pub fn TASK_GO_TO_COORD_WHILE_AIMING_AT_ENTITY_USING_COMBAT_STYLE(ped: Ped, p1: Any, p2: Any, p3: Any, p4: Any, p5: Any, p6: Any, p7: Any, p8: Any, p9: Any, p10: Any, p11: Any, p12: Any, p13: Any, p14: Any) { invoke_ignore!(0x78426D0982D083C9, ped, p1, p2, p3, p4, p5, p6, p7, p8, p9, p10, p11, p12, p13, p14) }
	pub fn TASK_GO_TO_ENTITY_WHILE_AIMING_AT_ENTITY(ped: Ped, p1: Any, p2: Any, p3: Any, p4: Any, p5: Any, p6: Any, p7: Any, p8: Any, p9: Any, p10: Any) { invoke_ignore!(0x97465886D35210E9, ped, p1, p2, p3, p4, p5, p6, p7, p8, p9, p10) }
	pub fn TASK_GO_TO_ENTITY_WHILE_AIMING_AT_ENTITY_USING_COMBAT_STYLE(ped: Ped, p1: Any, p2: Any, p3: Any, p4: Any, p5: Any, p6: Any, p7: Any, p8: Any, p9: Any, p10: Any) { invoke_ignore!(0xCEF0117C233026AD, ped, p1, p2, p3, p4, p5, p6, p7, p8, p9, p10) }
	pub fn TASK_GO_TO_COORD_AND_AIM_AT_HATED_ENTITIES_NEAR_COORD(ped: Ped, goToLocationX: f32, goToLocationY: f32, goToLocationZ: f32, focusLocationX: f32, focusLocationY: f32, focusLocationZ: f32, speed: f32, shootAtEnemies: bool, distanceToStopAt: f32, noRoadsDistance: f32, unkTrue: bool, unkFlag: i32, aimingFlag: i32, firingPattern: Hash) { invoke_ignore!(0xA55547801EB331FC, ped, goToLocationX, goToLocationY, goToLocationZ, focusLocationX, focusLocationY, focusLocationZ, speed, shootAtEnemies, distanceToStopAt, noRoadsDistance, unkTrue, unkFlag, aimingFlag, firingPattern) }
	pub fn TASK_GO_TO_COORD_AND_AIM_AT_HATED_ENTITIES_NEAR_COORD_USING_COMBAT_STYLE(ped: Ped, p1: Any, p2: Any, p3: Any, p4: Any, p5: Any, p6: Any, p7: Any, p8: Any, p9: Any, p10: Any, p11: Any, p12: Any, p13: Any, p14: Any) { invoke_ignore!(0x87BD711FC31EA273, ped, p1, p2, p3, p4, p5, p6, p7, p8, p9, p10, p11, p12, p13, p14) }
	pub fn SET_HIGH_FALL_TASK(ped: Ped, p1: i32, p2: i32, p3: i32) { invoke_ignore!(0x8C825BDC7741D37C, ped, p1, p2, p3) }
	pub fn _0x5217B7B6DB78E1F3(p0: Any, p1: Any, p2: Any, p3: Any, p4: Any) { invoke_ignore!(0x5217B7B6DB78E1F3, p0, p1, p2, p3, p4) }
	pub fn REQUEST_WAYPOINT_RECORDING(waypointRecording: & CStr) { invoke_ignore!(0x9EEFB62EB27B5792, waypointRecording) }
	pub fn GET_IS_WAYPOINT_RECORDING_LOADED(waypointRecording: & CStr) -> bool { invoke!(0xCB4E8BE8A0063C5D, waypointRecording) }
	pub fn REMOVE_WAYPOINT_RECORDING(waypointRecording: & CStr) { invoke_ignore!(0xFF1B8B4AA1C25DC8, waypointRecording) }
	pub fn _0xF718931A82EEB898() { invoke_ignore!(0xF718931A82EEB898) }
	pub fn WAYPOINT_RECORDING_GET_NUM_POINTS(waypointRecording: & CStr, points: &mut i32) -> bool { invoke!(0x5343532C01A07234, waypointRecording, points) }
	pub fn WAYPOINT_RECORDING_GET_COORD(waypointRecording: & CStr, point: i32, coord: &mut Vector3) -> bool { invoke!(0x2FB897405C90B361, waypointRecording, point, coord) }
	pub fn WAYPOINT_RECORDING_GET_SPEED_AT_POINT(waypointRecording: & CStr, point: i32) -> f32 { invoke!(0x005622AEBC33ACA9, waypointRecording, point) }
	pub fn WAYPOINT_RECORDING_GET_CLOSEST_WAYPOINT(waypointRecording: & CStr, x: f32, y: f32, z: f32, point: &mut i32) -> bool { invoke!(0xB629A298081F876F, waypointRecording, x, y, z, point) }
	pub fn TASK_FOLLOW_WAYPOINT_RECORDING_ADVANCED(ped: Ped, p1: Any) { invoke_ignore!(0x0CFC13EBC19BCA52, ped, p1) }
	pub fn TASK_FOLLOW_WAYPOINT_RECORDING(ped: Ped, waypointRecording: & CStr, p2: i32, flag: i32, p4: i32, p5: bool, p6: Any, p7: i32) { invoke_ignore!(0x0759591819534F7B, ped, waypointRecording, p2, flag, p4, p5, p6, p7) }
	pub fn TASK_FOLLOW_WAYPOINT_RECORDING_AT_OFFSET(ped: Ped, waypointRecording: & CStr, p2: f32, p3: i32, p4: i32, p5: i32, p6: bool) { invoke_ignore!(0xBE9B0520BD7C445B, ped, waypointRecording, p2, p3, p4, p5, p6) }
	pub fn TASK_FOLLOW_ENTITY_ALONG_WAYPOINT_RECORDING_AT_OFFSET(ped0: Ped, ped1: Ped, waypointRecording: & CStr, p3: f32, p4: f32, p5: i32, p6: i32, p7: i32, p8: bool) { invoke_ignore!(0x4D2B787BAE9AB760, ped0, ped1, waypointRecording, p3, p4, p5, p6, p7, p8) }
	pub fn IS_WAYPOINT_PLAYBACK_GOING_ON_FOR_PED(ped: Ped, waypointRecording: & CStr) -> bool { invoke!(0xE03B3F2D3DC59B64, ped, waypointRecording) }
	pub fn GET_PED_WAYPOINT_PROGRESS(ped: Ped) -> i32 { invoke!(0x2720AAA75001E094, ped) }
	pub fn GET_PED_WAYPOINT_DISTANCE(ped: Ped) -> f32 { invoke!(0xE6A877C64CAF1BC5, ped) }
	pub fn SET_PED_WAYPOINT_ROUTE_OFFSET(ped: Ped, p1: f32, p2: f32, p3: f32) -> Any { invoke!(0xED98E10B0AFCE4B4, ped, p1, p2, p3) }
	pub fn GET_WAYPOINT_DISTANCE_ALONG_ROUTE(waypointRecording: & CStr, p1: i32) -> f32 { invoke!(0xA5B769058763E497, waypointRecording, p1) }
	pub fn _0x3ACC128510142B9D(waypointRecording: & CStr, x: f32, y: f32, z: f32) -> f32 { invoke!(0x3ACC128510142B9D, waypointRecording, x, y, z) }
	pub fn WAYPOINT_PLAYBACK_GET_IS_PAUSED(ped: Ped) -> bool { invoke!(0x701375A7D43F01CB, ped) }
	pub fn WAYPOINT_PLAYBACK_GET_IS_AIMING(ped: Ped) -> bool { invoke!(0xD73A5D1F0325C71C, ped) }
	pub fn WAYPOINT_PLAYBACK_GET_IS_SHOOTING(ped: Ped) -> bool { invoke!(0xA5B94DF8AF058F46, ped) }
	pub fn WAYPOINT_PLAYBACK_PAUSE(ped: Ped, p1: Any, p2: Any, p3: Any) { invoke_ignore!(0x0F342546AA06FED5, ped, p1, p2, p3) }
	pub fn WAYPOINT_PLAYBACK_RESUME(ped: Ped, p1: bool, p2: i32, p3: i32) { invoke_ignore!(0x244F70C84C547D2D, ped, p1, p2, p3) }
	pub fn WAYPOINT_PLAYBACK_OVERRIDE_SPEED(ped: Ped, speed: f32, p2: Any, p3: Any, p4: Any) { invoke_ignore!(0x7D7D2B47FA788E85, ped, speed, p2, p3, p4) }
	pub fn WAYPOINT_PLAYBACK_USE_DEFAULT_SPEED(ped: Ped) { invoke_ignore!(0x6599D834B12D0800, ped) }
	pub fn GET_PED_WAYPOINT_OVERRIDE_SPEED(ped: Ped) -> f32 { invoke!(0xD39A2F3E7FCAFF08, ped) }
	pub fn USE_WAYPOINT_RECORDING_AS_ASSISTED_MOVEMENT_ROUTE(waypointRecording: & CStr, p1: bool, p2: f32, p3: f32, p4: bool) { invoke_ignore!(0x5A353B8E6B1095B5, waypointRecording, p1, p2, p3, p4) }
	pub fn WAYPOINT_PLAYBACK_START_AIMING_AT_PED(p0: Any, p1: Any, p2: Any, p3: Any) { invoke_ignore!(0x20E330937C399D29, p0, p1, p2, p3) }
	pub fn WAYPOINT_PLAYBACK_START_AIMING_AT_ENTITY(p0: Any, p1: Any, p2: Any, p3: Any) { invoke_ignore!(0x4F158205E0C74385, p0, p1, p2, p3) }
	pub fn WAYPOINT_PLAYBACK_START_AIMING_AT_COORD(p0: Any, p1: Any, p2: Any, p3: Any, p4: Any, p5: Any) { invoke_ignore!(0x8968400D900ED8B3, p0, p1, p2, p3, p4, p5) }
	pub fn WAYPOINT_PLAYBACK_START_SHOOTING_AT_PED(p0: Any, p1: Any, p2: Any, p3: Any, p4: Any) { invoke_ignore!(0xE70BA7B90F8390DC, p0, p1, p2, p3, p4) }
	pub fn WAYPOINT_PLAYBACK_START_SHOOTING_AT_ENTITY(p0: Any, p1: Any, p2: Any, p3: Any, p4: Any) { invoke_ignore!(0x4AF458F71C1196D2, p0, p1, p2, p3, p4) }
	pub fn WAYPOINT_PLAYBACK_START_SHOOTING_AT_COORD(p0: Any, p1: Any, p2: Any, p3: Any, p4: Any, p5: Any, p6: Any) { invoke_ignore!(0x057A25CFCC9DB671, p0, p1, p2, p3, p4, p5, p6) }
	pub fn WAYPOINT_PLAYBACK_STOP_AIMING_OR_SHOOTING(p0: Any) { invoke_ignore!(0x47EFA040EBB8E2EA, p0) }
	pub fn ASSISTED_MOVEMENT_REMOVE_ROUTE(route: & CStr) { invoke_ignore!(0x3548536485DD792B, route) }
	pub fn _CREATE_WAYPOINT_PATH(pathName: & CStr, p1: &mut Any, nodes: i32, p3: i32) -> bool { invoke!(0x5C885E0978B6AD60, pathName, p1, nodes, p3) }
	pub fn ASSISTED_MOVEMENT_IS_ROUTE_LOADED(route: & CStr) -> bool { invoke!(0x60F9A4393A21F741, route) }
	pub fn ASSISTED_MOVEMENT_SET_ROUTE_PROPERTIES(route: & CStr, props: i32) { invoke_ignore!(0xD5002D78B7162E1B, route, props) }
	pub fn SET_ENABLE_SPEED_RESTRAIN_FOR_WAYPOINT_RECORDING_LEADER(p0: Any, p1: Any) { invoke_ignore!(0x295F03DC97BEEBC1, p0, p1) }
	pub fn SET_UP_SPEED_RESTRAIN_INFORMATION_FOR_PLAYER_FOLLOWER(p0: Any, p1: Any, p2: Any, p3: Any, p4: Any, p5: Any, p6: Any, p7: Any, p8: Any) { invoke_ignore!(0xB5C51DD544F14F58, p0, p1, p2, p3, p4, p5, p6, p7, p8) }
	pub fn TASK_VEHICLE_FOLLOW_WAYPOINT_RECORDING(ped: Ped, vehicle: Vehicle, waypointRecording: & CStr, drivingMode: i32, p4: Any, eWaypoint: i32, flag: i32, p7: f32, p8: bool, stoppingDist: f32, p10: Any) { invoke_ignore!(0x3123FAA6DB1CF7ED, ped, vehicle, waypointRecording, drivingMode, p4, eWaypoint, flag, p7, p8, stoppingDist, p10) }
	pub fn _TASK_VEHICLE_FOLLOW_WAYPOINT_RECORDING_2(p0: Any, p1: Any, p2: Any, p3: Any, p4: Any, p5: Any, p6: Any, p7: Any, p8: Any, p9: Any) { invoke_ignore!(0x041D17A9E221AE30, p0, p1, p2, p3, p4, p5, p6, p7, p8, p9) }
	pub fn IS_WAYPOINT_PLAYBACK_GOING_ON_FOR_VEHICLE(p0: Any, p1: Any) -> bool { invoke!(0xF5134943EA29868C, p0, p1) }
	pub fn GET_VEHICLE_WAYPOINT_PROGRESS(vehicle: Vehicle) -> i32 { invoke!(0x9824CFF8FC66E159, vehicle) }
	pub fn GET_VEHICLE_WAYPOINT_TARGET_POINT(vehicle: Vehicle) -> i32 { invoke!(0x416B62AC8B9E5BBD, vehicle) }
	pub fn VEHICLE_WAYPOINT_PLAYBACK_PAUSE(vehicle: Vehicle) { invoke_ignore!(0x8A4E6AC373666BC5, vehicle) }
	pub fn VEHICLE_WAYPOINT_PLAYBACK_GET_IS_PAUSED(p0: Any) -> Any { invoke!(0x4D6D30AB18B0B089, p0) }
	pub fn VEHICLE_WAYPOINT_PLAYBACK_RESUME(vehicle: Vehicle) { invoke_ignore!(0xDC04FCAA7839D492, vehicle) }
	pub fn VEHICLE_WAYPOINT_PLAYBACK_USE_DEFAULT_SPEED(vehicle: Vehicle) { invoke_ignore!(0x5CEB25A7D2848963, vehicle) }
	pub fn VEHICLE_WAYPOINT_PLAYBACK_OVERRIDE_SPEED(vehicle: Vehicle, speed: f32) { invoke_ignore!(0x121F0593E0A431D7, vehicle, speed) }
	pub fn GET_VEHICLE_WAYPOINT_PLAYBACK_OVERRIDE_SPEED(p0: Any) -> Any { invoke!(0x3DC971EB22F73447, p0) }
	pub fn TASK_SET_BLOCKING_OF_NON_TEMPORARY_EVENTS(ped: Ped, toggle: bool) { invoke_ignore!(0x90D2156198831D69, ped, toggle) }
	pub fn TASK_SET_STEALTH_MOVEMENT(ped: Ped, p1: bool, p2: Any, p3: bool) { invoke_ignore!(0x4C3FA937B44A90FA, ped, p1, p2, p3) }
	pub fn TASK_SET_CROUCH_MOVEMENT(ped: Ped, p1: bool, p2: Any, p3: bool) { invoke_ignore!(0x17293C633C8AC019, ped, p1, p2, p3) }
	pub fn TASK_FORCE_MOTION_STATE(ped: Ped, motionStateHash: Hash, p2: bool) { invoke_ignore!(0x4F056E1AFFEF17AB, ped, motionStateHash, p2) }
	pub fn TASK_MOVE_NETWORK_BY_NAME(ped: Ped, task: & CStr, multiplier: f32, p3: bool, animDict: & CStr, flags: i32) { invoke_ignore!(0x2D537BA194896636, ped, task, multiplier, p3, animDict, flags) }
	pub fn TASK_MOVE_NETWORK_BY_NAME_WITH_INIT_PARAMS(ped: Ped, moveNetworkDefName: & CStr, taskData: &mut Any, p3: f32, p4: bool, animDict: & CStr, flags: i32) { invoke_ignore!(0x139805C2A67C4795, ped, moveNetworkDefName, taskData, p3, p4, animDict, flags) }
	pub fn TASK_MOVE_NETWORK_ADVANCED_BY_NAME_WITH_INIT_PARAMS(ped: Ped, moveNetworkDefName: & CStr, taskData: &mut Any, xPos: f32, yPos: f32, zPos: f32, xRot: f32, yRot: f32, zRot: f32, p9: i32, p10: f32, p11: i32, p12: i32, flag: i32, p14: i32) { invoke_ignore!(0x7B6A04F98BBAFB2C, ped, moveNetworkDefName, taskData, xPos, yPos, zPos, xRot, yRot, zRot, p9, p10, p11, p12, flag, p14) }
	pub fn TASK_MOVE_NETWORK_ADVANCED_BY_NAME_WITH_INIT_PARAMS_ATTACHED(ped: Ped, p1: Any, p2: Any, p3: Any, p4: Any, p5: Any, p6: Any, p7: Any, p8: Any, p9: Any, p10: Any, p11: Any, p12: Any, p13: Any, p14: Any, p15: Any, p16: Any, p17: Any) { invoke_ignore!(0xF92171093BCABED4, ped, p1, p2, p3, p4, p5, p6, p7, p8, p9, p10, p11, p12, p13, p14, p15, p16, p17) }
	pub fn IS_TASK_MOVE_NETWORK_ACTIVE(ped: Ped) -> bool { invoke!(0x921CE12C489C4C41, ped) }
	pub fn _GET_TASK_MOVE_NETWORK_ID(ped: Ped) -> Hash { invoke!(0xCACC2F9D994504B7, ped) }
	pub fn IS_TASK_MOVE_NETWORK_READY_FOR_TRANSITION(ped: Ped) -> bool { invoke!(0x30ED88D5E0C56A37, ped) }
	pub fn REQUEST_TASK_MOVE_NETWORK_STATE_TRANSITION(ped: Ped, name: & CStr) { invoke_ignore!(0xD01015C7316AE176, ped, name) }
	pub fn GET_TASK_MOVE_NETWORK_STATE(ped: Ped) -> *const char { invoke!(0x717E4D1F2048376D, ped) }
	pub fn _0xE9A6400D1A0E7A55(p0: Any) -> Any { invoke!(0xE9A6400D1A0E7A55, p0) }
	pub fn _0x615DC4A82E90BB48(p0: Any, p1: Any, p2: Any) { invoke_ignore!(0x615DC4A82E90BB48, p0, p1, p2) }
	pub fn SET_TASK_MOVE_NETWORK_SIGNAL_FLOAT(ped: Ped, signalName: & CStr, value: f32) { invoke_ignore!(0xD5BB4025AE449A4E, ped, signalName, value) }
	pub fn _SET_TASK_MOVE_NETWORK_SIGNAL_FLOAT_2(ped: Ped, signalName: & CStr, value: f32) { invoke_ignore!(0x099D4A855D53B03B, ped, signalName, value) }
	pub fn SET_TASK_MOVE_NETWORK_SIGNAL_BOOL(ped: Ped, signalName: & CStr, value: bool) { invoke_ignore!(0xB0A6CFD2C69C1088, ped, signalName, value) }
	pub fn _SET_TASK_MOVE_NETWORK_SIGNAL_VECTOR(ped: Ped, signalName: & CStr, x: f32, y: f32, z: f32) { invoke_ignore!(0x4662BFE01938D98D, ped, signalName, x, y, z) }
	pub fn _GET_TASK_MOVE_NETWORK_PHASE_FLOAT(ped: Ped, phaseName: & CStr) -> f32 { invoke!(0x844CEEE428EA35B0, ped, phaseName) }
	pub fn GET_TASK_MOVE_NETWORK_EVENT(ped: Ped, eventName: & CStr) -> bool { invoke!(0xB4F47213DF45A64C, ped, eventName) }
	pub fn _0x9585FF23C4B8EDE0(p0: Any, p1: Any) { invoke_ignore!(0x9585FF23C4B8EDE0, p0, p1) }
	pub fn _0xEAF87DA2BE78A15B(p0: Any, p1: Any) { invoke_ignore!(0xEAF87DA2BE78A15B, p0, p1) }
	pub fn _0x3BBEECC5B8F35318(p0: Any, p1: Any) { invoke_ignore!(0x3BBEECC5B8F35318, p0, p1) }
	pub fn IS_MOVE_BLEND_RATIO_STILL(moveBlendRatio: f32) -> bool { invoke!(0x349CE7B56DAFD95C, moveBlendRatio) }
	pub fn IS_MOVE_BLEND_RATIO_WALKING(moveBlendRatio: f32) -> bool { invoke!(0xF133BBBE91E1691F, moveBlendRatio) }
	pub fn IS_MOVE_BLEND_RATIO_RUNNING(moveBlendRatio: f32) -> bool { invoke!(0xD4D8636C0199A939, moveBlendRatio) }
	pub fn IS_MOVE_BLEND_RATIO_SPRINTING(moveBlendRatio: f32) -> bool { invoke!(0x24A2AD74FA9814E2, moveBlendRatio) }
	pub fn IS_PED_STILL(ped: Ped) -> bool { invoke!(0xAC29253EEF8F0180, ped) }
	pub fn IS_PED_WALKING(ped: Ped) -> bool { invoke!(0xDE4C184B2B9B071A, ped) }
	pub fn IS_PED_RUNNING(ped: Ped) -> bool { invoke!(0xC5286FFC176F28A2, ped) }
	pub fn IS_PED_SPRINTING(ped: Ped) -> bool { invoke!(0x57E457CD2C0FC168, ped) }
	pub fn IS_PED_IN_HIT_REACT(ped: Ped) -> bool { invoke!(0xF330A5C062B29BED, ped) }
	pub fn TASK_ARREST_PED(ped: Ped, target: Ped) { invoke_ignore!(0xF3B9A78A178572B1, ped, target) }
	pub fn IS_PED_BEING_ARRESTED(ped: Ped) -> bool { invoke!(0x90A09F3A45FED688, ped) }
	pub fn _IS_PED_ARRESTING_ANY_PED(ped: Ped) -> bool { invoke!(0xA9CC7856D52DBD25, ped) }
	pub fn _CUFF_PED(ped: Ped) { invoke_ignore!(0x7981037A96E7D174, ped) }
	pub fn UNCUFF_PED(ped: Ped) { invoke_ignore!(0x67406F2C8F87FC4F, ped) }
	pub fn IS_PED_CUFFED(ped: Ped) -> bool { invoke!(0x74E559B3BC910685, ped) }
	pub fn _IS_PED_DUELLING(ped: Ped) -> bool { invoke!(0xC8B29D18022EA2B7, ped) }
	pub fn TASK_DUEL(ped: Ped, p1: Any, p2: f32, entity: Entity, p4: f32, p5: i32, vPosOpponentX: f32, vPosOpponentY: f32, vPosOpponentZ: f32, fOpponentHead: f32, p10: i32) { invoke_ignore!(0x5D5B0D5BC3626E5A, ped, p1, p2, entity, p4, p5, vPosOpponentX, vPosOpponentY, vPosOpponentZ, fOpponentHead, p10) }
	pub fn _0x908BB14BCE85C80E(p0: Any) -> Any { invoke!(0x908BB14BCE85C80E, p0) }
	pub fn _0x1F7A9A9C38C13A56(p0: Any) -> Any { invoke!(0x1F7A9A9C38C13A56, p0) }
	pub fn _0x3FEB770D8ED9047A(p0: Any) -> Any { invoke!(0x3FEB770D8ED9047A, p0) }
	pub fn _0x30146C25686B7836(p0: Any, p1: Any) -> Any { invoke!(0x30146C25686B7836, p0, p1) }
	pub fn _0x59AE5CA4FFB4E378(p0: Any, p1: Any) -> Any { invoke!(0x59AE5CA4FFB4E378, p0, p1) }
	pub fn _0x748D5E0D2A1A4C61(p0: Any, p1: Any, p2: Any) { invoke_ignore!(0x748D5E0D2A1A4C61, p0, p1, p2) }
	pub fn END_DUEL(ped: Ped, p1: bool, p2: f32) { invoke_ignore!(0xEED08A3A98B847E2, ped, p1, p2) }
	pub fn _0x651F0530083C0E5A(p0: Any, p1: Any) { invoke_ignore!(0x651F0530083C0E5A, p0, p1) }
	pub fn TASK_CARRIABLE(entity: Entity, carryConfig: Hash, carrier: Ped, carriableSlot: i32, flags: i32) { invoke_ignore!(0xF0B4F759F35CC7F5, entity, carryConfig, carrier, carriableSlot, flags) }
	pub fn _0x9EBD34958AB6F824(p0: Any) { invoke_ignore!(0x9EBD34958AB6F824, p0) }
	pub fn GET_IS_CARRIABLE_ENTITY(entity: Entity) -> bool { invoke!(0x0CCFE72B43C9CF96, entity) }
	pub fn _0x10ADFDF07B7DFFBA(p0: Any, p1: Any, p2: Any) -> Any { invoke!(0x10ADFDF07B7DFFBA, p0, p1, p2) }
	pub fn TASK_PLACE_CARRIED_ENTITY_AT_COORD(ped: Ped, entity: Entity, x: f32, y: f32, z: f32, p5: f32, flags: i32) { invoke_ignore!(0xC7F0B43DCDC57E3D, ped, entity, x, y, z, p5, flags) }
	pub fn TASK_PLACE_CARRIED_ENTITY_ON_MOUNT(ped: Ped, entity: Entity, mount: Ped, p3: f32) { invoke_ignore!(0x6D3D87C57B3D52C7, ped, entity, mount, p3) }
	pub fn TASK_DUMP_CARRIABLE_FROM_PARENT(ped: Ped, ped2: Ped, entity: Entity) { invoke_ignore!(0x17CA98707B15926A, ped, ped2, entity) }
	pub fn _DETACH_CARRIABLE_PED(ped: Ped) { invoke_ignore!(0x36D188AECB26094B, ped) }
	pub fn _0xE2CF104ADD49D4BF(p0: Any) { invoke_ignore!(0xE2CF104ADD49D4BF, p0) }
	pub fn TASK_PICKUP_CARRIABLE_ENTITY(ped: Ped, entity: Entity) { invoke_ignore!(0x502EC17B1BED4BFA, ped, entity) }
	pub fn TASK_HOGTIE_TARGET_PED(ped: Ped, targetPed: Ped) { invoke_ignore!(0x27829AFD3E03AC1A, ped, targetPed) }
	pub fn _TASK_CUT_FREE_HOGTIED_TARGET_PED(ped: Ped, targetPed: Ped) { invoke_ignore!(0x81D16C4FF3A77ADF, ped, targetPed) }
	pub fn _TASK_CUT_FREE_HOGTIED_TARGET_PED_2(ped: Ped, targetPed: Ped, p2: f32) { invoke_ignore!(0x525421A507216084, ped, targetPed, p2) }
	pub fn _SET_HOGTIE_ESCAPE_TIMER(ped: Ped, time: f32) { invoke_ignore!(0xAB591AE6B48B913E, ped, time) }
	pub fn _GET_HOGTIE_ESCAPE_TIMER(ped: Ped) -> f32 { invoke!(0x4687E69D258BBE41, ped) }
	pub fn _0x03D741CB4052E26C(p0: Any) -> Any { invoke!(0x03D741CB4052E26C, p0) }
	pub fn _REQUEST_HERB_COMPOSITE_ASSET(asset: Hash) -> bool { invoke!(0x73F0D0327BFA0812, asset) }
	pub fn ARE_COMPOSITE_LOOTABLE_ENTITY_DEF_ASSETS_LOADED(asset: Hash) -> bool { invoke!(0x5E5D96BE25E9DF68, asset) }
	pub fn _CREATE_HERB_COMPOSITES(asset: Hash, x: f32, y: f32, z: f32, heading: f32, groundSetting: i32, p6: &mut Any, p7: i32) -> i32 { invoke!(0x5B4BBE80AD5972DC, asset, x, y, z, heading, groundSetting, p6, p7) }
	pub fn _DELETE_PATCH_OBJECTS_FROM_HERB_COMPOSITES(compositeId: i32, p1: bool) { invoke_ignore!(0x5758B1EE0C3FD4AC, compositeId, p1) }
	pub fn _GET_HERB_COMPOSITE_NUM_ENTITIES(compositeId: i32, outEntities: &mut Any) -> i32 { invoke!(0x96C6ED22FB742C3E, compositeId, outEntities) }
	pub fn _0xDF56A2B50C04DEA4(p0: Any, p1: Any) -> Any { invoke!(0xDF56A2B50C04DEA4, p0, p1) }
	pub fn TASK_LOOT_ENTITY(ped: Ped, entity: Entity) { invoke_ignore!(0x48FAE038401A2888, ped, entity) }
	pub fn TASK_BREAK_VEHICLE_DOOR_LOCK(ped: Ped, vehicle: Vehicle) { invoke_ignore!(0xBB28D1BC9EA8A6A5, ped, vehicle) }
	pub fn TASK_LOOT_NEAREST_ENTITY(ped: Ped, x: f32, y: f32, z: f32, p4: i32, p5: f32) { invoke_ignore!(0xCF1501CBC4059412, ped, x, y, z, p4, p5) }
	pub fn TASK_LASSO_PED(ped: Ped, targetPed: Ped) { invoke_ignore!(0xC716EB2BD16370A3, ped, targetPed) }
	pub fn TASK_HOGTIEABLE(ped: Ped) { invoke_ignore!(0x6AFD8FE0D723328F, ped) }
	pub fn UNHOGTIE_PED(ped: Ped, flags: i32, getupSetHash: Hash, p3: & CStr, p4: & CStr, p5: f32) { invoke_ignore!(0x79559BAD83CCD038, ped, flags, getupSetHash, p3, p4, p5) }
	pub fn _0x722D6A49200174FE(p0: Any, p1: Any, p2: Any, p3: Any, p4: Any) { invoke_ignore!(0x722D6A49200174FE, p0, p1, p2, p3, p4) }
	pub fn _MAKE_OBJECT_CARRIABLE(object: Object) { invoke_ignore!(0x78B4567E18B54480, object) }
	pub fn MAKE_OBJECT_NOT_CARRIABLE(object: Object) { invoke_ignore!(0x67BFCED22909834D, object) }
	pub fn _0x8E1DDE26D270CC5E(p0: Any, p1: Any) { invoke_ignore!(0x8E1DDE26D270CC5E, p0, p1) }
	pub fn _0xA6A76D666A281F2D(p0: Any, item: Hash) { invoke_ignore!(0xA6A76D666A281F2D, p0, item) }
	pub fn _0xA21AA2F0C2180125(p0: Any, p1: Any) { invoke_ignore!(0xA21AA2F0C2180125, p0, p1) }
	pub fn _FIND_MODEL_FOR_ITEM(item: Hash) -> Hash { invoke!(0xE47DD64B9F02677D, item) }
	pub fn _0xFF745B0346E19E2C(p0: Any) { invoke_ignore!(0xFF745B0346E19E2C, p0) }
	pub fn _0xB8F52A3F84A7CC59(p0: Any) -> Any { invoke!(0xB8F52A3F84A7CC59, p0) }
	pub fn _0x6AFDA2264925BD11(p0: Any) { invoke_ignore!(0x6AFDA2264925BD11, p0) }
	pub fn _0x816A3ACD265E2297(p0: Any, p1: Any) { invoke_ignore!(0x816A3ACD265E2297, p0, p1) }
	pub fn _0x4E806A395D43A458(p0: Any) { invoke_ignore!(0x4E806A395D43A458, p0) }
	pub fn SET_TEAM_CARRIABLE_ENTITY(p0: Any, p1: Any, p2: Any) { invoke_ignore!(0x545BF19F86E80F11, p0, p1, p2) }
	pub fn IS_TEAM_CARRIABLE_ENTITY(p0: Any, p1: Any) -> bool { invoke!(0x559A6F8C5133B4EE, p0, p1) }
	pub fn _IS_HAT_BEING_PICKED_UP(hatObject: Object) -> bool { invoke!(0x11CD066F54DA0133, hatObject) }
	pub fn _0x9ADDBB9242179D56(object: Object, ped: Ped) { invoke_ignore!(0x9ADDBB9242179D56, object, ped) }
	pub fn _IS_HAT_BEING_PICKED_UP_2(hatObject: Object) -> bool { invoke!(0x4ECCC2815CA79AE2, hatObject) }
	pub fn _TASK_EQUIP_HAT(hatObject: Object, ped: Ped) { invoke_ignore!(0xAA0AF6025160243A, hatObject, ped) }
	pub fn _0x7CB99FADDE73CD1B(p0: Any) -> Any { invoke!(0x7CB99FADDE73CD1B, p0) }
	pub fn _0xF3C3503276F4A034(entity: Entity, p1: Any) { invoke_ignore!(0xF3C3503276F4A034, entity, p1) }
	pub fn _0x6DAC799857EF3F11(p0: Any, p1: Any) -> Any { invoke!(0x6DAC799857EF3F11, p0, p1) }
	pub fn _0x920684BE432875B1(p0: Any) -> Any { invoke!(0x920684BE432875B1, p0) }
	pub fn SET_ENHANCED_BREAK_FREE(ped: Ped, p1: bool, clipset: & CStr) -> bool { invoke!(0x1BF9D36A5EAFFBAE, ped, p1, clipset) }
	pub fn _0x6AFD84AEAA3EA538(p0: Any) -> Any { invoke!(0x6AFD84AEAA3EA538, p0) }
	pub fn _0xBD1C3C0F271C39D3(p0: Any, p1: Any) { invoke_ignore!(0xBD1C3C0F271C39D3, p0, p1) }
	pub fn _0x1ECF56C040FD839C(p0: Any, p1: Any) { invoke_ignore!(0x1ECF56C040FD839C, p0, p1) }
	pub fn _0xF40A109B4B79A848(p0: Any, p1: Any, p2: Any) { invoke_ignore!(0xF40A109B4B79A848, p0, p1, p2) }
	pub fn _IS_PED_LEADING_HORSE(ped: Ped) -> bool { invoke!(0xEFC4303DDC6E60D3, ped) }
	pub fn _0xAC5045AB7F1A34FD(p0: Any) -> Any { invoke!(0xAC5045AB7F1A34FD, p0) }
	pub fn _GET_LED_HORSE_FROM_PED(ped: Ped) -> Ped { invoke!(0xED1F514AF4732258, ped) }
	pub fn TASK_TURN_TO_FACE_CLOSEST_PED(ped: Ped, p1: f32, p2: f32, p3: i32) { invoke_ignore!(0x84179419DBDD36F2, ped, p1, p2, p3) }
	pub fn TASK_CONFRONT(ped: Ped, targetPed: Ped, p2: i32) -> bool { invoke!(0x3A2A2071DF5CC569, ped, targetPed, p2) }
	pub fn TASK_POLICE(ped: Ped, p1: bool) -> bool { invoke!(0x87BE56724650408E, ped, p1) }
	pub fn TASK_MELEE(ped: Ped, targetPed: Ped, p2: Hash, p3: Any, p4: Any, p5: f32, p6: Any, p7: f32) -> bool { invoke!(0x482C99D0B38D1B0A, ped, targetPed, p2, p3, p4, p5, p6, p7) }
	pub fn TASK_GRAPPLE(ped: Ped, targetPed: Ped, grappleStyle: Hash, p3: i32, p4: f32, p5: i32, p6: i32) -> bool { invoke!(0x779A2FFACEFAEA7B, ped, targetPed, grappleStyle, p3, p4, p5, p6) }
	pub fn _TASK_INTIMIDATED(p0: Any, p1: Any, p2: Any, p3: Any, p4: Any) -> bool { invoke!(0x648B75D44930D6BD, p0, p1, p2, p3, p4) }
	pub fn _TASK_INTIMIDATED_2(victim: Ped, attacker: Ped, p2: i32, p3: bool, p4: bool, everyFrame: bool, p6: bool, p7: bool, flag: i32) -> bool { invoke!(0x933ACC1A1771A288, victim, attacker, p2, p3, p4, everyFrame, p6, p7, flag) }
	pub fn _0x2948235DB2058E99(p0: Any, p1: Any) { invoke_ignore!(0x2948235DB2058E99, p0, p1) }
	pub fn _0xB2F47A1AFDFCC595(p0: Any, p1: Any) { invoke_ignore!(0xB2F47A1AFDFCC595, p0, p1) }
	pub fn _0x41D1331AFAD5A091(ped: Ped, p1: i32, p2: Any) { invoke_ignore!(0x41D1331AFAD5A091, ped, p1, p2) }
	pub fn _0x801BD27403F3CBA0(p0: Any, p1: Any, p2: Any, p3: Any) { invoke_ignore!(0x801BD27403F3CBA0, p0, p1, p2, p3) }
	pub fn _0x0FE797DD9F70DFA6(p0: Any, p1: Any, p2: Any, p3: Any) { invoke_ignore!(0x0FE797DD9F70DFA6, p0, p1, p2, p3) }
	pub fn TASK_PERSISTENT_CHARACTER(ped: Ped) { invoke_ignore!(0x4391700CBD89C3D8, ped) }
	pub fn _0xFC7F71CF49F70B6B(p0: Any) { invoke_ignore!(0xFC7F71CF49F70B6B, p0) }
	pub fn _0xE01F55B2896F6B37(p0: Any, p1: Any) { invoke_ignore!(0xE01F55B2896F6B37, p0, p1) }
	pub fn _0xE62754D09354F6CF(p0: Any) -> Any { invoke!(0xE62754D09354F6CF, p0) }
	pub fn _0x4BA972D0E5AD8122(p0: Any, p1: Any) { invoke_ignore!(0x4BA972D0E5AD8122, p0, p1) }
	pub fn _GET_TASK_FISHING(ped: Ped, p1: &mut Any) -> bool { invoke!(0xF3735ACD11ACD500, ped, p1) }
	pub fn _SET_TASK_FISHING(ped: Ped, p1: &mut Any) -> bool { invoke!(0xF3735ACD11ACD501, ped, p1) }
	pub fn TASK_SWAP_FISHING_BAIT(ped: Ped, bait: & CStr, withoutBuoy: bool) { invoke_ignore!(0x2C28AC30A72722DA, ped, bait, withoutBuoy) }
	pub fn _SET_FISHING_BAIT(ped: Ped, bait: & CStr, withoutBuoy: bool, instantly: bool) { invoke_ignore!(0x9B0C7FA063E67629, ped, bait, withoutBuoy, instantly) }
	pub fn _0x1F298C7BD30D1240(ped: Ped) { invoke_ignore!(0x1F298C7BD30D1240, ped) }
	pub fn _PED_FISHINGROD_HOOK_ENTITY(ped: Ped, entity: Entity) { invoke_ignore!(0x1A52076D26E09004, ped, entity) }
	pub fn _PED_FISHINGROD_HOOK_OBJECT(ped: Ped, object: Object) { invoke_ignore!(0xCE71C2F9BAA3F975, ped, object) }
	pub fn _0xB520DBDA7FCF573F(ped: Ped) -> bool { invoke!(0xB520DBDA7FCF573F, ped) }
	pub fn _0x31BB338F64D5C861(ped: Ped, p1: bool) { invoke_ignore!(0x31BB338F64D5C861, ped, p1) }
	pub fn _0x517D01BF27B682D1(ped: Ped, entity: Entity, p2: f32, p3: f32, p4: f32, p5: f32, p6: i32) { invoke_ignore!(0x517D01BF27B682D1, ped, entity, p2, p3, p4, p5, p6) }
	pub fn _0x88FD60D846D9CD63(ped: Ped) { invoke_ignore!(0x88FD60D846D9CD63, ped) }
	pub fn _0x9050DF2C53801208(ped: Ped, p1: f32) { invoke_ignore!(0x9050DF2C53801208, ped, p1) }
	pub fn _0x22CDBF317C40A122(ped: Ped) { invoke_ignore!(0x22CDBF317C40A122, ped) }
	pub fn _0x5952DFA38FA529FE() -> Any { invoke!(0x5952DFA38FA529FE) }
	pub fn TASK_PLAY_EMOTE_WITH_HASH(ped: Ped, emoteType: i32, playbackMode: i32, emote: Hash, isSecondaryTask: bool, canBreakOut: bool, disableEarlyOutAnimTag: bool, ignoreInvalidMainTask: bool, destroyProps: bool) { invoke_ignore!(0xB31A277C1AC7B7FF, ped, emoteType, playbackMode, emote, isSecondaryTask, canBreakOut, disableEarlyOutAnimTag, ignoreInvalidMainTask, destroyProps) }
	pub fn _TASK_PLAY_EMOTE(ped: Ped, emoteType: i32, playbackMode: i32, emote: Hash, isSecondaryTask: bool, canBreakOut: bool, disableEarlyOutAnimTag: bool, ignoreInvalidMainTask: bool, destroyProps: bool) { invoke_ignore!(0x884E3436CC1F41DD, ped, emoteType, playbackMode, emote, isSecondaryTask, canBreakOut, disableEarlyOutAnimTag, ignoreInvalidMainTask, destroyProps) }
	pub fn _0x6A1AF481407BF6E9(p0: Any) { invoke_ignore!(0x6A1AF481407BF6E9, p0) }
	pub fn _TASK_EMOTE_OUTRO(ped: Ped) { invoke_ignore!(0xBDFEEB7600BCD938, ped) }
	pub fn _0xEC516FE805D2CB2D(p0: Any) { invoke_ignore!(0xEC516FE805D2CB2D, p0) }
	pub fn _0x59AEA4DC640814B9(p0: Any, p1: Any) { invoke_ignore!(0x59AEA4DC640814B9, p0, p1) }
	pub fn _0x11C7CE1AE38911B5(p0: Any) -> Any { invoke!(0x11C7CE1AE38911B5, p0) }
	pub fn _0xD0ABC4EA3B5E21A0(p0: Any, p1: Any) -> Any { invoke!(0xD0ABC4EA3B5E21A0, p0, p1) }
	pub fn IS_EMOTE_TASK_RUNNING(ped: Ped, p1: Any) -> bool { invoke!(0xCF9B71C0AF824036, ped, p1) }
}
pub mod TELEMETRY {
	use std::ffi::CStr;
	use crate::core::scripthook::native::*;
	use crate::core::types::*;

	pub fn _TELEMETRY_SET_IS_FLOW(toggle: bool) { invoke_ignore!(0x9BEE018A63FFFAD9, toggle) }
	pub fn _0xEC0BD8736DCAF841(toggle: bool) { invoke_ignore!(0xEC0BD8736DCAF841, toggle) }
	pub fn _TELEMETRY_MISSION_STARTED(p0: Any, p1: Any, p2: Any, p3: Any) { invoke_ignore!(0x15B0CC1B36F1DE29, p0, p1, p2, p3) }
	pub fn _TELEMETRY_MISSION_OVER(p0: Any, p1: Any) { invoke_ignore!(0xD894437E12C17AEC, p0, p1) }
	pub fn _TELEMETRY_MISSION_CHECKPOINT(p0: Any, p1: Any, p2: Any) { invoke_ignore!(0x8EC7890D446BD9C1, p0, p1, p2) }
	pub fn _TELEMETRY_PLAYER_SPAWNED(ped: Ped) { invoke_ignore!(0x5DA4718DF897EB25, ped) }
	pub fn _TELEMETRY_CREATE_UUID(uuid: &mut Any) -> bool { invoke!(0xE692D336F8A2A97F, uuid) }
	pub fn _TELEMETRY_MATCH_QUEUE(p0: Any, p1: Any, p2: Any, p3: Any, p4: Any, p5: Any, p6: Any) { invoke_ignore!(0x4C08D2B6D8BE17E4, p0, p1, p2, p3, p4, p5, p6) }
	pub fn _TELEMETRY_MATCH_STARTED(p0: &mut Any, p1: &mut Any) { invoke_ignore!(0xF620F47B4F4A78C4, p0, p1) }
	pub fn _TELEMETRY_MATCH_OVER(p0: Any, p1: Any, p2: Any, p3: Any, p4: Any) { invoke_ignore!(0xA2058154357726BB, p0, p1, p2, p3, p4) }
	pub fn _TELEMETRY_MATCH_VOTE(p0: &mut Any, p1: &mut Any) { invoke_ignore!(0xEF3C68F56BAD7B69, p0, p1) }
	pub fn _TELEMETRY_LOBBY_PROGRESSION(p0: Any, p1: Any, p2: Any, p3: Any) { invoke_ignore!(0xECD67E9FA677CCCF, p0, p1, p2, p3) }
	pub fn _TELEMETRY_GAME_PROGRESS(p0: Any, p1: Any) { invoke_ignore!(0x51EC204A6E5B5A1A, p0, p1) }
	pub fn _TELEMETRY_HERB_PICKED(herbType: Hash) { invoke_ignore!(0xAE693EC3A178F6C2, herbType) }
	pub fn _TELEMETRY_ANIMAL_SKINNED(type_: Hash, items: &mut Any) { invoke_ignore!(0x7581972ADF5D699A, type_, items) }
	pub fn _TELEMETRY_CAMP_CREATED(p0: Any) { invoke_ignore!(0x565EAA726B2CE3B7, p0) }
	pub fn _TELEMETRY_CAMP_SUPPLIES(p0: Any, p1: Any, p2: Any, p3: Any, p4: Any) { invoke_ignore!(0x217F47761376E16E, p0, p1, p2, p3, p4) }
	pub fn _TELEMETRY_REGION(regionHash: Hash) { invoke_ignore!(0xCD6F8A0335D821F9, regionHash) }
	pub fn _TELEMETRY_SHOP_ENTRY(shopType: Any, shopRegion: Any, region: Any, p3: Any, p4: Any, p5: Any) { invoke_ignore!(0x775B2ED944E44973, shopType, shopRegion, region, p3, p4, p5) }
	pub fn _TELEMETRY_SHOP_EXIT(p0: Any, p1: Any) { invoke_ignore!(0xF78E669FDC202E73, p0, p1) }
	pub fn _TELEMETRY_SET_SHOP_FOR_TRANSACTION(transactionId: i32, p1: Hash, p2: Hash) { invoke_ignore!(0xCA9E42F437625A85, transactionId, p1, p2) }
	pub fn _TELEMETRY_SHOP_PURCHASE(p0: Any, p1: Any, p2: Any, p3: Any, p4: Any) { invoke_ignore!(0x2A374E6F0075EE81, p0, p1, p2, p3, p4) }
	pub fn _TELEMETRY_SHOP_SELL(p0: Any, p1: Any, p2: Any, p3: Any, centSalePrice: i32) { invoke_ignore!(0x9BD8A9D0A774A6F8, p0, p1, p2, p3, centSalePrice) }
	pub fn _TELEMETRY_GOLD_STORE(p0: Any, p1: Any, p2: Any, p3: Any) { invoke_ignore!(0x536B6025E94AC48F, p0, p1, p2, p3) }
	pub fn _CLEAR_TELEMETRY_SHOP_UI() { invoke_ignore!(0x32D5898C4898CD95) }
	pub fn _TELEMETRY_SHOP_CUTSCENE(p0: Any, p1: Any, p2: Any, p3: Any) { invoke_ignore!(0xB0B19B56697836F5, p0, p1, p2, p3) }
	pub fn _TELEMETRY_AMBIENT_VIGNETTE(p0: Any, p1: Any, p2: Any, p3: Any, p4: Any, p5: Any, p6: Any) { invoke_ignore!(0x3145044F3990D321, p0, p1, p2, p3, p4, p5, p6) }
	pub fn _TELEMETRY_DISCOVERABLE(p0: Any) { invoke_ignore!(0xF5EAD898EF387E73, p0) }
	pub fn _TELEMETRY_HONOR(p0: Any, p1: Any) { invoke_ignore!(0xE6B763C7F4902201, p0, p1) }
	pub fn _TELEMETRY_CRAFT_ITEM(p0: Any, p1: Any, p2: Any, quantity: Any) { invoke_ignore!(0x78C2E029DB205A3A, p0, p1, p2, quantity) }
	pub fn TELEMETRY_CAMP_DONATE(transactionId: Any, p1: Any, p2: Any, p3: Any, p4: Any, slotId: Hash, p6: Hash, p7: Any, p8: bool) { invoke_ignore!(0xDF516E598D966D06, transactionId, p1, p2, p3, p4, slotId, p6, p7, p8) }
	pub fn _TELEMETRY_MOONSHINE_BREW(p0: Any, p1: Any, p2: Any, p3: Any, p4: Any, p5: Any, p6: Any, p7: Any, p8: Any, p9: Any, p10: Any) { invoke_ignore!(0xB5013EFBB5516867, p0, p1, p2, p3, p4, p5, p6, p7, p8, p9, p10) }
	pub fn _TELEMETRY_COLLECT(transactionId: Any, p1: Any, p2: Any, p3: Any, p4: Any, p5: Any, p6: Any) { invoke_ignore!(0xD6CB05DDAEE43AFD, transactionId, p1, p2, p3, p4, p5, p6) }
	pub fn _TELEMETRY_MISSION_ILO_OPTION(p0: Any, p1: Any) { invoke_ignore!(0xEA323F5E1A4DA2F1, p0, p1) }
	pub fn _TELEMETRY_MISSION_FAILED_TO_LAUNCH(p0: Any, p1: Any, x: f32, y: f32, z: f32, reason: i32) { invoke_ignore!(0x6571E4327390EC0B, p0, p1, x, y, z, reason) }
	pub fn _TELEMETRY_GANG_SHARES(p0: Any, p1: Any, p2: Any, p3: Any) { invoke_ignore!(0xE6DC9B21AC7A8729, p0, p1, p2, p3) }
	pub fn _TELEMETRY_FAST_TRAVEL(p0: Any, p1: Any, p2: Any, p3: Any, p4: Any) { invoke_ignore!(0x7CEF4AC79F7E7FAD, p0, p1, p2, p3, p4) }
	pub fn _TELEMETRY_NET_CAMP(p0: Any, p1: Any, p2: Any, p3: Any, p4: Any, p5: Any, p6: Any) { invoke_ignore!(0xA72773C3134F9A57, p0, p1, p2, p3, p4, p5, p6) }
	pub fn _TELEMETRY_RPG_GLOBAL_CALCULATE_ATTRIBUTE_CORE_DELTA() { invoke_ignore!(0x7E002A36AEFCFB55) }
	pub fn _TELEMETRY_SLEEP(p0: Any) { invoke_ignore!(0xF9F14080D80937BD, p0) }
	pub fn _TELEMETRY_PARLEY_FEUD(p0: Any, p1: Any, p2: Any, p3: Any, p4: Any) { invoke_ignore!(0xF37A2149BC9A8A27, p0, p1, p2, p3, p4) }
	pub fn TELEMETRY_PLAYER_MENU_PIN(p0: Any, p1: Any, p2: Any, p3: Any) { invoke_ignore!(0x076C5843371EB889, p0, p1, p2, p3) }
	pub fn _TELEMETRY_NOTORIETY(p0: Any, p1: Any, p2: Any, p3: Any) { invoke_ignore!(0xE26970A7AE0F28E9, p0, p1, p2, p3) }
	pub fn _TELEMETRY_DEFENSIVE(p0: Any, p1: Any, p2: Any) { invoke_ignore!(0xE57529D23541D2DD, p0, p1, p2) }
	pub fn _TELEMETRY_LOOT(p0: Any, p1: Any, p2: Any, p3: Any) { invoke_ignore!(0xCF63EF77B0DF0397, p0, p1, p2, p3) }
	pub fn _TELEMETRY_EMOTE_ADD_CATEGORY_TO_SAVE(p0: Any, p1: Any, emote: Hash) { invoke_ignore!(0x2C24AF8EEEEF8A55, p0, p1, emote) }
	pub fn _TELEMETRY_FAVOR_EMOTE(p0: Any, p1: Any, p2: Any) { invoke_ignore!(0x16B23D4F7A1F50D9, p0, p1, p2) }
	pub fn _TELEMETRY_POKER_OVER(p0: Any, p1: Any, p2: Any, p3: Any, p4: Any, p5: Any, p6: Any, p7: Any, p8: Any, p9: Any) { invoke_ignore!(0x8127C5AA05C5A210, p0, p1, p2, p3, p4, p5, p6, p7, p8, p9) }
	pub fn _TELEMETRY_TRIGGER_TRANSACTION_REQUEST(requestId: &mut Any, transactionId: &mut Any) -> bool { invoke!(0x80A02D9F948A8BCA, requestId, transactionId) }
	pub fn _TRY_GET_TELEMETRY_ID_FROM_TRANSACTION_ID(transactionId: &mut Any, requestId: &mut Any) -> bool { invoke!(0xF184B3ECE36219CF, transactionId, requestId) }
	pub fn _TELEMETRY_ROLE_BOUNTY(p0: Any) { invoke_ignore!(0xAB43D1C80B5E9500, p0) }
	pub fn _TELEMETRY_BOUNTY_TARGET(data: &mut Any) { invoke_ignore!(0x52FA31DB8F3AD25D, data) }
	pub fn _TELEMETRY_PRISON(transactionId: Any, bountyAmount: Any, ped: Ped, completionType: Any, jailTimeServed: Any, jailTimeLeft: Any, posseRole: Any) { invoke_ignore!(0xB204BF9F30298D77, transactionId, bountyAmount, ped, completionType, jailTimeServed, jailTimeLeft, posseRole) }
	pub fn _TELEMETRY_ROLE_TRADER(p0: Any, transactionId: Any) { invoke_ignore!(0x476038B5A0734C10, p0, transactionId) }
	pub fn _TELEMETRY_ROLE_MOONSHINER(p0: Any, transactionId: Any) { invoke_ignore!(0x99D40C5D74BC88E9, p0, transactionId) }
	pub fn _TELEMETRY_ROLE_COLLECTOR(transactionId: Any, collectible: Any, category: Any, p3: Any, p4: Any, p5: Any, p6: Any) { invoke_ignore!(0x4AC38DFD286DAD14, transactionId, collectible, category, p3, p4, p5, p6) }
	pub fn _TELEMETRY_PHOTO(p0: Any, p1: Any, p2: Any, p3: Any) { invoke_ignore!(0xED22BE4C5A399E63, p0, p1, p2, p3) }
	pub fn _TELEMETRY_START_GUN_LOCKER_INTERACTION() { invoke_ignore!(0xF0D54E0651DD7E07) }
	pub fn _TELEMETRY_GUN_LOCKER() { invoke_ignore!(0x415FE28ED44BFF14) }
	pub fn _TELEMETRY_GUN_LOCKER_WEAPON_STORED(p0: Hash) { invoke_ignore!(0xC3ADF4880784FA9C, p0) }
	pub fn _TELEMETRY_GUN_LOCKER_WEAPON_REMOVED(p0: Hash) { invoke_ignore!(0x317D9C9560529CC2, p0) }
	pub fn _TELEMETRY_ROLE_TOKEN_TRANSACTION(p0: Any, p1: Any, p2: Any, p3: Any, p4: Any, p5: Any) { invoke_ignore!(0x32C2939564D74BFF, p0, p1, p2, p3, p4, p5) }
	pub fn _TELEMETRY_COUPON(p0: Any, p1: Any, p2: Any, p3: Any, p4: Any, p5: Any) { invoke_ignore!(0x621D719C4836292B, p0, p1, p2, p3, p4, p5) }
	pub fn TELEMETRY_PERSONAL_VEHICLE_MOUNT(p0: Any, p1: Any, p2: Any, p3: Any) { invoke_ignore!(0xFF9052BC7A3B7D33, p0, p1, p2, p3) }
	pub fn _TELEMETRY_PERSONAL_VEHICLE_WAGON(p0: Any, p1: Any, p2: Any) { invoke_ignore!(0xE67AF24C5A3B6058, p0, p1, p2) }
	pub fn _TELEMETRY_MENU_NAVIGATION(p0: Any, p1: Any, p2: Any, p3: Any) { invoke_ignore!(0x3255D4D2082C6339, p0, p1, p2, p3) }
	pub fn _TELEMETRY_HUB_NAVIGATION(p0: Any, p1: Any, p2: Any, p3: Any) { invoke_ignore!(0x25CC50EC1A6F3A96, p0, p1, p2, p3) }
	pub fn _TELEMETRY_HUB_OFFERS(couponItem: Any, p1: Any) { invoke_ignore!(0x37AA282163B0D2C4, couponItem, p1) }
	pub fn _TELEMETRY_SAMPLE(transactionId: Any, animal: Any, p2: Any, bSampled: Any, bTranq: bool) { invoke_ignore!(0x61559675D23D8BD1, transactionId, animal, p2, bSampled, bTranq) }
	pub fn _TELEMETRY_ROLE_NATURALIST(transactionId: Any, p1: Any, p2: Any, p3: Any, p4: Any, p5: Any, p6: Any, p7: Any, p8: Any, p9: Any) { invoke_ignore!(0x6FB9EA308F302922, transactionId, p1, p2, p3, p4, p5, p6, p7, p8, p9) }
	pub fn _TELEMETRY_PHOTO_CAM(p0: Any, p1: Any, p2: Any, p3: Any, p4: Any, p5: Any, p6: Any, p7: Any, p8: Any) { invoke_ignore!(0x0777D65EE8A17517, p0, p1, p2, p3, p4, p5, p6, p7, p8) }
	pub fn _TELEMETRY_INTRO_SKIP(p0: Any, p1: Any, p2: Any) { invoke_ignore!(0x1B554723799245F4, p0, p1, p2) }
	pub fn ANALYTICS_PLAYTIME_FREEMODE_START() { invoke_ignore!(0xE9F24081D84931B8) }
	pub fn ANALYTICS_PLAYTIME_FREEMODE_END() { invoke_ignore!(0x3180E991D4B8F248) }
	pub fn _TELEMETRY_CUSTOM(args: &mut Any) { invoke_ignore!(0x40914CCF2A1AB531, args) }
	pub fn _TELEMETRY_MATCH_NOMINATION(args: &mut Any) { invoke_ignore!(0x330029E121380CEB, args) }
	pub fn _TELEMETRY_CHAR_CREATOR(p0: Any, p1: Any, p2: Any, p3: Any, p4: Any, p5: Any, p6: Any) { invoke_ignore!(0x7207AD471BC9278C, p0, p1, p2, p3, p4, p5, p6) }
	pub fn _0x6F5BC5C4EAB42B15(linkID: i32, type_: i32, contentId: & CStr) { invoke_ignore!(0x6F5BC5C4EAB42B15, linkID, type_, contentId) }
}
pub mod TXD {
	use std::ffi::CStr;
	use crate::core::scripthook::native::*;
	use crate::core::types::*;

	pub fn DOES_STREAMED_TXD_EXIST(txdHash: Hash) -> bool { invoke!(0xBA0163B277C2D2D0, txdHash) }
	pub fn REQUEST_STREAMED_TXD(txdHash: Hash, p1: bool) { invoke_ignore!(0xDB1BD07FB464584D, txdHash, p1) }
	pub fn HAS_STREAMED_TXD_LOADED(txdHash: Hash) -> bool { invoke!(0xBE72591D1509FFE4, txdHash) }
	pub fn SET_STREAMED_TXD_AS_NO_LONGER_NEEDED(txdHash: Hash) { invoke_ignore!(0x8232F37DF762ACB2, txdHash) }
	pub fn DOES_STREAMED_TEXTURE_DICT_EXIST(textureDict: & CStr) -> bool { invoke!(0x7332461FC59EB7EC, textureDict) }
	pub fn REQUEST_STREAMED_TEXTURE_DICT(textureDict: & CStr, p1: bool) { invoke_ignore!(0xC1BA29DF5631B0F8, textureDict, p1) }
	pub fn HAS_STREAMED_TEXTURE_DICT_LOADED(textureDict: & CStr) -> bool { invoke!(0x54D6900929CCF162, textureDict) }
	pub fn SET_STREAMED_TEXTURE_DICT_AS_NO_LONGER_NEEDED(textureDict: & CStr) { invoke_ignore!(0x4ACA10A91F66F1E2, textureDict) }
}
pub mod UIAPPS {
	use std::ffi::CStr;
	use crate::core::scripthook::native::*;
	use crate::core::types::*;

	pub fn IS_UIAPP_ACTIVE_BY_HASH(appNameHash: Hash) -> bool { invoke!(0x25B7A0206BDFAC76, appNameHash) }
	pub fn IS_ANY_UIAPP_ACTIVE() -> bool { invoke!(0xAC959AB99AAF3D9F) }
	pub fn IS_UIAPP_RUNNING_BY_HASH(appNameHash: Hash) -> bool { invoke!(0x4E511D093A86AD49, appNameHash) }
	pub fn IS_UIAPP_RUNNING(appName: & CStr) -> bool { invoke!(0xDE4A9B35D028979F, appName) }
	pub fn IS_ANY_UIAPP_RUNNING() -> bool { invoke!(0xDB30BEC7A7A5CBD3) }
	pub fn _GET_UIAPP_CURRENT_ACTIVITY_BY_HASH(appNameHash: Hash) -> Hash { invoke!(0x96FD694FE5BE55DC, appNameHash) }
	pub fn _CLOSE_UIAPP_BY_HASH(appNameHash: Hash) { invoke_ignore!(0x2FF10C9C3F92277E, appNameHash) }
	pub fn _CLOSE_UIAPP_BY_HASH_IMMEDIATE(appNameHash: Hash) { invoke_ignore!(0x04428420A248A354, appNameHash) }
	pub fn _CLOSE_UIAPP(appName: & CStr) { invoke_ignore!(0x818C6CA9B659E8EC, appName) }
	pub fn _CLOSE_UIAPP_IMMEDIATE(appName: & CStr) { invoke_ignore!(0x3015635426D1B17C, appName) }
	pub fn _CLOSE_ALL_UIAPPS() { invoke_ignore!(0xAD7B70F7230C5A12) }
	pub fn _CLOSE_ALL_UIAPPS_IMMEDIATE() { invoke_ignore!(0x12769EEB8DBD7A7B) }
	pub fn LAUNCH_UIAPP_BY_HASH_WITH_ENTRY(appNameHash: Hash, entryHash: Hash) -> i32 { invoke!(0xC1BCF31E975B3195, appNameHash, entryHash) }
	pub fn LAUNCH_UIAPP_BY_HASH(appNameHash: Hash) -> i32 { invoke!(0xC8FC7F4E4CF4F581, appNameHash) }
	pub fn LAUNCH_UIAPP_WITH_ENTRY(appName: & CStr, entry: & CStr) -> i32 { invoke!(0x7B2027BAC5C8EC89, appName, entry) }
	pub fn CAN_LAUNCH_UIAPP_BY_HASH_WITH_ENTRY(appNameHash: Hash, entryHash: Hash) -> bool { invoke!(0x16F47D434B6086BF, appNameHash, entryHash) }
	pub fn CAN_LAUNCH_UIAPP_BY_HASH(appNameHash: Hash) -> bool { invoke!(0xE555EC27D65EDE80, appNameHash) }
	pub fn REQUEST_UIAPP_TRANSITION_BY_HASH(appNameHash: Hash, transitionHash: Hash) -> bool { invoke!(0x7689CD255655BFD7, appNameHash, transitionHash) }
	pub fn IS_UIAPP_TRANSITIONING_BY_HASH(appNameHash: Hash) -> bool { invoke!(0x42095B886D30DE66, appNameHash) }
}
pub mod UIDEBUG {
	use std::ffi::CStr;
	use crate::core::scripthook::native::*;
	use crate::core::types::*;

	pub fn _BG_DISPLAY_TEXT(text: & CStr, x: f32, y: f32) { invoke_ignore!(0x16794E044C9EFB58, text, x, y) }
	pub fn _BG_SET_TEXT_SCALE(scaleX: f32, scaleY: f32) { invoke_ignore!(0xA1253A3C870B6843, scaleX, scaleY) }
	pub fn _BG_SET_TEXT_COLOR(red: i32, green: i32, blue: i32, alpha: i32) { invoke_ignore!(0x16FA5CE47F184F1E, red, green, blue, alpha) }
}
pub mod UIEVENTS {
	use std::ffi::CStr;
	use crate::core::scripthook::native::*;
	use crate::core::types::*;

	pub fn EVENTS_UI_IS_PENDING(hash: Hash) -> bool { invoke!(0x67ED5A7963F2F722, hash) }
	pub fn EVENTS_UI_GET_MESSAGE(hash: Hash, eventData: &mut Any) -> bool { invoke!(0xE24E957294241444, hash, eventData) }
	pub fn EVENTS_UI_PEEK_MESSAGE(hash: Hash, eventData: &mut Any) -> bool { invoke!(0x90237103F27F7937, hash, eventData) }
	pub fn EVENTS_UI_POP_MESSAGE(hash: Hash) { invoke_ignore!(0x8E8A2369F48EC839, hash) }
}
pub mod UIFEED {
	use std::ffi::CStr;
	use crate::core::scripthook::native::*;
	use crate::core::types::*;

	pub fn _UI_FEED_POST_HELP_TEXT(p0: &mut Any, p1: &mut Any, p2: bool) -> i32 { invoke!(0x049D5C615BD38BAD, p0, p1, p2) }
	pub fn _UI_FEED_POST_LOCATION_SHARD(duration: &mut Any, data: &mut Any, p2: bool, p3: bool) -> i32 { invoke!(0xD05590C1AB38F068, duration, data, p2, p3) }
	pub fn _UI_FEED_POST_OBJECTIVE(p0: &mut Any, p1: &mut Any, p2: bool) -> i32 { invoke!(0xCEDBF17EFCC0E4A4, p0, p1, p2) }
	pub fn _UI_FEED_POST_FEED_TICKER(p0: &mut Any, p1: &mut Any, p2: bool) -> i32 { invoke!(0xB2920B9760F0F36B, p0, p1, p2) }
	pub fn _UI_FEED_POST_SAMPLE_TOAST(p0: &mut Any, p1: &mut Any, p2: bool, p3: bool) -> i32 { invoke!(0x26E87218390E6729, p0, p1, p2, p3) }
	pub fn _0xAFF5BE9BA496CE40(p0: &mut Any, p1: &mut Any, p2: bool, p3: bool, collectableCategory: Hash) -> i32 { invoke!(0xAFF5BE9BA496CE40, p0, p1, p2, p3, collectableCategory) }
	pub fn _UI_FEED_POST_SAMPLE_TOAST_WITH_APP_LINK(p0: &mut Any, p1: &mut Any, p2: bool, p3: bool, p4: bool) -> i32 { invoke!(0x38838A646FB30AAE, p0, p1, p2, p3, p4) }
	pub fn _UI_FEED_POST_SAMPLE_NOTIFICATION(p0: &mut Any, p1: &mut Any, p2: i32, p3: i32) -> i32 { invoke!(0xC927890AA64E9661, p0, p1, p2, p3) }
	pub fn _UI_FEED_POST_RANKUP_TOAST(p0: &mut Any, p1: &mut Any, p2: i32, p3: i32) -> i32 { invoke!(0x3F9FDDBA79117C69, p0, p1, p2, p3) }
	pub fn _0x18D6869FBFFEC0F8(p0: &mut Any, p1: &mut Any, p2: bool, p3: bool) -> i32 { invoke!(0x18D6869FBFFEC0F8, p0, p1, p2, p3) }
	pub fn _UI_FEED_POST_SAMPLE_TOAST_RIGHT(p0: &mut Any, p1: &mut Any, p2: bool) -> i32 { invoke!(0xB249EBCB30DD88E0, p0, p1, p2) }
	pub fn _UI_FEED_POST_MISSION_NAME(p0: &mut Any, p1: &mut Any, p2: bool) -> i32 { invoke!(0x2024F4F333095FB1, p0, p1, p2) }
	pub fn _UI_FEED_POST_RETICLE_MESSAGE(p0: &mut Any, p1: &mut Any, p2: bool) -> i32 { invoke!(0x893128CDB4B81FBB, p0, p1, p2) }
	pub fn _UI_FEED_POST_ONE_TEXT_SHARD(p0: &mut Any, p1: &mut Any, p2: bool) -> i32 { invoke!(0x860DDFE97CC94DF0, p0, p1, p2) }
	pub fn _UI_FEED_POST_TWO_TEXT_SHARD(p0: &mut Any, p1: &mut Any, p2: bool, p3: bool) -> i32 { invoke!(0xA6F4216AB10EB08E, p0, p1, p2, p3) }
	pub fn _UI_FEED_POST_THREE_TEXT_SHARD(p0: &mut Any, p1: &mut Any, p2: bool, p3: bool, p4: bool) -> i32 { invoke!(0x02BCC0FE9EBA3529, p0, p1, p2, p3, p4) }
	pub fn _UI_FEED_POST_GAME_UPDATE_SHARD(p0: &mut Any, p1: &mut Any, p2: bool) -> i32 { invoke!(0x8D1249BD28791878, p0, p1, p2) }
	pub fn _UI_FEED_POST_VOICE_CHAT_FEED(p0: &mut Any, p1: &mut Any, p2: bool) -> i32 { invoke!(0xC48152BC6B3E821C, p0, p1, p2) }
	pub fn UI_FEED_CLEAR_CHANNEL(feedChannel: i32, p1: bool, p2: bool) { invoke_ignore!(0xDD1232B332CBB9E7, feedChannel, p1, p2) }
	pub fn _UI_FEED_CLEAR_ALL_CHANNELS() { invoke_ignore!(0x6035E8FBCA32AC5E) }
	pub fn _UI_FEED_CLEAR_HELP_TEXT_FEED(feedMessage: i32, p1: bool) { invoke_ignore!(0x2F901291EF177B02, feedMessage, p1) }
	pub fn _0x6D85126F6CCF02C9(feedChannel: i32, p1: i32, p2: bool) { invoke_ignore!(0x6D85126F6CCF02C9, feedChannel, p1, p2) }
	pub fn _0x4E88A65968A55C78(p0: &mut Any, p1: bool) -> i32 { invoke!(0x4E88A65968A55C78, p0, p1) }
	pub fn _0x0FD07141AD048AAE(p0: &mut Any, p1: bool) -> i32 { invoke!(0x0FD07141AD048AAE, p0, p1) }
	pub fn _UI_FEED_GET_MESSAGE_STATE(feedMessage: i32) -> i32 { invoke!(0x59FA676177DBE4C9, feedMessage) }
	pub fn UI_FEED_GET_CURRENT_MESSAGE(feedChannel: i32) -> i32 { invoke!(0xC17F69E1418CD11F, feedChannel) }
	pub fn _0xB7223B91CD6B7E07(feedChannel: i32) -> bool { invoke!(0xB7223B91CD6B7E07, feedChannel) }
}
pub mod UILOG {
	use std::ffi::CStr;
	use crate::core::scripthook::native::*;
	use crate::core::types::*;

	pub fn _UILOG_IS_ENTRY_REGISTERED(p0: i32, p1: Hash) -> bool { invoke!(0xB8188CCF52202475, p0, p1) }
	pub fn _UILOG_ADD_ENTRY_HASH(p0: i32, p1: i32, x: f32, y: f32, z: f32, p5: Hash, p6: Hash, p7: Any) { invoke_ignore!(0x69D5479982355D8F, p0, p1, x, y, z, p5, p6, p7) }
	pub fn _UILOG_ADD_ITEM_TO_TASK_LIST(p0: Any, p1: Any, p2: Any, p3: Any, p4: Any, p5: Any, p6: Any, p7: Any) { invoke_ignore!(0x49C63FDF69744A27, p0, p1, p2, p3, p4, p5, p6, p7) }
	pub fn _UILOG_SET_ENTRY_ICON_TEXTURE(p0: i32, p1: Hash, icon: Hash, iconDictionary: Hash) { invoke_ignore!(0x6965469934958D8F, p0, p1, icon, iconDictionary) }
	pub fn _UILOG_SET_ENTRY_BRIEF_TEXTURE(p0: i32, p1: Hash, texture: Hash, textureDictionary: Hash) { invoke_ignore!(0x69684D9936958D8F, p0, p1, texture, textureDictionary) }
	pub fn _UILOG_UPDATE_ENTRY_SUBHEADER(p0: i32, p1: Hash, p2: & CStr) { invoke_ignore!(0x80D6524190258C3E, p0, p1, p2) }
	pub fn _UILOG_SET_ENTRY_PINNED(p0: i32, p1: Hash, p2: bool) { invoke_ignore!(0x72A5CD214B342568, p0, p1, p2) }
	pub fn _UILOG_MARK_MISSION_COMPLETED(p0: Hash) { invoke_ignore!(0xDE31D66D1E54C471, p0) }
	pub fn _UILOG_MARK_ENTRY_AVAILABILITY(p0: i32, p1: Hash, p2: i32, p3: & CStr) { invoke_ignore!(0x13E8D7DD08543482, p0, p1, p2, p3) }
	pub fn _UILOG_MARK_ALL_ENTRIES_AVAILABILITY(p0: i32, p1: & CStr) { invoke_ignore!(0x3920574CF0A2B7B6, p0, p1) }
	pub fn _UILOG_REMOVE_ENTRY(p0: i32, p1: Hash) { invoke_ignore!(0xD594A19BE09A75C6, p0, p1) }
	pub fn _UILOG_SET_DISPLAY_COMPLETION_RATING(logEntryType: i32, p1: Hash, p2: bool) { invoke_ignore!(0xA31013798FADCADC, logEntryType, p1, p2) }
	pub fn _0xA49D6D503E3EA847(p0: Any, p1: Any, p2: Any, p3: Any, p4: Any) { invoke_ignore!(0xA49D6D503E3EA847, p0, p1, p2, p3, p4) }
	pub fn _0x763637F9B838B0A7(p0: i32, p1: Hash, p2: & CStr) { invoke_ignore!(0x763637F9B838B0A7, p0, p1, p2) }
	pub fn _UILOG_CLEAR_ALL_ENTRIES() { invoke_ignore!(0xB95B4EA6B1EDF035) }
	pub fn _UILOG_ADD_OR_UPDATE_OBJECTIVE(p0: i32, p1: Hash, p2: Hash, p3: & CStr, p4: bool, p5: bool, p6: bool) { invoke_ignore!(0xB43163388484CC87, p0, p1, p2, p3, p4, p5, p6) }
	pub fn _0xA20398536B7F1134(p0: Any, p1: Any, p2: Any, p3: Any, p4: Any, p5: Any) { invoke_ignore!(0xA20398536B7F1134, p0, p1, p2, p3, p4, p5) }
	pub fn _UILOG_SET_CACHED_OBJECTIVE(p0: & CStr) { invoke_ignore!(0xFA233F8FE190514C, p0) }
	pub fn _UILOG_CLEAR_CACHED_OBJECTIVE() { invoke_ignore!(0xDFF0D417277B41F8) }
	pub fn _UILOG_HAS_DISPLAYED_CACHED_OBJECTIVE() -> bool { invoke!(0xCC48FFBB45B54F71) }
	pub fn _UILOG_CLEAR_HAS_DISPLAYED_CACHED_OBJECTIVE() { invoke_ignore!(0xA3108D6981A5CADB) }
	pub fn _UILOG_SET_HAS_DISPLAYED_CACHED_OBJECTIVE() { invoke_ignore!(0xA3108D6981A5CADC) }
	pub fn _UILOG_GET_CACHED_OBJECTIVE() -> *const char { invoke!(0x15A4461BEB788096) }
	pub fn _UILOG_PRINT_CACHED_OBJECTIVE() { invoke_ignore!(0xE9990552DEC71600) }
	pub fn _UILOG_SET_PENDING_DETAILS_ID(p0: i32, p1: Hash) -> Any { invoke!(0x136A027CF37B0A4F, p0, p1) }
	pub fn _0x2A4765812202E671() -> Any { invoke!(0x2A4765812202E671) }
	pub fn _UILOG_ADD_TOTAL_TAKE_ENTRY(p0: Hash, p1: Hash, p2: & CStr, p3: & CStr, p4: Hash) { invoke_ignore!(0x60C59968E8E87E6B, p0, p1, p2, p3, p4) }
	pub fn _UILOG_SET_TOTAL_TAKE_SUMMARY(p0: & CStr, p1: & CStr) { invoke_ignore!(0xD106B211EF1B8F04, p0, p1) }
	pub fn _UILOG_POST_NOTIFICATION(data: &mut Any) -> i32 { invoke!(0x49E58FE6EF40B987, data) }
	pub fn _0xDA0A30153FCC0FFD() { invoke_ignore!(0xDA0A30153FCC0FFD) }
}
pub mod UIPINNING {
	use std::ffi::CStr;
	use crate::core::scripthook::native::*;
	use crate::core::types::*;

	pub fn _UIPINNING_GET_TOOLTIP_TEXT(hash: Hash) -> *const char { invoke!(0x3138582E0A13BFAB, hash) }
}
pub mod UITUTORIAL {
	use std::ffi::CStr;
	use crate::core::scripthook::native::*;
	use crate::core::types::*;

	pub fn _UITUTORIAL_GET_IS_THREAT_INDICATOR_CAPABLE_RADAR_SHOWN() -> bool { invoke!(0x2CC24A2A7A1489C4) }
	pub fn _UITUTORIAL_GET_IS_THREAT_INDICATOR_ON() -> bool { invoke!(0xFC2E0A5E9ED4E1B4) }
	pub fn _UITUTORIAL_SET_RPG_ICON_VISIBILITY(rpgIcon: i32, visibility: i32) { invoke_ignore!(0xC116E6DF68DCE667, rpgIcon, visibility) }
}
pub mod UISTATEMACHINE {
	use std::ffi::CStr;
	use crate::core::scripthook::native::*;
	use crate::core::types::*;

	pub fn _UIFLOWBLOCK_REQUEST(p0: Any) -> Any { invoke!(0xC0081B34E395CE48, p0) }
	pub fn _UIFLOWBLOCK_RELEASE(p0: Any) { invoke_ignore!(0xF320A77DD5F781DF, p0) }
	pub fn _UIFLOWBLOCK_IS_LOADED(p0: Any) -> Any { invoke!(0x10A93C057B6BD944, p0) }
	pub fn _UIFLOWBLOCK_ENTER(p0: Any, p1: Any) -> Any { invoke!(0x3B7519720C9DCB45, p0, p1) }
	pub fn UI_STATE_MACHINE_EXISTS(p0: Any) -> Any { invoke!(0x5D15569C0FEBF757, p0) }
	pub fn UI_STATE_MACHINE_CREATE(p0: Any, p1: Any) -> Any { invoke!(0x4C6F2C4B7A03A266, p0, p1) }
	pub fn UI_STATE_MACHINE_CAN_REQUEST_TRANSITION(p0: Any) -> Any { invoke!(0xF7C180F57F85D0B8, p0) }
	pub fn UI_STATE_MACHINE_REQUEST_TRANSITION(p0: Any, p1: Any) -> Any { invoke!(0x7EA9C3547E80350E, p0, p1) }
	pub fn UI_STATE_MACHINE_REQUEST_EXIT(p0: Any, p1: Any) { invoke_ignore!(0x6B9FE4F0BA521A19, p0, p1) }
	pub fn _UI_STATE_MACHINE_IS_EXITED(p0: Hash) -> bool { invoke!(0x11E73195E735B25B, p0) }
	pub fn UI_STATE_MACHINE_DESTROY(p0: Any) { invoke_ignore!(0x4EB122210A90E2D8, p0) }
	pub fn UI_STATE_MACHINE_DESTROY_AND_CLEAR(p0: &mut Any) { invoke_ignore!(0x2738D68D2B4E09E7, p0) }
}
pub mod UISTICKYFEED {
	use std::ffi::CStr;
	use crate::core::scripthook::native::*;
	use crate::core::types::*;

	pub fn _UI_STICKY_FEED_CREATE_ERROR_MESSAGE(p0: &mut Any, p1: &mut Any, p2: bool) -> i32 { invoke!(0x9F2CC2439A04E7BA, p0, p1, p2) }
	pub fn _UI_STICKY_FEED_CREATE_DEATH_FAIL_MESSAGE(p0: &mut Any, p1: &mut Any, p2: bool) -> i32 { invoke!(0x815C4065AE6E6071, p0, p1, p2) }
	pub fn _UI_STICKY_FEED_CREATE_WARNING_MESSAGE(p0: &mut Any, p1: &mut Any, p2: bool) -> i32 { invoke!(0x339E16B41780FC35, p0, p1, p2) }
	pub fn _UI_STICKY_FEED_UPDATE_MESSAGE(msgId: i32, p1: &mut Any, p2: bool) { invoke_ignore!(0xBC6F454E310124DA, msgId, p1, p2) }
	pub fn _UI_STICKY_FEED_CLEAR_MESSAGE(msgId: i32) { invoke_ignore!(0x00A15B94CBA4F76F, msgId) }
	pub fn _UI_STICKY_FEED_IS_CHANNEL_ACTIVE(stickyFeedChannel: i32) -> bool { invoke!(0xC5C395C60B542A3C, stickyFeedChannel) }
	pub fn _UI_STICKY_FEED_IS_ALERT_SCREEN_ACTIVE() -> bool { invoke!(0xF8806EC3FF840FDC) }
	pub fn _UI_STICKY_FEED_GET_MESSAGE_STATE(msgId: i32) -> i32 { invoke!(0x07954320D77F6A3D, msgId) }
}
pub mod UNLOCK {
	use std::ffi::CStr;
	use crate::core::scripthook::native::*;
	use crate::core::types::*;

	pub fn _UNLOCK_IS_UNLOCK_FLAG_SET(unlockHash: Hash, flag: i32) -> bool { invoke!(0x6B6369647F26F09F, unlockHash, flag) }
	pub fn UNLOCK_IS_UNLOCKED(unlockHash: Hash) -> bool { invoke!(0xC4B660C7B6040E75, unlockHash) }
	pub fn UNLOCK_SET_UNLOCKED(unlockHash: Hash, toggle: bool) { invoke_ignore!(0x1B7C5ADA8A6910A0, unlockHash, toggle) }
	pub fn UNLOCK_IS_VISIBLE(unlockHash: Hash) -> bool { invoke!(0x8588A14B75AF096B, unlockHash) }
	pub fn UNLOCK_SET_VISIBLE(unlockHash: Hash, toggle: bool) { invoke_ignore!(0x46B901A8ECDB5A61, unlockHash, toggle) }
	pub fn _UNLOCK_IS_NEW(unlockHash: Hash) -> bool { invoke!(0x644166BA7AA49DEA, unlockHash) }
	pub fn _UNLOCK_SET_NEW(unlockHash: Hash, toggle: bool) { invoke_ignore!(0xA6D79C7AEF870A99, unlockHash, toggle) }
	pub fn _UNLOCK_IS_LOOTABLE(unlockHash: Hash) -> bool { invoke!(0x66BF197E066050DE, unlockHash) }
	pub fn _UNLOCK_GET_ITEM_ROLE_UNLOCK_INFO(unlockHash: Hash, outData: &mut Any) { invoke_ignore!(0x7C1C2062CFAD06FE, unlockHash, outData) }
}
pub mod VEHICLE {
	use std::ffi::CStr;
	use crate::core::scripthook::native::*;
	use crate::core::types::*;

	pub fn _0x6355602C02EDC6DF(entity: Entity, p1: Any) { invoke_ignore!(0x6355602C02EDC6DF, entity, p1) }
	pub fn _SET_VEHICLE_IS_IN_HURRY(vehicle: Vehicle, enabled: bool) { invoke_ignore!(0xCE1531927AD6C9F8, vehicle, enabled) }
	pub fn CREATE_VEHICLE(modelHash: Hash, x: f32, y: f32, z: f32, heading: f32, isNetwork: bool, bScriptHostVeh: bool, bDontAutoCreateDraftAnimals: bool, p8: bool) -> Vehicle { invoke!(0xAF35D0D2583051B0, modelHash, x, y, z, heading, isNetwork, bScriptHostVeh, bDontAutoCreateDraftAnimals, p8) }
	pub fn _CREATE_DRAFT_VEHICLE(modelHash: Hash, x: f32, y: f32, z: f32, heading: f32, isNetwork: bool, bScriptHostVeh: bool, bDontAutoCreateDraftAnimals: bool, draftAnimalPopGroup: Hash, p9: bool) -> Vehicle { invoke!(0x214651FB1DFEBA89, modelHash, x, y, z, heading, isNetwork, bScriptHostVeh, bDontAutoCreateDraftAnimals, draftAnimalPopGroup, p9) }
	pub fn DELETE_VEHICLE(vehicle: &mut Vehicle) { invoke_ignore!(0xE20A909D8C4A70F8, vehicle) }
	pub fn _FADE_AND_DESTROY_VEHICLE(vehicle: &mut Vehicle) { invoke_ignore!(0x35DC1877312FBA0F, vehicle) }
	pub fn _IS_VEHICLE_FADING_OUT(vehicle: Vehicle) -> bool { invoke!(0x5136B284B67B35C7, vehicle) }
	pub fn SET_VEHICLE_ALLOW_HOMING_MISSLE_LOCKON(vehicle: Vehicle, toggle: bool) { invoke_ignore!(0x1240E8596A8308B9, vehicle, toggle) }
	pub fn SET_VEHICLE_ALLOW_NO_PASSENGERS_LOCKON(vehicle: Vehicle, toggle: bool) { invoke_ignore!(0xECB9E9BC887E8060, vehicle, toggle) }
	pub fn IS_VEHICLE_MODEL(vehicle: Vehicle, model: Hash) -> bool { invoke!(0x0045A54EC7A22455, vehicle, model) }
	pub fn _SET_ALL_VEHICLE_GENERATORS_DISABLED_FOR_VOLUME(volume: Volume, toggle: bool) { invoke_ignore!(0x424FFCB9F0D2D4B5, volume, toggle) }
	pub fn SET_ALL_VEHICLE_GENERATORS_ACTIVE_IN_AREA(x1: f32, y1: f32, z1: f32, x2: f32, y2: f32, z2: f32, p6: bool, p7: bool) { invoke_ignore!(0xBBB134FB9D50C0CC, x1, y1, z1, x2, y2, z2, p6, p7) }
	pub fn SET_ALL_VEHICLE_GENERATORS_ACTIVE() { invoke_ignore!(0x3D596E6E88A02C24) }
	pub fn SET_VEHICLE_ON_GROUND_PROPERLY(vehicle: Vehicle, p1: bool) -> bool { invoke!(0x7263332501E07F52, vehicle, p1) }
	pub fn IS_VEHICLE_STOPPED(vehicle: Vehicle) -> bool { invoke!(0x78C3311A73135241, vehicle) }
	pub fn GET_VEHICLE_NUMBER_OF_PASSENGERS(vehicle: Vehicle) -> i32 { invoke!(0x59F3F16577CD79B2, vehicle) }
	pub fn GET_VEHICLE_MAX_NUMBER_OF_PASSENGERS(vehicle: Vehicle) -> i32 { invoke!(0xA9C55F1C15E62E06, vehicle) }
	pub fn GET_VEHICLE_MODEL_NUMBER_OF_SEATS(modelHash: Hash) -> i32 { invoke!(0x9A578736FF3A17C3, modelHash) }
	pub fn IS_SEAT_WARP_ONLY(vehicle: Vehicle, seatIndex: i32) -> bool { invoke!(0x7892685BF6D9775E, vehicle, seatIndex) }
	pub fn _GET_VEHICLE_TURRET_SEAT(vehicle: Vehicle, seatIndex: &mut i32) -> bool { invoke!(0xFF5791B7639C2A46, vehicle, seatIndex) }
	pub fn _0xA9E185D498B9AC67(p0: Any, p1: Any) -> Any { invoke!(0xA9E185D498B9AC67, p0, p1) }
	pub fn SET_VEHICLE_DENSITY_MULTIPLIER_THIS_FRAME(multiplier: f32) { invoke_ignore!(0x606374EBFC27B133, multiplier) }
	pub fn SET_RANDOM_VEHICLE_DENSITY_MULTIPLIER_THIS_FRAME(multiplier: f32) { invoke_ignore!(0x1F91D44490E1EA0C, multiplier) }
	pub fn SET_PARKED_VEHICLE_DENSITY_MULTIPLIER_THIS_FRAME(multiplier: f32) { invoke_ignore!(0xFEDFA97638D61D4A, multiplier) }
	pub fn SET_DISABLE_RANDOM_TRAINS_THIS_FRAME(toggle: bool) { invoke_ignore!(0xD4288603E8766FF7, toggle) }
	pub fn SET_VEHICLE_DOORS_LOCKED(vehicle: Vehicle, doorLockStatus: i32) { invoke_ignore!(0x96F78A6A075D55D9, vehicle, doorLockStatus) }
	pub fn SET_VEHICLE_INDIVIDUAL_DOORS_LOCKED(vehicle: Vehicle, doorId: i32, doorLockStatus: i32) { invoke_ignore!(0xA9F1D75195CC40F6, vehicle, doorId, doorLockStatus) }
	pub fn SET_VEHICLE_DOORS_LOCKED_FOR_PLAYER(vehicle: Vehicle, player: Player, toggle: bool) { invoke_ignore!(0x359A8EA1FB8D6F0F, vehicle, player, toggle) }
	pub fn GET_VEHICLE_DOORS_LOCKED_FOR_PLAYER(vehicle: Vehicle, player: Player) -> bool { invoke!(0xFA2CDDFEB8BC898B, vehicle, player) }
	pub fn SET_VEHICLE_DOORS_LOCKED_FOR_ALL_PLAYERS(vehicle: Vehicle, toggle: bool) { invoke_ignore!(0x2381977DA948F8DC, vehicle, toggle) }
	pub fn SET_VEHICLE_DOORS_LOCKED_FOR_TEAM(vehicle: Vehicle, team: i32, toggle: bool) { invoke_ignore!(0xE712BC978770F105, vehicle, team, toggle) }
	pub fn _GET_VEHICLE_DOORS_LOCKED_FOR_TEAM(vehicle: Vehicle, team: i32) -> bool { invoke!(0xDD1E1393D966D39A, vehicle, team) }
	pub fn EXPLODE_VEHICLE(vehicle: Vehicle, isAudible: bool, isInvisible: bool, p3: Any, p4: Any) { invoke_ignore!(0x75DCED9EEC5769D7, vehicle, isAudible, isInvisible, p3, p4) }
	pub fn _0x750D42C013F64AE7(p0: Any, p1: Any) { invoke_ignore!(0x750D42C013F64AE7, p0, p1) }
	pub fn _0xE78993FF9022C064(p0: Any) { invoke_ignore!(0xE78993FF9022C064, p0) }
	pub fn _0x9868C0D0134855F7(p0: Any) { invoke_ignore!(0x9868C0D0134855F7, p0) }
	pub fn _HIDE_HORSE_REINS(vehicle: Vehicle) { invoke_ignore!(0x201B8ED4FF7FE9F5, vehicle) }
	pub fn _SHOW_HORSE_REINS(vehicle: Vehicle) { invoke_ignore!(0x41CDA90EE3450921, vehicle) }
	pub fn _0xD21A3D421E7F09F7(p0: Any, p1: Any) { invoke_ignore!(0xD21A3D421E7F09F7, p0, p1) }
	pub fn _0xA13028E22564A1BD(p0: Any, p1: Any) { invoke_ignore!(0xA13028E22564A1BD, p0, p1) }
	pub fn _0x485B05EF05B9AEE9(p0: Any, p1: Any) { invoke_ignore!(0x485B05EF05B9AEE9, p0, p1) }
	pub fn SET_BOAT_ANCHOR(vehicle: Vehicle, toggle: bool) { invoke_ignore!(0xAEAB044F05B92659, vehicle, toggle) }
	pub fn _0x6B53F4B811E583D2(vehicle: Vehicle, toggle: bool) { invoke_ignore!(0x6B53F4B811E583D2, vehicle, toggle) }
	pub fn CAN_ANCHOR_BOAT_HERE(vehicle: Vehicle) -> bool { invoke!(0xC075176CFB8B4128, vehicle) }
	pub fn SET_BOAT_REMAINS_ANCHORED_WHILE_PLAYER_IS_DRIVER(vehicle: Vehicle, p1: bool, p2: bool) { invoke_ignore!(0x286771F3059A37A7, vehicle, p1, p2) }
	pub fn SET_FORCE_LOW_LOD_ANCHOR_MODE(vehicle: Vehicle, p1: bool) { invoke_ignore!(0x75B49ACD73617437, vehicle, p1) }
	pub fn SET_BOAT_LOW_LOD_ANCHOR_DISTANCE(vehicle: Vehicle, value: f32) { invoke_ignore!(0xE3261532550D6A9F, vehicle, value) }
	pub fn SET_BOAT_SINKS_WHEN_WRECKED(vehicle: Vehicle, toggle: bool) { invoke_ignore!(0x62A6D317A011EA1D, vehicle, toggle) }
	pub fn _SET_FORCE_HIGH_LOD_VEHICLE(vehicle: Vehicle, p1: bool) { invoke_ignore!(0x1098CDA477890165, vehicle, p1) }
	pub fn _0x98A7598C579EE871(p0: Any, p1: Any, p2: Any) { invoke_ignore!(0x98A7598C579EE871, p0, p1, p2) }
	pub fn _0x9E8711C81AA17876(vehicle: Vehicle, p1: bool) { invoke_ignore!(0x9E8711C81AA17876, vehicle, p1) }
	pub fn SET_VEHICLE_STRONG(vehicle: Vehicle, toggle: bool) { invoke_ignore!(0xAB315515C9F8803D, vehicle, toggle) }
	pub fn IS_VEHICLE_SEAT_FREE(vehicle: Vehicle, seatIndex: i32) -> bool { invoke!(0xE052C1B1CAA4ECE4, vehicle, seatIndex) }
	pub fn GET_PED_IN_VEHICLE_SEAT(vehicle: Vehicle, seatIndex: i32) -> Ped { invoke!(0xBB40DD2270B65366, vehicle, seatIndex) }
	pub fn GET_LAST_PED_IN_VEHICLE_SEAT(vehicle: Vehicle, seatIndex: i32) -> Ped { invoke!(0x74583B19FEEAFDA7, vehicle, seatIndex) }
	pub fn IS_DRAFT_VEHICLE(vehicle: Vehicle) -> bool { invoke!(0xEA44E97849E9F3DD, vehicle) }
	pub fn _GET_PED_IN_DRAFT_HARNESS(vehicle: Vehicle, harnessId: i32) -> Ped { invoke!(0xA8BA0BAE0173457B, vehicle, harnessId) }
	pub fn SET_VEHICLE_FORWARD_SPEED(vehicle: Vehicle, speed: f32) { invoke_ignore!(0xF9F92AF49F12F6E7, vehicle, speed) }
	pub fn BRING_VEHICLE_TO_HALT(vehicle: Vehicle, distance: f32, duration: i32, unknown: bool) { invoke_ignore!(0x260BE8F09E326A20, vehicle, distance, duration, unknown) }
	pub fn _IS_VEHICLE_BROUGHT_TO_HALT(vehicle: Vehicle) -> bool { invoke!(0x404527BC03DA0E6C, vehicle) }
	pub fn STOP_BRINGING_VEHICLE_TO_HALT(vehicle: Vehicle) { invoke_ignore!(0x7C06330BFDDA182E, vehicle) }
	pub fn _0xE12F5ED49F44D40D(p0: Any) { invoke_ignore!(0xE12F5ED49F44D40D, p0) }
	pub fn _0xF6E3D38869D0F7AD(p0: Any) { invoke_ignore!(0xF6E3D38869D0F7AD, p0) }
	pub fn SET_VEHICLE_DOORS_SHUT(vehicle: Vehicle, closeInstantly: bool) { invoke_ignore!(0xA4FFCD645B11F25A, vehicle, closeInstantly) }
	pub fn SET_VEHICLE_TYRES_CAN_BURST(vehicle: Vehicle, toggle: bool) { invoke_ignore!(0xEBD0A4E935106FE5, vehicle, toggle) }
	pub fn SET_VEHICLE_WHEELS_CAN_BREAK(vehicle: Vehicle, enabled: bool) { invoke_ignore!(0x839137C40275FB77, vehicle, enabled) }
	pub fn SET_VEHICLE_DOORS_TO_OPEN_AT_ANY_DISTANCE(vehicle: Vehicle, toggle: bool) { invoke_ignore!(0x362CEDD2A41E0747, vehicle, toggle) }
	pub fn SET_VEHICLE_DOOR_OPEN(vehicle: Vehicle, doorId: i32, loose: bool, openInstantly: bool) { invoke_ignore!(0x550CE392A4672412, vehicle, doorId, loose, openInstantly) }
	pub fn REMOVE_VEHICLE_WINDOW(vehicle: Vehicle, windowIndex: i32) { invoke_ignore!(0x745F15A215F2DDF1, vehicle, windowIndex) }
	pub fn _0x8878FF3EEE2868A9(p0: Any, p1: Any) { invoke_ignore!(0x8878FF3EEE2868A9, p0, p1) }
	pub fn _SET_VEHICLE_DIRT_LEVEL_2(vehicle: Vehicle, dirtLevel: f32) { invoke_ignore!(0xBAE0EEDF93F05EAA, vehicle, dirtLevel) }
	pub fn _SET_VEHICLE_MUD_LEVEL(vehicle: Vehicle, mudLevel: f32) { invoke_ignore!(0x4D15E49764CB328A, vehicle, mudLevel) }
	pub fn SET_VEHICLE_LIGHTS(vehicle: Vehicle, state: i32) { invoke_ignore!(0x629F0A0E952CAE7D, vehicle, state) }
	pub fn SET_RANDOM_TRAINS(toggle: bool) { invoke_ignore!(0x1156C6EE7E82A98A, toggle) }
	pub fn _0x331CBD247FC5DAA8(configHash: Hash, x: f32, y: f32, z: f32, direction: bool, p5: bool) -> i32 { invoke!(0x331CBD247FC5DAA8, configHash, x, y, z, direction, p5) }
	pub fn _0x0516FAE561276EFC(trackIndex: i32) -> bool { invoke!(0x0516FAE561276EFC, trackIndex) }
	pub fn _GET_TRAIN_TRACK_FROM_TRAIN_VEHICLE(train: Vehicle) -> i32 { invoke!(0x45853F4E17D847D5, train) }
	pub fn _GET_TRAIN_VEHICLE_FROM_TRACK_INDEX(trackIndex: i32) -> Vehicle { invoke!(0x6E585A616ABB8401, trackIndex) }
	pub fn _0x15206E88FF7617DF(trackIndex: i32, p1: f32) { invoke_ignore!(0x15206E88FF7617DF, trackIndex, p1) }
	pub fn _0xA7966807953A18EE(trackIndex: i32, p1: f32) { invoke_ignore!(0xA7966807953A18EE, trackIndex, p1) }
	pub fn _0x6B34BE961F639E21(trackIndex: i32, p1: i32) { invoke_ignore!(0x6B34BE961F639E21, trackIndex, p1) }
	pub fn _0xE6BD7DD3FD474415(train: Vehicle, p1: bool) { invoke_ignore!(0xE6BD7DD3FD474415, train, p1) }
	pub fn _0x615B3B8E73634509(trackIndex: i32, p1: f32) { invoke_ignore!(0x615B3B8E73634509, trackIndex, p1) }
	pub fn _0x38E7DD70A242D5CB(trackIndex: i32, p1: i32) { invoke_ignore!(0x38E7DD70A242D5CB, trackIndex, p1) }
	pub fn _0x63509DDF102E08E8(trackIndex: i32, p1: i32) { invoke_ignore!(0x63509DDF102E08E8, trackIndex, p1) }
	pub fn _0x7408B5C66BA31ADB(p0: Any, p1: Any, p2: Any, p3: Any, p4: Any, p5: Any, p6: Any, p7: Any, p8: Any, p9: Any, p10: Any) { invoke_ignore!(0x7408B5C66BA31ADB, p0, p1, p2, p3, p4, p5, p6, p7, p8, p9, p10) }
	pub fn _0x41365DB586CD9E8E(trackIndex: i32, p1: f32) { invoke_ignore!(0x41365DB586CD9E8E, trackIndex, p1) }
	pub fn _0xD0AABE5B9F8FA589(trackIndex: i32, p1: f32) { invoke_ignore!(0xD0AABE5B9F8FA589, trackIndex, p1) }
	pub fn _0x427C919E9809E370(trackIndex: i32, p1: i32) { invoke_ignore!(0x427C919E9809E370, trackIndex, p1) }
	pub fn _DOES_TRAIN_EXIST_ON_TRACK(trackIndex: i32) -> bool { invoke!(0xC29996A337BDD099, trackIndex) }
	pub fn _GET_TRAIN_POSITION_ON_TRACK(trackIndex: i32) -> Vector3 { invoke!(0x1E8A921112891651, trackIndex) }
	pub fn _0xB4241AD8F5AEE9ED(trackIndex: i32) -> bool { invoke!(0xB4241AD8F5AEE9ED, trackIndex) }
	pub fn _0xA230A5DDE12ED374(p0: Any) { invoke_ignore!(0xA230A5DDE12ED374, p0) }
	pub fn _0x0D5FDF0D36FA10CD(trackIndex: i32) { invoke_ignore!(0x0D5FDF0D36FA10CD, trackIndex) }
	pub fn _0xE682002DB1F30669(p0: Any) { invoke_ignore!(0xE682002DB1F30669, p0) }
	pub fn _0x718EB706B6E998A0(trackIndex: i32) { invoke_ignore!(0x718EB706B6E998A0, trackIndex) }
	pub fn _0xF05DFAF1ADFEF2CD(trainConfig: Hash, x: f32, y: f32, z: f32, direction: bool, p5: bool) -> bool { invoke!(0xF05DFAF1ADFEF2CD, trainConfig, x, y, z, direction, p5) }
	pub fn _0xD1DF5E54F4ACBE1A(p0: Any, p1: Any, p2: Any, p3: Any, p4: Any, p5: Any, p6: Any) -> Any { invoke!(0xD1DF5E54F4ACBE1A, p0, p1, p2, p3, p4, p5, p6) }
	pub fn _0x0FDDEE66E3465726(p0: Any) -> Any { invoke!(0x0FDDEE66E3465726, p0) }
	pub fn _0x4C05B42A8D937796() { invoke_ignore!(0x4C05B42A8D937796) }
	pub fn _0xB961DD799A837BD7() { invoke_ignore!(0xB961DD799A837BD7) }
	pub fn _0x16B86A49E072AA85() { invoke_ignore!(0x16B86A49E072AA85) }
	pub fn _0x2A7413168F6CD5A8() { invoke_ignore!(0x2A7413168F6CD5A8) }
	pub fn _0xFFFE15B433300B8C(p0: Any, p1: Any, p2: Any) { invoke_ignore!(0xFFFE15B433300B8C, p0, p1, p2) }
	pub fn _0x6EA1273D525427F4(p0: Any, p1: Any, p2: Any) { invoke_ignore!(0x6EA1273D525427F4, p0, p1, p2) }
	pub fn _0x7BE0746539DEF0C8(p0: Any, p1: Any) -> Any { invoke!(0x7BE0746539DEF0C8, p0, p1) }
	pub fn _0x3137EDC899E6DAE4(p0: Any, p1: Any) { invoke_ignore!(0x3137EDC899E6DAE4, p0, p1) }
	pub fn _0x6C87F49BFA181DB5(x: f32, y: f32, z: f32) -> i32 { invoke!(0x6C87F49BFA181DB5, x, y, z) }
	pub fn _GET_TRACK_INDEX_FROM_COORDS(x: f32, y: f32, z: f32) -> i32 { invoke!(0x85D39F5E3B6D7EB0, x, y, z) }
	pub fn _GET_NEAREST_TRAIN_TRACK_POSITION(x: f32, y: f32, z: f32) -> Vector3 { invoke!(0x6DE03BCC15E81710, x, y, z) }
	pub fn DELETE_ALL_TRAINS() { invoke_ignore!(0xA3120A1385F17FF7) }
	pub fn _0x0E558D3A49D759D6(p0: Any, p1: Any) -> Any { invoke!(0x0E558D3A49D759D6, p0, p1) }
	pub fn _0xD4907EF4334C7602(p0: Any, p1: Any) { invoke_ignore!(0xD4907EF4334C7602, p0, p1) }
	pub fn _0x68830738A6BFB370(p0: Any, p1: Any) { invoke_ignore!(0x68830738A6BFB370, p0, p1) }
	pub fn SET_TRAIN_SPEED(train: Vehicle, speed: f32) { invoke_ignore!(0xDFBA6BBFF7CCAFBB, train, speed) }
	pub fn _SET_TRAIN_MAX_SPEED(train: Vehicle, speed: f32) { invoke_ignore!(0x9F29999DFDF2AEB8, train, speed) }
	pub fn SET_TRAIN_CRUISE_SPEED(train: Vehicle, speed: f32) { invoke_ignore!(0x01021EB2E96B793C, train, speed) }
	pub fn _GET_TRAIN_CARRIAGE_TRAILER_NUMBER(train: Vehicle) -> i32 { invoke!(0x60B7D1DCC312697D, train) }
	pub fn _GET_TRAIN_MODEL_FROM_TRAIN_CONFIG_BY_CAR_INDEX(trainConfig: Hash, trainCarIndex: i32) -> Hash { invoke!(0x8DF5F6A19F99F0D5, trainConfig, trainCarIndex) }
	pub fn _GET_NUM_CARS_FROM_TRAIN_CONFIG(trainConfig: Hash) -> i32 { invoke!(0x635423D55CA84FC8, trainConfig) }
	pub fn _GET_TRAIN_CAR(train: Vehicle) -> Entity { invoke!(0x671A07C9A1CD50A5, train) }
	pub fn _SET_TRAIN_STOPS_FOR_STATIONS(train: Vehicle, toggle: bool) { invoke_ignore!(0x4182C037AA1F0091, train, toggle) }
	pub fn _0xDD100CE1EBBF37E3(p0: Any, p1: Any) { invoke_ignore!(0xDD100CE1EBBF37E3, p0, p1) }
	pub fn _0x160C1B5AB48AB87C(train: Vehicle, p1: f32) { invoke_ignore!(0x160C1B5AB48AB87C, train, p1) }
	pub fn IS_TRAIN_WAITING_AT_STATION(train: Vehicle) -> bool { invoke!(0xE887BD31D97793F6, train) }
	pub fn _SET_TRAIN_HALT(train: Vehicle) { invoke_ignore!(0x3660BCAB3A6BB734, train) }
	pub fn _SET_TRAIN_LEAVE_STATION(train: Vehicle) { invoke_ignore!(0x787E43477746876F, train) }
	pub fn SET_RANDOM_BOATS(toggle: bool) { invoke_ignore!(0xF44D446D4E36DB87, toggle) }
	pub fn REQUEST_VEHICLE_RECORDING(recording: i32, script: & CStr) { invoke_ignore!(0xC474CF16EDA45DC9, recording, script) }
	pub fn HAS_VEHICLE_RECORDING_BEEN_LOADED(recording: i32, script: & CStr) -> bool { invoke!(0xBA9325BE372AB6EA, recording, script) }
	pub fn REMOVE_VEHICLE_RECORDING(p0: Any, p1: &mut Any) { invoke_ignore!(0x139E35755418F6AA, p0, p1) }
	pub fn GET_POSITION_OF_VEHICLE_RECORDING_AT_TIME(recording: i32, time: f32, script: & CStr) -> Vector3 { invoke!(0x1A00961A1BE94E5E, recording, time, script) }
	pub fn GET_ROTATION_OF_VEHICLE_RECORDING_AT_TIME(recording: i32, time: f32, script: & CStr) -> Vector3 { invoke!(0x61787DD28B8CC0D5, recording, time, script) }
	pub fn GET_TIME_POSITION_IN_RECORDING(vehicle: Vehicle) -> f32 { invoke!(0x233B51C7913FA031, vehicle) }
	pub fn START_PLAYBACK_RECORDED_VEHICLE(vehicle: Vehicle, recording: i32, script: & CStr, p3: bool) { invoke_ignore!(0x4932B84E3276508E, vehicle, recording, script, p3) }
	pub fn FORCE_PLAYBACK_RECORDED_VEHICLE_UPDATE(vehicle: Vehicle, p1: bool) { invoke_ignore!(0x59ECA796021B0539, vehicle, p1) }
	pub fn STOP_PLAYBACK_RECORDED_VEHICLE(vehicle: Vehicle) { invoke_ignore!(0xBF9B4D6267E8C26D, vehicle) }
	pub fn IS_PLAYBACK_GOING_ON_FOR_VEHICLE(vehicle: Vehicle) -> bool { invoke!(0x02774B3A9034278F, vehicle) }
	pub fn IS_PLAYBACK_USING_AI_GOING_ON_FOR_VEHICLE(vehicle: Vehicle) -> bool { invoke!(0x5A7472606EC5B7C1, vehicle) }
	pub fn SET_PLAYBACK_SPEED(vehicle: Vehicle, speed: f32) { invoke_ignore!(0xD78084EED4CD94C6, vehicle, speed) }
	pub fn SKIP_TIME_IN_PLAYBACK_RECORDED_VEHICLE(vehicle: Vehicle, time: f32) { invoke_ignore!(0x5F5E6379C59EFC56, vehicle, time) }
	pub fn GET_CLOSEST_VEHICLE(x: f32, y: f32, z: f32, radius: f32, modelHash: Hash, flags: i32) -> Vehicle { invoke!(0x52F45D033645181B, x, y, z, radius, modelHash, flags) }
	pub fn GET_TRAIN_CARRIAGE(train: Vehicle, trailerNumber: i32) -> Entity { invoke!(0xD0FB093A4CDB932C, train, trailerNumber) }
	pub fn DELETE_MISSION_TRAIN(train: &mut Vehicle) { invoke_ignore!(0x0D3630FB07E8B570, train) }
	pub fn SET_MISSION_TRAIN_AS_NO_LONGER_NEEDED(train: &mut Vehicle, flags: i32) { invoke_ignore!(0xBBE7648349B49BE8, train, flags) }
	pub fn SET_MISSION_TRAIN_COORDS(train: Vehicle, x: f32, y: f32, z: f32) { invoke_ignore!(0x7632755962AB9922, train, x, y, z) }
	pub fn _SET_MISSION_TRAIN_WARP_TO_COORDS(train: Vehicle, x: f32, y: f32, z: f32, direction: bool) { invoke_ignore!(0xC9EA26893C9E4024, train, x, y, z, direction) }
	pub fn _0xA72B1BF3857B94D7(train: Vehicle, p1: bool) { invoke_ignore!(0xA72B1BF3857B94D7, train, p1) }
	pub fn _IS_THIS_MODEL_A_DRAFT_VEHICLE(model: Hash) -> bool { invoke!(0xB9D5BDDA88E1BB66, model) }
	pub fn IS_THIS_MODEL_A_BOAT(model: Hash) -> bool { invoke!(0x799CFC7C5B743B15, model) }
	pub fn IS_THIS_MODEL_A_TRAIN(model: Hash) -> bool { invoke!(0xFC08C8F8C1EDF174, model) }
	pub fn SET_VEHICLE_CAN_BE_TARGETTED(vehicle: Vehicle, state: bool) { invoke_ignore!(0x05254BA0B44ADC16, vehicle, state) }
	pub fn SET_DONT_ALLOW_PLAYER_TO_ENTER_VEHICLE_IF_LOCKED_FOR_PLAYER(vehicle: Vehicle, p1: bool) { invoke_ignore!(0x63DC1F22C903B709, vehicle, p1) }
	pub fn SET_VEHICLE_CAN_BE_VISIBLY_DAMAGED(vehicle: Vehicle, state: bool) { invoke_ignore!(0x4BF8131AE811541C, vehicle, state) }
	pub fn SET_VEHICLE_HAS_UNBREAKABLE_LIGHTS(vehicle: Vehicle, p1: bool) { invoke_ignore!(0xC903855E028A05F2, vehicle, p1) }
	pub fn SET_VEHICLE_RESPECTS_LOCKS_WHEN_HAS_DRIVER(vehicle: Vehicle, p1: bool) { invoke_ignore!(0x33992A808DF1C1BA, vehicle, p1) }
	pub fn SET_VEHICLE_CAN_EJECT_PASSENGERS_IF_LOCKED(vehicle: Vehicle, p1: bool) { invoke_ignore!(0x065D03A9D6B2C6B5, vehicle, p1) }
	pub fn SET_VEHICLE_DIRT_LEVEL(vehicle: Vehicle, dirtLevel: f32) { invoke_ignore!(0x758C3460EE915D0A, vehicle, dirtLevel) }
	pub fn IS_VEHICLE_DOOR_FULLY_OPEN(vehicle: Vehicle, doorId: i32) -> bool { invoke!(0x7AE191143C7A9107, vehicle, doorId) }
	pub fn SET_VEHICLE_ENGINE_ON(vehicle: Vehicle, value: bool, instantly: bool) { invoke_ignore!(0xB64CFA14CB9A2E78, vehicle, value, instantly) }
	pub fn SET_VEHICLE_UNDRIVEABLE(vehicle: Vehicle, toggle: bool) { invoke_ignore!(0x6E884BAB713A2A94, vehicle, toggle) }
	pub fn SET_VEHICLE_PROVIDES_COVER(vehicle: Vehicle, toggle: bool) { invoke_ignore!(0x652712478F1721F4, vehicle, toggle) }
	pub fn _0x3053064F909B5F42(p0: Any, p1: Any) { invoke_ignore!(0x3053064F909B5F42, p0, p1) }
	pub fn SET_VEHICLE_DOOR_CONTROL(vehicle: Vehicle, doorId: i32, speed: i32, angle: f32) { invoke_ignore!(0xD57F10EBBA814ECF, vehicle, doorId, speed, angle) }
	pub fn SET_VEHICLE_DOOR_LATCHED(vehicle: Vehicle, doorId: i32, p2: bool, p3: bool, p4: bool) { invoke_ignore!(0x06F8A202EB312A3C, vehicle, doorId, p2, p3, p4) }
	pub fn SET_VEHICLE_DOOR_SHUT(vehicle: Vehicle, doorId: i32, closeInstantly: bool) { invoke_ignore!(0x6A3C24B91FD0EA09, vehicle, doorId, closeInstantly) }
	pub fn SET_VEHICLE_DOOR_BROKEN(vehicle: Vehicle, doorId: i32, deleteDoor: bool) { invoke_ignore!(0x9666CF20A1C6D780, vehicle, doorId, deleteDoor) }
	pub fn SET_VEHICLE_CAN_BREAK(vehicle: Vehicle, toggle: bool) { invoke_ignore!(0xC5ED9D59B4646611, vehicle, toggle) }
	pub fn SET_VEHICLE_IS_CONSIDERED_BY_PLAYER(vehicle: Vehicle, toggle: bool) { invoke_ignore!(0x54800D386C5825E5, vehicle, toggle) }
	pub fn SET_VEHICLE_MAY_BE_USED_BY_GOTO_POINT_ANY_MEANS(vehicle: Vehicle, p1: bool) { invoke_ignore!(0x7549B9E841940695, vehicle, p1) }
	pub fn GET_VEHICLE_DOOR_LOCK_STATUS(vehicle: Vehicle) -> i32 { invoke!(0xC867FD144F2469D3, vehicle) }
	pub fn SET_DOOR_ALLOWED_TO_BE_BROKEN_OFF(vehicle: Vehicle, doorId: i32, isBreakable: bool) { invoke_ignore!(0x081FB9D6422F804C, vehicle, doorId, isBreakable) }
	pub fn IS_VEHICLE_ON_ALL_WHEELS(vehicle: Vehicle) -> bool { invoke!(0x0D5D119529654EE0, vehicle) }
	pub fn _GET_TRAIN_DIRECTION(train: Vehicle) -> bool { invoke!(0x3C9628A811CBD724, train) }
	pub fn _GET_TRAIN_DIRECTION_FROM_INDEX(trackIndex: i32) -> bool { invoke!(0x67995318F5FAA496, trackIndex) }
	pub fn _0x09034479E6E3E269(train: Vehicle, trainTrack: &mut Hash, junctionIndex: &mut i32) -> Any { invoke!(0x09034479E6E3E269, train, trainTrack, junctionIndex) }
	pub fn _GET_TRAIN_TRACK_JUNCTION_AT_COORDS(trainTrack: Hash, x: f32, y: f32, z: f32, junctionIndex: &mut i32) -> bool { invoke!(0x86AFC343CF7F0B34, trainTrack, x, y, z, junctionIndex) }
	pub fn _0xD9BF3ED8EFB67EA3(p0: Any, p1: Any, p2: Any, p3: Any, p4: Any) -> Any { invoke!(0xD9BF3ED8EFB67EA3, p0, p1, p2, p3, p4) }
	pub fn _0x785639D89F8451AB(p0: Any, p1: Any) -> Vector3 { invoke!(0x785639D89F8451AB, p0, p1) }
	pub fn _SET_TRAIN_TRACK_JUNCTION_SWITCH(trainTrack: Hash, junctionIndex: i32, enabled: bool) { invoke_ignore!(0xE6C5E2125EB210C1, trainTrack, junctionIndex, enabled) }
	pub fn _0x3ABFA128F5BF5A70(trainTrack: Hash, junctionIndex: i32, enabled: bool) { invoke_ignore!(0x3ABFA128F5BF5A70, trainTrack, junctionIndex, enabled) }
	pub fn _0x2C46D2A591D8C322(p0: Any, p1: Any, p2: Any) -> Any { invoke!(0x2C46D2A591D8C322, p0, p1, p2) }
	pub fn _0xCAFF2C9747103C02(p0: Any, p1: Any, p2: Any) -> Any { invoke!(0xCAFF2C9747103C02, p0, p1, p2) }
	pub fn _SET_ALL_JUNCTIONS_CLEARED() { invoke_ignore!(0x138398153824E332) }
	pub fn _0x34BCF6209B9668A7(trackIndex: i32, p1: Any) { invoke_ignore!(0x34BCF6209B9668A7, trackIndex, p1) }
	pub fn _0xD0BA1853D76683C8(trackIndex: i32, x: f32, y: f32, z: f32, p4: Any) { invoke_ignore!(0xD0BA1853D76683C8, trackIndex, x, y, z, p4) }
	pub fn SET_TRAIN_OFFSET_FROM_STATION(train: Vehicle, offset: f32) { invoke_ignore!(0x8EC47DD4300BF063, train, offset) }
	pub fn _0xDC69F6913CCA0B99(p0: Any, p1: Any) { invoke_ignore!(0xDC69F6913CCA0B99, p0, p1) }
	pub fn _0x7840576C50A13DBA(train: Vehicle, p1: bool) { invoke_ignore!(0x7840576C50A13DBA, train, p1) }
	pub fn _0xD0116DF21E6C7B36(p0: Any, p1: Any) -> Any { invoke!(0xD0116DF21E6C7B36, p0, p1) }
	pub fn _DETACH_WAGON_ENTITY_FROM_TRAIN(entity: Entity) { invoke_ignore!(0x54CBDD6E1B4CB4DF, entity) }
	pub fn _0x1180A2974D251B7B(train: Vehicle) -> i32 { invoke!(0x1180A2974D251B7B, train) }
	pub fn GET_CURRENT_STATION_FOR_TRAIN(train: Vehicle) -> i32 { invoke!(0x86FA6D8B48667D75, train) }
	pub fn _0x9CC94A948EAF5372(trackIndex: i32, stationIndex: i32) -> Hash { invoke!(0x9CC94A948EAF5372, trackIndex, stationIndex) }
	pub fn _0xDE8C5B9F65017FA1(train: Vehicle) -> Any { invoke!(0xDE8C5B9F65017FA1, train) }
	pub fn _GET_CURRENT_TRACK_FOR_TRAIN(train: Vehicle) -> i32 { invoke!(0xAF787E081AC4A8EE, train) }
	pub fn _GET_STATION_COORDS_FROM_TRAIN_STATION_DATA(trackIndex: i32, stationIndex: i32) -> Vector3 { invoke!(0xBA958F68031DDBFC, trackIndex, stationIndex) }
	pub fn GET_TRACK_INDEX_OF_TRAIN(train: Vehicle) -> i32 { invoke!(0x865FEC2FA899F29C, train) }
	pub fn _0x1A861F899EBBE17C(train: Vehicle, p1: bool) { invoke_ignore!(0x1A861F899EBBE17C, train, p1) }
	pub fn _0xF8F7DA13CFBD4532(trackIndex: i32, p1: bool) { invoke_ignore!(0xF8F7DA13CFBD4532, trackIndex, p1) }
	pub fn _TRIGGER_TRAIN_WHISTLE(train: Vehicle, whistleSequence: & CStr, p2: bool, p3: bool) { invoke_ignore!(0xCFE122EC635CC2B2, train, whistleSequence, p2, p3) }
	pub fn _0x2BB2B5BCF0DF8008(p0: Any, p1: Any) { invoke_ignore!(0x2BB2B5BCF0DF8008, p0, p1) }
	pub fn _0x6703872EC09BC158(p0: Any, p1: Any) { invoke_ignore!(0x6703872EC09BC158, p0, p1) }
	pub fn _0x1BFBAFCC6760FF02(train: Vehicle, p1: bool) { invoke_ignore!(0x1BFBAFCC6760FF02, train, p1) }
	pub fn _0xF5EA41C1408695FB(p0: Any, p1: Any, p2: Any, p3: Any) -> Any { invoke!(0xF5EA41C1408695FB, p0, p1, p2, p3) }
	pub fn SET_VEHICLE_FIXED(vehicle: Vehicle) { invoke_ignore!(0x79811282A9D1AE56, vehicle) }
	pub fn SET_DISABLE_VEHICLE_PETROL_TANK_FIRES(vehicle: Vehicle, toggle: bool) { invoke_ignore!(0xB70986AB19B04AFF, vehicle, toggle) }
	pub fn SET_DISABLE_VEHICLE_PETROL_TANK_DAMAGE(vehicle: Vehicle, toggle: bool) { invoke_ignore!(0x5795FBE7A2001C14, vehicle, toggle) }
	pub fn SET_DISABLE_VEHICLE_ENGINE_FIRES(vehicle: Vehicle, p1: bool) { invoke_ignore!(0xD146EE5F2B06B95E, vehicle, p1) }
	pub fn SET_VEHICLE_LIMIT_SPEED_WHEN_PLAYER_INACTIVE(vehicle: Vehicle, p1: bool) { invoke_ignore!(0x8F75941C86EEBFCA, vehicle, p1) }
	pub fn SET_VEHICLE_STOP_INSTANTLY_WHEN_PLAYER_INACTIVE(vehicle: Vehicle, p1: bool) { invoke_ignore!(0xC84E138448507567, vehicle, p1) }
	pub fn REMOVE_VEHICLES_FROM_GENERATORS_IN_AREA(p0: Any, p1: Any, p2: Any, p3: Any, p4: Any, p5: Any) { invoke_ignore!(0xC619A44639BC0CB4, p0, p1, p2, p3, p4, p5) }
	pub fn SET_VEHICLE_STEER_BIAS(vehicle: Vehicle, value: f32) { invoke_ignore!(0x84DAAE11E9EE4FC3, vehicle, value) }
	pub fn IS_VEHICLE_EXTRA_TURNED_ON(vehicle: Vehicle, extraId: i32) -> bool { invoke!(0xFA9A55D9C4351625, vehicle, extraId) }
	pub fn SET_VEHICLE_EXTRA(vehicle: Vehicle, extraId: i32, disable: bool) { invoke_ignore!(0xBB6F89150BC9D16B, vehicle, extraId, disable) }
	pub fn DOES_EXTRA_EXIST(vehicle: Vehicle, extraId: i32) -> bool { invoke!(0xAF5E7E9A7620FFB5, vehicle, extraId) }
	pub fn SET_VEHICLE_DAMAGE(vehicle: Vehicle, xOffset: f32, yOffset: f32, zOffset: f32, damage: f32, radius: f32, p6: bool) { invoke_ignore!(0x1D7678F81452BB41, vehicle, xOffset, yOffset, zOffset, damage, radius, p6) }
	pub fn GET_VEHICLE_ENGINE_HEALTH(vehicle: Vehicle) -> f32 { invoke!(0x90DBFFAC43B22081, vehicle) }
	pub fn SET_VEHICLE_ENGINE_HEALTH(vehicle: Vehicle, health: f32) { invoke_ignore!(0x8BDC5B998B4654EF, vehicle, health) }
	pub fn GET_VEHICLE_PETROL_TANK_HEALTH(vehicle: Vehicle) -> f32 { invoke!(0x1E5A9B356D5098BE, vehicle) }
	pub fn SET_VEHICLE_PETROL_TANK_HEALTH(vehicle: Vehicle, health: f32) { invoke_ignore!(0x6AB2918EE3BEC94C, vehicle, health) }
	pub fn IS_VEHICLE_STUCK_TIMER_UP(vehicle: Vehicle, stuckType: i32, ms: i32) -> bool { invoke!(0x1ABA9753939503C5, vehicle, stuckType, ms) }
	pub fn RESET_VEHICLE_STUCK_TIMER(vehicle: Vehicle, nullAttributes: i32) { invoke_ignore!(0x23298B468F7D88B6, vehicle, nullAttributes) }
	pub fn IS_VEHICLE_DRIVEABLE(vehicle: Vehicle, p1: bool, p2: bool) -> bool { invoke!(0xB86D29B10F627379, vehicle, p1, p2) }
	pub fn IS_VEHICLE_WRECKED(vehicle: Vehicle) -> bool { invoke!(0xDDBEA5506C848227, vehicle) }
	pub fn _IS_VEHICLE_ON_FIRE(vehicle: Vehicle) -> bool { invoke!(0x0E3BF7ED4169EC43, vehicle) }
	pub fn SET_VEHICLE_HAS_BEEN_OWNED_BY_PLAYER(vehicle: Vehicle, owned: bool) { invoke_ignore!(0xBB5A3FA8ED3979C5, vehicle, owned) }
	pub fn START_VEHICLE_HORN(vehicle: Vehicle, duration: i32, mode: Hash, forever: bool) { invoke_ignore!(0xB4E3BFC39CA16057, vehicle, duration, mode, forever) }
	pub fn SET_VEHICLE_HAS_STRONG_AXLES(vehicle: Vehicle, toggle: bool) { invoke_ignore!(0x252253C8A45AA1FC, vehicle, toggle) }
	pub fn _SET_VEHICLE_SNOW_LEVEL(vehicle: Vehicle, snowLevel: f32) { invoke_ignore!(0x6F73EFAB11651D7F, vehicle, snowLevel) }
	pub fn _SET_VEHICLE_WET_LEVEL(vehicle: Vehicle, wetLevel: f32) { invoke_ignore!(0x5AABB09F6FBD1F87, vehicle, wetLevel) }
	pub fn _SET_VEHICLE_TINT(vehicle: Vehicle, tintId: i32) { invoke_ignore!(0x8268B098F6FCA4E2, vehicle, tintId) }
	pub fn _SET_VEHICLE_LIVERY(vehicle: Vehicle, liveryIndex: i32) { invoke_ignore!(0xF89D82A0582E46ED, vehicle, liveryIndex) }
	pub fn _GET_VEHICLE_TINT(vehicle: Vehicle) -> i32 { invoke!(0xA44D65E6C624526F, vehicle) }
	pub fn _GET_VEHICLE_LIVERY(vehicle: Vehicle) -> i32 { invoke!(0xBB765B8FD49A796C, vehicle) }
	pub fn IS_VEHICLE_WINDOW_INTACT(vehicle: Vehicle, windowIndex: i32) -> bool { invoke!(0x0E7910A63E05B12C, vehicle, windowIndex) }
	pub fn ARE_ANY_VEHICLE_SEATS_FREE(vehicle: Vehicle) -> bool { invoke!(0xA0A424505A1B6429, vehicle) }
	pub fn SET_VEHICLE_EXPLODES_ON_HIGH_EXPLOSION_DAMAGE(vehicle: Vehicle, toggle: bool) { invoke_ignore!(0xA402939C6761E1A3, vehicle, toggle) }
	pub fn SET_ALLOW_VEHICLE_EXPLODES_ON_CONTACT(vehicle: Vehicle, p1: bool) { invoke_ignore!(0x8D3230A0ED7DE39F, vehicle, p1) }
	pub fn IS_ANY_VEHICLE_NEAR_POINT(x: f32, y: f32, z: f32, radius: f32) -> bool { invoke!(0x5698BA4FD04D39C4, x, y, z, radius) }
	pub fn REQUEST_VEHICLE_HIGH_DETAIL_MODEL(vehicle: Vehicle) { invoke_ignore!(0x84B81EF78BD22357, vehicle) }
	pub fn REQUEST_VEHICLE_ASSET(vehicleHash: Hash, vehicleAsset: i32) { invoke_ignore!(0x81A15811460FAB3A, vehicleHash, vehicleAsset) }
	pub fn _0xCF9DA72002FC16BF(p0: Any, p1: Any, p2: Any) { invoke_ignore!(0xCF9DA72002FC16BF, p0, p1, p2) }
	pub fn HAS_VEHICLE_ASSET_LOADED(vehicleAsset: Hash) -> bool { invoke!(0xB935F3154BC913C8, vehicleAsset) }
	pub fn REMOVE_VEHICLE_ASSET(vehicleAsset: Hash) { invoke_ignore!(0x888A4E675B38F5AD, vehicleAsset) }
	pub fn SET_VEHICLE_AUTOMATICALLY_ATTACHES(vehicle: Vehicle, p1: bool, p2: Any) -> Any { invoke!(0x501354951CD942DE, vehicle, p1, p2) }
	pub fn _0x104D9A7B1C0D0783(vehicle: Vehicle, p1: f32) { invoke_ignore!(0x104D9A7B1C0D0783, vehicle, p1) }
	pub fn IS_VEHICLE_IN_BURNOUT(vehicle: Vehicle) -> bool { invoke!(0x3F5029A8FC060C48, vehicle) }
	pub fn SET_VEHICLE_HANDBRAKE(vehicle: Vehicle, toggle: bool) { invoke_ignore!(0x91BE51AEC4E99710, vehicle, toggle) }
	pub fn INSTANTLY_FILL_VEHICLE_POPULATION() { invoke_ignore!(0x1FF00DB43026B12F) }
	pub fn HAS_INSTANT_FILL_VEHICLE_POPULATION_FINISHED() -> bool { invoke!(0x2701D01D5E18FC31) }
	pub fn GET_VEHICLE_TRAILER_VEHICLE(vehicle: Vehicle, trailer: &mut Vehicle) -> bool { invoke!(0xCF867A239EC30741, vehicle, trailer) }
	pub fn GET_VEHICLE_ESTIMATED_MAX_SPEED(vehicle: Vehicle) -> f32 { invoke!(0xFE52F34491529F0B, vehicle) }
	pub fn ADD_ROAD_NODE_SPEED_ZONE(p0: Any, p1: Any, p2: Any, p3: Any, p4: Any, p5: Any, p6: Any, p7: Any, p8: Any, p9: Any, p10: Any) -> i32 { invoke!(0x4C221BAC54D735C3, p0, p1, p2, p3, p4, p5, p6, p7, p8, p9, p10) }
	pub fn REMOVE_ROAD_NODE_SPEED_ZONE(speedzone: i32) -> bool { invoke!(0xFE9AB3354ACE6C9C, speedzone) }
	pub fn IS_ENTRY_POINT_FOR_SEAT_CLEAR(ped: Ped, vehicle: Vehicle, seatIndex: i32, side: bool, onEnter: bool) -> bool { invoke!(0x80DDCCB2F4A3EB57, ped, vehicle, seatIndex, side, onEnter) }
	pub fn CAN_SHUFFLE_SEAT(vehicle: Vehicle, seatIndex: i32) -> bool { invoke!(0xF8B2D32A2231FD24, vehicle, seatIndex) }
	pub fn MODIFY_VEHICLE_TOP_SPEED(vehicle: Vehicle, value: f32) { invoke_ignore!(0x35AD938C74CACD6A, vehicle, value) }
	pub fn SET_VEHICLE_STAYS_FROZEN_WHEN_CLEANED_UP(vehicle: Vehicle, toggle: bool) { invoke_ignore!(0x23A3AB86E0807721, vehicle, toggle) }
	pub fn SET_VEHICLE_INFLUENCES_WANTED_LEVEL(vehicle: Vehicle, toggle: bool) { invoke_ignore!(0xC1842F40FD501DA2, vehicle, toggle) }
	pub fn _IS_BOAT_GROUNDED(vehicle: Vehicle) -> bool { invoke!(0x30D86B2B7622D0EB, vehicle) }
	pub fn SET_VEHICLE_NOT_STEALABLE_AMBIENTLY(vehicle: Vehicle, p1: bool) { invoke_ignore!(0x09C970AE59ABF6B2, vehicle, p1) }
	pub fn LOCK_DOORS_WHEN_NO_LONGER_NEEDED(vehicle: Vehicle) { invoke_ignore!(0x1EF36558FBDE2DAA, vehicle) }
	pub fn GET_LAST_DRIVEN_VEHICLE() -> Vehicle { invoke!(0xA94F3E0AB9695E19) }
	pub fn CLEAR_LAST_DRIVEN_VEHICLE() { invoke_ignore!(0x0EFC5DC62E67609B) }
	pub fn SET_PED_OWNS_VEHICLE(ped: Ped, vehicle: Vehicle) { invoke_ignore!(0x838C216C2B05A009, ped, vehicle) }
	pub fn _GET_VEHICLE_OWNER(vehicle: Vehicle) -> Entity { invoke!(0xB729679356A889AE, vehicle) }
	pub fn SET_VEHICLE_LOD_MULTIPLIER(vehicle: Vehicle, multiplier: f32) { invoke_ignore!(0x5F5E2B1B9EAECC0F, vehicle, multiplier) }
	pub fn _SET_VEHICLE_LOD_LEVEL(vehicle: Vehicle, lodLevel: i32) { invoke_ignore!(0x3FA7D7D1E0EA809E, vehicle, lodLevel) }
	pub fn SET_FORCE_VEHICLE_ENGINE_DAMAGE_BY_BULLET(vehicle: Vehicle, toggle: bool) { invoke_ignore!(0x7F8E2B131E1DCA6C, vehicle, toggle) }
	pub fn COPY_VEHICLE_DAMAGES(sourceVehicle: Vehicle, targetVehicle: Vehicle) { invoke_ignore!(0xDBC28A8C683CD80B, sourceVehicle, targetVehicle) }
	pub fn SET_VEHICLE_SHOOT_AT_TARGET(p0: Any, p1: Any, p2: Any, p3: Any, p4: Any, p5: Any) { invoke_ignore!(0xB79BE78C665B3E6D, p0, p1, p2, p3, p4, p5) }
	pub fn SET_FORCE_HD_VEHICLE(vehicle: Vehicle, toggle: bool) { invoke_ignore!(0x373CB1283308BD7B, vehicle, toggle) }
	pub fn TRACK_VEHICLE_VISIBILITY(vehicle: Vehicle) { invoke_ignore!(0x1F3969B140DEE157, vehicle) }
	pub fn IS_VEHICLE_VISIBLE(vehicle: Vehicle) -> bool { invoke!(0x424910CD5DE8C246, vehicle) }
	pub fn _0x13C190302369308B(p0: Any) -> Any { invoke!(0x13C190302369308B, p0) }
	pub fn SET_ENABLE_VEHICLE_SLIPSTREAMING(p0: bool) { invoke_ignore!(0x73F1E4F6DF26FE30, p0) }
	pub fn SET_VEHICLE_INACTIVE_DURING_PLAYBACK(vehicle: Vehicle, toggle: bool) { invoke_ignore!(0x4EA71B4C9DB3C3F1, vehicle, toggle) }
	pub fn SET_VEHICLE_ENGINE_CAN_DEGRADE(vehicle: Vehicle, toggle: bool) { invoke_ignore!(0x48E4C137A71C2688, vehicle, toggle) }
	pub fn SET_VEHICLE_IS_STOLEN(vehicle: Vehicle, isStolen: bool) { invoke_ignore!(0x6C32FC81DFF25C9A, vehicle, isStolen) }
	pub fn _0xCBF88256E44D5D39(vehicle: Vehicle, p1: bool) { invoke_ignore!(0xCBF88256E44D5D39, vehicle, p1) }
	pub fn _0xC325A6BAA62CF8A2(vehicle: Vehicle, p1: bool) { invoke_ignore!(0xC325A6BAA62CF8A2, vehicle, p1) }
	pub fn _0x0CD7914D17A970AB(p0: Any, p1: Any) { invoke_ignore!(0x0CD7914D17A970AB, p0, p1) }
	pub fn _0x23F66C36F8E5EAAB(p0: Any, p1: Any) { invoke_ignore!(0x23F66C36F8E5EAAB, p0, p1) }
	pub fn _0x697DF68F3A761A50(p0: Any) { invoke_ignore!(0x697DF68F3A761A50, p0) }
	pub fn _0x27E3F2B57209FA54(p0: Any, p1: Any) { invoke_ignore!(0x27E3F2B57209FA54, p0, p1) }
	pub fn DISABLE_VEHICLE_WEAPON(disabled: bool, weaponHash: Hash, vehicle: Vehicle, owner: Ped) { invoke_ignore!(0x94B1E71B144356A5, disabled, weaponHash, vehicle, owner) }
	pub fn SET_VEHICLE_CAN_BE_USED_BY_FLEEING_PEDS(vehicle: Vehicle, toggle: bool) { invoke_ignore!(0xE42952510F84AFDB, vehicle, toggle) }
	pub fn SET_VEHICLE_KEEP_ENGINE_ON_WHEN_ABANDONED(vehicle: Vehicle, toggle: bool) { invoke_ignore!(0x1549BA7FE83A2383, vehicle, toggle) }
	pub fn SET_VEHICLE_WHEELS_CAN_BREAK_OFF_WHEN_BLOW_UP(vehicle: Vehicle, toggle: bool) { invoke_ignore!(0xC462C79379ABBCB1, vehicle, toggle) }
	pub fn _0x15CC8C33D7FFCC4A(vehicle: Vehicle, p1: i32) { invoke_ignore!(0x15CC8C33D7FFCC4A, vehicle, p1) }
	pub fn SET_VEHICLE_AI_CAN_USE_EXCLUSIVE_SEATS(vehicle: Vehicle, toggle: bool) { invoke_ignore!(0x0893DAFBFA67110E, vehicle, toggle) }
	pub fn SET_VEHICLE_EXCLUSIVE_DRIVER(vehicle: Vehicle, ped: Ped, index: i32) { invoke_ignore!(0xC6B9BF123B9463B6, vehicle, ped, index) }
	pub fn _IS_PED_EXCLUSIVE_DRIVER_OF_VEHICLE(ped: Ped, vehicle: Vehicle, outIndex: &mut i32) -> bool { invoke!(0xB213D2A560B2E48B, ped, vehicle, outIndex) }
	pub fn _0xDC0556D0F484ECAA(p0: Any) { invoke_ignore!(0xDC0556D0F484ECAA, p0) }
	pub fn SET_DISABLE_SUPERDUMMY(vehicle: Vehicle, disable: bool) { invoke_ignore!(0x1716D787D9B94202, vehicle, disable) }
	pub fn GET_VEHICLE_BODY_HEALTH(vehicle: Vehicle) -> f32 { invoke!(0x42113B857E33C16E, vehicle) }
	pub fn SET_VEHICLE_BODY_HEALTH(vehicle: Vehicle, value: f32) { invoke_ignore!(0x55CCAAE4F28C67A0, vehicle, value) }
	pub fn _0xE777DDF3E78397E8(p0: Any) -> Any { invoke!(0xE777DDF3E78397E8, p0) }
	pub fn SET_VEHICLE_BROKEN_PARTS_DONT_AFFECT_AI_HANDLING(vehicle: Vehicle, p1: bool) { invoke_ignore!(0xCEC4CA2CAB8FA98C, vehicle, p1) }
	pub fn _0x012701ED938B85DE(p0: f32, p1: f32) { invoke_ignore!(0x012701ED938B85DE, p0, p1) }
	pub fn _0x8379E05871AD24E0() { invoke_ignore!(0x8379E05871AD24E0) }
	pub fn _SET_HORSE_TRAFFIC_GROUPING_DISTRIBUTION(p0: Any, p1: Any, p2: Any, p3: Any) { invoke_ignore!(0xF5FFB08976911B50, p0, p1, p2, p3) }
	pub fn _ATTACH_DRAFT_VEHICLE_HARNESS_PED(mount: Ped, draft: Vehicle, harnessId: i32) -> bool { invoke!(0x316CDB5B6E8F4110, mount, draft, harnessId) }
	pub fn _DETACH_DRAFT_VEHICLE_HARNESS_FROM_INDEX(draft: Vehicle, harnessId: i32) -> bool { invoke!(0x4402960666000E62, draft, harnessId) }
	pub fn _DETACH_DRAFT_VEHICLE_HARNESS_PED(draft: Vehicle, ped: Ped) -> bool { invoke!(0xB36D3EC70963BE60, draft, ped) }
	pub fn _0x0F7F603BDE08C4D3(p0: Any) { invoke_ignore!(0x0F7F603BDE08C4D3, p0) }
	pub fn _GET_NUM_DRAFT_VEHICLE_HARNESS_PED(modelHash: Hash) -> i32 { invoke!(0x5B1A26BB18E7D451, modelHash) }
	pub fn _GET_CHECKPOINT_TRAIN_SPAWN_LOCATION(trackIndex: i32, x: f32, y: f32, z: f32, distance: f32, direction: bool) -> Vector3 { invoke!(0x35D302397E524939, trackIndex, x, y, z, distance, direction) }
	pub fn _0xC399CC89FBA05DA0(vehicle: Vehicle, p1: bool) { invoke_ignore!(0xC399CC89FBA05DA0, vehicle, p1) }
	pub fn _GET_ROWING_OARS(vehicle: Vehicle, left: &mut Entity, right: &mut Entity) { invoke_ignore!(0xA6E210FB4283B767, vehicle, left, right) }
	pub fn GET_DRIVER_OF_VEHICLE(vehicle: Vehicle) -> Ped { invoke!(0x2963B5C1637E8A27, vehicle) }
	pub fn _SET_FORCE_COACH_ROBBERY_LOOT(vehicle: Vehicle, coachrobberyLoot: Hash) { invoke_ignore!(0xF489F94BFEE12BB0, vehicle, coachrobberyLoot) }
	pub fn _0x0BA4250D20007C2E(p0: Any) -> Any { invoke!(0x0BA4250D20007C2E, p0) }
	pub fn _0x2200AB13CBD10F4E(vehicle: Vehicle, x: f32, y: f32, z: f32, p4: bool, p5: f32) { invoke_ignore!(0x2200AB13CBD10F4E, vehicle, x, y, z, p4, p5) }
	pub fn _0xB42C87521D1BDD2F(vehicle: Vehicle, x: f32, y: f32, z: f32) { invoke_ignore!(0xB42C87521D1BDD2F, vehicle, x, y, z) }
	pub fn _0xC351394B932A6A50(p0: Any) { invoke_ignore!(0xC351394B932A6A50, p0) }
	pub fn _0x172E9DD35858DCD7(p0: Any) { invoke_ignore!(0x172E9DD35858DCD7, p0) }
	pub fn _GET_BREAKABLE_VEHICLE_LOCKS_STATE(vehicle: Vehicle) -> i32 { invoke!(0xE015CF1F2C0959D8, vehicle) }
	pub fn _0x877EA24EB1614495(p0: Any, p1: Any, p2: Any) -> Any { invoke!(0x877EA24EB1614495, p0, p1, p2) }
	pub fn _GET_BREAKABLE_VEHICLE_LOCK_OBJECT(vehicle: Vehicle, index: i32) -> Object { invoke!(0x58F2244C1286D09A, vehicle, index) }
	pub fn _GET_NUM_BREAKABLE_VEHICLE_LOCK_OBJECTS(vehicle: Vehicle) -> i32 { invoke!(0x2FA86833E3617E2D, vehicle) }
	pub fn SET_BREAKABLE_VEHICLE_LOCKS_UNBREAKABLE(vehicle: Vehicle, toggle: bool) { invoke_ignore!(0xBC4735F48CD983EF, vehicle, toggle) }
	pub fn _0x9D12796EF4BF9EA9(p0: Any) { invoke_ignore!(0x9D12796EF4BF9EA9, p0) }
	pub fn _0x850CE59DEC2028F3(vehicle: Vehicle, p1: Any) { invoke_ignore!(0x850CE59DEC2028F3, vehicle, p1) }
	pub fn _0x0355FE37240E2C77(p0: Any, p1: Any) { invoke_ignore!(0x0355FE37240E2C77, p0, p1) }
	pub fn _0x3D86997A86FEEF0D(p0: Any, p1: Any) { invoke_ignore!(0x3D86997A86FEEF0D, p0, p1) }
	pub fn _0xD826690B5CF3BEFF(vehicle: Vehicle, p1: Any) { invoke_ignore!(0xD826690B5CF3BEFF, vehicle, p1) }
	pub fn _BREAK_OFF_DRAFT_WHEEL(vehicle: Vehicle, wheelIndex: i32, destroyingForce: f32) { invoke_ignore!(0xC372B6A88F6E4AD8, vehicle, wheelIndex, destroyingForce) }
	pub fn GET_DRAFT_ANIMAL_COUNT(vehicle: Vehicle, expected: &mut i32, actual: &mut i32) -> bool { invoke!(0xA19447D83294E29F, vehicle, expected, actual) }
	pub fn _0x165BE2001E5E4B75(p0: Any) { invoke_ignore!(0x165BE2001E5E4B75, p0) }
	pub fn _SET_DRAFT_VEHICLE_ANIMALS_CAN_DETACH(draft: Vehicle, canDetach: bool) { invoke_ignore!(0x6090A031C69F384E, draft, canDetach) }
	pub fn _SET_DRAFT_VEHICLE_YOKE_CAN_BREAK(draft: Vehicle, canBreak: bool) { invoke_ignore!(0x226C6A4E3346D288, draft, canBreak) }
	pub fn _ADD_TRAIN_TEMPORARY_STOP(train: Vehicle, trackIndex: i32, x: f32, y: f32, z: f32) { invoke_ignore!(0x41503629D1139ABC, train, trackIndex, x, y, z) }
	pub fn _0x0794199B25E499E1(wagon: Vehicle, p1: bool) { invoke_ignore!(0x0794199B25E499E1, wagon, p1) }
	pub fn _0x73118A3EE9C9B6DB(wagon: Vehicle, p1: i32, p2: bool) { invoke_ignore!(0x73118A3EE9C9B6DB, wagon, p1, p2) }
	pub fn _0xE1C0F8781BF130C2(wagon: Vehicle, p1: i32) -> bool { invoke!(0xE1C0F8781BF130C2, wagon, p1) }
	pub fn _IS_VEHICLE_WHEEL_DESTROYED(vehicle: Vehicle, wheel: i32) -> bool { invoke!(0xCB2CA620C48BC875, vehicle, wheel) }
	pub fn _0x18714953CCED17D3(vehicle: Vehicle) -> bool { invoke!(0x18714953CCED17D3, vehicle) }
	pub fn _0x41F0B254DDF71473(wagon: Vehicle) { invoke_ignore!(0x41F0B254DDF71473, wagon) }
	pub fn _SET_VEHICLE_DETERIORATION(vehicle: Vehicle, amount: f32, p2: i32, p3: bool) { invoke_ignore!(0x8E5DA070BAD3279E, vehicle, amount, p2, p3) }
	pub fn _IS_VEHICLE_DOOR_BROKEN(vehicle: Vehicle, doorId: i32) -> bool { invoke!(0xE979BB5602AD3402, vehicle, doorId) }
	pub fn _BREAK_OFF_VEHICLE_WHEEL(vehicle: Vehicle, wheelIndex: i32) -> Entity { invoke!(0xD4F5EFB55769D272, vehicle, wheelIndex) }
	pub fn _DELETE_VEHICLE_LANTERNS(vehicle: Vehicle) -> bool { invoke!(0xE1A83D4A3B5D7938, vehicle) }
	pub fn _0x6DE072AC8A95FFC1(vehicle: Vehicle, p1: bool) { invoke_ignore!(0x6DE072AC8A95FFC1, vehicle, p1) }
	pub fn _SET_DRAFT_VEHICLE_DESIRED_SPEED(vehicle: Vehicle, speed: f32) { invoke_ignore!(0x0C3F0F7F92CA847C, vehicle, speed) }
	pub fn _GET_DRAFT_VEHICLE_DESIRED_SPEED(vehicle: Vehicle) -> f32 { invoke!(0xC6D7DDC843176701, vehicle) }
	pub fn _0xC4A2C11FC0D41916(vehicle: Vehicle, p1: bool) { invoke_ignore!(0xC4A2C11FC0D41916, vehicle, p1) }
	pub fn _0xFC4F15A7DDDC47B1(vehicle: Vehicle, p1: bool) { invoke_ignore!(0xFC4F15A7DDDC47B1, vehicle, p1) }
	pub fn _0x4C60C333F9CCA2B6(vehicle: Vehicle, p1: bool) { invoke_ignore!(0x4C60C333F9CCA2B6, vehicle, p1) }
	pub fn _0xCF342503CA4C8DF1(vehicle: Vehicle, p1: f32) { invoke_ignore!(0xCF342503CA4C8DF1, vehicle, p1) }
	pub fn _0x06A09A6E0C6D2A84(train: Vehicle, p1: bool) { invoke_ignore!(0x06A09A6E0C6D2A84, train, p1) }
	pub fn _0xAE7E66A61E7C17A5(train: Vehicle, p1: bool) { invoke_ignore!(0xAE7E66A61E7C17A5, train, p1) }
	pub fn _0xEF28A614B4B264B8(train: Vehicle, p1: bool) { invoke_ignore!(0xEF28A614B4B264B8, train, p1) }
	pub fn _0x04F0579DBDD32F34(vehicle: Vehicle) { invoke_ignore!(0x04F0579DBDD32F34, vehicle) }
	pub fn _0x12F6C6ED3EFF42DE(vehicle: Vehicle, x: f32, y: f32, z: f32) { invoke_ignore!(0x12F6C6ED3EFF42DE, vehicle, x, y, z) }
	pub fn _0x87B974E54C71BA7B(vehicle: Vehicle, p1: bool) { invoke_ignore!(0x87B974E54C71BA7B, vehicle, p1) }
	pub fn _HAS_TRAIN_LOADED(train: Vehicle) -> bool { invoke!(0xBD3C4A2ED509205E, train) }
	pub fn _CREATE_MISSION_TRAIN(configHash: Hash, x: f32, y: f32, z: f32, direction: bool, passengers: bool, p6: bool, conductor: bool) -> Vehicle { invoke!(0xC239DBD9A57D2A71, configHash, x, y, z, direction, passengers, p6, conductor) }
	pub fn _0xD1EFA8D68BF5D63D(p0: Any, p1: Any, p2: Any, p3: Any) { invoke_ignore!(0xD1EFA8D68BF5D63D, p0, p1, p2, p3) }
	pub fn _0x1121B07088ED3013(p0: Any) -> Any { invoke!(0x1121B07088ED3013, p0) }
	pub fn _0x42404D57D621601A(p0: Any) -> Any { invoke!(0x42404D57D621601A, p0) }
	pub fn _0x288CBB414C3C2FBB(p0: Any) -> Any { invoke!(0x288CBB414C3C2FBB, p0) }
	pub fn _0x6FD7BDF10304363A(p0: Any, p1: Any) { invoke_ignore!(0x6FD7BDF10304363A, p0, p1) }
	pub fn _0xCEB1F1EED484A5B4(p0: Any, p1: Any) { invoke_ignore!(0xCEB1F1EED484A5B4, p0, p1) }
	pub fn _0xF57DB8E83DCD8349(p0: Any) -> Any { invoke!(0xF57DB8E83DCD8349, p0) }
	pub fn _SET_BALLOON_HOVER_STATE(balloon: Vehicle, p1: f32) { invoke_ignore!(0x7C9E45A4CED2E8DA, balloon, p1) }
	pub fn _SET_DRAFT_VEHICLE_ALLOW_DRAFT_ANIMAL_AUTO_CREATION(vehicle: Vehicle, allow: bool) { invoke_ignore!(0x87344305778E5415, vehicle, allow) }
	pub fn _0x6835AFEA10E186F4(p0: Any, p1: Any) { invoke_ignore!(0x6835AFEA10E186F4, p0, p1) }
	pub fn _SET_DRAFT_ANIMAL_RANDOM_SEED(vehicle: Vehicle, seed: i32) { invoke_ignore!(0x8C6D9A399126C194, vehicle, seed) }
	pub fn _0x14DA8C4BC2CCD90A(p0: Any) -> Any { invoke!(0x14DA8C4BC2CCD90A, p0) }
	pub fn _0xCACAB2B123BBDBD6(p0: Any, p1: Any, p2: Any) -> Any { invoke!(0xCACAB2B123BBDBD6, p0, p1, p2) }
	pub fn _0xFF2B1F59FB892F14(p0: Any) { invoke_ignore!(0xFF2B1F59FB892F14, p0) }
	pub fn _0x5AADC7BBBB1BCEEB(p0: Any, p1: Any, p2: Any, p3: Any, p4: Any) { invoke_ignore!(0x5AADC7BBBB1BCEEB, p0, p1, p2, p3, p4) }
	pub fn _GET_VEHICLE_IS_PROP_SET_APPLIED(vehicle: Vehicle) -> bool { invoke!(0xD798DF5DB67B1659, vehicle) }
	pub fn _SET_BATCH_TARP_HEIGHT(vehicle: Vehicle, height: f32, immediately: bool) { invoke_ignore!(0x31F343383F19C987, vehicle, height, immediately) }
	pub fn _0x07E2E21E799080A0(p0: Any, p1: Any) { invoke_ignore!(0x07E2E21E799080A0, p0, p1) }
	pub fn _0xC2E62678D602853C(p0: Any, p1: Any, p2: Any, p3: Any, p4: Any, p5: Any, p6: Any) { invoke_ignore!(0xC2E62678D602853C, p0, p1, p2, p3, p4, p5, p6) }
	pub fn _0x873AAF600CC36DAC(p0: Any) { invoke_ignore!(0x873AAF600CC36DAC, p0) }
	pub fn _0x51C7694E140FAE43(p0: Any) -> Any { invoke!(0x51C7694E140FAE43, p0) }
	pub fn _0x8DECD262602548B9(p0: Any, p1: Any) { invoke_ignore!(0x8DECD262602548B9, p0, p1) }
	pub fn _0xCBC7B6F9A56B79F6(p0: Any, p1: Any) { invoke_ignore!(0xCBC7B6F9A56B79F6, p0, p1) }
	pub fn _0x37D238BE69F7378A(trackIndex: i32) -> bool { invoke!(0x37D238BE69F7378A, trackIndex) }
	pub fn _0x703D4FB366DA4452(p0: Any, p1: Any) { invoke_ignore!(0x703D4FB366DA4452, p0, p1) }
	pub fn _0x762FDC4C19E5A981(trainCarriage: Entity, p1: bool) { invoke_ignore!(0x762FDC4C19E5A981, trainCarriage, p1) }
	pub fn _0x2045429505158D1A(p0: Any) -> Any { invoke!(0x2045429505158D1A, p0) }
	pub fn _0x13EB275BF81636D1(p0: Any, p1: Any) { invoke_ignore!(0x13EB275BF81636D1, p0, p1) }
}
pub mod VOICE {
	use std::ffi::CStr;
	use crate::core::scripthook::native::*;
	use crate::core::types::*;

	pub fn _0xCCF71FCFA0070B1A() -> bool { invoke!(0xCCF71FCFA0070B1A) }
	pub fn _0x79F478FF5F9F4F05(enabled: bool) { invoke_ignore!(0x79F478FF5F9F4F05, enabled) }
	pub fn _0xAA35FD9ABAB490A3(player: Player) -> bool { invoke!(0xAA35FD9ABAB490A3, player) }
	pub fn _0x356135B9B10A2A82(gamerHandle: &mut Any) -> bool { invoke!(0x356135B9B10A2A82, gamerHandle) }
	pub fn _0xEF6F2A35FAAF2ED7(player: Player) -> bool { invoke!(0xEF6F2A35FAAF2ED7, player) }
	pub fn _0x49623BCFC3A3D829(player: Player, muted: bool) -> bool { invoke!(0x49623BCFC3A3D829, player, muted) }
	pub fn _0x919AF2D93E9AA89D(player: Player) -> bool { invoke!(0x919AF2D93E9AA89D, player) }
	pub fn _0x0DED260A1958A82E(player: Player) -> bool { invoke!(0x0DED260A1958A82E, player) }
	pub fn _0x8E462DB1EAA9C47C(player: Player) -> bool { invoke!(0x8E462DB1EAA9C47C, player) }
	pub fn _0x5CA7FB7D6DE49DCC(player: Player) -> f32 { invoke!(0x5CA7FB7D6DE49DCC, player) }
	pub fn _0x58125B691F6827D5(proximity: f32) { invoke_ignore!(0x58125B691F6827D5, proximity) }
	pub fn _0x2F82CAB262C8AE26(player: Player) -> f32 { invoke!(0x2F82CAB262C8AE26, player) }
	pub fn _0x08797A8C03868CB8(threshold: f32) { invoke_ignore!(0x08797A8C03868CB8, threshold) }
	pub fn _0xB779F4FA19269AEC(flag: bool) { invoke_ignore!(0xB779F4FA19269AEC, flag) }
	pub fn _0x1FBF7F5BA7E4BE3A(p0: i32) { invoke_ignore!(0x1FBF7F5BA7E4BE3A, p0) }
	pub fn _0xDC9B361CB7776673(player: Player) { invoke_ignore!(0xDC9B361CB7776673, player) }
	pub fn _0xEC8703E4536A9952() { invoke_ignore!(0xEC8703E4536A9952) }
	pub fn _0xDB622ECD3DCBE078(player: Player) -> Any { invoke!(0xDB622ECD3DCBE078, player) }
	pub fn _0xB6E79850B759A30E(teamId: i32, allow: bool) { invoke_ignore!(0xB6E79850B759A30E, teamId, allow) }
	pub fn _0x4791899615D70FA2(player: Player, p1: i32, p2: i32) { invoke_ignore!(0x4791899615D70FA2, player, p1, p2) }
	pub fn _0xF8938CF3984092A5(player: Player) { invoke_ignore!(0xF8938CF3984092A5, player) }
	pub fn _0x767931C727DF2ED7(player: Player, p1: i32) -> i32 { invoke!(0x767931C727DF2ED7, player, p1) }
	pub fn _0x1C38C3577901AF1F() { invoke_ignore!(0x1C38C3577901AF1F) }
	pub fn _0xB3E8841F6BDAF83E() { invoke_ignore!(0xB3E8841F6BDAF83E) }
}
pub mod VOLUME {
	use std::ffi::CStr;
	use crate::core::scripthook::native::*;
	use crate::core::types::*;

	pub fn _CREATE_VOLUME_BY_HASH(volumeType: Hash, x: f32, y: f32, z: f32, rotX: f32, rotY: f32, rotZ: f32, scaleX: f32, scaleY: f32, scaleZ: f32) -> Volume { invoke!(0x502022FA1AF9DC86, volumeType, x, y, z, rotX, rotY, rotZ, scaleX, scaleY, scaleZ) }
	pub fn CREATE_VOLUME_BOX(x: f32, y: f32, z: f32, rotX: f32, rotY: f32, rotZ: f32, scaleX: f32, scaleY: f32, scaleZ: f32) -> Volume { invoke!(0xDF85637F22706891, x, y, z, rotX, rotY, rotZ, scaleX, scaleY, scaleZ) }
	pub fn CREATE_VOLUME_CYLINDER(x: f32, y: f32, z: f32, rotX: f32, rotY: f32, rotZ: f32, scaleX: f32, scaleY: f32, scaleZ: f32) -> Volume { invoke!(0x0522D4774B82E3E6, x, y, z, rotX, rotY, rotZ, scaleX, scaleY, scaleZ) }
	pub fn CREATE_VOLUME_SPHERE(x: f32, y: f32, z: f32, rotX: f32, rotY: f32, rotZ: f32, scaleX: f32, scaleY: f32, scaleZ: f32) -> Volume { invoke!(0xB3FB80A32BAE3065, x, y, z, rotX, rotY, rotZ, scaleX, scaleY, scaleZ) }
	pub fn CREATE_VOLUME_AGGREGATE() -> Volume { invoke!(0x59F6F5C1D129F106) }
	pub fn _CREATE_VOLUME_BY_HASH_WITH_CUSTOM_NAME(volumeType: Hash, x: f32, y: f32, z: f32, rotX: f32, rotY: f32, rotZ: f32, scaleX: f32, scaleY: f32, scaleZ: f32, name: & CStr) -> Volume { invoke!(0x1F85E4AC774A201E, volumeType, x, y, z, rotX, rotY, rotZ, scaleX, scaleY, scaleZ, name) }
	pub fn _CREATE_ANTI_GRIEF_VOLUME(volumeType: Hash, x: f32, y: f32, z: f32, rotX: f32, rotY: f32, rotZ: f32, scaleX: f32, scaleY: f32, scaleZ: f32) -> Volume { invoke!(0x0EB78C2B156635B1, volumeType, x, y, z, rotX, rotY, rotZ, scaleX, scaleY, scaleZ) }
	pub fn _SET_ANTI_GRIEF_VOLUME_BLOCKS_HORSE(volume: Volume, toggle: bool) { invoke_ignore!(0xBE551C2CC421185D, volume, toggle) }
	pub fn _SET_ANTI_GRIEF_VOLUME_BLOCKS_PLAYER(volume: Volume, toggle: bool) { invoke_ignore!(0x5B23DFF8E0948BB2, volume, toggle) }
	pub fn _CREATE_WALK_AND_TALK_VOLUME(p0: Any, p1: Any, p2: Any, p3: Any, p4: Any, p5: Any, p6: Any, p7: Any, p8: Any, p9: Any, p10: Any, p11: Any, p12: Any) -> Volume { invoke!(0xFD0E389CD44434B6, p0, p1, p2, p3, p4, p5, p6, p7, p8, p9, p10, p11, p12) }
	pub fn _CREATE_SPEED_VOLUME(p0: Any, p1: Any, p2: Any, p3: Any, p4: Any, p5: Any, p6: Any, p7: Any, p8: Any, p9: Any, p10: Any, p11: Any, p12: Any, p13: Any, p14: Any) -> Volume { invoke!(0xBBE768E3AE76E07C, p0, p1, p2, p3, p4, p5, p6, p7, p8, p9, p10, p11, p12, p13, p14) }
	pub fn _CREATE_VOLUME_BOX_WITH_CUSTOM_NAME(x: f32, y: f32, z: f32, rotX: f32, rotY: f32, rotZ: f32, scaleX: f32, scaleY: f32, scaleZ: f32, name: & CStr) -> Volume { invoke!(0xF68485C7495D848E, x, y, z, rotX, rotY, rotZ, scaleX, scaleY, scaleZ, name) }
	pub fn _CREATE_VOLUME_CYLINDER_WITH_CUSTOM_NAME(x: f32, y: f32, z: f32, rotX: f32, rotY: f32, rotZ: f32, scaleX: f32, scaleY: f32, scaleZ: f32, name: & CStr) -> Volume { invoke!(0xDF1E350EDDF06E59, x, y, z, rotX, rotY, rotZ, scaleX, scaleY, scaleZ, name) }
	pub fn _CREATE_VOLUME_SPHERE_WITH_CUSTOM_NAME(x: f32, y: f32, z: f32, rotX: f32, rotY: f32, rotZ: f32, scaleX: f32, scaleY: f32, scaleZ: f32, name: & CStr) -> Volume { invoke!(0x10157BC3247FF3BA, x, y, z, rotX, rotY, rotZ, scaleX, scaleY, scaleZ, name) }
	pub fn _CREATE_VOLUME_AGGREGATE_WITH_CUSTOM_NAME(name: & CStr) -> Volume { invoke!(0x5D580DE6398BB162, name) }
	pub fn _ADD_BOUNDS_TO_AGGREGATE_VOLUME(volume: Volume, aggregate: Volume) { invoke_ignore!(0x6E0D3C3F828DA773, volume, aggregate) }
	pub fn _REMOVE_BOUNDS_FROM_AGGREGATE_VOLUME(volume: Volume, aggregate: Volume) { invoke_ignore!(0xF92FA8890DECECF6, volume, aggregate) }
	pub fn _ADD_VOLUME_TO_VOLUME_AGGREGATE(aggregate: Volume, typeHash: Hash, x: f32, y: f32, z: f32, rotX: f32, rotY: f32, rotZ: f32, scaleX: f32, scaleY: f32, scaleZ: f32) { invoke_ignore!(0x12FCAA23F2320422, aggregate, typeHash, x, y, z, rotX, rotY, rotZ, scaleX, scaleY, scaleZ) }
	pub fn _ADD_BOX_VOLUME_TO_VOLUME_AGGREGATE(aggregate: Volume, p1: f32, p2: f32, p3: f32, p4: f32, p5: f32, p6: f32, p7: f32, p8: f32, p9: f32) { invoke_ignore!(0x39816F6F94F385AD, aggregate, p1, p2, p3, p4, p5, p6, p7, p8, p9) }
	pub fn _ADD_CYLINDER_VOLUME_TO_VOLUME_AGGREGATE(aggregate: Volume, p1: f32, p2: f32, p3: f32, p4: f32, p5: f32, p6: f32, p7: f32, p8: f32, p9: f32) { invoke_ignore!(0xBCE668AAF83608BE, aggregate, p1, p2, p3, p4, p5, p6, p7, p8, p9) }
	pub fn _ADD_SPHERE_VOLUME_TO_VOLUME_AGGREGATE(aggregate: Volume, p1: f32, p2: f32, p3: f32, p4: f32, p5: f32, p6: f32, p7: f32, p8: f32, p9: f32) { invoke_ignore!(0x5B7D7BF36D2DE18B, aggregate, p1, p2, p3, p4, p5, p6, p7, p8, p9) }
	pub fn DELETE_VOLUME(volume: Volume) { invoke_ignore!(0x43F867EF5C463A53, volume) }
	pub fn DOES_VOLUME_EXIST(volume: Volume) -> bool { invoke!(0x92A78D0BEDB332A3, volume) }
	pub fn IS_POINT_IN_VOLUME(volume: Volume, x: f32, y: f32, z: f32) -> bool { invoke!(0xF256A75210C5C0EB, volume, x, y, z) }
	pub fn GET_VOLUME_COORDS(volume: Volume) -> Vector3 { invoke!(0xF70F00013A62F866, volume) }
	pub fn SET_VOLUME_COORDS(volume: Volume, posX: f32, posY: f32, posZ: f32) -> bool { invoke!(0x541B8576615C33DE, volume, posX, posY, posZ) }
	pub fn GET_VOLUME_ROTATION(volume: Volume) -> Vector3 { invoke!(0x18675BC914891122, volume) }
	pub fn SET_VOLUME_ROTATION(volume: Volume, rotX: f32, rotY: f32, rotZ: f32) -> bool { invoke!(0xA07CF1B21B56F041, volume, rotX, rotY, rotZ) }
	pub fn GET_VOLUME_SCALE(volume: Volume) -> Vector3 { invoke!(0x3E2A25B2416DD67E, volume) }
	pub fn SET_VOLUME_SCALE(volume: Volume, scaleX: f32, scaleY: f32, scaleZ: f32) -> bool { invoke!(0xA46E98BDC407E23D, volume, scaleX, scaleY, scaleZ) }
	pub fn _GET_VOLUME_BOUNDS(volume: Volume, min: &mut Vector3, max: &mut Vector3) { invoke_ignore!(0x5737199AF2DC609F, volume, min, max) }
	pub fn _0x748C5F51A18CB8F0(p0: bool) { invoke_ignore!(0x748C5F51A18CB8F0, p0) }
	pub fn _0x2B32B11520626229(p0: Any, p1: Any, p2: Any, p3: Any, p4: Any) -> Any { invoke!(0x2B32B11520626229, p0, p1, p2, p3, p4) }
	pub fn _0x40F769D31A00D5A0(p0: Any, p1: Any) -> Any { invoke!(0x40F769D31A00D5A0, p0, p1) }
	pub fn _0xD882C5B3991575B7(p0: Any, p1: Any, p2: Any, p3: Any, p4: Any) -> Vector3 { invoke!(0xD882C5B3991575B7, p0, p1, p2, p3, p4) }
	pub fn _0xB469CFD9E065EB99(p0: Any, p1: Any) { invoke_ignore!(0xB469CFD9E065EB99, p0, p1) }
	pub fn _0x7FD78DFD0C5D7B9B(p0: Any) -> Any { invoke!(0x7FD78DFD0C5D7B9B, p0) }
	pub fn _0xEE1D6FF54CAF7714(p0: Any, p1: Any) -> Any { invoke!(0xEE1D6FF54CAF7714, p0, p1) }
	pub fn _0xD52DF30355EA7C8E(p0: Any, p1: Any, p2: Any) { invoke_ignore!(0xD52DF30355EA7C8E, p0, p1, p2) }
	pub fn SET_VOLUME_OWNER_PERSISTENT_CHARACTER(volume: Volume, persChar: PersChar, p2: bool) { invoke_ignore!(0xE2BE6FFA4A13CBB0, volume, persChar, p2) }
	pub fn _0x6D5F9E69BA1BE783(p0: Any) { invoke_ignore!(0x6D5F9E69BA1BE783, p0) }
	pub fn _0x998202B206872672(p0: Any) { invoke_ignore!(0x998202B206872672, p0) }
	pub fn _0x4A8FEFC43FD8AC9B(p0: Any, p1: Any, p2: Any) { invoke_ignore!(0x4A8FEFC43FD8AC9B, p0, p1, p2) }
	pub fn _0xF3A2FBA5985C8CD5(p0: Any, p1: Any, p2: Any, p3: Any) { invoke_ignore!(0xF3A2FBA5985C8CD5, p0, p1, p2, p3) }
	pub fn _0x53D05D60E5F5B40C(p0: Any, p1: Any, p2: Any, p3: Any) { invoke_ignore!(0x53D05D60E5F5B40C, p0, p1, p2, p3) }
	pub fn _0xCA5C90D40665D5CE(p0: Any, p1: Any) -> Any { invoke!(0xCA5C90D40665D5CE, p0, p1) }
	pub fn _0x3EFABB21E14A6BD1(p0: Any, p1: Any, p2: Any) { invoke_ignore!(0x3EFABB21E14A6BD1, p0, p1, p2) }
	pub fn _IS_AGGREGATE_VOLUME(volume: Volume) -> bool { invoke!(0xFEFF01B5725BCD22, volume) }
	pub fn _CREATE_VOLUME_LOCK(x: f32, y: f32, z: f32, radius: f32, flag: i32, p5: Any) -> Volume { invoke!(0x00BBF7CEAE8C666A, x, y, z, radius, flag, p5) }
	pub fn _CREATE_VOLUME_LOCK_ATTACHED_TO_ENTITY(entity: Entity, radius: f32, flag: i32, p3: Any) -> Volume { invoke!(0xF383E96C4904DF0C, entity, radius, flag, p3) }
	pub fn _IS_VOLUME_LOCK_REQUEST_VALID_2(volLockRequestId: i32) -> bool { invoke!(0xF6A8A652A6B186CD, volLockRequestId) }
	pub fn _0xC4019CF9AE8E931A(volLockRequestId: i32) -> Vector3 { invoke!(0xC4019CF9AE8E931A, volLockRequestId) }
	pub fn _0xF6CE6F9C3897804E(p0: Any) -> Any { invoke!(0xF6CE6F9C3897804E, p0) }
	pub fn _0xF6F5447D418DAA82(p0: Any) -> Any { invoke!(0xF6F5447D418DAA82, p0) }
	pub fn _0xD4FA73FE628FEC63(p0: Any, p1: Any) { invoke_ignore!(0xD4FA73FE628FEC63, p0, p1) }
	pub fn _0xB440F4E35393FC39(volume: Volume, p1: Any) { invoke_ignore!(0xB440F4E35393FC39, volume, p1) }
	pub fn _0xD460135C98940274(volume: Volume, p1: Any) { invoke_ignore!(0xD460135C98940274, volume, p1) }
	pub fn _0xEBA87B9273835CF3(p0: Any, p1: Any) { invoke_ignore!(0xEBA87B9273835CF3, p0, p1) }
	pub fn _0xAA9EE2AAFC717623(p0: Any, p1: Any, p2: Any, p3: Any, p4: Any, p5: Any) -> Any { invoke!(0xAA9EE2AAFC717623, p0, p1, p2, p3, p4, p5) }
	pub fn _0x870E9981ED27C815(p0: Any, p1: Any, p2: Any, p3: Any, p4: Any, p5: Any) -> Any { invoke!(0x870E9981ED27C815, p0, p1, p2, p3, p4, p5) }
	pub fn DOES_VOLUME_COLLIDE_WITH_ANY_VOLUME_LOCK(x: f32, y: f32, z: f32, radius: f32, p4: bool, p5: i32, p6: i32) -> bool { invoke!(0x397769175A7DBB30, x, y, z, radius, p4, p5, p6) }
	pub fn _IS_POINT_NEAR_VOLUME_LOCK_CENTER(x: f32, y: f32, z: f32, radius: f32, p4: i32, p5: i32, flags: i32) -> bool { invoke!(0x769BB7626B8CDB06, x, y, z, radius, p4, p5, flags) }
	pub fn _0x51E52C9687FCDEEC(p0: Any, p1: Any, p2: Any, p3: Any, p4: Any, p5: Any, p6: Any) -> Any { invoke!(0x51E52C9687FCDEEC, p0, p1, p2, p3, p4, p5, p6) }
	pub fn _FIND_VOLUME_LOCK_REQUEST_ID_WITH_ARGS(args: &mut Any) -> i32 { invoke!(0x77A6E4AD0C496F81, args) }
	pub fn _MODIFY_VOLUME_LOCK_LOCATION(volLock: i32, x: f32, y: f32, z: f32) { invoke_ignore!(0xEC43C2FFB70E3F30, volLock, x, y, z) }
	pub fn _0x695DAC2DB928F308(p0: Any, p1: Any) { invoke_ignore!(0x695DAC2DB928F308, p0, p1) }
	pub fn _RELEASE_LOCK_VOLUME(volLockRequestId: i32) { invoke_ignore!(0xFDFECC6EE4491E11, volLockRequestId) }
	pub fn _0xAC355980681A7F89(p0: Any) { invoke_ignore!(0xAC355980681A7F89, p0) }
	pub fn _ADD_ENTRY_VOLUME_LOCK(args: &mut Any) -> bool { invoke!(0x58D3803FA639A3BB, args) }
	pub fn _0xC61E2FD926DBB406() { invoke_ignore!(0xC61E2FD926DBB406) }
	pub fn REQUEST_VOLUME_LOCK(x: f32, y: f32, z: f32, radius: f32, p4: i32, p5: i32) -> i32 { invoke!(0xF14BCEF290F869E1, x, y, z, radius, p4, p5) }
	pub fn REQUEST_VOLUME_LOCK_WITH_ARGS(args: &mut Any) -> i32 { invoke!(0x183C0B6CFEFFCAE4, args) }
	pub fn IS_VOLUME_LOCK_REQUEST_VALID(volLockRequestId: i32) -> bool { invoke!(0xA4A4359320345B34, volLockRequestId) }
	pub fn GET_VOLUME_LOCK_REQUEST_STATUS(volLockRequestId: i32) -> i32 { invoke!(0xB33A604345F58202, volLockRequestId) }
	pub fn _0x351D71B8B72B858B(p0: Any) -> Any { invoke!(0x351D71B8B72B858B, p0) }
	pub fn _0xFA15C9A320E707B0() { invoke_ignore!(0xFA15C9A320E707B0) }
	pub fn _0x52572B331E693AED(p0: Any, p1: Any, p2: Any) { invoke_ignore!(0x52572B331E693AED, p0, p1, p2) }
	pub fn _0x128FC3A893BF853A(p0: Any) { invoke_ignore!(0x128FC3A893BF853A, p0) }
	pub fn _SET_VOLUME_RELATIONSHIP(volume: Volume, relationshipGroup: Hash) { invoke_ignore!(0xFD010A2154B40676, volume, relationshipGroup) }
	pub fn _GET_VOLUME_RELATIONSHIP(volume: Volume) -> Hash { invoke!(0x666C2F53ABEFC952, volume) }
}
pub mod WATER {
	use std::ffi::CStr;
	use crate::core::scripthook::native::*;
	use crate::core::types::*;

	pub fn DISABLE_WATER_LOOKUP() { invoke_ignore!(0x754616EC6965D1FB) }
	pub fn ENABLE_WATER_LOOKUP() { invoke_ignore!(0x754616EC6965D1BF) }
	pub fn GET_WATER_HEIGHT(x: f32, y: f32, z: f32, height: &mut f32) -> bool { invoke!(0xFCA8B23F28813F69, x, y, z, height) }
	pub fn GET_WATER_HEIGHT_NO_WAVES(x: f32, y: f32, z: f32, height: &mut f32) -> bool { invoke!(0xDCF3690AA262C03F, x, y, z, height) }
	pub fn TEST_PROBE_AGAINST_ALL_WATER(x1: f32, y1: f32, z1: f32, x2: f32, y2: f32, z2: f32, flags: i32, intersectionPos: &mut Vector3) -> i32 { invoke!(0x8974647ED222EA5F, x1, y1, z1, x2, y2, z2, flags, intersectionPos) }
	pub fn TEST_VERTICAL_PROBE_AGAINST_ALL_WATER(x: f32, y: f32, z: f32, flags: i32, waterHeight: &mut f32) -> i32 { invoke!(0x2B3451FA1E3142E2, x, y, z, flags, waterHeight) }
	pub fn REMOVE_EXTRA_CALMING_QUAD(index: i32) { invoke_ignore!(0x4BEF8DD75AF6C71C, index) }
	pub fn _0x09A1C7DFDCE54FBC(p0: i32) { invoke_ignore!(0x09A1C7DFDCE54FBC, p0) }
	pub fn _0xF0FBF193F1F5C0EA(ped: Ped) { invoke_ignore!(0xF0FBF193F1F5C0EA, ped) }
	pub fn _0xA33F5069B0CB89B8() { invoke_ignore!(0xA33F5069B0CB89B8) }
	pub fn _0xB34A6009A0DB80B8(entity: Entity) { invoke_ignore!(0xB34A6009A0DB80B8, entity) }
	pub fn _SET_OCEAN_GUARMA_WATER_QUADRANT(wavesHeight: f32, p1: f32, p2: i32, p3: f32, p4: f32, p5: f32, p6: f32, wavesStrength: f32, p8: i32) { invoke_ignore!(0xC63540AEF8384732, wavesHeight, p1, p2, p3, p4, p5, p6, wavesStrength, p8) }
	pub fn _RESET_GUARMA_WATER_STATE() { invoke_ignore!(0xC63540AEF8384769) }
	pub fn _SET_WORLD_WATER_TYPE(waterType: i32) { invoke_ignore!(0xE8770EE02AEE45C2, waterType) }
	pub fn _GET_WORLD_WATER_TYPE() -> i32 { invoke!(0x189739A7631C1867) }
	pub fn _0x0DCEC6A92E497E17(entity: Entity, p1: i32) { invoke_ignore!(0x0DCEC6A92E497E17, entity, p1) }
	pub fn _0xE8126623008372AA() { invoke_ignore!(0xE8126623008372AA) }
}
pub mod WEAPON {
	use std::ffi::CStr;
	use crate::core::scripthook::native::*;
	use crate::core::types::*;

	pub fn _GET_WEAPON_UNLOCK(weaponHash: Hash) -> Hash { invoke!(0x865F36299079FB75, weaponHash) }
	pub fn _ADD_AMMO_TO_PED(ped: Ped, weaponHash: Hash, amount: i32, addReason: Hash) { invoke_ignore!(0xB190BCA3F4042F95, ped, weaponHash, amount, addReason) }
	pub fn _ADD_AMMO_TO_PED_BY_TYPE(ped: Ped, ammoType: Hash, amount: i32, addReason: Hash) { invoke_ignore!(0x106A811C6D3035F3, ped, ammoType, amount, addReason) }
	pub fn _REMOVE_AMMO_FROM_PED(ped: Ped, weaponHash: Hash, amount: i32, removeReason: Hash) { invoke_ignore!(0xF4823C813CB8277D, ped, weaponHash, amount, removeReason) }
	pub fn _REMOVE_AMMO_FROM_PED_BY_TYPE(ped: Ped, ammoHash: Hash, amount: i32, removeReason: Hash) { invoke_ignore!(0xB6CFEC32E3742779, ped, ammoHash, amount, removeReason) }
	pub fn _REMOVE_ALL_PED_AMMO(ped: Ped) { invoke_ignore!(0x1B83C0DEEBCBB214, ped) }
	pub fn SET_PED_AMMO(ped: Ped, weaponHash: Hash, ammo: i32) { invoke_ignore!(0x14E56BC5B5DB6A19, ped, weaponHash, ammo) }
	pub fn _0x8A779706DA5CA3DD(ped: Ped, p1: bool, p2: i32) { invoke_ignore!(0x8A779706DA5CA3DD, ped, p1, p2) }
	pub fn _0x9409C62504A8F9E9(vehicle: Vehicle, p1: bool) { invoke_ignore!(0x9409C62504A8F9E9, vehicle, p1) }
	pub fn _SET_VEHICLE_WEAPON_HEADING_LIMITS(vehicle: Vehicle, p1: i32, minHeading: f32, maxHeading: f32) { invoke_ignore!(0x56CB3B4305A4F7CE, vehicle, p1, minHeading, maxHeading) }
	pub fn _SET_VEHICLE_WEAPON_HEADING_LIMITS_2(vehicle: Vehicle, p1: i32, minHeading: f32, maxHeading: f32) -> Any { invoke!(0xBF5987E1CDE63501, vehicle, p1, minHeading, maxHeading) }
	pub fn SET_VEHICLE_WEAPON_HEADING(vehicle: Vehicle, seatIndex: i32, heading: f32, p3: bool) { invoke_ignore!(0x194D877FC5597B7D, vehicle, seatIndex, heading, p3) }
	pub fn SET_PED_INFINITE_AMMO(ped: Ped, toggle: bool, weaponHash: Hash) { invoke_ignore!(0x3EDCB0505123623B, ped, toggle, weaponHash) }
	pub fn _SET_PED_INFINITE_AMMO_CLIP(ped: Ped, toggle: bool) { invoke_ignore!(0xFBAA1E06B6BCA741, ped, toggle) }
	pub fn GET_AMMO_IN_PED_WEAPON(ped: Ped, weaponHash: Hash) -> i32 { invoke!(0x015A522136D7F951, ped, weaponHash) }
	pub fn GET_MAX_AMMO_IN_CLIP(ped: Ped, weaponHash: Hash, p2: bool) -> i32 { invoke!(0xA38DCFFCEA8962FA, ped, weaponHash, p2) }
	pub fn GET_AMMO_IN_CLIP(ped: Ped, ammo: &mut i32, weaponHash: Hash) -> bool { invoke!(0x2E1202248937775C, ped, ammo, weaponHash) }
	pub fn _GET_AMMO_IN_CLIP_BY_INVENTORY_UID(ped: Ped, ammo: &mut i32, inventoryUid: &mut Any) -> bool { invoke!(0x678F00858980F516, ped, ammo, inventoryUid) }
	pub fn SET_AMMO_IN_CLIP(ped: Ped, weaponHash: Hash, ammo: i32) -> bool { invoke!(0xDCD2A934D65CB497, ped, weaponHash, ammo) }
	pub fn _REFILL_AMMO_IN_CLIP(ped: Ped, clipInventoryUid: &mut Any, p2: i32) -> Any { invoke!(0xDF4A3404D022ADDE, ped, clipInventoryUid, p2) }
	pub fn GET_MAX_AMMO(ped: Ped, ammo: &mut i32, weaponHash: Hash) -> bool { invoke!(0xDC16122C7A20C933, ped, ammo, weaponHash) }
	pub fn SET_PED_AMMO_BY_TYPE(ped: Ped, ammoType: Hash, ammo: i32) { invoke_ignore!(0x5FD1E1F011E76D7E, ped, ammoType, ammo) }
	pub fn GET_PED_AMMO_BY_TYPE(ped: Ped, ammoType: Hash) -> i32 { invoke!(0x39D22031557946C1, ped, ammoType) }
	pub fn SET_PED_AMMO_TO_DROP(ped: Ped, p1: i32, p2: i32) { invoke_ignore!(0xA4EFEF9440A5B0EF, ped, p1, p2) }
	pub fn _GET_AMMO_TYPE_FOR_WEAPON(weaponHash: Hash) -> Hash { invoke!(0x5C2EA6C44F515F34, weaponHash) }
	pub fn GET_PED_AMMO_TYPE_FROM_WEAPON(ped: Ped, weaponHash: Hash) -> Hash { invoke!(0x7FEAD38B326B9F74, ped, weaponHash) }
	pub fn _GET_WEAPON_TYPE_FROM_AMMO_TYPE(ammoType: Hash) -> Hash { invoke!(0x7AA043F6C41D151E, ammoType) }
	pub fn _GET_WEAPON_COMPONENT_TYPE_MODEL(componentHash: Hash) -> Hash { invoke!(0x59DE03442B6C9598, componentHash) }
	pub fn _GET_WEAPONTYPE_MODEL(weaponHash: Hash) -> Hash { invoke!(0xF70825EB340E7D15, weaponHash) }
	pub fn _GET_WEAPONTYPE_SLOT(weaponHash: Hash) -> Hash { invoke!(0x46F032B8DDF46CDE, weaponHash) }
	pub fn GET_WEAPONTYPE_GROUP(weaponHash: Hash) -> Hash { invoke!(0xEDCA14CA5199FF25, weaponHash) }
	pub fn _0xF8204EF17410BF43(weaponGroupHash: Hash, p1: f32, p2: f32, p3: Any) -> Hash { invoke!(0xF8204EF17410BF43, weaponGroupHash, p1, p2, p3) }
	pub fn _IS_WEAPON_KIT(weaponHash: Hash) -> bool { invoke!(0x6ABAD7B0A854F8FB, weaponHash) }
	pub fn _IS_WEAPON_KIT_2(weaponHash: Hash) -> bool { invoke!(0x49E40483948AF062, weaponHash) }
	pub fn IS_WEAPON_BOW(weaponHash: Hash) -> bool { invoke!(0xC4DEC3CA8C365A5D, weaponHash) }
	pub fn _IS_WEAPON_LANTERN(weaponHash: Hash) -> bool { invoke!(0x79407D33328286C6, weaponHash) }
	pub fn _IS_WEAPON_TORCH(weaponHash: Hash) -> bool { invoke!(0x506F1DE1BFC75304, weaponHash) }
	pub fn GIVE_WEAPON_TO_PED_WITH_OPTIONS(ped: Ped, data: &mut Any, outData: &mut Any) -> bool { invoke!(0xBE7E42B07FD317AC, ped, data, outData) }
	pub fn GIVE_WEAPON_TO_PED(ped: Ped, weaponHash: Hash, ammoCount: i32, bForceInHand: bool, bForceInHolster: bool, attachPoint: i32, bAllowMultipleCopies: bool, p7: f32, p8: f32, addReason: Hash, bIgnoreUnlocks: bool, permanentDegradation: f32, p12: bool) -> Hash { invoke!(0x5E3BDDBCB83F3D84, ped, weaponHash, ammoCount, bForceInHand, bForceInHolster, attachPoint, bAllowMultipleCopies, p7, p8, addReason, bIgnoreUnlocks, permanentDegradation, p12) }
	pub fn _SET_FORCE_CURRENT_WEAPON_INTO_COCKED_STATE(ped: Ped, attachPoint: i32) { invoke_ignore!(0x5230D3F6EE56CFE6, ped, attachPoint) }
	pub fn _0x9F0E1892C7F228A8(p0: bool) -> Any { invoke!(0x9F0E1892C7F228A8, p0) }
	pub fn _HOLSTER_PED_WEAPONS(ped: Ped, p1: bool, p2: bool, p3: bool, immediately: bool) { invoke_ignore!(0x94A3C1B804D291EC, ped, p1, p2, p3, immediately) }
	pub fn _HIDE_PED_WEAPONS(ped: Ped, p0: i32, immediately: bool) { invoke_ignore!(0xFCCC886EDE3C63EC, ped, p0, immediately) }
	pub fn _0x4820A6939D7CEF28(p0: Any, p1: Any) { invoke_ignore!(0x4820A6939D7CEF28, p0, p1) }
	pub fn SET_CURRENT_PED_WEAPON(ped: Ped, weaponHash: Hash, equipNow: bool, attachPoint: i32, p4: bool, p5: bool) { invoke_ignore!(0xADF692B254977C0C, ped, weaponHash, equipNow, attachPoint, p4, p5) }
	pub fn GET_CURRENT_PED_WEAPON(ped: Ped, weaponHash: &mut Hash, p2: bool, attachPoint: i32, p4: bool) -> bool { invoke!(0x3A87E44BB9A01D54, ped, weaponHash, p2, attachPoint, p4) }
	pub fn GET_CURRENT_PED_WEAPON_ENTITY_INDEX(ped: Ped, attachPoint: i32) -> Entity { invoke!(0x3B390A939AF0B5FC, ped, attachPoint) }
	pub fn _ENABLE_WEAPON_RESTORE(ped: Ped) -> bool { invoke!(0xC395355843BE134B, ped) }
	pub fn GET_PED_BACKUP_WEAPON(ped: Ped, p1: bool) -> Hash { invoke!(0xC71FE230A513C30F, ped, p1) }
	pub fn _0x486C96A0DCD2BC92(p0: Any, p1: Any) -> Any { invoke!(0x486C96A0DCD2BC92, p0, p1) }
	pub fn _GET_PED_WORST_WEAPON(ped: Ped, p1: bool, p2: bool, p3: bool) -> Hash { invoke!(0xDA37A053C1522F5D, ped, p1, p2, p3) }
	pub fn GET_BEST_PED_WEAPON(ped: Ped, p1: bool, p2: bool) -> Hash { invoke!(0x8483E98E8B888AE2, ped, p1, p2) }
	pub fn _GET_BEST_PED_WEAPON_IN_INVENTORY(ped: Ped, p1: Any, guidPrimary: &mut Any) -> Any { invoke!(0x7B98500614C8E8B8, ped, p1, guidPrimary) }
	pub fn GET_BEST_PED_SHORTARM_GUID(ped: Ped, outGUID: &mut Any, p2: bool, p3: bool) { invoke_ignore!(0xF52BD94B47CCF736, ped, outGUID, p2, p3) }
	pub fn GET_PED_WEAPON_GUID_AT_ATTACH_POINT(ped: Ped, attachPoint: i32, weaponGuid: &mut Any) -> bool { invoke!(0x6929E22158E52265, ped, attachPoint, weaponGuid) }
	pub fn _GET_BEST_PED_WEAPON_IN_GROUP(ped: Ped, weaponGroup: Hash, p2: bool, p3: bool) -> Hash { invoke!(0x9F67929D98E7C6E8, ped, weaponGroup, p2, p3) }
	pub fn _GET_DEFAULT_UNARMED_WEAPON_HASH(ped: Ped) -> Hash { invoke!(0x08FF1099ED2E6E21, ped) }
	pub fn _SET_AMMO_IN_TURRET(vehicle: Vehicle, turretHash: Hash, ammo: i32) { invoke_ignore!(0xBDDA0C290C228159, vehicle, turretHash, ammo) }
	pub fn SET_CURRENT_PED_VEHICLE_WEAPON(ped: Ped, weaponHash: Hash) -> bool { invoke!(0x75C55983C2C39DAA, ped, weaponHash) }
	pub fn GET_CURRENT_PED_VEHICLE_WEAPON(ped: Ped, weaponHash: &mut Hash) -> bool { invoke!(0x1017582BCD3832DC, ped, weaponHash) }
	pub fn IS_PED_ARMED(ped: Ped, flags: i32) -> bool { invoke!(0xCB690F680A3EA971, ped, flags) }
	pub fn _0xA2091482ED42EF85(p0: Any, p1: Any) -> Any { invoke!(0xA2091482ED42EF85, p0, p1) }
	pub fn _IS_WEAPON_HOLSTER_STATE_CHANGING(ped: Ped) -> bool { invoke!(0x2387D6E9C6B478AA, ped) }
	pub fn IS_WEAPON_VALID(weaponHash: Hash) -> bool { invoke!(0x937C71165CF334B3, weaponHash) }
	pub fn _IS_AMMO_VALID(ammoHash: Hash) -> bool { invoke!(0x1F7977C9101F807F, ammoHash) }
	pub fn _0x23BF601A42F329A0(p0: Any) -> Any { invoke!(0x23BF601A42F329A0, p0) }
	pub fn IS_PED_CARRYING_WEAPON(ped: Ped, weaponHash: Hash) -> bool { invoke!(0xF29A186ED428B552, ped, weaponHash) }
	pub fn HAS_PED_GOT_WEAPON(ped: Ped, weaponHash: Hash, p2: i32, p3: bool) -> bool { invoke!(0x8DECB02F88F428BC, ped, weaponHash, p2, p3) }
	pub fn _0x07E1C35F0078C3F9(ped: Ped, weapon: Hash) -> bool { invoke!(0x07E1C35F0078C3F9, ped, weapon) }
	pub fn IS_PED_WEAPON_READY_TO_SHOOT(ped: Ped) -> bool { invoke!(0xB80CA294F2F26749, ped) }
	pub fn _GET_PED_WEAPON_IN_SLOT(ped: Ped, slotHash: Hash) -> Hash { invoke!(0xDBC4B552B2AE9A83, ped, slotHash) }
	pub fn GIVE_DELAYED_WEAPON_TO_PED(ped: Ped, weaponHash: Hash, ammoCount: i32, p3: bool, addReason: Hash) { invoke_ignore!(0xB282DC6EBD803C75, ped, weaponHash, ammoCount, p3, addReason) }
	pub fn REMOVE_ALL_PED_WEAPONS(ped: Ped, p1: bool, p2: bool) { invoke_ignore!(0xF25DF915FA38C5F3, ped, p1, p2) }
	pub fn REMOVE_WEAPON_FROM_PED(ped: Ped, weaponHash: Hash, p2: bool, removeReason: Hash) { invoke_ignore!(0x4899CB088EDF59B8, ped, weaponHash, p2, removeReason) }
	pub fn _REMOVE_WEAPON_FROM_PED_BY_GUID(ped: Ped, weaponGuid: &mut Any, removeReason: Hash) { invoke_ignore!(0x51C3B71591811485, ped, weaponGuid, removeReason) }
	pub fn HIDE_PED_WEAPON_FOR_SCRIPTED_CUTSCENE(ped: Ped, toggle: bool) { invoke_ignore!(0x6F6981D2253C208F, ped, toggle) }
	pub fn SET_PED_CURRENT_WEAPON_VISIBLE(ped: Ped, visible: bool, deselectWeapon: bool, p3: bool, p4: bool) { invoke_ignore!(0x0725A4CCFDED9A70, ped, visible, deselectWeapon, p3, p4) }
	pub fn _SET_PED_WEAPON_ATTACH_POINT_VISIBILITY(ped: Ped, attachPoint: i32, visible: bool) { invoke_ignore!(0x67E21ACC5C0C970C, ped, attachPoint, visible) }
	pub fn _SET_PED_ALL_WEAPONS_VISIBILITY(ped: Ped, visible: bool) { invoke_ignore!(0x4F806A6CFED89468, ped, visible) }
	pub fn SET_PED_DROPS_WEAPONS_WHEN_DEAD(ped: Ped, toggle: bool) { invoke_ignore!(0x476AE72C1D19D1A8, ped, toggle) }
	pub fn _0x431240A58484D5D0(ped: Ped, toggle: bool) { invoke_ignore!(0x431240A58484D5D0, ped, toggle) }
	pub fn _0x45E57FDD531C9477(ped: Ped, toggle: bool) { invoke_ignore!(0x45E57FDD531C9477, ped, toggle) }
	pub fn _0xF08D8FEB455F2C8C(ped: Ped, toggle: bool) { invoke_ignore!(0xF08D8FEB455F2C8C, ped, toggle) }
	pub fn _0x16D9841A85FA627E(ped: Ped, toggle: bool) { invoke_ignore!(0x16D9841A85FA627E, ped, toggle) }
	pub fn GET_PED_LAST_WEAPON_IMPACT_COORD(ped: Ped, coords: &mut Vector3) -> bool { invoke!(0x6C4D0409BA1A2BC2, ped, coords) }
	pub fn _CLEAR_PED_LAST_WEAPON_DAMAGE(ped: Ped) { invoke_ignore!(0x087D8F4BC65F68E4, ped) }
	pub fn _HAS_ENTITY_BEEN_DAMAGED_BY_WEAPON(entity: Entity, weaponName: Hash, weaponType: i32) -> bool { invoke!(0xDCF06D0CDFF68424, entity, weaponName, weaponType) }
	pub fn SET_PED_DROPS_INVENTORY_WEAPON(ped: Ped, weaponHash: Hash, xOffset: f32, yOffset: f32, zOffset: f32, ammoCount: i32) { invoke_ignore!(0x208A1888007FC0E6, ped, weaponHash, xOffset, yOffset, zOffset, ammoCount) }
	pub fn _0xB0FB9B196A3D13F0(p0: Any, p1: Any, p2: Any) { invoke_ignore!(0xB0FB9B196A3D13F0, p0, p1, p2) }
	pub fn _0x2EBF70E1D8C06683(ped: Ped, p1: Hash) { invoke_ignore!(0x2EBF70E1D8C06683, ped, p1) }
	pub fn _0x63B83A526329AFBC(p0: Any) { invoke_ignore!(0x63B83A526329AFBC, p0) }
	pub fn _MAKE_PED_RELOAD(ped: Ped) -> Any { invoke!(0x79E1E511FF7EFB13, ped) }
	pub fn _REFILL_AMMO_IN_CURRENT_PED_WEAPON(ped: Ped) -> Any { invoke!(0x0A2AB7B7ABC055F4, ped) }
	pub fn MAKE_PED_DROP_WEAPON(ped: Ped, p1: bool, attachPoint: i32, p3: bool, p4: bool) -> Entity { invoke!(0xCEF4C65DE502D367, ped, p1, attachPoint, p3, p4) }
	pub fn _GET_PED_CURRENT_HELD_WEAPON(ped: Ped) -> Hash { invoke!(0x8425C5F057012DAB, ped) }
	pub fn SET_ALLOW_ANY_WEAPON_DROP(ped: Ped, toggle: bool) { invoke_ignore!(0x78030C7867D8B9B6, ped, toggle) }
	pub fn _0xA3716A77DCF17424(p0: Any, p1: Any, p2: Any) { invoke_ignore!(0xA3716A77DCF17424, p0, p1, p2) }
	pub fn _0x457B16951AD77C1B(p0: Any) { invoke_ignore!(0x457B16951AD77C1B, p0) }
	pub fn _0x3799EFCC3C8CD5E1(p0: Any) -> Any { invoke!(0x3799EFCC3C8CD5E1, p0) }
	pub fn _GET_MAX_LOCKON_DISTANCE_OF_CURRENT_PED_WEAPON(ped: Ped) -> f32 { invoke!(0x79B1A6E780266DB0, ped) }
	pub fn _REMOVE_WEAPON_COMPONENT_FROM_PED(ped: Ped, componentHash: Hash, weaponHash: Hash) { invoke_ignore!(0x19F70C4D80494FF8, ped, componentHash, weaponHash) }
	pub fn _HAS_PED_GOT_WEAPON_COMPONENT(ped: Ped, componentHash: Hash, weaponHash: Hash) -> bool { invoke!(0xBBC67A6F965C688A, ped, componentHash, weaponHash) }
	pub fn _IS_PED_CURRENT_WEAPON_HOLSTERED(ped: Ped) -> bool { invoke!(0xBDD9C235D8D1052E, ped) }
	pub fn _0xD2209866B0CB72EA(p0: Any, p1: Any) -> Any { invoke!(0xD2209866B0CB72EA, p0, p1) }
	pub fn _REQUEST_WEAPON_ASSET(weaponHash: Hash, p1: i32, p2: bool) { invoke_ignore!(0x72D4CB5DB927009C, weaponHash, p1, p2) }
	pub fn _HAS_WEAPON_ASSET_LOADED(weaponHash: Hash) -> bool { invoke!(0xFF07CF465F48B830, weaponHash) }
	pub fn _REMOVE_WEAPON_ASSET(weaponHash: Hash) { invoke_ignore!(0xC3896D03E2852236, weaponHash) }
	pub fn _0xE9B3FEC825668291(p0: Any, p1: Any, p2: Any) { invoke_ignore!(0xE9B3FEC825668291, p0, p1, p2) }
	pub fn _0x9CCA3131E6B53C68(p0: Any, p1: Any, p2: Any) -> Any { invoke!(0x9CCA3131E6B53C68, p0, p1, p2) }
	pub fn _GET_WEAPON_NAME_2(weaponHash: Hash) -> *const char { invoke!(0x6D3AC61694A791C5, weaponHash) }
	pub fn _GET_WEAPON_NAME(weaponHash: Hash) -> *const char { invoke!(0x89CF5FF3D363311E, weaponHash) }
	pub fn _GET_WEAPON_NAME_WITH_PERMANENT_DEGRADATION(weaponHash: Hash, permanentDegradationLevel: f32) -> *const char { invoke!(0x7A56D66C78D8EF8E, weaponHash, permanentDegradationLevel) }
	pub fn _0xB832F1A686B9B810(p0: Any, p1: Any, p2: Any) { invoke_ignore!(0xB832F1A686B9B810, p0, p1, p2) }
	pub fn _0x5A695BD328586B44(p0: Any, p1: Any) -> Any { invoke!(0x5A695BD328586B44, p0, p1) }
	pub fn _0x641351E9AD103890(p0: Any, p1: Any) { invoke_ignore!(0x641351E9AD103890, p0, p1) }
	pub fn SET_INSTANTLY_EQUIP_WEAPON_PICKUPS(ped: Ped, toggle: bool) { invoke_ignore!(0x739B9C6D0E7F7F93, ped, toggle) }
	pub fn _SET_FORCE_AUTO_EQUIP(ped: Ped, toggle: bool) { invoke_ignore!(0xBE711B14A159E84F, ped, toggle) }
	pub fn _SEND_WEAPON_TO_INVENTORY(ped: Ped, weaponHash: Hash) { invoke_ignore!(0xE9BD19F8121ADE3E, ped, weaponHash) }
	pub fn _0x14FF0C2545527F9B(horse: Ped, weaponHash: Hash, ped: Ped) { invoke_ignore!(0x14FF0C2545527F9B, horse, weaponHash, ped) }
	pub fn _0xD4C6E24D955FF061(p0: Any) { invoke_ignore!(0xD4C6E24D955FF061, p0) }
	pub fn _0xAFFD0CCF31F469B8(p0: Any) -> Any { invoke!(0xAFFD0CCF31F469B8, p0) }
	pub fn _GET_WEAPON_STAT_ID(weaponHash: Hash) -> Hash { invoke!(0x8EC44AE8DECFF841, weaponHash) }
	pub fn _HAS_ENTITY_BEEN_DAMAGED_BY_WEAPON_RECENTLY(entity: Entity, weaponHash: Hash, ms: i32) -> bool { invoke!(0x9E2D5D6BC97A5F1E, entity, weaponHash, ms) }
	pub fn _GET_PED_HOGTIE_WEAPON(ped: Ped) -> Hash { invoke!(0x90EB1CB189923587, ped) }
	pub fn _CREATE_WEAPON_OBJECT(weaponHash: Hash, ammoCount: i32, x: f32, y: f32, z: f32, showWorldModel: bool, scale: f32) -> Object { invoke!(0x9888652B8BA77F73, weaponHash, ammoCount, x, y, z, showWorldModel, scale) }
	pub fn REMOVE_WEAPON_COMPONENT_FROM_WEAPON_OBJECT(weaponObject: Object, component: Hash) { invoke_ignore!(0xF7D82B0D66777611, weaponObject, component) }
	pub fn HAS_WEAPON_GOT_WEAPON_COMPONENT(weapon: Object, addonHash: Hash) -> bool { invoke!(0x76A18844E743BF91, weapon, addonHash) }
	pub fn _GIVE_WEAPON_COMPONENT_TO_WEAPON_OBJECT(weaponObject: &mut Object, ped: Ped, componentHash: Hash, p3: bool) { invoke_ignore!(0x1A47699E8D533E8F, weaponObject, ped, componentHash, p3) }
	pub fn _GET_WEAPON_OBJECT_FROM_PED(ped: Ped, p1: bool) -> Object { invoke!(0xC6A6789BB405D11C, ped, p1) }
	pub fn _GET_PED_WEAPON_OBJECT(ped: Ped, p1: bool) -> Object { invoke!(0x6CA484C9A7377E4F, ped, p1) }
	pub fn _GIVE_WEAPON_COMPONENT_TO_ENTITY(entity: Entity, componentHash: Hash, weaponHash: Hash, p3: bool) { invoke_ignore!(0x74C9090FDD1BB48E, entity, componentHash, weaponHash, p3) }
	pub fn _0x74C9080FDD1BB48E(p0: Any, p1: Any) { invoke_ignore!(0x74C9080FDD1BB48E, p0, p1) }
	pub fn _0x74C9080FDD1BB48F(p0: Any, p1: Any) { invoke_ignore!(0x74C9080FDD1BB48F, p0, p1) }
	pub fn _0x74C2365FDD1BB48F(p0: Any, p1: Any) { invoke_ignore!(0x74C2365FDD1BB48F, p0, p1) }
	pub fn _0x74C90AAACC1DD48F(p0: Any) { invoke_ignore!(0x74C90AAACC1DD48F, p0) }
	pub fn _0x74C8000FDD1BB111(p0: Any, p1: Any) -> Any { invoke!(0x74C8000FDD1BB111, p0, p1) }
	pub fn _0x74C8000FDD1BB222(p0: Any, p1: Any) -> Any { invoke!(0x74C8000FDD1BB222, p0, p1) }
	pub fn _GET_WEAPON_GUN_SPINNING_WEAPON_EMOTE_TRICK_TYPE_HASH(emote: Hash, weaponEmoteTrickType: i32) -> Hash { invoke!(0xF4601C1203B1A78D, emote, weaponEmoteTrickType) }
	pub fn _SET_ACTIVE_GUN_SPINNING_EQUIP_KIT_EMOTE_TWIRL(ped: Ped, emote: Hash) { invoke_ignore!(0xCBCFFF805F1B4596, ped, emote) }
	pub fn _GET_PED_GUN_SPINNING_EQUIPPED_KIT_EMOTE_TWIRL(ped: Ped) -> Hash { invoke!(0x2C4FEC3D0EFA9FC0, ped) }
	pub fn _SET_ACTIVE_GUN_SPINNING_KIT_EMOTE_TWIRL(ped: Ped, weaponEmoteTrickType: i32, spin: Hash) { invoke_ignore!(0x01F661BB9C71B465, ped, weaponEmoteTrickType, spin) }
	pub fn _GET_PED_GUN_SPINNING_HASH_FROM_WEAPON_EMOTE_VARIATION(ped: Ped, weaponEmoteVariation: i32) -> Hash { invoke!(0xF3B1620B920D1708, ped, weaponEmoteVariation) }
	pub fn _GET_WEAPON_EMOTE_VARIATION(ped: Ped, variation: i32) -> i32 { invoke!(0x86147D05FA831D3A, ped, variation) }
	pub fn _SET_GUN_SPINNING_INVENTORY_SLOT_ID_ACTIVATE(ped: Ped, emoteType: i32) { invoke_ignore!(0x408CF580C5E96D49, ped, emoteType) }
	pub fn _GET_CAN_TWIRL_WEAPON(weaponHash: Hash) -> bool { invoke!(0x6554ECCE226F2A2A, weaponHash) }
	pub fn _GET_CORRECT_KIT_EMOTE_TWIRL_GUN(ped: Ped, weaponGuid: &mut Any) -> bool { invoke!(0xCD356B42C57BFE01, ped, weaponGuid) }
	pub fn _0xBC9444F2FF94A9C0(p0: Any) -> Any { invoke!(0xBC9444F2FF94A9C0, p0) }
	pub fn _GET_DEFAULT_PED_WEAPON_COLLECTION(pedModel: Hash) -> Hash { invoke!(0xD42514C182121C23, pedModel) }
	pub fn _GIVE_WEAPON_COLLECTION_TO_PED(ped: Ped, weaponCollection: Hash) { invoke_ignore!(0x899A04AFCC725D04, ped, weaponCollection) }
	pub fn _0xF252A85B8F3F8C58(weaponCollection: Hash, dualwieldVariant: Hash) -> bool { invoke!(0xF252A85B8F3F8C58, weaponCollection, dualwieldVariant) }
	pub fn _0x9EEFD670F10656D7(weaponCollection: Hash, weaponGroup: Hash) -> Hash { invoke!(0x9EEFD670F10656D7, weaponCollection, weaponGroup) }
	pub fn _0xF2F585411E748B9C(p0: Any, p1: Any) -> Any { invoke!(0xF2F585411E748B9C, p0, p1) }
	pub fn GET_WEAPON_CLIP_SIZE(weaponHash: Hash) -> i32 { invoke!(0xD3750CCC00635FC2, weaponHash) }
	pub fn _0xA769D753922B031B(p0: Any, p1: Any, p2: Any) { invoke_ignore!(0xA769D753922B031B, p0, p1, p2) }
	pub fn _0xC5899C4CD2E2495D(p0: Any) { invoke_ignore!(0xC5899C4CD2E2495D, p0) }
	pub fn GET_ALLOW_DUAL_WIELD(ped: Ped) -> bool { invoke!(0x918990BD9CE08582, ped) }
	pub fn _SET_ALLOW_DUAL_WIELD(ped: Ped, allow: bool) { invoke_ignore!(0x83B8D50EB9446BBA, ped, allow) }
	pub fn _0x44C8F4908F1B2622(ped: Ped, ammoHash: Hash) -> bool { invoke!(0x44C8F4908F1B2622, ped, ammoHash) }
	pub fn _IS_WEAPON_KNIFE(weaponHash: Hash) -> bool { invoke!(0x792E3EF76C911959, weaponHash) }
	pub fn IS_WEAPON_REVOLVER(weaponHash: Hash) -> bool { invoke!(0xC212F1D05A8232BB, weaponHash) }
	pub fn IS_WEAPON_PISTOL(weaponHash: Hash) -> bool { invoke!(0xDDC64F5E31EEDAB6, weaponHash) }
	pub fn IS_WEAPON_REPEATER(weaponHash: Hash) -> bool { invoke!(0xDDB2578E95EF7138, weaponHash) }
	pub fn IS_WEAPON_RIFLE(weaponHash: Hash) -> bool { invoke!(0x0A82317B7EBFC420, weaponHash) }
	pub fn IS_WEAPON_SHOTGUN(weaponHash: Hash) -> bool { invoke!(0xC75386174ECE95D5, weaponHash) }
	pub fn _IS_WEAPON_SNIPER(weaponHash: Hash) -> bool { invoke!(0x6AD66548840472E5, weaponHash) }
	pub fn IS_WEAPON_MELEE_WEAPON(weaponHash: Hash) -> bool { invoke!(0x959383DCD42040DA, weaponHash) }
	pub fn _IS_WEAPON_THROWABLE(weaponHash: Hash) -> bool { invoke!(0x30E7C16B12DA8211, weaponHash) }
	pub fn _IS_WEAPON_LASSO(weaponHash: Hash) -> bool { invoke!(0x6E4E1A82081EABED, weaponHash) }
	pub fn _IS_WEAPON_BINOCULARS(weaponHash: Hash) -> bool { invoke!(0xC853230E76A152DF, weaponHash) }
	pub fn IS_WEAPON_A_GUN(weaponHash: Hash) -> bool { invoke!(0x705BE297EEBDB95D, weaponHash) }
	pub fn _IS_WEAPON_TWO_HANDED(weaponHash: Hash) -> bool { invoke!(0x0556E9D2ECF39D01, weaponHash) }
	pub fn _IS_WEAPON_ONE_HANDED(weaponHash: Hash) -> bool { invoke!(0xD955FEE4B87AFA07, weaponHash) }
	pub fn _IS_WEAPON_SILENT(weaponHash: Hash) -> bool { invoke!(0x5809DBCA0A37C82B, weaponHash) }
	pub fn _0xEA522F991E120D45(p0: Any) -> Any { invoke!(0xEA522F991E120D45, p0) }
	pub fn _IS_AMMO_SILENT(ammoHash: Hash) -> bool { invoke!(0xD2866CBA797E872E, ammoHash) }
	pub fn _IS_AMMO_SILENT_2(ammoHash: Hash) -> bool { invoke!(0x7EFACC589B98C488, ammoHash) }
	pub fn SHOULD_WEAPON_BE_DISCARDED_WHEN_SWAPPED(weaponHash: Hash) -> bool { invoke!(0x2C83212A7AA51D3D, weaponHash) }
	pub fn _0x5B235F24472F2C3B(p0: Any, p1: Any) -> Any { invoke!(0x5B235F24472F2C3B, p0, p1) }
	pub fn _0xBFCA7AFABF9D7967(p0: Any, p1: Any) -> Any { invoke!(0xBFCA7AFABF9D7967, p0, p1) }
	pub fn _0x495A04CAEC263AF8(p0: Any, p1: Any) -> Any { invoke!(0x495A04CAEC263AF8, p0, p1) }
	pub fn _0x95CA12E2C68043E5(p0: Any, p1: Any) -> Any { invoke!(0x95CA12E2C68043E5, p0, p1) }
	pub fn _0xABC18A28BAD4B46F(p0: Any, p1: Any) -> Any { invoke!(0xABC18A28BAD4B46F, p0, p1) }
	pub fn _0x80BB243789008A82(p0: Any, p1: Any) -> Any { invoke!(0x80BB243789008A82, p0, p1) }
	pub fn _GET_WEAPON_DAMAGE(weaponObject: Object) -> f32 { invoke!(0x904103D5D2333977, weaponObject) }
	pub fn _SET_WEAPON_DAMAGE(weaponObject: Object, level: f32, p2: bool) { invoke_ignore!(0xE22060121602493B, weaponObject, level, p2) }
	pub fn _GET_WEAPON_DIRT(weaponObject: Object) -> f32 { invoke!(0x810E8AE9AFEA7E54, weaponObject) }
	pub fn _SET_WEAPON_DIRT(weaponObject: Object, level: f32, p2: bool) { invoke_ignore!(0x812CE61DEBCAB948, weaponObject, level, p2) }
	pub fn _GET_WEAPON_SOOT(weaponObject: Object) -> f32 { invoke!(0x4BF66F8878F67663, weaponObject) }
	pub fn _SET_WEAPON_SOOT(weaponObject: Object, level: f32, p2: bool) { invoke_ignore!(0xA9EF4AD10BDDDB57, weaponObject, level, p2) }
	pub fn _SET_WEAPON_LEVEL_THRESHOLD(weaponObject: Object, threshold: f32) { invoke_ignore!(0xD4071EFC83794B2F, weaponObject, threshold) }
	pub fn GET_WEAPON_DEGRADATION(weaponObject: Object) -> f32 { invoke!(0x0D78E1097F89E637, weaponObject) }
	pub fn GET_WEAPON_PERMANENT_DEGRADATION(weaponObject: Object) -> f32 { invoke!(0xD56E5F336C675EFA, weaponObject) }
	pub fn _SET_WEAPON_DEGRADATION(weaponObject: Object, level: f32) { invoke_ignore!(0xA7A57E89E965D839, weaponObject, level) }
	pub fn _LISTEN_PROJECTILE_HIT_EVENTS(listen: bool) { invoke_ignore!(0xDA5D3F2C6DD5B5D4, listen) }
	pub fn _GET_WEAPON_SCALE(weaponObject: Object) -> f32 { invoke!(0x22084CA699219624, weaponObject) }
	pub fn _SET_WEAPON_SCALE(weaponObject: Object, scale: f32) { invoke_ignore!(0xC3544AD0522E69B4, weaponObject, scale) }
	pub fn _0x58425FCA3D3A2D15(p0: Any) -> Any { invoke!(0x58425FCA3D3A2D15, p0) }
	pub fn _0xEC97101A8F311282(p0: Any) -> Any { invoke!(0xEC97101A8F311282, p0) }
	pub fn _GET_CURRENT_PED_WEAPON_AMMO_TYPE(ped: Ped, weaponObject: Object) -> Hash { invoke!(0x7E7B19A4355FEE13, ped, weaponObject) }
	pub fn _GET_CURRENT_AMMO_TYPE_FROM_GUID(ped: Ped, weaponGuid: &mut Any) -> Hash { invoke!(0xAF9D167A5656D6A6, ped, weaponGuid) }
	pub fn _IS_AMMO_TYPE_VALID_FOR_WEAPON(weaponHash: Hash, ammoHash: Hash) -> bool { invoke!(0xC570B881754DF609, weaponHash, ammoHash) }
	pub fn _SET_AMMO_TYPE_FOR_PED_WEAPON(ped: Ped, weaponHash: Hash, ammoHash: Hash) { invoke_ignore!(0xCC9C4393523833E2, ped, weaponHash, ammoHash) }
	pub fn _0x183CE355115B6E75(p0: Any, p1: Any) { invoke_ignore!(0x183CE355115B6E75, p0, p1) }
	pub fn _SET_AMMO_TYPE_FOR_PED_WEAPON_INVENTORY(ped: Ped, weaponInventoryUid: &mut Any, ammoHash: Hash) { invoke_ignore!(0xEBE46B501BC3FBCF, ped, weaponInventoryUid, ammoHash) }
	pub fn _DISABLE_AMMO_TYPE_FOR_PED_WEAPON(ped: Ped, weaponHash: Hash, ammoHash: Hash) { invoke_ignore!(0xF0D728EEA3C99775, ped, weaponHash, ammoHash) }
	pub fn _DISABLE_AMMO_TYPE_FOR_PED(ped: Ped, ammoHash: Hash) { invoke_ignore!(0xAA5A52204E077883, ped, ammoHash) }
	pub fn _0xD63B4BA3A02A99E0(p0: Any, p1: Any) { invoke_ignore!(0xD63B4BA3A02A99E0, p0, p1) }
	pub fn _ENABLE_AMMO_TYPE_FOR_PED_WEAPON(ped: Ped, weaponHash: Hash, ammoHash: Hash) { invoke_ignore!(0x23FB9FACA28779C1, ped, weaponHash, ammoHash) }
	pub fn _ENABLE_AMMO_TYPE_FOR_PED(ped: Ped, weaponHash: Hash) { invoke_ignore!(0x3B7B7908B7ADFB4B, ped, weaponHash) }
	pub fn _0x404514D231DB27A0(p0: Any, p1: Any) { invoke_ignore!(0x404514D231DB27A0, p0, p1) }
	pub fn _0xD53846B9C931C181(p0: Any, p1: Any, p2: Any) { invoke_ignore!(0xD53846B9C931C181, p0, p1, p2) }
	pub fn _0x000FA7A4A8443AF7(p0: Any) { invoke_ignore!(0x000FA7A4A8443AF7, p0) }
	pub fn _0xECBB26529A737EF6(p0: Any) { invoke_ignore!(0xECBB26529A737EF6, p0) }
	pub fn _GET_WEAPON_ATTACH_POINT(ped: Ped, attachPoint: i32) -> i32 { invoke!(0xCAD4FE9398820D24, ped, attachPoint) }
	pub fn _0x4823F13A21F51964(p0: Any, p1: Any) -> Any { invoke!(0x4823F13A21F51964, p0, p1) }
	pub fn SET_CURRENT_PED_WEAPON_BY_GUID(ped: Ped, weaponUid: &mut Any, p2: bool, p3: bool, p4: bool, p5: bool) { invoke_ignore!(0x12FB95FE3D579238, ped, weaponUid, p2, p3, p4, p5) }
	pub fn SET_PLAYER_PED_QUICK_SWAP_WEAPON_BY_GUID(ped: Ped, guidPrimary: &mut Any, guidSecondary: &mut Any) { invoke_ignore!(0xEC1F85DA51D3D6C4, ped, guidPrimary, guidSecondary) }
	pub fn _GET_PLAYER_PED_QUICK_SWAP_WEAPON_BY_GUID(ped: Ped, guidPrimary: &mut Any, guidSecondary: &mut Any) { invoke_ignore!(0xB7E52A058B07C7E2, ped, guidPrimary, guidSecondary) }
	pub fn _0x0DE0944ECCB3DF5D(ped: Ped) -> bool { invoke!(0x0DE0944ECCB3DF5D, ped) }
	pub fn _0x46D42883E873C1D7(ped: Ped) -> Any { invoke!(0x46D42883E873C1D7, ped) }
	pub fn _IS_TARGET_PED_CONSTRAINED_BY_PED_USING_BOLAS(ped: Ped, targetPed: Ped) -> bool { invoke!(0x8D50F43298AB9545, ped, targetPed) }
	pub fn _0x65DC4AC5B96614CB(weaponHash: Hash) -> i32 { invoke!(0x65DC4AC5B96614CB, weaponHash) }
}
pub mod ZONE {
	use std::ffi::CStr;
	use crate::core::scripthook::native::*;
	use crate::core::types::*;

	pub fn _GET_MAP_ZONE_AT_COORDS(x: f32, y: f32, z: f32, type_: i32) -> Hash { invoke!(0x43AD8FC02B429D33, x, y, z, type_) }
	pub fn _GET_WATER_MAP_ZONE_AT_COORDS(x: f32, y: f32, z: f32) -> Hash { invoke!(0x5BA7A68A346A5A91, x, y, z) }
}
pub mod COMPAPP {
	use std::ffi::CStr;
	use crate::core::scripthook::native::*;
	use crate::core::types::*;

	pub fn _0xB6FD96420C0126A1(p0: Hash, p1: bool) { invoke_ignore!(0xB6FD96420C0126A1, p0, p1) }
	pub fn _0x74BCCEB233AD95B2(p0: Hash, p1: Hash) { invoke_ignore!(0x74BCCEB233AD95B2, p0, p1) }
	pub fn _0x29C733459A9011EB(p0: Hash, p1: & CStr) { invoke_ignore!(0x29C733459A9011EB, p0, p1) }
	pub fn _0x7AF1BB4504EA5ED9() -> bool { invoke!(0x7AF1BB4504EA5ED9) }
	pub fn _0xCCB4635A071FB62D() { invoke_ignore!(0xCCB4635A071FB62D) }
}