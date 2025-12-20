use super::*;

const WEAPON_SHADOWBALLDASH_STATUS_KIND_REGULAR: i32 = 0;

unsafe extern "C" fn luigifireball_start_pre(weapon: &mut L2CWeaponCommon) -> L2CValue {
    StatusModule::init_settings(weapon.module_accessor, smash::app::SituationKind(*SITUATION_KIND_AIR), *WEAPON_KINETIC_TYPE_NORMAL, GROUND_CORRECT_KIND_AIR.into(), smash::app::GroundCliffCheckKind(0), false, 0, 0, 0, 0);
    return 0.into();
}

unsafe extern "C" fn luigifireball_start_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.sub_shift_status_main(L2CValue::Ptr(luigifireball_start_main_loop as *const () as _));
    return 0.into();
}

unsafe extern "C" fn luigifireball_start_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if StatusModule::is_changing(fighter.module_accessor) != true {
        if fighter.global_table[0x17] == SITUATION_KIND_GROUND {
            if fighter.global_table[0x16] == SITUATION_KIND_AIR {
                if fighter.global_table[0x16] != SITUATION_KIND_GROUND {
                    KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_FALL);
                    GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));

                    if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_SPECIAL_N_FLAG_FIRST) {
                        MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_air_n"), 0.0, 1.0, false, 0.0, false, false);
                        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_SPECIAL_N_FLAG_FIRST);
                    } else {
                        MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new("special_air_n"), -1.0, 1.0, 0.0, false, false);
                    }
                } else {
                    KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_GROUND_STOP);
                    GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP_ATTACK));

                    if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_SPECIAL_N_FLAG_FIRST) {
                        MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_n"), 0.0, 1.0, false, 0.0, false, false);
                        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_SPECIAL_N_FLAG_FIRST);
                    } else {
                        MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new("special_n"), -1.0, 1.0, 0.0, false, false);
                    }
                }
            }
        }
        if fighter.global_table[0x17] == SITUATION_KIND_GROUND {
            if CancelModule::is_enable_cancel(fighter.module_accessor) == true {
                if fighter.global_table[0x16] == SITUATION_KIND_GROUND {
                    if fighter.sub_wait_ground_check_common(false.into()).get_bool() {
                        return 0.into()
                    }
                }
                if fighter.global_table[0x16] == SITUATION_KIND_AIR {
                    if fighter.sub_air_check_fall_common().get_bool() {
                        return 0.into()
                    }
                }
            }
            if MotionModule::is_end(fighter.module_accessor) {
                if fighter.global_table[0x16] != SITUATION_KIND_GROUND {
                    if fighter.global_table[0x16] != SITUATION_KIND_AIR {
                        return 0.into();
                    }
                    fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
                } else {
                    fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(), false.into());
                }
            }
        }
        if fighter.global_table[0x16] == SITUATION_KIND_GROUND {
            if fighter.global_table[0x16] != SITUATION_KIND_GROUND {
                KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_FALL);
                GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));

                if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_SPECIAL_N_FLAG_FIRST) {
                    MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_air_n"), 0.0, 1.0, false, 0.0, false, false);
                    WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_SPECIAL_N_FLAG_FIRST);
                } else {
                    MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new("special_air_n"), -1.0, 1.0, 0.0, false, false);
                }
            } else {
                KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_GROUND_STOP);
                GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP_ATTACK));

                if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_SPECIAL_N_FLAG_FIRST) {
                    MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_n"), 0.0, 1.0, false, 0.0, false, false);
                    WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_SPECIAL_N_FLAG_FIRST);
                } else {
                    MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new("special_n"), -1.0, 1.0, 0.0, false, false);
                }
            }   
        }
    } else {
        if fighter.global_table[0x16] != SITUATION_KIND_GROUND {
            KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_FALL);
            GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));

            if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_SPECIAL_N_FLAG_FIRST) {
                MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_air_n"), 0.0, 1.0, false, 0.0, false, false);
                WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_SPECIAL_N_FLAG_FIRST);
            } else {
                MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new("special_air_n"), -1.0, 1.0, 0.0, false, false);
            }
        } else {
            KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_GROUND_STOP);
            GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP_ATTACK));

            if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_SPECIAL_N_FLAG_FIRST) {
                MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_n"), 0.0, 1.0, false, 0.0, false, false);
                WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_SPECIAL_N_FLAG_FIRST);
            } else {
                MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new("special_n"), -1.0, 1.0, 0.0, false, false);
            }
        }
    }
    if CancelModule::is_enable_cancel(fighter.module_accessor) == true {
        if fighter.global_table[0x16] == SITUATION_KIND_GROUND {
            if fighter.sub_wait_ground_check_common(false.into()).get_bool() {
                return 0.into()
            }
        }
        if fighter.global_table[0x16] == SITUATION_KIND_AIR {
            if fighter.sub_air_check_fall_common().get_bool() {
                return 0.into()
            }
        }
    }
    if MotionModule::is_end(fighter.module_accessor) {
        if fighter.global_table[0x16] != SITUATION_KIND_GROUND {
            if fighter.global_table[0x16] != SITUATION_KIND_AIR {
                return 0.into();
            }
            fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
        } else {
            fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(), false.into());
        }
    }
    return 0.into();
}


unsafe extern "C" fn luigifireball_start_end(weapon: &mut L2CWeaponCommon) -> L2CValue {
    return 0.into();
}

pub fn install() {
    Agent::new("ness_shadowballdash")
        .status(Pre, WEAPON_SHADOWBALLDASH_STATUS_KIND_REGULAR, luigifireball_start_pre)
        .status(Main, WEAPON_SHADOWBALLDASH_STATUS_KIND_REGULAR, luigifireball_start_main)
        .status(End, WEAPON_SHADOWBALLDASH_STATUS_KIND_REGULAR, luigifireball_start_end)
        .install();
}