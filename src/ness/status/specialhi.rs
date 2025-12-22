use super::*;

unsafe extern "C" fn skullkid_specialhi_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(fighter.module_accessor, smash::app::SituationKind(*SITUATION_KIND_NONE), *FIGHTER_KINETIC_TYPE_UNIQ, GROUND_CORRECT_KIND_KEEP.into(), smash::app::GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE), true, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLAG, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_INT, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLOAT, 0);
    //FighterStatusModuleImpl::set_fighter_status_data(fighter.module_accessor, false, FIGHTER_TREADED_KIND_NO_REAC, false, false, false, *FIGHTER_LOG_MASK_FLAG_ACTION_TRIGGER_ON | *FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_HI, 0, *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_HI.into(), 0);
    return 0.into();
}

unsafe extern "C" fn skullkid_specialhi_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.global_table[0x16] != *SITUATION_KIND_GROUND {
        GroundModule::correct(fighter.module_accessor, smash::app::GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_MEWTWO_SPECIAL_HI_AIR);
        MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_air_hi_start"), 0.0, 1.0, false, 0.0, false, false);
    } else {
        GroundModule::correct(fighter.module_accessor, smash::app::GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_GROUND_STOP);
        MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_hi_start"), 0.0, 1.0, false, 0.0, false, false);
    }
    if StopModule::is_stop(fighter.module_accessor) == false {
        //FUN_710000abe0(StopModule::is_stop(fighter.module_accessor), fighter, false);
    }
    fighter.clear_lua_stack();
    //fighter.push_lua_stack(&mut L2CValue::new_num(0x20cbc92683));
    fighter.push_lua_stack(&mut L2CValue::new_num(1.0));
    fighter.push_lua_stack(&mut L2CValue::new_num((*FIGHTER_LOG_DATA_INT_ATTACK_NUM_KIND) as f32));
    fighter.push_lua_stack(&mut L2CValue::new_num((*FIGHTER_LOG_ATTACK_KIND_ADDITIONS_ATTACK_01 + -1) as f32));
    sv_battle_object::notify_event_msc_cmd(fighter.lua_state_agent);
    fighter.sub_shift_status_main(L2CValue::Ptr(skullkid_specialhi_main_loop as *const () as _));
    return 0.into();
}

unsafe extern "C" fn skullkid_specialhi_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if !fighter.sub_transition_group_check_air_cliff().get_bool() {
        if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_MEWTWO_STATUS_SPECIAL_HI_FLAG_MOVE_WAIT) {
            if MotionModule::is_end(fighter.module_accessor) {
                //WorkModule::set_int(fighter.module_accessor, WorkModule::get_param_int(fighter.module_accessor, Hash40::new("param_special_hi"), Hash40::new("move_wait_frame")), *FIGHTER_MEWTWO_STATUS_SPECIAL_HI_WORK_INT_MOVE_WAIT_FRAME);
                WorkModule::on_flag(fighter.module_accessor, *FIGHTER_MEWTWO_STATUS_SPECIAL_HI_FLAG_MOVE_WAIT);
                let cliff_check = GroundModule::cliff_check(fighter.module_accessor) as i32;
                WorkModule::set_int(fighter.module_accessor, cliff_check, *FIGHTER_MEWTWO_STATUS_SPECIAL_HI_WORK_INT_CLIFF_CHECK);
                //FUN_7100009a70(fighter);
            }
        } else {
            if WorkModule::get_int(fighter.module_accessor, *FIGHTER_MEWTWO_STATUS_SPECIAL_HI_WORK_INT_MOVE_WAIT_FRAME) <= 0 {
                fighter.change_status(FIGHTER_MEWTWO_STATUS_KIND_SPECIAL_HI_2.into(), false.into());
            }
        }
    } else {
        return 1.into();
    }
    return 0.into();
}

