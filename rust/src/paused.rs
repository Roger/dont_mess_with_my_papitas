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
        let input = Input::godot_singleton();
        if input.is_action_just_pressed("escape", false) {
            let tree = unsafe { base.get_tree().unwrap().assume_safe() };
            let is_paused = tree.is_paused();
            tree.set_pause(!is_paused);
            base.set_visible(!is_paused);
        }
    }
}
