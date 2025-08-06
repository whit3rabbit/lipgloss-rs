//! Positioning and alignment utilities for placing text within boxes.
//!
//! This module provides functions and types for positioning text content within
//! rectangular areas, supporting both horizontal and vertical alignment. The
//! positioning system uses normalized values from 0.0 to 1.0, where 0.0 represents
//! the start (left/top), 0.5 represents the center, and 1.0 represents the end
//! (right/bottom).
//!
//! # Examples
//!
//! ```
//! use lipgloss::position::{place, CENTER};
//! use lipgloss::whitespace::WhitespaceOption;
//!
//! // Center text in a 20x5 box
//! let result = place(20, 5, CENTER, CENTER, "Hello", &[]);
//! ```
//!
//! # Key Components
//!
//! - [`Position`] - A value type representing a position along an axis
//! - Positioning functions: [`place`], [`place_horizontal`], [`place_vertical`]
//! - Pre-defined positions: [`TOP`], [`BOTTOM`], [`CENTER`], [`LEFT`], [`RIGHT`]

/// Position represents a position along a horizontal or vertical axis.
///
/// A value of 0.0 represents the start (left or top) and 1.0 represents the
/// end (right or bottom). 0.5 represents the center.
///
/// # Examples
///
/// ```
/// use lipgloss::position::{Position, CENTER, LEFT, RIGHT};
///
/// // Create a custom position
/// let pos = Position(0.75);
/// assert_eq!(pos.value(), 0.75);
///
/// // Use predefined positions
/// assert_eq!(LEFT.value(), 0.0);
/// assert_eq!(CENTER.value(), 0.5);
/// assert_eq!(RIGHT.value(), 1.0);
/// ```
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Position(pub f64);

impl Position {
    /// Clamp the position to the range [0.0, 1.0].
    ///
    /// This method ensures that the position value is always within the valid range,
    /// where 0.0 represents the start and 1.0 represents the end.
    ///
    /// # Returns
    ///
    /// The clamped position value between 0.0 and 1.0 inclusive.
    ///
    /// # Examples
    ///
    /// ```
    /// use lipgloss::position::Position;
    ///
    /// assert_eq!(Position(-0.5).value(), 0.0);
    /// assert_eq!(Position(0.5).value(), 0.5);
    /// assert_eq!(Position(1.5).value(), 1.0);
    /// ```
    pub fn value(self) -> f64 {
        self.0.clamp(0.0, 1.0)
    }
}

// Top-level constants for convenience (Go-style names) are defined below.

use crate::renderer::default_renderer;
use crate::renderer::Renderer;
use crate::utils::width_visible;
use crate::whitespace::{new_whitespace, WhitespaceOption};

/// Place a string inside a box of width x height at the given horizontal and
/// vertical positions.
///
/// This function positions text within a box of the specified dimensions. The text
/// is placed according to the horizontal and vertical positions provided, with
/// whitespace filling the remaining space.
///
/// # Arguments
///
/// * `width_px` - The width of the box in characters
/// * `height_px` - The height of the box in lines
/// * `h_pos` - Horizontal position (0.0 = left, 0.5 = center, 1.0 = right)
/// * `v_pos` - Vertical position (0.0 = top, 0.5 = center, 1.0 = bottom)
/// * `s` - The string to place in the box
/// * `opts` - Whitespace rendering options
///
/// # Returns
///
/// A string with the text positioned within the box, padded with whitespace.
///
/// # Examples
///
/// ```
/// use lipgloss::position::{place, Position, CENTER};
/// use lipgloss::whitespace::WhitespaceOption;
///
/// let result = place(
///     10,
///     3,
///     CENTER,
///     CENTER,
///     "Hi",
///     &[]
/// );
/// // Result will be "Hi" centered in a 10x3 box
/// ```
pub fn place(
    width_px: i32,
    height_px: i32,
    h_pos: Position,
    v_pos: Position,
    s: &str,
    opts: &[WhitespaceOption],
) -> String {
    let r = default_renderer();
    r.place(width_px, height_px, h_pos, v_pos, s, opts)
}

/// Place a string horizontally in a box of the given width.
///
/// This function positions text horizontally within a box of the specified width.
/// The text is aligned according to the position value, with whitespace filling
/// the remaining horizontal space.
///
/// # Arguments
///
/// * `width_px` - The width of the box in characters
/// * `pos` - Horizontal position (0.0 = left, 0.5 = center, 1.0 = right)
/// * `s` - The string to place in the box
/// * `opts` - Whitespace rendering options
///
/// # Returns
///
/// A string with the text horizontally positioned, padded with whitespace.
///
/// # Examples
///
/// ```
/// use lipgloss::position::{place_horizontal, LEFT, CENTER, RIGHT};
/// use lipgloss::whitespace::WhitespaceOption;
///
/// // Left-aligned
/// let left = place_horizontal(10, LEFT, "Hi", &[]);
/// // Result: "Hi        "
///
/// // Center-aligned
/// let center = place_horizontal(10, CENTER, "Hi", &[]);
/// // Result: "    Hi    "
///
/// // Right-aligned
/// let right = place_horizontal(10, RIGHT, "Hi", &[]);
/// // Result: "        Hi"
/// ```
pub fn place_horizontal(
    width_px: i32,
    pos: Position,
    s: &str,
    opts: &[WhitespaceOption],
) -> String {
    let r = default_renderer();
    r.place_horizontal(width_px, pos, s, opts)
}

