//! Node and tree data structures for building hierarchical content.
//!
//! This module provides the core data structures for representing tree nodes
//! and collections of children. It supports styling, filtering, and complex
//! tree operations with Go lipgloss compatibility.

use lipgloss::Style;
use std::fmt;

/// Helper trait to enable cloning boxed trait objects for Node.
///
/// This trait provides the mechanism for cloning `Box<dyn Node>` objects,
/// which is necessary for tree operations that need to duplicate node structures.
/// It's automatically implemented for any type that implements `Node + Clone`.
///
/// # Implementation Note
///
/// This trait is automatically implemented via a blanket implementation,
/// so you typically don't need to implement it manually.
pub trait CloneNode {
    /// Creates a cloned copy of this node as a boxed trait object.
    ///
    /// # Returns
    ///
    /// A new `Box<dyn Node>` containing a clone of this node.
    fn clone_node(&self) -> Box<dyn Node>;
}

impl<T> CloneNode for T
where
    T: 'static + Node + Clone,
{
    fn clone_node(&self) -> Box<dyn Node> {
        Box::new(self.clone())
    }
}

/// Trait defining a node in a tree structure.
///
/// `Node` represents an individual element in a tree that can have a value,
/// children, visibility state, and various styling options. It supports both
/// simple leaf nodes and complex tree nodes with nested children.
///
/// # Core Functionality
///
/// - **Value**: Each node has a string value that represents its content
/// - **Children**: Nodes can have child nodes forming a hierarchical structure
/// - **Visibility**: Nodes can be hidden from rendering
/// - **Styling**: Nodes support custom enumerators, indenters, and styling functions
///
/// # Examples
///
/// ```rust
/// use lipgloss_tree::{Tree, Leaf, Node};
///
/// // Create a simple leaf node
/// let leaf = Leaf::new("Hello", false);
/// assert_eq!(leaf.value(), "Hello");
/// assert!(!leaf.hidden());
///
/// // Create a tree node with children
/// let tree = Tree::new()
///     .root("Parent")
///     .child(vec!["Child 1".into(), "Child 2".into()]);
/// assert_eq!(tree.value(), "Parent");
/// assert_eq!(tree.children().length(), 2);
/// ```
pub trait Node: fmt::Display + CloneNode {
    /// Returns the string value of this node.
    ///
    /// The value is the textual content that will be displayed when
    /// the tree is rendered.
    ///
    /// # Returns
    ///
    /// The node's value as a `String`
    fn value(&self) -> String;

    /// Returns the children of this node.
    ///
    /// Children are returned as a boxed trait object to allow for
    /// different implementations of the `Children` trait.
    ///
    /// # Returns
    ///
    /// A `Box<dyn Children>` containing this node's children
    fn children(&self) -> Box<dyn Children>;

    /// Returns whether this node is hidden from rendering.
    ///
    /// Hidden nodes are not displayed in the tree output and are
    /// typically filtered out during rendering.
    ///
    /// # Returns
    ///
    /// `true` if the node is hidden, `false` otherwise
    fn hidden(&self) -> bool;

    /// Sets the visibility state of this node.
    ///
    /// # Arguments
    ///
    /// * `hidden` - `true` to hide the node, `false` to show it
    fn set_hidden(&mut self, hidden: bool);

    /// Updates the value of this node.
    ///
    /// # Arguments
    ///
    /// * `value` - The new string value for this node
    fn set_value(&mut self, value: String);

    // Optional per-node renderer overrides. Default to None.
    
    /// Returns the custom enumerator function for this node, if any.
    ///
    /// The enumerator function generates branch characters (like ├──, └──)
    /// for this specific node, overriding the default renderer behavior.
    ///
    /// # Returns
    ///
    /// An optional reference to the node's custom enumerator function
    fn get_enumerator(&self) -> Option<&crate::Enumerator> {
        None
    }

    /// Returns the custom indenter function for this node, if any.
    ///
    /// The indenter function generates indentation strings for child content
    /// under this node, overriding the default renderer behavior.
    ///
    /// # Returns
    ///
    /// An optional reference to the node's custom indenter function
    fn get_indenter(&self) -> Option<&crate::Indenter> {
        None
    }

    /// Returns the base item style for this node, if any.
    ///
    /// The base item style is applied to the node's content before any
    /// style functions are applied.
    ///
    /// # Returns
    ///
    /// An optional reference to the node's base item style
    fn get_item_style(&self) -> Option<&Style> {
        None
    }

    /// Returns the base enumerator style for this node, if any.
    ///
    /// The base enumerator style is applied to the node's branch characters
    /// before any style functions are applied.
    ///
    /// # Returns
    ///
    /// An optional reference to the node's base enumerator style
    fn get_enumerator_style(&self) -> Option<&Style> {
        None
    }

