/// took from: https://github.com/wyattjsmith1/gdrust/blob/807003fd67f2eb6bdfd961ea717f75d310d8266d/gdrust/src/unsafe_functions/node_ext.rs
/// and addapted to godot-rust 0.10
use gdnative::api::SceneTree;
use gdnative::prelude::{NewRef, Node, NodePath, Shared, SubClass, TRef};

pub trait NodeExt {
    /// Gets a typed node from a node path. This has an explicit `unsafe` block, and can panic. The
    /// unsafe code is calling `assume_safe` on the node at `path`.
    /// # Panics
    /// - If no node is found at the path.
    /// - If a node is found at the path, but is not the correct type.
    ///
    /// # GdScript equivalent
    /// ```gdscript
    /// get_node(path)
    /// ```
    fn expect_node<T: SubClass<Node>, P: Into<NodePath>>(&self, path: P) -> TRef<T>;

    fn try_get_cast_node<T: SubClass<Node>, P: Into<NodePath>>(&self, path: P) -> Option<TRef<T>>;

    /// Gets the parent node with a type. This has an explicit `unsafe` block, and can panic. The
    /// unsafe code is calling `assume_safe` on the parent node.
    /// # Panics
    /// - If no parent is found (root node).
    /// - If a node is found at the path, but is not the correct type.
    ///
    /// # GdScript equivalent
    /// ```gdscript
    /// get_parent()
    /// ```
    fn expect_parent<T: SubClass<Node>>(&self) -> TRef<T>;

    /// Gets the scene tree. This has an explicit `unsafe` block, and can panic. The unsafe code is
    /// calling `assume_safe` on the scene tree.
    /// # Panics
    /// - If the scene tree is not found.
    ///
    /// # GdScript Equivalent
    /// ```gdscript
    /// get_tree()
    /// ```
    fn expect_tree(&self) -> TRef<SceneTree>;
}

impl<'a, T: SubClass<Node>> NodeExt for &'a T {
    fn try_get_cast_node<Child: SubClass<Node>, P: Into<NodePath>>(
        &self,
        path: P,
    ) -> Option<TRef<'a, Child, Shared>> {
        let path = path.into();
        let node = unsafe {
            self.upcast()
                .get_node(path.new_ref())?
                .assume_safe()
                .cast::<Child>()
                .expect("Could not cast")
        };
        Some(node)
    }

    fn expect_node<Child: SubClass<Node>, P: Into<NodePath>>(
        &self,
        path: P,
    ) -> TRef<'a, Child, Shared> {
        let path = path.into();
        unsafe {
            self.upcast()
                .get_node(path.new_ref())
                .expect(format!("Could not find a node at {}", path.new_ref().to_string()).as_str())
                .assume_safe()
                .cast::<Child>()
                .expect("Could not cast")
        }
    }

    fn expect_parent<Child: SubClass<Node>>(&self) -> TRef<'a, Child, Shared> {
        unsafe {
            self.upcast()
                .get_parent()
                .expect("Could not get a parent node")
                .assume_safe()
                .cast::<Child>()
                .expect("Could not cast")
        }
    }

    fn expect_tree(&self) -> TRef<'a, SceneTree, Shared> {
        unsafe {
            self.upcast()
                .get_tree()
                .expect("Expected scene tree, but couldn't find it")
                .assume_safe()
        }
    }
}
