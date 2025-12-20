use super::*;

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

pub fn install() {
    Agent::new("ness")
        .game_acmd("game_specials", skullkid_game_specials, Default)
        .game_acmd("game_specialairs", skullkid_game_specialairs, Default)
        .install();
}