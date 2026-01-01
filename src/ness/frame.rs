use super::*;

//pub static mut nairboosts: [i32; 8] = [1; 8];

use skyline::hooks::{getRegionAddress, Region};

static mut INT_OFFSET : usize = 0x4e53a0;
static mut FLOAT_OFFSET : usize = 0x4e53e0;

#[skyline::hook(offset=INT_OFFSET)]
pub unsafe fn int_param_accessor_hook(boma: u64, param_type: u64, param_hash: u64) -> i32 {
    let ret = original!()(boma, param_type, param_hash);
    let module_accessor = &mut *(*((boma as *mut u64).offset(1)) as *mut BattleObjectModuleAccessor);
    let fighter_kind = smash::app::utility::get_kind(module_accessor);
    let color = WorkModule::get_int(module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR);
    if param_hash == 0 {
        if fighter_kind == FIGHTER_KIND_NESS {
            if color == 6 || color == 7 {
                if param_type == hash40("jump_count_max") {
                    return 2; //2
                } 
            } 
        }
    }
    ret
}

#[skyline::hook(offset=FLOAT_OFFSET)]
pub unsafe fn float_param_accessor_hook(boma: u64, param_type: u64, param_hash: u64) -> f32 {
    let ret = original!()(boma, param_type, param_hash);
    let module_accessor = &mut *(*((boma as *mut u64).offset(1)) as *mut BattleObjectModuleAccessor);
    let fighter_kind = smash::app::utility::get_kind(module_accessor);
    let color = WorkModule::get_int(module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR);
    if param_hash == 0 {
        if fighter_kind == FIGHTER_KIND_NESS {
            if color == 6 || color == 7 {
                if param_type == hash40("dash_speed") {
                    return 1.9; //1.826
                } else if param_type == hash40("weight") {
                    return 76.0; //94.0
                } else if param_type == hash40("run_speed_max") {
                    return 1.7; //1.609
                } else if param_type == hash40("run_accel_mul") {
                    return 0.08; //0.07161
                } else if param_type == hash40("run_accel_add") {
                    return 0.05; //0.044
                } else if param_type == hash40("jump_speed_x") {
                    return 0.9; //1.0
                } else if param_type == hash40("jump_speed_x_mul") {
                    return 0.7; //0.8
                } else if param_type == hash40("jump_speed_x_max") {
                    return 1.05; //1.3
                } else if param_type == hash40("jump_aerial_speed_x_mul") {
                    return 0.5; //0.8
                } else if param_type == hash40("jump_initial_y") {
                    return 16.964; //18.964
                } else if param_type == hash40("jump_y") {
                    return 28.48; //34.48
                } else if param_type == hash40("mini_jump_y") {
                    return 13.65; //16.65
                } else if param_type == hash40("jump_aerial_y") {
                    return 29.0; //45.65
                } else if param_type == hash40("air_accel_y") {
                    return 0.082; //0.077
                } else if param_type == hash40("air_speed_y_stable") {
                    return 2.0; //1.31
                } else if param_type == hash40("air_accel_x_mul") {
                    return 0.05; //0.09
                } else if param_type == hash40("air_accel_x_add") {
                    return 0.004; //0.01
                } else if param_type == hash40("air_brake_x") {
                    return 0.015; //0.0225
                } else if param_type == hash40("air_speed_x_stable") {
                    return 1.0; //1.007
                }
            }
        }
    }
    ret
}

fn find_subsequence(haystack: &[u8], needle: &[u8]) -> Option<usize> {
    haystack.windows(needle.len()).position(|window| window == needle)
}

static INT_SEARCH_CODE: &[u8] = &[
    0x00, 0x1c, 0x40, 0xf9, 0x08, 0x00, 0x40, 0xf9, 0x03, 0x11, 0x40, 0xf9,
];

static FLOAT_SEARCH_CODE: &[u8] = &[
    0x00, 0x1c, 0x40, 0xf9, 0x08, 0x00, 0x40, 0xf9, 0x03, 0x19, 0x40, 0xf9,
];

////// fighter frames
unsafe extern "C" fn skullkid_frame(fighter: &mut L2CFighterCommon) {
    unsafe {
        let color = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR);
        let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
        if color == 6 || color == 7 {
            PostureModule::set_scale(fighter.module_accessor, 1.20, false);
            if StatusModule::situation_kind(fighter.module_accessor) == *SITUATION_KIND_GROUND {
                nairboosts[entry_id] = 1;
                //allow_nairboost[entry_id] = true;
            }
            //if MotionModule::motion_kind(fighter.module_accessor) == hash40("jumpaerialf") ||
            //MotionModule::motion_kind(fighter.module_accessor) == hash40("jumpaerialb") {
            //    KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
            //}
            //if MotionModule::motion_kind(fighter.module_accessor) == hash40("jumpsquat") ||
            //MotionModule::motion_kind(fighter.module_accessor) == hash40("jumpfront") ||
            //MotionModule::motion_kind(fighter.module_accessor) == hash40("jumpfrontmini") ||
            //MotionModule::motion_kind(fighter.module_accessor) == hash40("jumpback") ||
            //MotionModule::motion_kind(fighter.module_accessor) == hash40("jumpbackmini") {
            //    if MotionModule::frame(fighter.module_accessor) > 10.0 {
            //        if ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_ATTACK) {
            //            let xCtrl = ControlModule::get_stick_x(fighter.module_accessor);
            //            let yCtrl = ControlModule::get_stick_y(fighter.module_accessor);
            //        if xCtrl == 0.0 && yCtrl == 0.0 {
            //            KineticModule::clear_speed_all(fighter.module_accessor);
            //            allow_nairboost[entry_id] = true;
            //        } else {
            //            allow_nairboost[entry_id] = false;
            //            }
            //        }
            //    }
            //}
        }
    }
}

