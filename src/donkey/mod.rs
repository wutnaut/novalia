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

pub static mut dk_upB_boostcount: [i32; 8] = [3; 8];
pub static mut dk_upB_nopress: [bool; 8] = [true; 8];
pub static mut dk_upB_press: [bool; 8] = [false; 8];

unsafe extern "C" fn donkey_game_specialairhi(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.module_accessor;
    let entry_id = WorkModule::get_int(agent.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    if macros::is_excute(agent) {
        macros::FT_MOTION_RATE(agent, 2.0);
    }
    frame(lua_state, 2.0);
    if macros::is_excute(agent) {
        macros::FT_MOTION_RATE(agent, 0.33);
    }
    frame(lua_state, 5.0);
    if macros::is_excute(agent) {
        macros::FT_MOTION_RATE(agent, 0.667);
        HitModule::set_whole(boma, HitStatus(*HIT_STATUS_XLU), 0);
        macros::ATTACK(agent, 1, 0, Hash40::new("armr"), 12.0, 361, 90, 0, 60, 8.0, 0.0, 0.0, 0.0, None, None, None, 1.25, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_BODY);
        macros::ATTACK(agent, 2, 0, Hash40::new("arml"), 12.0, 361, 90, 0, 60, 8.0, 0.0, 0.0, 0.0, None, None, None, 1.25, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_BODY);
        macros::ATTACK(agent, 3, 0, Hash40::new("top"), 12.0, 361, 90, 0, 60, 5.0, 0.0, 10.5, -5.0, None, None, None, 1.25, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_BODY);
    }
    frame(lua_state, 10.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(boma);
        HitModule::set_whole(boma, smash::app::HitStatus(*HIT_STATUS_NORMAL), 0);
        macros::FT_MOTION_RATE(agent, 1.0);
    }
    frame(lua_state, 19.0);
    // looping decreasingly effective multihits
    let mut damage = 6.0;
    for _ in 0..7 {
        if macros::is_excute(agent) {
            macros::ATTACK(agent, 0, 0, Hash40::new("top"), damage + 1.0, 361, 80, 0, 30, 8.0, 0.0, 11.0, 15.0, Some(0.0), Some(11.0), Some(-6.0), 1.0, 1.5, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_BODY);
        }
        if dk_upB_boostcount[entry_id] > 0 {
            if ControlModule::check_button_on(agent.module_accessor, *CONTROL_PAD_BUTTON_SPECIAL) && dk_upB_nopress[entry_id] == true { 
                KineticModule::add_speed(agent.module_accessor, &Vector3f{ x: 0.0, y: 0.2, z: 0.0 });
                //KineticModule::add_speed(agent.module_accessor, &Vector3f{ x: 0.0, y: (0.15 + (5.0 - (dk_upB_boostcount as f32) / 100.0)), z: 0.0 });
                macros::EFFECT(agent, Hash40::new("sys_attack_speedline"), Hash40::new("top"), 0, 6, 0, 90, 0, 0, 1.1, 0, 0, 0, 0, 0, 0, true);
                dk_upB_press[entry_id] = true;
                dk_upB_nopress[entry_id] = false;
                dk_upB_boostcount[entry_id] -= 1;
            }
        }
        wait(lua_state, 2.0);
        if ControlModule::check_button_off(agent.module_accessor, *CONTROL_PAD_BUTTON_SPECIAL) { 
            dk_upB_nopress[entry_id] = true;
            dk_upB_press[entry_id] = false;
        }
        if macros::is_excute(agent) {
            AttackModule::clear_all(boma);
            damage = damage * 0.5;
        }
        wait(lua_state, 5.0);
    }
    frame(lua_state, 62.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_DONKEY_STATUS_SPECIAL_HI_FLAG_YACL_DEFAULT);
        WorkModule::on_flag(boma, *FIGHTER_DONKEY_STATUS_SPECIAL_HI_FLAG_GROUND_SPINEND);
        //old finisher
        //ATTACK(agent, 0, 0, Hash40::new("top"), 4.0, 361, 85, 0, 75, 8.0, 0.0, 11.0, 20.0, Some(0.0), Some(11.0), Some(-4.0), 1.25, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_BODY);
    }
    wait(lua_state, 1.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(boma);
    }
    wait(lua_state, 1.0);
    if macros::is_excute(agent) {
        MotionModule::set_rate(boma, 1.5);
    }
}

unsafe extern "C" fn dk_upbboost_frame(fighter: &mut L2CFighterCommon) {
    unsafe {
        //let color = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR);
        let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
        if StatusModule::situation_kind(fighter.module_accessor) == *SITUATION_KIND_GROUND {
            dk_upB_boostcount[entry_id] = 3;
        }
    }
}

pub fn install() {
    Agent::new("donkey")
        .game_acmd("game_specialairhi", donkey_game_specialairhi, Default)    
        .on_line(Main, dk_upbboost_frame)
        .install();
}