    /// Returns the item style function for this node, if any.
    ///
    /// The item style function provides dynamic styling based on the node's
    /// position and context within its parent's children.
    ///
    /// # Returns
    ///
    /// An optional reference to the node's item style function
    fn get_item_style_func(&self) -> Option<&crate::StyleFunc> {
        None
    }

    /// Returns the enumerator style function for this node, if any.
    ///
    /// The enumerator style function provides dynamic styling for branch
    /// characters based on the node's position and context.
    ///
    /// # Returns
    ///
    /// An optional reference to the node's enumerator style function
    fn get_enumerator_style_func(&self) -> Option<&crate::StyleFunc> {
        None
    }
}

impl Clone for NodeChildren {
    fn clone(&self) -> Self {
        let mut cloned = NodeChildren::new();
        for i in 0..self.length() {
            if let Some(n) = self.at(i) {
                cloned.append(n.clone_node());
            }
        }
        cloned
    }
}

/// A filtered view of children that applies start/end bounds and optional hidden filtering.
///
/// `Slice` provides a window into a larger children collection, allowing you to
/// view only a subset of children within specified bounds. It can also optionally
/// skip hidden nodes during iteration.
///
/// # Examples
///
/// ```rust,ignore
/// // Internal usage only - not part of public API
/// let slice = Slice::new(children, 1, 5, true); // Skip hidden nodes
/// ```
#[allow(dead_code)]
struct Slice {
    /// The underlying children collection
    data: Box<dyn Children>,
    /// Starting index (inclusive)
    start: usize,
    /// Ending index (exclusive)
    end: usize,
    /// Whether to skip hidden nodes
    skip_hidden: bool,
}

#[allow(dead_code)]
impl Slice {
    /// Creates a new slice view of the given children collection.
    ///
    /// # Arguments
    ///
    /// * `data` - The underlying children collection to slice
    /// * `start` - Starting index (inclusive)
    /// * `end` - Ending index (exclusive)
    /// * `skip_hidden` - Whether to filter out hidden nodes
    ///
    /// # Returns
    ///
    /// A new `Slice` that provides a filtered view of the children
    fn new(data: Box<dyn Children>, start: usize, end: usize, skip_hidden: bool) -> Self {
        Self {
            data,
            start,
            end,
            skip_hidden,
        }
    }
}

impl Children for Slice {
    fn at(&self, index: usize) -> Option<&dyn Node> {
        let mut j = 0usize;
        let upper = self.end.min(self.data.length());
        for i in self.start..upper {
            if let Some(node) = self.data.at(i) {
                if self.skip_hidden && node.hidden() {
                    continue;
                }
                if j == index {
                    return Some(node);
                }
                j += 1;
            }
        }
        None
    }

    fn length(&self) -> usize {
        let mut count = 0usize;
        let upper = self.end.min(self.data.length());
        for i in self.start..upper {
            if let Some(node) = self.data.at(i) {
                if self.skip_hidden && node.hidden() {
                    continue;
                }
                count += 1;
            }
        }
        count
    }
}

/// Trait defining a collection of child nodes in a tree structure.
///
/// `Children` provides the interface for accessing and querying collections
/// of tree nodes. Implementations can provide different storage strategies,
/// filtering capabilities, or views of the underlying data.
///
/// # Examples
///
/// ```rust
/// use lipgloss_tree::{NodeChildren, Children, Leaf};
///
/// let mut children = NodeChildren::new();
/// children.append(Box::new(Leaf::new("Item 1", false)));
/// children.append(Box::new(Leaf::new("Item 2", false)));
///
/// assert_eq!(children.length(), 2);
/// assert_eq!(children.at(0).unwrap().value(), "Item 1");
/// ```
pub trait Children {
    /// Returns the node at the given index.
    ///
    /// # Arguments
    ///
    /// * `index` - The zero-based index of the desired node
    ///
    /// # Returns
    ///
    /// An optional reference to the node at the given index,
    /// or `None` if the index is out of bounds
    fn at(&self, index: usize) -> Option<&dyn Node>;

    /// Returns the total number of children in this collection.
    ///
    /// # Returns
    ///
    /// The count of child nodes as a `usize`
    fn length(&self) -> usize;
}

/// A concrete implementation of the `Children` trait using a vector of boxed nodes.
///
/// `NodeChildren` is the primary implementation for storing and managing
/// collections of tree nodes. It provides efficient access, modification,
/// and iteration over child nodes.
///
/// # Examples
///
/// ```rust
/// use lipgloss_tree::{NodeChildren, Leaf};
///
/// let mut children = NodeChildren::new();
/// children.append(Box::new(Leaf::new("First", false)));
/// children.append(Box::new(Leaf::new("Second", false)));
///
/// assert_eq!(children.length(), 2);
/// ```
pub struct NodeChildren {
    /// Vector storing the child nodes as boxed trait objects
    nodes: Vec<Box<dyn Node>>,
}

