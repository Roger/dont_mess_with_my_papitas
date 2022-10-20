use crate::node_ext::NodeExt;
use crate::presistent_state::PersistentState;
use gdnative::api::*;
use gdnative::prelude::*;

#[derive(NativeClass)]
#[inherit(Control)]
pub struct Title {
    showing_about: bool,
    cerdos: usize,
    almare: usize,
    issy: usize,
}

#[methods]
impl Title {
    fn new(_base: &Control) -> Self {
        Title {
            showing_about: false,
            cerdos: 0,
            almare: 86,
            issy: 0,
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
        }).unwrap();
        let score_label = base.expect_node::<Label, _>("Score");
        score_label.set_text(&format!("Best Score: {score}"));
    }

    #[method]
    fn _on_new_game(&self, #[base] base: &Control) {
        if self.showing_about {
            return;
        }
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

    #[method]
    fn _on_cerdos_button_up(&mut self, #[base] base: &Control) {
        self.cerdos += 1;
        if self.cerdos >= 10 {
            base.expect_node::<Sprite, _>("About/LosCerdosSonLaOndaLoca").set_visible(true);
        }
    }

    #[method]
    fn _on_almare_button_up(&mut self, #[base] base: &Control) {
        self.almare += 1;
        if self.almare >= 96 {
            base.expect_node::<Sprite, _>("About/Almare96").set_visible(true);
        }
    }

    #[method]
    fn _on_issy_button_up(&mut self, #[base] base: &Control) {
        self.issy += 1;
        if self.issy >= 10 {
            base.expect_node::<Sprite, _>("About/IssyOfMars").set_visible(true);
        }
    }

}
