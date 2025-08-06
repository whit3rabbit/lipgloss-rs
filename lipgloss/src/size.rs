//! String size measurement utilities for terminal display.
//!
//! This module provides functions for accurately measuring the display size of strings
//! in terminal cells. Unlike simple string length measurements, these functions properly
//! handle:
//!
//! - ANSI escape sequences (which are ignored in width calculations)
//! - Wide characters like CJK characters and emojis (which may occupy multiple cells)
//! - Multi-line strings
//!
//! # Why Use These Functions?
//!
//! Standard string length methods like `len()` or counting chars/graphemes won't give
//! accurate results for terminal display because:
//! - ANSI escape codes are counted but don't occupy display space
//! - Unicode characters can occupy 0, 1, or 2 terminal cells
//! - Different terminals may render characters differently
//!
//! # Examples
//!
//! ```
//! use lipgloss::size::{width, height, size};
//!
//! // ASCII text
//! assert_eq!(width("Hello"), 5);
//! assert_eq!(height("Hello"), 1);
//!
//! // Multi-line text
//! assert_eq!(width("Hello\nWorld!"), 6); // "World!" is longer
//! assert_eq!(height("Hello\nWorld!"), 2);
//!
//! // Wide characters (emoji)
//! let emoji_text = "Hello 👋";
//! assert!(width(emoji_text) > emoji_text.chars().count());
//! ```

use crate::utils::{height as str_height, width_visible as line_width};

/// Width returns the cell width of characters in the string. ANSI sequences are
/// ignored and characters wider than one cell (such as Chinese characters and
/// emojis) are appropriately measured.
///
/// You should use this instead of `s.len()` or counting runes, as neither will
/// give you accurate results in a terminal.
///
/// For multi-line strings, this returns the width of the widest line.
///
/// # Arguments
///
/// * `s` - The string to measure
///
/// # Returns
///
/// The maximum width in terminal cells of any line in the string.
///
/// # Examples
///
/// ```
/// use lipgloss::size::width;
///
/// // Simple ASCII text
/// assert_eq!(width("Hello, World!"), 13);
///
/// // ANSI escape sequences are ignored
/// let colored = "\x1b[31mRed Text\x1b[0m";
/// assert_eq!(width(colored), 8); // Only "Red Text" is counted
///
/// // Wide characters (CJK)
/// assert_eq!(width("你好"), 4); // Each character is 2 cells wide
///
/// // Emoji (width may vary by terminal)
/// let emoji = "👋 Hello";
/// assert!(width(emoji) >= 7); // Emoji typically 2 cells + space + "Hello"
///
/// // Multi-line strings return the widest line
/// assert_eq!(width("Short\nMuch longer line\nMid"), 16);
/// ```
pub fn width(s: &str) -> usize {
    let mut w = 0usize;
    for l in s.split('\n') {
        let lw = line_width(l);
        if lw > w {
            w = lw;
        }
    }
    w
}

/// Height returns height of a string in cells. This is done simply by counting
/// `\n` characters. If your strings use `\r\n` for newlines you should convert
/// them to `\n` first, or write a separate function for measuring height.
///
/// # Arguments
///
/// * `s` - The string to measure
///
/// # Returns
///
/// The number of lines in the string (minimum of 1).
///
/// # Examples
///
/// ```
/// use lipgloss::size::height;
///
/// // Single line
/// assert_eq!(height("Hello, World!"), 1);
///
/// // Empty string still has height 1
/// assert_eq!(height(""), 1);
///
/// // Multi-line string
/// assert_eq!(height("Line 1\nLine 2\nLine 3"), 3);
///
/// // Trailing newline adds an extra line
/// assert_eq!(height("Hello\n"), 2);
///
/// // ANSI codes don't affect height
/// assert_eq!(height("\x1b[31mRed\x1b[0m\nBlue"), 2);
/// ```
///
/// # Note
///
/// This function counts `\n` characters. If your text uses `\r\n` (Windows-style)
/// line endings, you should normalize them to `\n` first:
///
/// ```
/// use lipgloss::size::height;
///
/// let windows_text = "Line 1\r\nLine 2\r\n";
/// let normalized = windows_text.replace("\r\n", "\n");
/// assert_eq!(height(&normalized), 3);
/// ```
pub fn height(s: &str) -> usize {
    str_height(s)
}

/// Size returns the width and height of the string in cells. ANSI sequences are
/// ignored and characters wider than one cell (such as Chinese characters and
/// emojis) are appropriately measured.
///
/// This is a convenience function that combines [`width`] and [`height`].
///
/// # Arguments
///
/// * `s` - The string to measure
///
/// # Returns
///
/// A tuple of `(width, height)` where:
/// - `width` is the maximum line width in terminal cells
/// - `height` is the number of lines
///
/// # Examples
///
/// ```
/// use lipgloss::size::size;
///
/// // Single line
/// assert_eq!(size("Hello, World!"), (13, 1));
///
/// // Multi-line with different widths
/// let text = "Short\nThis is a longer line\nMedium";
/// assert_eq!(size(text), (21, 3));
///
/// // With ANSI codes and wide characters
/// let complex = "\x1b[1mBold\x1b[0m\n你好世界"; // "Hello World" in Chinese
/// let (w, h) = size(complex);
/// assert_eq!(h, 2);
/// assert!(w >= 8); // Chinese characters are wider
///
/// // Empty string
/// assert_eq!(size(""), (0, 1));
/// ```
pub fn size(s: &str) -> (usize, usize) {
    (width(s), height(s))
}
