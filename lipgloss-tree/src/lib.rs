//! # lipgloss-tree
//!
//! A Rust library for rendering styled tree structures in terminal applications.
//! This crate is part of the lipgloss-rs ecosystem, providing a 1:1 Rust port
//! of the Go [lipgloss/tree] library from [Charm].
//!
//! [lipgloss/tree]: https://github.com/charmbracelet/lipgloss/tree/main/tree
//! [Charm]: https://charm.sh
//!
//! ## Features
//!
//! - **Rich tree rendering** with customizable branch characters (├──, └──, etc.)
//! - **Custom enumerators** supporting Roman numerals, bullet points, or any custom format
//! - **Advanced styling** with colors, padding, borders, and text formatting
//! - **Multi-line content** support with proper indentation
//! - **Style inheritance** from parent to child nodes
//! - **Alignment control** for mixed-width enumerators
//!
//! ## Quick Start
//!
//! ```rust
//! use lipgloss_tree::Tree;
//!
//! let tree = Tree::new()
//!     .root("My Project")
//!     .child(vec![
//!         "src/",
//!         "README.md",
//!         Tree::new()
//!             .root("docs/")
//!             .child(vec!["guide.md", "api.md"]),
//!     ]);
//!
//! println!("{}", tree);
//! ```
//!
//! This produces:
//!
//! ```text
//! My Project
//! ├── src/
//! ├── README.md
//! ├── docs/
//! │   ├── guide.md
//! │   └── api.md
//! ```
//!
//! ## Advanced Usage
//!
//! ### Custom Styling
//!
//! ```rust
//! use lipgloss::{Style, Color};
//! use lipgloss_tree::{Tree, Renderer, TreeStyle};
//!
//! let custom_style = TreeStyle {
//!     enumerator_func: |_, _| Style::new().foreground(Color::from("blue")),
//!     item_func: |_, _| Style::new().foreground(Color::from("green")),
//!     root: Style::new().bold(true).foreground(Color::from("magenta")),
//!     ..TreeStyle::default()
//! };
//!
//! let tree = Tree::new()
//!     .root("Styled Tree")
//!     .child(vec!["Item 1", "Item 2"]);
//!
//! let renderer = Renderer::new().style(custom_style);
//! // Use renderer.render(&tree, true, "") for custom rendering
//! ```
//!
//! ### Custom Enumerators
//!
//! ```rust
//! use lipgloss_tree::Tree;
//!
//! let tree = Tree::new()
//!     .root("Roman List")
//!     .child(vec!["First", "Second", "Third"])
//!     .enumerator(|_, i| match i + 1 {
//!         1 => "I".to_string(),
//!         2 => "II".to_string(),  
//!         3 => "III".to_string(),
//!         n => n.to_string(),
//!     });
//! ```
//!
//! ## Architecture
//!
//! The crate is organized into three main modules:
//!
//! - [`children`] - Node and tree data structures
//! - [`enumerator`] - Functions for generating branch characters and indentation  
//! - [`renderer`] - Core rendering engine with styling support

#![warn(missing_docs)]

/// Node and tree data structures for building hierarchical content.
pub mod children;
/// Functions for generating branch characters and indentation strings.
pub mod enumerator;
/// Core rendering engine with styling and formatting support.
pub mod renderer;

// Re-export the main types and functions
pub use children::{new_string_data, root, Children, Filter, Leaf, Node, NodeChildren, Tree};
pub use enumerator::{
    default_enumerator, default_indenter, rounded_enumerator, Enumerator, Indenter, StyleFunc,
};
pub use renderer::Renderer;

// Go API compatibility aliases
/// Trait alias for `Children` - provides compatibility with Go naming conventions
pub use Children as ChildrenTrait;
/// Type alias for `Leaf` - provides compatibility with Go naming conventions  
pub use Leaf as LeafType;
/// Type alias for `NodeChildren` - provides compatibility with Go naming conventions
pub use NodeChildren as NodeChildrenType;
/// Type alias for `Tree` - provides compatibility with Go naming conventions
pub use Tree as TreeType;

/// Creates a new empty tree.
///
/// This is a convenience function equivalent to `Tree::new()`.
/// The tree starts with no root value and no children.
///
/// # Returns
///
/// A new empty `Tree` instance
///
/// # Examples
///
/// ```rust
/// use lipgloss_tree;
///
/// let tree = lipgloss_tree::new();
/// assert!(tree.to_string().is_empty());
/// ```
pub fn new() -> Tree {
    Tree::new()
}

/// Creates a new tree with a root value.
///
/// This is a convenience function that creates a new tree and immediately
/// sets its root value, equivalent to `Tree::new().root(root)`.
///
/// # Arguments
///
/// * `root` - The root value for the tree (anything that can be converted to a String)
///
/// # Returns
///
/// A new `Tree` instance with the specified root value
///
/// # Examples
///
/// ```rust
/// use lipgloss_tree;
///
/// let tree = lipgloss_tree::new_with_root("My Root");
/// println!("{}", tree); // Outputs: "My Root"
/// ```
pub fn new_with_root(root: impl Into<String>) -> Tree {
    Tree::new().root(root)
}