unsafe extern "C" fn skullkid_specialhi_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.global_table[0xb] != FIGHTER_MEWTWO_STATUS_KIND_SPECIAL_HI_2 {
        VisibilityModule::set_whole(fighter.module_accessor, true);
    }
    return 0.into();
}

unsafe extern "C" fn skullkid_specialhi2_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(fighter.module_accessor, smash::app::SituationKind(*SITUATION_KIND_NONE), *FIGHTER_KINETIC_TYPE_UNIQ, GROUND_CORRECT_KIND_KEEP.into(), smash::app::GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE), false, *FIGHTER_STATUS_WORK_KEEP_FLAG_ALL_FLAG, *FIGHTER_STATUS_WORK_KEEP_FLAG_ALL_INT, *FIGHTER_STATUS_WORK_KEEP_FLAG_ALL_FLOAT, *FS_SUCCEEDS_KEEP_ATTACK);
    //FighterStatusModuleImpl::set_fighter_status_data(fighter.module_accessor, false, *FIGHTER_TREADED_KIND_NO_REAC, false, false, false, (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_HI).try_into().unwrap(), (*FIGHTER_STATUS_ATTR_DISABLE_DISSOLVE_CURSOR).try_into().unwrap() | *FIGHTER_STATUS_ATTR_HIDE_NAME_CURSOR, FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_HI.into(), 0);
    return 0.into();
}

unsafe extern "C" fn skullkid_specialhi2_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    //FUN_7100009a70();
    WorkModule::set_int(fighter.module_accessor, 0, *FIGHTER_MEWTWO_STATUS_SPECIAL_HI_WORK_INT_FRAME);

    if StopModule::is_stop(fighter.module_accessor) == false {
        //FUN_7100009c90(StopModule::is_stop(fighter.module_accessor), fighter, false);
    }
    fighter.sub_shift_status_main(L2CValue::Ptr(skullkid_specialhi2_main_loop as *const () as _));
    return 0.into();
}

