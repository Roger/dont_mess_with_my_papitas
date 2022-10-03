use gdnative::api::*;
use gdnative::prelude::*;
// use crate::node_ext::NodeExt;
// use crate::hud;

#[derive(NativeClass)]
#[inherit(Node2D)]
pub struct Data {
    pub score: i32,
}

#[methods]
impl Data {
    fn new(_base: &Node2D) -> Self {
        Data {
            score: 0,
        }
    }

    #[method]
    pub fn update_score(&mut self, #[base] _base: &Node2D, score: i32) {
        if score > self.score {
            dbg!(self.score);
            self.score = score;
        }
    }
}
