use super::*;
use smash::app::sv_animcmd::SET_AIR;

//---------------SMASH ATTACKS-----------------

// SIDE SMASH
unsafe extern "C" fn slippy_effect_attacks4(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.module_accessor;
    let color = WorkModule::get_int(agent.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR);
    if color == 7 {
        frame(agent.lua_state_agent, 1.0);
        if macros::is_excute(agent) {
            macros::EFFECT(agent, Hash40::new("sys_smash_flash"), Hash40::new("toer"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
        }
        frame(agent.lua_state_agent, 11.0);
        if macros::is_excute(agent) {
            macros::LANDING_EFFECT(agent, Hash40::new("sys_atk_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
        }
        frame(agent.lua_state_agent, 13.0);
        if macros::is_excute(agent) {
            macros::EFFECT_FOLLOW(agent, Hash40::new("fox_smash_arc"), Hash40::new("top"), -0.5, 9, 1.5, 180, -160, 70, 0.7, true);
            macros::LAST_EFFECT_SET_RATE(agent, 1.3);
            macros::LAST_EFFECT_SET_COLOR(agent, 0.0, 1.0, 0.0);
        }
    } else {
        frame(agent.lua_state_agent, 1.0);
        if macros::is_excute(agent) {
            macros::EFFECT(agent, Hash40::new("sys_smash_flash"), Hash40::new("toer"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
        }
        frame(agent.lua_state_agent, 11.0);
        if macros::is_excute(agent) {
            macros::LANDING_EFFECT(agent, Hash40::new("sys_atk_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
        }
        frame(agent.lua_state_agent, 13.0);
        if macros::is_excute(agent) {
            macros::EFFECT_FOLLOW(agent, Hash40::new("fox_smash_arc"), Hash40::new("top"), -0.5, 9, 1.5, 180, -160, 70, 0.7, true);
            macros::LAST_EFFECT_SET_RATE(agent, 1.3);
        }
    }
}

// UP SMASH
unsafe extern "C" fn slippy_effect_attackhi4(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.module_accessor;
    let color = WorkModule::get_int(agent.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR);
    if color == 7 {
        frame(lua_state, 1.0);
        if macros::is_excute(agent) {
            macros::EFFECT(agent, Hash40::new("sys_smash_flash"), Hash40::new("top"), 0, 17, 4, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
        }
        frame(lua_state, 7.0);
        if macros::is_excute(agent) {
            macros::LANDING_EFFECT(agent, Hash40::new("sys_h_smoke_a"), Hash40::new("top"), 2, 0, 0, 0, 0, 0, 0.9, 0, 0, 0, 0, 0, 0, false);
        }
        frame(lua_state, 8.0);
        if macros::is_excute(agent) {
            macros::EFFECT_FOLLOW(agent, Hash40::new("fox_smash_arc"), Hash40::new("top"), 0, 12.2, 1.2, 0, 40, 90, 0.9, true);
            macros::LAST_EFFECT_SET_RATE(agent, 0.9);
            macros::LAST_EFFECT_SET_COLOR(agent, 0.0, 1.0, 0.0);
        }
        frame(lua_state, 15.0);
        if macros::is_excute(agent) {
            EffectModule::kill_kind(boma, Hash40::new("fox_smash_arc"), false, false);
        }
        frame(lua_state, 27.0);
        if macros::is_excute(agent) {
            macros::FOOT_EFFECT(agent, Hash40::new("sys_landing_smoke_s"), Hash40::new("top"), -1.5, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
        }
    } else {
        frame(lua_state, 1.0);
        if macros::is_excute(agent) {
            macros::EFFECT(agent, Hash40::new("sys_smash_flash"), Hash40::new("top"), 0, 17, 4, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
        }
        frame(lua_state, 7.0);
        if macros::is_excute(agent) {
            macros::LANDING_EFFECT(agent, Hash40::new("sys_h_smoke_a"), Hash40::new("top"), 2, 0, 0, 0, 0, 0, 0.9, 0, 0, 0, 0, 0, 0, false);
        }
        frame(lua_state, 8.0);
        if macros::is_excute(agent) {
            macros::EFFECT_FOLLOW(agent, Hash40::new("fox_smash_arc"), Hash40::new("top"), 0, 12.2, 1.2, 0, 40, 90, 0.9, true);
            macros::LAST_EFFECT_SET_RATE(agent, 0.9);
        }
        frame(lua_state, 15.0);
        if macros::is_excute(agent) {
            EffectModule::kill_kind(boma, Hash40::new("fox_smash_arc"), false, false);
        }
        frame(lua_state, 27.0);
        if macros::is_excute(agent) {
            macros::FOOT_EFFECT(agent, Hash40::new("sys_landing_smoke_s"), Hash40::new("top"), -1.5, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
        }
    }
}

// DOWN SMASH


pub fn install() {
    Agent::new("fox")
        //.game_acmd("game_attacklw4", slippy_game_attacklw4, Default)
        .effect_acmd("effect_attacks4", slippy_effect_attacks4, Default)
        .effect_acmd("effect_attackhi4", slippy_effect_attackhi4, Default)
        //.sound_acmd("sound_attacklw4", sound_attacklw4, Default)
        //.effect_acmd("effect_attacklw4charge", effect_attacklw4charge, Default)
        //.sound_acmd("sound_attacklw4charge", sound_attacklw4charge, Default)
        //.game_acmd("game_attackhi4", game_attackhi4, Default)
        //.effect_acmd("effect_attackhi4", effect_attackhi4, Default)
        //.effect_acmd("effect_attackhi4charge", effect_attackhi4charge, Default)
        //.sound_acmd("sound_attackhi4", sound_attackhi4, Default)
        //.sound_acmd("sound_attackhi4charge", sound_attackhi4charge, Default)
        .install();
}