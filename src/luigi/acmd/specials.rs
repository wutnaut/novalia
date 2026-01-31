use super::*;

//---------------SPECIALS--------------------

// GROUNDED SIDE SPECIAL 
unsafe extern "C" fn mrl_game_specials(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.module_accessor;
    let color = WorkModule::get_int(agent.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR);
    if color == 7 {
        frame(lua_state, 4.0);
        if macros::is_excute(agent) {
            macros::SA_SET(agent, *SITUATION_KIND_AIR);
            macros::CORRECT(agent, *GROUND_CORRECT_KIND_AIR);
            //KineticModule::change_kinetic(boma, *FIGHTER_KINETIC_TYPE_LUIGI_SPECIAL_S_RAM);
            KineticModule::change_kinetic(boma, *FIGHTER_KINETIC_TYPE_LUIGI_SPECIAL_AIR_S); // new
            let charge = 1.0 + WorkModule::get_float(agent.module_accessor, *FIGHTER_LUIGI_STATUS_SPECIAL_S_RAM_FLOAT_CHARGE) / 100.0;
            macros::SET_SPEED_EX(agent, 0.9 * charge, 1.55 * charge, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
        }
        frame(lua_state, 5.0);
        if macros::is_excute(agent) {
            macros::ATTACK(agent, 0, 0, Hash40::new("neck"), 10.0, 361, 80, 0, 30, 4.8, -1.0, 0.0, 0.0, None, None, None, 1.1, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_SPEED, false, 4, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_HEAD);
            AttackModule::set_attack_keep_rumble(boma, 0, true);
            JostleModule::set_status(boma, false);
            notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ON_DROP); // new - ledge check?
        }
        frame(lua_state, 6.0);
        if macros::is_excute(agent) {
            //WorkModule::on_flag(boma, *FIGHTER_LUIGI_STATUS_SPECIAL_S_RAM_FLAG_GROUND_CHECK);
        }
        frame(lua_state, 8.0);
        if macros::is_excute(agent) {
            macros::ATTACK(agent, 0, 0, Hash40::new("neck"), 10.0, 361, 80, 0, 30, 4.8, -1.0, 0.0, 0.0, None, None, None, 1.1, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_SPEED, false, 4, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_HEAD);
        }
        frame(lua_state, 14.0);
        if macros::is_excute(agent) {
            //WorkModule::enable_transition_term(boma, *FIGHTER_LUIGI_STATUS_SPECIAL_S_RAM_TRANSITION_TERM_ID_GROUND);
        }
        frame(lua_state, 20.0);
        if macros::is_excute(agent) {
            //notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ON_DROP);
            //WorkModule::on_flag(boma, *FIGHTER_LUIGI_STATUS_SPECIAL_S_RAM_FLAG_BRAKE);
        }
        frame(lua_state, 34.0);
        if macros::is_excute(agent) {
            AttackModule::clear_all(boma);
            WorkModule::on_flag(boma, *FIGHTER_STATUS_WORK_ID_FLAG_RESERVE_GRAVITY_STABLE_UNABLE);
        }
    } else {
        frame(lua_state, 4.0);
        if macros::is_excute(agent) {
            macros::SA_SET(agent, *SITUATION_KIND_AIR);
            KineticModule::change_kinetic(boma, *FIGHTER_KINETIC_TYPE_LUIGI_SPECIAL_S_RAM);
            macros::CORRECT(agent, *GROUND_CORRECT_KIND_AIR);
        }
        frame(lua_state, 5.0);
        if macros::is_excute(agent) {
            macros::ATTACK(agent, 0, 0, Hash40::new("neck"), 10.0, 361, 80, 0, 30, 4.8, -1.0, 0.0, 0.0, None, None, None, 1.1, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_SPEED, false, 4, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_HEAD);
            AttackModule::set_attack_keep_rumble(boma, 0, true);
            JostleModule::set_status(boma, false);
        }
        frame(lua_state, 6.0);
        if macros::is_excute(agent) {
            WorkModule::on_flag(boma, *FIGHTER_LUIGI_STATUS_SPECIAL_S_RAM_FLAG_GROUND_CHECK);
        }
        frame(lua_state, 8.0);
        if macros::is_excute(agent) {
            macros::ATTACK(agent, 0, 0, Hash40::new("neck"), 10.0, 361, 80, 0, 30, 4.8, -1.0, 0.0, 0.0, None, None, None, 1.1, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_SPEED, false, 4, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_HEAD);
        }
        frame(lua_state, 14.0);
        if macros::is_excute(agent) {
            WorkModule::enable_transition_term(boma, *FIGHTER_LUIGI_STATUS_SPECIAL_S_RAM_TRANSITION_TERM_ID_GROUND);
        }
        frame(lua_state, 20.0);
        if macros::is_excute(agent) {
            notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ON_DROP);
            WorkModule::on_flag(boma, *FIGHTER_LUIGI_STATUS_SPECIAL_S_RAM_FLAG_BRAKE);
        }
        frame(lua_state, 34.0);
        if macros::is_excute(agent) {
            AttackModule::clear_all(boma);
        }
    }
}

