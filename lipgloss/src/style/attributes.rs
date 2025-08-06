//! Text attribute setter methods for the Style struct.
//!
//! This module provides all the methods for setting visual text attributes on a [`Style`].
//! These attributes control how text appears in the terminal, including font weight,
//! decoration, and special visual effects.
//!
//! All methods in this module follow the builder pattern, taking `self` by value and
//! returning `Self` to enable method chaining. Each method sets the corresponding
//! attribute and marks the property as explicitly set.
//!
//! # Available Text Attributes
//!
//! - **Font Weight**: `bold()`, `faint()`
//! - **Font Style**: `italic()`
//! - **Text Decoration**: `underline()`, `strikethrough()`, `blink()`
//! - **Visual Effects**: `reverse()` (swap foreground/background)
//! - **Spacing Control**: `underline_spaces()`, `strikethrough_spaces()`, `color_whitespace()`
//! - **Layout**: `inline()` (render without line breaks)
//!
//! # Examples
//!
//! ## Basic Text Styling
//!
//! ```rust,no_run
//! use lipgloss::Style;
//!
//! let style = Style::new()
//!     .bold(true)
//!     .italic(true)
//!     .underline(true);
//!
//! let text = style.render("Important message");
//! println!("{}", text);
//! ```
//!
//! ## Chaining Multiple Attributes
//!
//! ```rust,no_run
//! use lipgloss::Style;
//!
//! let warning_style = Style::new()
//!     .bold(true)
//!     .blink(true)
//!     .reverse(true);
//!
//! println!("{}", warning_style.render("⚠ WARNING ⚠"));
//! ```
//!
//! ## Space and Whitespace Handling
//!
//! ```rust,no_run
//! use lipgloss::Style;
//!
//! let underlined_style = Style::new()
//!     .underline(true)
//!     .underline_spaces(true)  // Underline spaces too
//!     .color_whitespace(true); // Apply colors to whitespace
//!
//! println!("{}", underlined_style.render("Text with spaces"));
//! ```

use crate::style::{properties::*, Style};

impl Style {
    /// Set whether to render text inline without adding line breaks.
    ///
    /// When `true`, the text will be rendered without automatic line breaks,
    /// useful for creating continuous text flows or inline elements.
    /// When `false` (default), normal line breaking behavior applies.
    ///
    /// # Arguments
    ///
    /// * `v` - Whether to render inline (`true`) or with normal line breaks (`false`)
    ///
    /// # Returns
    ///
    /// Returns `Self` for method chaining.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use lipgloss::Style;
    ///
    /// let inline_style = Style::new().inline(true);
    /// let block_style = Style::new().inline(false);
    ///
    /// // Inline text flows continuously
    /// let inline_text = inline_style.render("This is inline");
    ///
    /// // Block text allows line breaks
    /// let block_text = block_style.render("This can break\ninto multiple lines");
    /// ```
    pub fn inline(mut self, v: bool) -> Self {
        self.set_attr(ATTR_INLINE, v);
        self.set_prop(INLINE_KEY);
        self
    }

    /// Set whether to render text with bold font weight.
    ///
    /// When `true`, text appears with increased font weight (bold).
    /// When `false`, text uses normal font weight.
    ///
    /// # Arguments
    ///
    /// * `v` - Whether to apply bold styling (`true`) or normal weight (`false`)
    ///
    /// # Returns
    ///
    /// Returns `Self` for method chaining.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use lipgloss::Style;
    ///
    /// let bold_style = Style::new().bold(true);
    /// let normal_style = Style::new().bold(false);
    ///
    /// println!("{}", bold_style.render("Bold text"));
    /// println!("{}", normal_style.render("Normal text"));
    /// ```
    ///
    /// # Terminal Support
    ///
    /// Bold text is widely supported across terminal emulators. On terminals
    /// that don't support bold, the text may appear with a brighter color instead.
    pub fn bold(mut self, v: bool) -> Self {
        self.set_attr(ATTR_BOLD, v);
        self.set_prop(BOLD_KEY);
        self
    }