unsafe extern "C" fn skullkid_specialhi2_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.sub_transition_group_check_air_cliff().get_bool() {
        return 1.into();
    }
    //if !(WorkModule::get_param_int(fighter.module_accessor, Hash40::new("param_special_hi"), Hash40::new("move_time")) <= WorkModule::get_int(fighter.module_accessor, *FIGHTER_MEWTWO_STATUS_SPECIAL_HI_WORK_INT_FRAME)) {
        if GroundModule::is_touch(fighter.module_accessor, GROUND_TOUCH_FLAG_ALL.into()) == true {
            //FUN_710000a720(GroundModule::is_touch(fighter.module_accessor, GROUND_TOUCH_FLAG_ALL.into()), fighter, GROUND_TOUCH_FLAG_UP);
            if GroundModule::is_touch(fighter.module_accessor, GROUND_TOUCH_FLAG_ALL.into()) != true {
                //FUN_710000a720(GroundModule::is_touch(fighter.module_accessor, GROUND_TOUCH_FLAG_ALL.into()), fighter, GROUND_TOUCH_FLAG_DOWN);
                if GroundModule::is_touch(fighter.module_accessor, GROUND_TOUCH_FLAG_ALL.into()) != true {
                    //FUN_710000a720(GroundModule::is_touch(fighter.module_accessor, GROUND_TOUCH_FLAG_ALL.into()), fighter, GROUND_TOUCH_FLAG_LEFT);
                    if GroundModule::is_touch(fighter.module_accessor, GROUND_TOUCH_FLAG_ALL.into()) != true {
                        //FUN_710000a720(GroundModule::is_touch(fighter.module_accessor, GROUND_TOUCH_FLAG_ALL.into()), fighter, GROUND_TOUCH_FLAG_RIGHT);
                        if GroundModule::is_touch(fighter.module_accessor, GROUND_TOUCH_FLAG_ALL.into()) != true {
                            if StatusModule::is_changing(fighter.module_accessor) == true {
                                return 0.into();
                            }
                        }
                    }
                }
            }
            return 0.into();
        }
        if StatusModule::is_changing(fighter.module_accessor) == true {
            if fighter.global_table[0x16] != *SITUATION_KIND_GROUND {
                GroundModule::correct(fighter.module_accessor, smash::app::GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
            } else {
                GroundModule::correct(fighter.module_accessor, smash::app::GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
            }
            return 0.into();
        }
        if fighter.global_table[0x17] == *SITUATION_KIND_GROUND {
            if fighter.global_table[0x16] == *SITUATION_KIND_AIR {
                if fighter.global_table[0x16] != *SITUATION_KIND_GROUND {
                GroundModule::correct(fighter.module_accessor, smash::app::GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
                } else {
                    GroundModule::correct(fighter.module_accessor, smash::app::GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
                }
                return 0.into();
            }
        }
        if fighter.global_table[0x17] == *SITUATION_KIND_GROUND {
            return 0.into();
        }
        if fighter.global_table[0x16] == *SITUATION_KIND_GROUND {
            if fighter.global_table[0x16] != *SITUATION_KIND_GROUND {
                GroundModule::correct(fighter.module_accessor, smash::app::GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
            } else {
                GroundModule::correct(fighter.module_accessor, smash::app::GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
            }
            return 0.into();
        }
    //} else {
        fighter.change_status(FIGHTER_MEWTWO_STATUS_KIND_SPECIAL_HI_3.into(), false.into());
    //}
    return 0.into();
}

unsafe extern "C" fn skullkid_specialhi2_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.global_table[0xb] != FIGHTER_MEWTWO_STATUS_KIND_SPECIAL_HI_3 {
        VisibilityModule::set_whole(fighter.module_accessor, true);
    }
    return 0.into();
}

unsafe extern "C" fn skullkid_specialhi3_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(fighter.module_accessor, smash::app::SituationKind(*SITUATION_KIND_NONE), *FIGHTER_KINETIC_TYPE_UNIQ, GROUND_CORRECT_KIND_KEEP.into(), smash::app::GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE), true, *FIGHTER_STATUS_WORK_KEEP_FLAG_ALL_FLAG, *FIGHTER_STATUS_WORK_KEEP_FLAG_ALL_INT, *FIGHTER_STATUS_WORK_KEEP_FLAG_ALL_FLOAT, 0);
    //FighterStatusModuleImpl::set_fighter_status_data(fighter.module_accessor, false, *FIGHTER_TREADED_KIND_NO_REAC, false, false, false, (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_HI).try_into().unwrap(), 0, *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_HI.into(), 0);
    return 0.into();
}

unsafe extern "C" fn skullkid_specialhi3_init(fighter: &mut L2CFighterCommon) -> L2CValue {
    KineticModule::clear_speed_all(fighter.module_accessor);
    if fighter.global_table[0x16] != *SITUATION_KIND_GROUND {
        //let wrap_xy_speed_air = WorkModule::get_param_float(fighter.module_accessor, Hash40::new("param_special_hi"), Hash40::new("wrap_xy_speed_air"));
        //let wrap_xy_speed = WorkModule::get_param_float(fighter.module_accessor, Hash40::new("param_special_hi"), Hash40::new("wrap_xy_speed"));
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_AIR_STOP_X_NORMAL_MAX);
        //fighter.clear_lua_stack();
        //fighter.push_lua_stack(&mut L2CValue::new_num((*FIGHTER_KINETIC_ENERGY_ID_STOP) as f32));
        //fighter.push_lua_stack(&mut L2CValue::new_num(sv_math::vec2_normalize(KineticModule::get_sum_speed_x(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_ALL), KineticModule::get_sum_speed_y(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_ALL)) * wrap_xy_speed_air));
        //fighter.push_lua_stack(&mut L2CValue::new_num((sv_math::vec2_normalize(KineticModule::get_sum_speed_x(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_ALL), KineticModule::get_sum_speed_y(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_ALL)) >> 0x20) * wrap_xy_speed_air));
        sv_kinetic_energy::set_speed(fighter.lua_state_agent);
        //fighter.clear_lua_stack();
        //fighter.push_lua_stack(&mut L2CValue::new_num((*FIGHTER_KINETIC_ENERGY_ID_STOP) as f32));
        //fighter.clear_lua_stack();
        //fighter.push_lua_stack(&mut L2CValue::new_num((*FIGHTER_KINETIC_ENERGY_ID_STOP) as f32));
        //fighter.push_lua_stack(&mut L2CValue::new_num(sv_kinetic_energy::get_limit_speed_x(fighter.lua_state_agent)));
        //fighter.push_lua_stack(&mut L2CValue::new_num(-1.0));
        sv_kinetic_energy::set_limit_speed(fighter.lua_state_agent);
        KineticModule::enable_energy(fighter.module_accessor, (*FIGHTER_KINETIC_ENERGY_ID_STOP) as i32);
        KineticModule::unable_energy(fighter.module_accessor, (*FIGHTER_KINETIC_ENERGY_ID_GRAVITY) as i32);
    } else {
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_GROUND_STOP);
        fighter.clear_lua_stack();
        //fighter.push_lua_stack(&mut L2CValue::new_num(wrap_xy_speed));
        //fighter.push_lua_stack(&mut L2CValue::new_num((*KINETIC_ENERGY_RESERVE_ATTRIBUTE_ALL) as f32 * wrap_xy_speed));
        fighter.push_lua_stack(&mut L2CValue::new_num(0.0));
        sv_kinetic_energy::set_speed(fighter.lua_state_agent);
        KineticModule::enable_energy(fighter.module_accessor, (*FIGHTER_KINETIC_ENERGY_ID_STOP) as i32);
    }
    VisibilityModule::set_whole(fighter.module_accessor, true);
    return 0.into();
}

