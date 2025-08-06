//! Utility functions for text measurement, ANSI handling, and style ranges.
//!
//! This module provides essential utilities for working with styled text in terminal environments:
//!
//! - **Text Measurement**: Functions to calculate display width and height of text
//! - **ANSI Handling**: Utilities to strip ANSI escape sequences and calculate visible dimensions
//! - **Style Ranges**: Advanced styling capabilities for applying styles to specific character ranges
//! - **Go Compatibility**: API aliases that maintain compatibility with the original Go implementation
//!
//! # Key Features
//!
//! ## Unicode-Aware Text Measurement
//!
//! The width calculation functions properly handle:
//! - CJK (Chinese, Japanese, Korean) characters that occupy 2 terminal cells
//! - Zero-width characters and combining marks
//! - ANSI escape sequences that don't affect visible width
//!
//! ## ANSI Escape Sequence Handling
//!
//! Functions that work with styled terminal output:
//! - Strip ANSI codes while preserving text content
//! - Calculate visible dimensions ignoring color/style codes
//! - Process multi-line text with mixed styled and unstyled content
//!
//! ## Advanced Styling with Ranges
//!
//! Apply different styles to specific portions of text:
//! - Character-range based styling (similar to syntax highlighting)
//! - Index-based styling for search results or highlighting
//! - Overlapping range support with proper precedence
//!
//! # Examples
//!
//! ## Basic Text Measurement
//!
//! ```rust
//! use lipgloss::utils::{width, height, width_visible};
//!
//! // Unicode-aware width calculation
//! assert_eq!(width("Hello"), 5);
//! assert_eq!(width("こんにちは"), 10);  // CJK characters are 2 cells wide
//!
//! // Height calculation
//! assert_eq!(height("Hello\nWorld"), 2);
//!
//! // Visible width ignoring ANSI codes
//! let styled_text = "\x1b[31mRed Text\x1b[0m";
//! assert_eq!(width_visible(styled_text), 8);  // ANSI codes ignored
//! ```
//!
//! ## ANSI Processing
//!
//! ```rust  
//! use lipgloss::utils::{strip_ansi, get_lines_visible};
//!
//! // Strip ANSI escape sequences
//! let colored = "\x1b[31mHello\x1b[0m \x1b[32mWorld\x1b[0m";
//! assert_eq!(strip_ansi(colored), "Hello World");
//!
//! // Process multi-line styled text
//! let text = "\x1b[34mBlue Line\x1b[0m\n\x1b[31mRed Line\x1b[0m";
//! let (clean_lines, max_width) = get_lines_visible(text);
//! assert_eq!(clean_lines, vec!["Blue Line", "Red Line"]);
//! assert_eq!(max_width, 9);
//! ```
//!
//! ## Style Ranges
//!
//! ```rust
//! use lipgloss::{Style, Color, utils::{Range, style_ranges}};
//!
//! let bold_style = Style::new().bold(true);
//! let red_style = Style::new().foreground("red");
//!
//! let ranges = vec![
//!     Range::new(0, 5, bold_style),    // Make "Hello" bold
//!     Range::new(6, 11, red_style),    // Make "World" red
//! ];
//!
//! let result = style_ranges("Hello World", &ranges);
//! // Result contains "Hello" in bold and "World" in red
//! ```

use strip_ansi_escapes as ansi;
use unicode_width::UnicodeWidthStr;

/// Returns the display width of a string in terminal cells.
///
/// This function calculates how many terminal cells the string will occupy when displayed,
/// properly handling Unicode characters including:
/// - ASCII characters (1 cell each)
/// - CJK (Chinese, Japanese, Korean) characters (2 cells each)
/// - Zero-width characters and combining marks (0 cells)
/// - Other Unicode characters according to the Unicode Standard Annex #11
///
/// Note that this function does NOT strip ANSI escape sequences. For strings containing
/// ANSI codes, use [`width_visible`] instead.
///
/// # Arguments
///
/// * `s` - The string to measure
///
/// # Returns
///
/// The number of terminal cells the string occupies
///
/// # Examples
///
/// ```rust
/// use lipgloss::utils::width;
///
/// // ASCII characters are 1 cell each
/// assert_eq!(width("Hello"), 5);
/// assert_eq!(width(""), 0);
///
/// // CJK characters are 2 cells each
/// assert_eq!(width("中文"), 4);  // 2 characters × 2 cells = 4 cells
/// assert_eq!(width("こんにちは"), 10);  // 5 characters × 2 cells = 10 cells
///
/// // Mixed ASCII and CJK
/// assert_eq!(width("Hello世界"), 9);  // 5 + 4 = 9 cells
/// ```
pub fn width(s: &str) -> usize {
    UnicodeWidthStr::width(s)
}

