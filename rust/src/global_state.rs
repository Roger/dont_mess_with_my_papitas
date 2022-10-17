use gdnative::api::*;
use gdnative::prelude::*;

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
}

#[methods]
impl GlobalState {
    fn register_signals(builder: &ClassBuilder<Self>) {
        builder.signal("state_changed").done();
    }

    fn new(_base: &Node) -> Self {
        GlobalState {
            player_life: 100.0,
            coins: 0,
            seeds: 1,
            score: 0,
            collectable_buff: None,
        }
    }

    #[method]
    fn _ready(&mut self, #[base] base: &Node) {
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

    // #[method]
    pub fn get_buff(&mut self) -> Option<Buff> {
        None
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