impl NodeChildren {
    /// Creates a new empty `NodeChildren` collection.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use lipgloss_tree::NodeChildren;
    ///
    /// let children = NodeChildren::new();
    /// assert_eq!(children.length(), 0);
    /// ```
    pub fn new() -> Self {
        Self { nodes: Vec::new() }
    }

    /// Creates a `NodeChildren` collection from a vector of boxed nodes.
    ///
    /// # Arguments
    ///
    /// * `nodes` - A vector of boxed nodes to initialize the collection with
    ///
    /// # Examples
    ///
    /// ```rust
    /// use lipgloss_tree::{NodeChildren, Leaf};
    ///
    /// let nodes = vec![
    ///     Box::new(Leaf::new("A", false)) as Box<dyn lipgloss_tree::Node>,
    ///     Box::new(Leaf::new("B", false)) as Box<dyn lipgloss_tree::Node>,
    /// ];
    /// let children = NodeChildren::from_nodes(nodes);
    /// assert_eq!(children.length(), 2);
    /// ```
    pub fn from_nodes(nodes: Vec<Box<dyn Node>>) -> Self {
        Self { nodes }
    }

    /// Appends a child node to the end of the collection.
    ///
    /// # Arguments
    ///
    /// * `child` - The boxed node to add to the collection
    ///
    /// # Examples
    ///
    /// ```rust
    /// use lipgloss_tree::{NodeChildren, Leaf};
    ///
    /// let mut children = NodeChildren::new();
    /// children.append(Box::new(Leaf::new("New Item", false)));
    /// assert_eq!(children.length(), 1);
    /// ```
    pub fn append(&mut self, child: Box<dyn Node>) {
        self.nodes.push(child);
    }

    /// Removes and returns the child node at the given index.
    ///
    /// # Arguments
    ///
    /// * `index` - The zero-based index of the node to remove
    ///
    /// # Returns
    ///
    /// The removed node if the index was valid, or `None` if out of bounds
    ///
    /// # Examples
    ///
    /// ```rust
    /// use lipgloss_tree::{NodeChildren, Leaf};
    ///
    /// let mut children = NodeChildren::new();
    /// children.append(Box::new(Leaf::new("Remove me", false)));
    /// 
    /// let removed = children.remove(0);
    /// assert!(removed.is_some());
    /// assert_eq!(children.length(), 0);
    /// ```
    pub fn remove(&mut self, index: usize) -> Option<Box<dyn Node>> {
        if index < self.nodes.len() {
            Some(self.nodes.remove(index))
        } else {
            None
        }
    }

    /// Returns a mutable reference to the boxed node at the given index.
    ///
    /// This allows you to modify the node in place or replace it entirely.
    ///
    /// # Arguments
    ///
    /// * `index` - The zero-based index of the desired node
    ///
    /// # Returns
    ///
    /// A mutable reference to the boxed node, or `None` if the index is out of bounds
    ///
    /// # Examples
    ///
    /// ```rust
    /// use lipgloss_tree::{NodeChildren, Leaf};
    ///
    /// let mut children = NodeChildren::new();
    /// children.append(Box::new(Leaf::new("Original", false)));
    ///
    /// if let Some(node) = children.at_mut(0) {
    ///     node.set_value("Modified".to_string());
    /// }
    /// ```
    pub fn at_mut(&mut self, index: usize) -> Option<&mut Box<dyn Node>> {
        self.nodes.get_mut(index)
    }
}

impl Default for NodeChildren {
    /// Creates a default (empty) `NodeChildren` collection.
    ///
    /// Equivalent to `NodeChildren::new()` - creates an empty collection
    /// with no child nodes.
    fn default() -> Self {
        Self::new()
    }
}

impl Children for NodeChildren {
    fn at(&self, index: usize) -> Option<&dyn Node> {
        self.nodes.get(index).map(|node| node.as_ref())
    }

    fn length(&self) -> usize {
        self.nodes.len()
    }
}

/// A terminal node in the tree that contains a value but no children.
///
/// `Leaf` represents the simplest type of tree node - one that holds a string
/// value and a visibility flag, but cannot have child nodes. This is useful
/// for representing individual items in a tree structure.
///
/// # Examples
///
/// ```rust
/// use lipgloss_tree::{Leaf, Node};
///
/// let leaf = Leaf::new("Hello, World!", false);
/// assert_eq!(leaf.value(), "Hello, World!");
/// assert!(!leaf.hidden());
/// assert_eq!(leaf.children().length(), 0);
///
/// println!("{}", leaf); // Outputs: Hello, World!
/// ```
#[derive(Debug, Clone)]
pub struct Leaf {
    /// The textual content of this leaf node
    value: String,
    /// Whether this leaf is hidden from rendering
    hidden: bool,
}