    /// Set whether to render text with italic font style.
    ///
    /// When `true`, text appears slanted (italic/oblique style).
    /// When `false`, text uses normal upright style.
    ///
    /// # Arguments
    ///
    /// * `v` - Whether to apply italic styling (`true`) or normal style (`false`)
    ///
    /// # Returns
    ///
    /// Returns `Self` for method chaining.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use lipgloss::Style;
    ///
    /// let italic_style = Style::new().italic(true);
    /// let emphasis_style = Style::new().italic(true).bold(true);
    ///
    /// println!("{}", italic_style.render("Italic text"));
    /// println!("{}", emphasis_style.render("Bold italic text"));
    /// ```
    ///
    /// # Terminal Support
    ///
    /// Italic support varies by terminal emulator. Modern terminals generally
    /// support italic text, but some older or minimal terminals may not display
    /// the italic effect.
    pub fn italic(mut self, v: bool) -> Self {
        self.set_attr(ATTR_ITALIC, v);
        self.set_prop(ITALIC_KEY);
        self
    }

    /// Set whether to render text with underline decoration.
    ///
    /// When `true`, text appears with a line underneath each character.
    /// When `false`, no underline decoration is applied.
    ///
    /// # Arguments
    ///
    /// * `v` - Whether to apply underline (`true`) or no underline (`false`)
    ///
    /// # Returns
    ///
    /// Returns `Self` for method chaining.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use lipgloss::Style;
    ///
    /// let link_style = Style::new()
    ///     .underline(true);
    ///
    /// let title_style = Style::new()
    ///     .underline(true)
    ///     .bold(true);
    ///
    /// println!("{}", link_style.render("https://example.com"));
    /// println!("{}", title_style.render("Important Title"));
    /// ```
    ///
    /// # See Also
    ///
    /// - [`underline_spaces()`](Self::underline_spaces) - Control whether spaces are underlined
    /// - [`strikethrough()`](Self::strikethrough) - Alternative text decoration
    pub fn underline(mut self, v: bool) -> Self {
        self.set_attr(ATTR_UNDERLINE, v);
        self.set_prop(UNDERLINE_KEY);
        self
    }

    /// Set whether to render text with strikethrough decoration.
    ///
    /// When `true`, text appears with a line through the middle of each character,
    /// commonly used to indicate deleted or outdated content.
    /// When `false`, no strikethrough decoration is applied.
    ///
    /// # Arguments
    ///
    /// * `v` - Whether to apply strikethrough (`true`) or no strikethrough (`false`)
    ///
    /// # Returns
    ///
    /// Returns `Self` for method chaining.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use lipgloss::Style;
    ///
    /// let deleted_style = Style::new()
    ///     .strikethrough(true);
    ///
    /// let correction_style = Style::new()
    ///     .strikethrough(true)
    ///     .faint(true);
    ///
    /// println!("{}", deleted_style.render("Old text"));
    /// println!("{}", correction_style.render("Incorrect information"));
    /// ```
    ///
    /// # See Also
    ///
    /// - [`strikethrough_spaces()`](Self::strikethrough_spaces) - Control whether spaces are struck through
    /// - [`underline()`](Self::underline) - Alternative text decoration
    pub fn strikethrough(mut self, v: bool) -> Self {
        self.set_attr(ATTR_STRIKETHROUGH, v);
        self.set_prop(STRIKETHROUGH_KEY);
        self
    }

