use gdnative::api::*;
use gdnative::prelude::*;
use instant::Instant;

use rand::prelude::IteratorRandom;
use strum::IntoEnumIterator;

use crate::global_state::Power;
use crate::input_const::*;
use crate::joystick::Joytype;
use crate::node_ext::NodeExt;
use crate::utils::get_global_state_instance;

#[derive(NativeClass)]
#[inherit(StaticBody2D)]
pub struct Merchant {
    player_close: bool,
    power: Power,
    sold: bool,
    last_power: Instant,
}

#[methods]
impl Merchant {
    fn new(_base: &StaticBody2D) -> Self {
        let mut rng = rand::thread_rng();
        let power = Power::iter().choose(&mut rng).unwrap();
        Merchant {
            player_close: false,
            last_power: Instant::now(),
            sold: false,
            power,
        }
    }

    #[method]
    fn _ready(&self, #[base] base: &StaticBody2D) {
        let player = base.expect_node::<AnimationPlayer, _>("AnimationPlayer");
        player.set_current_animation("ItemsFloating");
        let power_sprite = base.expect_node::<Sprite, _>("Inventory2/Sprite");
        power_sprite.set_texture(self.power.to_texture());
    }

    #[method]
    fn _on_sell_area_entered(&mut self, area: Ref<Area2D>) {
        let area = unsafe { area.assume_safe() };
        if area.name().to_string() == "PlayerHurtbox" {
            self.player_close = true;
        }
    }

    #[method]
    fn _on_sell_area_exited(&mut self, area: Ref<Area2D>) {
        let area = unsafe { area.assume_safe() };
        if area.name().to_string() == "PlayerHurtbox" {
            self.player_close = false;
        }
    }

    #[method]
    fn _process(&mut self, #[base] base: &StaticBody2D, _delta: f64) {
        if self.last_power.elapsed().as_secs() >= 10 {
            self.last_power = Instant::now();
            self.sold = false;

            let mut rng = rand::thread_rng();
            let mut power = Power::iter().choose(&mut rng).unwrap();
            while power == self.power {
                power = Power::iter().choose(&mut rng).unwrap();
            }
            self.power = power;

            let power_sprite = base.expect_node::<Sprite, _>("Inventory2/Sprite");
            power_sprite.set_texture(self.power.to_texture());
            power_sprite.set_visible(true);
        }

        if !self.player_close {
            return;
        }

        if Joytype::get().is_keyboard() {
            self.handle_keyboard(base);
        } else {
            self.handle_joystick(base);
        }

    }

    fn handle_keyboard(&mut self, base: &StaticBody2D) {
        let seeds_sprite = base.expect_node::<Sprite, _>("Inventory1/Sprite");
        let power_sprite = base.expect_node::<Sprite, _>("Inventory2/Sprite");

        let input = Input::godot_singleton();

        let viewport = base.get_viewport().unwrap();
        let viewport = unsafe { viewport.assume_safe() };
        let mouse_pos = viewport.get_mouse_position();

        seeds_sprite.set_scale(Vector2::new(0.6, 0.6));
        power_sprite.set_scale(Vector2::new(0.6, 0.6));

        let seeds_rect = base.expect_node::<ReferenceRect, _>("Inventory1/ReferenceRect");
        let power_rect = base.expect_node::<ReferenceRect, _>("Inventory2/ReferenceRect");

        let global_seeds_rect = seeds_rect.get_global_rect();
        let global_power_rect = power_rect.get_global_rect();

        if global_seeds_rect.contains_point(mouse_pos) {
            seeds_sprite.set_scale(Vector2::new(0.8, 0.8));
            if input.is_action_just_released(INPUT_SECOND_ACTION, false) {
                self.buy_seed(base);
            }
        } else if global_power_rect.contains_point(mouse_pos) {
            power_sprite.set_scale(Vector2::new(0.8, 0.8));
            if input.is_action_just_released(INPUT_SECOND_ACTION, false) {
                self.buy_power(base);
            }
        }
    }

    fn handle_joystick(&mut self, base: &StaticBody2D) {
        let input = Input::godot_singleton();

        if input.is_action_just_released(INPUT_THIRD_ACTION, false) {
            self.buy_seed(base);
        } else if input.is_action_just_released(INPUT_FORTH_ACTION, false) {
            self.buy_power(base);
        }
    }

    fn buy_seed(&self, base: &StaticBody2D) {
        let state = get_global_state_instance(base);
        state
            .map_mut(|x, o| {
                if x.coins >= 5 {
                    x.update_seeds(&o, 1);
                    x.update_coins(&o, -5);
                    base.expect_node::<AudioStreamPlayer, _>("Sound")
                        .play(0.0);
                }
            })
            .unwrap();
    }

    fn buy_power(&mut self, base: &StaticBody2D) {
        let state = get_global_state_instance(base);
        state
            .map_mut(|x, o| {
                if x.coins >= 5 {
                    self.sold = true;
                    x.set_power(&o, self.power.clone());
                    x.update_coins(&o, -5);
                    base.expect_node::<AudioStreamPlayer, _>("Sound")
                        .play(0.0);
                    base.expect_node::<Sprite, _>("Inventory2/Sprite")
                        .set_visible(false);
                }
            })
            .unwrap();
    }
}
