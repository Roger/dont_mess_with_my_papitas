mod game;
mod hud;
mod player;
mod merchant;
mod papita;
mod slime;
mod coin;
mod dirt;
mod globalscope;
mod node_ext;
mod input_const;

use gdnative::prelude::{godot_init, InitHandle, Variant};

// Function that registers all exposed classes to Godot
fn init(handle: InitHandle) {
    handle.add_class::<game::Game>();
    handle.add_class::<hud::Hud>();
    handle.add_class::<player::Player>();
    handle.add_class::<slime::Slime>();
    handle.add_class::<papita::Papita>();
    handle.add_class::<coin::Coin>();
    handle.add_class::<dirt::Dirt>();
    handle.add_class::<merchant::Merchant>();
}

// macros that create the entry-points of the dynamic library.
godot_init!(init);
