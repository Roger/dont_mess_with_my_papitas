mod coin;
mod dirt;
mod game;
mod global_state;
mod globalscope;
mod hitbox;
mod hud;
mod input_const;
mod joystick;
mod merchant;
mod node_ext;
mod papita;
mod paused;
mod player;
mod presistent_state;
mod sign;
mod slime;
mod title;
mod utils;

use gdnative::prelude::{godot_init, InitHandle, Variant};

// Function that registers all exposed classes to Godot
fn init(handle: InitHandle) {
    handle.add_class::<presistent_state::PersistentState>();
    handle.add_class::<global_state::GlobalState>();
    handle.add_class::<title::Title>();
    handle.add_class::<sign::Sign>();
    handle.add_class::<hitbox::Hitbox>();
    handle.add_class::<paused::Paused>();
    handle.add_class::<game::Game>();
    handle.add_class::<hud::Hud>();
    handle.add_class::<player::Player>();
    handle.add_class::<slime::Slime>();
    handle.add_class::<papita::Papita>();
    handle.add_class::<coin::Coin>();
    handle.add_class::<dirt::Dirt>();
    handle.add_class::<merchant::Merchant>();
    handle.add_class::<merchant::MerchantEye>();
}

// macros that create the entry-points of the dynamic library.
godot_init!(init);
