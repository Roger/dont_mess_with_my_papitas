use gdnative::api::*;
use gdnative::prelude::*;

use crate::global_state::GlobalState;
use crate::node_ext::NodeExt;

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

        let state = unsafe {
            base.get_node_as_instance::<GlobalState>("/root/GlobalState")
                .unwrap()
        };

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

        // let state = base.get_node("/root/GlobalState").unwrap();
        // let state = unsafe { state.assume_safe() };

        // state
        //     .connect(
        //         "state_changed",
        //         unsafe { base.assume_shared() },
        //         "on_state_changed",
        //         VariantArray::new_shared(),
        //         0,
        //     )
        //     .unwrap();
    }

    #[method]
    pub fn on_state_changed(&self, #[base] base: &Node, state: GlobalState) {
        self.update_coins(base, &state);
        self.update_life(base, &state);
        self.update_score(base, &state);
        self.update_seeds(base, &state);
    }

    pub fn update_life(&self, base: &Node, state: &GlobalState) {
        let percent = 48.0 / 100.0 * state.player_life;
        let hearts = base.expect_node::<Sprite, _>("Hearts/Full");
        let rect = Rect2::new(Vector2::new(0.0, 0.0), Vector2::new(percent, 16.0));
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
