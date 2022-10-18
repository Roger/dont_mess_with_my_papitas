use gdnative::api::*;
use gdnative::prelude::*;

use crate::node_ext::NodeExt;

#[derive(NativeClass)]
#[inherit(Node2D)]
pub struct Sign;

#[methods]
impl Sign {
    fn new(_base: &Node2D) -> Self {
        Sign
    }

    #[method]
    fn _on_sign_entered(&mut self, #[base] base: &Node2D, area: Ref<Area2D>) {
        let area = unsafe { area.assume_safe() };
        if area.name().to_string() == "PlayerHurtbox" {
            let sprite = base.expect_node::<Sprite, _>("Sprite");
            sprite.set_modulate(Color::from_rgba(1.0, 1.0, 1.0, 0.5));
        }
    }

    #[method]
    fn _on_sign_exited(&mut self, #[base] base: &Node2D, area: Ref<Area2D>) {
        let area = unsafe { area.assume_safe() };
        if area.name().to_string() == "PlayerHurtbox" {
            let sprite = base.expect_node::<Sprite, _>("Sprite");
            sprite.set_modulate(Color::from_rgba(1.0, 1.0, 1.0, 1.0));
        }
    }
}
