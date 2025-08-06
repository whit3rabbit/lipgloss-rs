//! Rendering utility methods for Style
//!
//! This module provides internal utility methods for the [`Style`] struct that handle
//! various text processing and rendering operations. These utilities are designed to
//! work with terminal output and handle ANSI escape sequences, Unicode characters,
//! and text layout operations.
//!
//! ## Key Features
//!
//! - **Tab Conversion**: Convert tabs to spaces based on configurable tab width
//! - **Color Parsing**: Parse hexadecimal color strings with support for RGB and RGBA formats
//! - **Text Truncation**: Truncate text by visible width or height while preserving ANSI sequences
//! - **Text Wrapping**: Hard wrap text with ANSI sequence awareness
//! - **Tokenization**: Split text on configurable breakpoint characters
//!
//! ## ANSI Sequence Handling
//!
//! Most utilities in this module are ANSI-aware, meaning they properly handle ANSI
//! escape sequences (like color codes) when calculating text width, wrapping, or
//! truncating text. This ensures that styling information is preserved during
//! text processing operations.
//!
//! ## Unicode Support
//!
//! Text width calculations use the `unicode-width` crate to properly handle
//! wide characters (like CJK characters) and zero-width characters (like
//! combining marks and emoji modifiers).

use crate::security::{safe_repeat, MAX_ANSI_SEQ_LEN};
use crate::style::{properties::*, Style};

#[allow(dead_code)]
impl Style {
    /// Convert tabs to spaces if tab width is set
    ///
    /// This method converts all tab characters (`\t`) in the input string to spaces
    /// based on the style's configured tab width. If no tab width is set or the
    /// tab width is zero or negative, the string is returned unchanged.
    ///
    /// # Arguments
    ///
    /// * `s` - The input string that may contain tab characters
    ///
    /// # Returns
    ///
    /// A new string with tab characters replaced by the appropriate number of spaces,
    /// or the original string if tab width is not configured
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use lipgloss::Style;
    ///
    /// let style = Style::new().tab_width(4);
    /// let result = style.maybe_convert_tabs("hello\tworld");
    /// assert_eq!(result, "hello    world"); // Tab replaced with 4 spaces
    ///
    /// // Without tab width set, tabs are preserved
    /// let style = Style::new();
    /// let result = style.maybe_convert_tabs("hello\tworld");
    /// assert_eq!(result, "hello\tworld"); // Tab unchanged
    /// ```
    pub fn maybe_convert_tabs(&self, s: &str) -> String {
        if !self.is_set(TAB_WIDTH_KEY) || self.tab_width <= 0 {
            return s.to_string();
        }

        let tab_spaces = safe_repeat(' ', self.tab_width as usize);
        s.replace('\t', &tab_spaces)
    }

    /// Truncate text to visible width while preserving ANSI escape sequences
    ///
    /// Truncates a single line of text to fit within the specified maximum visible width.
    /// This function is ANSI-aware, meaning it preserves all ANSI escape sequences
    /// (such as color codes) while only counting the visible characters toward the width limit.
    /// Unicode characters are properly handled using their display width.
    ///
    /// # Arguments
    ///
    /// * `s` - The input string to truncate
    /// * `maxw` - Maximum visible width in characters
    ///
    /// # Returns
    ///
    /// A truncated string that fits within `maxw` visible characters, with all
    /// ANSI escape sequences preserved
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use lipgloss::Style;
    ///
    /// // Basic truncation
    /// let result = Style::truncate_visible_line("Hello World", 5);
    /// assert_eq!(result, "Hello");
    ///
    /// // ANSI sequences are preserved and don't count toward width
    /// let colored = "\x1b[31mHello\x1b[0m World";
    /// let result = Style::truncate_visible_line(colored, 8);
    /// assert_eq!(result, "\x1b[31mHello\x1b[0m Wo");
    ///
    /// // Wide characters (like CJK) are handled correctly
    /// let result = Style::truncate_visible_line("你好世界", 4); // Each character is width 2
    /// assert_eq!(result, "你好");
    ///
    /// // Zero width returns empty string
    /// let result = Style::truncate_visible_line("Hello", 0);
    /// assert_eq!(result, "");
    /// ```
    pub fn truncate_visible_line(s: &str, maxw: usize) -> String {
        if maxw == 0 {
            return String::new();
        }

        let mut result = String::new();
        let mut width = 0;
        let mut chars = s.chars().peekable();

        while let Some(ch) = chars.next() {
            if ch == '\x1b' {
                // Preserve ANSI escape sequence
                result.push(ch);
                // Read until we find a terminating byte or hit safety cap
                let mut scanned = 0usize;
                for esc_ch in chars.by_ref() {
                    result.push(esc_ch);
                    scanned += 1;
                    // Break on SGR 'm', or on any CSI final byte (@ through ~) excluding '[',
                    // or if we exceed our maximum safe sequence length
                    if esc_ch == 'm'
                        || (esc_ch != '[' && ('@'..='~').contains(&esc_ch))
                        || scanned >= MAX_ANSI_SEQ_LEN
                    {
                        break;
                    }
                }
                continue;
            }

            let char_width = unicode_width::UnicodeWidthChar::width(ch).unwrap_or(0);
            if width + char_width > maxw {
                break;
            }

            result.push(ch);
            width += char_width;
        }

        result
    }

