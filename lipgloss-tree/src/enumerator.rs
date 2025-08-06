use crate::Children;
use lipgloss::Style;

/// Enumerator enumerates a tree. Typically, this is used to draw the branches
/// for the tree nodes and is different for the last child.
///
/// For example, the default enumerator would be:
///
/// ```rust
/// # use lipgloss_tree::Children;
/// fn tree_enumerator(children: &dyn Children, index: usize) -> String {
///     if children.length() - 1 == index {
///         "└──".to_string()
///     } else {
///         "├──".to_string()
///     }
/// }
/// ```
pub type Enumerator = fn(&dyn Children, usize) -> String;

/// Indenter indents the children of a tree.
///
/// Indenters allow for displaying nested tree items with connecting borders
/// to sibling nodes.
///
/// For example, the default indenter would be:
///
/// ```rust
/// # use lipgloss_tree::Children;
/// fn tree_indenter(children: &dyn Children, index: usize) -> String {
///     if children.length() - 1 == index {
///         "   ".to_string()
///     } else {
///         "│  ".to_string()
///     }
/// }
/// ```
pub type Indenter = fn(&dyn Children, usize) -> String;

/// StyleFunc is the style function that determines the style of an item.
///
/// It takes the children and index and determines the lipgloss Style to use
/// for that index.
pub type StyleFunc = fn(&dyn Children, usize) -> Style;

/// DefaultEnumerator enumerates a tree.
///
/// ```text
/// ├── Foo
/// ├── Bar
/// ├── Baz
/// └── Qux
/// ```
pub fn default_enumerator(children: &dyn Children, index: usize) -> String {
    if children.length() > 0 && children.length() - 1 == index {
        "└──".to_string()
    } else {
        "├──".to_string()
    }
}

/// RoundedEnumerator enumerates a tree with rounded edges.
///
/// ```text
/// ├── Foo
/// ├── Bar
/// ├── Baz
/// ╰── Qux
/// ```
pub fn rounded_enumerator(children: &dyn Children, index: usize) -> String {
    if children.length() > 0 && children.length() - 1 == index {
        "╰──".to_string()
    } else {
        "├──".to_string()
    }
}

/// DefaultIndenter indents a tree for nested trees and multiline content.
///
/// ```text
/// ├── Foo
/// ├── Bar
/// │   ├── Qux
/// │   ├── Quux
/// │   │   ├── Foo
/// │   │   └── Bar
/// │   └── Quuux
/// └── Baz
/// ```
pub fn default_indenter(children: &dyn Children, index: usize) -> String {
    if children.length() > 0 && children.length() - 1 == index {
        "    ".to_string()  // 4 spaces for last item (to align with "└── " width)
    } else {
        "│   ".to_string()   // 3 spaces after │ (to align with "├── " width)
    }
}
