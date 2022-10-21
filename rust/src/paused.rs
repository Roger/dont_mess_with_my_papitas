use crate::node_ext::NodeExt;
use gdnative::api::*;
use gdnative::prelude::*;

#[derive(NativeClass)]
#[inherit(Node2D)]
pub struct Paused;

#[methods]
impl Paused {
    fn new(_base: &Node2D) -> Self {
        Paused
    }

    #[method]
    fn _process(&mut self, #[base] base: &Node2D, _delta: f64) {
        let tree = unsafe { base.get_tree().unwrap().assume_safe() };
        let is_paused = tree.is_paused();

        let music = base.expect_node::<Control, _>("Sound/Music");
        let sfx = base.expect_node::<Control, _>("Sound/SFX");

        if is_paused && !music.has_focus() && !sfx.has_focus() {
            music.grab_focus();
        }

        // base.expect_node::<Control, _>("Sound/Music").grab_focus();
        let input = Input::godot_singleton();
        if input.is_action_just_pressed("pause", false) {
            tree.set_pause(!is_paused);
            base.set_visible(!is_paused);
        }
    }

    #[method]
    fn _on_volume_changed(&self, #[base] base: &Node2D, value: f64, name: String) {
        let audio_server = AudioServer::godot_singleton();
        audio_server.set_bus_volume_db(audio_server.get_bus_index(&name), value);
        let player = base.expect_node::<AudioStreamPlayer, _>(format!("Sound/Player{}", name));
        if !player.is_playing() {
            player.play(0.0);
        }
    }
}
