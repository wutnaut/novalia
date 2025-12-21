use super::*;

//const WEAPON_SHADOWBALL_STATUS_KIND_REGULAR: i32 = 0;

unsafe extern "C" fn luigifireball_start_pre(weapon: &mut L2CWeaponCommon) -> L2CValue {
    StatusModule::init_settings(weapon.module_accessor, smash::app::SituationKind(*SITUATION_KIND_AIR), *WEAPON_KINETIC_TYPE_NORMAL, GROUND_CORRECT_KIND_AIR.into(), smash::app::GroundCliffCheckKind(0), false, 0, 0, 0, 0);
    return 0.into();
}

unsafe extern "C" fn luigifireball_start_main(weapon: &mut L2CWeaponCommon) -> L2CValue {
    MotionModule::change_motion(weapon.module_accessor, Hash40::new("regular"), 0.0, 1.0, false, 0.0, false, false);
    weapon.fastshift(L2CValue::Ptr(luigifireball_start_main_loop as *const () as _));
    return 0.into();
}

unsafe extern "C" fn luigifireball_start_main_loop(weapon: &mut L2CWeaponCommon) -> L2CValue {
    return 0.into();
}


unsafe extern "C" fn luigifireball_start_end(weapon: &mut L2CWeaponCommon) -> L2CValue {
    return 0.into();
}

pub fn install() {
    Agent::new("ness_shadowball")
        .status(Pre, WEAPON_SHADOWBALL_STATUS_KIND_REGULAR, luigifireball_start_pre)
        .status(Main, WEAPON_SHADOWBALL_STATUS_KIND_REGULAR, luigifireball_start_main)
        .status(End, WEAPON_SHADOWBALL_STATUS_KIND_REGULAR, luigifireball_start_end)
        .install();
}