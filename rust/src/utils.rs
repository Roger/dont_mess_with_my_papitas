use gdnative::prelude::*;

use crate::hud::Hud;
use crate::node_ext::NodeExt;

pub fn reparent_to_hud<'a, Base: SubClass<Node2D>>(base: &Base) {
    let base = base.upcast::<Node2D>();
    let hud = base.expect_node::<Node, _>("/root/World/Hud");

    let global_pos = base.global_position();
    let parent = base.expect_parent::<Node>();
    let base_ref = unsafe { base.assume_shared() };
    parent.remove_child(base_ref);
    let base_ref = unsafe { base.assume_shared() };
    hud.add_child(base_ref, false);
    base.set_global_position(global_pos);
}

pub fn get_hud_instance<Base: SubClass<Node2D>>(base: &Base) -> TInstance<Hud> {
    let base = base.upcast();
    unsafe { base.get_node_as_instance::<Hud>("/root/World/Hud") }.unwrap()
}