impl Leaf {
    /// Creates a new leaf node with the given value and visibility.
    ///
    /// # Arguments
    ///
    /// * `value` - The content for this leaf (anything convertible to String)
    /// * `hidden` - Whether this leaf should be hidden from rendering
    ///
    /// # Examples
    ///
    /// ```rust
    /// use lipgloss_tree::Leaf;
    ///
    /// let visible_leaf = Leaf::new("Visible", false);
    /// let hidden_leaf = Leaf::new("Hidden", true);
    ///
    /// assert!(!visible_leaf.hidden());
    /// assert!(hidden_leaf.hidden());
    /// ```
    pub fn new(value: impl Into<String>, hidden: bool) -> Self {
        Self {
            value: value.into(),
            hidden,
        }
    }
}

impl Node for Leaf {
    fn value(&self) -> String {
        self.value.clone()
    }

    fn children(&self) -> Box<dyn Children> {
        Box::new(NodeChildren::new())
    }

    fn hidden(&self) -> bool {
        self.hidden
    }

    fn set_hidden(&mut self, hidden: bool) {
        self.hidden = hidden;
    }

    fn set_value(&mut self, value: String) {
        self.value = value;
    }

    fn get_enumerator(&self) -> Option<&crate::Enumerator> {
        None
    }

    fn get_indenter(&self) -> Option<&crate::Indenter> {
        None
    }
}

impl fmt::Display for Leaf {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.value)
    }
}

/// A tree node that can contain both a value and child nodes.
///
/// `Tree` is the main building block for creating hierarchical tree structures.
/// Unlike `Leaf` nodes, trees can have children and support advanced features
/// like custom styling, enumerators, indenters, and offset-based child filtering.
///
/// # Key Features
///
/// - **Hierarchical structure**: Can contain child nodes forming a tree
/// - **Custom styling**: Support for root, item, and enumerator styling
/// - **Custom rendering**: Override enumerators and indenters per-node
/// - **Child filtering**: Use offsets to display only a subset of children
/// - **Builder pattern**: Fluent API for easy tree construction
///
/// # Examples
///
/// ```rust
/// use lipgloss_tree::Tree;
///
/// let tree = Tree::new()
///     .root("My Project")
///     .child(vec![
///         "file1.txt".into(),
///         "file2.txt".into(),
///         Tree::new()
///             .root("subfolder")
///             .child(vec!["nested.txt".into()])
///             .into(),
///     ]);
///
/// println!("{}", tree);
/// ```
///
/// This creates a tree structure like:
/// ```text
/// My Project
/// ├── file1.txt
/// ├── file2.txt
/// ├── subfolder
/// │   └── nested.txt
/// ```
#[derive(Clone)]
pub struct Tree {
    /// The root value of this tree node
    value: String,
    /// Whether this tree node is hidden from rendering
    hidden: bool,
    /// Offset bounds [start, end] for filtering children
    offset: [usize; 2],
    /// Collection of child nodes
    children: NodeChildren,

    // Style and rendering properties
    /// Custom enumerator function for this tree
    enumerator: Option<crate::Enumerator>,
    /// Custom indenter function for this tree
    indenter: Option<crate::Indenter>,
    /// Style applied to the root value
    root_style: Option<Style>,
    /// Base style applied to all child items
    item_style: Option<Style>,
    /// Base style applied to all enumerators
    enumerator_style: Option<Style>,
    /// Dynamic styling function for items
    item_style_func: Option<crate::StyleFunc>,
    /// Dynamic styling function for enumerators
    enumerator_style_func: Option<crate::StyleFunc>,
}

impl Tree {
    /// Creates a new empty tree with default settings.
    ///
    /// The tree starts with no root value, no children, and default
    /// rendering settings.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use lipgloss_tree::Tree;
    ///
    /// let tree = Tree::new();
    /// assert!(tree.value().is_empty());
    /// assert_eq!(tree.children().length(), 0);
    /// ```
    pub fn new() -> Self {
        Self {
            value: String::new(),
            hidden: false,
            offset: [0, 0],
            children: NodeChildren::new(),
            enumerator: None,
            indenter: None,
            root_style: None,
            item_style: None,
            enumerator_style: None,
            item_style_func: None,
            enumerator_style_func: None,
        }
    }