/// Place a string vertically in a box of the given height.
///
/// This function positions text vertically within a box of the specified height.
/// The text is aligned according to the position value, with empty lines filling
/// the remaining vertical space.
///
/// # Arguments
///
/// * `height_px` - The height of the box in lines
/// * `pos` - Vertical position (0.0 = top, 0.5 = center, 1.0 = bottom)
/// * `s` - The string to place in the box
/// * `opts` - Whitespace rendering options
///
/// # Returns
///
/// A string with the text vertically positioned, padded with empty lines.
///
/// # Examples
///
/// ```
/// use lipgloss::position::{place_vertical, TOP, CENTER, BOTTOM};
/// use lipgloss::whitespace::WhitespaceOption;
///
/// // Top-aligned
/// let top = place_vertical(5, TOP, "Hi", &[]);
/// // Result: "Hi\n  \n  \n  \n  "
///
/// // Center-aligned
/// let center = place_vertical(5, CENTER, "Hi", &[]);
/// // Result: "  \n  \nHi\n  \n  "
///
/// // Bottom-aligned
/// let bottom = place_vertical(5, BOTTOM, "Hi", &[]);
/// // Result: "  \n  \n  \n  \nHi"
/// ```
pub fn place_vertical(height_px: i32, pos: Position, s: &str, opts: &[WhitespaceOption]) -> String {
    let r = default_renderer();
    r.place_vertical(height_px, pos, s, opts)
}

impl Renderer {
    /// Place a string inside a box of width x height at the given horizontal and
    /// vertical positions using this renderer's settings.
    ///
    /// This method combines horizontal and vertical positioning to place text
    /// within a box of the specified dimensions.
    ///
    /// # Arguments
    ///
    /// * `width_px` - The width of the box in characters
    /// * `height_px` - The height of the box in lines
    /// * `h_pos` - Horizontal position (0.0 = left, 0.5 = center, 1.0 = right)
    /// * `v_pos` - Vertical position (0.0 = top, 0.5 = center, 1.0 = bottom)
    /// * `s` - The string to place in the box
    /// * `opts` - Whitespace rendering options
    ///
    /// # Returns
    ///
    /// A string with the text positioned within the box, padded with whitespace.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use lipgloss::position::{Position, CENTER};
    /// use lipgloss::renderer::Renderer;
    /// use lipgloss::whitespace::WhitespaceOption;
    ///
    /// let renderer = Renderer::default();
    /// let result = renderer.place(
    ///     20,
    ///     5,
    ///     CENTER,
    ///     CENTER,
    ///     "Hello",
    ///     &[]
    /// );
    /// ```
    pub fn place(
        &self,
        width_px: i32,
        height_px: i32,
        h_pos: Position,
        v_pos: Position,
        s: &str,
        opts: &[WhitespaceOption],
    ) -> String {
        self.place_vertical(
            height_px,
            v_pos,
            &self.place_horizontal(width_px, h_pos, s, opts),
            opts,
        )
    }

    /// Place a string horizontally in a box of the given width using this renderer's settings.
    ///
    /// This method positions text horizontally within a box, handling multi-line
    /// strings by aligning each line according to the position value.
    ///
    /// # Arguments
    ///
    /// * `width_px` - The width of the box in characters
    /// * `pos` - Horizontal position (0.0 = left, 0.5 = center, 1.0 = right)
    /// * `s` - The string to place in the box
    /// * `opts` - Whitespace rendering options
    ///
    /// # Returns
    ///
    /// A string with each line horizontally positioned, padded with whitespace.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use lipgloss::position::{Position, CENTER};
    /// use lipgloss::renderer::Renderer;
    /// use lipgloss::whitespace::WhitespaceOption;
    ///
    /// let renderer = Renderer::default();
    /// let result = renderer.place_horizontal(
    ///     20,
    ///     CENTER,
    ///     "Hello\nWorld",
    ///     &[]
    /// );
    /// // Each line will be centered within 20 characters
    /// ```
    pub fn place_horizontal(
        &self,
        width_px: i32,
        pos: Position,
        s: &str,
        opts: &[WhitespaceOption],
    ) -> String {
        let lines: Vec<&str> = s.split('\n').collect();
        let content_width = lines.iter().map(|l| width_visible(l)).max().unwrap_or(0) as i32;
        let gap = width_px - content_width;
        if gap <= 0 {
            return s.to_string();
        }

        let ws = new_whitespace(self, opts);
        let mut out = String::new();
        let v = pos.value();

        for (i, l) in lines.iter().enumerate() {
            // Is this line shorter than the longest line?
            let short = (content_width as usize).saturating_sub(width_visible(l));

            if (v - 0.0).abs() < f64::EPSILON {
                // Left
                out.push_str(l);
                out.push_str(&ws.render((gap as usize) + short));
            } else if (v - 1.0).abs() < f64::EPSILON {
                // Right
                out.push_str(&ws.render((gap as usize) + short));
                out.push_str(l);
            } else {
                // Somewhere in the middle per Go: split proportionally
                let total_gap = (gap as usize) + short;
                let split = ((total_gap as f64) * v).round() as usize;
                let left = total_gap - split;
                let right = total_gap - left;
                out.push_str(&ws.render(left));
                out.push_str(l);
                out.push_str(&ws.render(right));
            }

            if i < lines.len() - 1 {
                out.push('\n');
            }
        }
        out
    }

