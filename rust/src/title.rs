// use crate::input_const::*;
use crate::node_ext::NodeExt;
use crate::presistent_state::PersistentState;
use gdnative::api::*;
use gdnative::prelude::*;
// use instant::Instant;

#[derive(NativeClass)]
#[inherit(Control)]
pub struct Title {
    showing_about: bool,
}

#[methods]
impl Title {
    fn new(_base: &Control) -> Self {
        Title {
            showing_about: false,
        }
    }

    #[method]
    fn _ready(&mut self, #[base] base: &Control) {
        let persistence = unsafe {
            base.get_node_as_instance::<PersistentState>("/root/PersistentState")
                .unwrap()
        };
        let mut score = 0;
        persistence.map_mut(|x, _o| {
            score = x.score;
        })
        .ok()
        .unwrap_or_else(|| godot_print!("Unable to get data"));
        let score_label = base.expect_node::<Label, _>("Score");
        score_label.set_text(&format!("Best Score: {score}"));
    }

    #[method]
    fn _on_new_game(&self, #[base] base: &Control) {
        unsafe { base.get_tree().unwrap().assume_safe() }
            .change_scene("res://scenes/Game.tscn")
            .unwrap();
    }

    #[method]
    fn _on_about(&mut self, #[base] base: &Control) {
        let animation_player = base.expect_node::<AnimationPlayer, _>("AnimationPlayer");
        if self.showing_about {
            animation_player.set_current_animation("MoveAboutOut");
            self.showing_about = false;
        } else {
            animation_player.set_current_animation("MoveAboutIn");
            self.showing_about = true;
        }
        animation_player.set_active(true);
        let about_button = base.expect_node::<TextureButton, _>("About/About");
        about_button.release_focus();
    }
}
