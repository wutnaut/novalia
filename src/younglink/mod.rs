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

static mut rng: i32 = 0;

pub unsafe fn windwaker_rng_roll() {
    rng = smash::app::sv_math::rand(hash40("agent"), 100);
}

unsafe extern "C" fn ylink_game_speciallw(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.module_accessor;
    windwaker_rng_roll();
    frame(lua_state, 17.0);
    if macros::is_excute(agent) {
        if rng >= 0 && rng < 40 {
            WorkModule::on_flag(boma, *FIGHTER_LINK_STATUS_WORK_ID_FLAG_BOMB_GENERATE_LINKBOMB);
        }
        if rng >= 40 && rng < 66{
            ItemModule::have_item(boma, smash::app::ItemKind(*ITEM_KIND_BOMBCHU), 0, 0, false, false);
            //macros::PLAY_SE(agent, Hash40::new("se_item_bomchu_run"));
        }
        if rng >= 66 && rng < 72 {
            ItemModule::have_item(boma, smash::app::ItemKind(*ITEM_KIND_DEKU), 0, 0, false, false);
            //wait(lua_state, 5.0);
            //macros::PLAY_SE(agent, Hash40::new("se_item_bumper_set"));
        }
        if rng >= 72 && rng < 78 {
            ItemModule::attach_item(boma, smash::app::ItemKind(*ITEM_KIND_USAGIHAT), 0, false);
        }
        if rng >= 78 && rng < 84 {
            ItemModule::have_item(boma, smash::app::ItemKind(*ITEM_KIND_BEETLE), 0, 0, false, false);
            //macros::PLAY_SE(agent, Hash40::new("se_item_beetle_fly"));
        }
        if rng >= 84 && rng < 90 {
            ItemModule::have_item(boma, smash::app::ItemKind(*ITEM_KIND_CHICKEN), 0, 0, false, false);
            //macros::PLAY_SE(agent, Hash40::new("se_item_chicken_attack"));
        }
        if rng >= 90 && rng < 96 {
            ItemModule::have_item(boma, smash::app::ItemKind(*ITEM_KIND_FAIRYBOTTLE), 0, 0, false, false);
            //macros::PLAY_SE(agent, Hash40::new("se_item_fairybottle_fairy"));
        }
        if rng >= 96 {
            ItemModule::have_item(boma, smash::app::ItemKind(*ITEM_KIND_MAGICPOT), 0, 0, false, false);
            //macros::PLAY_SE(agent, Hash40::new("se_item_magicpot_oneshot"));
            //wait(lua_state, 10.0);
            //macros::PLAY_SE(agent, Hash40::new("se_item_magicpot_oneshot"));
            //wait(lua_state, 10.0);
            //macros::PLAY_SE(agent, Hash40::new("se_item_magicpot_oneshot"));
        }
        //if rng == 8 {
        //    ItemModule::have_item(boma, smash::app::ItemKind(*ITEM_KIND_HEART), 0, 0, false, false);
        //}
        //if rng == 1 {
        //    ItemModule::have_item(boma, smash::app::ItemKind(*ITEM_KIND_ASSIST), 0, 0, false, false);
        //}
    }
}


unsafe extern "C" fn ylink_sound_speciallw(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.module_accessor;
    frame(agent.lua_state_agent, 17.0);
    if macros::is_excute(agent) {
        if rng >= 0 && rng < 40 {
            macros::PLAY_SE(agent, Hash40::new("se_younglink_special_l01"));
        }
        if rng >= 40 && rng < 66{
            macros::PLAY_STATUS(agent, Hash40::new("se_item_bomchu_run"));
        }
        if rng >= 66 && rng < 72 {
            wait(lua_state, 5.0);
            macros::PLAY_SE(agent, Hash40::new("se_item_bumper_set"));
        }
        if rng >= 72 && rng < 78 {
            // bunny hood makes its own sound
        }
        if rng >= 78 && rng < 84 {
            macros::PLAY_STATUS(agent, Hash40::new("se_item_beetle_fly"));
        }
        if rng >= 84 && rng < 90 {
            macros::PLAY_SE(agent, Hash40::new("se_item_chicken_attack"));
        }
        if rng >= 90 && rng < 96 {
            macros::PLAY_SE(agent, Hash40::new("se_item_fairybottle_fairy"));
        }
        if rng >= 96 {
            macros::PLAY_SE(agent, Hash40::new("se_item_magicpot_oneshot"));
            wait(lua_state, 10.0);
            macros::PLAY_SE(agent, Hash40::new("se_item_magicpot_oneshot"));
            wait(lua_state, 10.0);
            macros::PLAY_SE(agent, Hash40::new("se_item_magicpot_oneshot"));
        }
    }
    wait(lua_state, 20.0);
    macros::STOP_SE(agent, Hash40::new("se_item_bomchu_run"));
    macros::STOP_SE(agent, Hash40::new("se_item_beetle_fly"));
}

unsafe extern "C" fn ylink_game_attacks4(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.module_accessor;
    frame(lua_state, 6.0);
    macros::FT_MOTION_RATE_RANGE(agent, 6.0, 14.0, 7.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
    }
    frame(lua_state, 7.0);
    if macros::is_excute(agent) {
        if ItemModule::is_attach_item(boma, smash::app::ItemKind(*ITEM_KIND_USAGIHAT)) {
            KineticModule::add_speed(agent.module_accessor, &Vector3f{ x: 2.0, y: 0.0, z: 0.0 });
        }
    }
    frame(lua_state, 14.0);
    macros::FT_MOTION_RATE(agent, 2.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("shoulderl"), 14.0, 48, 110, 0, 33, 3.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 10, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        macros::ATTACK(agent, 1, 0, Hash40::new("sword"), 14.0, 48, 110, 0, 33, 3.0, 1.0, 0.0, -1.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 10, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        macros::ATTACK(agent, 2, 0, Hash40::new("sword"), 14.0, 48, 110, 0, 33, 3.0, 6.0, 0.0, -1.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 10, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
    }
    frame(lua_state, 15.0);
    macros::FT_MOTION_RATE(agent, 1.0);
    frame(lua_state, 18.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(boma);
    }
}


pub fn install() {
    Agent::new("younglink")
        .game_acmd("game_speciallw", ylink_game_speciallw, Default)
        .game_acmd("game_specialairlw", ylink_game_speciallw, Default)
        .sound_acmd("sound_speciallw", ylink_sound_speciallw, Default)
        .sound_acmd("sound_specialairlw", ylink_sound_speciallw, Default)
        .game_acmd("game_attacks4", ylink_game_attacks4, Default)
        .install();
}