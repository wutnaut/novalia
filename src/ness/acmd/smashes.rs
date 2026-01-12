use super::*;

//---------------SMASH ATTACKS-----------------

// UP SMASH
unsafe extern "C" fn skullkid_game_attackhi4(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
	let boma = agent.module_accessor;
    let color = WorkModule::get_int(agent.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR);
    if color == 6 || color == 7 {
        frame(agent.lua_state_agent, 1.0);
        macros::FT_MOTION_RATE(agent, 0.6);
        if macros::is_excute(agent) {
            ArticleModule::remove_exist(boma, *FIGHTER_NESS_GENERATE_ARTICLE_YOYO, smash::app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
		    ArticleModule::remove_exist(boma, *FIGHTER_NESS_GENERATE_ARTICLE_YOYO_HEAD, smash::app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
        }
        frame(agent.lua_state_agent, 5.0);
        execute(agent.lua_state_agent, 5.0);
        if macros::is_excute(agent) {
            WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
        }
        frame(agent.lua_state_agent, 28.0);
        if macros::is_excute(agent) {
            macros::ATTACK(agent, 0, 0, Hash40::new("top"), 3.0, 110, 20, 0, 50, 5.5, 0.0, 5.0, -7.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_PSI);
            macros::ATTACK(agent, 1, 0, Hash40::new("top"), 3.0, 110, 20, 0, 50, 5.5, 0.0, 5.0, 7.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_PSI);
        }
        frame(agent.lua_state_agent, 30.0);
        macros::FT_MOTION_RATE(agent, 0.7);
        if macros::is_excute(agent) {
            macros::ATTACK(agent, 2, 0, Hash40::new("top"), 21.0, 95, 77, 0, 48, 7.0, 0.0, 16.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_LL, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_PSI);
        }
        frame(agent.lua_state_agent, 33.0);
        if macros::is_excute(agent) {
            AttackModule::clear_all(agent.module_accessor);
            macros::ATTACK(agent, 1, 0, Hash40::new("top"), 20.0, 95, 77, 0, 42, 6.8, 0.0, 20.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_LL, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_PSI);
        }
        frame(agent.lua_state_agent, 38.0);
        if macros::is_excute(agent) {
            macros::ATTACK(agent, 2, 0, Hash40::new("top"), 18.0, 95, 77, 0, 37, 6.6, 0.0, 24.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_LL, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_PSI);
            AttackModule::clear(agent.module_accessor, 1, false);
        }
        frame(agent.lua_state_agent, 43.0);
        if macros::is_excute(agent) {
            macros::ATTACK(agent, 3, 0, Hash40::new("top"), 17.0, 95, 77, 0, 37, 6.4, 0.0, 28.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_PSI);
            AttackModule::clear(agent.module_accessor, 2, false);
        }
        frame(agent.lua_state_agent, 48.0);
        if macros::is_excute(agent) {
            macros::ATTACK(agent, 0, 0, Hash40::new("top"), 16.0, 95, 77, 0, 32, 6.2, 0.0, 32.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_PSI);
            AttackModule::clear(agent.module_accessor, 3, false);
        }
        frame(agent.lua_state_agent, 53.0);
        macros::FT_MOTION_RATE(agent, 6.0);
        if macros::is_excute(agent) {
            macros::ATTACK(agent, 0, 0, Hash40::new("top"), 15.0, 95, 77, 0, 32, 6.0, 0.0, 36.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_PSI);
        }
        frame(agent.lua_state_agent, 55.0);
        if macros::is_excute(agent) {
            AttackModule::clear_all(agent.module_accessor);
        }
         frame(agent.lua_state_agent, 59.0);
        macros::FT_MOTION_RATE(agent, 1.0);
    } else {
        frame(lua_state, 1.0);
        macros::FT_MOTION_RATE(agent, 0.74);
        frame(lua_state, 3.0);
        if macros::is_excute(agent) {
            ArticleModule::set_visibility_whole(boma, *FIGHTER_NESS_GENERATE_ARTICLE_YOYO_HEAD, true, smash::app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
            ArticleModule::set_visibility_whole(boma, *FIGHTER_NESS_GENERATE_ARTICLE_YOYO, true, smash::app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
        }
        frame(lua_state, 11.0);
        if macros::is_excute(agent) {
            ArticleModule::shoot(boma, *FIGHTER_NESS_GENERATE_ARTICLE_YOYO_HEAD, smash::app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL), false);
        }
        frame(lua_state, 12.0);
        macros::FT_MOTION_RATE(agent, 1.0);
        if macros::is_excute(agent) {
            WorkModule::on_flag(boma, /*Flag*/ *FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
        }
        frame(lua_state, 56.0);
        if macros::is_excute(agent) {
            ArticleModule::remove_exist(boma, *FIGHTER_NESS_GENERATE_ARTICLE_YOYO, smash::app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
            ArticleModule::remove_exist(boma, *FIGHTER_NESS_GENERATE_ARTICLE_YOYO_HEAD, smash::app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
        }
    }
}

// UP SMASH EFFECT
unsafe extern "C" fn skullkid_effect_attackhi4(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.module_accessor;
    let color = WorkModule::get_int(agent.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR);
    if color == 6 || color == 7 {
        frame(agent.lua_state_agent, 1.0);
        if macros::is_excute(agent) {
            macros::EFFECT(agent, Hash40::new("sys_smash_flash"), Hash40::new("top"), 0, 13, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
        }
        frame(agent.lua_state_agent, 20.0);
        if macros::is_excute(agent) {
            macros::EFFECT_FOLLOW_FLIP(agent, Hash40::new("ness_psi_hold"), Hash40::new("ness_psi_hold"), Hash40::new("haver"), 0, 0, 0.3, 0, 0, 0, 1, true, *EF_FLIP_YZ);
        }
        frame(agent.lua_state_agent, 23.0);
        if macros::is_excute(agent) {
            macros::EFFECT_OFF_KIND(agent, Hash40::new("ness_psi_hold"), true, true);
            macros::EFFECT_FOLLOW_FLIP(agent, Hash40::new("ness_psi_catch"), Hash40::new("ness_psi_catch"), Hash40::new("top"), 0, 15, -0.5, 0, 0, 0, 1.5, true, *EF_FLIP_YZ);
        }
        frame(agent.lua_state_agent, 25.0);
        if macros::is_excute(agent) {
            macros::LANDING_EFFECT(agent, Hash40::new("sys_crown"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, false);
            macros::LAST_EFFECT_SET_COLOR(agent, 0.58, 0.0, 0.82);
        }
        frame(agent.lua_state_agent, 26.0);
        if macros::is_excute(agent) {
            macros::EFFECT(agent, Hash40::new("mewtwo_pk_attack_a"), Hash40::new("top"), 0, 9, 0, 0, 0, 0, 0.75, 0, 0, 0, 0, 0, 0, true);
            macros::EFFECT_FOLLOW(agent, Hash40::new("mewtwo_pk_attack_c"), Hash40::new("top"), 0, 15, 0, -90, 0, 0, 0.65, true);
        }
        frame(agent.lua_state_agent, 30.0);
        if macros::is_excute(agent) {
            macros::EFFECT_OFF_KIND(agent, Hash40::new("ness_psi_catch"), false, false);
        }
        frame(agent.lua_state_agent, 38.0);
        if macros::is_excute(agent) {
            macros::EFFECT_OFF_KIND(agent, Hash40::new("mewtwo_pk_attack_c"), true, true);
            macros::EFFECT_FOLLOW(agent, Hash40::new("mewtwo_pk_attack_c"), Hash40::new("top"), 0, 25, 0, -90, 0, 0, 0.6, true);
        }
        frame(agent.lua_state_agent, 48.0);
        if macros::is_excute(agent) {
            macros::EFFECT_OFF_KIND(agent, Hash40::new("mewtwo_pk_attack_c"), true, true);
            macros::EFFECT_FOLLOW(agent, Hash40::new("mewtwo_pk_attack_c"), Hash40::new("top"), 0, 30, 0, -90, 0, 0, 0.55, true);
        }
        frame(agent.lua_state_agent, 54.0);
        if macros::is_excute(agent) {
            macros::EFFECT_OFF_KIND(agent, Hash40::new("mewtwo_pk_attack_c"), false, false);
        }
    } else {
        frame(agent.lua_state_agent, 1.0);
        if macros::is_excute(agent) {
            macros::EFFECT(agent, Hash40::new("sys_smash_flash"), Hash40::new("top"), 0, 14, 12, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
        }
        frame(agent.lua_state_agent, 10.0);
        if macros::is_excute(agent) {
            macros::FOOT_EFFECT(agent, Hash40::new("sys_run_smoke"), Hash40::new("top"), 7.5, 0, 0, 0, 180, 0, 1.2, 0, 0, 0, 0, 0, 0, false);
        }
        frame(agent.lua_state_agent, 33.0);
        if macros::is_excute(agent) {
            macros::FOOT_EFFECT(agent, Hash40::new("sys_run_smoke"), Hash40::new("top"), -10, 0, 0, 0, 0, 0, 1.2, 0, 0, 0, 0, 0, 0, false);
        }
    }
}

// UP SMASH CHARGE EFFECT
unsafe extern "C" fn skullkid_effect_attackhi4charge(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.module_accessor;
    let color = WorkModule::get_int(agent.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR);
    if color == 6 || color == 7 {
        frame(agent.lua_state_agent, 1.0);
        if macros::is_excute(agent) {
            macros::EFFECT_FOLLOW_FLIP(agent, Hash40::new("ness_psi_hold"), Hash40::new("ness_psi_hold"), Hash40::new("top"), -4, 3.5, 1, 0, 0, 0, 1, true, *EF_FLIP_YZ);
        }
        if macros::is_excute(agent) {
            macros::FOOT_EFFECT(agent, Hash40::new("sys_run_smoke"), Hash40::new("top"), -4, 0, 0, 0, 0, 0, 1, 5, 0, 4, 0, 0, 0, false);
        }
        wait(agent.lua_state_agent, 5.0);
    } else {
        frame(agent.lua_state_agent, 5.0);
        if macros::is_excute(agent) {
            macros::FOOT_EFFECT(agent, Hash40::new("sys_run_smoke"), Hash40::new("top"), 4, 0, 0, 0, 0, 0, 1, 8, 0, 4, 0, 0, 0, false);
        }
        wait(agent.lua_state_agent, 5.0);
    }
}

// UP SMASH SOUND
unsafe extern "C" fn skullkid_sound_attackhi4(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.module_accessor;
    let color = WorkModule::get_int(agent.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR);
    if color == 6 || color == 7 {
        frame(agent.lua_state_agent, 6.0);
        if macros::is_excute(agent) {
            macros::STOP_SE(agent, Hash40::new("se_common_smash_start_04"));
            macros::PLAY_SE(agent, Hash40::new("se_ness_smash_h01"));
        }
        wait(agent.lua_state_agent, 18.0);
        if macros::is_excute(agent) {
            macros::PLAY_STATUS(agent, Hash40::new("se_ness_smash_h02"));
        }
        wait(agent.lua_state_agent, 1.0);
        if macros::is_excute(agent) {
            macros::PLAY_SE(agent, Hash40::new("vc_ness_attack05"));
        }
        wait(agent.lua_state_agent, 56.0);
        if macros::is_excute(agent) {
            macros::PLAY_SE(agent, Hash40::new("se_ness_landing01"));
        }
    } else {
        frame(agent.lua_state_agent, 14.0);
        if macros::is_excute(agent) {
            macros::STOP_SE(agent, Hash40::new("se_common_smash_start_04"));
            macros::PLAY_SE(agent, Hash40::new("vc_ness_attack06"));
            macros::PLAY_SE(agent, Hash40::new("se_ness_pk_l"));
            macros::PLAY_SE(agent, Hash40::new("se_ness_yoyo_swing"));
        }
        frame(agent.lua_state_agent, 38.0);
        if macros::is_excute(agent) {
            macros::PLAY_SE(agent, Hash40::new("se_ness_yoyo_catch"));
        }
    }
}

// UP SMASH CHARGE SOUND
unsafe extern "C" fn skullkid_sound_attackhi4charge(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.module_accessor;
    let color = WorkModule::get_int(agent.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR);
    if color == 6 || color == 7 {
        frame(agent.lua_state_agent, 2.0);
        if macros::is_excute(agent) {
            macros::PLAY_SE(agent, Hash40::new("se_common_smash_start_04"));
            macros::PLAY_STATUS(agent, Hash40::new("se_ness_pk_charge"));
        }
    } else {
        frame(agent.lua_state_agent, 2.0);
        if macros::is_excute(agent) {
            macros::PLAY_SE(agent, Hash40::new("se_common_smash_start_04"));
            macros::PLAY_STATUS(agent, Hash40::new("se_ness_yoyo_hold"));
        }
    }
}

// DOWN SMASH
unsafe extern "C" fn skullkid_game_attacklw4(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.module_accessor;
    let color = WorkModule::get_int(agent.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR);
    if color == 6 || color == 7 {
        frame(agent.lua_state_agent, 1.0);
        if macros::is_excute(agent) {
            ArticleModule::remove_exist(boma, *FIGHTER_NESS_GENERATE_ARTICLE_YOYO, smash::app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
		    ArticleModule::remove_exist(boma, *FIGHTER_NESS_GENERATE_ARTICLE_YOYO_HEAD, smash::app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
        }
        frame(agent.lua_state_agent, 6.0);
        if macros::is_excute(agent) {
            WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
        }
        frame(agent.lua_state_agent, 20.0);
        if macros::is_excute(agent) {
            macros::ATTACK(agent, 0, 0, Hash40::new("top"), 4.0, 60, 0, 0, 40, 3.5, 0.0, 3.0, 10.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, -10, 0.0, 8, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_PSI);
        }
        wait(agent.lua_state_agent, 3.0);
        if macros::is_excute(agent) {
            AttackModule::set_target_category(agent.module_accessor, 0, *COLLISION_CATEGORY_MASK_NO_IF as u32);
        }
        frame(agent.lua_state_agent, 29.0);
        if macros::is_excute(agent) {
            macros::ATTACK(agent, 0, 0, Hash40::new("top"), 5.0, 60, 0, 0, 40, 4.0, 3.3, 3.5, 15.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, -8, 0.0, 9, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_PSI);
        }
        wait(agent.lua_state_agent, 3.0);
        if macros::is_excute(agent) {
            AttackModule::set_target_category(agent.module_accessor, 0, *COLLISION_CATEGORY_MASK_NO_IF as u32);
        }
        frame(agent.lua_state_agent, 39.0);
        if macros::is_excute(agent) {
            macros::ATTACK(agent, 0, 0, Hash40::new("top"), 10.0, 70, 40, 0, 70, 4.5, -5.6, 3.6, 20.5, None, None, None, 0.35, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, -6, 0.0, 9, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_paralyze"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_PSI);
        }
        wait(agent.lua_state_agent, 7.0);
        if macros::is_excute(agent) {
            AttackModule::clear_all(agent.module_accessor);
        }
    } else {
        frame(lua_state, 3.0);
        if macros::is_excute(agent) {
            ArticleModule::set_visibility_whole(boma, *FIGHTER_NESS_GENERATE_ARTICLE_YOYO_HEAD, true, smash::app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
            ArticleModule::set_visibility_whole(boma, *FIGHTER_NESS_GENERATE_ARTICLE_YOYO, true, smash::app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
        }
        frame(lua_state, 6.0);
        if macros::is_excute(agent) {
            ArticleModule::shoot(boma, *FIGHTER_NESS_GENERATE_ARTICLE_YOYO_HEAD, smash::app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL), false);
        }
        frame(lua_state, 12.0);
        if macros::is_excute(agent) {
            WorkModule::on_flag(boma, /*Flag*/ *FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
        }
        frame(lua_state, 54.0);
        if macros::is_excute(agent) {
            ArticleModule::remove_exist(boma, *FIGHTER_NESS_GENERATE_ARTICLE_YOYO, smash::app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
            ArticleModule::remove_exist(boma, *FIGHTER_NESS_GENERATE_ARTICLE_YOYO_HEAD, smash::app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
        }
    }
}

// DOWN SMASH EFFECT
unsafe extern "C" fn skullkid_effect_attacklw4(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
	let boma = agent.module_accessor;
    let color = WorkModule::get_int(agent.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR);
    if color == 6 || color == 7 {
        frame(agent.lua_state_agent, 3.0);
        if macros::is_excute(agent) {
            macros::EFFECT(agent, Hash40::new("sys_smash_flash"), Hash40::new("havel"), 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
        }
        frame(agent.lua_state_agent, 8.0);
        if macros::is_excute(agent) {
            macros::EFFECT_FOLLOW_FLIP(agent, Hash40::new("ness_psi_hold"), Hash40::new("ness_psi_hold"), Hash40::new("havel"), 0.5, 0.5, 1.3, 0, 0, 0, 1, true, *EF_FLIP_YZ);
        }
        frame(agent.lua_state_agent, 20.0);
        if macros::is_excute(agent) {
            macros::EFFECT_OFF_KIND(agent, Hash40::new("ness_psi_hold"), false, false);
            macros::EFFECT_FOLLOW_FLIP(agent, Hash40::new("mewtwo_pk_attack_c"), Hash40::new("mewtwo_pk_attack_c"), Hash40::new("top"), 0, 2.5, 11, 0, 0, 0, 0.35, true, *EF_FLIP_YZ);
        }
        frame(agent.lua_state_agent, 21.0);
        if macros::is_excute(agent) {
            macros::LANDING_EFFECT(agent, Hash40::new("sys_dash_smoke"), Hash40::new("top"), -3, 0, 0, 0, 0, 0, 0.7, 2, 2, 2, 0, 0, 0, false);
            macros::LAST_EFFECT_SET_RATE(agent, 1.3);
        }
        frame(agent.lua_state_agent, 29.0);
        if macros::is_excute(agent) {
            macros::EFFECT_FOLLOW_FLIP(agent, Hash40::new("mewtwo_pk_attack_c"), Hash40::new("mewtwo_pk_attack_c"), Hash40::new("top"), 0, 2.5, 14.5, 0, 0, 0, 0.4, true, *EF_FLIP_YZ);
        }
        frame(agent.lua_state_agent, 30.0);
        if macros::is_excute(agent) {
            macros::LANDING_EFFECT(agent, Hash40::new("sys_dash_smoke"), Hash40::new("top"), -3, 0, 0, 0, 0, 0, 0.7, 2, 2, 2, 0, 0, 0, false);
            macros::LAST_EFFECT_SET_RATE(agent, 1.3);
        }
        frame(agent.lua_state_agent, 39.0);
        if macros::is_excute(agent) {
            macros::EFFECT_FOLLOW_FLIP(agent, Hash40::new("mewtwo_pk_attack_c"), Hash40::new("mewtwo_pk_attack_c"), Hash40::new("top"), 0, 2.5, 20, 0, 0, 0, 0.5, true, *EF_FLIP_YZ);
        }
        frame(agent.lua_state_agent, 40.0);
        if macros::is_excute(agent) {
            macros::LANDING_EFFECT(agent, Hash40::new("sys_dash_smoke"), Hash40::new("top"), -3, 0, 0, 0, 0, 0, 1, 2, 2, 2, 0, 0, 0, false);
        }
    } else {
        frame(agent.lua_state_agent, 1.0);
        if macros::is_excute(agent) {
            macros::EFFECT(agent, Hash40::new("sys_smash_flash"), Hash40::new("top"), 0, 3.5, 9.5, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
        }
        frame(agent.lua_state_agent, 19.0);
        if macros::is_excute(agent) {
            macros::LANDING_EFFECT(agent, Hash40::new("sys_dash_smoke"), Hash40::new("top"), -12, 0, 0.5, 0, 0, 0, 0.5, 0, 0, 0, 0, 0, 0, false);
        }
        frame(agent.lua_state_agent, 21.0);
        if macros::is_excute(agent) {
            macros::FOOT_EFFECT(agent, Hash40::new("sys_run_smoke"), Hash40::new("top"), 0, 0, 2, 0, 0, 0, 0.6, 3, 0, 1, 0, 0, 0, false);
        }
        frame(agent.lua_state_agent, 22.0);
        if macros::is_excute(agent) {
            macros::FOOT_EFFECT(agent, Hash40::new("sys_run_smoke"), Hash40::new("top"), 8, 0, 3, 0, 0, 0, 0.6, 3, 0, 1, 0, 0, 0, false);
        }
        frame(agent.lua_state_agent, 23.0);
        if macros::is_excute(agent) {
            macros::FOOT_EFFECT(agent, Hash40::new("sys_run_smoke"), Hash40::new("top"), 16, 0, 1, 0, 0, 0, 0.6, 3, 0, 1, 0, 0, 0, false);
        }
        frame(agent.lua_state_agent, 25.0);
        if macros::is_excute(agent) {
            macros::LANDING_EFFECT(agent, Hash40::new("sys_dash_smoke"), Hash40::new("top"), 20, 0, 0.5, 0, 180, 0, 0.5, 2, 0, 0, 0, 0, 0, false);
        }
    }
}
// DOWN SMASH SOUND
unsafe extern "C" fn skullkid_sound_attacklw4(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.module_accessor;
    let color = WorkModule::get_int(agent.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR);
    if color == 6 || color == 7 {
        frame(lua_state, 7.0);
        if macros::is_excute(agent) {
            macros::STOP_SE(agent, Hash40::new("se_common_smash_start_04"));
            macros::PLAY_SE(agent, Hash40::new("se_ness_smash_l04"));
        }
        frame(lua_state, 8.0);
        if macros::is_excute(agent) { }
        wait(lua_state, 9.0);
        if macros::is_excute(agent) {
            macros::PLAY_SE(agent, Hash40::new("vc_ness_attack05"));
        }
        wait(lua_state, 3.0);
        if macros::is_excute(agent) {
            macros::PLAY_SE(agent, Hash40::new("se_ness_smash_l01"));
        }
    } else {
        frame(agent.lua_state_agent, 13.0);
        if macros::is_excute(agent) {
            macros::STOP_SE(agent, Hash40::new("se_common_smash_start_04"));
            macros::PLAY_SE(agent, Hash40::new("vc_ness_attack07"));
            macros::PLAY_SE(agent, Hash40::new("se_ness_pk_l"));
            macros::PLAY_SE(agent, Hash40::new("se_ness_yoyo_swing"));
        }
        frame(agent.lua_state_agent, 26.0);
        if macros::is_excute(agent) {
            macros::PLAY_SE(agent, Hash40::new("se_ness_pk_l"));
            macros::PLAY_SE(agent, Hash40::new("se_ness_yoyo_swing"));
        }
        frame(agent.lua_state_agent, 41.0);
        if macros::is_excute(agent) {
            macros::PLAY_SE(agent, Hash40::new("se_ness_yoyo_catch"));
        }
    }
}

// DOWN SMASH CHARGE EFFECT
unsafe extern "C" fn skullkid_effect_attacklw4charge(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.module_accessor;
    let color = WorkModule::get_int(agent.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR);
    if color == 6 || color == 7 {
        frame(agent.lua_state_agent, 5.0);
        if macros::is_excute(agent) {
            macros::EFFECT_FOLLOW_FLIP(agent, Hash40::new("ness_psi_hold"), Hash40::new("ness_psi_hold"), Hash40::new("havel"), 0.5, 0.5, 1.3, 0, 0, 0, 1, true, *EF_FLIP_YZ);
        }
        if macros::is_excute(agent) {
            macros::FOOT_EFFECT(agent, Hash40::new("sys_run_smoke"), Hash40::new("top"), -4, 0, 0, 0, 0, 0, 1, 4, 0, 6, 0, 0, 0, false);
        }
        wait(agent.lua_state_agent, 7.0);
    } else {
        frame(agent.lua_state_agent, 5.0);
        if macros::is_excute(agent) {
            macros::FOOT_EFFECT(agent, Hash40::new("sys_run_smoke"), Hash40::new("top"), 4, 0, 0, 0, 0, 0, 1, 8, 0, 4, 0, 0, 0, false);
        }
        wait(agent.lua_state_agent, 5.0);
    }
}

// DOWN SMASH CHARGE SOUND
unsafe extern "C" fn skullkid_sound_attacklw4charge(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.module_accessor;
    let color = WorkModule::get_int(agent.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR);
    if color == 6 || color == 7 {
        frame(agent.lua_state_agent, 2.0);
        if macros::is_excute(agent) {
            macros::PLAY_SE(agent, Hash40::new("se_common_smash_start_04"));
            //macros::PLAY_STATUS(agent, Hash40::new("se_ness_yoyo_hold"));
        }
    } else {
        frame(agent.lua_state_agent, 2.0);
        if macros::is_excute(agent) {
            macros::PLAY_SE(agent, Hash40::new("se_common_smash_start_04"));
            macros::PLAY_STATUS(agent, Hash40::new("se_ness_yoyo_hold"));
        }
    }
}

pub fn install() {
    Agent::new("ness")
        .game_acmd("game_attacklw4", skullkid_game_attacklw4, Default)
        .effect_acmd("effect_attacklw4", skullkid_effect_attacklw4, Default)
        .sound_acmd("sound_attacklw4", skullkid_sound_attacklw4, Default)
        .effect_acmd("effect_attacklw4charge", skullkid_effect_attacklw4charge, Default)
        .sound_acmd("sound_attacklw4charge", skullkid_sound_attacklw4charge, Default)
        .game_acmd("game_attackhi4", skullkid_game_attackhi4, Default)
        .effect_acmd("effect_attackhi4", skullkid_effect_attackhi4, Default)
        .effect_acmd("effect_attackhi4charge", skullkid_effect_attackhi4charge, Default)
        .sound_acmd("sound_attackhi4", skullkid_sound_attackhi4, Default)
        .sound_acmd("sound_attackhi4charge", skullkid_sound_attackhi4charge, Default)
        .install();
}