// Go-style aliases for API name parity

/// Go-style alias for [`new_range`].
///
/// This function maintains compatibility with the original Go implementation
/// by providing the same PascalCase naming convention.
///
/// # Examples
///
/// ```rust
/// use lipgloss::{Style, Color, utils::NewRange};
///
/// let style = Style::new().foreground("red");
/// let range = NewRange(0, 5, style);
/// ```
#[allow(non_snake_case)]
pub fn NewRange(start: usize, end: usize, style: Style) -> Range {
    new_range(start, end, style)
}

/// Go-style alias for [`style_ranges`].
///
/// This function maintains compatibility with the original Go implementation
/// by providing the same PascalCase naming convention.
///
/// # Examples
///
/// ```rust
/// use lipgloss::{Style, Color, utils::{Range, StyleRanges}};
///
/// let bold_style = Style::new().bold(true);
/// let ranges = vec![Range::new(0, 5, bold_style)];
/// let result = StyleRanges("Hello World", &ranges);
/// ```
#[allow(non_snake_case)]
pub fn StyleRanges(s: &str, ranges: &[Range]) -> String {
    style_ranges(s, ranges)
}

/// Go-style alias for [`style_runes`].
///
/// This function maintains compatibility with the original Go implementation
/// by providing the same PascalCase naming convention.
///
/// # Examples
///
/// ```rust
/// use lipgloss::{Style, Color, utils::StyleRunes};
///
/// let highlight = Style::new().background("yellow");
/// let normal = Style::new();
/// let indices = vec![0, 2, 4]; // Highlight characters at positions 0, 2, 4
/// let result = StyleRunes("Hello", &indices, highlight, normal);
/// ```
#[allow(non_snake_case)]
pub fn StyleRunes(s: &str, indices: &[usize], matched: Style, unmatched: Style) -> String {
    style_runes(s, indices, matched, unmatched)
}

/// Strips ANSI escape sequences from a string, returning clean text.
///
/// This function removes all ANSI escape sequences (color codes, cursor movements,
/// text formatting, etc.) from the input string, leaving only the visible text content.
/// This is useful for:
/// - Calculating the actual text length for layout purposes
/// - Saving clean text to files
/// - Processing styled terminal output
///
/// # Arguments
///
/// * `s` - The string that may contain ANSI escape sequences
///
/// # Returns
///
/// A new `String` with all ANSI escape sequences removed
///
/// # Examples
///
/// ```rust
/// use lipgloss::utils::strip_ansi;
///
/// // Remove color codes
/// let colored = "\x1b[31mRed Text\x1b[0m";
/// assert_eq!(strip_ansi(colored), "Red Text");
///
/// // Remove multiple formatting codes
/// let formatted = "\x1b[1m\x1b[31mBold Red\x1b[0m Normal";
/// assert_eq!(strip_ansi(formatted), "Bold Red Normal");
///
/// // Plain text is unchanged
/// assert_eq!(strip_ansi("Plain text"), "Plain text");
/// ```
pub fn strip_ansi(s: &str) -> String {
    let bytes = ansi::strip(s.as_bytes());
    String::from_utf8_lossy(&bytes).into_owned()
}

