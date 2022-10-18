use gdnative::api::*;
use gdnative::prelude::*;

use crate::global_state::GlobalState;
use crate::node_ext::NodeExt;
use crate::utils::get_global_state_instance;

#[derive(NativeClass)]
#[inherit(Node)]
pub struct Hud;

#[methods]
impl Hud {
    fn new(_base: &Node) -> Self {
        Hud
    }

    #[method]
    fn _ready(&mut self, #[base] base: &Node) {
        let hearts = base.expect_node::<Sprite, _>("Hearts/Full");
        let rect = Rect2::new(Vector2::new(0.0, 0.0), Vector2::new(48.0, 16.0));
        hearts.set_region_rect(rect);
        let state = get_global_state_instance(base);

        state
            .map_mut(|state, owner| {
                owner
                    .connect(
                        "state_changed",
                        unsafe { base.assume_shared() },
                        "on_state_changed",
                        VariantArray::new_shared(),
                        0,
                    )
                    .unwrap();
                self.on_state_changed(base, state.clone());
            })
            .unwrap();
    }

    #[method]
    pub fn on_state_changed(&self, #[base] base: &Node, state: GlobalState) {
        self.update_coins(base, &state);
        self.update_life(base, &state);
        self.update_score(base, &state);
        self.update_seeds(base, &state);
    }

    pub fn update_life(&self, base: &Node, state: &GlobalState) {
        let number_hearts = state.player_life.trunc();
        let heart_width = 12.0;
        let margin = 4.0;
        let width = state.player_life * heart_width + number_hearts * margin + margin / 2.0;

        let hearts = base.expect_node::<Sprite, _>("Hearts/Full");
        let rect = Rect2::new(Vector2::new(0.0, 0.0), Vector2::new(width, 16.0));
        hearts.set_region_rect(rect);
    }

    pub fn update_coins(&self, base: &Node, state: &GlobalState) {
        base.expect_node::<Label, _>("Coins/Amount")
            .set_text(state.coins.to_string());
    }

    pub fn update_seeds(&self, base: &Node, state: &GlobalState) {
        base.expect_node::<Label, _>("Inventory1/SeedsCount")
            .set_text(state.seeds.to_string());
    }

    pub fn update_score(&self, base: &Node, state: &GlobalState) {
        let score = base.expect_node::<Label, _>("Score/Amount");
        score.set_text(state.score.to_string());
    }
}
