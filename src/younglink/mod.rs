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
    rng = smash::app::sv_math::rand(hash40("agent"), 1000);
}

unsafe extern "C" fn ylink_game_speciallw(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.module_accessor;
    windwaker_rng_roll();
    frame(lua_state, 17.0);
    if macros::is_excute(agent) {
        if rng == 0 {
            ItemModule::have_item(boma, smash::app::ItemKind(*ITEM_KIND_ASSIST), 0, 0, false, false);
            //ItemModule::born_item(boma, 0);
        }
        if rng >= 1 && rng < 400 {
            WorkModule::on_flag(boma, *FIGHTER_LINK_STATUS_WORK_ID_FLAG_BOMB_GENERATE_LINKBOMB);
        }
        if rng >= 400 && rng < 660{
            ItemModule::have_item(boma, smash::app::ItemKind(*ITEM_KIND_BOMBCHU), 0, 0, false, false);
        }
        if rng >= 660 && rng < 720 {
            ItemModule::have_item(boma, smash::app::ItemKind(*ITEM_KIND_DEKU), 0, 0, false, false);
        }
        if rng >= 720 && rng < 780 {
            ItemModule::attach_item(boma, smash::app::ItemKind(*ITEM_KIND_USAGIHAT), 0, false);
        }
        if rng >= 780 && rng < 840 {
            ItemModule::have_item(boma, smash::app::ItemKind(*ITEM_KIND_BEETLE), 0, 0, false, false);
        }
        if rng >= 840 && rng < 900 {
            ItemModule::have_item(boma, smash::app::ItemKind(*ITEM_KIND_CHICKEN), 0, 0, false, false);
        }
        if rng >= 900 && rng < 960 {
            ItemModule::have_item(boma, smash::app::ItemKind(*ITEM_KIND_FAIRYBOTTLE), 0, 0, false, false);
        }
        if rng >= 960 {
            ItemModule::have_item(boma, smash::app::ItemKind(*ITEM_KIND_MAGICPOT), 0, 0, false, false);
        }
    }
}


unsafe extern "C" fn ylink_sound_speciallw(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.module_accessor;
    frame(agent.lua_state_agent, 17.0);
    if macros::is_excute(agent) {
        if rng >= 1 && rng < 400 {
            macros::PLAY_SE(agent, Hash40::new("se_younglink_special_l01"));
        }
        if rng >= 400 && rng < 660{
            macros::PLAY_STATUS(agent, Hash40::new("se_item_bomchu_run"));
        }
        if rng >= 660 && rng < 720 {
            wait(lua_state, 5.0);
            macros::PLAY_SE(agent, Hash40::new("se_item_bumper_set"));
        }
        if rng >= 720 && rng < 780 {
            // bunny hood makes its own sound
        }
        if rng >= 780 && rng < 840 {
            macros::PLAY_STATUS(agent, Hash40::new("se_item_beetle_fly"));
        }
        if rng >= 840 && rng < 900 {
            macros::PLAY_SE(agent, Hash40::new("se_item_chicken_attack"));
        }
        if rng >= 900 && rng < 960 {
            macros::PLAY_SE(agent, Hash40::new("se_item_fairybottle_fairy"));
        }
        if rng >= 960 {
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

unsafe extern "C" fn ylink_game_speciallw_test(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.module_accessor;
    frame(lua_state, 17.0);
    if macros::is_excute(agent) {
        ItemModule::have_item(boma, smash::app::ItemKind(*ITEM_KIND_ASSIST), 0, 0, false, false);
        //ItemModule::born_item(boma, 0);
    }
}

#[repr(C)]
pub struct CreateItemParam {
    founder_pos: Vector4f,
    item_pos: Vector4f,
    item_kind: ItemKind,
    another_battle_object_id: u32,
    variation_kind: i32,
    lr_dir: f32,
    owner_id: u32,
    unk_20: u32,
    pokeball_or_assist_kind: i32,
    unk_0: u64,
    weird_flag: u64,
    unk_1_weird: u64,
    unk_approx_0: f32,
    unk_02: f32
}

#[skyline::hook(offset = 0x15db0b0)]
pub unsafe fn create_item(item_manager: *mut smash::app::ItemManager, create_item_param: *mut CreateItemParam, unk: bool, unk2: bool, unk3: bool) -> *mut BattleObject {
    if (*create_item_param).pokeball_or_assist_kind == *ITEM_KIND_NONE
    || (*create_item_param).pokeball_or_assist_kind == 0 {
        let list = [*ITEM_KIND_SKULLKID, *ITEM_KIND_MOON];
        let rnd = sv_math::rand(hash40("item"), list.len() as i32) as usize;
        (*create_item_param).pokeball_or_assist_kind = list[rnd];
    }
    original!()(item_manager, create_item_param, unk, unk2, unk3)
}


pub fn install() {
    //skyline::install_hooks!(create_item);
    Agent::new("younglink")
        .game_acmd("game_speciallw", ylink_game_speciallw, Default)
        .game_acmd("game_specialairlw", ylink_game_speciallw, Default)
        .sound_acmd("sound_speciallw", ylink_sound_speciallw, Default)
        .sound_acmd("sound_specialairlw", ylink_sound_speciallw, Default)
        .game_acmd("game_attacks4", ylink_game_attacks4, Default)
        .install();
}