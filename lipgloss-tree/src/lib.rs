pub mod children;
pub mod enumerator;
pub mod renderer;

// Re-export the main types and functions
pub use children::{new_string_data, root, Children, Filter, Leaf, Node, NodeChildren, Tree};
pub use enumerator::{
    default_enumerator, default_indenter, rounded_enumerator, Enumerator, Indenter, StyleFunc,
};
pub use renderer::Renderer;

// Go API compatibility aliases
pub use Children as ChildrenTrait;
pub use Leaf as LeafType;
pub use NodeChildren as NodeChildrenType;
pub use Tree as TreeType;

/// Creates a new tree.
pub fn new() -> Tree {
    Tree::new()
}

/// Creates a new tree with a root value.
pub fn new_with_root(root: impl Into<String>) -> Tree {
    Tree::new().root(root)
}
