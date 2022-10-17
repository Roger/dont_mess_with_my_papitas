use gdnative::api::*;
use gdnative::prelude::*;

use crate::input_const::*;
use crate::node_ext::NodeExt;
use crate::utils::get_global_state_instance;

#[derive(NativeClass)]
#[inherit(StaticBody2D)]
pub struct Merchant;

#[methods]
impl Merchant {
    fn new(_base: &StaticBody2D) -> Self {
        Merchant
    }

    #[method]
    fn _on_sell_area_entered(&mut self, #[base] base: &StaticBody2D, area: Ref<Area2D>) {
        let area = unsafe { area.assume_safe() };
        let inventory = base.expect_node::<Node2D, _>("Inventory");
        if area.name().to_string() == "PlayerHurtbox" {
            inventory.set_visible(true);
        }
    }

    #[method]
    fn _on_sell_area_exited(&mut self, #[base] base: &StaticBody2D, area: Ref<Area2D>) {
        let area = unsafe { area.assume_safe() };
        let inventory = base.expect_node::<Node2D, _>("Inventory");
        if area.name().to_string() == "PlayerHurtbox" {
            inventory.set_visible(false);
        }
    }

    #[method]
    fn _process(&mut self, #[base] base: &StaticBody2D, _delta: f64) {
        let reference_rect = base.expect_node::<ReferenceRect, _>("Inventory/ReferenceRect");
        // let timer = base.expect_node::<Timer, _>("Timer");
        // let player = base.expect_node::<Node2D, _>("/root/World/Playground/Player");
        // let player_pos = player.global_position();
        // let dirt_pos = base.global_position();
        // if player_pos.distance_to(dirt_pos) > 25.0 || self.planted {
        //     reference_rect.set_editor_only(true);
        //     return;
        // }

        let input = Input::godot_singleton();

        let viewport = base.get_viewport().unwrap();
        let viewport = unsafe { viewport.assume_safe() };
        let mouse_pos = viewport.get_mouse_position();

        let global_rect = reference_rect.get_global_rect();
        if global_rect.contains_point(mouse_pos) {
            reference_rect.set_editor_only(false);
            if input.is_action_just_released(INPUT_SECOND_ACTION, false) {
                self.buy(base);
            }
        } else {
            reference_rect.set_editor_only(true);
        }
    }

    fn buy(&self, base: &StaticBody2D) {
        let state = get_global_state_instance(base);
        // let mut removed_seed = false;
        state
            .map_mut(|x, o| {
                if x.coins >= 5 {
                    x.update_seeds(&o, 1);
                    x.update_coins(&o, -5);
                    // removed_seed = true;
                    base.expect_node::<AudioStreamPlayer2D, _>("Sound")
                        .play(0.0);
                }
            })
            .unwrap();
        // removed_seed
    }
}