/// Returns the visible display width of a string, ignoring ANSI escape sequences.
///
/// This function first strips all ANSI escape sequences from the string, then calculates
/// the display width of the remaining visible text. This is essential for layout
/// calculations when working with styled terminal output.
///
/// # Arguments
///
/// * `s` - The string to measure (may contain ANSI escape sequences)
///
/// # Returns
///
/// The number of terminal cells the visible text occupies
///
/// # Examples
///
/// ```rust
/// use lipgloss::utils::width_visible;
///
/// // ANSI color codes are ignored
/// let red_text = "\x1b[31mHello\x1b[0m";
/// assert_eq!(width_visible(red_text), 5);
///
/// // Multiple formatting codes are stripped
/// let complex = "\x1b[1m\x1b[31mBold Red\x1b[0m";
/// assert_eq!(width_visible(complex), 8);
///
/// // CJK characters still count as 2 cells after stripping
/// let styled_cjk = "\x1b[32m中文\x1b[0m";
/// assert_eq!(width_visible(styled_cjk), 4);
///
/// // Plain text works the same as width()
/// assert_eq!(width_visible("Plain"), 5);
/// ```
///
/// # See Also
///
/// - [`width`] - For measuring strings without ANSI codes
/// - [`strip_ansi`] - For removing ANSI codes without measuring
pub fn width_visible(s: &str) -> usize {
    let clean = strip_ansi(s);
    UnicodeWidthStr::width(clean.as_str())
}

/// Splits text into lines, strips ANSI codes, and returns the maximum visible width.
///
/// This function processes multi-line text by:
/// 1. Splitting the input into lines at newline characters
/// 2. Stripping ANSI escape sequences from each line
/// 3. Calculating the maximum visible width across all lines
///
/// This is particularly useful for layout calculations when working with styled
/// multi-line terminal output.
///
/// # Arguments
///
/// * `s` - The multi-line string to process (may contain ANSI escape sequences)
///
/// # Returns
///
/// A tuple containing:
/// - `Vec<String>`: Lines with ANSI codes stripped (owned strings)
/// - `usize`: Maximum visible width among all lines
///
/// # Examples
///
/// ```rust
/// use lipgloss::utils::get_lines_visible;
///
/// // Process styled multi-line text
/// let text = "\x1b[31mRed Line\x1b[0m\n\x1b[32mGreen Line\x1b[0m";
/// let (lines, max_width) = get_lines_visible(text);
/// assert_eq!(lines, vec!["Red Line", "Green Line"]);
/// assert_eq!(max_width, 10);  // "Green Line" is longest at 10 chars
///
/// // Handle empty lines
/// let text_with_empty = "Line 1\n\nLine 3";
/// let (lines, max_width) = get_lines_visible(text_with_empty);
/// assert_eq!(lines, vec!["Line 1", "", "Line 3"]);
/// assert_eq!(max_width, 6);
/// ```
///
/// # See Also
///
/// - [`get_lines`] - For processing without ANSI stripping
/// - [`height`] - For getting just the line count
pub fn get_lines_visible(s: &str) -> (Vec<String>, usize) {
    let mut lines = Vec::new();
    let mut maxw = 0usize;
    for raw in s.split('\n') {
        let clean = strip_ansi(raw);
        let w = UnicodeWidthStr::width(clean.as_str());
        if w > maxw {
            maxw = w;
        }
        lines.push(clean);
    }
    (lines, maxw)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_strip_ansi_and_visible_width() {
        let colored = "\x1b[31mHello\x1b[0m"; // red "Hello"
        assert_eq!(strip_ansi(colored), "Hello");
        assert_eq!(width_visible(colored), 5);

        // CJK wide rune still counts as width 2 after stripping (no ANSI here)
        let s = "中";
        assert_eq!(width_visible(s), 2);
    }

    #[test]
    fn test_get_lines_visible() {
        let s = "\x1b[34mBlue\x1b[0m\n\x1b[31mRed\x1b[0m";
        let (lines, maxw) = get_lines_visible(s);
        assert_eq!(lines, vec!["Blue", "Red"]);
        assert_eq!(maxw, 4);
        // height counting remains separate behavior
        assert_eq!(height(s), 2);
    }
}

/// Returns the number of lines in a string.
///
/// This function counts lines by counting newline characters and adding 1,
/// which means:
/// - Empty strings have height 1
/// - Strings without newlines have height 1
/// - Each newline character increases the height by 1
///
/// This matches the behavior of the original Go implementation.
///
/// # Arguments
///
/// * `s` - The string to measure
///
/// # Returns
///
/// The number of lines in the string
///
/// # Examples
///
/// ```rust
/// use lipgloss::utils::height;
///
/// // Single line strings
/// assert_eq!(height("Hello"), 1);
/// assert_eq!(height(""), 1);  // Empty string still has height 1
///
/// // Multi-line strings
/// assert_eq!(height("Line 1\nLine 2"), 2);
/// assert_eq!(height("A\nB\nC"), 3);
///
/// // Trailing newlines count
/// assert_eq!(height("Line\n"), 2);
/// assert_eq!(height("Line\n\n"), 3);
/// ```
pub fn height(s: &str) -> usize {
    // Go's Height counts lines as number of '\n' + 1, yielding 1 for empty.
    s.chars().filter(|&c| c == '\n').count() + 1
}

