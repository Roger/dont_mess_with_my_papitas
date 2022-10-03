use crate::node_ext::NodeExt;
use gdnative::api::*;
use gdnative::prelude::*;
use instant::Instant;
use crate::input_const::*;

#[derive(NativeClass)]
#[inherit(Node2D)]
pub struct Papita {
    life: isize,
    last_hit: Option<Instant>,
}

#[methods]
impl Papita {
    fn new(_base: &Node2D) -> Self {
        Papita {
            life: 3,
            last_hit: None
        }
    }

    #[method]
    fn _ready(&mut self, #[base] base: &Node2D) {
        let animation_player = base.expect_node::<AnimationPlayer, _>("AnimationPlayer");
        animation_player.set_current_animation("PapitaUpDown");
        animation_player.set_active(true);
    }

    #[method]
    fn _on_hurtbox_entered(&mut self, #[base] base: &Node2D, area: Ref<Area2D>) {
        let area = unsafe { area.assume_safe() };
        if area.name().to_string() != "HitboxEnemy" {
            return;
        }
        if let Some(last_hit) = self.last_hit {
            if last_hit.elapsed().as_secs() >= 1 {
                self.last_hit = None;
            }
            return;
        }

        self.life -= 1;
        if self.life <= 0 {
            base.queue_free();
        }
        self.last_hit = Some(Instant::now());
    }

    fn unplant_potato(&self, base: &Node2D) {
        let dirts = base.expect_node::<Node, _>("/root/World/Dirts");
        let rc = ResourceLoader::godot_singleton()
            .load("res://scenes/Dirt.tscn", "PackedScene", false)
            .unwrap();
        let scene = unsafe { rc.assume_safe().cast::<PackedScene>() }.unwrap();
        let instance = scene
            .instance(PackedScene::GEN_EDIT_STATE_DISABLED)
            .unwrap();
        let instance = unsafe { instance.assume_safe().cast::<Node2D>().unwrap() };
        instance.set_global_position(base.global_position());
        dirts.add_child(instance, false);
    }

    #[method]
    fn _process(&mut self, #[base] base: &Node2D, _delta: f64) {
        let reference_rect = base.expect_node::<ReferenceRect, _>("ReferenceRect");
        let player = base.expect_node::<Node2D, _>("/root/World/Playground/Player");
        let player_pos = player.global_position();
        let potato_pos = base.global_position();
        if player_pos.distance_to(potato_pos) > 25.0 {
            reference_rect.set_editor_only(true);
            return;
        }

        let input = Input::godot_singleton();

        let viewport = base.get_viewport().unwrap();
        let viewport = unsafe { viewport.assume_safe() };
        let mouse_pos = viewport.get_mouse_position();

        let global_rect = reference_rect.get_global_rect();
        if global_rect.contains_point(mouse_pos) {
            reference_rect.set_editor_only(false);
            if input.is_action_pressed(INPUT_SECOND_ACTION, false) {
                self.unplant_potato(base);
                base.queue_free();
            }
        } else {
            reference_rect.set_editor_only(true);
        }
    }
}
