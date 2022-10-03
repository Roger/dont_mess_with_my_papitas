use crate::node_ext::NodeExt;
use gdnative::api::*;
use gdnative::prelude::*;
use rand::prelude::IteratorRandom;

#[derive(NativeClass)]
#[inherit(Node2D)]
pub struct Game {
    spawn_points: Vec<Vector2>,
    wave: usize,
}

#[methods]
impl Game {
    fn new(_base: &Node2D) -> Self {
        Game {
            spawn_points: Vec::default(),
            wave: 1,
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
    fn _on_timeout(&mut self, #[base] base: &Node2D) {
        self.wave += 1;

        let mut rng = rand::thread_rng();
        let mut spawn_points = self.spawn_points.clone();

        // add 1 slime every 3 waves
        for _ in 0..(self.wave / 3) + 1 {
            let amount = spawn_points.len();
            if amount == 0 {
                return;
            }
            let selected = (0..amount).choose(&mut rng).unwrap();
            let spawn_point = spawn_points.remove(selected);
            self.spawn_slime(base, spawn_point);
        }
    }

    fn spawn_slime(&mut self, base: &Node2D, spawn_point: Vector2) {
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
