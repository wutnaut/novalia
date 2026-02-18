use super::*;

unsafe extern "C" fn mrl_specialsend_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    WorkModule::enable_transition_term(fighter.module_accessor, FIGHTER_STATUS_TRANSITION_TERM_ID_WAIT);
    WorkModule::enable_transition_term(fighter.module_accessor, FIGHTER_STATUS_TRANSITION_TERM_ID_FALL);
    if fighter.global_table[0x16] != smash::app::SituationKind(*SITUATION_KIND_GROUND) {
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_LUIGI_SPECIAL_AIR_S_END);
        GroundModule::correct(fighter.module_accessor, *smash::app::GroundCorrectKind(GROUND_CORRECT_KIND_AIR));
        if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_LUIGI_STATUS_SPECIAL_S_END_FLAG_FIRST) {
            MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_air_s_end"), 0.0, 1.0, false, 0.0, false, false);
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_LUIGI_STATUS_SPECIAL_S_END_FLAG_FIRST);
        } else {
            MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new("special_air_s_end"), -1.0, 1.0, 0.0, false, false);
        }
        WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_FALL);
        WorkModule::unable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_WAIT);
        WorkModule::set_int(fighter.module_accessor, smash::app::SituationKind(*SITUATION_KIND_GROUND) as i32, *FIGHTER_LUIGI_STATUS_SPECIAL_S_END_INT_MTRANS);
    } else {
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_LUIGI_SPECIAL_S_END);
        GroundModule::correct(fighter.module_accessor, smash::app::GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
        if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_LUIGI_STATUS_SPECIAL_S_END_FLAG_FIRST) {
            MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_s_end"), 0.0, 1.0, false, 0.0, false, false);
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_LUIGI_STATUS_SPECIAL_S_END_FLAG_FIRST);
        } else {
            MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new("special_s_end"), -1.0, 1.0, 0.0, false, false);
        }
        sv_battle_object::notify_event_msc_cmd(fighter.lua_state_agent);
        WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_WAIT);
        WorkModule::unable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_FALL);
        WorkModule::set_int(fighter.module_accessor, smash::app::SituationKind(*SITUATION_KIND_AIR), *FIGHTER_LUIGI_STATUS_SPECIAL_S_END_INT_MTRANS);
    }
    fighter.sub_shift_status_main(L2CValue::Ptr(mrl_specialsend_main_loop as *const () as _));
    return 0.into();
}

unsafe extern "C" fn mrl_specialsend_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let trigger = false;
    if CancelModule::is_enable_cancel(fighter.module_accessor) != true {
// LAB_710000d864:
        if WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_WAIT) {
            if MotionModule::is_end(fighter.module_accessor) {
                fighter.change_status(*FIGHTER_STATUS_KIND_WAIT.into(), false.into());
                return 0.into();
            }
        }
        if WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_FALL) {
            if MotionModule::is_end(fighter.module_accessor) {
                fighter.change_status(*FIGHTER_STATUS_KIND_FALL_SPECIAL.into(), false.into());
                return 0.into();
            }
        }
        trigger = false;
        if StatusModule::is_changing(fighter.module_accessor) {
            if fighter.global_table[0x17] != WorkModule::get_int(fighter.module_accessor, *FIGHTER_LUIGI_STATUS_SPECIAL_S_END_INT_MTRANS) {
                if fighter.global_table[0x16] == WorkModule::get_int(fighter.module_accessor, *FIGHTER_LUIGI_STATUS_SPECIAL_S_END_INT_MTRANS) {
                    trigger = true;
                }
            }
        }
        if trigger {
            if fighter.global_table[0x16] != smash::app::SituationKind(*SITUATION_KIND_GROUND) {
                KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_LUIGI_SPECIAL_AIR_S_END);
                GroundModule::correct(fighter.module_accessor, smash::app::GroundCorrectKind(GROUND_CORRECT_KIND_AIR));
                if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_LUIGI_STATUS_SPECIAL_S_END_FLAG_FIRST) {
                    MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_air_s_end"), 0.0, 1.0, false, 0.0, false, false);
                    WorkModule::on_flag(fighter.module_accessor, *FIGHTER_LUIGI_STATUS_SPECIAL_S_END_FLAG_FIRST);
                } else {
                    MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new("special_air_s_end"), -1.0, 1.0, 0.0, false, false);
                }
                WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_FALL);
                WorkModule::unable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_WAIT);
                WorkModule::set_int(fighter.module_accessor, smash::app::SituationKind(*SITUATION_KIND_GROUND), *FIGHTER_LUIGI_STATUS_SPECIAL_S_END_INT_MTRANS);
            } else {
                KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_LUIGI_SPECIAL_S_END);
                GroundModule::correct(fighter.module_accessor, smash::app::GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
                if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_LUIGI_STATUS_SPECIAL_S_END_FLAG_FIRST) {
                    MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_s_end"), 0.0, 1.0, false, 0.0, false, false);
                    WorkModule::on_flag(fighter.module_accessor, *FIGHTER_LUIGI_STATUS_SPECIAL_S_END_FLAG_FIRST);
                } else {
                    MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new("special_s_end"), -1.0, 1.0, 0.0, false, false);
                }
                sv_battle_object::notify_event_msc_cmd(fighter.lua_state_agent);
                WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_WAIT);
                WorkModule::unable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_FALL);
                WorkModule::set_int(fighter.module_accessor, smash::app::SituationKind(*SITUATION_KIND_AIR), *FIGHTER_LUIGI_STATUS_SPECIAL_S_END_INT_MTRANS);
            }
        }
    } else {
        if fighter.sub_wait_ground_check_common(0x70).get_bool() {
        } else {
            if fighter.sub_air_check_fall_common().get_bool() == false {
                return 1.into();
                // goto LAB_710000d864;
            } 
        }
    }
    return 0.into();
}

pub fn install() {
    Agent::new("luigi")
        //.status(End, *FIGHTER_LUIGI_STATUS_KIND_SPECIAL_HI_DROP, mrl_specialhi_end)
        .status(Main, *FIGHTER_STATUS_KIND_SPECIAL_S_END, mrl_specialsend_main)
        .install();
}