use super::*;

//---------------SPECIALS--------------------

// GROUNDED SIDE SPECIAL 

// AERIAL SIDE SPECIAL

// GROUNDED UP SPECIAL 

unsafe extern "C" fn slippy_effect_specialhi(agent: &mut L2CAgentBase) {
    let color = WorkModule::get_int(agent.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR);
    if color == 7 {
        frame(agent.lua_state_agent, 1.0);
        if macros::is_excute(agent) {
            macros::EFFECT(agent, Hash40::new("fox_firefox_start"), Hash40::new("waist"), -8, 0, 0, 0, 90, 10, 1, 0, 0, 0, 0, 0, 0, true);
            macros::LAST_EFFECT_SET_COLOR(agent, 0.0, 1.0, 0.0);
            macros::EFFECT_FOLLOW_NO_STOP(agent, Hash40::new("fox_firefox"), Hash40::new("rot"), 0.75, -2.8, 2, 90, 0, 0, 1, true);
            macros::LAST_EFFECT_SET_COLOR(agent, 0.0, 1.0, 0.0);
            EffectModule::enable_sync_init_pos_last(agent.module_accessor);
        }
        if macros::is_excute(agent) {
            macros::BURN_COLOR(agent, 2, 0.1, 0, 0.5);
        }
        wait(agent.lua_state_agent, 2.0);
        if macros::is_excute(agent) {
            macros::BURN_COLOR_FRAME(agent, 2, 1, 0.2, 0.1, 0);
        }
        wait(agent.lua_state_agent, 1.0);
        if macros::is_excute(agent) {
            macros::BURN_COLOR_NORMAL(agent);
        }
        wait(agent.lua_state_agent, 1.0);
    } else {
        frame(agent.lua_state_agent, 1.0);
        if macros::is_excute(agent) {
            macros::EFFECT(agent, Hash40::new("fox_firefox_start"), Hash40::new("waist"), -8, 0, 0, 0, 90, 10, 1, 0, 0, 0, 0, 0, 0, true);
            macros::EFFECT_FOLLOW_NO_STOP(agent, Hash40::new("fox_firefox"), Hash40::new("rot"), 0.75, -2.8, 2, 90, 0, 0, 1, true);
            EffectModule::enable_sync_init_pos_last(agent.module_accessor);
        }
        if macros::is_excute(agent) {
            macros::BURN_COLOR(agent, 2, 0.1, 0, 0.5);
        }
        wait(agent.lua_state_agent, 2.0);
        if macros::is_excute(agent) {
            macros::BURN_COLOR_FRAME(agent, 2, 1, 0.2, 0.1, 0);
        }
        wait(agent.lua_state_agent, 1.0);
        if macros::is_excute(agent) {
            macros::BURN_COLOR_NORMAL(agent);
        }
        wait(agent.lua_state_agent, 1.0);
    }
}