/// Splits text into lines and returns the maximum content width.
///
/// This function processes multi-line text by splitting it at newline characters
/// and calculating the maximum display width across all lines. Unlike [`get_lines_visible`],
/// this function does NOT strip ANSI escape sequences, so styled text may affect
/// the width calculations.
///
/// # Arguments
///
/// * `s` - The multi-line string to process
///
/// # Returns
///
/// A tuple containing:
/// - `Vec<&str>`: Line references (borrowed from the input string)
/// - `usize`: Maximum display width among all lines
///
/// # Examples
///
/// ```rust
/// use lipgloss::utils::get_lines;
///
/// // Process plain multi-line text
/// let text = "Short\nA longer line\nMed";
/// let (lines, max_width) = get_lines(text);
/// assert_eq!(lines, vec!["Short", "A longer line", "Med"]);
/// assert_eq!(max_width, 13);  // "A longer line" is 13 characters
///
/// // Single line
/// let single = "Just one line";
/// let (lines, max_width) = get_lines(single);
/// assert_eq!(lines, vec!["Just one line"]);
/// assert_eq!(max_width, 13);
/// ```
///
/// # Note
///
/// This function does not strip ANSI escape sequences. For styled text,
/// consider using [`get_lines_visible`] instead to get accurate visible widths.
///
/// # See Also
///
/// - [`get_lines_visible`] - For processing text with ANSI codes stripped
/// - [`height`] - For getting just the line count
pub fn get_lines(s: &str) -> (Vec<&str>, usize) {
    let lines: Vec<&str> = s.split('\n').collect();
    let mut maxw = 0usize;
    for l in &lines {
        let w = width(l);
        if w > maxw {
            maxw = w;
        }
    }
    (lines, maxw)
}

// -----------------------------
// Style range helpers (Go parity)
// -----------------------------
use crate::color::TerminalColor;
use crate::style::Style;

/// Represents a character range with an associated style for selective text styling.
///
/// A `Range` specifies a half-open interval [start, end) of character positions
/// and the style to apply to that text segment. This enables sophisticated text
/// styling like syntax highlighting, search result highlighting, or selective formatting.
///
/// # Character Indexing
///
/// Indices are based on Unicode scalar values (Rust `char`), which closely matches
/// the rune-based indexing of the original Go implementation. This means:
/// - ASCII characters count as 1 index each
/// - Multi-byte Unicode characters (including CJK) count as 1 index each
/// - Emoji and other complex characters count as 1 index each
///
/// # Range Behavior
///
/// - **Half-open**: Range [2, 5) includes characters at positions 2, 3, 4 (not 5)
/// - **Clamping**: Out-of-bounds indices are automatically clamped to valid range
/// - **Invalid ranges**: Empty ranges (start >= end) are ignored
/// - **Overlapping**: Later ranges override earlier ones in the same positions
///
/// # Examples
///
/// ```rust
/// use lipgloss::{Style, Color, utils::Range};
///
/// // Create a range to make characters 0-5 bold
/// let bold_style = Style::new().bold(true);
/// let range = Range::new(0, 5, bold_style);
///
/// // Multiple ranges for different styling
/// let red_style = Style::new().foreground("red");
/// let blue_style = Style::new().foreground("blue");
///
/// let ranges = vec![
///     Range::new(0, 5, red_style),    // "Hello" in red
///     Range::new(6, 11, blue_style),  // "World" in blue
/// ];
///
/// // Ranges work with Unicode text
/// let text = "Hello 世界";
/// let unicode_range = Range::new(6, 8, Style::new().bold(true));  // Style the CJK characters
/// ```
#[derive(Clone, Debug)]
pub struct Range {
    /// The starting character index (inclusive)
    pub start: usize,
    /// The ending character index (exclusive)
    pub end: usize,
    /// The style to apply to this range
    pub style: Style,
}

