use gdnative::api::file::ModeFlags;
use gdnative::api::*;
use gdnative::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(NativeClass, Serialize, Deserialize, Default)]
#[inherit(Node2D)]
pub struct PersistentState {
    pub score: i32,
}

#[methods]
impl PersistentState {
    fn new(_base: &Node2D) -> Self {
        let file = File::new();
        match file.open("user://savefile.json", ModeFlags::READ.into()) {
            Ok(_) => {
                let text = file.get_as_text(true).to_string();
                serde_json::from_str(&text).unwrap_or_else(|err| {
                    godot_error!("{:?}", err);
                    Default::default()
                })
            }
            Err(err) => {
                godot_print!("Loading defaults: {:?}", err);
                Default::default()
            }
        }
    }

    fn save(&self) {
        let file = File::new();
        file.open("user://savefile.json", ModeFlags::WRITE.into())
            .unwrap();
        file.store_string(serde_json::to_string_pretty(self).unwrap());
    }

    #[method]
    pub fn update_score(&mut self, #[base] _base: &Node2D, score: i32) {
        if score > self.score {
            self.score = score;
            self.save();
        }
    }
}