unsafe extern "C" fn slippy_effect_specialhibound(agent: &mut L2CAgentBase) {
    let color = WorkModule::get_int(agent.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR);
    if color == 7 {
        if macros::is_excute(agent) {
            macros::LANDING_EFFECT(agent, Hash40::new("sys_crown"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
            macros::BURN_COLOR(agent, 2, 0.1, 0, 0.4);
        }
        frame(agent.lua_state_agent, 1.0);
        if macros::is_excute(agent) {
            macros::EFFECT(agent, Hash40::new("sys_damage_fire"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1.1, 0, 0, 0, 0, 0, 0, true);
            macros::LAST_EFFECT_SET_COLOR(agent, 0.0, 1.0, 0.0);
        }
        frame(agent.lua_state_agent, 6.0);
        if macros::is_excute(agent) {
            macros::EFFECT(agent, Hash40::new("sys_damage_fire"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1.1, 0, 0, 0, 0, 0, 0, true);
            macros::LAST_EFFECT_SET_COLOR(agent, 0.0, 1.0, 0.0);
        }
        frame(agent.lua_state_agent, 11.0);
        if macros::is_excute(agent) {
            macros::EFFECT(agent, Hash40::new("sys_damage_fire"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1.1, 0, 0, 0, 0, 0, 0, true);
            macros::LAST_EFFECT_SET_COLOR(agent, 0.0, 1.0, 0.0);
            macros::BURN_COLOR_FRAME(agent, 2, 1, 0.2, 0.1, 0);
        }
        frame(agent.lua_state_agent, 12.0);
        if macros::is_excute(agent) {
            macros::BURN_COLOR_NORMAL(agent);
        }
    } else {
        if macros::is_excute(agent) {
            macros::LANDING_EFFECT(agent, Hash40::new("sys_crown"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
            macros::BURN_COLOR(agent, 2, 0.1, 0, 0.4);
        }
        frame(agent.lua_state_agent, 1.0);
        if macros::is_excute(agent) {
            macros::EFFECT(agent, Hash40::new("sys_damage_fire"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1.1, 0, 0, 0, 0, 0, 0, true);
        }
        frame(agent.lua_state_agent, 6.0);
        if macros::is_excute(agent) {
            macros::EFFECT(agent, Hash40::new("sys_damage_fire"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1.1, 0, 0, 0, 0, 0, 0, true);
        }
        frame(agent.lua_state_agent, 11.0);
        if macros::is_excute(agent) {
            macros::EFFECT(agent, Hash40::new("sys_damage_fire"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1.1, 0, 0, 0, 0, 0, 0, true);
            macros::BURN_COLOR_FRAME(agent, 2, 1, 0.2, 0.1, 0);
        }
        frame(agent.lua_state_agent, 12.0);
        if macros::is_excute(agent) {
            macros::BURN_COLOR_NORMAL(agent);
        }
    }
}

unsafe extern "C" fn slippy_effect_specialhifall(agent: &mut L2CAgentBase) {
    let color = WorkModule::get_int(agent.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR);
    if color == 7 {
        if macros::is_excute(agent) {
            macros::BURN_COLOR(agent, 2, 0.1, 0, 0.4);
            macros::EFFECT(agent, Hash40::new("sys_damage_fire"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1.1, 0, 0, 0, 0, 0, 0, true);
            macros::LAST_EFFECT_SET_COLOR(agent, 0.0, 1.0, 0.0);
            EffectModule::enable_sync_init_pos_last(agent.module_accessor);
        }
        frame(agent.lua_state_agent, 10.0);
        if macros::is_excute(agent) {
            macros::EFFECT(agent, Hash40::new("sys_damage_fire"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1.1, 0, 0, 0, 0, 0, 0, true);
            macros::LAST_EFFECT_SET_COLOR(agent, 0.0, 1.0, 0.0);
            EffectModule::enable_sync_init_pos_last(agent.module_accessor);
            macros::BURN_COLOR_FRAME(agent, 2, 1, 0.2, 0.1, 0);
        }
        frame(agent.lua_state_agent, 11.0);
        if macros::is_excute(agent) {
            macros::BURN_COLOR_NORMAL(agent);
        }
    } else {
        if macros::is_excute(agent) {
            macros::BURN_COLOR(agent, 2, 0.1, 0, 0.4);
            macros::EFFECT(agent, Hash40::new("sys_damage_fire"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1.1, 0, 0, 0, 0, 0, 0, true);
            EffectModule::enable_sync_init_pos_last(agent.module_accessor);
        }
        frame(agent.lua_state_agent, 10.0);
        if macros::is_excute(agent) {
            macros::EFFECT(agent, Hash40::new("sys_damage_fire"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1.1, 0, 0, 0, 0, 0, 0, true);
            EffectModule::enable_sync_init_pos_last(agent.module_accessor);
            macros::BURN_COLOR_FRAME(agent, 2, 1, 0.2, 0.1, 0);
        }
        frame(agent.lua_state_agent, 11.0);
        if macros::is_excute(agent) {
            macros::BURN_COLOR_NORMAL(agent);
        }
    }
}

unsafe extern "C" fn slippy_effect_specialhihold(agent: &mut L2CAgentBase) {
    let color = WorkModule::get_int(agent.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR);
    if color == 7 {
        frame(agent.lua_state_agent, 1.0);
        if macros::is_excute(agent) {
            macros::EFFECT_FOLLOW_NO_STOP(agent, Hash40::new("fox_firefox_hold"), Hash40::new("top"), 0, 0.5, 0, 0, 0, 0, 1, true);
            macros::LAST_EFFECT_SET_COLOR(agent, 0.0, 1.0, 0.0);
        }
        if macros::is_excute(agent) {
            macros::LANDING_EFFECT(agent, Hash40::new("sys_down_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.73, 0, 0, 0, 0, 0, 0, false);
            macros::LAST_EFFECT_SET_RATE(agent, 1.5);
            macros::BURN_COLOR(agent, 2, 0.1, 0, 0.7);
        }
        wait(agent.lua_state_agent, 2.0);
        if macros::is_excute(agent) {
            macros::BURN_COLOR_FRAME(agent, 2, 1, 0.2, 0.1, 0);
        }
        wait(agent.lua_state_agent, 3.0);
        if macros::is_excute(agent) {
            macros::BURN_COLOR_NORMAL(agent);
            macros::FLASH(agent, 1, 0.5, 0.3, 0.5);
        }
        wait(agent.lua_state_agent, 1.0);
        if macros::is_excute(agent) {
            macros::COL_NORMAL(agent);
        }
        wait(agent.lua_state_agent, 1.0);
    } else {
        frame(agent.lua_state_agent, 1.0);
        if macros::is_excute(agent) {
            macros::EFFECT_FOLLOW_NO_STOP(agent, Hash40::new("fox_firefox_hold"), Hash40::new("top"), 0, 0.5, 0, 0, 0, 0, 1, true);
        }
        if macros::is_excute(agent) {
            macros::LANDING_EFFECT(agent, Hash40::new("sys_down_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.73, 0, 0, 0, 0, 0, 0, false);
            macros::LAST_EFFECT_SET_RATE(agent, 1.5);
            macros::BURN_COLOR(agent, 2, 0.1, 0, 0.7);
        }
        wait(agent.lua_state_agent, 2.0);
        if macros::is_excute(agent) {
            macros::BURN_COLOR_FRAME(agent, 2, 1, 0.2, 0.1, 0);
        }
        wait(agent.lua_state_agent, 3.0);
        if macros::is_excute(agent) {
            macros::BURN_COLOR_NORMAL(agent);
            macros::FLASH(agent, 1, 0.5, 0.3, 0.5);
        }
        wait(agent.lua_state_agent, 1.0);
        if macros::is_excute(agent) {
            macros::COL_NORMAL(agent);
        }
        wait(agent.lua_state_agent, 1.0);
    }
}

unsafe extern "C" fn slippy_effect_specialhiholdair(agent: &mut L2CAgentBase) {
    let color = WorkModule::get_int(agent.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR);
    if color == 7 {
        frame(agent.lua_state_agent, 1.0);
        if macros::is_excute(agent) {
            macros::EFFECT_FOLLOW_NO_STOP(agent, Hash40::new("fox_firefox_hold"), Hash40::new("top"), 0, -2, 0, 0, 0, 0, 1, true);
            macros::LAST_EFFECT_SET_COLOR(agent, 0.0, 1.0, 0.0);
        }
        if macros::is_excute(agent) {
            macros::BURN_COLOR(agent, 2, 0.1, 0, 0.7);
        }
        wait(agent.lua_state_agent, 2.0);
        if macros::is_excute(agent) {
            macros::BURN_COLOR_FRAME(agent, 2, 1, 0.2, 0.1, 0);
        }
        wait(agent.lua_state_agent, 3.0);
        if macros::is_excute(agent) {
            macros::BURN_COLOR_NORMAL(agent);
            macros::FLASH(agent, 1, 0.5, 0.3, 0.5);
        }
        wait(agent.lua_state_agent, 1.0);
        if macros::is_excute(agent) {
            macros::COL_NORMAL(agent);
        }
        wait(agent.lua_state_agent, 1.0);
    } else {
        frame(agent.lua_state_agent, 1.0);
        if macros::is_excute(agent) {
            macros::EFFECT_FOLLOW_NO_STOP(agent, Hash40::new("fox_firefox_hold"), Hash40::new("top"), 0, -2, 0, 0, 0, 0, 1, true);
        }
        if macros::is_excute(agent) {
            macros::BURN_COLOR(agent, 2, 0.1, 0, 0.7);
        }
        wait(agent.lua_state_agent, 2.0);
        if macros::is_excute(agent) {
            macros::BURN_COLOR_FRAME(agent, 2, 1, 0.2, 0.1, 0);
        }
        wait(agent.lua_state_agent, 3.0);
        if macros::is_excute(agent) {
            macros::BURN_COLOR_NORMAL(agent);
            macros::FLASH(agent, 1, 0.5, 0.3, 0.5);
        }
        wait(agent.lua_state_agent, 1.0);
        if macros::is_excute(agent) {
            macros::COL_NORMAL(agent);
        }
        wait(agent.lua_state_agent, 1.0);
    }
}

unsafe extern "C" fn slippy_effect_specialhilanding(agent: &mut L2CAgentBase) {
    let color = WorkModule::get_int(agent.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR);
    if color == 7 {
        if macros::is_excute(agent) {
            macros::EFFECT(agent, Hash40::new("sys_damage_fire"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1.1, 0, 0, 0, 0, 0, 0, true);
            macros::LAST_EFFECT_SET_COLOR(agent, 0.0, 1.0, 0.0);
            EffectModule::enable_sync_init_pos_last(agent.module_accessor);
            macros::BURN_COLOR(agent, 2, 0.1, 0, 0.3);
        }
        wait(agent.lua_state_agent, 7.0);
        if macros::is_excute(agent) {
            macros::BURN_COLOR_FRAME(agent, 2, 1, 0.2, 0.1, 0);
        }
        wait(agent.lua_state_agent, 1.0);
        if macros::is_excute(agent) {
            macros::BURN_COLOR_NORMAL(agent);
        }
        frame(agent.lua_state_agent, 8.0);
        if macros::is_excute(agent) {
            macros::EFFECT(agent, Hash40::new("sys_damage_fire"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1.1, 0, 0, 0, 0, 0, 0, true);
            macros::LAST_EFFECT_SET_COLOR(agent, 0.0, 1.0, 0.0);
            macros::LANDING_EFFECT(agent, Hash40::new("sys_dash_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.7, 0, 0, 0, 0, 0, 0, false);
        }
    } else {
        if macros::is_excute(agent) {
            macros::EFFECT(agent, Hash40::new("sys_damage_fire"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1.1, 0, 0, 0, 0, 0, 0, true);
            EffectModule::enable_sync_init_pos_last(agent.module_accessor);
            macros::BURN_COLOR(agent, 2, 0.1, 0, 0.3);
        }
        wait(agent.lua_state_agent, 7.0);
        if macros::is_excute(agent) {
            macros::BURN_COLOR_FRAME(agent, 2, 1, 0.2, 0.1, 0);
        }
        wait(agent.lua_state_agent, 1.0);
        if macros::is_excute(agent) {
            macros::BURN_COLOR_NORMAL(agent);
        }
        frame(agent.lua_state_agent, 8.0);
        if macros::is_excute(agent) {
            macros::EFFECT(agent, Hash40::new("sys_damage_fire"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1.1, 0, 0, 0, 0, 0, 0, true);
            macros::LANDING_EFFECT(agent, Hash40::new("sys_dash_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.7, 0, 0, 0, 0, 0, 0, false);
        }
    }
}

// AERIAL UP SPECIAL

// GROUNDED DOWN SPECIAL 

unsafe extern "C" fn slippy_game_speciallwstart(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
	let boma = agent.module_accessor;
    let slippy_speed_x = KineticModule::get_sum_speed_x(boma, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN).abs();
    let color = WorkModule::get_int(agent.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR);
    if color == 7 {
        frame(agent.lua_state_agent, 2.0);
        if macros::is_excute(agent) {
            KineticModule::clear_speed_all(agent.module_accessor);
        }
        frame(agent.lua_state_agent, 3.0);
        if macros::is_excute(agent) {
            macros::ATTACK(agent, 0, 0, Hash40::new("top"), 2.0, 361, 32, 0, 66, 7.5, 0.0, 6.5, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_ENERGY);
            macros::ATTACK(agent, 1, 0, Hash40::new("top"), 2.0, 361, 45, 0, 66, 7.5, 0.0, 6.5, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_ENERGY);
            KineticModule::add_speed(agent.module_accessor, &Vector3f{ x: 0.0, y: slippy_speed_x * 3.0, z: 0.0 });
        }
    } else {
        frame(agent.lua_state_agent, 3.0);
        if macros::is_excute(agent) {
            macros::ATTACK(agent, 0, 0, Hash40::new("top"), 2.0, 10, 32, 0, 66, 7.5, 0.0, 6.5, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_ENERGY);
            macros::ATTACK(agent, 1, 0, Hash40::new("top"), 2.0, 24, 45, 0, 66, 7.5, 0.0, 6.5, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_ENERGY);
        }
    }
}

unsafe extern "C" fn slippy_effect_speciallwstart(agent: &mut L2CAgentBase) {
    let color = WorkModule::get_int(agent.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR);
    if color == 7 {
        if macros::is_excute(agent) {
            macros::FOOT_EFFECT(agent, Hash40::new("null"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
            macros::EFFECT_FOLLOW(agent, Hash40::new("fox_ref_flash"), Hash40::new("reflector"), 1.2, 0, -0.5, 0, 0, 0, 1, true);
        }
        frame(agent.lua_state_agent, 2.0);
        if macros::is_excute(agent) {
            macros::EFFECT_FOLLOW(agent, Hash40::new("fox_ref_start"), Hash40::new("top"), 0, 6, 0, 0, 0, 0, 0.75, false);
            macros::LAST_EFFECT_SET_COLOR(agent, 0.0, 1.0, 0.0);
        }
    } else {
        if macros::is_excute(agent) {
            macros::FOOT_EFFECT(agent, Hash40::new("null"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
            macros::EFFECT_FOLLOW(agent, Hash40::new("fox_ref_flash"), Hash40::new("reflector"), 1.2, 0, -0.5, 0, 0, 0, 1, true);
        }
        frame(agent.lua_state_agent, 2.0);
        if macros::is_excute(agent) {
            macros::EFFECT_FOLLOW(agent, Hash40::new("fox_ref_start"), Hash40::new("top"), 0, 6, 0, 0, 0, 0, 0.75, false);
        }
    }
}

unsafe extern "C" fn slippy_effect_speciallwloop(agent: &mut L2CAgentBase) {
    let color = WorkModule::get_int(agent.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR);
    if color == 7 {
        frame(agent.lua_state_agent, 1.0);
        //get_value_int(*SO_VAR_INT_CURRENT_STATUS);
        //if(0x10d030(0, *FIGHTER_FOX_STATUS_KIND_SPECIAL_LW_LOOP)){
            if !WorkModule::is_flag(agent.module_accessor, *FIGHTER_FOX_REFLECTOR_STATUS_WORK_ID_FLAG_SET_EFFECT) {
                if macros::is_excute(agent) {
                    macros::EFFECT_FOLLOW(agent, Hash40::new("fox_ref_loop"), Hash40::new("top"), 0, 6, 0, 0, 0, 0, 1, true);
                    macros::LAST_EFFECT_SET_COLOR(agent, 0.0, 1.0, 0.0);
                    macros::EFFECT_FOLLOW(agent, Hash40::new("fox_ref_flash"), Hash40::new("reflector"), 1.2, 0, -0.5, 0, 0, 0, 1, true);
                    macros::LAST_EFFECT_SET_COLOR(agent, 0.0, 1.0, 0.0);
                    WorkModule::on_flag(agent.module_accessor, *FIGHTER_FOX_REFLECTOR_STATUS_WORK_ID_FLAG_SET_EFFECT);
                }
            //}
        }
        if macros::is_excute(agent) {
            macros::FLASH(agent, 0, 1, 0, 0.25);
        }
        wait(agent.lua_state_agent, 2.0);
        if macros::is_excute(agent) {
            macros::COL_NORMAL(agent);
        }
        wait(agent.lua_state_agent, 1.0);
    } else {
        frame(agent.lua_state_agent, 1.0);
        //get_value_int(*SO_VAR_INT_CURRENT_STATUS);
        //if(0x10d030(0, *FIGHTER_FOX_STATUS_KIND_SPECIAL_LW_LOOP)){
            if !WorkModule::is_flag(agent.module_accessor, *FIGHTER_FOX_REFLECTOR_STATUS_WORK_ID_FLAG_SET_EFFECT) {
                if macros::is_excute(agent) {
                    macros::EFFECT_FOLLOW(agent, Hash40::new("fox_ref_loop"), Hash40::new("top"), 0, 6, 0, 0, 0, 0, 1, true);
                    macros::EFFECT_FOLLOW(agent, Hash40::new("fox_ref_flash"), Hash40::new("reflector"), 1.2, 0, -0.5, 0, 0, 0, 1, true);
                    WorkModule::on_flag(agent.module_accessor, *FIGHTER_FOX_REFLECTOR_STATUS_WORK_ID_FLAG_SET_EFFECT);
                }
            //}
        }
        if macros::is_excute(agent) {
            macros::FLASH(agent, 0, 0.5, 1, 0.25);
        }
        wait(agent.lua_state_agent, 2.0);
        if macros::is_excute(agent) {
            macros::COL_NORMAL(agent);
        }
        wait(agent.lua_state_agent, 1.0);
    }
}

unsafe extern "C" fn slippy_effect_speciallwhit(agent: &mut L2CAgentBase) {
    let color = WorkModule::get_int(agent.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR);
    if color == 7 {
        if macros::is_excute(agent) {
            macros::EFFECT_OFF_KIND(agent, Hash40::new("fox_ref_loop"), true, true);
            macros::LAST_EFFECT_SET_COLOR(agent, 0.0, 1.0, 0.0);
            macros::EFFECT_FOLLOW(agent, Hash40::new("fox_ref_start"), Hash40::new("top"), 0, 6, 0, 0, 0, 0, 1, true);
            macros::LAST_EFFECT_SET_COLOR(agent, 0.0, 1.0, 0.0);
            macros::EFFECT_FOLLOW(agent, Hash40::new("fox_ref_ref"), Hash40::new("top"), 0, 6, 0, 0, 0, 0, 1, true);
            macros::LAST_EFFECT_SET_COLOR(agent, 0.0, 1.0, 0.0);
            macros::EFFECT_FOLLOW(agent, Hash40::new("fox_ref_flash"), Hash40::new("reflector"), 1.2, 0, -0.5, 0, 0, 0, 1.5, true);
            macros::LAST_EFFECT_SET_COLOR(agent, 0.0, 1.0, 0.0);
        }
        frame(agent.lua_state_agent, 2.0);
        if macros::is_excute(agent) {
            macros::EFFECT_FOLLOW(agent, Hash40::new("fox_ref_loop"), Hash40::new("top"), 0, 6, 0, 0, 0, 0, 1, true);
            macros::LAST_EFFECT_SET_COLOR(agent, 0.0, 1.0, 0.0);
        }
        frame(agent.lua_state_agent, 6.0);
        if macros::is_excute(agent) {
            macros::EFFECT_FOLLOW(agent, Hash40::new("fox_ref_ref"), Hash40::new("top"), 0, 6, 0, 0, 0, 0, 1, true);
            macros::LAST_EFFECT_SET_COLOR(agent, 0.0, 1.0, 0.0);
            macros::EFFECT_FOLLOW(agent, Hash40::new("fox_ref_flash"), Hash40::new("reflector"), 1.2, 0, -0.5, 0, 0, 0, 1.5, true);
            macros::LAST_EFFECT_SET_COLOR(agent, 0.0, 1.0, 0.0);
        }
    } else {
        if macros::is_excute(agent) {
            macros::EFFECT_OFF_KIND(agent, Hash40::new("fox_ref_loop"), true, true);
            macros::EFFECT_FOLLOW(agent, Hash40::new("fox_ref_start"), Hash40::new("top"), 0, 6, 0, 0, 0, 0, 1, true);
            macros::EFFECT_FOLLOW(agent, Hash40::new("fox_ref_ref"), Hash40::new("top"), 0, 6, 0, 0, 0, 0, 1, true);
            macros::EFFECT_FOLLOW(agent, Hash40::new("fox_ref_flash"), Hash40::new("reflector"), 1.2, 0, -0.5, 0, 0, 0, 1.5, true);
        }
        frame(agent.lua_state_agent, 2.0);
        if macros::is_excute(agent) {
            macros::EFFECT_FOLLOW(agent, Hash40::new("fox_ref_loop"), Hash40::new("top"), 0, 6, 0, 0, 0, 0, 1, true);
        }
        frame(agent.lua_state_agent, 6.0);
        if macros::is_excute(agent) {
            macros::EFFECT_FOLLOW(agent, Hash40::new("fox_ref_ref"), Hash40::new("top"), 0, 6, 0, 0, 0, 0, 1, true);
            macros::EFFECT_FOLLOW(agent, Hash40::new("fox_ref_flash"), Hash40::new("reflector"), 1.2, 0, -0.5, 0, 0, 0, 1.5, true);
        }
    }
}

// AERIAL DOWN SPECIAL

pub fn install() {
    Agent::new("fox")
        //.game_acmd("game_specials", game_specials, Default)
        //.game_acmd("game_specialairs", game_specialairs, Default)
        //.game_acmd("game_specialhi", game_specialhi, Default)
        //.effect_acmd("effect_specialhi", effect_specialhi, Default)
        //.sound_acmd("sound_specialhi", sound_specialhi, Default)
        //.game_acmd("game_specialairhi", game_specialairhi, Default)
        //.effect_acmd("effect_specialairhi", effect_specialairhi, Default)
        //.sound_acmd("sound_specialairhi", sound_specialairhi, Default)
        .game_acmd("game_speciallwstart", slippy_game_speciallwstart, Default)
        //.effect_acmd("effect_speciallwstart", slippy_effect_speciallwstart, Default)
        //.effect_acmd("effect_speciallwloop", slippy_effect_speciallwloop, Default)
        //.effect_acmd("effect_speciallwhit", slippy_effect_speciallwhit, Default)
        .game_acmd("game_specialairlwstart", slippy_game_speciallwstart, Default)
        //.effect_acmd("effect_specialairlwstart", slippy_effect_speciallwstart, Default)
        //.effect_acmd("effect_specialairlwloop", slippy_effect_speciallwloop, Default)
        
        //.effect_acmd("effect_specialhi", slippy_effect_specialhi, Default)
        //.effect_acmd("effect_specialhibound", slippy_effect_specialhibound, Default)
        //.effect_acmd("effect_specialhifall", slippy_effect_specialhifall, Default)
        //.effect_acmd("effect_specialhihold", slippy_effect_specialhihold, Default)
        //.effect_acmd("effect_specialhiholdair", slippy_effect_specialhiholdair, Default)
        //.effect_acmd("effect_specialhilanding", slippy_effect_specialhilanding, Default)

        //.game_acmd("game_specialairlwhold", game_specialairlwhold, Default)
        .install();
}