impl Range {
    /// Creates a new `Range` with the specified character positions and style.
    ///
    /// # Arguments
    ///
    /// * `start` - The starting character index (inclusive)
    /// * `end` - The ending character index (exclusive)
    /// * `style` - The style to apply to characters in this range
    ///
    /// # Returns
    ///
    /// A new `Range` instance
    ///
    /// # Examples
    ///
    /// ```rust
    /// use lipgloss::{Style, Color, utils::Range};
    ///
    /// let style = Style::new().foreground("red").bold(true);
    /// let range = Range::new(0, 5, style);
    ///
    /// assert_eq!(range.start, 0);
    /// assert_eq!(range.end, 5);
    /// ```
    pub fn new(start: usize, end: usize, style: Style) -> Self {
        Self { start, end, style }
    }
}

/// Creates a new [`Range`] (convenience function).
///
/// This is a convenience constructor that mirrors the Go implementation's
/// `NewRange` function, providing a functional-style API.
///
/// # Arguments
///
/// * `start` - The starting character index (inclusive)
/// * `end` - The ending character index (exclusive)  
/// * `style` - The style to apply to characters in this range
///
/// # Returns
///
/// A new `Range` instance
///
/// # Examples
///
/// ```rust
/// use lipgloss::{Style, Color, utils::new_range};
///
/// let highlight = Style::new().background("yellow");
/// let range = new_range(10, 15, highlight);
/// ```
///
/// # See Also
///
/// - [`Range::new`] - The struct method equivalent
/// - [`NewRange`] - Go-style PascalCase alias
pub fn new_range(start: usize, end: usize, style: Style) -> Range {
    Range::new(start, end, style)
}

/// Applies styles to specific character ranges within a string.
///
/// This function enables sophisticated text styling by applying different styles
/// to different portions of the same string. It's particularly useful for:
/// - Syntax highlighting in code editors
/// - Search result highlighting
/// - Selective text formatting
/// - Creating rich terminal output
///
/// # Behavior
///
/// - **Character-based**: Ranges use Unicode scalar value (char) indexing
/// - **Half-open intervals**: Range [2, 5) includes characters 2, 3, 4 (not 5)
/// - **Overlap handling**: Later ranges override earlier ones at the same positions
/// - **Out-of-bounds**: Indices are automatically clamped to valid range
/// - **Invalid ranges**: Empty or inverted ranges (start >= end) are ignored
/// - **Grouping optimization**: Consecutive characters with identical styling are grouped
///
/// # Arguments
///
/// * `s` - The input string to style
/// * `ranges` - A slice of [`Range`] objects specifying character ranges and their styles
///
/// # Returns
///
/// A new `String` with the specified styles applied via ANSI escape sequences
///
/// # Examples
///
/// ## Basic Range Styling
///
/// ```rust
/// use lipgloss::{Style, Color, utils::{Range, style_ranges}};
///
/// let text = "Hello World";
/// let bold = Style::new().bold(true);
/// let red = Style::new().foreground("red");
///
/// let ranges = vec![
///     Range::new(0, 5, bold),  // "Hello" in bold
///     Range::new(6, 11, red),  // "World" in red
/// ];
///
/// let styled = style_ranges(text, &ranges);
/// // Result: "\x1b[1mHello\x1b[0m \x1b[31mWorld\x1b[0m"
/// ```
///
/// ## Overlapping Ranges
///
/// ```rust
/// use lipgloss::{Style, Color, utils::{Range, style_ranges}};
///
/// let text = "Hello";
/// let bold = Style::new().bold(true);
/// let red = Style::new().foreground("red");
///
/// let ranges = vec![
///     Range::new(0, 5, bold),  // Entire word bold
///     Range::new(2, 4, red),   // "ll" also red (overrides bold)
/// ];
///
/// let styled = style_ranges(text, &ranges);
/// // "He" is bold, "ll" is red, "o" is bold
/// ```
///
/// ## Unicode Support
///
/// ```rust
/// use lipgloss::{Style, Color, utils::{Range, style_ranges}};
///
/// let text = "Hello 世界!";  // "Hello World!" with CJK characters
/// let highlight = Style::new().background("yellow");
///
/// let ranges = vec![
///     Range::new(6, 8, highlight),  // Highlight the CJK characters
/// ];
///
/// let styled = style_ranges(text, &ranges);
/// ```
///
/// # Performance Notes
///
/// The function groups consecutive characters with identical styles to minimize
/// ANSI escape sequences in the output, making it efficient for large texts
/// with many small ranges.
///
/// # See Also
///
/// - [`style_runes`] - For index-based styling (highlight specific characters)
/// - [`Range`] - The range specification struct
/// - [`StyleRanges`] - Go-style PascalCase alias
pub fn style_ranges(s: &str, ranges: &[Range]) -> String {
    if s.is_empty() {
        return String::new();
    }

    let chars: Vec<char> = s.chars().collect();
    let n = chars.len();

    if ranges.is_empty() || n == 0 {
        return s.to_string();
    }

    // For each index, record the last style that applies (later ranges override earlier ones)
    let mut map: Vec<Option<Style>> = vec![None; n];
    for r in ranges {
        if r.start >= r.end {
            continue;
        }
        let start = r.start.min(n);
        let end = r.end.min(n);
        if start >= end {
            continue;
        }
        for slot in &mut map[start..end] {
            *slot = Some(r.style.clone());
        }
    }

    // Build output by grouping consecutive indices with the same style option
    let mut out = String::new();
    let mut i = 0usize;
    while i < n {
        let _current = map[i].as_ref();
        let mut j = i + 1;
        // Note: The Go version uses pointer equality, but since we clone styles in Rust,
        // we need to use value equality instead. This is handled in the fallback below.
        // Fallback: advance while the Option discriminant and substantive style equality match
        while j < n {
            let eq = match (&map[i], &map[j]) {
                (None, None) => true,
                (Some(a), Some(b)) => {
                    // Use efficient field-based comparison instead of string rendering
                    a.is_equivalent(b)
                }
                _ => false,
            };
            if !eq {
                break;
            }
            j += 1;
        }

        let segment: String = chars[i..j].iter().collect();
        match &map[i] {
            Some(st) => out.push_str(&st.apply(&segment)),
            None => out.push_str(&segment),
        }
        i = j;
    }

    out
}

