use std::f64::consts::FRAC_PI_4;

use crate::input_const::*;
use crate::node_ext::NodeExt;
use crate::utils::get_global_state_instance;
use gdnative::api::*;
use gdnative::prelude::*;
use instant::Instant;

const ACCELERATION: f32 = 800.0;
const MAX_VELOCITY: f32 = 120.0;

pub enum PlayerState {
    Move,
    Attack,
}

#[derive(NativeClass)]
#[inherit(KinematicBody2D)]
pub struct Player {
    state: PlayerState,
    velocity: Vector2,
    last_hit: Option<Instant>,
}

#[methods]
impl Player {
    /// The "construcaor" of the class.
    fn new(_base: &KinematicBody2D) -> Self {
        Player {
            state: PlayerState::Move,
            velocity: Vector2::ZERO,
            last_hit: None,
        }
    }

    #[method]
    fn _ready(&mut self, #[base] base: &KinematicBody2D) {
        // base.set_physics_process(true);
        // let anim = "Idle";
        // let anim_player =
        //     unsafe { base.get_node_as::<AnimationPlayer>("AnimationPlayer") }.unwrap();
        // let animation = anim_player.get_animation(anim).unwrap();
        // unsafe { animation.assume_safe() }.set_loop(true);
        // anim_player.play(anim, -1.0, 1.0, false);

        // let animation_tree = unsafe { animation_tree.assume_safe() };
        let animation_tree = unsafe { base.get_node_as::<AnimationTree>("AnimationTree").unwrap() };
        animation_tree.set_active(true);
    }

    #[method]
    fn _physics_process(&mut self, #[base] base: &KinematicBody2D, delta: f64) {
        match self.state {
            PlayerState::Move => self.move_state(base),
            PlayerState::Attack => self.attack_state(base),
        }
    }

    #[method]
    fn on_attack_finish(&mut self) {
        self.state = PlayerState::Move;
    }

    fn attack_state(&mut self, base: &KinematicBody2D) {
        self.velocity = Vector2::ZERO;

        let animation_tree = unsafe { base.get_node_as::<AnimationTree>("AnimationTree").unwrap() };
        let playback = animation_tree.get("parameters/playback");
        let playback = playback
            .try_to_object::<AnimationNodeStateMachinePlayback>()
            .unwrap();
        let playback = unsafe { playback.assume_safe() };

        // let input = Input::godot_singleton();
        // let mut input_vector = Vector2::ZERO;
        // input_vector.x = (input.get_action_strength(INPUT_RIGHT, false)
        //     - input.get_action_strength(INPUT_LEFT, false)) as f32;
        // input_vector.y = (input.get_action_strength(INPUT_DOWN, false)
        //     - input.get_action_strength(INPUT_UP, false)) as f32;
        // input_vector = input_vector.normalized();
        // if input_vector != Vector2::ZERO {
        //     animation_tree.set("parameters/Idle/blend_position", input_vector);
        //     animation_tree.set("parameters/Walk/blend_position", input_vector);
        //     animation_tree.set("parameters/Attack/blend_position", input_vector);
        //     // if !input.is_action_pressed(INPUT_ATTACK, false) {
        //     //     playback.travel("Idle");
        //         // self.state = PlayerState::Move;
        //         // return;
        //     // }
        // }

        playback.travel("Attack");
    }

    fn move_state(&mut self, base: &KinematicBody2D) {
        let input = Input::godot_singleton();
        let mut input_vector = Vector2::ZERO;
        input_vector.x = (input.get_action_strength(INPUT_RIGHT, false)
            - input.get_action_strength(INPUT_LEFT, false)) as f32;
        input_vector.y = (input.get_action_strength(INPUT_DOWN, false)
            - input.get_action_strength(INPUT_UP, false)) as f32;

        let animation_tree = unsafe { base.get_node_as::<AnimationTree>("AnimationTree").unwrap() };
        let playback = animation_tree.get("parameters/playback");
        let playback = playback
            .try_to_object::<AnimationNodeStateMachinePlayback>()
            .unwrap();
        let playback = unsafe { playback.assume_safe() };

        if input_vector != Vector2::ZERO {
            input_vector = input_vector.normalized();
            self.velocity = self
                .velocity
                .move_toward(input_vector * MAX_VELOCITY, ACCELERATION as f32);

            animation_tree.set("parameters/Idle/blend_position", input_vector);
            animation_tree.set("parameters/Walk/blend_position", input_vector);
            animation_tree.set("parameters/Attack/blend_position", input_vector);
            playback.travel("Walk");
        } else {
            self.velocity = Vector2::ZERO;
            playback.travel("Idle");
        }

        if input.is_action_pressed(INPUT_ATTACK, false) {
            self.state = PlayerState::Attack;
        }

        self.velocity =
            base.move_and_slide(self.velocity, Vector2::ZERO, false, 4, FRAC_PI_4, true);
    }

    #[method]
    fn _process(&mut self, #[base] base: &KinematicBody2D, _delay: f64) {
        self.handle_hit_cooldown(base);
    }

    fn handle_hit_cooldown(&mut self, base: &KinematicBody2D) {
        let sprite = base.expect_node::<Sprite, _>("Sprite");
        match self.last_hit {
            // if last hit expired
            Some(last_hit) if last_hit.elapsed().as_secs() >= 1 => {
                self.last_hit = None;
            }
            _ => (),
        };
        if self.last_hit.is_none() {
            sprite.set_modulate(Color::from_rgba(1.0, 1.0, 1.0, 1.0));
        } else {
            sprite.set_modulate(Color::from_rgba(1.0, 0.7, 0.7, 1.0));
        }
    }

    #[method]
    fn _on_hurtbox_entered(&mut self, #[base] base: &KinematicBody2D, area: Ref<Area2D>) {
        let area = unsafe { area.assume_safe() };
        if area.name().to_string() != "HitboxEnemy" {
            return;
        }

        if self.last_hit.is_some() {
            return;
        }
        self.last_hit = Some(Instant::now());

        let state = get_global_state_instance(base);
        let life = state.map_mut(|x, o| {
            x.update_life(&o, -10.0)
        })
        .unwrap();

        if life <= 0.0 {
            unsafe { base.get_tree().unwrap().assume_safe() }
                .change_scene("res://scenes/Title.tscn")
                .unwrap();
            base.queue_free();
        }
    }
}