    /// Sets the root value of this tree.
    ///
    /// The root value is displayed at the top of the tree when rendered.
    /// If no root is set, the tree will start directly with its children.
    ///
    /// # Arguments
    ///
    /// * `root` - The root value (anything convertible to String)
    ///
    /// # Examples
    ///
    /// ```rust
    /// use lipgloss_tree::Tree;
    ///
    /// let tree = Tree::new().root("My Root");
    /// assert_eq!(tree.value(), "My Root");
    /// ```
    pub fn root(mut self, root: impl Into<String>) -> Self {
        self.value = root.into();
        self
    }

    /// Sets the visibility of this tree.
    ///
    /// Hidden trees are not rendered in the output, but their children
    /// may still be visible depending on the rendering context.
    ///
    /// # Arguments
    ///
    /// * `hide` - `true` to hide this tree, `false` to show it
    ///
    /// # Examples
    ///
    /// ```rust
    /// use lipgloss_tree::Tree;
    ///
    /// let hidden_tree = Tree::new().root("Hidden").hide(true);
    /// assert!(hidden_tree.hidden());
    /// ```
    pub fn hide(mut self, hide: bool) -> Self {
        self.hidden = hide;
        self
    }

    /// Sets the offset range for displaying children.
    ///
    /// This allows you to display only a subset of the tree's children,
    /// which is useful for pagination or filtering large trees.
    ///
    /// # Arguments
    ///
    /// * `start` - Starting index (inclusive) for child display
    /// * `end` - Ending index (exclusive) for child display
    ///
    /// # Examples
    ///
    /// ```rust
    /// use lipgloss_tree::Tree;
    ///
    /// let tree = Tree::new()
    ///     .child(vec!["A".into(), "B".into(), "C".into(), "D".into()])
    ///     .offset(1, 3); // Will only show children B and C
    /// ```
    ///
    /// # Note
    ///
    /// If `start > end`, the values will be swapped. If `end` exceeds the
    /// number of children, it will be clamped to the children count.
    pub fn offset(mut self, start: usize, end: usize) -> Self {
        let (start, end) = if start > end {
            (end, start)
        } else {
            (start, end)
        };

        let end = if end > self.children.length() {
            self.children.length()
        } else {
            end
        };

        self.offset = [start, end];
        self
    }

    /// Adds multiple children to this tree.
    ///
    /// This method accepts a vector of boxed nodes and appends them all
    /// to the tree's children collection. It matches Go's `Child(...any)` method
    /// for API compatibility.
    ///
    /// # Arguments
    ///
    /// * `children` - A vector of boxed nodes to add as children
    ///
    /// # Examples
    ///
    /// ```rust
    /// use lipgloss_tree::Tree;
    ///
    /// let tree = Tree::new()
    ///     .root("Parent")
    ///     .child(vec![
    ///         "Child 1".into(),
    ///         "Child 2".into(),
    ///         Tree::new().root("Nested").into(),
    ///     ]);
    ///
    /// assert_eq!(tree.children().length(), 3);
    /// ```
    pub fn child(mut self, children: Vec<Box<dyn Node>>) -> Self {
        for child in children {
            self.children.append(child);
        }
        self
    }

    /// Adds a single child to this tree.
    ///
    /// This is a convenience method for adding one child at a time,
    /// accepting anything that can be converted into a boxed node.
    ///
    /// # Arguments
    ///
    /// * `child` - A node (or convertible value) to add as a child
    ///
    /// # Examples
    ///
    /// ```rust
    /// use lipgloss_tree::{Tree, Leaf};
    ///
    /// let tree = Tree::new()
    ///     .root("Parent")
    ///     .add_child("String child")
    ///     .add_child(Leaf::new("Leaf child", false))
    ///     .add_child(Tree::new().root("Tree child"));
    ///
    /// assert_eq!(tree.children().length(), 3);
    /// ```
    pub fn add_child(mut self, child: impl Into<Box<dyn Node>>) -> Self {
        self.children.append(child.into());
        self
    }

    /// Sets a custom enumerator function for this tree.
    ///
    /// The enumerator generates branch characters (like ├──, └──) based on
    /// each child's position. This overrides the default enumerator for this tree.
    ///
    /// # Arguments
    ///
    /// * `enumerator` - A function that takes `(children, index)` and returns a string
    ///
    /// # Examples
    ///
    /// ```rust
    /// use lipgloss_tree::Tree;
    ///
    /// let tree = Tree::new()
    ///     .root("Custom")
    ///     .enumerator(|children, i| {
    ///         if i == children.length() - 1 {
    ///             "╰──".to_string()  // Rounded last branch
    ///         } else {
    ///             "├──".to_string()  // Standard branch
    ///         }
    ///     })
    ///     .child(vec!["A".into(), "B".into()]);
    /// ```
    pub fn enumerator(mut self, enumerator: crate::Enumerator) -> Self {
        self.enumerator = Some(enumerator);
        self
    }

