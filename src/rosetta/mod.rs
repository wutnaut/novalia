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

unsafe extern "C" fn rosetta_size_frame(fighter: &mut L2CFighterCommon) {
    unsafe {
        PostureModule::set_scale(fighter.module_accessor, 0.9, false);
    }
}

pub fn install() {
    Agent::new("rosetta")
        .on_line(Main, rosetta_size_frame)
        .install();
}