unsafe extern "C" fn mrl_effect_specials(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.module_accessor;
    let color = WorkModule::get_int(agent.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR);
    if color == 7 {
        if macros::is_excute(agent) {
            macros::EFFECT_FOLLOW_NO_STOP(agent, Hash40::new("luigi_rocket_jet"), Hash40::new("top"), 0, 4, 4, 0, 0, 0, 1, true);
            EffectModule::enable_sync_init_pos_last(boma);
        }
        frame(lua_state, 3.0);
        if macros::is_excute(agent) {
            macros::LANDING_EFFECT(agent, Hash40::new("sys_dash_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
        }
    } else {
        if macros::is_excute(agent) {
            macros::EFFECT_FOLLOW_NO_STOP(agent, Hash40::new("luigi_rocket_jet"), Hash40::new("top"), 0, 4, 4, 0, 0, 0, 1, true);
            EffectModule::enable_sync_init_pos_last(boma);
        }
        frame(lua_state, 3.0);
        if macros::is_excute(agent) {
            macros::LANDING_EFFECT(agent, Hash40::new("sys_dash_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
        }
    }
}

// AERIAL SIDE SPECIAL

// GROUNDED UP SPECIAL 
unsafe extern "C" fn mrl_game_specialhi(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.module_accessor;
    let color = WorkModule::get_int(agent.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR);
    if color == 7 {
        frame(lua_state, 3.0);
        if macros::is_excute(agent) {
            notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ON_DROP_BOTH_SIDES);
        }
        frame(lua_state, 8.0);
        if macros::is_excute(agent) {
            WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_SUPER_JUMP_PUNCH_FLAG_REVERSE_LR);
            macros::ATTACK(agent, 0, 0, Hash40::new("top"), 25.0, 0, 88, 0, 50, 2.2, 1.2, 6.0, 7.0, Some(-1.2), Some(6.0), Some(7.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_BAT, *ATTACK_REGION_PUNCH);
            WorkModule::on_flag(agent.module_accessor, *FIGHTER_LUIGI_STATUS_SPECIAL_HI_FLAG_CRITICAL_HIT);
        }
        wait(lua_state, 1.0);
        if macros::is_excute(agent) {
            macros::ATTACK(agent, 0, 0, Hash40::new("head"), 1.0, 80, 1, 0, 1, 5.8, 1.0, 2.2, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_coin"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_COIN, *ATTACK_REGION_PUNCH);
            macros::ATTACK(agent, 1, 0, Hash40::new("hip"), 1.0, 80, 1, 0, 1, 4.7, 0.0, 4.8, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_coin"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_COIN, *ATTACK_REGION_PUNCH);
            WorkModule::off_flag(boma, *FIGHTER_LUIGI_STATUS_SPECIAL_HI_FLAG_CRITICAL_HIT);
        }
        wait(lua_state, 1.0);
        if macros::is_excute(agent) {
            WorkModule::on_flag(boma, *FIGHTER_STATUS_SUPER_JUMP_PUNCH_FLAG_MOVE_TRANS);
        }
        wait(lua_state, 1.0);
        if macros::is_excute(agent) {
            macros::SA_SET(agent, *SITUATION_KIND_AIR);
        }
        frame(lua_state, 14.0);
        if macros::is_excute(agent) {
            AttackModule::clear_all(boma);
        }
        frame(lua_state, 20.0);
        if macros::is_excute(agent) {
            macros::ATTACK(agent, 2, 0, Hash40::new("handl"), 25.0, 0, 88, 0, 50, 2.2, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_BAT, *ATTACK_REGION_PUNCH);
            WorkModule::on_flag(boma, *FIGHTER_LUIGI_STATUS_SPECIAL_HI_FLAG_CRITICAL_HIT);
        }
        frame(lua_state, 22.0);
        if macros::is_excute(agent) {
            AttackModule::clear_all(boma);
        }
        frame(lua_state, 30.0);
        if macros::is_excute(agent) {
            notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ON_DROP_BOTH_SIDES);
        }
    } else {
        frame(lua_state, 3.0);
        if macros::is_excute(agent) {
            notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ON_DROP_BOTH_SIDES);
        }
        frame(lua_state, 8.0);
        if macros::is_excute(agent) {
            WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_SUPER_JUMP_PUNCH_FLAG_REVERSE_LR);
            macros::ATTACK(agent, 0, 0, Hash40::new("top"), 25.0, 88, 88, 0, 50, 2.2, 1.2, 6.0, 7.0, Some(-1.2), Some(6.0), Some(7.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_BAT, *ATTACK_REGION_PUNCH);
            WorkModule::on_flag(agent.module_accessor, *FIGHTER_LUIGI_STATUS_SPECIAL_HI_FLAG_CRITICAL_HIT);
        }
        wait(lua_state, 1.0);
        if macros::is_excute(agent) {
            macros::ATTACK(agent, 0, 0, Hash40::new("head"), 1.0, 80, 1, 0, 1, 5.8, 1.0, 2.2, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_coin"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_COIN, *ATTACK_REGION_PUNCH);
            macros::ATTACK(agent, 1, 0, Hash40::new("hip"), 1.0, 80, 1, 0, 1, 4.7, 0.0, 4.8, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_coin"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_COIN, *ATTACK_REGION_PUNCH);
            WorkModule::off_flag(boma, *FIGHTER_LUIGI_STATUS_SPECIAL_HI_FLAG_CRITICAL_HIT);
        }
        wait(lua_state, 1.0);
        if macros::is_excute(agent) {
            WorkModule::on_flag(boma, *FIGHTER_STATUS_SUPER_JUMP_PUNCH_FLAG_MOVE_TRANS);
        }
        wait(lua_state, 1.0);
        if macros::is_excute(agent) {
            macros::SA_SET(agent, *SITUATION_KIND_AIR);
        }
        frame(lua_state, 24.0);
        if macros::is_excute(agent) {
            AttackModule::clear_all(boma);
        }
        frame(lua_state, 30.0);
        if macros::is_excute(agent) {
            notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ON_DROP_BOTH_SIDES);
        }
    }
}

// AERIAL UP SPECIAL

// GROUNDED DOWN SPECIAL 
unsafe extern "C" fn mrl_game_specialairlw(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.module_accessor;
    let color = WorkModule::get_int(agent.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR);
    if color == 7 {
        frame(lua_state, 10.0);
        if macros::is_excute(agent) {
            WorkModule::on_flag(boma, *FIGHTER_LUIGI_STATUS_SPECIAL_LW_FLAG_RISE);
            macros::ATTACK(agent, 1, 0, Hash40::new("top"), 8.0, 75, 100, 0, 40, 5.5, 0.0, 10.0, -5.5, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_BODY);
            macros::ATTACK(agent, 2, 0, Hash40::new("top"), 8.0, 75, 100, 0, 40, 5.5, 0.0, 10.0, 5.5, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_BODY);
            macros::ATTACK(agent, 0, 0, Hash40::new("top"), 8.0, 75, 100, 0, 40, 6.0, 0.0, 5.5, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_BODY);
        }
        frame(lua_state, 34.0);
        macros::FT_MOTION_RATE(agent, 0.6);
        if macros::is_excute(agent) {
            AttackModule::clear_all(boma);
        }
        frame(lua_state, 44.0);
        macros::FT_MOTION_RATE(agent, 1.0);
        if macros::is_excute(agent) {
            WorkModule::on_flag(boma, *FIGHTER_LUIGI_STATUS_SPECIAL_LW_FLAG_LIMIT_X_DEC);
            macros::ATTACK(agent, 0, 0, Hash40::new("top"), 10.0, 90, 100, 0, 60, 6.0, 0.0, 10.0, -8.5, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BODY);
            macros::ATTACK(agent, 1, 0, Hash40::new("top"), 10.0, 90, 100, 0, 60, 6.0, 0.0, 10.0, 8.5, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BODY);
            macros::ATTACK(agent, 2, 0, Hash40::new("top"), 10.0, 90, 100, 0, 60, 6.5, 0.0, 5.5, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BODY);
        }
        frame(lua_state, 48.0);
        if macros::is_excute(agent) {
            WorkModule::off_flag(boma, *FIGHTER_LUIGI_STATUS_SPECIAL_LW_FLAG_RISE);
            AttackModule::clear_all(boma);
        }
        frame(lua_state, 60.0);
        macros::FT_MOTION_RATE(agent, 0.2);
        
    } else {
        frame(lua_state, 10.0);
        if macros::is_excute(agent) {
            WorkModule::on_flag(boma, *FIGHTER_LUIGI_STATUS_SPECIAL_LW_FLAG_RISE);
            macros::ATTACK(agent, 1, 0, Hash40::new("top"), 8.0, 75, 100, 0, 40, 5.5, 0.0, 10.0, -5.5, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_BODY);
            macros::ATTACK(agent, 2, 0, Hash40::new("top"), 8.0, 75, 100, 0, 40, 5.5, 0.0, 10.0, 5.5, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_BODY);
            macros::ATTACK(agent, 0, 0, Hash40::new("top"), 8.0, 75, 100, 0, 40, 6.0, 0.0, 5.5, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_BODY);
        }
        frame(lua_state, 34.0);
        macros::FT_MOTION_RATE(agent, 0.6);
        if macros::is_excute(agent) {
            AttackModule::clear_all(boma);
        }
        frame(lua_state, 44.0);
        macros::FT_MOTION_RATE(agent, 1.0);
        if macros::is_excute(agent) {
            WorkModule::on_flag(boma, *FIGHTER_LUIGI_STATUS_SPECIAL_LW_FLAG_LIMIT_X_DEC);
            macros::ATTACK(agent, 0, 0, Hash40::new("top"), 10.0, 90, 100, 0, 60, 6.0, 0.0, 10.0, -8.5, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BODY);
            macros::ATTACK(agent, 1, 0, Hash40::new("top"), 10.0, 90, 100, 0, 60, 6.0, 0.0, 10.0, 8.5, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BODY);
            macros::ATTACK(agent, 2, 0, Hash40::new("top"), 10.0, 90, 100, 0, 60, 6.5, 0.0, 5.5, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BODY);
        }
        frame(lua_state, 48.0);
        if macros::is_excute(agent) {
            WorkModule::off_flag(boma, *FIGHTER_LUIGI_STATUS_SPECIAL_LW_FLAG_RISE);
            AttackModule::clear_all(boma);
        }
    }
}

pub fn install() {
    Agent::new("luigi")
        .game_acmd("game_specialhi", mrl_game_specialhi, Default)
        .game_acmd("game_specialairhi", mrl_game_specialhi, Default)
        .game_acmd("game_specials", mrl_game_specials, Default)
        .effect_acmd("effect_specials", mrl_effect_specials, Default)
        .game_acmd("game_specialairlw", mrl_game_specialairlw, Default)
        .install();
}