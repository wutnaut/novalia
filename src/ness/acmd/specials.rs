use super::*;
use crate::EFFECT_FLW_POS_UNSYNC_VIS;

//---------------SPECIALS--------------------

// GROUND SIDE SPECIAL 
unsafe extern "C" fn skullkid_game_specials(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.module_accessor;
    let color = WorkModule::get_int(agent.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR);
    if color == 6 || color == 7 {
        frame(agent.lua_state_agent, 1.0);
        macros::FT_MOTION_RATE(agent, 0.85);
        frame(agent.lua_state_agent, 20.0);
        macros::FT_MOTION_RATE(agent, 1.0);
        frame(agent.lua_state_agent, 21.0);
        if macros::is_excute(agent) {
            ArticleModule::generate_article(agent.module_accessor, FIGHTER_NESS_GENERATE_ARTICLE_SHADOWBALL, false, -1);
            WorkModule::on_flag(agent.module_accessor, *FIGHTER_NESS_STATUS_SPECIAL_S_FLAG_SHOOT);
        }
        macros::FT_MOTION_RATE(agent, 1.0);
    } else {
        frame(agent.lua_state_agent, 1.0);
        macros::FT_MOTION_RATE(agent, 0.85);
        frame(agent.lua_state_agent, 20.0);
        macros::FT_MOTION_RATE(agent, 1.0);
        frame(agent.lua_state_agent, 21.0);
        if macros::is_excute(agent) {
            ArticleModule::generate_article(agent.module_accessor, *FIGHTER_NESS_GENERATE_ARTICLE_PK_FIRE, false, -1);
            WorkModule::on_flag(agent.module_accessor, *FIGHTER_NESS_STATUS_SPECIAL_S_FLAG_SHOOT);
        }
        macros::FT_MOTION_RATE(agent, 1.0);
    }
}

// AIR SIDE SPECIAL
unsafe extern "C" fn skullkid_game_specialairs(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.module_accessor;
    let color = WorkModule::get_int(agent.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR);
    if color == 6 || color == 7 {
        frame(agent.lua_state_agent, 1.0);
        macros::FT_MOTION_RATE(agent, 0.7);
        frame(agent.lua_state_agent, 21.0);
        macros::FT_MOTION_RATE(agent, 1.0);
        if macros::is_excute(agent) {
            ArticleModule::generate_article(agent.module_accessor, FIGHTER_NESS_GENERATE_ARTICLE_SHADOWBALL, false, -1);
            WorkModule::on_flag(agent.module_accessor, *FIGHTER_NESS_STATUS_SPECIAL_S_FLAG_SHOOT);
        }
        macros::FT_MOTION_RATE(agent, 1.0);
    } else {
        frame(agent.lua_state_agent, 1.0);
        macros::FT_MOTION_RATE(agent, 0.85);
        frame(agent.lua_state_agent, 20.0);
        macros::FT_MOTION_RATE(agent, 1.0);
        frame(agent.lua_state_agent, 21.0);
        if macros::is_excute(agent) {
            ArticleModule::generate_article(agent.module_accessor, *FIGHTER_NESS_GENERATE_ARTICLE_PK_FIRE, false, -1);
            WorkModule::on_flag(agent.module_accessor, *FIGHTER_NESS_STATUS_SPECIAL_S_FLAG_SHOOT);
        }
        macros::FT_MOTION_RATE(agent, 1.0);
    }
}

// GROUND HI SPECIAL
unsafe extern "C" fn skullkid_game_specialhi(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        GroundModule::select_cliff_hangdata(agent.module_accessor, *FIGHTER_MEWTWO_CLIFF_HANG_DATA_SPECIAL_HI as u32);
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
    }
}

