//! Text joining utilities for combining multiple strings with alignment.
//!
//! This module provides functions to join multiple text blocks either horizontally
//! or vertically, with precise control over alignment. It handles multi-line strings
//! properly, preserving ANSI escape sequences and ensuring proper padding for
//! visual alignment in terminal interfaces.
//!
//! # Examples
//!
//! ```
//! use lipgloss::join::{join_horizontal, join_vertical};
//! use lipgloss::position::{CENTER, TOP, LEFT};
//!
//! // Join two blocks horizontally with top alignment
//! let block1 = "Hello\nWorld";
//! let block2 = "Rust\nis\nawesome";
//! let joined = join_horizontal(TOP, &[block1, block2]);
//!
//! // Join blocks vertically with left alignment
//! let header = "Title";
//! let content = "Content goes here";
//! let footer = "Footer";
//! let page = join_vertical(LEFT, &[header, content, footer]);
//! ```

use crate::position::Position;
use crate::security::safe_repeat;
use crate::utils::width_visible as line_width;

/// Joins multiple strings horizontally with vertical alignment control.
///
/// This function takes multiple text blocks (which may contain newlines) and
/// arranges them side-by-side. The vertical alignment of blocks with different
/// heights is controlled by the `pos` parameter. Shorter blocks are padded with
/// empty lines to match the height of the tallest block.
///
/// The function preserves ANSI escape sequences and calculates visible width
/// correctly for proper alignment, ensuring that styled text displays correctly.
///
/// # Arguments
///
/// * `pos` - Vertical alignment position (0.0 = top, 0.5 = center, 1.0 = bottom)
/// * `strs` - Slice of strings to join horizontally
///
/// # Returns
///
/// A single string with all input strings arranged horizontally
///
/// # Examples
///
/// ```
/// use lipgloss::join::join_horizontal;
/// use lipgloss::position::{TOP, CENTER, BOTTOM};
///
/// // Top-aligned blocks
/// let left = "Line 1\nLine 2";
/// let right = "A\nB\nC\nD";
/// let result = join_horizontal(TOP, &[left, right]);
/// // Result:
/// // Line 1A
/// // Line 2B
/// //       C
/// //       D
///
/// // Center-aligned blocks
/// let result = join_horizontal(CENTER, &[left, right]);
/// // Result:
/// //       A
/// // Line 1B
/// // Line 2C
/// //       D
///
/// // Bottom-aligned blocks
/// let result = join_horizontal(BOTTOM, &[left, right]);
/// // Result:
/// //       A
/// //       B
/// // Line 1C
/// // Line 2D
/// ```
pub fn join_horizontal(pos: Position, strs: &[&str]) -> String {
    if strs.is_empty() {
        return String::new();
    }
    if strs.len() == 1 {
        return strs[0].to_string();
    }

    // Break into lines (preserve ANSI) and track max widths and max height.
    let mut blocks: Vec<Vec<String>> = Vec::with_capacity(strs.len());
    let mut max_widths: Vec<usize> = Vec::with_capacity(strs.len());
    let mut max_height: usize = 0;

    for s in strs {
        let lines: Vec<String> = s.split('\n').map(|l| l.to_string()).collect();
        if lines.len() > max_height {
            max_height = lines.len();
        }
        let w = lines.iter().map(|l| line_width(l)).max().unwrap_or(0);
        blocks.push(lines);
        max_widths.push(w);
    }

    // Pad blocks to equal height according to pos
    let v = pos.value();
    for b in &mut blocks {
        if b.len() >= max_height {
            continue;
        }
        let need = max_height - b.len();
        let mut extra = vec![String::new(); need];
        if (v - 0.0).abs() < f64::EPSILON {
            // Top: add extra lines at bottom
            b.extend(extra);
        } else if (v - 1.0).abs() < f64::EPSILON {
            // Bottom: add at top
            extra.append(b);
            *b = extra;
        } else {
            // Middle: split using Go parity: prepend extraLines[top:], append extraLines[bottom:]
            let split = (need as f64 * v).round() as usize;
            let top = need - split; // number used to compute slice start
            let bottom = need - top; // number used to compute slice start

            // Prepend (need - top) empties, append (need - bottom) empties
            let prepend = need - top;
            let append = need - bottom;

            let mut newv: Vec<String> = Vec::with_capacity(max_height);
            newv.extend(std::iter::repeat_n(String::new(), prepend));
            newv.append(b);
            newv.extend(std::iter::repeat_n(String::new(), append));
            *b = newv;
        }
    }

    // Merge line-by-line, padding each line of each block to its own max width
    let mut out = String::new();
    for i in 0..max_height {
        for (j, block) in blocks.iter().enumerate() {
            let line = if i < block.len() {
                block[i].as_str()
            } else {
                ""
            };
            out.push_str(line);

            let pad = max_widths[j].saturating_sub(line_width(line));
            if pad > 0 {
                out.push_str(&safe_repeat(' ', pad));
            }
        }
        if i < max_height - 1 {
            out.push('\n');
        }
    }

    out
}

