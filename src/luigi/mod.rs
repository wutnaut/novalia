use super::*;

pub mod acmd;
pub mod fireball;
pub mod status;
pub mod frame;

pub static mut upb_boosts: [i32; 8] = [1; 8];

pub fn install() {
    acmd::install();
    fireball::install();
    status::install();
    frame::install();
}