    /// Place a string vertically in a box of the given height using this renderer's settings.
    ///
    /// This method positions text vertically within a box by adding empty lines
    /// above and/or below the content according to the position value.
    ///
    /// # Arguments
    ///
    /// * `height_px` - The height of the box in lines
    /// * `pos` - Vertical position (0.0 = top, 0.5 = center, 1.0 = bottom)
    /// * `s` - The string to place in the box
    /// * `opts` - Whitespace rendering options
    ///
    /// # Returns
    ///
    /// A string with the text vertically positioned, padded with empty lines.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use lipgloss::position::{Position, CENTER};
    /// use lipgloss::renderer::Renderer;
    /// use lipgloss::whitespace::WhitespaceOption;
    ///
    /// let renderer = Renderer::default();
    /// let result = renderer.place_vertical(
    ///     10,
    ///     CENTER,
    ///     "Hello\nWorld",
    ///     &[]
    /// );
    /// // The two-line text will be centered within 10 lines
    /// ```
    pub fn place_vertical(
        &self,
        height_px: i32,
        pos: Position,
        s: &str,
        opts: &[WhitespaceOption],
    ) -> String {
        let content_height = s.chars().filter(|&c| c == '\n').count() as i32 + 1;
        let gap = height_px - content_height;
        if gap <= 0 {
            return s.to_string();
        }

        let ws = new_whitespace(self, opts);
        let width_px = s.split('\n').map(width_visible).max().unwrap_or(0);
        let empty_line = ws.render(width_px);
        let v = pos.value();
        let mut out = String::new();
        if (v - 0.0).abs() < f64::EPSILON {
            // Top
            out.push_str(s);
            for _ in 0..gap {
                out.push('\n');
                out.push_str(&empty_line);
            }
        } else if (v - 1.0).abs() < f64::EPSILON {
            // Bottom
            for _ in 0..gap {
                out.push_str(&empty_line);
                out.push('\n');
            }
            out.push_str(s);
        } else {
            // Middle
            let split = ((gap as f64) * v).round() as i32;
            let top = gap - split;
            let bottom = gap - top;
            for _ in 0..top {
                out.push_str(&empty_line);
                out.push('\n');
            }
            out.push_str(s);
            for _ in 0..bottom {
                out.push('\n');
                out.push_str(&empty_line);
            }
        }
        out
    }
}

// Aliases for readability, mirroring Go constants.
/// Position at the top of a vertical axis.
///
/// # Examples
///
/// ```
/// use lipgloss::position::{place_vertical, TOP};
/// use lipgloss::whitespace::WhitespaceOption;
///
/// let result = place_vertical(5, TOP, "Text", &[]);
/// // Text will appear at the top with empty lines below
/// ```
pub const TOP: Position = Position(0.0);

/// Position at the bottom of a vertical axis.
///
/// # Examples
///
/// ```
/// use lipgloss::position::{place_vertical, BOTTOM};
/// use lipgloss::whitespace::WhitespaceOption;
///
/// let result = place_vertical(5, BOTTOM, "Text", &[]);
/// // Text will appear at the bottom with empty lines above
/// ```
pub const BOTTOM: Position = Position(1.0);

/// Position at the center of an axis (horizontal or vertical).
///
/// # Examples
///
/// ```
/// use lipgloss::position::{place, CENTER};
/// use lipgloss::whitespace::WhitespaceOption;
///
/// let result = place(10, 5, CENTER, CENTER, "Hi", &[]);
/// // "Hi" will be centered both horizontally and vertically
/// ```
pub const CENTER: Position = Position(0.5);

/// Position at the left of a horizontal axis.
///
/// # Examples
///
/// ```
/// use lipgloss::position::{place_horizontal, LEFT};
/// use lipgloss::whitespace::WhitespaceOption;
///
/// let result = place_horizontal(10, LEFT, "Text", &[]);
/// // Text will be left-aligned with spaces on the right
/// ```
pub const LEFT: Position = Position(0.0);

/// Position at the right of a horizontal axis.
///
/// # Examples
///
/// ```
/// use lipgloss::position::{place_horizontal, RIGHT};
/// use lipgloss::whitespace::WhitespaceOption;
///
/// let result = place_horizontal(10, RIGHT, "Text", &[]);
/// // Text will be right-aligned with spaces on the left
/// ```
pub const RIGHT: Position = Position(1.0);
