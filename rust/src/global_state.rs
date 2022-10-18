use rand::{thread_rng, Rng};

use gdnative::api::*;
use gdnative::prelude::*;
use instant::Instant;

use crate::presistent_state::PersistentState;

#[derive(ToVariant, FromVariant, Debug, Clone)]
pub enum Buff {
    Heart,
}

#[derive(NativeClass, ToVariant, FromVariant, Debug, Clone)]
#[inherit(Node)]
#[register_with(Self::register_signals)]
pub struct GlobalState {
    pub player_life: f32,
    pub coins: i32,
    pub seeds: i32,
    pub score: i32,
    pub collectable_buff: Option<Buff>,
    #[variant(skip)]
    last_collectable: Option<Instant>,
}

impl Default for GlobalState {
    fn default() -> Self {
        GlobalState {
            player_life: 3.0,
            coins: 0,
            seeds: 1,
            score: 0,
            collectable_buff: None,
            last_collectable: Some(Instant::now()),
        }
    }
}

#[methods]
impl GlobalState {
    fn register_signals(builder: &ClassBuilder<Self>) {
        builder.signal("state_changed").done();
    }

    fn new(_base: &Node) -> Self {
        Self::default()
    }

    #[method]
    pub fn reset(&mut self, #[base] base: &Node) {
        *self = Self::default();
        self.state_changed(base);
    }

    fn state_changed(&self, base: &Node) {
        base.emit_signal("state_changed", &[Variant::new(self)]);
    }

    // #[method]
    // pub fn update_score(&mut self, #[base] _base: &Node2D, score: i32) {
    //     if score > self.score {
    //         self.score = score;
    //     }
    // }

    pub fn get_buff(&mut self) -> Option<Buff> {
        // Unwrap is ok, only when converted to variant is set to None
        if self.collectable_buff.is_some()
            || self.last_collectable.unwrap().elapsed().as_secs() < 10
        {
            return None;
        }

        let mut rng = thread_rng();
        if rng.gen_bool(1.0 / 1000.0) {
            self.collectable_buff = Some(Buff::Heart);
            self.collectable_buff.clone()
        } else {
            None
        }
    }

    #[method]
    pub fn collect_buff(&mut self, #[base] base: &Node) {
        // Unwrap is ok, only when converted to variant is set to None
        if let Some(Buff::Heart) = self.collectable_buff {
            self.last_collectable = Some(Instant::now());
            self.collectable_buff = None;
            // Half heart
            self.player_life = 3.0_f32.min(self.player_life + 0.5);
            self.state_changed(base);
        } else {
            godot_error!("No active buff");
        }
    }

    #[method]
    pub fn update_life(&mut self, #[base] base: &Node, life: f32) -> f32 {
        self.player_life += life;
        self.state_changed(base);
        self.player_life
    }

    #[method]
    pub fn update_coins(&mut self, #[base] base: &Node, coins: i32) -> i32 {
        self.coins += coins;
        self.state_changed(base);
        self.coins
    }

    #[method]
    pub fn update_seeds(&mut self, #[base] base: &Node, seeds: i32) -> i32 {
        self.seeds += seeds;
        self.state_changed(base);
        self.seeds
    }

    #[method]
    pub fn update_score(&mut self, #[base] base: &Node, score: i32) -> i32 {
        self.score += score;
        self.state_changed(base);

        let persistence = unsafe {
            base.get_node_as_instance::<PersistentState>("/root/PersistentState")
                .unwrap()
        };
        persistence
            .map_mut(|x, o| {
                x.update_score(&o, self.score);
            })
            .unwrap();

        self.score
    }
}
