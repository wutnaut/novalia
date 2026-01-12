use super::*;


//---------------THROWS--------------------

// FORWARD THROW

// BACK THROW

// UP THROW

// DOWN THROW

// GRAB

unsafe extern "C" fn luigi_game_catch(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.module_accessor;
    macros::FT_MOTION_RATE_RANGE(agent, 1.0, 13.0, 5.0);
    if macros::is_excute(agent) {
        if ArticleModule::is_exist(boma, *FIGHTER_LUIGI_GENERATE_ARTICLE_OBAKYUMU) {
            ArticleModule::remove_exist(boma, *FIGHTER_LUIGI_GENERATE_ARTICLE_OBAKYUMU, smash::app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
        }
        if ArticleModule::is_exist(boma, *FIGHTER_LUIGI_GENERATE_ARTICLE_PLUNGER) {
            ArticleModule::remove_exist(boma, *FIGHTER_LUIGI_GENERATE_ARTICLE_PLUNGER, smash::app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
        }
        ArticleModule::generate_article(boma, *FIGHTER_LUIGI_GENERATE_ARTICLE_OBAKYUMU, false, -1);
        ArticleModule::generate_article(boma, *FIGHTER_LUIGI_GENERATE_ARTICLE_OBAKYUMU, false, -1);
        ArticleModule::change_motion(boma, *FIGHTER_LUIGI_GENERATE_ARTICLE_OBAKYUMU, smash::phx::Hash40::new("catch"), false, 0.0);
        ArticleModule::generate_article(boma, *FIGHTER_LUIGI_GENERATE_ARTICLE_PLUNGER, false, -1);
        ArticleModule::set_visibility_whole(boma, *FIGHTER_LUIGI_GENERATE_ARTICLE_PLUNGER, false, smash::app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
        ArticleModule::set_rate(boma, *FIGHTER_LUIGI_GENERATE_ARTICLE_OBAKYUMU, (13.0-1.0)/5.0);
    }
    frame(lua_state, 13.0);
    macros::FT_MOTION_RATE(agent, 1.0);
    if macros::is_excute(agent) {
        ArticleModule::set_rate(boma, *FIGHTER_LUIGI_GENERATE_ARTICLE_OBAKYUMU, 1.0);
        GrabModule::set_rebound(boma, true);
        macros::SEARCH(agent, 0, 0, Hash40::new("throw"), 3.0, 0.0, 0.0, -1.5, None, None, None, *COLLISION_KIND_MASK_HIT, *HIT_STATUS_MASK_NORMAL, 1, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_FIGHTER, *COLLISION_PART_MASK_BODY_HEAD, false);
    }
    frame(lua_state, 14.0);
    if macros::is_excute(agent) {
        macros::CATCH(agent, 0, Hash40::new("top"), 3.5, 0.0, 6.6, 1.0, Some(0.0), Some(6.6), Some(11.0), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_GA);
        macros::CATCH(agent, 1, Hash40::new("top"), 5.5, 0.0, 5.6, 11.0, Some(0.0), Some(7.6), Some(11.0), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_GA);
        ArticleModule::set_visibility_whole(boma, *FIGHTER_LUIGI_GENERATE_ARTICLE_PLUNGER, true, smash::app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
        search!(agent, *MA_MSC_CMD_SEARCH_SEARCH_SCH_CLR_ALL);
    }
    macros::game_CaptureCutCommon(agent);
    wait(lua_state, 2.0);
    if macros::is_excute(agent) {
        grab!(agent, *MA_MSC_CMD_GRAB_CLEAR_ALL);
        WorkModule::on_flag(boma, *FIGHTER_STATUS_CATCH_FLAG_CATCH_WAIT);
        GrabModule::set_rebound(boma, false);
    }
    wait(lua_state, 20.0);
    if macros::is_excute(agent) {
        ArticleModule::set_visibility_whole(boma, *FIGHTER_LUIGI_GENERATE_ARTICLE_PLUNGER, false, smash::app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
    }
}

pub fn install() {
    Agent::new("luigi")
        .game_acmd("game_catch", luigi_game_catch, Default)
        .install();
}