    /// Truncate text to maximum height
    ///
    /// Truncates multi-line text to fit within the style's configured maximum height.
    /// If the input text has fewer lines than the maximum height, it is returned unchanged.
    /// Otherwise, only the first `max_height` lines are kept.
    ///
    /// # Arguments
    ///
    /// * `s` - The input string which may contain multiple lines separated by '\n'
    ///
    /// # Returns
    ///
    /// A string containing at most `max_height` lines from the beginning of the input
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use lipgloss::Style;
    ///
    /// let style = Style::new().max_height(2);
    /// let text = "Line 1\nLine 2\nLine 3\nLine 4";
    /// let result = style.truncate_height(text);
    /// assert_eq!(result, "Line 1\nLine 2");
    ///
    /// // If text has fewer lines than max_height, it's unchanged
    /// let style = Style::new().max_height(5);
    /// let text = "Line 1\nLine 2";
    /// let result = style.truncate_height(text);
    /// assert_eq!(result, "Line 1\nLine 2");
    /// ```
    pub fn truncate_height(&self, s: &str) -> String {
        let lines: Vec<&str> = s.split('\n').collect();
        if lines.len() <= self.max_height as usize {
            return s.to_string();
        }

        lines[0..self.max_height as usize].join("\n")
    }

    /// Truncate each line to maximum width while preserving ANSI sequences
    ///
    /// Truncates each line of multi-line text to fit within the style's configured
    /// maximum width. This method processes each line independently using
    /// [`truncate_visible_line`], ensuring that ANSI escape sequences are preserved
    /// and Unicode characters are handled correctly.
    ///
    /// # Arguments
    ///
    /// * `s` - The input string which may contain multiple lines separated by '\n'
    ///
    /// # Returns
    ///
    /// A string where each line has been truncated to fit within `max_width` visible
    /// characters, with all ANSI sequences preserved
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use lipgloss::Style;
    ///
    /// let style = Style::new().max_width(5);
    /// let text = "Hello World\nThis is a long line\nShort";
    /// let result = style.truncate_width(text);
    /// assert_eq!(result, "Hello\nThis \nShort");
    ///
    /// // ANSI sequences are preserved on each line
    /// let style = Style::new().max_width(8);
    /// let colored = "\x1b[31mRed text here\x1b[0m\n\x1b[32mGreen text\x1b[0m";
    /// let result = style.truncate_width(colored);
    /// // Each line truncated while preserving color codes
    /// ```
    ///
    /// [`truncate_visible_line`]: Style::truncate_visible_line
    pub fn truncate_width(&self, s: &str) -> String {
        let lines: Vec<&str> = s.split('\n').collect();
        let truncated: Vec<String> = lines
            .iter()
            .map(|line| Self::truncate_visible_line(line, self.max_width as usize))
            .collect();
        truncated.join("\n")
    }

    pub fn word_wrap_ansi_aware(text: &str, width: usize) -> Vec<String> {
        if width == 0 {
            return vec![String::new()];
        }

        let mut lines = Vec::new();
        let mut current_line = String::new();
        let mut current_width = 0;

        // Tokenize by spaces to handle words, preserving ANSI sequences
        let tokens = Self::tokenize_with_breakpoints(text, &[' ']);

        for token in tokens {
            let token_width = crate::width_visible(&token);

            if token == " " {
                // Handle spaces between words
                if current_width > 0 && current_width < width {
                    current_line.push(' ');
                    current_width += 1;
                }
                continue;
            }

            if current_width + token_width > width && current_width > 0 {
                // Current token doesn't fit, wrap to new line
                lines.push(current_line.clone());
                current_line.clear();
                current_width = 0;
            }

            if token_width > width {
                // Token is longer than line width, hard-wrap it
                if !current_line.is_empty() {
                    lines.push(current_line.clone());
                    current_line.clear();
                }

                let mut wrapped_token = Self::hard_wrap_ansi_aware(&token, width);
                if !wrapped_token.is_empty() {
                    // Add all but the last part of the hard-wrapped token to lines
                    lines.extend(wrapped_token.drain(..wrapped_token.len() - 1));
                    // The last part becomes the new current line
                    current_line = wrapped_token.pop().unwrap_or_default();
                    current_width = crate::width_visible(&current_line);
                }
            } else {
                // Token fits, add to current line
                current_line.push_str(&token);
                current_width += token_width;
            }
        }

        if !current_line.is_empty() {
            lines.push(current_line);
        }

        if lines.is_empty() {
            vec![String::new()]
        } else {
            lines
        }
    }