    /// Sets a custom indenter function for this tree.
    ///
    /// The indenter generates indentation strings for nested child content.
    /// This overrides the default indenter for this tree.
    ///
    /// # Arguments
    ///
    /// * `indenter` - A function that takes `(children, index)` and returns an indentation string
    ///
    /// # Examples
    ///
    /// ```rust
    /// use lipgloss_tree::Tree;
    ///
    /// let tree = Tree::new()
    ///     .indenter(|children, i| {
    ///         if i == children.length() - 1 {
    ///             "    ".to_string()  // Spaces for last child
    ///         } else {
    ///             "│   ".to_string()  // Vertical line for continuing
    ///         }
    ///     });
    /// ```
    pub fn indenter(mut self, indenter: crate::Indenter) -> Self {
        self.indenter = Some(indenter);
        self
    }

    /// Sets the styling for the root value of this tree.
    ///
    /// # Arguments
    ///
    /// * `style` - The lipgloss `Style` to apply to the root
    ///
    /// # Examples
    ///
    /// ```rust
    /// use lipgloss_tree::Tree;
    /// use lipgloss::Style;
    ///
    /// let tree = Tree::new()
    ///     .root("Styled Root")
    ///     .root_style(Style::new().bold(true).foreground("blue".into()));
    /// ```
    pub fn root_style(mut self, style: Style) -> Self {
        self.root_style = Some(style);
        self
    }

    /// Sets the base style applied to all child items.
    ///
    /// This style is applied to item content before any style functions.
    ///
    /// # Arguments
    ///
    /// * `style` - The lipgloss `Style` to apply to all items
    ///
    /// # Examples
    ///
    /// ```rust
    /// use lipgloss_tree::Tree;
    /// use lipgloss::Style;
    ///
    /// let tree = Tree::new()
    ///     .child(vec!["Item 1".into(), "Item 2".into()])
    ///     .item_style(Style::new().foreground("green".into()));
    /// ```
    pub fn item_style(mut self, style: Style) -> Self {
        self.item_style = Some(style);
        self
    }

    /// Sets the base style applied to all enumerators (branch characters).
    ///
    /// This style is applied to enumerators before any style functions.
    ///
    /// # Arguments
    ///
    /// * `style` - The lipgloss `Style` to apply to all enumerators
    ///
    /// # Examples
    ///
    /// ```rust
    /// use lipgloss_tree::Tree;
    /// use lipgloss::Style;
    ///
    /// let tree = Tree::new()
    ///     .child(vec!["Item 1".into(), "Item 2".into()])
    ///     .enumerator_style(Style::new().foreground("yellow".into()));
    /// ```
    pub fn enumerator_style(mut self, style: Style) -> Self {
        self.enumerator_style = Some(style);
        self
    }

    /// Sets a dynamic styling function for child items.
    ///
    /// The function receives the children collection and the current index,
    /// allowing for context-aware styling decisions.
    ///
    /// # Arguments
    ///
    /// * `func` - A function that takes `(children, index)` and returns a `Style`
    ///
    /// # Examples
    ///
    /// ```rust
    /// use lipgloss_tree::Tree;
    /// use lipgloss::Style;
    ///
    /// let tree = Tree::new()
    ///     .child(vec!["First".into(), "Second".into(), "Third".into()])
    ///     .item_style_func(|_children, i| {
    ///         if i % 2 == 0 {
    ///             Style::new().foreground("red".into())
    ///         } else {
    ///             Style::new().foreground("blue".into())
    ///         }
    ///     });
    /// ```
    pub fn item_style_func(mut self, func: crate::StyleFunc) -> Self {
        self.item_style_func = Some(func);
        self
    }

    /// Sets a dynamic styling function for enumerators (branch characters).
    ///
    /// The function receives the children collection and the current index,
    /// allowing for context-aware styling of branch characters.
    ///
    /// # Arguments
    ///
    /// * `func` - A function that takes `(children, index)` and returns a `Style`
    ///
    /// # Examples
    ///
    /// ```rust
    /// use lipgloss_tree::Tree;
    /// use lipgloss::Style;
    ///
    /// let tree = Tree::new()
    ///     .child(vec!["Item 1".into(), "Item 2".into()])
    ///     .enumerator_style_func(|children, i| {
    ///         if i == children.length() - 1 {
    ///             Style::new().foreground("red".into())   // Last item in red
    ///         } else {
    ///             Style::new().foreground("green".into()) // Others in green
    ///         }
    ///     });
    /// ```
    pub fn enumerator_style_func(mut self, func: crate::StyleFunc) -> Self {
        self.enumerator_style_func = Some(func);
        self
    }

