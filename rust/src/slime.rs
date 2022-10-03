use std::f64::consts::FRAC_PI_4;

use gdnative::api::*;
use gdnative::prelude::*;
use instant::Instant;

use rand::Rng;

use crate::node_ext::NodeExt;

#[derive(NativeClass)]
#[inherit(KinematicBody2D)]
// #[register_with(Self::register_builder)]
pub struct Slime {
    dying: Option<Instant>,
    velocity: Vector2,
    target: Option<i64>,
    follow_player: bool,
    life: usize,
}

#[methods]
impl Slime {
    fn new(_base: &KinematicBody2D) -> Self {
        Slime {
            dying: None,
            velocity: Vector2::ZERO,
            target: None,
            follow_player: false,
            life: 10,
        }
    }

    // #[method]
    // fn _ready(&mut self, #[base] _base: &KinematicBody2D) {
    // }

    fn get_random_target_id(&self, base: &KinematicBody2D) -> Option<i64> {
        let papitas = self.expect_node::<Node2D, _>(base, "%Papitas");
        let child_count = papitas.get_child_count();
        if child_count == 0 {
            None
        } else {
            Some(rand::thread_rng().gen_range(0..child_count))
        }
    }

    /// Try Get node from the parent
    fn try_get_node<T: SubClass<Node>, P: Into<NodePath>>(
        &self,
        base: &KinematicBody2D,
        path: P,
    ) -> Option<TRef<T>> {
        unsafe {
            let parent = base.get_parent().unwrap().assume_safe();
            let node = parent.get_node(path)?.assume_safe().cast::<T>().unwrap();
            Some(node)
        }
    }

    // Assume that node exists
    fn expect_node<T: SubClass<Node>, P: Into<NodePath>>(
        &self,
        base: &KinematicBody2D,
        path: P,
    ) -> TRef<T> {
        self.try_get_node::<T, _>(base, path).unwrap()
    }

    fn get_target_vector(&mut self, base: &KinematicBody2D) -> Vector2 {
        let papitas = self.expect_node::<Node2D, _>(base, "%Papitas");
        let child_count = papitas.get_child_count();

        // if the player is close or there is no more papitas
        if self.follow_player || child_count == 0 {
            if let Some(player) = self.try_get_node::<KinematicBody2D, _>(base, "%Player") {
                return player.global_position();
            }
        }

        if let Some(target) = self.target {
            if let Some(target_obj) = papitas.get_child(target) {
                let target_obj = unsafe { target_obj.assume_safe() };
                return target_obj.cast::<Node2D>().unwrap().global_position();
            }
        }
        if let Some(random_target) = self.get_random_target_id(base) {
            if let Some(target_obj) = papitas.get_child(random_target) {
                let target_obj = unsafe { target_obj.assume_safe() };
                self.target = Some(random_target);
                return target_obj.cast::<Node2D>().unwrap().global_position();
            }
        }

        // if there is not target, don't move
        base.global_position()
    }

    #[method]
    fn _on_hurtbox_area_entered(&mut self, #[base] base: &KinematicBody2D, area: Ref<Area2D>) {
        let area = unsafe { area.assume_safe() };
        // when the slime enters the game area, reenable the collision
        if area.name().to_string() == "GameArea" {
            // let collision = base.expect_node::<CollisionShape2D, _>("CollisionShape2D");
            // collision.set_disabled(false);
            return;
        } else if area.name().to_string() != "Hitbox" {
            return;
        }
        let animated_sprite = base.expect_node::<AnimatedSprite, _>("AnimatedSprite");
        animated_sprite.set_animation("dying");
        let collision = base.expect_node::<CollisionShape2D, _>("CollisionShape2D");
        collision.set_disabled(true);

        let collision_hitbox = base.expect_node::<CollisionShape2D, _>("HitboxEnemy/Collision");
        collision_hitbox.set_disabled(true);
        self.dying = Some(Instant::now());
        self.drop_item(base);
    }

    #[method]
    fn _on_vision_entered(&mut self, area: Ref<Area2D>) {
        let area = unsafe { area.assume_safe() };
        if area.name().to_string() == "PlayerHurtbox" {
            self.follow_player = true;
        }
    }

    #[method]
    fn _on_vision_exited(&mut self, area: Ref<Area2D>) {
        let area = unsafe { area.assume_safe() };
        if area.name().to_string() == "PlayerHurtbox" {
            self.follow_player = false;
        }
    }

    fn is_dead(&self) -> bool {
        if let Some(timer) = self.dying {
            return timer.elapsed().as_secs_f32() > 2.0;
        }
        false
    }

    fn drop_item(&self, base: &KinematicBody2D) {
        let playground = self.expect_node::<YSort, _>(base, "/root/World/Playground");
        let rc = ResourceLoader::godot_singleton()
            .load("res://scenes/Coin.tscn", "PackedScene", false)
            .unwrap();
        let scene = unsafe { rc.assume_safe().cast::<PackedScene>() }.unwrap();
        let instance = scene
            .instance(PackedScene::GEN_EDIT_STATE_DISABLED)
            .unwrap();
        let instance = unsafe { instance.assume_safe().cast::<Node2D>().unwrap() };
        instance.set_global_position(base.global_position());
        playground.add_child(instance, false);
    }

    #[method]
    fn _physics_process(&mut self, #[base] base: &KinematicBody2D, delta: f64) {
        if self.is_dead() {
            base.queue_free();
        } else if self.dying.is_none() {
            let target_vector = self.get_target_vector(base);
            let direction = base.global_position().direction_to(target_vector);
            self.velocity = self
                .velocity
                .move_toward(direction * 50.0, 100.0 * delta as f32);
            self.velocity =
                base.move_and_slide(self.velocity, Vector2::ZERO, false, 4, FRAC_PI_4, true);
            base.expect_node::<AnimatedSprite, _>("AnimatedSprite")
                .set_flip_h(self.velocity.x < 0.0);
        }
    }
}
