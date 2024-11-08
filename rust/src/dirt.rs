use crate::global_state;
use crate::input_const::*;
use crate::joystick::Joytype;
use crate::node_ext::NodeExt;
use gdnative::api::*;
use gdnative::prelude::*;
use instant::Instant;

#[derive(NativeClass)]
#[inherit(Node2D)]
pub struct Dirt {
    planted: bool,
    created_at: Instant,
    player_inside: bool,
}

#[methods]
impl Dirt {
    fn new(_base: &Node2D) -> Self {
        Dirt {
            planted: false,
            created_at: Instant::now(),
            player_inside: false,
        }
    }

    fn consume_seed(&self, base: &Node2D) -> bool {
        let state = unsafe {
            base.get_node_as_instance::<global_state::GlobalState>("/root/GlobalState")
                .unwrap()
        };
        let mut removed_seed = false;
        state
            .map_mut(|x, o| {
                if x.seeds > 0 {
                    x.update_seeds(&o, -1);
                    removed_seed = true;
                }
            })
            .unwrap();
        removed_seed
    }

    #[method]
    fn _process(&mut self, #[base] base: &Node2D, _delta: f64) {
        if self.created_at.elapsed().as_millis() < 500 {
            return;
        }

        if Joytype::get().is_keyboard() {
            self.handle_keyboard(base);
        } else {
            self.handle_joystick(base);
        }
    }

    fn handle_keyboard(&mut self, base: &Node2D) {
        let reference_rect = base.expect_node::<ReferenceRect, _>("ReferenceRect");
        let timer = base.expect_node::<Timer, _>("Timer");
        let player = base.expect_node::<Node2D, _>("/root/World/Playground/Player");
        let player_pos = player.global_position();
        let dirt_pos = base.global_position();

        if player_pos.distance_to(dirt_pos) > 25.0 || self.planted {
            reference_rect.set_editor_only(true);
            return;
        }

        let input = Input::godot_singleton();
        let seed = base.expect_node::<Sprite, _>("Seed");

        let viewport = base.get_viewport().unwrap();
        let viewport = unsafe { viewport.assume_safe() };
        let mouse_pos = viewport.get_mouse_position();

        let global_rect = reference_rect.get_global_rect();
        if global_rect.contains_point(mouse_pos) {
            reference_rect.set_editor_only(false);
            if input.is_action_pressed(INPUT_SECOND_ACTION, false) {
                if self.consume_seed(base) {
                    seed.set_visible(true);
                    timer.start(10.0);
                    self.planted = true;
                }
            }
        } else {
            reference_rect.set_editor_only(true);
        }
    }

    fn handle_joystick(&mut self, base: &Node2D) {
        let reference_rect = base.expect_node::<ReferenceRect, _>("ReferenceRect");
        let timer = base.expect_node::<Timer, _>("Timer");

        if !self.player_inside || self.planted {
            reference_rect.set_editor_only(true);
            return;
        }

        let input = Input::godot_singleton();
        let seed = base.expect_node::<Sprite, _>("Seed");

        reference_rect.set_editor_only(false);
        if input.is_action_pressed(INPUT_SECOND_ACTION, false) {
            if self.consume_seed(base) {
                seed.set_visible(true);
                timer.start(10.0);
                self.planted = true;
            }
        }
    }

    fn grow_potato(&self, base: &Node2D) {
        let papitas = base.expect_node::<Node, _>("/root/World/Papitas");
        let rc = ResourceLoader::godot_singleton()
            .load("res://scenes/Papita.tscn", "PackedScene", false)
            .unwrap();
        let scene = unsafe { rc.assume_safe().cast::<PackedScene>() }.unwrap();
        let instance = scene
            .instance(PackedScene::GEN_EDIT_STATE_DISABLED)
            .unwrap();
        let instance = unsafe { instance.assume_safe().cast::<Node2D>().unwrap() };
        instance.set_global_position(base.global_position());
        papitas.add_child(instance, false);
    }

    #[method]
    fn _on_timeout(&self, #[base] base: &Node2D) {
        self.grow_potato(base);
        base.queue_free();
    }

    #[method]
    fn _on_area_entered(&mut self, #[base] _base: &Node2D, area: Ref<Area2D>) {
        let area = unsafe { area.assume_safe() };
        if area.name().to_string() == "PlayerPointer" {
            self.player_inside = true;
        }
    }

    #[method]
    fn _on_area_exited(&mut self, #[base] _base: &Node2D, area: Ref<Area2D>) {
        let area = unsafe { area.assume_safe() };
        if area.name().to_string() == "PlayerPointer" {
            self.player_inside = false;
        }
    }
}
