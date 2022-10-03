use crate::hud;
use crate::input_const::*;
use crate::node_ext::NodeExt;
use gdnative::api::*;
use gdnative::prelude::*;
use instant::Instant;

#[derive(NativeClass)]
#[inherit(Node2D)]
pub struct Dirt {
    planted: bool,
    created_at: Instant,
}

#[methods]
impl Dirt {
    fn new(_base: &Node2D) -> Self {
        Dirt {
            planted: false,
            created_at: Instant::now(),
        }
    }

    // #[method]
    // fn _ready(&mut self, #[base] _base: &Node2D) {}

    fn consume_seed(&self, base: &Node2D) -> bool {
        let hud = unsafe {
            base.get_node_as_instance::<hud::Hud>("/root//World/Hud")
                .unwrap()
        };
        let mut removed_seed = false;
        hud.map_mut(|x, o| {
            if x.seeds > 0 {
                x.update_seeds(&o, -1);
                removed_seed = true;
            }
        })
        .ok()
        .unwrap_or_else(|| godot_print!("Unable to get hud"));
        removed_seed
    }

    #[method]
    fn _process(&mut self, #[base] base: &Node2D, _delta: f64) {
        if self.created_at.elapsed().as_millis() < 500 {
            return;
        }

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
}
