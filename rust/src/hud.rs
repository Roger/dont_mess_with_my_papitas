use gdnative::api::*;
use gdnative::prelude::*;

use crate::data::Data;
use crate::node_ext::NodeExt;

#[derive(NativeClass)]
#[inherit(Node)]
pub struct Hud {
    pub player_life: f32,
    pub coins: i32,
    pub seeds: i32,
    pub score: i32,
}

#[methods]
impl Hud {
    fn new(_base: &Node) -> Self {
        Hud {
            player_life: 100.0,
            coins: 0,
            seeds: 1,
            score: 0,
        }
    }

    #[method]
    fn _ready(&mut self, #[base] base: &Node) {
        let hearts = base.expect_node::<Sprite, _>("Hearts/Full");
        let rect = Rect2::new(Vector2::new(0.0, 0.0), Vector2::new(48.0, 16.0));
        hearts.set_region_rect(rect);
    }

    #[method]
    pub fn update_life(&mut self, #[base] base: &Node, life: f32) {
        self.player_life += life;
        let percent = 48.0 / 100.0 * self.player_life;
        let hearts = base.expect_node::<Sprite, _>("Hearts/Full");
        let rect = Rect2::new(Vector2::new(0.0, 0.0), Vector2::new(percent, 16.0));
        hearts.set_region_rect(rect);
    }

    #[method]
    pub fn update_coins(&mut self, #[base] base: &Node, coins: i32) {
        self.coins += coins;
        let coins = base.expect_node::<Label, _>("Coins/Amount");
        coins.set_text(self.coins.to_string());
    }

    #[method]
    pub fn update_seeds(&mut self, #[base] base: &Node, seeds: i32) -> i32 {
        self.seeds += seeds;
        let seeds = base.expect_node::<Label, _>("Inventory1/SeedsCount");
        seeds.set_text(self.seeds.to_string());
        self.seeds
    }

    #[method]
    pub fn update_score(&mut self, #[base] base: &Node, score: i32) {
        self.score += score;
        let score = base.expect_node::<Label, _>("Score/Amount");
        score.set_text(self.score.to_string());

        let data = unsafe {
            base.get_node_as_instance::<Data>("/root/Data")
                .unwrap()
        };
        data.map_mut(|x, o| {
            x.update_score(&o, self.score);
        })
        .ok()
        .unwrap_or_else(|| godot_print!("Unable to get data"));

    }

}