/// Joins multiple strings vertically with horizontal alignment control.
///
/// This function takes multiple text blocks (which may contain newlines) and
/// stacks them vertically. Each line in each block is aligned horizontally
/// according to the `pos` parameter. All lines are padded to the width of
/// the widest line across all blocks.
///
/// The function preserves ANSI escape sequences and calculates visible width
/// correctly for proper alignment, ensuring that styled text displays correctly.
///
/// # Arguments
///
/// * `pos` - Horizontal alignment position (0.0 = left, 0.5 = center, 1.0 = right)
/// * `strs` - Slice of strings to join vertically
///
/// # Returns
///
/// A single string with all input strings stacked vertically
///
/// # Examples
///
/// ```
/// use lipgloss::join::join_vertical;
/// use lipgloss::position::{LEFT, CENTER, RIGHT};
///
/// // Left-aligned blocks
/// let header = "Header";
/// let body = "This is the body";
/// let footer = "Footer";
/// let result = join_vertical(LEFT, &[header, body, footer]);
/// // Result:
/// // Header          
/// // This is the body
/// // Footer          
///
/// // Center-aligned blocks
/// let result = join_vertical(CENTER, &[header, body, footer]);
/// // Result:
/// //      Header     
/// // This is the body
/// //      Footer     
///
/// // Right-aligned blocks
/// let result = join_vertical(RIGHT, &[header, body, footer]);
/// // Result:
/// //           Header
/// // This is the body
/// //           Footer
///
/// // Multi-line blocks are handled line by line
/// let block1 = "Short\nA bit longer";
/// let block2 = "Medium length";
/// let result = join_vertical(CENTER, &[block1, block2]);
/// ```
pub fn join_vertical(pos: Position, strs: &[&str]) -> String {
    if strs.is_empty() {
        return String::new();
    }
    if strs.len() == 1 {
        return strs[0].to_string();
    }

    // Break into lines (preserve ANSI) and track the maximum width across all blocks
    let mut blocks: Vec<Vec<String>> = Vec::with_capacity(strs.len());
    let mut max_width: usize = 0;
    for s in strs {
        let lines: Vec<String> = s.split('\n').map(|l| l.to_string()).collect();
        let w = lines.iter().map(|l| line_width(l)).max().unwrap_or(0);
        if w > max_width {
            max_width = w;
        }
        blocks.push(lines);
    }

    let v = pos.value();
    let mut out = String::new();
    for (bi, block) in blocks.iter().enumerate() {
        for (li, line) in block.iter().enumerate() {
            let line = line.as_str();
            let w = max_width.saturating_sub(line_width(line));
            if (v - 0.0).abs() < f64::EPSILON {
                // Left
                out.push_str(line);
                if w > 0 {
                    out.push_str(&safe_repeat(' ', w));
                }
            } else if (v - 1.0).abs() < f64::EPSILON {
                // Right
                if w > 0 {
                    out.push_str(&safe_repeat(' ', w));
                }
                out.push_str(line);
            } else {
                // Middle
                if w < 1 {
                    out.push_str(line);
                } else {
                    let split = (w as f64 * v).round() as usize;
                    let right = w - split;
                    let left = w - right;
                    if left > 0 {
                        out.push_str(&safe_repeat(' ', left));
                    }
                    out.push_str(line);
                    if right > 0 {
                        out.push_str(&safe_repeat(' ', right));
                    }
                }
            }

            // newline unless this is the very last line of the last block
            if !(bi == blocks.len() - 1 && li == block.len() - 1) {
                out.push('\n');
            }
        }
    }

    out
}
