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

unsafe extern "C" fn wolf_game_attackhi4(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 4.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("armr"), 6.5, 96, 120, 0, 25, 4.5, 3.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_PUNCH);
        macros::ATTACK(agent, 1, 0, Hash40::new("armr"), 6.5, 96, 120, 0, 25, 3.0, -1.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_PUNCH);
        macros::ATTACK(agent, 2, 0, Hash40::new("shoulderr"), 6.5, 96, 120, 0, 25, 3.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_PUNCH);
        AttackModule::set_add_reaction_frame(agent.module_accessor, 0, 2.0, false);
        AttackModule::set_add_reaction_frame(agent.module_accessor, 1, 2.0, false);
        AttackModule::set_add_reaction_frame(agent.module_accessor, 2, 2.0, false);
    }
    wait(agent.lua_state_agent, 7.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 26.0);
    macros::FT_MOTION_RATE(agent, 0.2);
}

unsafe extern "C" fn wolf_effect_attackhi4(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 4.0);
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW(agent, Hash40::new("wolf_scratch_arc"), Hash40::new("top"), 3, 7.5, 0, -20, 70, 90, 1.0, true);
    }
}

unsafe extern "C" fn wolf_sound_attackhi4(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 4.0);
    if macros::is_excute(agent) {
        macros::PLAY_SEQUENCE(agent, Hash40::new("seq_wolf_rnd_attack_s"));
        macros::PLAY_SE(agent, Hash40::new("se_wolf_attackhard_s01"));
    }
}

unsafe extern "C" fn wolf_game_attackairn(agent: &mut L2CAgentBase) {
    let boma = agent.module_accessor;
    frame(agent.lua_state_agent, 1.0);
    macros::FT_MOTION_RATE(agent, 0.3);
    frame(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
    frame(agent.lua_state_agent, 12.0);
    macros::FT_MOTION_RATE(agent, 1.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 1.0, 365, 50, 0, 45, 4.5, 0.0, 5.8, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 3, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
    }
    wait(agent.lua_state_agent, 10.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 22.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 8.0, 361, 95, 0, 55, 4.8, 0.0, 5.8, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
    }
    wait(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 28.0);
    if macros::is_excute(agent) {
        WorkModule::off_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
}

unsafe extern "C" fn wolf_effect_attackairn(agent: &mut L2CAgentBase) {
    let boma = agent.module_accessor;
    frame(agent.lua_state_agent, 1.0);
    macros::FT_MOTION_RATE(agent, 0.3);
    frame(agent.lua_state_agent, 12.0);
    macros::FT_MOTION_RATE(agent, 1.0);
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW(agent, Hash40::new("wolf_ref_loop"), Hash40::new("top"), 0, 6.5, 0, 0, 0, 0, 1, true);
        EffectModule::set_scale_last(boma, &Vector3f{x: 0.5, y: 0.5, z: 0.5});
    }
    wait(agent.lua_state_agent, 10.0);
    if macros::is_excute(agent) {
        macros::EFFECT_OFF_KIND(agent, Hash40::new("wolf_ref_loop"), false, false);
        macros::STOP_SE(agent, Hash40::new("se_wolf_special_l02"));
    }
    frame(agent.lua_state_agent, 22.0);
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW(agent, Hash40::new("wolf_ref_start"), Hash40::new("top"), 0, 6.5, 1, 0, 0, 0, 1, true);
        EffectModule::set_scale_last(boma, &Vector3f{x: 0.7, y: 0.7, z: 0.7});
        macros::EFFECT_FOLLOW(agent, Hash40::new("wolf_ref_flash"), Hash40::new("reflector"), -0.5, 0, 0, 0, 0, 0, 1, true);
        EffectModule::set_scale_last(boma, &Vector3f{x: 0.7, y: 0.7, z: 0.7});
        macros::EFFECT_FOLLOW(agent, Hash40::new("wolf_ref_loop"), Hash40::new("top"), 0, 6.5, 0, 0, 0, 0, 1, true);
        EffectModule::set_scale_last(boma, &Vector3f{x: 0.8, y: 0.8, z: 0.8});
    }
    wait(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        macros::EFFECT_OFF_KIND(agent, Hash40::new("wolf_ref_loop"), false, false);
    
    }
}

unsafe extern "C" fn wolf_sound_attackairn(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 1.0);
    macros::FT_MOTION_RATE(agent, 0.3);
    frame(agent.lua_state_agent, 12.0);
    macros::FT_MOTION_RATE(agent, 1.0);
    if macros::is_excute(agent) {
        macros::PLAY_STATUS(agent, Hash40::new("se_wolf_special_l02"));
    }
    wait(agent.lua_state_agent, 10.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
        macros::EFFECT_OFF_KIND(agent, Hash40::new("wolf_ref_loop"), false, false);
        macros::STOP_SE(agent, Hash40::new("se_wolf_special_l02"));
    }
    frame(agent.lua_state_agent, 22.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_wolf_special_l01"));
    }
    wait(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        macros::EFFECT_OFF_KIND(agent, Hash40::new("wolf_ref_loop"), false, false);
    }
}

pub fn install() {
    Agent::new("wolf")
        //.game_acmd("game_attackhi4", wolf_game_attackhi4, Default)
        //.effect_acmd("effect_attackhi4", wolf_effect_attackhi4, Default)
        //.sound_acmd("sound_attackhi4", wolf_sound_attackhi4, Default)
        .game_acmd("game_attackairn", wolf_game_attackairn, Default)
        .effect_acmd("effect_attackairn", wolf_effect_attackairn, Default)
        .sound_acmd("sound_attackairn", wolf_sound_attackairn, Default)
        .install();
}