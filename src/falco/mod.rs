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

unsafe extern "C" fn falco_frame(fighter: &mut L2CFighterCommon) {
    unsafe {
        // No special fall
        if MotionModule::motion_kind(fighter.module_accessor) == hash40("fall_special") {
            fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
            //WorkModule::set_int(fighter.module_accessor, 1, *FIGHTER_INSTANCE_WORK_ID_INT_JUMP_COUNT);
        }
    }
}

unsafe extern "C" fn falco_game_specialsstart(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.module_accessor;
    frame(lua_state, 1.0);
    macros::FT_MOTION_RATE(agent, 0.68);
    if macros::is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_STATUS_ATTACK_AIR_FLAG_LANDING_CLEAR_SPEED);
        WorkModule::on_flag(boma, *FIGHTER_INSTANCE_WORK_ID_FLAG_NO_SPEED_OPERATION_CHK);
        macros::SET_SPEED_EX(agent, 0, 0.8, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
        WorkModule::off_flag(boma, *FIGHTER_INSTANCE_WORK_ID_FLAG_NO_SPEED_OPERATION_CHK);
        JostleModule::set_status(boma, false);
        WorkModule::on_flag(boma, *FIGHTER_STATUS_WORK_ID_FLAG_RESERVE_GRAVITY_STABLE_UNABLE);
    }
}

unsafe extern "C" fn falco_game_specials(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.module_accessor;
    frame(lua_state, 1.0);
    macros::FT_MOTION_RATE(agent, 1.0);
    if macros::is_excute(agent) {
        JostleModule::set_status(boma, false);
    }
    frame(lua_state, 2.0);
    if macros::is_excute(agent) {
        ArticleModule::generate_article(boma, *FIGHTER_FALCO_GENERATE_ARTICLE_ILLUSION, false, -1);
    }
    frame(lua_state, 4.0);
    if macros::is_excute(agent) {
        ArticleModule::generate_article(boma, *FIGHTER_FALCO_GENERATE_ARTICLE_ILLUSION, false, -1);
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ON_DROP_BOTH_SIDES);
        //macros::SET_SPEED_EX(agent, 4.625, 0, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
        //KineticModule::add_speed(agent.module_accessor, &Vector3f{ x: 2.0, y: 0.0, z: 0.0 });
    }
}

unsafe extern "C" fn falco_game_specialairsend(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.module_accessor;
    frame(lua_state, 1.0);
    macros::FT_MOTION_RATE(agent, 0.2);
    frame(lua_state, 2.0);
    if macros::is_excute(agent) { 
        KineticModule::add_speed(agent.module_accessor, &Vector3f{ x: 1.0, y: 0.0, z: 0.0 });
    }
}

pub fn install() {
    Agent::new("falco")
        .on_line(Main, falco_frame)
        .game_acmd("game_specialsstart", falco_game_specialsstart, Default)
        .game_acmd("game_specials", falco_game_specials, Default)
        .game_acmd("game_specialairsend", falco_game_specialairsend, Default)
        .install();
}