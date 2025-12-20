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

unsafe extern "C" fn global_fighter_frame_meteor_cancel(fighter: &mut L2CFighterCommon) {
    unsafe {
        if MotionModule::motion_kind(fighter.module_accessor) == hash40("fall_down") {
            fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
        }
    }
}

pub fn install() {
    Agent::new("fighter")
        .on_line(Main, global_fighter_frame_meteor_cancel)
        .install();
}