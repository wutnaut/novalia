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

unsafe extern "C" fn byleth_game_speciallw(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.module_accessor;
    if macros::is_excute(agent) {
        WorkModule::set_int(boma, 0, *WEAPON_MASTER_AXE_INSTANCE_WORK_ID_INT_CRITICAL_ATTACK_ID);
    }
    frame(lua_state, 61.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("haver"), 25.0, 51, 59, 0, 93, 5.7, 0.0, 14.0, 1.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 45, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_MASTER_AXE, *ATTACK_REGION_OBJECT);
    }
    frame(lua_state, 67.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(boma);
    }
}

unsafe extern "C" fn byleth_game_specialairlw(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.module_accessor;
    if macros::is_excute(agent) {
        WorkModule::set_int(boma, 0, *WEAPON_MASTER_AXE_INSTANCE_WORK_ID_INT_CRITICAL_ATTACK_ID);
    }
    frame(lua_state, 61.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("haver"), 25.0, 270, 85, 0, 93, 5.7, 0.0, 14.0, 1.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 35, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_MASTER_AXE, *ATTACK_REGION_OBJECT);
    }
    frame(lua_state, 67.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(boma);
    }
}

pub fn install() {
    Agent::new("master_axe")
        .game_acmd("game_speciallw", byleth_game_speciallw, Default)
        .game_acmd("game_specialairlw", byleth_game_specialairlw, Default)
        .install();
}
