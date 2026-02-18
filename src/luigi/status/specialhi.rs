use super::*;
use smash::phx::*;

unsafe extern "C" fn mrl_specialhi_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    return 0.into();
}

unsafe extern "C" fn mrl_specialhi_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let color = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR);
    let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    WorkModule::set_int64(fighter.module_accessor, hash40("special_hi") as i64, *FIGHTER_STATUS_SUPER_JUMP_PUNCH_WORK_INT_MOTION_KIND);
    WorkModule::set_int64(fighter.module_accessor, hash40("special_air_hi") as i64, *FIGHTER_STATUS_SUPER_JUMP_PUNCH_WORK_INT_MOTION_KIND_AIR);
    WorkModule::set_float(fighter.module_accessor, WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_hi"), hash40("lr_stick_x")), *FIGHTER_STATUS_SUPER_JUMP_PUNCH_WORK_FLOAT_CONST_LR_STICK_X);
    WorkModule::set_float(fighter.module_accessor, WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_hi"), hash40("dir_stick_x")), *FIGHTER_STATUS_SUPER_JUMP_PUNCH_WORK_FLOAT_CONST_DIR_STICK_X);
    WorkModule::set_float(fighter.module_accessor, WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_hi"), hash40("dir_mul")), *FIGHTER_STATUS_SUPER_JUMP_PUNCH_WORK_FLOAT_CONST_DIR_MUL);
    WorkModule::set_float(fighter.module_accessor, WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_hi"), hash40("pass_mul")), *FIGHTER_STATUS_SUPER_JUMP_PUNCH_WORK_FLOAT_CONST_PASS_MUL);
    WorkModule::set_float(fighter.module_accessor, WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_hi"), hash40("air_accel_y")), *FIGHTER_STATUS_SUPER_JUMP_PUNCH_WORK_FLOAT_CONST_AIR_ACCEL_Y);
    WorkModule::set_float(fighter.module_accessor, WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_hi"), hash40("air_start_x_mul")), *FIGHTER_STATUS_SUPER_JUMP_PUNCH_WORK_FLOAT_CONST_AIR_START_X_MUL);
    WorkModule::set_float(fighter.module_accessor, WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_hi"), hash40("air_pass_mul")), *FIGHTER_STATUS_SUPER_JUMP_PUNCH_WORK_FLOAT_CONST_AIR_PASS_MUL);
    WorkModule::set_float(fighter.module_accessor, WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_hi"), hash40("fall_max_x")), *FIGHTER_STATUS_SUPER_JUMP_PUNCH_WORK_FLOAT_CONST_FALL_X_MUL);
    WorkModule::set_int(fighter.module_accessor, WorkModule::get_param_int(fighter.module_accessor, hash40("param_special_hi"), hash40("landing_frame")) as i32, *FIGHTER_STATUS_SUPER_JUMP_PUNCH_WORK_INT_CONST_LANDING_FRAME);
    if color == 7 {
        if upb_boosts[entry_id] > 0 && !(fighter.global_table[0x16] == SITUATION_KIND_GROUND) {
            WorkModule::set_int(fighter.module_accessor, *FIGHTER_STATUS_KIND_FALL, *FIGHTER_STATUS_SUPER_JUMP_PUNCH_WORK_INT_STATUS_KIND_END);
            upb_boosts[entry_id] -= 1;
        } else {
            WorkModule::set_int(fighter.module_accessor, *FIGHTER_LUIGI_STATUS_KIND_SPECIAL_HI_DROP, *FIGHTER_STATUS_SUPER_JUMP_PUNCH_WORK_INT_STATUS_KIND_END);
        }
    } else {
        WorkModule::set_int(fighter.module_accessor, *FIGHTER_LUIGI_STATUS_KIND_SPECIAL_HI_DROP, *FIGHTER_STATUS_SUPER_JUMP_PUNCH_WORK_INT_STATUS_KIND_END);
    }
    WorkModule::set_float(fighter.module_accessor, WorkModule::get_param_int(fighter.module_accessor, hash40("param_special_hi"), hash40("landing_frame")) as f32, *FIGHTER_INSTANCE_WORK_ID_FLOAT_LANDING_FRAME);
    fighter.super_jump_punch(L2CValue::Void());
    fighter.sub_shift_status_main(L2CValue::Ptr(mrl_specialhi_main_loop as *const () as _));
    return 0.into();
}

unsafe extern "C" fn mrl_specialhi_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if !fighter.sub_transition_group_check_air_cliff().get_bool(){
        fighter.super_jump_punch_main();
    }
    return 0.into();
}

unsafe extern "C" fn mrl_specialhidrop_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    sv_kinetic_energy::set_stable_speed(fighter.lua_state_agent);
    sv_kinetic_energy::controller_set_accel_x_mul(fighter.lua_state_agent);
    sv_kinetic_energy::controller_set_accel_x_add(fighter.lua_state_agent);
    MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_hi_drop"), 0.0, 1.0, false, 0.0, false, false);
    fighter.sub_shift_status_main(L2CValue::Ptr(mrl_specialhidrop_main_loop as *const () as _));
    return 0.into();
}


unsafe extern "C" fn mrl_specialhidrop_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.sub_transition_group_check_air_cliff().get_bool() {
        return 0.into();
    }
    if CancelModule::is_enable_cancel(fighter.module_accessor) == false {
        if !WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_LANDING_FALL_SPECIAL) {
            if !MotionModule::is_end(fighter.module_accessor) {
                return 0.into();
            } 
            //fighter.change_status(FIGHTER_LUIGI_STATUS_KIND_SPECIAL_HI_FALL.into(), false.into());
            fighter.change_status(FIGHTER_LUIGI_STATUS_KIND_SPECIAL_HI_FALL.into(), false.into());
        } else {
            if fighter.global_table[0x16] != SITUATION_KIND_GROUND {
                if !MotionModule::is_end(fighter.module_accessor) {
                    return 0.into();
                } 
                //fighter.change_status(FIGHTER_LUIGI_STATUS_KIND_SPECIAL_HI_FALL.into(), false.into());
                fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
            }
            //fighter.change_status(FIGHTER_LUIGI_STATUS_KIND_SPECIAL_HI_LANDING_FALL.into(), false.into());
            fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
        }
    } else {
        if fighter.sub_wait_ground_check_common(false.into()).get_bool() == false {
            if fighter.sub_air_check_fall_common().get_bool() == true {
                return 0.into();
            } 
            if !WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_LANDING_FALL_SPECIAL) {
                if !MotionModule::is_end(fighter.module_accessor) {
                    return 0.into();
                } 
                //fighter.change_status(FIGHTER_LUIGI_STATUS_KIND_SPECIAL_HI_FALL.into(), false.into());
                fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
            } else {
                if fighter.global_table[0x16] != SITUATION_KIND_GROUND {
                    if !MotionModule::is_end(fighter.module_accessor) {
                        return 0.into();
                    } 
                    //fighter.change_status(FIGHTER_LUIGI_STATUS_KIND_SPECIAL_HI_FALL.into(), false.into());
                    fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
                }
                //fighter.change_status(FIGHTER_LUIGI_STATUS_KIND_SPECIAL_HI_LANDING_FALL.into(), false.into());
                fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
            }
        }
    }
    return 0.into();
}


pub fn install() {
    Agent::new("luigi")
        .status(Main, *FIGHTER_LUIGI_STATUS_KIND_SPECIAL_HI_DROP, mrl_specialhidrop_main)
        //.status(Main, *FIGHTER_STATUS_KIND_SPECIAL_HI, mrl_specialhi_main)
        .install();
}