    /// Gets the enumerator for this tree.
    pub fn get_enumerator(&self) -> Option<&crate::Enumerator> {
        self.enumerator.as_ref()
    }

    /// Gets the indenter for this tree.
    pub fn get_indenter(&self) -> Option<&crate::Indenter> {
        self.indenter.as_ref()
    }

    /// Gets the root style.
    pub fn get_root_style(&self) -> Option<&Style> {
        self.root_style.as_ref()
    }

    /// Gets the item style function.
    pub fn get_item_style_func(&self) -> Option<&crate::StyleFunc> {
        self.item_style_func.as_ref()
    }

    /// Gets the enumerator style function.
    pub fn get_enumerator_style_func(&self) -> Option<&crate::StyleFunc> {
        self.enumerator_style_func.as_ref()
    }

    /// Gets the item style.
    pub fn get_item_style(&self) -> Option<&Style> {
        self.item_style.as_ref()
    }

    /// Gets the enumerator style.
    pub fn get_enumerator_style(&self) -> Option<&Style> {
        self.enumerator_style.as_ref()
    }

}

impl Default for Tree {
    /// Creates a default tree instance.
    ///
    /// Equivalent to `Tree::new()` - creates an empty tree with no root value,
    /// no children, and default rendering settings.
    fn default() -> Self {
        Self::new()
    }
}

impl Node for Tree {
    fn value(&self) -> String {
        self.value.clone()
    }

    fn children(&self) -> Box<dyn Children> {
        // Return a subset of children based on offset, cloning nodes to preserve structure
        let start = self.offset[0];
        let end = if self.offset[1] == 0 {
            self.children.length()
        } else {
            self.children.length().saturating_sub(self.offset[1])
        };

        let mut filtered_children = NodeChildren::new();
        for i in start..end.min(self.children.length()) {
            if let Some(node) = self.children.at(i) {
                if node.hidden() {
                    continue;
                }
                filtered_children.append(node.clone_node());
            }
        }
        Box::new(filtered_children)
    }

    fn hidden(&self) -> bool {
        self.hidden
    }

    fn set_hidden(&mut self, hidden: bool) {
        self.hidden = hidden;
    }

    fn set_value(&mut self, value: String) {
        self.value = value;
    }

    fn get_enumerator(&self) -> Option<&crate::Enumerator> {
        self.enumerator.as_ref()
    }

    fn get_indenter(&self) -> Option<&crate::Indenter> {
        self.indenter.as_ref()
    }


    fn get_item_style(&self) -> Option<&Style> {
        self.item_style.as_ref()
    }

    fn get_enumerator_style(&self) -> Option<&Style> {
        self.enumerator_style.as_ref()
    }

    fn get_item_style_func(&self) -> Option<&crate::StyleFunc> {
        self.item_style_func.as_ref()
    }

    fn get_enumerator_style_func(&self) -> Option<&crate::StyleFunc> {
        self.enumerator_style_func.as_ref()
    }
}

impl fmt::Display for Tree {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut renderer = crate::renderer::Renderer::new()
            .enumerator(self.enumerator.unwrap_or(crate::default_enumerator))
            .indenter(self.indenter.unwrap_or(crate::default_indenter));

        // Build a TreeStyle snapshot from this Tree and apply to renderer
        let style = crate::renderer::TreeStyle {
            enumerator_func: self
                .enumerator_style_func
                .unwrap_or(|_, _| Style::new().padding_right(1)),
            item_func: self.item_style_func.unwrap_or(|_, _| Style::new()),
            root: self.root_style.clone().unwrap_or_default(),
            enumerator_base: self.enumerator_style.clone(),
            item_base: self.item_style.clone(),
        };
        renderer = renderer.style(style);

        let output = renderer.render(self, true, "");
        write!(f, "{}", output)
    }
}

// Direct conversions to Box<dyn Node> for the simplified API
impl From<&str> for Box<dyn Node> {
    fn from(s: &str) -> Self {
        Box::new(Leaf::new(s.to_string(), false))
    }
}

impl From<String> for Box<dyn Node> {
    fn from(s: String) -> Self {
        Box::new(Leaf::new(s, false))
    }
}

impl From<Tree> for Box<dyn Node> {
    fn from(tree: Tree) -> Self {
        Box::new(tree)
    }
}

impl From<Leaf> for Box<dyn Node> {
    fn from(leaf: Leaf) -> Self {
        Box::new(leaf)
    }
}

