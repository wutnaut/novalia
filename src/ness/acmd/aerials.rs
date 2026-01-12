use super::*;

//--------------------AERIAL ATTACKS------------------------

// FORWARD AERIAL
unsafe extern "C" fn skullkid_game_attackairf(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.module_accessor;
    let color = WorkModule::get_int(agent.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR);
    if color == 6 || color == 7 {
        frame(agent.lua_state_agent, 2.0);
        if macros::is_excute(agent) {
            WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
        }
        frame(agent.lua_state_agent, 9.0);
        if macros::is_excute(agent) {
            macros::ATTACK(agent, 0, 0, Hash40::new("footl"), 8.0, 361, 85, 0, 10, 3.3, 0.0, 2.9, 0.0, None, None, None, 0.8, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
            macros::ATTACK(agent, 1, 0, Hash40::new("toel"), 10.0, 42, 94, 0, 30, 5.0, 3.0, 0.4, 0.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PSI);
        }
        wait(agent.lua_state_agent, 2.0);
        if macros::is_excute(agent) {
            AttackModule::clear(agent.module_accessor, 0, false);
        }
        frame(agent.lua_state_agent, 13.0);
        if macros::is_excute(agent) {
            AttackModule::clear_all(agent.module_accessor);
        }
        frame(agent.lua_state_agent, 34.0);
        if macros::is_excute(agent) {
            WorkModule::off_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
        }
    } else {
        frame(lua_state, 1.0);
        //MotionModule::set_rate(boma, (8.0 - 1.0) / 13.0);
        frame(lua_state, 2.0);
        if macros::is_excute(agent) {
            WorkModule::on_flag(boma, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
        }
        frame(lua_state, 8.0);
        MotionModule::set_rate(boma, 0.6);
        if macros::is_excute(agent) {
            macros::ATTACK(agent, 0, 0, Hash40::new("top"), 2.0, 361, 0, 40, 29, 2.5, 0.0, 3.8, 6.0, Some(0.0), Some(3.8), Some(3.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 5, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_none"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_PSI);
            macros::ATTACK(agent, 1, 0, Hash40::new("top"), 3.0, 367, 0, 40, 29, 6.5, 0.0, 5.3, 11.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 5, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_PSI);
            for _i in 0..3 {
                macros::EFFECT_FOLLOW(agent, Hash40::new("ness_psi_rush"), Hash40::new("top"), 0, 5, 10.2, 90, 0, 90, 1.6, true);
                macros::EFFECT_FOLLOW(agent, Hash40::new("ness_psi_rush"), Hash40::new("top"), 0, 5, 10.2, 90, 0, 90, 1.6, true);
                macros::LAST_EFFECT_SET_COLOR(agent, 0.0, 0.0, 1.0);
                macros::LAST_EFFECT_SET_ALPHA(agent, 0.5);
                wait(lua_state, 2.0);
            }
        }
        frame(lua_state, 14.0);
        MotionModule::set_rate(boma, 0.8); //(47.0-12.0)/29.0);
        if macros::is_excute(agent) {
            AttackModule::clear_all(boma);
            macros::ATTACK(agent, 1, 0, Hash40::new("top"), 8.0, 361, 80, 0, 55, 6.5, 0.0, 5.3, 11.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_PSI);
            macros::EFFECT_FOLLOW(agent, Hash40::new("sys_hit_purple"), Hash40::new("top"), 0, 5, 10.2, 90, 0, 90, 0.4, true);
            macros::LAST_EFFECT_SET_RATE(agent, 1.5);
            macros::LAST_EFFECT_SET_ALPHA(agent, 0.4);
            macros::EFFECT_FOLLOW(agent, Hash40::new("sys_hit_purple"), Hash40::new("top"), 0, 5, 10.2, 90, 0, 90, 0.4, true);
            macros::LAST_EFFECT_SET_RATE(agent, 1.5);
            macros::LAST_EFFECT_SET_COLOR(agent, 0.0, 0.0, 1.0);
            macros::LAST_EFFECT_SET_ALPHA(agent, 0.8);
        }
        frame(lua_state, 18.0);
        MotionModule::set_rate(boma, 1.0);
        if macros::is_excute(agent) {
            AttackModule::clear_all(boma);
        }
        frame(lua_state, 20.0);
        if macros::is_excute(agent) {
            WorkModule::off_flag(boma, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
        }
    }
}

// FORWARD AERIAL EFFECT
unsafe extern "C" fn skullkid_effect_attackairf(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.module_accessor;
    let color = WorkModule::get_int(agent.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR);
    if color == 6 || color == 7 {
        frame(agent.lua_state_agent, 9.0);
        if macros::is_excute(agent) {
            macros::EFFECT_FOLLOW(agent, Hash40::new("ness_psi_atk"), Hash40::new("top"), 0, 5, 12.5, 0, 0, 0, 0.85, true);
        }
        frame(agent.lua_state_agent, 14.0);
        if macros::is_excute(agent) {
            macros::EFFECT_OFF_KIND(agent, Hash40::new("ness_psi_atk"), false, false);
        }
    }
}

// BACK AERIAL
unsafe extern "C" fn skullkid_game_attackairb(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.module_accessor;
    let color = WorkModule::get_int(agent.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR);
    if color == 6 || color == 7 {
        frame(agent.lua_state_agent, 3.0);
        if macros::is_excute(agent) {
            WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
        }
        frame(agent.lua_state_agent, 15.0);
        if macros::is_excute(agent) {
            macros::ATTACK(agent, 0, 0, Hash40::new("kneer"), 9.0, 361, 80, 0, 30, 3.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_PSI);
            macros::ATTACK(agent, 1, 0, Hash40::new("kneer"), 12.0, 280, 90, 0, 30, 4.5, 4.5, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_PSI);
            macros::ATTACK(agent, 2, 0, Hash40::new("kneer"), 12.0, 280, 90, 0, 30, 4.5, 4.5, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_PSI);
            macros::ATTACK(agent, 3, 0, Hash40::new("kneer"), 7.0, 361, 60, 0, 10, 3.5, 8.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_PSI);
        }
        frame(agent.lua_state_agent, 19.0);
        if macros::is_excute(agent) {
            macros::ATTACK(agent, 0, 0, Hash40::new("kneer"), 9.0, 361, 80, 0, 30, 3.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_PART, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_PSI);
            macros::ATTACK(agent, 2, 0, Hash40::new("kneer"), 12.0, 361, 90, 0, 30, 4.5, 4.5, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_PART, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_PSI);
            macros::ATTACK(agent, 3, 0, Hash40::new("kneer"), 7.0, 361, 60, 0, 10, 3.5, 8.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_PART, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_PSI);
        }
        frame(agent.lua_state_agent, 20.0);
        if macros::is_excute(agent) {
            AttackModule::clear(agent.module_accessor, 1, false);
        }
        frame(agent.lua_state_agent, 23.0);
        if macros::is_excute(agent) {
            AttackModule::clear_all(agent.module_accessor);
        }
        frame(agent.lua_state_agent, 34.0);
        if macros::is_excute(agent) {
            WorkModule::off_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
        }
    } else {
        frame(lua_state, 1.0);
        MotionModule::set_rate(boma, (10.0 - 1.0) / 4.0);
        frame(lua_state, 10.0);
        MotionModule::set_rate(boma, (12.0 - 10.0) / 6.0);
        if macros::is_excute(agent) {
            WorkModule::on_flag(boma, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
            macros::ATTACK(agent, 0, 0, Hash40::new("kneer"), 10.0, 361, 100, 0, 20, 4.0, -1.0, 0.0, 1.5, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_PSI);
            macros::ATTACK(agent, 1, 0, Hash40::new("kneer"), 16.0, 361, 105, 0, 35, 5.0, 3.7, 2.0, 1.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_PSI);
        }
        frame(lua_state, 12.0);
        MotionModule::set_rate(boma, (26.0 - 12.0) / 10.0);
        if macros::is_excute(agent) {
            AttackModule::clear_all(boma);
        }
        frame(lua_state, 26.0);
        if macros::is_excute(agent) {
            WorkModule::off_flag(boma, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
        }
    }
}

// BACK AERIAL EFFECT
unsafe extern "C" fn skullkid_effect_attackairb(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.module_accessor;
    let color = WorkModule::get_int(agent.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR);
    if color == 6 || color == 7 {
        //frame(agent.lua_state_agent, 9.0);
        //if macros::is_excute(agent) {
            //macros::EFFECT(agent, Hash40::new("sys_smash_flash"), Hash40::new("kneer"), 6.5, 0, 0, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, true);
          //  EffectModule::set_scale_last(agent.module_accessor, &Vector3f{x: 0.5, y: 0.5, z: 0.5});
        //}
        frame(agent.lua_state_agent, 15.0);
        if macros::is_excute(agent) {
            macros::EFFECT_FOLLOW(agent, Hash40::new("ness_psi_atk"), Hash40::new("kneer"), 4.7, 0, 0, 0, 90, 0, 0.65, true);
            macros::LAST_EFFECT_SET_RATE(agent, 1.6);
            EffectModule::set_scale_last(agent.module_accessor, &Vector3f{x: 0.8, y: 0.8, z: 0.8});
        }
        frame(agent.lua_state_agent, 21.0);
        if macros::is_excute(agent) {
            macros::EFFECT_FOLLOW(agent, Hash40::new("ness_psi_atk"), Hash40::new("kneer"), 7, 0, 0, 0, 90, 0, 0.85, true);
            macros::LAST_EFFECT_SET_RATE(agent, 1.6);
        }
    }
}

// UP AERIAL
unsafe extern "C" fn skullkid_game_attackairhi(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let color = WorkModule::get_int(agent.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR);
    let entry_id = WorkModule::get_int(agent.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    if color == 6 || color == 7 {
        frame(agent.lua_state_agent, 8.0);
        if macros::is_excute(agent) {
            WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
            macros::ATTACK(agent, 0, 0, Hash40::new("handr"), 8.0, 70, 80, 0, 50, 5.5, 4.5, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_PSI);
            macros::ATTACK(agent, 1, 0, Hash40::new("handr"), 8.0, 70, 80, 0, 50, 3.5, -2.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_PSI);
        }
        frame(agent.lua_state_agent, 15.0);
        if macros::is_excute(agent) {
            macros::ATTACK(agent, 0, 0, Hash40::new("shoulderr"), 8.0, 60, 80, 0, 50, 6.5, 3.0, 0.0, 0.0, Some(5.0), Some(0.0), Some(0.0), 1.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_PSI);
            macros::ATTACK(agent, 1, 0, Hash40::new("handr"), 8.0, 70, 80, 0, 50, 3.5, -3.0, 0.0, 1.0, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_PSI);
        }
        frame(agent.lua_state_agent, 17.0);
        if macros::is_excute(agent) {
            AttackModule::clear_all(agent.module_accessor);
        }
        macros::FT_MOTION_RATE(agent, 0.85);
        frame(agent.lua_state_agent, 37.0);
        if macros::is_excute(agent) {
            WorkModule::off_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
        }
    } else {
        frame(agent.lua_state_agent, 8.0);
        if macros::is_excute(agent) {
            WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
            macros::ATTACK(agent, 0, 0, Hash40::new("handr"), 2.5, 366, 100, 30, 0, 5.5, 4.5, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 2, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_PSI);
            macros::ATTACK(agent, 1, 0, Hash40::new("handr"), 2.5, 366, 100, 30, 0, 3.5, -2.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 2, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_PSI);
        }
        frame(agent.lua_state_agent, 15.0);
        if macros::is_excute(agent) {
            macros::ATTACK(agent, 0, 0, Hash40::new("shoulderr"), 5.0, 73, 180, 0, 40, 6.5, 3.0, 0.0, 0.0, Some(5.0), Some(0.0), Some(0.0), 1.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 2, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_PSI);
            macros::ATTACK(agent, 1, 0, Hash40::new("handr"), 5.0, 73, 180, 0, 40, 3.5, -3.0, 0.0, 1.0, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 2, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_PSI);
        }
        frame(agent.lua_state_agent, 17.0);
        if macros::is_excute(agent) {
            AttackModule::clear_all(agent.module_accessor);
        }
        macros::FT_MOTION_RATE(agent, 0.85);
        frame(agent.lua_state_agent, 37.0);
        if macros::is_excute(agent) {
            WorkModule::off_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
        }
    }
}

// NEUTRAL AERIAL
unsafe extern "C" fn skullkid_game_attackairn(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.module_accessor;
    let color = WorkModule::get_int(agent.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR);
    let entry_id = WorkModule::get_int(agent.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    if color == 6 || color == 7 {
        frame(agent.lua_state_agent, 1.0);
        macros::FT_MOTION_RATE(agent, 2.8);
        if macros::is_excute(agent) {
            KineticModule::clear_speed_all(agent.module_accessor);
        }
        frame(agent.lua_state_agent, 5.0);
        if macros::is_excute(agent) {
            WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
            //if allow_nairboost[entry_id] {
            let mut xCtrl = ControlModule::get_stick_x(agent.module_accessor);
            let mut yCtrl = ControlModule::get_stick_y(agent.module_accessor);
            if xCtrl != 0.0 || yCtrl != 0.0 {
                if !ControlModule::check_button_on(agent.module_accessor, *CONTROL_PAD_BUTTON_JUMP) && nairboosts[entry_id] > 0 { 
                    nairboosts[entry_id] = nairboosts[entry_id] - 1;
                    if yCtrl <= 0.66 {
                        yCtrl = 0.66;
                    }
                    if xCtrl > 0.66 {
                        xCtrl = 0.66;
                    }
                    if PostureModule::lr(agent.module_accessor) == 1.0 {
                        KineticModule::add_speed(agent.module_accessor, &Vector3f{ x: xCtrl * 1.1, y: yCtrl * 2.5, z: 0.0 });
                    } else {
                        KineticModule::add_speed(agent.module_accessor, &Vector3f{ x: xCtrl * -1.1, y: yCtrl * 2.5, z: 0.0 });
                    }
                }
            }
            macros::ATTACK(agent, 0, 0, Hash40::new("handr"), 8.0, 90, 80, 0, 20, 4.2, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_PSI);
            macros::ATTACK(agent, 1, 0, Hash40::new("handl"), 8.0, 90, 80, 0, 20, 4.2, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_PSI);
            macros::ATTACK(agent, 2, 0, Hash40::new("hip"), 6.0, 90, 80, 0, 20, 5.4, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_PSI);
        }
        wait(agent.lua_state_agent, 3.0);
        macros::FT_MOTION_RATE(agent, 1.0);
        if macros::is_excute(agent) {
            AttackModule::clear_all(agent.module_accessor);
        }
        wait(agent.lua_state_agent, 1.0);
        if macros::is_excute(agent) {
            macros::ATTACK(agent, 0, 0, Hash40::new("handr"), 10.0, 361, 90, 0, 30, 4.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_PSI);
            macros::ATTACK(agent, 1, 0, Hash40::new("handl"), 10.0, 361, 90, 0, 30, 4.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_PSI);
            macros::ATTACK(agent, 2, 0, Hash40::new("hip"), 7.0, 90, 80, 0, 20, 5.4, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_PSI);
        }
        wait(agent.lua_state_agent, 8.0);
        if macros::is_excute(agent) {
            AttackModule::clear_all(agent.module_accessor);
        }
        wait(agent.lua_state_agent, 3.0);
        if macros::is_excute(agent) {
            CancelModule::enable_cancel(agent.module_accessor);
        }
        frame(agent.lua_state_agent, 26.0);
        if macros::is_excute(agent) {
            WorkModule::off_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
        }
    } else {
        frame(agent.lua_state_agent, 5.0);
        if macros::is_excute(agent) {
            WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
            macros::ATTACK(agent, 0, 0, Hash40::new("handr"), 11.0, 361, 100, 0, 20, 4.2, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_PSI);
            macros::ATTACK(agent, 1, 0, Hash40::new("handl"), 11.0, 361, 100, 0, 20, 4.2, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_PSI);
            macros::ATTACK(agent, 2, 0, Hash40::new("hip"), 11.0, 361, 100, 0, 20, 5.4, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_PSI);
        }
        wait(agent.lua_state_agent, 8.0);
        if macros::is_excute(agent) {
            macros::ATTACK(agent, 0, 0, Hash40::new("handr"), 7.0, 361, 100, 0, 10, 4.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_PSI);
            macros::ATTACK(agent, 1, 0, Hash40::new("handl"), 7.0, 361, 100, 0, 10, 4.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_PSI);
            macros::ATTACK(agent, 2, 0, Hash40::new("hip"), 7.0, 361, 100, 0, 10, 5.5, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_PSI);
        }
        frame(agent.lua_state_agent, 16.0);
        if macros::is_excute(agent) {
            AttackModule::clear_all(agent.module_accessor);
        }
        frame(agent.lua_state_agent, 26.0);
        if macros::is_excute(agent) {
            WorkModule::off_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
        }
    }
}

// DOWN AERIAL
unsafe extern "C" fn skullkid_game_attackairlw(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.module_accessor;
    let color = WorkModule::get_int(agent.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR);
    if color == 6 || color == 7 {
        //frame(agent.lua_state_agent, 1.0);
        //macros::FT_MOTION_RATE(agent, 0.895);
        frame(agent.lua_state_agent, 20.0);
        //macros::FT_MOTION_RATE(agent, 1.0);
        if macros::is_excute(agent) {
            WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
            macros::ATTACK(agent, 0, 0, Hash40::new("top"), 14.0, 270, 74, 0, 37, 6.0, 0.0, -2.0, 0.0, None, None, None, 1.25, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PSI);
            //macros::ATTACK(agent, 1, 0, Hash40::new("top"), 14.0, 270, 74, 0, 37, 6.0, 0.0, -2.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_paralyze"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PSI);
        }
        wait(agent.lua_state_agent, 3.0);
        if macros::is_excute(agent) {
            //AttackModule::clear_all(agent.module_accessor);
            //AttackModule::clear(boma, 0, false);
            macros::ATTACK(agent, 0, 0, Hash40::new("top"), 12.0, 361, 70, 0, 30, 6.0, 0.0, -2.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_none"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PSI);
        }
        wait(agent.lua_state_agent, 4.0);
        if macros::is_excute(agent) {
            AttackModule::clear_all(agent.module_accessor);
        }
        frame(agent.lua_state_agent, 41.0);
        if macros::is_excute(agent) {
            WorkModule::off_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
        }
    } else {
        frame(agent.lua_state_agent, 1.0);
        macros::FT_MOTION_RATE(agent, 0.895);
        frame(agent.lua_state_agent, 20.0);
        macros::FT_MOTION_RATE(agent, 1.0);
        if macros::is_excute(agent) {
            WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
            macros::ATTACK(agent, 0, 0, Hash40::new("top"), 14.0, 270, 74, 0, 37, 6.0, 0.0, -2.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PSI);
        }
        wait(agent.lua_state_agent, 3.0);
        if macros::is_excute(agent) {
            macros::ATTACK(agent, 0, 0, Hash40::new("top"), 12.0, 45, 74, 0, 30, 6.0, 0.0, -2.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PSI);
        }
        wait(agent.lua_state_agent, 4.0);
        if macros::is_excute(agent) {
            AttackModule::clear_all(agent.module_accessor);
        }
        frame(agent.lua_state_agent, 41.0);
        if macros::is_excute(agent) {
            WorkModule::off_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
        }
    }
}

pub fn install() {
    Agent::new("ness")
        .game_acmd("game_attackairf", skullkid_game_attackairf, Default)
        .effect_acmd("effect_attackairf", skullkid_effect_attackairf, Default)
		.game_acmd("game_attackairb", skullkid_game_attackairb, Default)
        .effect_acmd("effect_attackairb", skullkid_effect_attackairb, Default)
        .game_acmd("game_attackairhi", skullkid_game_attackairhi, Default)
        .game_acmd("game_attackairn", skullkid_game_attackairn, Default)
        .game_acmd("game_attackairlw", skullkid_game_attackairlw, Default)
        .install();
}