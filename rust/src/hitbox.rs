use gdnative::api::*;
use gdnative::prelude::*;

#[derive(NativeClass)]
#[inherit(Area2D)]
// #[register_with(Self::register)]
pub struct Hitbox {
    #[property]
    strength: f32,
}

#[methods]
impl Hitbox {
    fn new(_base: &Area2D) -> Self {
        Hitbox {
            strength: 1.0,
        }
    }
}
