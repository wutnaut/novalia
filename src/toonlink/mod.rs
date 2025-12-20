use {
    smash::{
        lua2cpp::*,
        phx::*,
        app::{sv_animcmd::*, lua_bind::*, *},
        lib::{lua_const::*, L2CValue, L2CAgent},
        hash40
    },
    smash_script::*,
    smashline::{*, Priority::*}
};

unsafe extern "C" fn windwaker_game_attackdash(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.module_accessor;
    frame(lua_state, 1.0);
    macros::FT_MOTION_RATE_RANGE(agent, 1.0, 9.0, 6.0);
    //frame(lua_state, 5.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 0.0, 180, 0, 50, 50, 14.0, 0.0, 8.0, 25.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.3, 0, false, false, true, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_TOONLINK_HIT, *ATTACK_REGION_SWORD);
        macros::EFFECT(agent, Hash40::new("sys_dash_smoke"), Hash40::new("top"), 0, 0, 45, 0, 0, 0, 1.5, 0, 0, 0, 0, 0, 0, true);
        if WorkModule::get_int(agent.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) == 5 {
            macros::LAST_EFFECT_SET_COLOR(agent, 0.58, 0.0, 0.82);
        }
        KineticModule::add_speed(agent.module_accessor, &Vector3f{ x: 0.4, y: 0.0, z: 0.0 });
    }
    frame(lua_state, 9.0);
    macros::FT_MOTION_RATE(agent, 1.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(boma);
        macros::ATTACK(agent, 0, 0, Hash40::new("shoulderl"), 8.0, 63, 105, 0, 39, 3.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.3, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_TOONLINK_HIT, *ATTACK_REGION_SWORD);
        macros::ATTACK(agent, 1, 0, Hash40::new("sword2"), 8.0, 63, 105, 0, 39, 3.5, 1.0, 0.0, -2.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.3, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_TOONLINK_HIT, *ATTACK_REGION_SWORD);
        macros::ATTACK(agent, 2, 0, Hash40::new("sword2"), 8.0, 63, 105, 0, 39, 4.0, 5.0, 0.0, -2.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.3, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_TOONLINK_HIT, *ATTACK_REGION_SWORD);
    }
    frame(lua_state, 13.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(boma);
    }
}


// aerials