/// Applies different styles to specific character indices versus the rest of the text.
///
/// This function is ideal for highlighting specific characters while styling the
/// remainder differently. Common use cases include:
/// - Search result highlighting (highlight matching characters)
/// - Spell checking (highlight misspelled characters)
/// - Pattern matching visualization
/// - Interactive text selection
///
/// # Behavior
///
/// - **Character-based**: Indices refer to Unicode scalar values (char positions)
/// - **Duplicate handling**: Duplicate indices are automatically ignored
/// - **Out-of-bounds**: Invalid indices are silently ignored
/// - **Binary styling**: Each character gets either `matched` or `unmatched` style
///
/// # Arguments
///
/// * `s` - The input string to style
/// * `indices` - Character positions to apply the `matched` style to
/// * `matched` - Style for characters at the specified indices
/// * `unmatched` - Style for all other characters
///
/// # Returns
///
/// A new `String` with styles applied via ANSI escape sequences
///
/// # Examples
///
/// ## Search Highlighting
///
/// ```rust
/// use lipgloss::{Style, Color, utils::style_runes};
///
/// let text = "programming";
/// let highlight = Style::new().background("yellow");
/// let normal = Style::new();
///
/// // Highlight 'r' characters (at positions 1 and 8)
/// let indices = vec![1, 8];
/// let result = style_runes(text, &indices, highlight, normal);
/// // 'p' normal, 'r' highlighted, 'ogrammin' normal, 'g' highlighted
/// ```
///
/// ## Pattern Matching
///
/// ```rust
/// use lipgloss::{Style, Color, utils::style_runes};
///
/// let text = "Hello World";
/// let matched = Style::new().foreground("red").bold(true);
/// let unmatched = Style::new().faint(true);
///
/// // Highlight vowels: H(e)ll(o) W(o)rld -> positions 1, 4, 7
/// let vowel_positions = vec![1, 4, 7];
/// let styled = style_runes(text, &vowel_positions, matched, unmatched);
/// // Vowels are bold red, consonants are faint
/// ```
///
/// ## Unicode Text
///
/// ```rust
/// use lipgloss::{Style, Color, utils::style_runes};
///
/// let text = "Hello 世界!";
/// let highlight = Style::new().background("cyan");
/// let normal = Style::new();
///
/// // Highlight the CJK characters at positions 6 and 7
/// let indices = vec![6, 7];
/// let result = style_runes(text, &indices, highlight, normal);
/// ```
///
/// # Performance Notes
///
/// This function processes the string character by character, making it efficient
/// for sparse highlighting (few highlighted characters) but less optimal than
/// [`style_ranges`] for contiguous styled regions.
///
/// # See Also
///
/// - [`style_ranges`] - For range-based styling (more efficient for contiguous regions)
/// - [`StyleRunes`] - Go-style PascalCase alias
pub fn style_runes(s: &str, indices: &[usize], matched: Style, unmatched: Style) -> String {
    if s.is_empty() {
        return String::new();
    }

    use std::collections::HashSet;
    let idx_set: HashSet<usize> = indices.iter().copied().collect();
    let mut out = String::new();
    for (i, ch) in s.chars().enumerate() {
        let seg = ch.to_string();
        if idx_set.contains(&i) {
            out.push_str(&matched.apply(&seg));
        } else {
            out.push_str(&unmatched.apply(&seg));
        }
    }
    out
}

