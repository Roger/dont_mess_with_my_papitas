use crate::globalscope::lerp;
use crate::node_ext::NodeExt;
use crate::utils::get_global_state_instance;
use crate::utils::reparent_to_hud;
use gdnative::api::*;
use gdnative::prelude::*;
use instant::Instant;

#[derive(Debug)]
enum State {
    FLOOR,
    COLLECTED,
    FLYING,
}

impl State {
    fn on_floor(&self) -> bool {
        match self {
            State::FLOOR => true,
            _ => false,
        }
    }
}

#[derive(NativeClass)]
#[inherit(Node2D)]
pub struct Coin {
    started_at: Instant,
    state: State,
}

#[methods]
impl Coin {
    fn new(_base: &Node2D) -> Self {
        Coin {
            started_at: Instant::now(),
            state: State::FLOOR,
        }
    }

    #[method]
    fn _ready(&mut self, #[base] base: &Node2D) {
        let animation_player = base.expect_node::<AnimationPlayer, _>("AnimationPlayer");
        animation_player.set_current_animation("UpDown");
        animation_player.set_active(true);
    }

    #[method]
    fn _process(&mut self, #[base] base: &Node2D, delta: f64) {
        let sprite = base.expect_node::<Sprite, _>("Coin");
        match self.state {
            State::FLOOR => {
                match self.started_at.elapsed().as_millis() {
                    // On 6 seconds remove the coin
                    6000.. => base.queue_free(),
                    // Oscillate transparency in the last 2 seconds
                    millis @ 3000..=6999 => {
                        let alpha = (millis as f32 % 150.0) / 250.0 + 0.3;
                        sprite.set_modulate(Color::from_rgba(1.0, 1.0, 1.0, alpha));
                    }
                    _ => (),
                }
            }
            // Reparent to hud on collect, to make the coin
            // on top of everything while flying
            State::COLLECTED => {
                sprite.set_modulate(Color::from_rgba(1.0, 1.0, 1.0, 1.0));
                reparent_to_hud(base);
                self.state = State::FLYING;
            }
            State::FLYING => {
                let target_pos = base
                    .expect_node::<Node2D, _>("/root/World/Hud/Coins")
                    .global_position();
                let mut global_pos = base.global_position();
                if global_pos.distance_to(target_pos) < 3.0 {
                    let state = get_global_state_instance(base);
                    state
                        .map_mut(|x, o| {
                            x.update_coins(&o, 1);
                        })
                        .unwrap();
                    base.queue_free();
                }

                let w = 13.0 * delta as f32;
                global_pos.x = lerp(global_pos.x..=target_pos.x, w);
                global_pos.y = lerp(global_pos.y..=target_pos.y, w);
                base.set_global_position(global_pos);
            }
        }
    }

    #[method]
    fn _on_coin_entered(&mut self, #[base] base: &Node2D, area: Ref<Area2D>) {
        if self.started_at.elapsed().as_millis() < 10 || !self.state.on_floor() {
            return;
        }
        let area = unsafe { area.assume_safe() };
        if area.name().to_string() == "PlayerHurtbox" {
            self.state = State::COLLECTED;
            base.expect_node::<AudioStreamPlayer2D, _>("Sound").play(0.0);
        }
    }
}