unsafe extern "C" fn skullkid_specialhi3_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.global_table[0x16] != *SITUATION_KIND_GROUND {
        //let landing_frame = WorkModule::get_param_float(fighter.module_accessor, Hash40::new("param_special_hi"), Hash40::new("landing_frame"));
        //let fall_x_mull_value = WorkModule::get_param_float(fighter.module_accessor, Hash40::new("param_special_hi"), Hash40::new("fall_x_mull_value"));
        GroundModule::correct(fighter.module_accessor, smash::app::GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
        MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_air_hi"), 0.0, 1.0, false, 0.0, false, false);
       // WorkModule::set_float(fighter.module_accessor, landing_frame, *FIGHTER_INSTANCE_WORK_ID_FLOAT_LANDING_FRAME);
        //WorkModule::set_float(fighter.module_accessor, fall_x_mull_value, *FIGHTER_INSTANCE_WORK_ID_FLOAT_FALL_X_MAX_MUL);
        WorkModule::set_int(fighter.module_accessor, *SITUATION_KIND_AIR, *FIGHTER_MEWTWO_STATUS_SPECIAL_HI_WORK_INT_START_SITUATION);
    } else {
        GroundModule::correct(fighter.module_accessor, smash::app::GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
        MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_hi"), 0.0, 1.0, false, 0.0, false, false);
        WorkModule::set_int(fighter.module_accessor, *SITUATION_KIND_GROUND, *FIGHTER_MEWTWO_STATUS_SPECIAL_HI_WORK_INT_START_SITUATION);
    }
    fighter.sub_shift_status_main(L2CValue::Ptr(skullkid_specialhi3_main_loop as *const () as _));
    return 0.into();
}

unsafe extern "C" fn skullkid_specialhi3_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let mut return_value = 0;
    if !fighter.sub_transition_group_check_air_cliff().get_bool() {
        if CancelModule::is_enable_cancel(fighter.module_accessor) != true {
            if WorkModule::get_int(fighter.module_accessor, *FIGHTER_MEWTWO_STATUS_SPECIAL_HI_WORK_INT_START_SITUATION) != *SITUATION_KIND_GROUND {
                if MotionModule::is_end(fighter.module_accessor) != true {
                    if fighter.global_table[0x16] != *SITUATION_KIND_GROUND {
                    }
                    fighter.change_status(FIGHTER_STATUS_KIND_LANDING_FALL_SPECIAL.into(), false.into());
                } else {
                    fighter.change_status(FIGHTER_STATUS_KIND_FALL_SPECIAL.into(), false.into());
                }
            } else {
                if fighter.global_table[0x16] == *SITUATION_KIND_AIR {
                    fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
                }
                if MotionModule::is_end(fighter.module_accessor) == true {
                    fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(), false.into());
                }
                if fighter.global_table[0x8].get_bool() != false {
                    return_value = 0;
                } else {
                    if fighter.global_table[0x16] != *SITUATION_KIND_GROUND {
                        if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_MEWTWO_STATUS_SPECIAL_HI_FLAG_1) {
                            fighter.clear_lua_stack();
                            fighter.push_lua_stack(&mut L2CValue::new_num((*FIGHTER_KINETIC_ENERGY_ID_STOP) as f32));
                            fighter.clear_lua_stack();
                            fighter.push_lua_stack(&mut L2CValue::new_num((*FIGHTER_KINETIC_ENERGY_ID_STOP) as f32));
                            fighter.clear_lua_stack();
                            fighter.push_lua_stack(&mut L2CValue::new_num((*FIGHTER_KINETIC_ENERGY_ID_STOP) as f32));
                            let speed_x = sv_kinetic_energy::get_speed_x(fighter.lua_state_agent);
                            fighter.push_lua_stack(&mut L2CValue::new_num(speed_x));
                            fighter.push_lua_stack(&mut L2CValue::new_num((*FIGHTER_KINETIC_ENERGY_ID_STOP) as f32));
                            sv_kinetic_energy::set_speed(fighter.lua_state_agent);
                        } else {
                            fighter.clear_lua_stack();
                            fighter.push_lua_stack(&mut L2CValue::new_num((*FIGHTER_KINETIC_ENERGY_ID_GRAVITY) as f32));
                            let speed_y = KineticModule::get_sum_speed_y(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
                            fighter.push_lua_stack(&mut L2CValue::new_num(speed_y));
                            sv_kinetic_energy::set_speed(fighter.lua_state_agent);
                            KineticModule::enable_energy(fighter.module_accessor, (*FIGHTER_KINETIC_ENERGY_ID_GRAVITY) as i32);
                            //if !(KineticModule::get_sum_speed_x(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN) < WorkModule::get_param_float(fighter.module_accessor, Hash40::new("air_speed_x_stable"), 0) * WorkModule::get_param_float(fighter.module_accessor, Hash40::new("param_special_hi"), Hash40::new("wrap_x_max"))) {
                            //    if WorkModule::get_param_float(fighter.module_accessor, Hash40::new("air_speed_x_stable"), 0) * WorkModule::get_param_float(fighter.module_accessor, Hash40::new("param_special_hi"), Hash40::new("wrap_x_max")) < KineticModule::get_sum_speed_x(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN) {
                            //        new_var = WorkModule::get_param_float(fighter.module_accessor, Hash40::new("air_speed_x_stable"), 0) * WorkModule::get_param_float(fighter.module_accessor, Hash40::new("param_special_hi"), Hash40::new("wrap_x_max"));
                            //    }
                            //} else {
                            //    new_var = WorkModule::get_param_float(fighter.module_accessor, Hash40::new("air_speed_x_stable"), 0) * WorkModule::get_param_float(fighter.module_accessor, Hash40::new("param_special_hi"), Hash40::new("wrap_x_max"));
                           // }
                            //fighter.clear_lua_stack();
                            //fighter.push_lua_stack(&mut L2CValue::new_num((*FIGHTER_KINETIC_ENERGY_ID_STOP) as f32));
                            //fighter.push_lua_stack(&mut L2CValue::new_num(new_var));
                            //fighter.push_lua_stack(&mut L2CValue::new_num(0.0));
                            sv_kinetic_energy::set_speed(fighter.lua_state_agent);
                        }
                    }
                    return_value = 1;
                }
            }
            return 0.into();
        }
        if fighter.sub_wait_ground_check_common(false.into()).get_bool() != false {
        } else {
            if fighter.sub_air_check_fall_common().get_bool() == false {
                if WorkModule::get_int(fighter.module_accessor, *FIGHTER_MEWTWO_STATUS_SPECIAL_HI_WORK_INT_START_SITUATION) != *SITUATION_KIND_GROUND {
                    if MotionModule::is_end(fighter.module_accessor) != true {
                        if fighter.global_table[0x16] != *SITUATION_KIND_GROUND {
                        }
                        fighter.change_status(FIGHTER_STATUS_KIND_LANDING_FALL_SPECIAL.into(), false.into());
                    } else {
                        fighter.change_status(FIGHTER_STATUS_KIND_FALL_SPECIAL.into(), false.into());
                    }
                }
            } else {
                if fighter.global_table[0x16] == *SITUATION_KIND_AIR {
                    fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
                }
                if MotionModule::is_end(fighter.module_accessor) == true {
                    fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(), false.into());
                }

                if fighter.global_table[0x8].get_bool() != false {
                    return_value = 0;
                } else {
                    if fighter.global_table[0x16] != *SITUATION_KIND_GROUND {
                        if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_MEWTWO_STATUS_SPECIAL_HI_FLAG_1) {
                            fighter.clear_lua_stack();
                            fighter.push_lua_stack(&mut L2CValue::new_num((*FIGHTER_KINETIC_ENERGY_ID_STOP) as f32));
                            fighter.clear_lua_stack();
                            fighter.push_lua_stack(&mut L2CValue::new_num((*FIGHTER_KINETIC_ENERGY_ID_STOP) as f32));
                            fighter.clear_lua_stack();
                            fighter.push_lua_stack(&mut L2CValue::new_num((*FIGHTER_KINETIC_ENERGY_ID_STOP) as f32));
                            let speed_x = sv_kinetic_energy::get_speed_x(fighter.lua_state_agent);
                            fighter.push_lua_stack(&mut L2CValue::new_num(speed_x));
                            fighter.push_lua_stack(&mut L2CValue::new_num((*FIGHTER_KINETIC_ENERGY_ID_STOP) as f32));
                            sv_kinetic_energy::set_speed(fighter.lua_state_agent);
                        } else {
                            fighter.clear_lua_stack();
                            fighter.push_lua_stack(&mut L2CValue::new_num((*FIGHTER_KINETIC_ENERGY_ID_GRAVITY) as f32));
                            let speed_y = KineticModule::get_sum_speed_y(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
                            fighter.push_lua_stack(&mut L2CValue::new_num(speed_y));
                            sv_kinetic_energy::set_speed(fighter.lua_state_agent);
                            KineticModule::enable_energy(fighter.module_accessor, (*FIGHTER_KINETIC_ENERGY_ID_GRAVITY) as i32);
                            //if !(KineticModule::get_sum_speed_x(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN) < WorkModule::get_param_float(fighter.module_accessor, Hash40::new("air_speed_x_stable"), 0) * WorkModule::get_param_float(fighter.module_accessor, Hash40::new("param_special_hi"), Hash40::new("wrap_x_max"))) {
                            //    if WorkModule::get_param_float(fighter.module_accessor, Hash40::new("air_speed_x_stable"), 0) * WorkModule::get_param_float(fighter.module_accessor, Hash40::new("param_special_hi"), Hash40::new("wrap_x_max")) < KineticModule::get_sum_speed_x(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN) {
                            //        new_var = WorkModule::get_param_float(fighter.module_accessor, Hash40::new("air_speed_x_stable"), 0) * WorkModule::get_param_float(fighter.module_accessor, Hash40::new("param_special_hi"), Hash40::new("wrap_x_max"));
                            //    }
                            //} else {
                            //    new_var = WorkModule::get_param_float(fighter.module_accessor, Hash40::new("air_speed_x_stable"), 0) * WorkModule::get_param_float(fighter.module_accessor, Hash40::new("param_special_hi"), Hash40::new("wrap_x_max"));
                            //}
                            fighter.clear_lua_stack();
                            fighter.push_lua_stack(&mut L2CValue::new_num(((*FIGHTER_KINETIC_ENERGY_ID_STOP) as f32) as f32));
                            //fighter.push_lua_stack(&mut L2CValue::new_num(new_var));
                            fighter.push_lua_stack(&mut L2CValue::new_num(0.0));
                            sv_kinetic_energy::set_speed(fighter.lua_state_agent);
                        }
                    }
                    return_value = 1;
                }
            }
        }
    }
    return_value = 1;
    return return_value.into();
}


