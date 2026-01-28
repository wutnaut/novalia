use super::*;

unsafe extern "C" fn dry_bowser_frame(fighter: &mut L2CFighterCommon) {
    unsafe {
        let color = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR);
        //let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
        if color == 4 || color == 5 || color == 6 || color == 7 {
            PostureModule::set_scale(fighter.module_accessor, 0.6, false);
        }
    }
}

pub fn install() {
    Agent::new("koopa")
        .on_line(Main, dry_bowser_frame)
        .install();
}