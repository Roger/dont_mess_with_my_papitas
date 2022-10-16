use gdnative::api::*;
use gdnative::prelude::*;

#[derive(NativeClass)]
#[inherit(Node2D)]
pub struct PersistentState {
    pub score: i32,
}

#[methods]
impl PersistentState {
    fn new(_base: &Node2D) -> Self {
        PersistentState {
            score: 0,
        }
    }

    #[method]
    pub fn update_score(&mut self, #[base] _base: &Node2D, score: i32) {
        if score > self.score {
            self.score = score;
        }
    }
}
