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

#[skyline::main(name = "novalia_balance")]
pub fn main() {
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

	unsafe {
        FIGHTER_NESS_GENERATE_ARTICLE_SHADOWBALL += smashline::clone_weapon("mario", *smash::lib::lua_const::WEAPON_KIND_MARIO_FIREBALL, "ness", "shadowball", false);
        FIGHTER_NESS_GENERATE_ARTICLE_SHADOWBALLDASH += smashline::clone_weapon("luigi", *smash::lib::lua_const::WEAPON_KIND_LUIGI_FIREBALL, "ness", "shadowballdash", false);
    }
}