    /// Set whether to reverse foreground and background colors.
    ///
    /// When `true`, the foreground and background colors are swapped, creating
    /// an inverted or highlighted effect. This is useful for creating selection
    /// highlights or attention-grabbing text.
    /// When `false`, colors appear in their normal arrangement.
    ///
    /// # Arguments
    ///
    /// * `v` - Whether to reverse colors (`true`) or use normal colors (`false`)
    ///
    /// # Returns
    ///
    /// Returns `Self` for method chaining.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use lipgloss::Style;
    ///
    /// let highlight_style = Style::new()
    ///     .reverse(true)
    ///     .bold(true);
    ///
    /// let selection_style = Style::new()
    ///     .reverse(true); // Will swap foreground and background
    ///
    /// println!("{}", highlight_style.render("Selected item"));
    /// println!("{}", selection_style.render("Highlighted text"));
    /// ```
    ///
    /// # Terminal Support
    ///
    /// Color reversal is widely supported across terminal emulators.
    /// If no explicit colors are set, the terminal's default foreground
    /// and background colors will be reversed.
    pub fn reverse(mut self, v: bool) -> Self {
        self.set_attr(ATTR_REVERSE, v);
        self.set_prop(REVERSE_KEY);
        self
    }

    /// Set whether to make text blink or flash.
    ///
    /// When `true`, text will periodically flash or blink to draw attention.
    /// When `false`, text remains static with no blinking effect.
    ///
    /// # Arguments
    ///
    /// * `v` - Whether to enable blinking (`true`) or disable it (`false`)
    ///
    /// # Returns
    ///
    /// Returns `Self` for method chaining.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use lipgloss::Style;
    ///
    /// let alert_style = Style::new()
    ///     .blink(true)
    ///     .bold(true);
    ///
    /// let warning_style = Style::new()
    ///     .blink(true)
    ///     .reverse(true);
    ///
    /// println!("{}", alert_style.render("⚠ ALERT ⚠"));
    /// println!("{}", warning_style.render("URGENT"));
    /// ```
    ///
    /// # Terminal Support
    ///
    /// Blinking text support varies significantly across terminal emulators.
    /// Many modern terminals disable blinking by default due to accessibility
    /// concerns. Consider using other attention-grabbing techniques like
    /// [`reverse()`](Self::reverse) or [`bold()`](Self::bold) for better compatibility.
    ///
    /// # Accessibility Note
    ///
    /// Blinking text can cause issues for users with certain visual conditions
    /// or photosensitive epilepsy. Use sparingly and consider providing
    /// alternatives for critical information.
    pub fn blink(mut self, v: bool) -> Self {
        self.set_attr(ATTR_BLINK, v);
        self.set_prop(BLINK_KEY);
        self
    }

    /// Set whether to render text with reduced intensity (faint/dim).
    ///
    /// When `true`, text appears with decreased brightness or intensity,
    /// useful for secondary information, comments, or de-emphasized content.
    /// When `false`, text uses normal intensity.
    ///
    /// # Arguments
    ///
    /// * `v` - Whether to apply faint styling (`true`) or normal intensity (`false`)
    ///
    /// # Returns
    ///
    /// Returns `Self` for method chaining.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use lipgloss::Style;
    ///
    /// let comment_style = Style::new()
    ///     .faint(true)
    ///     .italic(true);
    ///
    /// let metadata_style = Style::new()
    ///     .faint(true);
    ///
    /// println!("{}", comment_style.render("// This is a comment"));
    /// println!("{}", metadata_style.render("Last modified: 2024-01-01"));
    /// ```
    ///
    /// # Terminal Support
    ///
    /// Faint text is supported by most modern terminal emulators. The exact
    /// appearance varies by terminal - some reduce brightness, others use
    /// a lighter color variant.
    ///
    /// # Note
    ///
    /// Faint and [`bold()`](Self::bold) are generally mutually exclusive.
    /// The behavior when both are applied depends on the terminal emulator.
    pub fn faint(mut self, v: bool) -> Self {
        self.set_attr(ATTR_FAINT, v);
        self.set_prop(FAINT_KEY);
        self
    }

