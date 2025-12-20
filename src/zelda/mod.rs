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

unsafe extern "C" fn zelda_game_attacks3(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma =agent.module_accessor;
    frame(lua_state, 12.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("arml"), 11.5, 361, 68, 0, 72, 2.1, -2.0, 0.2, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_MAGIC);
        macros::ATTACK(agent, 1, 0, Hash40::new("arml"), 11.5, 361, 68, 0, 72, 2.2, 0.7, 0.3, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_MAGIC);
        macros::ATTACK(agent, 2, 0, Hash40::new("arml"), 11.5, 361, 68, 0, 72, 2.3, 3.2, 0.4, -0.1, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_MAGIC);
        macros::ATTACK(agent, 3, 0, Hash40::new("handl"), 15.0, 361, 69, 0, 72, 2.4, 3.5, 0.0, 0.7, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_MAGIC);
        macros::ATTACK(agent, 4, 0, Hash40::new("handl"), 15.0, 361, 69, 0, 72, 2.5, 6.2, 0.0, 1.4, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_MAGIC);
        macros::ATTACK(agent, 5, 0, Hash40::new("handl"), 6.0, 180, 69, 0, 55, 3.5, 9.2, 0.0, 2.1, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_MAGIC);
    }
    wait(lua_state, 2.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(boma);
    }
    macros::FT_MOTION_RATE(agent, 0.88);
}

unsafe extern "C" fn zelda_effect_attacks3(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.module_accessor;
    frame(lua_state, 10.0);
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW(agent, Hash40::new("zelda_atk_flash_s"), Hash40::new("havel"), 0, 0, 0, 0, 0, 0, 1, true);
        macros::EFFECT_FOLLOW_FLIP(agent, Hash40::new("zelda_atk_arc"), Hash40::new("zelda_atk_arc"), Hash40::new("top"), 0, 11.3, 2, 3, 0, 5, 1, true, *EF_FLIP_YZ);
    }
    frame(lua_state, 14.0);
    if macros::is_excute(agent) {
        macros::FOOT_EFFECT(agent, Hash40::new("sys_run_smoke"), Hash40::new("top"), -4, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
}



pub fn install() {
    Agent::new("zelda")
        .game_acmd("game_attacks3", zelda_game_attacks3, Default)
        .game_acmd("effect_attacks3", zelda_effect_attacks3, Default)
        .install();
}