use super::*;

pub mod acmd;
pub mod frame;
pub mod status;
pub mod shadowball;
pub mod shadowballdash;

//use crate::ness::install;

pub static mut FIGHTER_NESS_GENERATE_ARTICLE_SHADOWBALL :i32 = 0x8;
pub static mut FIGHTER_NESS_GENERATE_ARTICLE_SHADOWBALLDASH :i32 = 0x8;

pub const WEAPON_SHADOWBALL_STATUS_KIND_REGULAR: i32 = 0;
pub const WEAPON_SHADOWBALLDASH_STATUS_KIND_REGULAR: i32 = 0;

pub static mut nairboosts: [i32; 8] = [1; 8];

pub fn install() {
    acmd::install();
    frame::install();
    status::install();
    shadowball::install();
    shadowballdash::install();
}