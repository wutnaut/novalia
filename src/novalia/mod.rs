use super::*;

static mut already_too_intense: bool = false;

unsafe extern "C" fn intense_endgame_frame(fighter: &mut L2CFighterCommon) {
    unsafe {
        if already_too_intense {
            //return;
        } else {
            let boma1 = sv_battle_object::module_accessor(Fighter::get_id_from_entry_id(0));
            let boma2 = sv_battle_object::module_accessor(Fighter::get_id_from_entry_id(1));
            //if DamageModule::damage(boma1, 0) >= 100.0 && DamageModule::damage(boma2, 0) >= 100.0 {
            if DamageModule::damage(boma2, 0) >= 100.0 {
                EffectModule::req_screen(fighter.module_accessor, Hash40::new("bg_demon_final"), false, false, false);
                already_too_intense = true;
            }
        }
    }
}

pub fn install() {
    Agent::new("fighter")
        //.on_line(Main, intense_endgame_frame)
        .install();
}