    ///
    /// # Notes
    ///
    /// This is a hard wrap function that breaks at character boundaries, not word
    /// boundaries. For word-aware wrapping, consider using a different approach.
    pub fn hard_wrap_ansi_aware(text: &str, width: usize) -> Vec<String> {
        if width == 0 {
            return vec![String::new()];
        }

        let mut lines = Vec::new();
        let mut current_line = String::new();
        let mut current_width = 0;
        let mut chars = text.chars().peekable();

        while let Some(ch) = chars.next() {
            if ch == '\x1b' {
                // Preserve ANSI escape sequence
                current_line.push(ch);
                // Read until we find a terminating byte or hit safety cap
                let mut scanned = 0usize;
                for esc_ch in chars.by_ref() {
                    current_line.push(esc_ch);
                    scanned += 1;
                    if esc_ch == 'm'
                        || (esc_ch != '[' && ('@'..='~').contains(&esc_ch))
                        || scanned >= MAX_ANSI_SEQ_LEN
                    {
                        break;
                    }
                }
                continue;
            }

            let char_width = unicode_width::UnicodeWidthChar::width(ch).unwrap_or(0);

            if current_width + char_width > width && current_width > 0 {
                // Wrap to new line
                lines.push(current_line);
                current_line = String::new();
                current_width = 0;
            }

            current_line.push(ch);
            current_width += char_width;
        }

        if !current_line.is_empty() {
            lines.push(current_line);
        }

        if lines.is_empty() {
            vec![String::new()]
        } else {
            lines
        }
    }

    /// Tokenize text with configurable breakpoint characters while preserving ANSI sequences
    ///
    /// Splits text into tokens using the specified breakpoint characters. ANSI escape
    /// sequences are preserved within tokens and do not cause tokenization. Each
    /// breakpoint character becomes its own token, and non-empty sequences between
    /// breakpoints become separate tokens.
    ///
    /// # Arguments
    ///
    /// * `s` - The input string to tokenize
    /// * `break_chars` - A slice of characters that should trigger tokenization
    ///
    /// # Returns
    ///
    /// A vector of string tokens, where each token is either:
    /// - A sequence of non-breakpoint characters (may include ANSI sequences)
    /// - A single breakpoint character
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use lipgloss::Style;
    ///
    /// // Basic tokenization on spaces and commas
    /// let result = Style::tokenize_with_breakpoints("hello, world test", &[' ', ',']);
    /// assert_eq!(result, vec!["hello", ",", " ", "world", " ", "test"]);
    ///
    /// // ANSI sequences are preserved within tokens
    /// let colored = "\x1b[31mred\x1b[0m text";
    /// let result = Style::tokenize_with_breakpoints(colored, &[' ']);
    /// assert_eq!(result, vec!["\x1b[31mred\x1b[0m", " ", "text"]);
    ///
    /// // Multiple consecutive breakpoints create separate tokens
    /// let result = Style::tokenize_with_breakpoints("a,,b", &[',']);
    /// assert_eq!(result, vec!["a", ",", ",", "b"]);
    ///
    /// // Empty input returns empty vector
    /// let result = Style::tokenize_with_breakpoints("", &[' ']);
    /// assert_eq!(result, Vec::<String>::new());
    /// ```
    ///
    /// # Notes
    ///
    /// This function is useful for implementing word wrapping or other text layout
    /// algorithms that need to respect certain character boundaries while preserving
    /// text styling.
    pub fn tokenize_with_breakpoints(s: &str, break_chars: &[char]) -> Vec<String> {
        let mut tokens: Vec<String> = Vec::new();
        let mut current = String::new();
        let mut chars = s.chars().peekable();

        while let Some(ch) = chars.next() {
            if ch == '\x1b' {
                // Preserve ANSI escape sequence
                current.push(ch);
                // Read until we find a terminating byte or hit safety cap
                let mut scanned = 0usize;
                for esc_ch in chars.by_ref() {
                    current.push(esc_ch);
                    scanned += 1;
                    if esc_ch == 'm'
                        || (esc_ch != '[' && ('@'..='~').contains(&esc_ch))
                        || scanned >= MAX_ANSI_SEQ_LEN
                    {
                        break;
                    }
                }
            } else if break_chars.contains(&ch) {
                // Breakpoint character - end current token and create new token for break char
                if !current.is_empty() {
                    tokens.push(current);
                    current = String::new();
                }
                tokens.push(ch.to_string());
            } else {
                current.push(ch);
            }
        }

        if !current.is_empty() {
            tokens.push(current);
        }

        tokens
    }
}
