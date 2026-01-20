use super::*;

pub mod acmd;
pub mod fireball;

pub fn install() {
    acmd::install();
    fireball::install();
}