unsafe extern "C" fn windwaker_game_attackairn(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.module_accessor;
    frame(lua_state, 3.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(boma);
        macros::ATTACK(agent, 4, 0, Hash40::new("top"), 0.0, 180, 0, 50, 50, 12.0, 0.0, 10.0, 17.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.3, 0, false, false, true, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_TOONLINK_HIT, *ATTACK_REGION_SWORD);
        macros::EFFECT_FOLLOW(agent, Hash40::new("sys_dash_smoke"), Hash40::new("top"), 0, -1.0, 25, 0, 0, 0, 1.5, true);
        if WorkModule::get_int(agent.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) == 5 {
            macros::LAST_EFFECT_SET_COLOR(agent, 0.58, 0.0, 0.82);
        }
    }
    frame(lua_state, 4.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
    frame(lua_state, 6.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(boma);
        macros::ATTACK(agent, 0, 0, Hash40::new("sword2"), 10.5, 50, 100, 0, 20, 3.5, 5.0, 0.0, -1.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.3, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_TOONLINK_HIT, *ATTACK_REGION_SWORD);
        macros::ATTACK(agent, 1, 0, Hash40::new("sword2"), 10.5, 50, 100, 0, 20, 3.5, 0.0, 0.0, -1.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.3, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_TOONLINK_HIT, *ATTACK_REGION_SWORD);
        macros::ATTACK(agent, 2, 0, Hash40::new("arml"), 10.5, 50, 100, 0, 20, 3.5, -0.5, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.3, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_TOONLINK_HIT, *ATTACK_REGION_SWORD);
        macros::ATTACK(agent, 3, 0, Hash40::new("clavicler"), 10.5, 50, 100, 0, 20, 3.5, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.3, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_TOONLINK_HIT, *ATTACK_REGION_SWORD);
    }
    frame(lua_state, 10.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(boma);
        macros::ATTACK(agent, 4, 0, Hash40::new("top"), 0.0, 0, 0, 50, 50, 12.0, 0.0, 12.0, -17.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.3, 0, false, false, true, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_TOONLINK_HIT, *ATTACK_REGION_SWORD);
        macros::EFFECT_FOLLOW_FLIP(agent, Hash40::new("sys_dash_smoke"), Hash40::new("sys_dash_smoke"), Hash40::new("top"), 0, -1.0, -26.0, 0, 180, 0, 1.5, true, *EF_FLIP_NONE);
        if WorkModule::get_int(agent.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) == 5 {
            macros::LAST_EFFECT_SET_COLOR(agent, 0.58, 0.0, 0.82);
        }
    }
    frame(lua_state, 13.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(boma);
        macros::ATTACK(agent, 0, 0, Hash40::new("sword2"), 9.0, 35, 100, 0, 20, 3.5, 5.0, 0.0, -1.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.3, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_TOONLINK_HIT, *ATTACK_REGION_SWORD);
        macros::ATTACK(agent, 1, 0, Hash40::new("sword2"), 9.0, 35, 100, 0, 20, 3.5, 0.0, 0.0, -1.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.3, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_TOONLINK_HIT, *ATTACK_REGION_SWORD);
        macros::ATTACK(agent, 2, 0, Hash40::new("arml"), 9.0, 35, 100, 0, 20, 3.5, -0.5, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.3, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_TOONLINK_HIT, *ATTACK_REGION_SWORD);
        macros::ATTACK(agent, 3, 0, Hash40::new("clavicler"), 9.0, 35, 100, 0, 20, 3.5, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.3, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_TOONLINK_HIT, *ATTACK_REGION_SWORD);
    }
    frame(lua_state, 19.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 42.0);
    if macros::is_excute(agent) {
        WorkModule::off_flag(boma, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
}

unsafe extern "C" fn windwaker_game_attackairf(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.module_accessor;
    frame(lua_state, 1.0);
    macros::FT_MOTION_RATE(agent, 0.833);
    frame(lua_state, 4.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
    frame(lua_state, 9.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 3, 0, Hash40::new("top"), 0.0, 150, 0, 50, 50, 15.0, 0.0, 12.0, 18.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.3, 0, false, false, true, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_FIGHTER, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_TOONLINK_HIT, *ATTACK_REGION_SWORD);
        //macros::ATTACK(agent, 3, 0, Hash40::new("top"), 0.1, 110, 0, 500, 500, 15.0, 0.0, 12.0, 18.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.3, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_NO_FIGHTER, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_TOONLINK_HIT, *ATTACK_REGION_SWORD);
        //macros::ATTACK(agent, 3, 0, Hash40::new("top"), 0.0, 110, 0, 5, 5, 15.0, 0.0, 12.0, 18.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.3, 0, false, false, true, false, false, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_NO_FIGHTER, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_TOONLINK_HIT, *ATTACK_REGION_SWORD);
        //macros::EFFECT(agent, Hash40::new("sys_dash_smoke"), Hash40::new("top"), 0, -1, 27, 0, 0, 0, 1.5, 0, 0, 0, 0, 0, 0, true);
        macros::EFFECT_FOLLOW(agent, Hash40::new("sys_dash_smoke"), Hash40::new("top"), 0, -1, 27, 0, 0, 0, 1.5, true);
        if WorkModule::get_int(agent.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) == 5 {
            macros::LAST_EFFECT_SET_COLOR(agent, 0.58, 0.0, 0.82);
        }
    }
    frame(lua_state, 12.0);
    macros::FT_MOTION_RATE(agent, 1.0);
    frame(lua_state, 13.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(boma);
        macros::ATTACK(agent, 0, 0, Hash40::new("shoulderl"), 13.0, 361, 98, 0, 35, 3.0, 0.0, 0.0, 0.0,  None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_TOONLINK_HIT, *ATTACK_REGION_SWORD);
        macros::ATTACK(agent, 1, 0, Hash40::new("sword2"), 13.0, 361, 98, 0, 35, 3.5, 1.0, -1.5, -2.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_TOONLINK_HIT, *ATTACK_REGION_SWORD);
        macros::ATTACK(agent, 2, 0, Hash40::new("sword2"), 13.0, 361, 98, 0, 35, 4.0, 5.0, -1.5, -2.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_TOONLINK_HIT, *ATTACK_REGION_SWORD);
    }
    wait(lua_state, 3.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 32.0);
    if macros::is_excute(agent) {
        WorkModule::off_flag(boma, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
}

unsafe extern "C" fn windwaker_game_attackairb(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.module_accessor;
    if macros::is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
    frame(lua_state, 2.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 3, 0, Hash40::new("top"), 0.0, 30, 0, 50, 50, 15.0, 0.0, 12.0, -18.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.3, 0, false, false, true, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_TOONLINK_HIT, *ATTACK_REGION_SWORD);
        //macros::EFFECT_FLIP(agent, Hash40::new("sys_dash_smoke"), Hash40::new("sys_dash_smoke"), Hash40::new("top"), 0, 0, -20, 0, 0, 0, 1.5, 0, 0, 0, 0, 0, 0, true, *EF_FLIP_YZ);
        macros::EFFECT_FOLLOW_FLIP(agent, Hash40::new("sys_dash_smoke"), Hash40::new("sys_dash_smoke"), Hash40::new("top"), 0, -3.0, -26.0, 0, 180, 0.0, 1.5, true, *EF_FLIP_NONE);
        if WorkModule::get_int(agent.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) == 5 {
            macros::LAST_EFFECT_SET_COLOR(agent, 0.58, 0.0, 0.82);
        }
    }
    frame(lua_state, 5.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(boma);
        macros::ATTACK(agent, 0, 0, Hash40::new("shoulderl"), 9.0, 67, 105, 0, 25, 3.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_TOONLINK_HIT, *ATTACK_REGION_SWORD);
        macros::ATTACK(agent, 1, 0, Hash40::new("sword2"), 9.0, 67, 105, 0, 25, 3.5, 1.0, 0.0, -2.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_TOONLINK_HIT, *ATTACK_REGION_SWORD);
        macros::ATTACK(agent, 2, 0, Hash40::new("sword2"), 9.0, 67, 105, 0, 25, 4.0, 5.0, 0.0, -2.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_TOONLINK_HIT, *ATTACK_REGION_SWORD);
    }
    frame(lua_state, 12.0);
    macros::FT_MOTION_RATE(agent, 1.667);
    if macros::is_excute(agent) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 19.0);
    if macros::is_excute(agent) {
        WorkModule::off_flag(boma, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
}

unsafe extern "C" fn windwaker_game_attackairhi(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.module_accessor;
    frame(lua_state, 2.0);
    if macros::is_excute(agent) {
        if KineticModule::get_sum_speed_y(boma, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN) < 0.0 {
            KineticModule::add_speed(agent.module_accessor, &Vector3f{ x: 0.0, y: 2.5, z: 0.0 });
        } else {
            KineticModule::add_speed(agent.module_accessor, &Vector3f{ x: 0.0, y: 0.7, z: 0.0 });
        }
    }
    macros::FT_MOTION_RATE(agent, 0.5);
    //frame(lua_state, 6.0);
    //if macros::is_excute(agent) {
    //    WorkModule::on_flag(boma, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    //}
    frame(lua_state, 10.0);
    macros::FT_MOTION_RATE(agent, 0.5);
    frame(lua_state, 11.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("arml"), 1.5, 365, 100, 0, 35, 3.0, 0.0, 0.0, 0.0, None, None, None, 0.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 4, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        macros::ATTACK(agent, 1, 0, Hash40::new("sword2"), 1.5, 365, 100, 0, 35, 3.5, 1.0, 0.0, 0.0, None, None, None, 0.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 4, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        macros::ATTACK(agent, 2, 0, Hash40::new("sword2"), 1.5, 365, 100, 0, 35, 4.0, 5.0, 0.0, 0.0, None, None, None, 0.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 4, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        //macros::ATTACK(agent, 3, 0, Hash40::new("top"), 0.0, 90, 0, 50, 50, 5.0, 0.0, 30.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.3, 0, false, false, true, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_TOONLINK_HIT, *ATTACK_REGION_SWORD);
        macros::ATTACK(agent, 4, 0, Hash40::new("top"), 0.0, 150, 0, 30, 30, 4.0, 0.0, 22.0, 8.0, Some(0.0), Some(17.0), Some(6.0), 0.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 4, false, false, true, true, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_TOONLINK_HIT, *ATTACK_REGION_SWORD);
        macros::ATTACK(agent, 5, 0, Hash40::new("top"), 0.0, 30, 0, 30, 30, 4.0, 0.0, 22.0, -8.0, Some(0.0), Some(17.0), Some(-6.0), 0.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 4, false, false, true, true, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_TOONLINK_HIT, *ATTACK_REGION_SWORD);
    }
    frame(lua_state, 33.0);
    macros::FT_MOTION_RATE(agent, 1.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("arml"), 8.0, 80, 126, 0, 40, 3.5, 0.0, 0.0, 0.0, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_TOONLINK_HIT, *ATTACK_REGION_SWORD);
        macros::ATTACK(agent, 1, 0, Hash40::new("sword2"), 8.0, 80, 126, 0, 40, 4.0, 1.0, 0.0, 0.0, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_TOONLINK_HIT, *ATTACK_REGION_SWORD);
        macros::ATTACK(agent, 2, 0, Hash40::new("sword2"), 8.0, 80, 126, 0, 40, 4.5, 5.0, 0.0, 0.0, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_TOONLINK_HIT, *ATTACK_REGION_SWORD);
        //macros::ATTACK(agent, 3, 0, Hash40::new("top"), 0.0, 90, 0, 50, 50, 5.0, 0.0, 30.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.3, 0, false, false, true, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_TOONLINK_HIT, *ATTACK_REGION_SWORD);
        macros::ATTACK(agent, 4, 0, Hash40::new("top"), 0.0, 150, 0, 30, 30, 4.0, 0.0, 22.0, 8.0, Some(0.0), Some(17.0), Some(6.0), 0.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, true, true, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_TOONLINK_HIT, *ATTACK_REGION_SWORD);
        macros::ATTACK(agent, 5, 0, Hash40::new("top"), 0.0, 30, 0, 30, 30, 4.0, 0.0, 22.0, -8.0, Some(0.0), Some(17.0), Some(-6.0), 0.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, true, true, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_TOONLINK_HIT, *ATTACK_REGION_SWORD);
    }
    frame(lua_state, 39.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(boma);
    }
    //wait(lua_state, 6.0);
    //if macros::is_excute(agent) {
    //    WorkModule::off_flag(boma, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    //}
}

unsafe extern "C" fn windwaker_game_attackairlw(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.module_accessor;
    let mut has_pogoed = false;
    wait(lua_state, 9.0);
    macros::FT_MOTION_RATE(agent, 0.3);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 14.0, 100, 88, 0, 35, 5.0, 1.0, -1.1, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_odin_slash"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_TOONLINK_HIT, *ATTACK_REGION_SWORD);
        macros::ATTACK(agent, 1, 0, Hash40::new("top"), 14.0, 280, 105, 0, 55, 3.5, 1.0, 5.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_TOONLINK_HIT, *ATTACK_REGION_SWORD);
    }
    frame(lua_state, 15.0);
    if macros::is_excute(agent) {
        //AttackModule::clear_all(boma);
        if AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_HIT | *COLLISION_KIND_MASK_SHIELD) {
            macros::FT_MOTION_RATE(agent, 1.0);
            KineticModule::resume_energy(boma, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);
            WorkModule::off_flag(boma, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
            WorkModule::off_flag(boma, *FIGHTER_STATUS_WORK_ID_FLAG_RESERVE_GRAVITY_STABLE_UNABLE);
            macros::SET_SPEED_EX(agent, 0, 2.1, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
            //macros::FT_MOTION_RATE_RANGE(agent, 46.0, 57.0, 26.0);
            AttackModule::clear_all(boma);
            wait(lua_state, 15.0);
            macros::ATTACK(agent, 0, 0, Hash40::new("top"), 6.0, 361, 98, 0, 40, 5.0, 1.0, -1.1, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_TOONLINK_HIT, *ATTACK_REGION_SWORD);
            macros::ATTACK(agent, 1, 0, Hash40::new("top"), 14.0, 280, 105, 0, 55, 3.5, 1.0, 5.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_TOONLINK_HIT, *ATTACK_REGION_SWORD);
            has_pogoed = true;
            WorkModule::on_flag(boma, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
        }
    }
    frame(lua_state, 25.0);
    //macros::FT_MOTION_RATE(agent, 1.0);
    if macros::is_excute(agent) {
        //AttackModule::clear_all(boma);
        if AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_HIT | *COLLISION_KIND_MASK_SHIELD) && !has_pogoed {
            macros::FT_MOTION_RATE(agent, 1.0);
            KineticModule::resume_energy(boma, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);
            WorkModule::off_flag(boma, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
            WorkModule::off_flag(boma, *FIGHTER_STATUS_WORK_ID_FLAG_RESERVE_GRAVITY_STABLE_UNABLE);
            macros::SET_SPEED_EX(agent, 0, 2.0, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
            //macros::FT_MOTION_RATE_RANGE(agent, 46.0, 57.0, 26.0);
            AttackModule::clear_all(boma);
            wait(lua_state, 15.0);
            macros::ATTACK(agent, 0, 0, Hash40::new("top"), 6.0, 361, 98, 0, 40, 5.0, 1.0, -1.1, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_TOONLINK_HIT, *ATTACK_REGION_SWORD);
            macros::ATTACK(agent, 1, 0, Hash40::new("top"), 14.0, 280, 105, 0, 55, 3.5, 1.0, 5.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_TOONLINK_HIT, *ATTACK_REGION_SWORD);
            has_pogoed = true;
            WorkModule::on_flag(boma, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
        }
    }
    frame(lua_state, 35.0);
    if macros::is_excute(agent) {
        if AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_HIT | *COLLISION_KIND_MASK_SHIELD) && !has_pogoed {
            macros::FT_MOTION_RATE(agent, 1.0);
            KineticModule::resume_energy(boma, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);
            WorkModule::off_flag(boma, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
            WorkModule::off_flag(boma, *FIGHTER_STATUS_WORK_ID_FLAG_RESERVE_GRAVITY_STABLE_UNABLE);
            macros::SET_SPEED_EX(agent, 0, 1.9, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
            //macros::FT_MOTION_RATE_RANGE(agent, 46.0, 57.0, 26.0);
            AttackModule::clear_all(boma);
            wait(lua_state, 15.0);
            macros::ATTACK(agent, 0, 0, Hash40::new("top"), 6.0, 361, 98, 0, 40, 5.0, 1.0, -1.1, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_TOONLINK_HIT, *ATTACK_REGION_SWORD);
            macros::ATTACK(agent, 1, 0, Hash40::new("top"), 14.0, 280, 105, 0, 55, 3.5, 1.0, 5.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_TOONLINK_HIT, *ATTACK_REGION_SWORD);
            has_pogoed = true;
            WorkModule::on_flag(boma, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
        }
    }
    frame(lua_state, 45.0);
    if macros::is_excute(agent) {
        if AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_HIT | *COLLISION_KIND_MASK_SHIELD) && !has_pogoed {
            macros::FT_MOTION_RATE(agent, 1.0);
            KineticModule::resume_energy(boma, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);
            WorkModule::off_flag(boma, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
            WorkModule::off_flag(boma, *FIGHTER_STATUS_WORK_ID_FLAG_RESERVE_GRAVITY_STABLE_UNABLE);
            macros::SET_SPEED_EX(agent, 0, 1.8, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
            //macros::FT_MOTION_RATE_RANGE(agent, 46.0, 57.0, 26.0);
            AttackModule::clear_all(boma);
            wait(lua_state, 15.0);
            macros::ATTACK(agent, 0, 0, Hash40::new("top"), 6.0, 361, 98, 0, 40, 5.0, 1.0, -1.1, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_TOONLINK_HIT, *ATTACK_REGION_SWORD);
            macros::ATTACK(agent, 1, 0, Hash40::new("top"), 14.0, 280, 105, 0, 55, 3.5, 1.0, 5.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_TOONLINK_HIT, *ATTACK_REGION_SWORD);
            has_pogoed = true;
            WorkModule::on_flag(boma, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);

        }
    }
    frame(lua_state, 55.0);
    if macros::is_excute(agent) {
        if AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_HIT | *COLLISION_KIND_MASK_SHIELD) && !has_pogoed {
            macros::FT_MOTION_RATE(agent, 1.0);
            KineticModule::resume_energy(boma, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);
            WorkModule::off_flag(boma, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
            WorkModule::off_flag(boma, *FIGHTER_STATUS_WORK_ID_FLAG_RESERVE_GRAVITY_STABLE_UNABLE);
            macros::SET_SPEED_EX(agent, 0, 1.7, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
            //macros::FT_MOTION_RATE_RANGE(agent, 46.0, 57.0, 26.0);
            AttackModule::clear_all(boma);
            wait(lua_state, 8.0);
            macros::ATTACK(agent, 0, 0, Hash40::new("top"), 6.0, 361, 98, 0, 40, 5.0, 1.0, -1.1, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_TOONLINK_HIT, *ATTACK_REGION_SWORD);
            macros::ATTACK(agent, 1, 0, Hash40::new("top"), 14.0, 280, 105, 0, 55, 3.5, 1.0, 5.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_TOONLINK_HIT, *ATTACK_REGION_SWORD);
            has_pogoed = true;
            WorkModule::on_flag(boma, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
        }
    }
    frame(lua_state, 65.0);
    macros::FT_MOTION_RATE(agent, 1.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(boma);
        //KineticModule::change_kinetic(boma, *FIGHTER_KINETIC_TYPE_FALL);
        if AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_HIT | *COLLISION_KIND_MASK_SHIELD) && !has_pogoed {
            KineticModule::resume_energy(boma, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);
            WorkModule::off_flag(boma, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
            WorkModule::off_flag(boma, *FIGHTER_STATUS_WORK_ID_FLAG_RESERVE_GRAVITY_STABLE_UNABLE);
            macros::SET_SPEED_EX(agent, 0, 1.625, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
            //macros::FT_MOTION_RATE_RANGE(agent, 46.0, 57.0, 26.0);
        }
    }
}

unsafe extern "C" fn windwaker_effect_attackairlw(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    frame(lua_state, 9.0);
    if macros::is_excute(agent) {
        macros::EFFECT(agent, Hash40::new("sys_smash_flash"), Hash40::new("havel"), 0, -8, 0, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, true);
        macros::LAST_EFFECT_SET_RATE(agent, 1.7);
    }
    frame(lua_state, 12.0);
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW(agent, Hash40::new("toonlink_sword"), Hash40::new("sword1"), -0.03, 0.05, 0, 0, 0, -0.87, 1.01, true);
        macros::EFFECT_FOLLOW(agent, Hash40::new("toonlink_sword_flare"), Hash40::new("sword1"), -0.03, 0.05, 0, 0, 0, -0.87, 1.01, true);
    }
}

unsafe extern "C" fn windwaker_game_attackairlw2attack(agent: &mut L2CAgentBase) {

}

unsafe extern "C" fn windwaker_game_attackairlw2bound(agent: &mut L2CAgentBase) {

}

unsafe extern "C" fn windwaker_game_attackairlwoffgravity(agent: &mut L2CAgentBase) {
    
}

// smashes

unsafe extern "C" fn windwaker_game_attacks4(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.module_accessor;
    frame(lua_state, 9.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
    }
    frame(lua_state, 16.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("arml"), 16.0, 40, 105, 0, 25, 3.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 10, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_TOONLINK_HIT, *ATTACK_REGION_SWORD);
        macros::ATTACK(agent, 1, 0, Hash40::new("sword2"), 16.0, 40, 105, 0, 25, 3.5, 1.0, 0.0, -2.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 10, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_TOONLINK_HIT, *ATTACK_REGION_SWORD);
        macros::ATTACK(agent, 2, 0, Hash40::new("sword2"), 16.0, 40, 105, 0, 25, 4.0, 5.0, 0.0, -2.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 10, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_TOONLINK_HIT, *ATTACK_REGION_SWORD);
        //macros::ATTACK(agent, 3, 0, Hash40::new("top"), 0.0, 75, 0, 60, 60, 5.0, 0.0, 15.0, 18.0, Some(0.0), Some(5.0), Some(18.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.3, 0, false, false, false, true, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_kick"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_TOONLINK_HIT, *ATTACK_REGION_SWORD);
        //macros::EFFECT(agent, Hash40::new("sys_dash_smoke"), Hash40::new("top"), 0, 0, -20, 0, 0, 0, -1.5, 0, 0, 0, 0, 0, 0, true);
        shield!(agent, *MA_MSC_CMD_REFLECTOR, *COLLISION_KIND_REFLECTOR, 
            0, // ID
            Hash40::new("top"), // Bone
            6.0, // Size
            0.0, 12.0, 18.0, // X1, Y1, Z1
            0.0, 6.0, 18.0, // X2, Y2, Z2
            1.0, // Damage multiplier
            1.1, // Speed multiplier
            45, // Max damage
            false, // Unknown
            2.0, // Life multiplier
            *FIGHTER_REFLECTOR_GROUP_HOMERUNBAT); // "type"
            macros::ATK_SET_SHIELD_SETOFF_MUL(agent, 0, 2.0);
            macros::ATK_SET_SHIELD_SETOFF_MUL(agent, 1, 2.0);
            macros::ATK_SET_SHIELD_SETOFF_MUL(agent, 2, 2.0);
    }
    wait(lua_state, 4.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(boma);
        shield!(agent, *MA_MSC_CMD_SHIELD_OFF, *COLLISION_KIND_REFLECTOR, 0, *FIGHTER_REFLECTOR_GROUP_HOMERUNBAT);
    } 
}

unsafe extern "C" fn windwaker_game_specialairhi(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.module_accessor;
    let mut ww_upB_nopress = true;
    let mut ww_upB_press = false;
    let mut ww_upB_boostcount = 0;
    //if macros::is_excute(agent) {
    //    boma.select_cliff_hangdata_from_name("special_hi");
    //}
    frame(lua_state, 8.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 4.0, 88, 100, 90, 0, 5.0, 0.0, 7.0, 13.0, Some(0.0), Some(7.0), Some(5.0), 0.75, 0.3, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_TOONLINK_HIT, *ATTACK_REGION_SWORD);
        macros::ATTACK(agent, 1, 0, Hash40::new("top"), 4.0, 74, 100, 83, 0, 5.0, 0.0, 7.0, 13.0, Some(0.0), Some(7.0), Some(5.0), 0.75, 0.3, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_TOONLINK_HIT, *ATTACK_REGION_SWORD);
        AttackModule::set_no_damage_fly_smoke_all(boma, true, false);
        macros::ATTACK(agent, 3, 0, Hash40::new("top"), 0.0, 110, 0, 100, 100, 10.0, 0.0, -10.0, 9.0, Some(0.0), Some(2.0), Some(9.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, true, false, false, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_TOONLINK_HIT, *ATTACK_REGION_SWORD);
        macros::ATTACK(agent, 4, 0, Hash40::new("top"), 0.0, 110, 0, 100, 100, 10.0, 0.0, -10.0, 9.0, Some(0.0), Some(2.0), Some(9.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_slip"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_TOONLINK_HIT, *ATTACK_REGION_SWORD);
        macros::EFFECT(agent, Hash40::new("sys_landing_smoke_s"), Hash40::new("top"), 0, -7, 9, 180, 0, 0, 1.5, 0, 0, 0, 0, 0, 0, true);
        if WorkModule::get_int(agent.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) == 5 {
            macros::LAST_EFFECT_SET_COLOR(agent, 0.58, 0.0, 0.82);
        }
        macros::EFFECT(agent, Hash40::new("sys_h_smoke_a"), Hash40::new("top"), 0, -5, 9, 70, 0, 0, 1.2, 0, 0, 0, 0, 0, 0, true);
        if WorkModule::get_int(agent.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) == 5 {
            macros::LAST_EFFECT_SET_COLOR(agent, 0.58, 0.0, 0.82);
        }
        if ww_upB_boostcount < 4 {
            if ControlModule::check_button_on(agent.module_accessor, *CONTROL_PAD_BUTTON_SPECIAL) && ww_upB_nopress == true { 
                KineticModule::add_speed(agent.module_accessor, &Vector3f{ x: 0.0, y: 0.2, z: 0.0 });
                macros::EFFECT(agent, Hash40::new("sys_attack_speedline"), Hash40::new("top"), 0, 6, 0, 90, 0, 0, 1.1, 0, 0, 0, 0, 0, 0, true);
                if WorkModule::get_int(agent.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) == 5 {
                    macros::LAST_EFFECT_SET_COLOR(agent, 0.29, 0.0, 0.41);
                }
                ww_upB_press = true;
                ww_upB_nopress = false;
                ww_upB_boostcount += 1;
            }
        }
    }
    wait(lua_state, 3.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(boma);
        if ControlModule::check_button_off(agent.module_accessor, *CONTROL_PAD_BUTTON_SPECIAL) { 
            ww_upB_nopress = true;
            ww_upB_press = false;
        }
    }
    frame(lua_state, 12.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 2.0, 50, 100, 90, 0, 5.5, 0.0, 7.0, -10.0, None, None, None, 0.75, 0.3, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_TOONLINK_HIT, *ATTACK_REGION_SWORD);
        AttackModule::set_no_damage_fly_smoke_all(boma, true, false);
        if ControlModule::check_button_off(agent.module_accessor, *CONTROL_PAD_BUTTON_SPECIAL) { 
            ww_upB_nopress = true;
            ww_upB_press = false;
        }
    }
    wait(lua_state, 2.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(boma);
        if ControlModule::check_button_off(agent.module_accessor, *CONTROL_PAD_BUTTON_SPECIAL) { 
            ww_upB_nopress = true;
            ww_upB_press = false;
        }
    }
    frame(lua_state, 16.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 2.0, 88, 100, 85, 0, 5.0, 0.0, 7.5, 13.0, Some(0.0), Some(7.0), Some(5.0), 0.75, 0.3, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_TOONLINK_HIT, *ATTACK_REGION_SWORD);
        macros::ATTACK(agent, 1, 0, Hash40::new("top"), 2.0, 95, 100, 65, 0, 5.0, 0.0, 7.5, 13.0, Some(0.0), Some(7.0), Some(5.0), 0.75, 0.3, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_TOONLINK_HIT, *ATTACK_REGION_SWORD);
        AttackModule::set_no_damage_fly_smoke_all(boma, true, false);
        if ww_upB_boostcount < 4 {
            if ControlModule::check_button_on(agent.module_accessor, *CONTROL_PAD_BUTTON_SPECIAL) && ww_upB_nopress == true { 
                KineticModule::add_speed(agent.module_accessor, &Vector3f{ x: 0.0, y: 0.2, z: 0.0 });
                macros::EFFECT(agent, Hash40::new("sys_attack_speedline"), Hash40::new("top"), 0, 6, 0, 90, 0, 0, 1.1, 0, 0, 0, 0, 0, 0, true);
                if WorkModule::get_int(agent.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) == 5 {
                    macros::LAST_EFFECT_SET_COLOR(agent, 0.29, 0.0, 0.41);
                }
                ww_upB_press = true;
                ww_upB_nopress = false;
                ww_upB_boostcount += 1;
            }
        }
    }
    wait(lua_state, 3.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(boma);
        if ControlModule::check_button_off(agent.module_accessor, *CONTROL_PAD_BUTTON_SPECIAL) { 
            ww_upB_nopress = true;
            ww_upB_press = false;
        }
    }
    frame(lua_state, 19.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 2.0, 40, 100, 88, 0, 5.5, 0.0, 3.5, -8.5, None, None, None, 0.75, 0.3, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_TOONLINK_HIT, *ATTACK_REGION_SWORD);
        AttackModule::set_no_damage_fly_smoke_all(boma, true, false);
        if ControlModule::check_button_off(agent.module_accessor, *CONTROL_PAD_BUTTON_SPECIAL) { 
            ww_upB_nopress = true;
            ww_upB_press = false;
        }
    }
    wait(lua_state, 2.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(boma);
        if ControlModule::check_button_off(agent.module_accessor, *CONTROL_PAD_BUTTON_SPECIAL) { 
            ww_upB_nopress = true;
            ww_upB_press = false;
        }
    }
    frame(lua_state, 22.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 2.0, 60, 100, 60, 0, 5.0, 0.0, 7.0, 5.0, Some(0.0), Some(7.0), Some(5.0), 0.75, 0.3, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_TOONLINK_HIT, *ATTACK_REGION_SWORD);
        macros::ATTACK(agent, 1, 0, Hash40::new("top"), 2.0, 90, 100, 88, 0, 5.0, 0.0, 8.0, 13.0, Some(0.0), Some(7.0), Some(5.0), 0.75, 0.3, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_TOONLINK_HIT, *ATTACK_REGION_SWORD);
        macros::ATTACK(agent, 2, 0, Hash40::new("top"), 2.0, 95, 100, 65, 0, 5.0, 0.0, 8.0, 13.0, Some(0.0), Some(7.0), Some(5.0), 0.75, 0.3, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_TOONLINK_HIT, *ATTACK_REGION_SWORD);
        AttackModule::set_no_damage_fly_smoke_all(boma, true, false);
        if ww_upB_boostcount < 4 {
            if ControlModule::check_button_on(agent.module_accessor, *CONTROL_PAD_BUTTON_SPECIAL) && ww_upB_nopress == true { 
                KineticModule::add_speed(agent.module_accessor, &Vector3f{ x: 0.0, y: 0.2, z: 0.0 });
                macros::EFFECT(agent, Hash40::new("sys_attack_speedline"), Hash40::new("top"), 0, 6, 0, 90, 0, 0, 1.1, 0, 0, 0, 0, 0, 0, true);
                if WorkModule::get_int(agent.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) == 5 {
                    macros::LAST_EFFECT_SET_COLOR(agent, 0.29, 0.0, 0.41);
                }
                ww_upB_press = true;
                ww_upB_nopress = false;
                ww_upB_boostcount += 1;
            }
        }
    }
    wait(lua_state, 3.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(boma);
        if ControlModule::check_button_off(agent.module_accessor, *CONTROL_PAD_BUTTON_SPECIAL) { 
            ww_upB_nopress = true;
            ww_upB_press = false;
        }
    }
    frame(lua_state, 27.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 2.0, 30, 100, 80, 0, 5.5, 0.0, 4.0, -10.0, None, None, None, 0.75, 0.3, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_TOONLINK_HIT, *ATTACK_REGION_SWORD);
        AttackModule::set_no_damage_fly_smoke_all(boma, true, false);
        if ControlModule::check_button_off(agent.module_accessor, *CONTROL_PAD_BUTTON_SPECIAL) { 
            ww_upB_nopress = true;
            ww_upB_press = false;
        }
    }
    wait(lua_state, 2.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(boma);
        if ControlModule::check_button_off(agent.module_accessor, *CONTROL_PAD_BUTTON_SPECIAL) { 
            ww_upB_nopress = true;
            ww_upB_press = false;
        }
    }
    frame(lua_state, 31.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 2.0, 70, 100, 58, 0, 5.0, 0.0, 7.0, 5.0, Some(0.0), Some(7.0), Some(5.0), 0.75, 0.3, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_TOONLINK_HIT, *ATTACK_REGION_SWORD);
        macros::ATTACK(agent, 1, 0, Hash40::new("top"), 2.0, 120, 100, 20, 0, 5.0, 0.0, 8.5, 12.0, Some(0.0), Some(7.0), Some(5.0), 0.75, 0.3, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_TOONLINK_HIT, *ATTACK_REGION_SWORD);
        macros::ATTACK(agent, 2, 0, Hash40::new("top"), 2.0, 95, 100, 65, 0, 5.0, 0.0, 8.5, 12.0, Some(0.0), Some(7.0), Some(5.0), 0.75, 0.3, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_TOONLINK_HIT, *ATTACK_REGION_SWORD);
        AttackModule::set_no_damage_fly_smoke_all(boma, true, false);
        if ww_upB_boostcount < 4 {
            if ControlModule::check_button_on(agent.module_accessor, *CONTROL_PAD_BUTTON_SPECIAL) && ww_upB_nopress == true { 
                KineticModule::add_speed(agent.module_accessor, &Vector3f{ x: 0.0, y: 0.2, z: 0.0 });
                macros::EFFECT(agent, Hash40::new("sys_attack_speedline"), Hash40::new("top"), 0, 6, 0, 90, 0, 0, 1.1, 0, 0, 0, 0, 0, 0, true);
                if WorkModule::get_int(agent.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) == 5 {
                    macros::LAST_EFFECT_SET_COLOR(agent, 0.29, 0.0, 0.41);
                }
                ww_upB_press = true;
                ww_upB_nopress = false;
                ww_upB_boostcount += 1;
            }
        }
    }
    wait(lua_state, 3.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(boma);

    }
    frame(lua_state, 35.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 2.0, 20, 100, 80, 0, 5.5, 0.0, 5.5, -9.0, None, None, None, 0.75, 0.3, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_TOONLINK_HIT, *ATTACK_REGION_SWORD);
        AttackModule::set_no_damage_fly_smoke_all(boma, true, false);
    }
    wait(lua_state, 1.0);
    macros::FT_MOTION_RATE_RANGE(agent, 35.0, 42.0, 3.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 42.0);
    macros::FT_MOTION_RATE(agent, 1.0);
    if macros::is_excute(agent) {
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ON_DROP_BOTH_SIDES);
    }
    frame(lua_state, 44.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 4.0, 45, 186, 0, 40, 7.0, 0.0, 8.5, 10.75, Some(0.0), Some(8.5), Some(6.25), 1.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_TOONLINK_HIT, *ATTACK_REGION_SWORD);
    }
    frame(lua_state, 48.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_LINK_STATUS_RSLASH_FLAG_RESET_SPEED_MAX_X);
    }
    frame(lua_state, 49.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(boma);
    }
}


pub fn install() {
    Agent::new("toonlink")
        .game_acmd("game_attackdash", windwaker_game_attackdash, Default)
        .game_acmd("game_attackairlw", windwaker_game_attackairlw, Default)
        .game_acmd("effect_attackairlw", windwaker_effect_attackairlw, Default)
        .game_acmd("game_attackairlw2attack", windwaker_game_attackairlw2attack, Default)
        .game_acmd("game_attackairlw2bound", windwaker_game_attackairlw2bound, Default)
        .game_acmd("game_attackairlwoffgravity", windwaker_game_attackairlwoffgravity, Default)
        .game_acmd("game_attackairn", windwaker_game_attackairn, Default)
        .game_acmd("game_attackairf", windwaker_game_attackairf, Default)
        .game_acmd("game_attackairb", windwaker_game_attackairb, Default)
        .game_acmd("game_attackairhi", windwaker_game_attackairhi, Default)
        .game_acmd("game_attacks4", windwaker_game_attacks4, Default)
        .game_acmd("game_specialairhi", windwaker_game_specialairhi, Default)
        .install();
}