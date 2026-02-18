use super::*;

pub static mut delayframes: [i32; 8] = [30; 8];

unsafe extern "C" fn robin_green_frame(fighter: &mut L2CFighterCommon) {
    unsafe {
        if MotionModule::motion_kind(fighter.module_accessor) == hash40("attack_100") ||
        MotionModule::motion_kind(fighter.module_accessor) == hash40("attack_100_start") ||
        MotionModule::motion_kind(fighter.module_accessor) == hash40("attack_100_end") ||
        MotionModule::motion_kind(fighter.module_accessor) == hash40("special_hi") ||
        MotionModule::motion_kind(fighter.module_accessor) == hash40("special_hi2") || 
        //StatusModule::status_kind(fighter.module_accessor) == statuses::reflet::FLOAT
        (StatusModule::situation_kind(fighter.module_accessor) == *SITUATION_KIND_AIR && KineticModule::get_sum_speed_y(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN) == 0.0) {
        } else {
            let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
            if delayframes[entry_id] > 0 {
                delayframes[entry_id] -= 1;
            } else {
                if WorkModule::get_int(fighter.module_accessor, *FIGHTER_REFLET_INSTANCE_WORK_ID_INT_SPECIAL_HI_CURRENT_POINT) < 20 {
                    WorkModule::inc_int(fighter.module_accessor, *FIGHTER_REFLET_INSTANCE_WORK_ID_INT_SPECIAL_HI_CURRENT_POINT);
                }
                delayframes[entry_id] = 30;
            }
        }
    }
}

pub fn install() {
    Agent::new("reflet")
        .on_line(Main, robin_green_frame)
        .install();
}