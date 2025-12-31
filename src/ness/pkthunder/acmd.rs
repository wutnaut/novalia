use super::*;


unsafe extern "C" fn skullkid_game_move(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
	let boma = agent.module_accessor;
    let color = WorkModule::get_int(agent.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR);
    if color == 6 || color == 7 {
        if macros::is_excute(agent) {
            macros::ATTACK(agent, 0, 0, Hash40::new("top"), 11.0, 110, 55, 0, 70, 4.4, 0.0, 0.0, 0.0, None, None, None, 0.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 48, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_PSI);
        } 
    } else {
        if macros::is_excute(agent) {
            macros::ATTACK(agent, 0, 0, Hash40::new("top"), 11.0, 110, 55, 0, 70, 4.4, 0.0, 0.0, 0.0, None, None, None, 0.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 48, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_PSI);
        }
    }
}

unsafe extern "C" fn skullkid_effect_move(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
	let boma = agent.module_accessor;
    let color = WorkModule::get_int(agent.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR);
    if color == 6 || color == 7 {
        if macros::is_excute(agent) {
            macros::EFFECT_FOLLOW(agent, Hash40::new("mewtwo_shadowball"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, true);
            macros::EFFECT_FOLLOW(agent, Hash40::new("mewtwo_shadowball_tail"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, true);
        } 
    } else {
        if macros::is_excute(agent) {
            macros::EFFECT_FOLLOW(agent, Hash40::new("ness_pkt_bullet"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, true);
        }
    }
}

/* 
unsafe extern "C" fn skullkid_sound_move(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::SET_TAKEOUT_SE_STATUS(agent, Hash40::new("se_ness_special_h02"));
        macros::PLAY_STATUS(agent, Hash40::new("se_ness_special_h02"));
    }
}
*/

unsafe extern "C" fn skullkid_game_movechild(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
	let boma = agent.module_accessor;
    let color = WorkModule::get_int(agent.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR);
    if color == 6 || color == 7 {
        if macros::is_excute(agent) {
            macros::ATTACK(agent, 1, 1, Hash40::new("top"), 1.0, 361, 60, 0, 20, 2.3, 0.0, 0.0, 0.0, None, None, None, 0.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 48, false, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_PSI);
        } 
    } else {
        if macros::is_excute(agent) {
            macros::ATTACK(agent, 1, 1, Hash40::new("top"), 1.0, 361, 60, 0, 20, 2.3, 0.0, 0.0, 0.0, None, None, None, 0.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 48, false, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_PSI);
        }
    }
}

pub fn install() {
    Agent::new("ness_pkthunder")
        .game_acmd("game_move", skullkid_game_move, Default)
        .effect_acmd("effect_move", skullkid_effect_move, Default)
        //.sound_acmd("sound_move", skullkid_sound_move, Default)
        .game_acmd("game_movechild", skullkid_game_movechild, Default)
        .install();
}