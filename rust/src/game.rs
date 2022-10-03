use crate::node_ext::NodeExt;
use gdnative::api::*;
use gdnative::prelude::*;
use rand::prelude::IteratorRandom;

#[derive(NativeClass)]
#[inherit(Node2D)]
pub struct Game {
    spawn_points: Vec<Vector2>,
}

#[methods]
impl Game {
    fn new(_base: &Node2D) -> Self {
        Game {
            spawn_points: Vec::default(),
        }
    }

    #[method]
    fn _ready(&mut self, #[base] base: &Node2D) {
        let spawn_points = base.expect_node::<Node, _>("%SpawnPoints").get_children();
        let spawn_points: Vec<_> = spawn_points
            .iter()
            .map(|v| unsafe {
                v.to_object::<Position2D>()
                    .unwrap()
                    .assume_safe()
                    .global_position()
            })
            .collect();
        self.spawn_points = spawn_points;
        let timer = base.expect_node::<Timer, _>("%Timer");
        timer.start(10.0);
    }

    #[method]
    fn _on_timeout(&self, #[base] base: &Node2D) {
        let spawn_point = self
            .spawn_points
            .iter()
            .choose(&mut rand::thread_rng())
            .unwrap();

        let slimes = base.expect_node::<Node, _>("%Slimes");
        let rc = ResourceLoader::godot_singleton()
            .load("res://scenes/Slime.tscn", "PackedScene", false)
            .unwrap();
        let scene = unsafe { rc.assume_safe().cast::<PackedScene>() }.unwrap();
        let instance = scene
            .instance(PackedScene::GEN_EDIT_STATE_DISABLED)
            .unwrap();
        let instance = unsafe { instance.assume_safe().cast::<KinematicBody2D>().unwrap() };
        instance.set_global_position(spawn_point.clone());
        slimes.add_child(instance, false);
    }

    // This function will be called in every frame
    // #[method]
    // fn _process(&mut self, #[base] _base: &Node2D, delta: f64) {
    // }
}