unsafe extern "C" fn skullkid_effect_specialhi(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::FLASH(agent, 1, 1, 1, 1);
        //macros::EFFECT_FLW_POS_UNSYNC_VIS(agent, Hash40::new("mewtwo_teleport_end"), Hash40::new("top"), 0, 8.5, 0, 0, 0, 0, 0.9, false);
        macros::EFFECT_FOLLOW(agent, Hash40::new("mewtwo_teleport_end"), Hash40::new("top"), 0, 8.5, 0, 0, 0, 0, 0.9, false);
    }
    frame(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        macros::LANDING_EFFECT(agent, Hash40::new("sys_down_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, false);
    }
    for _ in 0..4 {
        if macros::is_excute(agent) {
            macros::FLASH(agent, 0.8, 0.2, 1, 0.5);
        }
        wait(agent.lua_state_agent, 2.0);
        if macros::is_excute(agent) {
            macros::FLASH(agent, 1, 1, 1, 0.8);
        }
        wait(agent.lua_state_agent, 1.0);
    }
    if macros::is_excute(agent) {
        macros::COL_NORMAL(agent);
    }
}

unsafe extern "C" fn skullkid_sound_specialhihold(agent: &mut L2CAgentBase) {
    let color = WorkModule::get_int(agent.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR);
    if color == 6 || color == 7 {
        frame(agent.lua_state_agent, 1.0);
        if macros::is_excute(agent) {
            macros::PLAY_STATUS(agent, Hash40::new("se_ness_special_l01"));
        }
    } else {
        frame(agent.lua_state_agent, 1.0);
        if macros::is_excute(agent) {
            macros::PLAY_STATUS(agent, Hash40::new("se_ness_special_h01"));
        }
    }
}

// AIR HI SPECIAL (PK CANONBALL)
unsafe extern "C" fn effect_specialairhi(agent: &mut L2CAgentBase) {
    let color = WorkModule::get_int(agent.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR);
    if color == 6 || color == 7 {
        if macros::is_excute(agent) {
            macros::EFFECT_FOLLOW(agent, Hash40::new("mewtwo_final_aura"), Hash40::new("rot"), 0, 1, 6, 0, 0, 0, 1, true);
            EffectModule::enable_sync_init_pos_last(agent.module_accessor);
            macros::EFFECT(agent, Hash40::new("mewtwo_pk_attack_c"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 2.0, 0, 0, 0, 0, 0, 0, true);
        }
        for _ in 0..16 {
            if macros::is_excute(agent) {
                macros::BURN_COLOR(agent, 0.5, 0.2, 1, 0.9);
            }
            wait(agent.lua_state_agent, 1.0);
            if macros::is_excute(agent) {
                macros::BURN_COLOR_FRAME(agent, 1, 0.5, 0.2, 1, 0);
                macros::BURN_COLOR_NORMAL(agent);
                macros::FLASH(agent, 0, 0, 0.1, 0.8);
            }
            wait(agent.lua_state_agent, 1.0);
            if macros::is_excute(agent) {
                macros::FLASH_FRM(agent, 1, 0, 0, 0.1, 0);
                macros::COL_NORMAL(agent);
            }
        }
        frame(agent.lua_state_agent, 33.0);
        if macros::is_excute(agent) {
            macros::EFFECT_OFF_KIND(agent, Hash40::new("mewtwo_final_aura"), false, false);
            EffectModule::enable_sync_init_pos_last(agent.module_accessor);
            macros::EFFECT_FOLLOW(agent, Hash40::new("ness_pkt_hold"), Hash40::new("top"), 0, 6, 0, 0, 0, 0, 0.9, true);
        }
        if macros::is_excute(agent) {
            macros::BURN_COLOR(agent, 0.7, 0.2, 1, 0.6);
        }
        wait(agent.lua_state_agent, 1.0);
        if macros::is_excute(agent) {
            macros::BURN_COLOR_FRAME(agent, 1, 0.7, 0.2, 1, 0);
            macros::BURN_COLOR_NORMAL(agent);
        }
        wait(agent.lua_state_agent, 3.0);
        if macros::is_excute(agent) {
            macros::FLASH(agent, 0.8, 0.7, 1, 0.5);
        }
        wait(agent.lua_state_agent, 1.0);
        if macros::is_excute(agent) {
            macros::FLASH_FRM(agent, 1, 0.8, 0.7, 1, 0);
            macros::COL_NORMAL(agent);
        }
        wait(agent.lua_state_agent, 3.0);
    } else {
            if macros::is_excute(agent) {
            macros::EFFECT_FOLLOW(agent, Hash40::new("ness_pkt_attack"), Hash40::new("rot"), 0, 1, 6, 0, 0, 0, 1, true);
            EffectModule::enable_sync_init_pos_last(agent.module_accessor);
            macros::EFFECT(agent, Hash40::new("ness_pkt_bomb"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
        }
        for _ in 0..16 {
            if macros::is_excute(agent) {
                macros::BURN_COLOR(agent, 0.5, 0.2, 1, 0.9);
            }
            wait(agent.lua_state_agent, 1.0);
            if macros::is_excute(agent) {
                macros::BURN_COLOR_FRAME(agent, 1, 0.5, 0.2, 1, 0);
                macros::BURN_COLOR_NORMAL(agent);
                macros::FLASH(agent, 0, 0, 0.1, 0.8);
            }
            wait(agent.lua_state_agent, 1.0);
            if macros::is_excute(agent) {
                macros::FLASH_FRM(agent, 1, 0, 0, 0.1, 0);
                macros::COL_NORMAL(agent);
            }
        }
        frame(agent.lua_state_agent, 33.0);
        if macros::is_excute(agent) {
            macros::EFFECT_OFF_KIND(agent, Hash40::new("ness_pkt_attack"), false, false);
            EffectModule::enable_sync_init_pos_last(agent.module_accessor);
            macros::EFFECT_FOLLOW(agent, Hash40::new("ness_pkt_hold"), Hash40::new("top"), 0, 6, 0, 0, 0, 0, 0.9, true);
        }
        if macros::is_excute(agent) {
            macros::BURN_COLOR(agent, 0.7, 0.2, 1, 0.6);
        }
        wait(agent.lua_state_agent, 1.0);
        if macros::is_excute(agent) {
            macros::BURN_COLOR_FRAME(agent, 1, 0.7, 0.2, 1, 0);
            macros::BURN_COLOR_NORMAL(agent);
        }
        wait(agent.lua_state_agent, 3.0);
        if macros::is_excute(agent) {
            macros::FLASH(agent, 0.8, 0.7, 1, 0.5);
        }
        wait(agent.lua_state_agent, 1.0);
        if macros::is_excute(agent) {
            macros::FLASH_FRM(agent, 1, 0.8, 0.7, 1, 0);
            macros::COL_NORMAL(agent);
        }
        wait(agent.lua_state_agent, 3.0);
    }
}

// GROUND LOW SPECIAL
unsafe extern "C" fn skullkid_game_speciallwhold(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
	let boma = agent.module_accessor;
    let color = WorkModule::get_int(agent.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR);
    if color == 6 || color == 7 {
        for _ in 0..999 {
            if macros::is_excute(agent) {
                macros::ATTACK(agent, 0, 0, Hash40::new("top"), 5.0, 55, 96, 0, 32, 2.5, 0.0, 6.5, 2.0, None, None, None, 0.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_ENERGY);
                macros::ATTACK(agent, 1, 0, Hash40::new("top"), 5.0, 55, 96, 0, 32, 8.9, 0.0, 6.7, 0.0, None, None, None, 0.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_ENERGY);
                macros::ATK_SET_SHIELD_SETOFF_MUL(agent, 0, 0.36);
                macros::ATK_SET_SHIELD_SETOFF_MUL(agent, 1, 0.36);
            }
            wait(lua_state, 5.0);
            if macros::is_excute(agent) {
                AttackModule::clear_all(boma);
            }
            wait(lua_state, 14.0);
        }
    } else {
        for _ in 0..999 {
            if macros::is_excute(agent) {
                macros::ATTACK(agent, 0, 0, Hash40::new("top"), 5.0, 55, 96, 0, 32, 2.5, 0.0, 6.5, 2.0, None, None, None, 0.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_ENERGY);
                macros::ATTACK(agent, 1, 0, Hash40::new("top"), 5.0, 55, 96, 0, 32, 8.9, 0.0, 6.7, 0.0, None, None, None, 0.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_ENERGY);
                macros::ATK_SET_SHIELD_SETOFF_MUL(agent, 0, 0.36);
                macros::ATK_SET_SHIELD_SETOFF_MUL(agent, 1, 0.36);
            }
            wait(lua_state, 5.0);
            if macros::is_excute(agent) {
                AttackModule::clear_all(boma);
            }
            wait(lua_state, 14.0);
        }
    }
}

pub fn install() {
    Agent::new("ness")
        .game_acmd("game_specials", skullkid_game_specials, Default)
        .game_acmd("game_specialairs", skullkid_game_specialairs, Default)
        //.game_acmd("game_specialhi", skullkid_game_specialhi, Default)
        //.effect_acmd("effect_specialhi", skullkid_effect_specialhi, Default)
        .sound_acmd("sound_specialhihold", skullkid_sound_specialhihold, Default)
        //.game_acmd("game_specialairhi", skullkid_game_specialhi, Default)
        //.effect_acmd("effect_specialairhi", skullkid_effect_specialhi, Default)
        //.sound_acmd("sound_specialairhi", skullkid_sound_specialhi, Default)
        .game_acmd("game_speciallwhold", skullkid_game_speciallwhold, Default)
        .game_acmd("game_specialairlwhold", skullkid_game_speciallwhold, Default)
        .install();
}