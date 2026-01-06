use super::*;

use skyline::hooks::{getRegionAddress, Region};

// FIGHTER FRAME
unsafe extern "C" fn fighter_frame(fighter: &mut L2CFighterCommon) {
    unsafe {
        let color = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR);
        let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
        if color == 6 || color == 7 {

        }
    }
}

unsafe extern "C" fn empty_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    0.into()
}

pub fn install() {
    Agent::new("fox")
        //.on_line(Main, fighter_frame)
        .install();
}