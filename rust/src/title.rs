use crate::joystick::Joytype;
use crate::node_ext::NodeExt;
use crate::presistent_state::PersistentState;
use gdnative::api::*;
use gdnative::prelude::*;

enum Showing {
    Main,
    Howto,
    About,
}

impl Showing {
    fn is_main(&self) -> bool {
        match self {
            Showing::Main => true,
            _ => false,
        }
    }
}

#[derive(NativeClass)]
#[inherit(Control)]
pub struct Title {
    joytype: Joytype,
    showing: Showing,
    cerdos: usize,
    almare: usize,
    issy: usize,
}

#[methods]
impl Title {
    fn new(_base: &Control) -> Self {
        Title {
            showing: Showing::Main,
            joytype: Joytype::get(),
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
        persistence
            .map_mut(|x, _o| {
                score = x.score;
            })
            .unwrap();
        let score_label = base.expect_node::<Label, _>("Score");
        score_label.set_text(&format!("Best Score: {score}"));

        base.expect_node::<Control, _>("NewGame").grab_focus();
        self.handle_show_papita(base);

        let input = Input::godot_singleton();
        input
            .connect(
                "joy_connection_changed",
                unsafe { base.assume_shared() },
                "_on_joy_connection_changed",
                VariantArray::new_shared(),
                object::ConnectFlags::DEFERRED.into(),
            )
            .unwrap();
    }

    #[method]
    fn _on_joy_connection_changed(
        &mut self,
        #[base] base: &Control,
        _device_id: i64,
        _connected: bool,
    ) {
        self.joytype = Joytype::get();
        self.handle_show_papita(base);
    }

    fn handle_show_papita(&self, base: &Control) {
        let new_btn = base.expect_node::<Button, _>("NewGame");
        let howto_btn = base.expect_node::<Button, _>("HowToPlay/HowToPlay");

        let new_papita = base.expect_node::<Sprite, _>("NewGame/Papita");
        let howto_papita = base.expect_node::<Sprite, _>("HowToPlay/HowToPlay/Papita");
        let about_papita = base.expect_node::<Sprite, _>("About/About/Papita");

        new_papita.set_visible(false);
        howto_papita.set_visible(false);
        about_papita.set_visible(false);

        if self.joytype.is_keyboard() {
            return;
        }

        if new_btn.has_focus() {
            new_papita.set_visible(true);
        } else if howto_btn.has_focus() {
            howto_papita.set_visible(true);
        } else {
            about_papita.set_visible(true);
        }
    }

    fn handle_hover(&self, base: &Control, button_name: &str) {
        let button = base.expect_node::<Button, _>(button_name);
        let outline = if button.is_hovered() { 1 } else { 0 };

        let font = button
            .get("custom_fonts/font")
            .to_object::<DynamicFont>()
            .unwrap();
        unsafe { font.assume_safe() }.set_outline_size(outline);
    }

    #[method]
    fn _process(&mut self, #[base] base: &Control, _delta: f64) {
        self.handle_hover(base, "NewGame");
        self.handle_hover(base, "HowToPlay/HowToPlay");
        self.handle_hover(base, "About/About");
        self.handle_show_papita(base);

        let nintendo = base.expect_node::<Sprite, _>("HowToPlay/NintendoControl");
        let play = base.expect_node::<Sprite, _>("HowToPlay/PlayControl");
        let xbox = base.expect_node::<Sprite, _>("HowToPlay/XBoxControl");
        nintendo.set_visible(false);
        play.set_visible(false);
        xbox.set_visible(false);

        match self.joytype {
            Joytype::Nintendo => nintendo.set_visible(true),
            Joytype::Psx => play.set_visible(true),
            _ => xbox.set_visible(true),
        }
    }

    #[method]
    fn _on_new_game(&self, #[base] base: &Control) {
        if !self.showing.is_main() {
            return;
        }
        unsafe { base.get_tree().unwrap().assume_safe() }
            .change_scene("res://scenes/Game.tscn")
            .unwrap();
    }

    #[method]
    fn _on_about(&mut self, #[base] base: &Control) {
        let animation_player = base.expect_node::<AnimationPlayer, _>("AnimationPlayer");
        if animation_player.is_playing() {
            return;
        }
        match self.showing {
            Showing::About => {
                animation_player.set_current_animation("MoveAboutOut");
                self.showing = Showing::Main;
            }
            Showing::Howto => {
                animation_player.set_current_animation("MoveAboutInFromHowto");
                self.showing = Showing::About;
            }
            Showing::Main => {
                animation_player.set_current_animation("MoveAboutIn");
                self.showing = Showing::About;
            }
        }

        animation_player.set_active(true);
    }

    #[method]
    fn _on_howto(&mut self, #[base] base: &Control) {
        let animation_player = base.expect_node::<AnimationPlayer, _>("AnimationPlayer");
        if animation_player.is_playing() {
            return;
        }
        match self.showing {
            Showing::About => {
                animation_player.set_current_animation("MoveHowtoInFromAbout");
                self.showing = Showing::Howto;
            }
            Showing::Howto => {
                animation_player.set_current_animation("MoveHowtoOut");
                self.showing = Showing::Main;
            }
            Showing::Main => {
                animation_player.set_current_animation("MoveHowtoIn");
                self.showing = Showing::Howto;
            }
        }

        animation_player.set_active(true);
    }

    #[method]
    fn _on_cerdos_button_up(&mut self, #[base] base: &Control) {
        self.cerdos += 1;
        let sprite = base.expect_node::<Sprite, _>("About/LosCerdosSonLaOndaLoca");

        if !sprite.is_visible() {
            base.expect_node::<AudioStreamPlayer, _>("EEA").play(0.0);
        }
        sprite.set_visible(self.cerdos >= 10);
    }

    #[method]
    fn _on_almare_button_up(&mut self, #[base] base: &Control) {
        self.almare += 1;
        let sprite = base.expect_node::<Sprite, _>("About/Almare96");

        if !sprite.is_visible() {
            base.expect_node::<AudioStreamPlayer, _>("EEA").play(0.0);
        }
        sprite.set_visible(self.almare >= 96);
    }

    #[method]
    fn _on_issy_button_up(&mut self, #[base] base: &Control) {
        self.issy += 1;
        let sprite = base.expect_node::<Sprite, _>("About/IssyOfMars");

        if !sprite.is_visible() {
            base.expect_node::<AudioStreamPlayer, _>("EEA").play(0.0);
        }
        sprite.set_visible(self.issy >= 10);

    }
}
