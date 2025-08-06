//! Text alignment utilities for terminal user interfaces.
//!
//! This module provides high-level text alignment functions that build on top of the
//! lower-level positioning primitives. It offers a simple, intuitive API for aligning
//! text within specified dimensions, similar to the Go lipgloss library.
//!
//! # Examples
//!
//! ```
//! use lipgloss::align::{align, HorizontalAlignment, VerticalAlignment};
//! use lipgloss::whitespace::WhitespaceOption;
//!
//! // Center text both horizontally and vertically
//! let centered = align(
//!     20, // width
//!     5,  // height
//!     HorizontalAlignment::Center,
//!     VerticalAlignment::Center,
//!     "Hello",
//!     &[],
//! );
//!
//! // Right-align text horizontally
//! let right_aligned = lipgloss::align::align_horizontal(
//!     30,
//!     HorizontalAlignment::Right,
//!     "Right aligned text",
//!     &[],
//! );
//! ```

use crate::position::{
    place, place_horizontal, place_vertical, Position, BOTTOM, CENTER, LEFT, RIGHT, TOP,
};
use crate::whitespace::WhitespaceOption;

/// Specifies horizontal alignment options for text positioning.
///
/// This enum provides a high-level interface for horizontal text alignment,
/// mapping to the underlying position system used internally.
///
/// # Examples
///
/// ```
/// use lipgloss::align::{align_horizontal, HorizontalAlignment};
///
/// // Left align (default behavior)
/// let left = align_horizontal(20, HorizontalAlignment::Left, "Text", &[]);
///
/// // Center align
/// let center = align_horizontal(20, HorizontalAlignment::Center, "Text", &[]);
///
/// // Right align
/// let right = align_horizontal(20, HorizontalAlignment::Right, "Text", &[]);
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HorizontalAlignment {
    /// Align text to the left edge of the available space
    Left,
    /// Center text horizontally within the available space
    Center,
    /// Align text to the right edge of the available space
    Right,
}

/// Specifies vertical alignment options for text positioning.
///
/// This enum provides a high-level interface for vertical text alignment,
/// mapping to the underlying position system used internally.
///
/// # Examples
///
/// ```
/// use lipgloss::align::{align_vertical, VerticalAlignment};
///
/// // Top align (default behavior)
/// let top = align_vertical(10, VerticalAlignment::Top, "Text", &[]);
///
/// // Center align
/// let center = align_vertical(10, VerticalAlignment::Center, "Text", &[]);
///
/// // Bottom align
/// let bottom = align_vertical(10, VerticalAlignment::Bottom, "Text", &[]);
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VerticalAlignment {
    /// Align text to the top edge of the available space
    Top,
    /// Center text vertically within the available space
    Center,
    /// Align text to the bottom edge of the available space
    Bottom,
}

/// Converts a high-level horizontal alignment to the internal position representation.
///
/// This function maps the user-friendly `HorizontalAlignment` enum to the
/// lower-level `Position` type used by the positioning system.
///
/// # Arguments
///
/// * `h` - The horizontal alignment to convert
///
/// # Returns
///
/// The corresponding `Position` value for horizontal placement
fn to_position_h(h: HorizontalAlignment) -> Position {
    match h {
        HorizontalAlignment::Left => LEFT,
        HorizontalAlignment::Center => CENTER,
        HorizontalAlignment::Right => RIGHT,
    }
}

/// Converts a high-level vertical alignment to the internal position representation.
///
/// This function maps the user-friendly `VerticalAlignment` enum to the
/// lower-level `Position` type used by the positioning system.
///
/// # Arguments
///
/// * `v` - The vertical alignment to convert
///
/// # Returns
///
/// The corresponding `Position` value for vertical placement
fn to_position_v(v: VerticalAlignment) -> Position {
    match v {
        VerticalAlignment::Top => TOP,
        VerticalAlignment::Center => CENTER,
        VerticalAlignment::Bottom => BOTTOM,
    }
}