/// Creates a new tree with the specified root value.
///
/// This is a convenience function equivalent to `Tree::new().root(root)`.
///
/// # Arguments
///
/// * `root` - The root value for the tree (anything convertible to String)
///
/// # Returns
///
/// A new `Tree` instance with the specified root value
///
/// # Examples
///
/// ```rust
/// use lipgloss_tree::root;
///
/// let tree = root("My Project")
///     .child(vec!["file1.txt".into(), "file2.txt".into()]);
///
/// assert_eq!(tree.value(), "My Project");
/// assert_eq!(tree.children().length(), 2);
/// ```
pub fn root(root: impl Into<String>) -> Tree {
    Tree::new().root(root)
}

/// Creates a `NodeChildren` collection from string slice data.
///
/// This is a convenience function for quickly creating a collection of
/// leaf nodes from string data. Each string becomes a visible leaf node.
///
/// # Arguments
///
/// * `data` - A slice of string references to convert into leaf nodes
///
/// # Returns
///
/// A `NodeChildren` collection containing a leaf node for each string
///
/// # Examples
///
/// ```rust
/// use lipgloss_tree::new_string_data;
///
/// let children = new_string_data(&["Item 1", "Item 2", "Item 3"]);
/// assert_eq!(children.length(), 3);
/// assert_eq!(children.at(0).unwrap().value(), "Item 1");
/// ```
pub fn new_string_data(data: &[&str]) -> NodeChildren {
    let mut result = NodeChildren::new();
    for &item in data {
        result.append(Box::new(Leaf::new(item, false)));
    }
    result
}

/// A filtered view of children that applies a predicate function.
///
/// `Filter` wraps a children collection and applies a filtering function
/// to determine which children should be visible. Only children that pass
/// the filter predicate are accessible through the `Children` interface.
///
/// # Examples
///
/// ```rust
/// use lipgloss_tree::{Filter, new_string_data};
///
/// let data = new_string_data(&["A", "B", "C", "D"]);
/// let filtered = Filter::new(Box::new(data))
///     .filter(|i| i % 2 == 0); // Only even indices
///
/// assert_eq!(filtered.length(), 2); // Only "A" and "C" are visible
/// ```
pub struct Filter {
    /// The underlying children collection to filter
    data: Box<dyn Children>,
    /// Optional predicate function for filtering
    filter: Option<Box<dyn Fn(usize) -> bool>>,
}

impl Filter {
    /// Creates a new filter with no filtering applied.
    ///
    /// Initially, all children from the underlying data will be visible.
    /// Use the `filter()` method to apply filtering logic.
    ///
    /// # Arguments
    ///
    /// * `data` - The children collection to wrap and filter
    ///
    /// # Examples
    ///
    /// ```rust
    /// use lipgloss_tree::{Filter, new_string_data};
    ///
    /// let data = new_string_data(&["Item 1", "Item 2"]);
    /// let filter = Filter::new(Box::new(data));
    /// assert_eq!(filter.length(), 2); // No filtering applied yet
    /// ```
    pub fn new(data: Box<dyn Children>) -> Self {
        Self { data, filter: None }
    }

    /// Sets the filtering predicate function.
    ///
    /// The function receives the index of each child in the underlying
    /// collection and should return `true` for children that should be
    /// visible, `false` for children that should be hidden.
    ///
    /// # Arguments
    ///
    /// * `f` - A predicate function that takes an index and returns a boolean
    ///
    /// # Examples
    ///
    /// ```rust
    /// use lipgloss_tree::{Filter, new_string_data};
    ///
    /// let data = new_string_data(&["A", "B", "C", "D", "E"]);
    /// let filtered = Filter::new(Box::new(data))
    ///     .filter(|i| i >= 2); // Only show items at index 2 and higher
    ///
    /// assert_eq!(filtered.length(), 3); // "C", "D", "E"
    /// assert_eq!(filtered.at(0).unwrap().value(), "C");
    /// ```
    pub fn filter<F>(mut self, f: F) -> Self
    where
        F: Fn(usize) -> bool + 'static,
    {
        self.filter = Some(Box::new(f));
        self
    }
}

impl Children for Filter {
    fn at(&self, index: usize) -> Option<&dyn Node> {
        if let Some(ref filter_func) = self.filter {
            let mut j = 0;
            for i in 0..self.data.length() {
                if filter_func(i) {
                    if j == index {
                        return self.data.at(i);
                    }
                    j += 1;
                }
            }
            None
        } else {
            self.data.at(index)
        }
    }

    fn length(&self) -> usize {
        if let Some(ref filter_func) = self.filter {
            let mut count = 0;
            for i in 0..self.data.length() {
                if filter_func(i) {
                    count += 1;
                }
            }
            count
        } else {
            self.data.length()
        }
    }
}
