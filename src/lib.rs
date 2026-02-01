#![feature(
    concat_idents,
    proc_macro_hygiene
)]
#![allow(
    unused_imports,
	unused_macros,
	unused_variables,
	unused_assignments,
	unused_unsafe,
	non_upper_case_globals,
	non_snake_case,
    clippy::borrow_interior_mutable_const
)]

use smash::app::lua_bind::*;
use smash::lib::lua_const::*;
use smash::app::utility::get_kind;
use smash::lua2cpp::*;
use smashline::*;
use smash_script::*;
use smash::phx::Hash40;
use {
    smash::{
        lua2cpp::*,
        phx::*,
        app::{sv_animcmd::*, lua_bind::*, *},
        lib::{lua_const::*, L2CValue, L2CAgent},
        hash40
    },
    skyline::hooks::InlineCtx,
    skyline::libc::*,
    skyline::nn::ro::LookupSymbol,
    smash2::*,
    smash_script::*,
    smashline::*,
    smashline::Priority::*
};

use crate::ness::FIGHTER_NESS_GENERATE_ARTICLE_SHADOWBALL;
use crate::ness::FIGHTER_NESS_GENERATE_ARTICLE_SHADOWBALLDASH;

use skyline::hooks::{getRegionAddress, Region}; // for param edits

mod novalia;
mod ganon;
mod byleth_axe;
mod zelda;
mod ness;
mod sheik;
mod luigi;
mod toonlink;
mod donkey;
mod buddy;
mod younglink;
mod wolf;
mod rosetta;
mod fox;
mod koopa;


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
                if param_type == hash40("jump_count_max") {
                    return 2; //2
                } 
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
                } else if param_type == hash40("jump_speed_x") {
                    return 0.9; //1.0
                } else if param_type == hash40("jump_speed_x_mul") {
                    return 0.7; //0.8
                } else if param_type == hash40("jump_speed_x_max") {
                    return 1.05; //1.3
                } else if param_type == hash40("jump_aerial_speed_x_mul") {
                    return 0.5; //0.8
                } else if param_type == hash40("jump_initial_y") {
                    return 16.964; //18.964
                } else if param_type == hash40("jump_y") {
                    return 28.48; //34.48
                } else if param_type == hash40("mini_jump_y") {
                    return 13.65; //16.65
                } else if param_type == hash40("jump_aerial_y") {
                    return 29.0; //45.65
                } else if param_type == hash40("air_accel_y") {
                    return 0.082; //0.077
                } else if param_type == hash40("air_speed_y_stable") {
                    return 2.0; //1.31
                } else if param_type == hash40("air_accel_x_mul") {
                    return 0.05; //0.09
                } else if param_type == hash40("air_accel_x_add") {
                    return 0.004; //0.01
                } else if param_type == hash40("air_brake_x") {
                    return 0.015; //0.0225
                } else if param_type == hash40("air_speed_x_stable") {
                    return 1.0; //1.007
                }
            }
        } else if fighter_kind == FIGHTER_KIND_FOX {
            if color == 7 {
                if param_type == hash40("dash_speed") {
                    return 1.9; //2.09
                } else if param_type == hash40("weight") {
                    return 76.0; //77.0
                } else if param_type == hash40("run_speed_max") {
                    return 1.85; //2.402
                } else if param_type == hash40("run_accel_mul") {
                    return 0.95; //0.12221
                } else if param_type == hash40("run_accel_add") {
                    return 0.035; //0.044
                } else if param_type == hash40("jump_speed_x") {
                    return 1.0; //0.68
                } else if param_type == hash40("jump_speed_x_mul") {
                    return 2.0; //0.83
                } else if param_type == hash40("jump_speed_x_max") {
                    return 3.0; //1.7
                } else if param_type == hash40("jump_aerial_speed_x_mul") {
                    return 2.0; //0.8
                } else if param_type == hash40("jump_initial_y") {
                    return 14.964; //19.25
                } else if param_type == hash40("jump_y") {
                    return 18.0; //35
                } else if param_type == hash40("mini_jump_y") {
                    return 11.65; //16.4
                } else if param_type == hash40("jump_aerial_y") {
                    return 20.0; //37
                } else if param_type == hash40("air_accel_y") {
                    return 0.1; //0.23
                } else if param_type == hash40("air_speed_y_stable") {
                    return 1.8; //2.1
                } else if param_type == hash40("air_accel_x_mul") {
                    return 0.05; //0.08
                } else if param_type == hash40("air_accel_x_add") {
                    return 0.01; //0.01
                } else if param_type == hash40("air_brake_x") {
                    return 0.015; //0.015
                } else if param_type == hash40("air_speed_x_stable") {
                    return 1.5; //1.11
                }
            }
        } else if fighter_kind == FIGHTER_KIND_KOOPA {
            if color == 4 || color == 5 || color == 6  || color == 7 {
                if param_type == hash40("dash_speed") {
                    return 2.09; //2.09
                } else if param_type == hash40("weight") {
                    return 77.0; //77.0
                } else if param_type == hash40("run_speed_max") {
                    return 2.402; //2.402
                } else if param_type == hash40("run_accel_mul") {
                    return 0.12221; //0.12221
                } else if param_type == hash40("run_accel_add") {
                    return 0.044; //0.044
                } else if param_type == hash40("jump_speed_x") {
                    return 0.68; //0.68
                } else if param_type == hash40("jump_speed_x_mul") {
                    return 0.83; //0.83
                } else if param_type == hash40("jump_speed_x_max") {
                    return 1.7; //1.7
                } else if param_type == hash40("jump_aerial_speed_x_mul") {
                    return 0.8; //0.8
                } else if param_type == hash40("jump_initial_y") {
                    return 19.25; //19.25
                } else if param_type == hash40("jump_y") {
                    return 35.0; //35
                } else if param_type == hash40("mini_jump_y") {
                    return 16.4; //16.4
                } else if param_type == hash40("jump_aerial_y") {
                    return 37.0; //37
                } else if param_type == hash40("air_accel_y") {
                    return 0.23; //0.23
                } else if param_type == hash40("air_speed_y_stable") {
                    return 2.1; //2.1
                } else if param_type == hash40("air_accel_x_mul") {
                    return 0.08; //0.08
                } else if param_type == hash40("air_accel_x_add") {
                    return 0.01; //0.01
                } else if param_type == hash40("air_brake_x") {
                    return 0.015; //0.015
                } else if param_type == hash40("air_speed_x_stable") {
                    return 1.11; //1.11
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

#[skyline::main(name = "novalia_balance")]
pub fn main() {
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
        int_param_accessor_hook,
        float_param_accessor_hook
    );

    novalia::install();
    ganon::install();
	byleth_axe::install();
	zelda::install();
	ness::install();
	sheik::install();
	luigi::install();
	toonlink::install();
	donkey::install();
	buddy::install();
	younglink::install();
	wolf::install();
	rosetta::install();
    fox::install();
    koopa::install();

	unsafe {
        FIGHTER_NESS_GENERATE_ARTICLE_SHADOWBALL += smashline::clone_weapon("mario", *smash::lib::lua_const::WEAPON_KIND_MARIO_FIREBALL, "ness", "shadowball", false);
        FIGHTER_NESS_GENERATE_ARTICLE_SHADOWBALLDASH += smashline::clone_weapon("luigi", *smash::lib::lua_const::WEAPON_KIND_LUIGI_FIREBALL, "ness", "shadowballdash", false);
    }
}