/// Aligns text both horizontally and vertically within a specified area.
///
/// This function positions text within a rectangular area defined by width and height,
/// using the specified horizontal and vertical alignments. The text is padded with
/// whitespace as needed to achieve the desired alignment.
///
/// # Arguments
///
/// * `width` - The width of the area in terminal columns
/// * `height` - The height of the area in terminal lines
/// * `h` - The horizontal alignment (Left, Center, or Right)
/// * `v` - The vertical alignment (Top, Center, or Bottom)
/// * `s` - The text to align
/// * `opts` - Whitespace rendering options (e.g., custom characters or styles)
///
/// # Returns
///
/// A string containing the aligned text with appropriate padding
///
/// # Examples
///
/// ```
/// use lipgloss::align::{align, HorizontalAlignment, VerticalAlignment};
///
/// // Center a greeting in a 20x5 box
/// let centered = align(
///     20,
///     5,
///     HorizontalAlignment::Center,
///     VerticalAlignment::Center,
///     "Hello",
///     &[],
/// );
/// // The result will have the appropriate vertical padding
///
/// // Bottom-right align text
/// let bottom_right = align(
///     30,
///     10,
///     HorizontalAlignment::Right,
///     VerticalAlignment::Bottom,
///     "Status: OK",
///     &[],
/// );
/// ```
pub fn align(
    width: i32,
    height: i32,
    h: HorizontalAlignment,
    v: VerticalAlignment,
    s: &str,
    opts: &[WhitespaceOption],
) -> String {
    place(width, height, to_position_h(h), to_position_v(v), s, opts)
}

/// Aligns text horizontally within a specified width.
///
/// This function positions text horizontally within the given width, adding
/// padding spaces as needed. The text's original line structure is preserved.
///
/// # Arguments
///
/// * `width` - The total width in terminal columns
/// * `h` - The horizontal alignment (Left, Center, or Right)
/// * `s` - The text to align
/// * `opts` - Whitespace rendering options (e.g., custom characters or styles)
///
/// # Returns
///
/// A string with each line aligned according to the specified alignment
///
/// # Examples
///
/// ```
/// use lipgloss::align::{align_horizontal, HorizontalAlignment};
///
/// // Right-align text in 30 columns
/// let right = align_horizontal(
///     30,
///     HorizontalAlignment::Right,
///     "Right aligned",
///     &[],
/// );
/// assert!(right.starts_with("                 ")); // padded on left
///
/// // Center multi-line text
/// let centered = align_horizontal(
///     20,
///     HorizontalAlignment::Center,
///     "Line 1\nLine 2",
///     &[],
/// );
/// // Each line will be centered within 20 columns
/// ```
pub fn align_horizontal(
    width: i32,
    h: HorizontalAlignment,
    s: &str,
    opts: &[WhitespaceOption],
) -> String {
    place_horizontal(width, to_position_h(h), s, opts)
}

/// Aligns text vertically within a specified height.
///
/// This function positions text vertically within the given height by adding
/// empty lines above and/or below the content as needed.
///
/// # Arguments
///
/// * `height` - The total height in terminal lines
/// * `v` - The vertical alignment (Top, Center, or Bottom)
/// * `s` - The text to align
/// * `opts` - Whitespace rendering options (e.g., custom characters or styles)
///
/// # Returns
///
/// A string with the appropriate number of lines to achieve the alignment
///
/// # Examples
///
/// ```
/// use lipgloss::align::{align_vertical, VerticalAlignment};
///
/// // Bottom-align text in 10 lines
/// let bottom = align_vertical(
///     10,
///     VerticalAlignment::Bottom,
///     "Status line",
///     &[],
/// );
/// assert_eq!(bottom.lines().count(), 10);
///
/// // Center multi-line text vertically
/// let centered = align_vertical(
///     8,
///     VerticalAlignment::Center,
///     "Line 1\nLine 2\nLine 3",
///     &[],
/// );
/// // The 3 lines will be centered within 8 total lines
/// ```
pub fn align_vertical(
    height: i32,
    v: VerticalAlignment,
    s: &str,
    opts: &[WhitespaceOption],
) -> String {
    place_vertical(height, to_position_v(v), s, opts)
}
