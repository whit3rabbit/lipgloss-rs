//! Enumerator functions for tree branch characters and indentation.
//!
//! This module provides the core functions for generating tree branch
//! characters (├──, └──, etc.) and indentation strings for nested content.
//! It includes both built-in enumerators with box-drawing characters and
//! type definitions for custom enumerator functions.

use crate::Children;
use lipgloss::Style;

/// Function type for generating tree branch characters.
///
/// An `Enumerator` takes a children collection and an index, then returns
/// the appropriate branch character string for that position. Typically,
/// the last child in a sequence gets a different character (└──) than
/// intermediate children (├──).
///
/// # Arguments
///
/// * `children` - The children collection being enumerated
/// * `index` - The zero-based index of the current child
///
/// # Returns
///
/// A string representing the branch character(s) for this position
///
/// # Examples
///
/// ```rust
/// use lipgloss_tree::{Children, Enumerator};
///
/// let custom_enumerator: Enumerator = |children, index| {
///     if index == children.length() - 1 {
///         "└──".to_string()  // Last child
///     } else {
///         "├──".to_string()  // Intermediate child
///     }
/// };
/// ```
pub type Enumerator = fn(&dyn Children, usize) -> String;

/// Function type for generating indentation strings for nested tree content.
///
/// An `Indenter` generates the indentation used for child nodes that appear
/// below their parent. This creates the visual connection lines between
/// parent and nested content. The indentation typically differs based on
/// whether there are more siblings following (│ continues the line) or if
/// this is the last child (spaces for clean termination).
///
/// # Arguments
///
/// * `children` - The children collection being indented
/// * `index` - The zero-based index of the current child
///
/// # Returns
///
/// A string representing the indentation for nested content under this child
///
/// # Examples
///
/// ```rust
/// use lipgloss_tree::{Children, Indenter};
///
/// let custom_indenter: Indenter = |children, index| {
///     if index == children.length() - 1 {
///         "    ".to_string()  // Spaces for last child
///     } else {
///         "│   ".to_string()  // Vertical line for continuing branches
///     }
/// };
/// ```
pub type Indenter = fn(&dyn Children, usize) -> String;

/// Function type for generating styles based on child position.
///
/// A `StyleFunc` dynamically determines the styling to apply to tree elements
/// based on the child's position within its parent's children collection.
/// This allows for positional styling like alternating colors, highlighting
/// specific indices, or applying different styles to first/last children.
///
/// # Arguments
///
/// * `children` - The children collection containing the styled element
/// * `index` - The zero-based index of the current child being styled
///
/// # Returns
///
/// A `Style` object with the desired formatting for this position
///
/// # Examples
///
/// ```rust
/// use lipgloss::{Style, Color};
/// use lipgloss_tree::{Children, StyleFunc};
///
/// // Alternating colors based on index
/// let alternating_style: StyleFunc = |_, index| {
///     if index % 2 == 0 {
///         Style::new().foreground(Color::from("blue"))
///     } else {
///         Style::new().foreground(Color::from("green"))
///     }
/// };
///
/// // Highlight first and last items
/// let highlight_ends: StyleFunc = |children, index| {
///     if index == 0 || index == children.length() - 1 {
///         Style::new().bold(true).foreground(Color::from("red"))
///     } else {
///         Style::new()
///     }
/// };
/// ```
pub type StyleFunc = fn(&dyn Children, usize) -> Style;

/// Default tree enumerator using standard box-drawing characters.
///
/// This enumerator generates the classic tree structure using Unicode
/// box-drawing characters. Intermediate children get "├──" and the
/// last child gets "└──" to properly terminate the branch.
///
/// # Arguments
///
/// * `children` - The children collection being enumerated
/// * `index` - The zero-based index of the current child
///
/// # Returns
///
/// "├──" for intermediate children, "└──" for the last child
///
/// # Output Example
///
/// ```text
/// ├── Foo
/// ├── Bar
/// ├── Baz
/// └── Qux
/// ```
///
/// # Examples
///
/// ```rust
/// use lipgloss_tree::{default_enumerator, new_string_data};
///
/// let children = new_string_data(&["Foo", "Bar", "Qux"]);
/// assert_eq!(default_enumerator(&children, 0), "├──");  // First child
/// assert_eq!(default_enumerator(&children, 1), "├──");  // Middle child  
/// assert_eq!(default_enumerator(&children, 2), "└──");  // Last child
/// ```
pub fn default_enumerator(children: &dyn Children, index: usize) -> String {
    if children.length() > 0 && children.length() - 1 == index {
        "└──".to_string()
    } else {
        "├──".to_string()
    }
}

/// Tree enumerator using rounded box-drawing characters for the last child.
///
/// Similar to the default enumerator, but uses a rounded corner character
/// (╰──) for the last child instead of the standard corner (└──). This
/// provides a softer, more modern aesthetic while maintaining the same
/// tree structure.
///
/// # Arguments
///
/// * `children` - The children collection being enumerated
/// * `index` - The zero-based index of the current child
///
/// # Returns
///
/// "├──" for intermediate children, "╰──" for the last child
///
/// # Output Example
///
/// ```text
/// ├── Foo
/// ├── Bar
/// ├── Baz
/// ╰── Qux
/// ```
///
/// # Examples
///
/// ```rust
/// use lipgloss_tree::{rounded_enumerator, new_string_data};
///
/// let children = new_string_data(&["Foo", "Bar", "Qux"]);
/// assert_eq!(rounded_enumerator(&children, 0), "├──");  // First child
/// assert_eq!(rounded_enumerator(&children, 1), "├──");  // Middle child
/// assert_eq!(rounded_enumerator(&children, 2), "╰──");  // Last child (rounded)
/// ```
pub fn rounded_enumerator(children: &dyn Children, index: usize) -> String {
    if children.length() > 0 && children.length() - 1 == index {
        "╰──".to_string()
    } else {
        "├──".to_string()
    }
}

/// Default tree indenter for nested content and multiline text.
///
/// This indenter creates the visual connection lines for nested tree content.
/// It uses a vertical line (│) with spaces for continuing branches and
/// plain spaces for content under the last child. The spacing is carefully
/// calculated to align with the default enumerator characters.
///
/// # Arguments
///
/// * `children` - The children collection being indented
/// * `index` - The zero-based index of the current child
///
/// # Returns
///
/// "│   " (vertical line + 3 spaces) for continuing branches,
/// "    " (4 spaces) for content under the last child
///
/// # Output Example
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
///
/// # Examples
///
/// ```rust
/// use lipgloss_tree::{default_indenter, new_string_data};
///
/// let children = new_string_data(&["Foo", "Bar", "Baz"]);
/// assert_eq!(default_indenter(&children, 0), "│   ");   // Continuing branch
/// assert_eq!(default_indenter(&children, 1), "│   ");   // Continuing branch
/// assert_eq!(default_indenter(&children, 2), "    ");   // Last child (spaces only)
/// ```
///
/// # Design Notes
///
/// The spacing is designed to align with standard enumerator widths:
/// - "├── " is 4 characters wide
/// - "└── " is 4 characters wide  
/// - "│   " provides continuing vertical line + 3 spaces = 4 total
/// - "    " provides 4 spaces for clean termination
pub fn default_indenter(children: &dyn Children, index: usize) -> String {
    if children.length() > 0 && children.length() - 1 == index {
        "    ".to_string()  // 4 spaces for last item (to align with "└── " width)
    } else {
        "│   ".to_string()   // 3 spaces after │ (to align with "├── " width)
    }
}
