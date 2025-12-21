use super::*;

//pub static mut nairboosts: [i32; 8] = [1; 8];

use skyline::hooks::{getRegionAddress, Region};

static mut INT_OFFSET : usize = 0x4e53a0;
static mut FLOAT_OFFSET : usize = 0x4e53e0;

#[skyline::hook(offset=INT_OFFSET)]
pub unsafe fn int_param_accessor_hook(boma: u64, param_type: u64, param_hash: u64) -> i32 {
    let ret = original!()(boma, param_type, param_hash);
    let module_accessor = &mut *(*((boma as *mut u64).offset(1)) as *mut BattleObjectModuleAccessor);
    let fighter_kind = smash::app::utility::get_kind(module_accessor);
    let color = WorkModule::get_int(module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR);
    if param_hash == 0 {
        if fighter_kind == FIGHTER_KIND_NESS {
            if color == 6 || color == 7 {
                //if param_type == hash40("jump_count_max") {
                //    return 2; //2
                //} 
            } 
        }
    }
    ret
}

#[skyline::hook(offset=FLOAT_OFFSET)]
pub unsafe fn float_param_accessor_hook(boma: u64, param_type: u64, param_hash: u64) -> f32 {
    let ret = original!()(boma, param_type, param_hash);
    let module_accessor = &mut *(*((boma as *mut u64).offset(1)) as *mut BattleObjectModuleAccessor);
    let fighter_kind = smash::app::utility::get_kind(module_accessor);
    let color = WorkModule::get_int(module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR);
    if param_hash == 0 {
        if fighter_kind == FIGHTER_KIND_NESS {
            if color == 6 || color == 7 {
                if param_type == hash40("dash_speed") {
                    return 1.9; //1.826
                } else if param_type == hash40("weight") {
                    return 76.0; //94.0
                } else if param_type == hash40("run_speed_max") {
                    return 1.7; //1.609
                } else if param_type == hash40("run_accel_mul") {
                    return 0.08; //0.07161
                } else if param_type == hash40("run_accel_add") {
                    return 0.05; //0.044
                } else if param_type == hash40("jump_initial_y") {
                    return 16.964; //18.964
                } else if param_type == hash40("jump_y") {
                    return 28.48; //34.48
                } else if param_type == hash40("mini_jump_y") {
                    return 13.65; //16.65
                //} else if param_type == hash40("jump_aerial_y") {
                //    return 45.65; //45.65 doesnt seem to adjust ness 2nd jump
                } else if param_type == hash40("air_accel_y") {
                    return 0.082; //0.077
                } else if param_type == hash40("air_speed_y_stable") {
                    return 2.0; //1.31
                } else if param_type == hash40("air_accel_x_mul") {
                    return 0.07; //0.09
                } else if param_type == hash40("air_accel_x_add") {
                    return 0.006; //0.01
                } else if param_type == hash40("air_brake_x") {
                    return 0.018; //0.0225
                } else if param_type == hash40("air_speed_x_stable") {
                    return 1.0; //1.007
                }
            }
        }
    }
    ret
}

fn find_subsequence(haystack: &[u8], needle: &[u8]) -> Option<usize> {
    haystack.windows(needle.len()).position(|window| window == needle)
}

static INT_SEARCH_CODE: &[u8] = &[
    0x00, 0x1c, 0x40, 0xf9, 0x08, 0x00, 0x40, 0xf9, 0x03, 0x11, 0x40, 0xf9,
];

static FLOAT_SEARCH_CODE: &[u8] = &[
    0x00, 0x1c, 0x40, 0xf9, 0x08, 0x00, 0x40, 0xf9, 0x03, 0x19, 0x40, 0xf9,
];

////// fighter frames
unsafe extern "C" fn skullkid_frame(fighter: &mut L2CFighterCommon) {
    unsafe {
        let color = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR);
        let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
        if color == 6 || color == 7 {
            PostureModule::set_scale(fighter.module_accessor, 1.20, false);
            if StatusModule::situation_kind(fighter.module_accessor) == *SITUATION_KIND_GROUND {
                nairboosts[entry_id] = 1;
                //allow_nairboost[entry_id] = true;
            }
            //if MotionModule::motion_kind(fighter.module_accessor) == hash40("jumpsquat") ||
            //MotionModule::motion_kind(fighter.module_accessor) == hash40("jumpfront") ||
            //MotionModule::motion_kind(fighter.module_accessor) == hash40("jumpfrontmini") ||
            //MotionModule::motion_kind(fighter.module_accessor) == hash40("jumpback") ||
            //MotionModule::motion_kind(fighter.module_accessor) == hash40("jumpbackmini") {
            //    if MotionModule::frame(fighter.module_accessor) > 10.0 {
            //        if ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_ATTACK) {
            //            let xCtrl = ControlModule::get_stick_x(fighter.module_accessor);
            //            let yCtrl = ControlModule::get_stick_y(fighter.module_accessor);
            //        if xCtrl == 0.0 && yCtrl == 0.0 {
            //            KineticModule::clear_speed_all(fighter.module_accessor);
            //            allow_nairboost[entry_id] = true;
            //        } else {
            //            allow_nairboost[entry_id] = false;
            //            }
            //        }
            //    }
            //}
        }
    }
}

pub fn install() {
    unsafe {
        let text_ptr = getRegionAddress(Region::Text) as *const u8;
        let text_size = (getRegionAddress(Region::Rodata) as usize) - (text_ptr as usize);
        let text = std::slice::from_raw_parts(text_ptr, text_size);
        if let Some(offset) = find_subsequence(text, INT_SEARCH_CODE) {
            INT_OFFSET = offset;
        }
        if let Some(offset) = find_subsequence(text, FLOAT_SEARCH_CODE) {
            FLOAT_OFFSET = offset;
        }
    }

    skyline::install_hooks!(
        //int_param_accessor_hook,
        float_param_accessor_hook
    );

    Agent::new("ness")
        .on_line(Main, skullkid_frame)
        .install();
}