// -----------------------------
// CSS-style shorthand helpers (Go parity)
// -----------------------------

/// Helper for CSS-style shorthand integer values (padding, margin).
///
/// This function implements CSS-style shorthand rules for specifying values
/// for the four sides of a box (top, right, bottom, left). It follows the
/// same rules as CSS shorthand properties:
///
/// - 1 value: all sides use the same value
/// - 2 values: top/bottom use first, left/right use second  
/// - 3 values: top uses first, left/right use second, bottom uses third
/// - 4 values: top, right, bottom, left (clockwise from top)
/// - 5+ values: invalid, returns `ok = false`
///
/// # Arguments
///
/// * `values` - Slice of integer values following CSS shorthand rules
///
/// # Returns
///
/// A tuple containing `(top, right, bottom, left, ok)` where:
/// - `top`, `right`, `bottom`, `left` are the resolved values for each side
/// - `ok` is `true` if the input was valid, `false` otherwise
///
/// # Examples
///
/// ```rust
/// use lipgloss::utils::which_sides_int;
///
/// // 1 value: all sides = 10
/// let (t, r, b, l, ok) = which_sides_int(&[10]);
/// assert!(ok && t == 10 && r == 10 && b == 10 && l == 10);
///
/// // 2 values: vertical = 5, horizontal = 10
/// let (t, r, b, l, ok) = which_sides_int(&[5, 10]);
/// assert!(ok && t == 5 && r == 10 && b == 5 && l == 10);
///
/// // 4 values: clockwise from top
/// let (t, r, b, l, ok) = which_sides_int(&[1, 2, 3, 4]);
/// assert!(ok && t == 1 && r == 2 && b == 3 && l == 4);
///
/// // Invalid: too many values
/// let (_, _, _, _, ok) = which_sides_int(&[1, 2, 3, 4, 5]);
/// assert!(!ok);
/// ```
pub fn which_sides_int(values: &[i32]) -> (i32, i32, i32, i32, bool) {
    match values.len() {
        1 => {
            let val = values[0];
            (val, val, val, val, true)
        }
        2 => {
            let vertical = values[0];
            let horizontal = values[1];
            (vertical, horizontal, vertical, horizontal, true)
        }
        3 => {
            let top = values[0];
            let horizontal = values[1];
            let bottom = values[2];
            (top, horizontal, bottom, horizontal, true)
        }
        4 => {
            let top = values[0];
            let right = values[1];
            let bottom = values[2];
            let left = values[3];
            (top, right, bottom, left, true)
        }
        _ => {
            // Invalid: 0 values or more than 4 values
            (0, 0, 0, 0, false)
        }
    }
}

