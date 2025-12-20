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

unsafe extern "C" fn ganon_game_specialairlw(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.module_accessor;
    frame(lua_state, 16.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_GANON_STATUS_WORK_ID_FLAG_GANON_KICK_WALL_CHECK);
        JostleModule::set_status(boma, false);
        macros::ATTACK(agent, 0, 0, Hash40::new("legl"), 15.0, 290, 46, 0, 46, 5.0, 9.5, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 25, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        macros::ATTACK(agent, 1, 0, Hash40::new("legl"), 15.0, 290, 46, 0, 46, 3.5, 5.5, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 25, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
    }
    frame(lua_state, 19.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("legl"), 14.0, 290, 100, 0, 50, 5.0, 9.5, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 25, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        macros::ATTACK(agent, 1, 0, Hash40::new("legl"), 14.0, 290, 100, 0, 50, 3.5, 5.5, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 25, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
    }
    frame(lua_state, 39.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(boma);
        JostleModule::set_status(boma, true);
    }
}

unsafe extern "C" fn ganon_game_attackairlw(agent: &mut L2CAgentBase) {  // dair
    let lua_state = agent.lua_state_agent;
    let boma = agent.module_accessor;
    frame(lua_state, 1.0);
    if macros::is_excute(agent) {
        //FighterAreaModuleImpl::enable_fix_jostle_area_xy(boma, 4.5, 4.5, 12.5, 0);
    }
    frame(lua_state, 4.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
    frame(lua_state, 16.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 19.0, 270,  73, 0,  0, 7.0, 0.0,  1.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_KICK);
        macros::ATTACK(agent, 1, 0, Hash40::new("top"), 17.0, 270, 100, 0, 20, 6.0, 0.0, 10.0, 1.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_KICK);
    }
    wait(lua_state, 4.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(boma);
        //FighterAreaModuleImpl::enable_fix_jostle_area_xy(boma, 4.5, 4.5, 12.5, 11);
    }
    frame(lua_state, 29.0);
    if macros::is_excute(agent) {
        WorkModule::off_flag(boma, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
}

unsafe extern "C" fn ganon_game_attack11(agent: &mut L2CAgentBase) { // jab
    let lua_state = agent.lua_state_agent;
    let boma = agent.module_accessor;
    if WorkModule::get_int(agent.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) == 6 {
        frame(lua_state, 1.0);
        macros::FT_MOTION_RATE(agent, 3.0 / 4.0);
        frame(lua_state, 5.0);
        macros::FT_MOTION_RATE(agent, 1.0);
        frame(lua_state, 6.0);
        if macros::is_excute(agent) {
            macros::ATTACK(agent, 0, 0, Hash40::new("top"), 5.0, 50, 40, 0, 95, 5.0, 0.0, 12.0, 7.0, Some(0.0), Some(15.5), Some(7.0), 1.0, 1.5, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_PUNCH);
            macros::ATTACK(agent, 1, 0, Hash40::new("top"), 5.0, 50, 40, 0, 95, 3.5, 0.0, 12.5, 12.0, Some(0.0), Some(15.5), Some(12.0), 1.0, 1.5, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_PUNCH);
            macros::ATTACK(agent, 2, 0, Hash40::new("top"), 5.0, 50, 40, 0, 95, 2.0, 0.0, 13.0, 15.5, Some(0.0), Some(15.5), Some(15.5), 1.0, 1.5, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_PUNCH);
            shield!(agent, *MA_MSC_CMD_REFLECTOR, *COLLISION_KIND_REFLECTOR, 
            0, // ID
            Hash40::new("top"), // Bone
            8.0, // Size
            0.0, 12.0, 12.0, // X1, Y1, Z1
            0.0, 12.0, 12.0, // X2, Y2, Z2
            3.0, // Damage multiplier
            0.2, // Speed multiplier
            45, // Max damage
            false, // Unknown
            10.0, // Life multiplier
            *FIGHTER_REFLECTOR_GROUP_HOMERUNBAT); // "type"
            macros::ATK_SET_SHIELD_SETOFF_MUL(agent, 0, 2.0);
            macros::ATK_SET_SHIELD_SETOFF_MUL(agent, 1, 2.0);
            macros::ATK_SET_SHIELD_SETOFF_MUL(agent, 2, 2.0);
        }
        frame(lua_state, 9.0);
        macros::FT_MOTION_RATE_RANGE(agent, 9.0, 30.0, 16.0);
        if macros::is_excute(agent) {
            AttackModule::clear_all(boma);
            shield!(agent, *MA_MSC_CMD_SHIELD_OFF, *COLLISION_KIND_REFLECTOR, 0, *FIGHTER_REFLECTOR_GROUP_HOMERUNBAT);
        }
        frame(lua_state, 30.0);
        macros::FT_MOTION_RATE(agent, 1.0);
        if macros::is_excute(agent) {
            AttackModule::clear_all(boma);
        }
        } 
    else {
        frame(lua_state, 1.0);
        macros::FT_MOTION_RATE(agent, 3.0 / 4.0);
        frame(lua_state, 5.0);
        macros::FT_MOTION_RATE(agent, 1.0);
        frame(lua_state, 6.0);
        if macros::is_excute(agent) {
            macros::ATTACK(agent, 0, 0, Hash40::new("top"), 5.0, 50, 40, 0, 95, 5.0, 0.0, 12.0, 7.0, Some(0.0), Some(15.5), Some(7.0), 1.0, 1.5, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_PUNCH);
            macros::ATTACK(agent, 1, 0, Hash40::new("top"), 5.0, 50, 40, 0, 95, 3.5, 0.0, 12.5, 12.0, Some(0.0), Some(15.5), Some(12.0), 1.0, 1.5, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_PUNCH);
            macros::ATTACK(agent, 2, 0, Hash40::new("top"), 5.0, 50, 40, 0, 95, 2.0, 0.0, 13.0, 15.5, Some(0.0), Some(15.5), Some(15.5), 1.0, 1.5, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_PUNCH);
        }
        frame(lua_state, 9.0);
        macros::FT_MOTION_RATE_RANGE(agent, 9.0, 30.0, 16.0);
        if macros::is_excute(agent) {
            AttackModule::clear_all(boma);
        }
        frame(lua_state, 30.0);
        macros::FT_MOTION_RATE(agent, 1.0);
    }
}

unsafe extern "C" fn ganon_game_attackhi3(agent: &mut L2CAgentBase) {  // up tilt
    let lua_state = agent.lua_state_agent;
    let boma = agent.module_accessor;
    if WorkModule::get_int(agent.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) == 6 {
        if macros::is_excute(agent) {
            ArticleModule::remove_exist(boma, *FIGHTER_GANON_GENERATE_ARTICLE_SWORD, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
            ArticleModule::generate_article(boma, *FIGHTER_GANON_GENERATE_ARTICLE_SWORD, false, 0);
        }
        frame(lua_state, 1.0);
        if macros::is_excute(agent) {
            KineticModule::add_speed(agent.module_accessor, &Vector3f{ x: 2.0, y: 0.0, z: 0.0 });
        }
        frame(lua_state, 11.0);
        macros::FT_MOTION_RATE(agent, 0.5);
        frame(lua_state, 13.0);
        macros::FT_MOTION_RATE(agent, 1.0);
        if macros::is_excute(agent) {
            macros::ATTACK(agent, 0, 0, Hash40::new("haver"), 14.0, 78, 50, 0, 20, 5.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_SWORD);
            macros::ATTACK(agent, 1, 0, Hash40::new("haver"), 14.0, 78, 50, 0, 20, 4.5, 0.0, 6.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_SWORD);
            macros::ATTACK(agent, 2, 0, Hash40::new("haver"), 14.0, 78, 50, 0, 20, 4.5, 0.0, 12.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_SWORD);
            macros::ATTACK(agent, 3, 0, Hash40::new("haver"), 14.0, 78, 50, 0, 20, 4.5, 0.0, 18.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_SWORD);
            macros::ATTACK(agent, 4, 0, Hash40::new("armr"), 14.0, 78, 50, 0, 20, 3.5, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_SWORD);
        }
        frame(lua_state, 15.0);
        macros::FT_MOTION_RATE_RANGE(agent, 15.0, 27.0, 7.0);
        if macros::is_excute(agent) {
            AttackModule::clear(boma, 3, false);
            AttackModule::clear(boma, 4, false);
            macros::ATTACK(agent, 0, 0, Hash40::new("haver"), 8.0, 78, 50, 0, 20, 5.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_SWORD);
            macros::ATTACK(agent, 1, 0, Hash40::new("haver"), 8.0, 78, 50, 0, 20, 4.5, 0.0, 6.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_SWORD);
            macros::ATTACK(agent, 2, 0, Hash40::new("haver"), 8.0, 78, 50, 0, 20, 4.5, 0.0, 12.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_SWORD);
        }
        frame(lua_state, 27.0);
        macros::FT_MOTION_RATE_RANGE(agent, 27.0, 38.0, 16.0);
        if macros::is_excute(agent) {
            AttackModule::clear_all(boma);
        }
        frame(lua_state, 38.0);
        macros::FT_MOTION_RATE(agent, 1.0);
        frame(lua_state, 41.0);
        if macros::is_excute(agent) {
            ArticleModule::remove_exist(boma, *FIGHTER_GANON_GENERATE_ARTICLE_SWORD, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
        }
    } else {
        if macros::is_excute(agent) {
            ArticleModule::remove_exist(boma, *FIGHTER_GANON_GENERATE_ARTICLE_SWORD, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
            ArticleModule::generate_article(boma, *FIGHTER_GANON_GENERATE_ARTICLE_SWORD, false, 0);
        }
        frame(lua_state, 11.0);
        macros::FT_MOTION_RATE(agent, 0.5);
        frame(lua_state, 13.0);
        macros::FT_MOTION_RATE(agent, 1.0);
        if macros::is_excute(agent) {
            macros::ATTACK(agent, 0, 0, Hash40::new("haver"), 14.0, 78, 75, 0, 50, 5.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_SWORD);
            macros::ATTACK(agent, 1, 0, Hash40::new("haver"), 14.0, 78, 75, 0, 50, 4.5, 0.0, 6.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_SWORD);
            macros::ATTACK(agent, 2, 0, Hash40::new("haver"), 14.0, 78, 75, 0, 50, 4.5, 0.0, 12.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_SWORD);
            macros::ATTACK(agent, 3, 0, Hash40::new("haver"), 14.0, 78, 75, 0, 50, 4.5, 0.0, 18.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_SWORD);
            macros::ATTACK(agent, 4, 0, Hash40::new("armr"), 14.0, 78, 75, 0, 50, 3.5, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_SWORD);
        }
        frame(lua_state, 15.0);
        macros::FT_MOTION_RATE_RANGE(agent, 15.0, 27.0, 7.0);
        if macros::is_excute(agent) {
            AttackModule::clear(boma, 3, false);
            AttackModule::clear(boma, 4, false);
            macros::ATTACK(agent, 0, 0, Hash40::new("haver"), 8.0, 78, 75, 0, 50, 5.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_SWORD);
            macros::ATTACK(agent, 1, 0, Hash40::new("haver"), 8.0, 78, 75, 0, 50, 4.5, 0.0, 6.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_SWORD);
            macros::ATTACK(agent, 2, 0, Hash40::new("haver"), 8.0, 78, 75, 0, 50, 4.5, 0.0, 12.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_SWORD);
        }
        frame(lua_state, 27.0);
        macros::FT_MOTION_RATE_RANGE(agent, 27.0, 38.0, 16.0);
        if macros::is_excute(agent) {
            AttackModule::clear_all(boma);
        }
        frame(lua_state, 38.0);
        macros::FT_MOTION_RATE(agent, 1.0);
        frame(lua_state, 41.0);
        if macros::is_excute(agent) {
            ArticleModule::remove_exist(boma, *FIGHTER_GANON_GENERATE_ARTICLE_SWORD, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
        }
    }
}

#[skyline::hook(replace = sv_animcmd::ATTACK)]
unsafe fn attack_replace(lua_state: u64) {
    // Declare boma and fighter_kind from lua_state parameter
    let boma = smash::app::sv_system::battle_object_module_accessor(lua_state);
    let fighter_kind = smash::app::utility::get_kind(boma);
    
    // Declare l2c_agent from lua_state parameter
    let mut l2c_agent = L2CAgent::new(lua_state);
    // Declare hitbox_params as a growable array from stack we're getting from lua_state
    let hitbox_params: Vec<L2CValue> = (0..36).map(|i| l2c_agent.pop_lua_stack(i + 1)).collect();
    // Clear stack
    l2c_agent.clear_lua_stack();
    // Repeat 36 times
    for i in 0..36 {
        if fighter_kind == FIGHTER_KIND_GANON {
            // If index of for loop is a specific number, set it to a certain value
            if i == 32 {
                l2c_agent.push_lua_stack(&mut L2CValue::new_int(hash40("collision_attr_purple")));
            }
            // If index isn't one of the aforementioned values, use the values originally passed into the function
            else {
                l2c_agent.push_lua_stack(&mut hitbox_params[i].clone());
            }  
        } else {
            l2c_agent.push_lua_stack(&mut hitbox_params[i].clone());
        }
    }
    original!()(lua_state);
}

pub fn install() {
    Agent::new("ganon")
        .game_acmd("game_attackairlw", ganon_game_attackairlw, Default)    
        .game_acmd("game_attack11", ganon_game_attack11, Default)
        .game_acmd("game_attackhi3", ganon_game_attackhi3, Default)
        .game_acmd("game_specialairlw", ganon_game_specialairlw, Default)
        .install();
    skyline::install_hooks!(
        attack_replace
    );
}
