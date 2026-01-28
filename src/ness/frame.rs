use super::*;

////// fighter frames
unsafe extern "C" fn skullkid_frame(fighter: &mut L2CFighterCommon) {
    unsafe {
        let color = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR);
        let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
        if color == 6 || color == 7 {
            PostureModule::set_scale(fighter.module_accessor, 1.20, false);
            if StatusModule::situation_kind(fighter.module_accessor) == *SITUATION_KIND_GROUND {
                nairboosts[entry_id] = 1;
            }
            if MotionModule::motion_kind(fighter.module_accessor) == hash40("jump_squat")  ||
            MotionModule::motion_kind(fighter.module_accessor) == hash40("jump_f") ||
            MotionModule::motion_kind(fighter.module_accessor) == hash40("jump_f_mini") ||
            MotionModule::motion_kind(fighter.module_accessor) == hash40("jump_b") ||
            MotionModule::motion_kind(fighter.module_accessor) == hash40("jump_b_mini") ||
            MotionModule::motion_kind(fighter.module_accessor) == hash40("cliff_jump_quick1") ||
            MotionModule::motion_kind(fighter.module_accessor) == hash40("cliff_jump_quick1jr") ||
            MotionModule::motion_kind(fighter.module_accessor) == hash40("cliff_jump_quick2") {
                if MotionModule::frame(fighter.module_accessor) < 8.0 {
                    earlyjump[entry_id] = true;
                    macros::EFFECT(fighter, Hash40::new("sys_damage_purple"), Hash40::new("waist"), 0, 0, 0, 0, 0, 0, 0.5, 10, 4, 4, 0, 0, 0, false);
                } else {
                    earlyjump[entry_id] = false;
                }
            } else {
                if !MotionModule::motion_kind(fighter.module_accessor) == hash40("attack_air_n") {
                    earlyjump[entry_id] = false;
                }
            }
        }
    }
}

unsafe extern "C" fn skullkid_effect_jumpaerial(agent: &mut L2CAgentBase) {
    let color = WorkModule::get_int(agent.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR);
    if color == 6 || color == 7 {
        /*frame(agent.lua_state_agent, 1.0);
        for _ in 0..4 {
            if macros::is_excute(agent) {
                macros::EFFECT(agent, Hash40::new("sys_flash"), Hash40::new("waist"), 0, 0, 0, 0, 0, 0, 0.5, 10, 10, 10, 0, 0, 0, false);
                macros::LAST_EFFECT_SET_COLOR(agent, 0.58, 0.0, 0.82);
                macros::LAST_EFFECT_SET_ALPHA(agent, 0.8);
            }
        wait(agent.lua_state_agent, 6.0);
        }*/
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
    if color == 6 || color == 7 {
        
        //MotionModule::set_trans_move_speed_no_scale(fighter.module_accessor, true);
    } else {
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

unsafe extern "C" fn skullkid_status_JumpAerial_Exit(fighter: &mut L2CFighterCommon) -> L2CValue {
    0.into()
}

unsafe extern "C" fn skullkid_attackair_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let color = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR);
    let jumps = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_JUMP_COUNT);
    fighter.sub_attack_air_common(false.into());
    if color == 6 || color == 7 {
        //if ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_JUMP) {
        //    EffectModule::req_follow(fighter.module_accessor, Hash40::new("mewtwo_final_aura"), Hash40::new("hip"), &Vector3f{x: 0.0, y: 0.0, z: 0.0}, &Vector3f{x: 0.0, y: 0.0, z: 0.0}, 1.0, true, 0, 0, 0, 0, 0, false, false);
        //}
    } else {
        MotionModule::set_trans_move_speed_no_scale(fighter.module_accessor, true);
    }
    fighter.sub_shift_status_main(L2CValue::Ptr(skullkid_attackair_main_loop as *const () as _));
    return 0.into();
}

unsafe extern "C" fn skullkid_attackair_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let color = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR);
    let mut return_value = false;
    if !fighter.status_AttackAir_Main_common().get_bool() {
        fighter.sub_air_check_superleaf_fall_slowly();
        if fighter.global_table[8].get_bool() != true {
            if color == 6 || color == 7 {
                fighter.sub_attack_air_inherit_jump_aerial_motion_uniq_process_exec();
                //EffectModule::req_follow(fighter.module_accessor, Hash40::new("mewtwo_final_aura"), Hash40::new("hip"), &Vector3f{x: 0.0, y: 0.0, z: 0.0}, &Vector3f{x: 0.0, y: 0.0, z: 0.0}, 1.0, true, 0, 0, 0, 0, 0, false, false);
            } else {
                fighter.sub_attack_air_inherit_jump_aerial_motion_uniq_process_exec_fix_pos();
            }
        }
    } else {
        return_value = true;
    }
    return return_value.into();
}

unsafe extern "C" fn skullkid_attackair_exit(fighter: &mut L2CFighterCommon) -> L2CValue {
    EffectModule::kill_kind(fighter.module_accessor, Hash40::new("mewtwo_final_aura"), false, true);
    fighter.sub_attack_air_inherit_jump_aerial_motion_uniq_process_exit();
    0.into()
}

unsafe extern "C" fn empty_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    0.into()
}

pub fn install() {
    Agent::new("ness")
        .on_line(Main, skullkid_frame)
        .status(Main, *FIGHTER_STATUS_KIND_JUMP_AERIAL, skullkid_status_JumpAerial_main)
        .status(Pre, *FIGHTER_STATUS_KIND_JUMP_AERIAL, skullkid_status_JumpAerial_Pre)
        .status(Exit, *FIGHTER_STATUS_KIND_JUMP_AERIAL, skullkid_status_JumpAerial_Exit)

        .effect_acmd("effect_jumpaerialfront", skullkid_effect_jumpaerial, Default)
        .effect_acmd("effect_jumpaerialback", skullkid_effect_jumpaerial, Default)
        

        //.status(Init, *FIGHTER_STATUS_KIND_ATTACK_AIR, empty_status)
        //.status(Exec, *FIGHTER_STATUS_KIND_ATTACK_AIR, empty_status)
        .status(Main, *FIGHTER_STATUS_KIND_ATTACK_AIR, skullkid_attackair_main)
        //.status(Exit, *FIGHTER_STATUS_KIND_ATTACK_AIR, empty_status)
        //.status(Exit, *FIGHTER_STATUS_KIND_ATTACK_AIR, skullkid_attackair_exit)


        .install();
}