unsafe extern "C" fn skullkid_specialhi3_exec(fighter: &mut L2CFighterCommon) -> L2CValue {
    return 0.into();
}

unsafe extern "C" fn skullkid_specialhi3_exec_stop(fighter: &mut L2CFighterCommon) -> L2CValue {
    return 0.into();
}

unsafe extern "C" fn skullkid_specialhi3_exit(fighter: &mut L2CFighterCommon) -> L2CValue {
    return 0.into();
}

unsafe extern "C" fn skullkid_specialhi3_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.global_table[0xb] != FIGHTER_STATUS_KIND_LANDING_FALL_SPECIAL {
        if fighter.global_table[0xb] != FIGHTER_STATUS_KIND_FALL_SPECIAL {
            WorkModule::set_float(fighter.module_accessor, 1.0, *FIGHTER_INSTANCE_WORK_ID_FLOAT_FALL_X_MAX_MUL);
            WorkModule::set_float(fighter.module_accessor, 0.0, *FIGHTER_INSTANCE_WORK_ID_FLOAT_LANDING_FRAME);
        }
    }
    VisibilityModule::set_whole(fighter.module_accessor, true);
    return 0.into();
}

pub fn install() {
    Agent::new("ness")
        .status(Pre, FIGHTER_SKULLKID_STATUS_KIND_SPECIAL_HI, skullkid_specialhi_pre)
        .status(Main, FIGHTER_SKULLKID_STATUS_KIND_SPECIAL_HI, skullkid_specialhi_main)
        .status(End, FIGHTER_SKULLKID_STATUS_KIND_SPECIAL_HI, skullkid_specialhi_end)
        .status(Pre, FIGHTER_SKULLKID_STATUS_KIND_SPECIAL_HI2, skullkid_specialhi2_pre)
        .status(Main, FIGHTER_SKULLKID_STATUS_KIND_SPECIAL_HI2, skullkid_specialhi2_main)
        .status(End, FIGHTER_SKULLKID_STATUS_KIND_SPECIAL_HI2, skullkid_specialhi2_end)
        .status(Pre, FIGHTER_SKULLKID_STATUS_KIND_SPECIAL_HI3, skullkid_specialhi3_pre)
        .status(Main, FIGHTER_SKULLKID_STATUS_KIND_SPECIAL_HI3, skullkid_specialhi3_main)
        .status(End, FIGHTER_SKULLKID_STATUS_KIND_SPECIAL_HI3, skullkid_specialhi3_end)
        .status(Init, FIGHTER_SKULLKID_STATUS_KIND_SPECIAL_HI3, skullkid_specialhi3_init)
        .status(Exec, FIGHTER_SKULLKID_STATUS_KIND_SPECIAL_HI3, skullkid_specialhi3_exec)
        .status(Exit, FIGHTER_SKULLKID_STATUS_KIND_SPECIAL_HI3, skullkid_specialhi3_exit)
        .install();
}