    /// Set whether to apply underline decoration to space characters.
    ///
    /// When `true`, space characters will also be underlined when [`underline()`](Self::underline)
    /// is enabled, creating continuous underlines across words.
    /// When `false` (default), only non-space characters are underlined.
    ///
    /// # Arguments
    ///
    /// * `v` - Whether to underline spaces (`true`) or skip them (`false`)
    ///
    /// # Returns
    ///
    /// Returns `Self` for method chaining.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use lipgloss::Style;
    ///
    /// let continuous_underline = Style::new()
    ///     .underline(true)
    ///     .underline_spaces(true);
    ///
    /// let word_underline = Style::new()
    ///     .underline(true)
    ///     .underline_spaces(false);
    ///
    /// // Creates: "Hello_____World" (continuous underline)
    /// println!("{}", continuous_underline.render("Hello World"));
    ///
    /// // Creates: "Hello World" (only words underlined)
    /// //           -----   -----
    /// println!("{}", word_underline.render("Hello World"));
    /// ```
    ///
    /// # Note
    ///
    /// This setting only has an effect when [`underline()`](Self::underline) is also `true`.
    /// It provides finer control over how underline decoration is applied to whitespace.
    pub fn underline_spaces(mut self, v: bool) -> Self {
        self.set_attr(ATTR_UNDERLINE_SPACES, v);
        self.set_prop(UNDERLINE_SPACES_KEY);
        self
    }

    /// Set whether to apply strikethrough decoration to space characters.
    ///
    /// When `true`, space characters will also be struck through when [`strikethrough()`](Self::strikethrough)
    /// is enabled, creating continuous strikethrough lines across words.
    /// When `false` (default), only non-space characters are struck through.
    ///
    /// # Arguments
    ///
    /// * `v` - Whether to strikethrough spaces (`true`) or skip them (`false`)
    ///
    /// # Returns
    ///
    /// Returns `Self` for method chaining.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use lipgloss::Style;
    ///
    /// let continuous_strike = Style::new()
    ///     .strikethrough(true)
    ///     .strikethrough_spaces(true);
    ///
    /// let word_strike = Style::new()
    ///     .strikethrough(true)
    ///     .strikethrough_spaces(false);
    ///
    /// // Creates continuous line through everything including spaces
    /// println!("{}", continuous_strike.render("Old text here"));
    ///
    /// // Creates lines only through words, not spaces
    /// println!("{}", word_strike.render("Old text here"));
    /// ```
    ///
    /// # Note
    ///
    /// This setting only has an effect when [`strikethrough()`](Self::strikethrough) is also `true`.
    /// It provides finer control over how strikethrough decoration is applied to whitespace.
    pub fn strikethrough_spaces(mut self, v: bool) -> Self {
        self.set_attr(ATTR_STRIKETHROUGH_SPACES, v);
        self.set_prop(STRIKETHROUGH_SPACES_KEY);
        self
    }

    /// Set whether to apply foreground and background colors to whitespace characters.
    ///
    /// When `true`, space characters and other whitespace will also receive the
    /// foreground and background colors set on this style.
    /// When `false` (default), whitespace characters remain uncolored.
    ///
    /// # Arguments
    ///
    /// * `v` - Whether to color whitespace (`true`) or leave it uncolored (`false`)
    ///
    /// # Returns
    ///
    /// Returns `Self` for method chaining.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use lipgloss::Style;
    ///
    /// let highlight_style = Style::new()
    ///     .color_whitespace(true);
    ///
    /// let normal_style = Style::new()
    ///     .color_whitespace(false);
    ///
    /// // Colors the entire text including spaces
    /// println!("{}", highlight_style.render("Hello World"));
    ///
    /// // Colors only the words, spaces remain uncolored
    /// println!("{}", normal_style.render("Hello World"));
    /// ```
    ///
    /// # Use Cases
    ///
    /// This is particularly useful for:
    /// - Creating solid background highlights that include spaces
    /// - Building table cells or bordered content where the entire area should be colored
    /// - Making selection highlights that cover whitespace
    ///
    /// # Note
    ///
    /// This setting affects both foreground and background colors applied to the style.
    pub fn color_whitespace(mut self, v: bool) -> Self {
        self.set_attr(ATTR_COLOR_WHITESPACE, v);
        self.set_prop(COLOR_WHITESPACE_KEY);
        self
    }
}
