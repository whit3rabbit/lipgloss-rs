//! Lipgloss - Terminal styling made simple and delightful
//!
//! Lipgloss is a Rust port of the popular Go library for styling terminal layouts
//! and building Terminal User Interfaces (TUIs). It provides a comprehensive set of
//! tools for creating beautiful, styled terminal output with support for colors,
//! borders, alignment, positioning, and complex layout management.
//!
//! # Features
//!
//! - **Rich styling**: Colors (ANSI 16, 256, and true color), bold, italic, underline, strikethrough
//! - **Flexible layouts**: Horizontal and vertical joining, precise positioning
//! - **Border system**: Multiple built-in border styles with customization options
//! - **Color profiles**: Automatic adaptation to terminal capabilities
//! - **Unicode support**: Proper handling of wide characters and emojis
//! - **Composable design**: Chain styles and combine multiple elements seamlessly
//!
//! # Quick Start
//!
//! ```rust
//! use lipgloss::{Style, Color, rounded_border};
//!
//! // Create a styled text block
//! let style = Style::new()
//!     .foreground(Color("205".to_string()))
//!     .background(Color("235".to_string()))
//!     .padding(1, 2, 1, 2)
//!     .border(rounded_border())
//!     .border_foreground(Color("63".to_string()));
//!
//! let rendered = style.render("Hello, Lipgloss!");
//! println!("{}", rendered);
//! ```
//!
//! # Core Modules
//!
//! - [`style`] - Core styling functionality and the main `Style` struct
//! - [`color`] - Color definitions and terminal color profile management
//! - [`mod@gradient`] - Color gradients and 2D color grids for advanced styling
//! - [`border`] - Border styles and customization
//! - [`mod@align`] - Text alignment utilities
//! - [`position`] - Positioning and placement functions
//! - [`join`] - Horizontal and vertical layout joining
//! - [`mod@size`] - Dimension measurement and calculation
//! - [`whitespace`] - Styled whitespace and filler generation
//! - [`renderer`] - Terminal rendering and color profile detection
//!
//! # Advanced Usage
//!
//! Complex layouts with multiple components:
//!
//! ```rust
//! use lipgloss::{Style, Color, join_horizontal, join_vertical, normal_border, CENTER, LEFT, TOP};
//!
//! let header = Style::new()
//!     .align_horizontal(CENTER)
//!     .foreground(Color("86".to_string()))
//!     .render("Application Title");
//!
//! let left_panel = Style::new()
//!     .width(30)
//!     .padding(1, 2, 1, 2)
//!     .border(normal_border())
//!     .render("Left content");
//!
//! let right_panel = Style::new()
//!     .width(30)
//!     .padding(1, 2, 1, 2)
//!     .border(normal_border())
//!     .render("Right content");
//!
//! let body = join_horizontal(TOP, &[&left_panel, &right_panel]);
//! let layout = join_vertical(LEFT, &[&header, &body]);
//!
//! println!("{}", layout);
//! ```
//!
//! This crate maintains API compatibility with the original Go implementation
//! while following Rust idioms and leveraging Rust's type system for safer
//! terminal styling.

#![allow(missing_docs)]

// Constants for Go API compatibility

/// Special value for [`Style::tab_width`] to disable tab conversion entirely.
///
/// When passed to [`Style::tab_width`], this value instructs the renderer to leave
/// tab characters (`\t`) unchanged rather than converting them to spaces. This is
/// useful when you want to preserve the original tab characters in the output.
///
/// # Examples
///
/// ```rust
/// use lipgloss::{Style, NO_TAB_CONVERSION};
///
/// // Leave tabs intact (don't convert to spaces)
/// let style = Style::new().tab_width(NO_TAB_CONVERSION);
/// let text_with_tabs = "Column1\tColumn2\tColumn3";
/// let rendered = style.render(text_with_tabs);
/// // Output contains original tab characters
/// ```
///
/// # See Also
///
/// - [`Style::tab_width`] - Set tab width or disable conversion
pub const NO_TAB_CONVERSION: i32 = -1;

pub mod align;
pub mod blending;
pub mod border;
pub mod color;
pub mod gradient;
pub mod join;
pub mod position;
pub mod renderer;
pub mod security;
pub mod size;
pub mod style;
pub mod utils;
pub mod whitespace;

pub use align::*;
pub use blending::{blend_1d, blend_2d};
pub use border::*;
pub use color::*;
pub use gradient::{bilinear_interpolation_grid, gradient, gradient_rgb};
pub use join::*;
pub use position::*;
pub use renderer::*;
pub use size::*;
pub use style::*;

// Avoid re-exporting all of utils to prevent name clashes with size (width/height).
// Re-export only the public helpers and API-parity items we want at the crate root.
pub use utils::{
    get_lines,
    get_lines_visible,
    new_range,
    strip_ansi,
    style_ranges,
    style_runes,
    which_sides_bool,
    which_sides_color,
    // CSS helper functions for shorthand notation
    which_sides_int,
    width_visible,
    // Go API parity aliases and types
    NewRange,
    Range,
    StyleRanges,
    StyleRunes,
};
