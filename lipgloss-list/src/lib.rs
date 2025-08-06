//! Package list allows you to build lists, as simple or complicated as you need.
//!
//! Simply, define a list with some items and set its rendering properties, like
//! enumerator and styling:
//!
//! ```rust
//! use lipgloss_list::{List, arabic};
//!
//! let groceries = List::new()
//!     .items(vec!["Bananas", "Barley", "Cashews", "Milk"])
//!     .enumerator(arabic);
//!
//! println!("{}", groceries);
//! ```

pub mod enumerator;

use lipgloss::Style;
use lipgloss_tree::{Children, Leaf, Node, Tree};
use std::fmt;

// Re-export enumerators for convenience
pub use enumerator::{alphabet, arabic, asterisk, bullet, dash, roman, Enumerator, Indenter};

/// Items represents the list items.
pub type Items = Box<dyn Children>;

/// StyleFunc is the style function that determines the style of an item.
///
/// It takes the list items and index of the list and determines the lipgloss
/// Style to use for that index.
///
/// Example:
///
/// ```rust
/// use lipgloss::{Color, Style};
/// use lipgloss_list::List;
/// use lipgloss_tree::Children;
///
/// let style_func = |items: &dyn Children, i: usize| {
///     match i {
///         0 => Style::new().foreground(Color::from("#ff0000")),
///         1 => Style::new().foreground(Color::from("#00ff00")),
///         2 => Style::new().foreground(Color::from("#0000ff")),
///         _ => Style::new(),
///     }
/// };
/// ```
pub type StyleFunc = fn(&dyn Children, usize) -> Style;

/// List represents a list of items that can be displayed. Lists can contain
/// lists as items, they will be rendered as nested (sub)lists.
///
/// In fact, lists can contain anything as items, like Table or Tree.
pub struct List {
    tree: Tree,
}

impl List {
    /// Creates a new list with the given items.
    ///
    /// ```rust
    /// use lipgloss_list::List;
    ///
    /// let alphabet = List::new()
    ///     .items(vec!["A", "B", "C", "D", "E", "F"]);
    /// ```
    ///
    /// Items can be other lists, trees, tables, rendered markdown;
    /// anything you want, really.
    pub fn new() -> Self {
        let mut tree = Tree::new();
        tree = tree
            .enumerator(bullet as lipgloss_tree::Enumerator)
            .indenter(list_indenter);

        Self { tree }
    }

    /// Creates a new list with initial items.
    pub fn from_items(items: Vec<&str>) -> Self {
        let mut list = Self::new();
        for item in items {
            list = list.item(item);
        }
        list
    }

    /// Returns whether this list is hidden.
    pub fn hidden(&self) -> bool {
        self.tree.hidden()
    }

    /// Hides this list.
    /// If this list is hidden, it will not be shown when rendered.
    pub fn hide(mut self, hide: bool) -> Self {
        self.tree = self.tree.hide(hide);
        self
    }

    /// Sets the start and end offset for the list.
    ///
    /// Example:
    /// ```rust
    /// use lipgloss_list::List;
    ///
    /// let l = List::new()
    ///     .items(vec!["A", "B", "C", "D"])
    ///     .offset(1, 1);
    ///
    /// println!("{}", l);
    /// // • B
    /// // • C
    /// ```
    pub fn offset(mut self, start: usize, end: usize) -> Self {
        self.tree = self.tree.offset(start, end);
        self
    }

    /// Returns the value of this node.
    pub fn value(&self) -> String {
        self.tree.value()
    }

    /// Sets the enumerator style for all enumerators.
    ///
    /// To set the enumerator style conditionally based on the item value or index,
    /// use `enumerator_style_func`.
    pub fn enumerator_style(mut self, style: Style) -> Self {
        self.tree = self.tree.enumerator_style(style);
        self
    }

    /// Sets the enumerator style function for the list items.
    ///
    /// Use this to conditionally set different styles based on the current items,
    /// sibling items, or index values (i.e. even or odd).
    ///
    /// Example:
    ///
    /// ```rust
    /// use lipgloss::{Color, Style};
    /// use lipgloss_list::List;
    ///
    /// let l = List::new()
    ///     .enumerator_style_func(|_items, i| {
    ///         if i % 2 == 0 {
    ///             Style::new().foreground(Color::from("#ff69b4"))
    ///         } else {
    ///             Style::new()
    ///         }
    ///     });
    /// ```
    pub fn enumerator_style_func(mut self, f: StyleFunc) -> Self {
        self.tree = self
            .tree
            .enumerator_style_func(f as lipgloss_tree::StyleFunc);
        self
    }

    /// Sets the indenter implementation. This is used to change the way
    /// the tree is indented. The default indenter places no indentation
    /// for lists (unlike trees).
    ///
    /// You can define your own indenter.
    ///
    /// ```rust
    /// use lipgloss_tree::Children;
    /// use lipgloss_list::List;
    ///
    /// fn arrow_indenter(_children: &dyn Children, _index: usize) -> String {
    ///     "→ ".to_string()
    /// }
    ///
    /// let l = List::new()
    ///     .items(vec!["Foo", "Bar", "Baz"])
    ///     .indenter(arrow_indenter);
    /// ```
    pub fn indenter(mut self, indenter: Indenter) -> Self {
        self.tree = self.tree.indenter(indenter as lipgloss_tree::Indenter);
        self
    }

