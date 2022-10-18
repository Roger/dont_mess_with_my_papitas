use std::f64::consts::FRAC_PI_4;

use gdnative::api::*;
use gdnative::prelude::*;
use instant::Instant;

use rand::prelude::IteratorRandom;
use rand::Rng;
use strum::AsRefStr;
use strum::{EnumIter, IntoEnumIterator};

use crate::node_ext::NodeExt;

#[derive(EnumIter, AsRefStr)]
enum SlimeKind {
    #[strum(serialize = "orange")]
    Orange,
    #[strum(serialize = "green")]
    Green,
    #[strum(serialize = "blue")]
    Blue,
}

#[derive(NativeClass)]
#[inherit(KinematicBody2D)]
// #[register_with(Self::register_builder)]
pub struct Slime {
    dying: Option<Instant>,
    last_flip: Option<Instant>,
    velocity: Vector2,
    target: Option<i64>,

    kind: SlimeKind,
    life: f32,
    strength: f32,
    speed: f32,
}

#[methods]
impl Slime {
    fn new(_base: &KinematicBody2D) -> Self {
        let mut rng = rand::thread_rng();
        let kind = SlimeKind::iter().choose(&mut rng).unwrap();

        let (life, strengh, speed) = match kind {
            SlimeKind::Orange => (10.0, 2.0, 1.0),
            SlimeKind::Green => (20.0, 1.0, 1.0),
            SlimeKind::Blue => (10.0, 1.0, 1.5),
        };

        Slime {
            dying: None,
            velocity: Vector2::ZERO,
            target: None,
            last_flip: None,
            kind,
            life,
            strength: strengh,
            speed,
        }
    }

    #[method]
    fn _ready(&mut self, #[base] base: &KinematicBody2D) {
        let sprite = base.expect_node::<Sprite, _>("Sprite");
        let tex = load::<Texture>(format!(
            "res://assets/enemies/slime-{}.png",
            self.kind.as_ref()
        ))
        .unwrap();
        sprite.set_texture(tex);

        let hitbox = base.expect_node::<Area2D, _>("HitboxEnemy");
        hitbox.set("strength", self.strength);
    }

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

        if let Some(player) = self.try_get_node::<KinematicBody2D, _>(base, "%Player") {
            let player_global_pos = player.global_position();
            if player_global_pos.distance_to(base.global_position()) < 50.0 {
                return player_global_pos;
            }
        }

        // if the player is close or there is no more papitas
        if child_count == 0 {
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
        if self.dying.is_some() || area.name().to_string() != "Hitbox" {
            return;
        }

        let animation_player = base.expect_node::<AnimationPlayer, _>("AnimationPlayer");
        animation_player.set_current_animation("Dying");

        self.dying = Some(Instant::now());
        self.drop_item(base);
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
    fn _physics_process(&mut self, #[base] base: &KinematicBody2D, _delta: f64) {
        if self.is_dead() {
            base.queue_free();
        } else if self.dying.is_none() {
            let target_vector = self.get_target_vector(base);
            let global_pos = base.global_position();

            let animation_player = base.expect_node::<AnimationPlayer, _>("AnimationPlayer");
            let collision = base.expect_node::<CollisionShape2D, _>("HitboxEnemy/Collision");
            if global_pos.distance_to(target_vector) < 10.0 {
                animation_player.set_current_animation("Attack");
                collision.set_disabled(false);
                return;
            } else {
                collision.set_disabled(true);
                animation_player.set_current_animation("Walk");
            }
            let direction = global_pos.direction_to(target_vector);

            let accel = 50.0 * self.speed;

            self.velocity = self.velocity.move_toward(direction * accel, 100.0 as f32);
            self.velocity =
                base.move_and_slide(self.velocity, Vector2::ZERO, false, 4, FRAC_PI_4, true);

            // Only turn vision to target max once every 150 milliseconds
            if let Some(last_flip) = self.last_flip {
                if last_flip.elapsed().as_millis() < 150 {
                    return;
                }
            }
            self.last_flip = Some(Instant::now());
            base.expect_node::<Sprite, _>("Sprite")
                .set_flip_h(self.velocity.x < 0.0);
        }
    }
}
