use super::*;

pub mod acmd;
pub mod frame;
pub mod status;
pub mod shadowball;
pub mod shadowballdash;
pub mod pkthunder;

pub static mut FIGHTER_NESS_GENERATE_ARTICLE_SHADOWBALL :i32 = 0x8;
pub static mut FIGHTER_NESS_GENERATE_ARTICLE_SHADOWBALLDASH :i32 = 0x8;

pub const WEAPON_SHADOWBALL_STATUS_KIND_REGULAR: i32 = 0;
pub const WEAPON_SHADOWBALLDASH_STATUS_KIND_REGULAR: i32 = 0;

//pub const FIGHTER_SKULLKID_STATUS_KIND_SPECIAL_HI: i32 = 0;
//pub const FIGHTER_SKULLKID_STATUS_KIND_SPECIAL_HI2: i32 = 0;
//pub const FIGHTER_SKULLKID_STATUS_KIND_SPECIAL_HI3: i32 = 0;


pub static mut nairboosts: [i32; 8] = [1; 8];
pub static mut earlyjump: [bool; 8] = [false; 8];

pub fn install() {
    acmd::install();
    frame::install();
    status::install();
    shadowball::install();
    shadowballdash::install();
    pkthunder::install();
}