    /// Sets the item style for all items.
    ///
    /// To set the item style conditionally based on the item value or index,
    /// use `item_style_func`.
    pub fn item_style(mut self, style: Style) -> Self {
        self.tree = self.tree.item_style(style);
        self
    }

    /// Sets the item style function for the list items.
    ///
    /// Use this to conditionally set different styles based on the current items,
    /// sibling items, or index values.
    ///
    /// Example:
    ///
    /// ```rust
    /// use lipgloss::{Color, Style};
    /// use lipgloss_list::List;
    ///
    /// let l = List::new()
    ///     .item_style_func(|_items, i| {
    ///         if i == 0 {
    ///             Style::new().foreground(Color::from("#ff69b4"))
    ///         } else {
    ///             Style::new()
    ///         }
    ///     });
    /// ```
    pub fn item_style_func(mut self, f: StyleFunc) -> Self {
        self.tree = self.tree.item_style_func(f as lipgloss_tree::StyleFunc);
        self
    }

    /// Appends an item to the list.
    ///
    /// ```rust
    /// use lipgloss_list::List;
    ///
    /// let l = List::new()
    ///     .item("Foo")
    ///     .item("Bar")
    ///     .item("Baz");
    /// ```
    pub fn item(mut self, item: &str) -> Self {
        let leaf: Box<dyn Node> = Box::new(Leaf::new(item, false));
        self.tree = self.tree.add_child(leaf);
        self
    }

    /// Appends a generic node to the list.
    ///
    /// This allows adding any `lipgloss_tree::Node` implementation directly.
    pub fn item_node(mut self, node: Box<dyn Node>) -> Self {
        self.tree = self.tree.add_child(node);
        self
    }

    /// Appends another list as a sublist (nested list).
    pub fn item_list(mut self, list: List) -> Self {
        // `Tree` implements `Node`, so we can add it directly as a child.
        let node: Box<dyn Node> = Box::new(list.tree);
        self.tree = self.tree.add_child(node);
        self
    }

    /// Appends multiple items to the list.
    ///
    /// ```rust
    /// use lipgloss_list::List;
    ///
    /// let l = List::new()
    ///     .items(vec!["Foo", "Bar", "Baz"]);
    /// ```
    pub fn items(mut self, items: Vec<&str>) -> Self {
        for item in items {
            self = self.item(item);
        }
        self
    }

    /// Sets the list enumerator.
    ///
    /// There are several predefined enumerators:
    /// • alphabet
    /// • arabic
    /// • bullet
    /// • dash
    /// • roman
    /// • asterisk
    ///
    /// Or, define your own.
    ///
    /// ```rust
    /// use lipgloss_list::{List, arabic};
    ///
    /// let l = List::new()
    ///     .items(vec!["Foo", "Bar", "Baz"])
    ///     .enumerator(arabic);
    /// ```
    pub fn enumerator(mut self, enumerator: Enumerator) -> Self {
        self.tree = self
            .tree
            .enumerator(enumerator as lipgloss_tree::Enumerator);
        self
    }
}

impl Default for List {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for List {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.tree)
    }
}

/// List indenter for nested content within list items.
/// 
/// CRITICAL SPACING REQUIREMENTS:
/// - Regular sublists MUST have 2-space indentation before bullets
/// - Example: "• Parent\n  • Child" (2 spaces before child bullet)
/// - Golden files expect this 2-space pattern for proper visual hierarchy
///
/// GO IMPLEMENTATION DISCREPANCY:
/// - Go's list.New() sets indenter to return single space: `return " "`
/// - However, Go's output produces 2-space indentation for sublists
/// - This suggests Go has additional logic that doubles indentation for nested lists
/// - Our implementation achieves correct output by returning 2 spaces directly
///
/// TREE-IN-LIST ISSUE:
/// - When trees are nested in lists (via item_node), this 2-space indenter
///   can cause extra spacing after tree symbols
/// - Tree symbols have padding_right(1) built-in, expecting 1-space list_indenter
/// - With 2-space list_indenter: tree gets "├──  content" instead of "├── content"
/// - This affects golden_complex_sublist test specifically
///
/// USAGE:
/// - Called by tree renderer for each child's indentation
/// - Applied to continuation lines and nested content
/// - NOT applied to the enumerator/bullet itself
fn list_indenter(_children: &dyn Children, _index: usize) -> String {
    "  ".to_string() // 2 spaces required for sublist visual hierarchy
}

// Go API compatibility aliases
pub use List as ListType;

/// Creates a new list.
pub fn new() -> List {
    List::new()
}

/// Creates a new list with items.
pub fn from_items(items: Vec<&str>) -> List {
    List::from_items(items)
}