unsafe extern "C" fn skullkid_effect_jumpaerial(agent: &mut L2CAgentBase) {
    let color = WorkModule::get_int(agent.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR);
    if color == 6 || color == 7 {
        frame(agent.lua_state_agent, 1.0);
        for _ in 0..4 {
            if macros::is_excute(agent) {
                macros::EFFECT(agent, Hash40::new("sys_flash"), Hash40::new("waist"), 0, 0, 0, 0, 0, 0, 0.5, 10, 10, 10, 0, 0, 0, false);
                macros::LAST_EFFECT_SET_COLOR(agent, 0.58, 0.0, 0.82);
                macros::LAST_EFFECT_SET_ALPHA(agent, 0.8);
            }
        wait(agent.lua_state_agent, 6.0);
        }
    } else {
        frame(agent.lua_state_agent, 1.0);
        for _ in 0..4 {
            if macros::is_excute(agent) {
                macros::EFFECT(agent, Hash40::new("sys_flash"), Hash40::new("waist"), 0, 0, 0, 0, 0, 0, 1, 10, 10, 10, 0, 0, 0, false);
            }
        wait(agent.lua_state_agent, 6.0);
        }
    }
}

unsafe extern "C" fn skullkid_status_JumpAerial_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let color = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR);
    if color == 6 || color == 7 {} else {
        MotionModule::set_trans_move_speed_no_scale(fighter.module_accessor, true);
    }   
    fighter.status_JumpAerial()
}

unsafe extern "C" fn skullkid_status_JumpAerial_Pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    let color = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR);
    if color == 6 || color == 7 {
        StatusModule::init_settings(
            fighter.module_accessor,
            smash::app::SituationKind(*SITUATION_KIND_AIR),
            *FIGHTER_KINETIC_TYPE_JUMP_AERIAL,
            *GROUND_CORRECT_KIND_AIR as u32,
            smash::app::GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_ON_DROP_BOTH_SIDES),
            true,
            *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLAG,
            *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_INT,
            *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLOAT,
            0
        );
        FighterStatusModuleImpl::set_fighter_status_data(
            fighter.module_accessor,
            false, 
            *FIGHTER_TREADED_KIND_ENABLE,
            true,  
            false, 
            true,  
            0,
            *FIGHTER_STATUS_ATTR_INTO_DOOR as u32,
            0,
            0
        );
        return 0.into();
    } else {
            StatusModule::init_settings(
            fighter.module_accessor,
            smash::app::SituationKind(*SITUATION_KIND_AIR),
            *FIGHTER_KINETIC_TYPE_JUMP_AERIAL_MOTION,  
            *GROUND_CORRECT_KIND_AIR as u32,
            smash::app::GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_ON_DROP_BOTH_SIDES),
            true,
            *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLAG,
            *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_INT,
            *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLOAT,
            0
        );
        FighterStatusModuleImpl::set_fighter_status_data(
            fighter.module_accessor,
            false, 
            *FIGHTER_TREADED_KIND_ENABLE,
            true,  
            false, 
            true,  
            0,
            *FIGHTER_STATUS_ATTR_INTO_DOOR as u32,
            0,
            0
        );
        return 0.into();
    }
}


unsafe extern "C" fn skullkid_attackair_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let color = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR);
    fighter.sub_attack_air_common(false.into());
    if color == 6 || color == 7 {} else {
        MotionModule::set_trans_move_speed_no_scale(fighter.module_accessor, true);
    }
    fighter.sub_shift_status_main(L2CValue::Ptr(skullkid_attackair_main_loop as *const () as _));
    return 0.into();
}

unsafe extern "C" fn skullkid_attackair_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let mut return_value = false;
    if !fighter.status_AttackAir_Main_common().get_bool() {
        fighter.sub_air_check_superleaf_fall_slowly();
        if fighter.global_table[8].get_bool() != true {
            fighter.sub_attack_air_inherit_jump_aerial_motion_uniq_process_exec_fix_pos();
        }
    } else {
        return_value = true;
    }
    return return_value.into();
}

unsafe extern "C" fn empty_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    0.into()
}

pub fn install() {
    unsafe {
        let text_ptr = getRegionAddress(Region::Text) as *const u8;
        let text_size = (getRegionAddress(Region::Rodata) as usize) - (text_ptr as usize);
        let text = std::slice::from_raw_parts(text_ptr, text_size);
        if let Some(offset) = find_subsequence(text, INT_SEARCH_CODE) {
            INT_OFFSET = offset;
        }
        if let Some(offset) = find_subsequence(text, FLOAT_SEARCH_CODE) {
            FLOAT_OFFSET = offset;
        }
    }

    skyline::install_hooks!(
        int_param_accessor_hook,
        float_param_accessor_hook
    );

    Agent::new("ness")
        .on_line(Main, skullkid_frame)
        .status(Main, *FIGHTER_STATUS_KIND_JUMP_AERIAL, skullkid_status_JumpAerial_main)
        .status(Pre, *FIGHTER_STATUS_KIND_JUMP_AERIAL, skullkid_status_JumpAerial_Pre)

        .effect_acmd("effect_jumpaerialfront", skullkid_effect_jumpaerial, Default)
        .effect_acmd("effect_jumpaerialback", skullkid_effect_jumpaerial, Default)
        

        //.status(Init, *FIGHTER_STATUS_KIND_ATTACK_AIR, empty_status)
        //.status(Exec, *FIGHTER_STATUS_KIND_ATTACK_AIR, empty_status)
        .status(Main, *FIGHTER_STATUS_KIND_ATTACK_AIR, skullkid_attackair_main)
        //.status(Exit, *FIGHTER_STATUS_KIND_ATTACK_AIR, empty_status)

        .install();
}