/// Helper for CSS-style shorthand boolean values (border sides).
///
/// This function implements CSS-style shorthand rules for specifying boolean
/// values for the four sides of a box. It follows the same pattern as
/// [`which_sides_int`] but operates on boolean values.
///
/// # Arguments
///
/// * `values` - Slice of boolean values following CSS shorthand rules
///
/// # Returns
///
/// A tuple containing `(top, right, bottom, left, ok)` where:
/// - `top`, `right`, `bottom`, `left` are the resolved boolean values for each side
/// - `ok` is `true` if the input was valid, `false` otherwise
///
/// # Examples
///
/// ```rust
/// use lipgloss::utils::which_sides_bool;
///
/// // 1 value: all sides enabled
/// let (t, r, b, l, ok) = which_sides_bool(&[true]);
/// assert!(ok && t && r && b && l);
///
/// // 2 values: top/bottom enabled, left/right disabled
/// let (t, r, b, l, ok) = which_sides_bool(&[true, false]);
/// assert!(ok && t && !r && b && !l);
///
/// // Invalid: too many values
/// let (_, _, _, _, ok) = which_sides_bool(&[true, false, true, false, true]);
/// assert!(!ok);
/// ```
pub fn which_sides_bool(values: &[bool]) -> (bool, bool, bool, bool, bool) {
    match values.len() {
        1 => {
            let val = values[0];
            (val, val, val, val, true)
        }
        2 => {
            let vertical = values[0];
            let horizontal = values[1];
            (vertical, horizontal, vertical, horizontal, true)
        }
        3 => {
            let top = values[0];
            let horizontal = values[1];
            let bottom = values[2];
            (top, horizontal, bottom, horizontal, true)
        }
        4 => {
            let top = values[0];
            let right = values[1];
            let bottom = values[2];
            let left = values[3];
            (top, right, bottom, left, true)
        }
        _ => {
            // Invalid: 0 values or more than 4 values
            (false, false, false, false, false)
        }
    }
}

/// Helper for CSS-style shorthand color values (border colors).
///
/// This function implements CSS-style shorthand rules for specifying colors
/// for the four sides of a box. It follows the same pattern as [`which_sides_int`]
/// but operates on colors that implement [`TerminalColor`].
///
/// # Arguments
///
/// * `values` - Slice of color values following CSS shorthand rules
///
/// # Returns
///
/// A tuple containing `(top, right, bottom, left, ok)` where:
/// - `top`, `right`, `bottom`, `left` are the resolved colors for each side
/// - `ok` is `true` if the input was valid, `false` otherwise
///
/// # Examples
///
/// ```rust
/// use lipgloss::utils::which_sides_color;
///
/// // 1 value: all sides use same color
/// let colors = ["red"];
/// let (t, r, b, l, ok) = which_sides_color(&colors);
/// assert!(ok);
///
/// // 4 values: clockwise from top
/// let colors = ["red", "green", "blue", "yellow"];
/// let (t, r, b, l, ok) = which_sides_color(&colors);
/// assert!(ok);
/// ```
pub fn which_sides_color<C: TerminalColor + Clone>(values: &[C]) -> (C, C, C, C, bool) {
    match values.len() {
        1 => {
            let val = values[0].clone();
            (val.clone(), val.clone(), val.clone(), val, true)
        }
        2 => {
            let vertical = values[0].clone();
            let horizontal = values[1].clone();
            (
                vertical.clone(),
                horizontal.clone(),
                vertical,
                horizontal,
                true,
            )
        }
        3 => {
            let top = values[0].clone();
            let horizontal = values[1].clone();
            let bottom = values[2].clone();
            (top, horizontal.clone(), bottom, horizontal, true)
        }
        4 => {
            let top = values[0].clone();
            let right = values[1].clone();
            let bottom = values[2].clone();
            let left = values[3].clone();
            (top, right, bottom, left, true)
        }
        _ => {
            // Invalid: 0 values or more than 4 values
            // We need to return some default, but we can't create a default TerminalColor
            // So we'll use a different approach - return the first element or panic
            if values.is_empty() {
                // For empty input, we can't provide a meaningful default
                // The caller should check `ok` before using the values
                panic!("Cannot provide default color for empty input")
            } else {
                // For too many values, return the first 4 (or repeat first as needed)
                let default = values[0].clone();
                (
                    default.clone(),
                    default.clone(),
                    default.clone(),
                    default,
                    false,
                )
            }
        }
    }
}
