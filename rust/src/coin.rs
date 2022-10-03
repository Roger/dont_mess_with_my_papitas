use gdnative::api::*;
use gdnative::prelude::*;
use instant::Instant;
use crate::node_ext::NodeExt;
use crate::hud;

#[derive(NativeClass)]
#[inherit(Node2D)]
pub struct Coin {
    started_at: Instant,
}

#[methods]
impl Coin {
    fn new(_base: &Node2D) -> Self {
        Coin {
            started_at: Instant::now(),
        }
    }

    #[method]
    fn _ready(&mut self, #[base] base: &Node2D) {
        let animation_player = base.expect_node::<AnimationPlayer, _>("AnimationPlayer");
        animation_player.set_current_animation("UpDown");
        animation_player.set_active(true);
    }

    #[method]
    fn _on_coin_entered(&mut self, #[base] base: &Node2D, area: Ref<Area2D>) {
        if self.started_at.elapsed().as_millis() < 10 {
            return;
        }
        let area = unsafe { area.assume_safe() };
        if area.name().to_string() == "PlayerHurtbox" {

            let hud = unsafe { base.get_node_as_instance::<hud::Hud>("/root//World/Hud").unwrap() };
            hud.map_mut(|x, o| {
                x.update_coins(&o, 1);
            })
            .ok()
            .unwrap_or_else(|| godot_print!("Unable to get hud"));

            base.queue_free();
        }
    }
}
