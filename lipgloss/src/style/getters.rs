//! Style property getter methods
//!
//! This module provides getter methods for the [`Style`] struct, allowing read-only access
//! to all style properties. These methods follow a consistent pattern of checking if a
//! property has been explicitly set before returning either the configured value or a
//! sensible default.
//!
//! # Property Access Pattern
//!
//! Most getters follow this pattern:
//! 1. Check if the property has been explicitly set using [`is_set`](Style::is_set)
//! 2. Return the configured value if set, or a default value if not set
//! 3. For boolean attributes, also check the attribute bit using [`get_attr`](Style::get_attr)
//!
//! # Examples
//!
//! ```rust
//! use lipgloss::Style;
//!
//! let style = Style::new()
//!     .bold(true)
//!     .foreground("red")
//!     .padding(2, 2, 2, 2)
//!     .margin(1, 1, 1, 1);
//!
//! // Text attributes
//! assert_eq!(style.get_bold(), true);
//! assert_eq!(style.get_italic(), false); // Not set, returns default
//!
//! // Colors
//! assert!(style.get_foreground().is_some());
//! assert!(style.get_background().is_none()); // Not set
//!
//! // Dimensions
//! assert_eq!(style.get_padding(), (2, 2, 2, 2));
//! assert_eq!(style.get_margin(), (1, 1, 1, 1));
//! ```

use crate::border::{hidden_border, Border};
use crate::color::Color;
use crate::position::{Position, LEFT, TOP};
use crate::style::{properties::*, Style};

impl Style {
    // ---------- Text attribute getters ----------

    /// Gets the bold text attribute setting.
    ///
    /// Returns `true` if bold text has been explicitly enabled, `false` otherwise.
    /// This checks both that the bold property has been set and that the bold
    /// attribute bit is enabled.
    ///
    /// # Returns
    ///
    /// Returns `true` if bold is enabled, `false` if disabled or not set.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use lipgloss::Style;
    ///
    /// let style = Style::new().bold(true);
    /// assert_eq!(style.get_bold(), true);
    ///
    /// let style = Style::new().bold(false);
    /// assert_eq!(style.get_bold(), false);
    ///
    /// let style = Style::new(); // Not set
    /// assert_eq!(style.get_bold(), false);
    /// ```
    pub fn get_bold(&self) -> bool {
        self.is_set(BOLD_KEY) && self.get_attr(ATTR_BOLD)
    }

    /// Gets the italic text attribute setting.
    ///
    /// Returns `true` if italic text has been explicitly enabled, `false` otherwise.
    ///
    /// # Returns
    ///
    /// Returns `true` if italic is enabled, `false` if disabled or not set.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use lipgloss::Style;
    ///
    /// let style = Style::new().italic(true);
    /// assert_eq!(style.get_italic(), true);
    ///
    /// let style = Style::new(); // Not set
    /// assert_eq!(style.get_italic(), false);
    /// ```
    pub fn get_italic(&self) -> bool {
        self.is_set(ITALIC_KEY) && self.get_attr(ATTR_ITALIC)
    }

    /// Gets the underline text attribute setting.
    ///
    /// Returns `true` if underlined text has been explicitly enabled, `false` otherwise.
    ///
    /// # Returns
    ///
    /// Returns `true` if underline is enabled, `false` if disabled or not set.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use lipgloss::Style;
    ///
    /// let style = Style::new().underline(true);
    /// assert_eq!(style.get_underline(), true);
    /// ```
    pub fn get_underline(&self) -> bool {
        self.is_set(UNDERLINE_KEY) && self.get_attr(ATTR_UNDERLINE)
    }

    /// Gets the strikethrough text attribute setting.
    ///
    /// Returns `true` if strikethrough text has been explicitly enabled, `false` otherwise.
    ///
    /// # Returns
    ///
    /// Returns `true` if strikethrough is enabled, `false` if disabled or not set.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use lipgloss::Style;
    ///
    /// let style = Style::new().strikethrough(true);
    /// assert_eq!(style.get_strikethrough(), true);
    /// ```
    pub fn get_strikethrough(&self) -> bool {
        self.is_set(STRIKETHROUGH_KEY) && self.get_attr(ATTR_STRIKETHROUGH)
    }

    /// Gets the reverse (inverted colors) text attribute setting.
    ///
    /// Returns `true` if reverse/inverted text has been explicitly enabled, `false` otherwise.
    /// Reverse text swaps the foreground and background colors.
    ///
    /// # Returns
    ///
    /// Returns `true` if reverse is enabled, `false` if disabled or not set.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use lipgloss::Style;
    ///
    /// let style = Style::new().reverse(true);
    /// assert_eq!(style.get_reverse(), true);
    /// ```
    pub fn get_reverse(&self) -> bool {
        self.is_set(REVERSE_KEY) && self.get_attr(ATTR_REVERSE)
    }

    /// Gets the blink text attribute setting.
    ///
    /// Returns `true` if blinking text has been explicitly enabled, `false` otherwise.
    /// Note that blink support varies by terminal.
    ///
    /// # Returns
    ///
    /// Returns `true` if blink is enabled, `false` if disabled or not set.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use lipgloss::Style;
    ///
    /// let style = Style::new().blink(true);
    /// assert_eq!(style.get_blink(), true);
    /// ```
    pub fn get_blink(&self) -> bool {
        self.is_set(BLINK_KEY) && self.get_attr(ATTR_BLINK)
    }

    /// Gets the faint (dim) text attribute setting.
    ///
    /// Returns `true` if faint/dim text has been explicitly enabled, `false` otherwise.
    /// Faint text appears with reduced intensity.
    ///
    /// # Returns
    ///
    /// Returns `true` if faint is enabled, `false` if disabled or not set.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use lipgloss::Style;
    ///
    /// let style = Style::new().faint(true);
    /// assert_eq!(style.get_faint(), true);
    /// ```
    pub fn get_faint(&self) -> bool {
        self.is_set(FAINT_KEY) && self.get_attr(ATTR_FAINT)
    }

    /// Gets the underline spaces setting.
    ///
    /// Returns `true` if underlining whitespace characters has been explicitly enabled,
    /// `false` otherwise. When enabled, spaces and other whitespace will also be underlined.
    ///
    /// # Returns
    ///
    /// Returns `true` if underline spaces is enabled, `false` if disabled or not set.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use lipgloss::Style;
    ///
    /// let style = Style::new().underline_spaces(true);
    /// assert_eq!(style.get_underline_spaces(), true);
    /// ```
    pub fn get_underline_spaces(&self) -> bool {
        self.is_set(UNDERLINE_SPACES_KEY) && self.get_attr(ATTR_UNDERLINE_SPACES)
    }

    /// Gets the strikethrough spaces setting.
    ///
    /// Returns `true` if striking through whitespace characters has been explicitly enabled,
    /// `false` otherwise. When enabled, spaces and other whitespace will also be struck through.
    ///
    /// # Returns
    ///
    /// Returns `true` if strikethrough spaces is enabled, `false` if disabled or not set.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use lipgloss::Style;
    ///
    /// let style = Style::new().strikethrough_spaces(true);
    /// assert_eq!(style.get_strikethrough_spaces(), true);
    /// ```
    pub fn get_strikethrough_spaces(&self) -> bool {
        self.is_set(STRIKETHROUGH_SPACES_KEY) && self.get_attr(ATTR_STRIKETHROUGH_SPACES)
    }

    /// Gets the color whitespace setting.
    ///
    /// Returns `true` if coloring whitespace characters has been explicitly enabled,
    /// `false` otherwise. When enabled, background colors will be applied to spaces
    /// and other whitespace characters.
    ///
    /// # Returns
    ///
    /// Returns `true` if color whitespace is enabled, `false` if disabled or not set.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use lipgloss::Style;
    ///
    /// let style = Style::new().color_whitespace(true);
    /// assert_eq!(style.get_color_whitespace(), true);
    /// ```
    pub fn get_color_whitespace(&self) -> bool {
        self.is_set(COLOR_WHITESPACE_KEY) && self.get_attr(ATTR_COLOR_WHITESPACE)
    }

    /// Gets the inline rendering setting.
    ///
    /// Returns `true` if inline rendering has been explicitly enabled, `false` otherwise.
    /// Inline rendering affects how the style is applied in flowing text contexts.
    ///
    /// # Returns
    ///
    /// Returns `true` if inline is enabled, `false` if disabled or not set.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use lipgloss::Style;
    ///
    /// let style = Style::new().inline(true);
    /// assert_eq!(style.get_inline(), true);
    /// ```
    pub fn get_inline(&self) -> bool {
        self.is_set(INLINE_KEY) && self.get_attr(ATTR_INLINE)
    }

    // ---------- Color getters ----------

    /// Gets the foreground (text) color.
    ///
    /// Returns the configured foreground color if one has been set, or `None` if
    /// no foreground color has been configured.
    ///
    /// # Returns
    ///
    /// Returns `Some(Color)` if a foreground color is set, `None` otherwise.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use lipgloss::Style;
    ///
    /// let style = Style::new().foreground("red");
    /// assert!(style.get_foreground().is_some());
    ///
    /// let style = Style::new(); // No color set
    /// assert!(style.get_foreground().is_none());
    /// ```
    pub fn get_foreground(&self) -> Option<Color> {
        if self.is_set(FOREGROUND_KEY) {
            self.fg_color.as_ref().map(|s| Color::from(s.as_str()))
        } else {
            None
        }
    }

    /// Gets the background color.
    ///
    /// Returns the configured background color if one has been set, or `None` if
    /// no background color has been configured.
    ///
    /// # Returns
    ///
    /// Returns `Some(Color)` if a background color is set, `None` otherwise.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use lipgloss::Style;
    ///
    /// let style = Style::new().background("blue");
    /// assert!(style.get_background().is_some());
    ///
    /// let style = Style::new(); // No color set
    /// assert!(style.get_background().is_none());
    /// ```
    pub fn get_background(&self) -> Option<Color> {
        if self.is_set(BACKGROUND_KEY) {
            self.bg_color.as_ref().map(|s| Color::from(s.as_str()))
        } else {
            None
        }
    }

    /// Gets the margin background color.
    ///
    /// Returns the configured margin background color if one has been set, or `None` if
    /// no margin background color has been configured.
    ///
    /// # Returns
    ///
    /// Returns `Some(Color)` if a margin background color is set, `None` otherwise.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use lipgloss::Style;
    ///
    /// let style = Style::new().margin_background("purple");
    /// assert!(style.get_margin_background().is_some());
    ///
    /// let style = Style::new(); // No color set
    /// assert!(style.get_margin_background().is_none());
    /// ```
    pub fn get_margin_background(&self) -> Option<Color> {
        if self.is_set(MARGIN_BACKGROUND_KEY) {
            self.margin_bg_color.as_ref().map(|s| Color::from(s.as_str()))
        } else {
            None
        }
    }

    // ---------- Size getters ----------

    /// Gets the width constraint.
    ///
    /// Returns the configured width if one has been set, or `0` if no width
    /// constraint has been configured.
    ///
    /// # Returns
    ///
    /// Returns the width in characters, or `0` if not set.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use lipgloss::Style;
    ///
    /// let style = Style::new().width(40);
    /// assert_eq!(style.get_width(), 40);
    ///
    /// let style = Style::new(); // No width set
    /// assert_eq!(style.get_width(), 0);
    /// ```
    pub fn get_width(&self) -> i32 {
        if self.is_set(WIDTH_KEY) {
            self.width
        } else {
            0
        }
    }

    /// Gets the height constraint.
    ///
    /// Returns the configured height if one has been set, or `0` if no height
    /// constraint has been configured.
    ///
    /// # Returns
    ///
    /// Returns the height in lines, or `0` if not set.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use lipgloss::Style;
    ///
    /// let style = Style::new().height(10);
    /// assert_eq!(style.get_height(), 10);
    ///
    /// let style = Style::new(); // No height set
    /// assert_eq!(style.get_height(), 0);
    /// ```
    pub fn get_height(&self) -> i32 {
        if self.is_set(HEIGHT_KEY) {
            self.height
        } else {
            0
        }
    }

    /// Gets the maximum width constraint.
    ///
    /// Returns the configured maximum width if one has been set, or `0` if no
    /// maximum width constraint has been configured.
    ///
    /// # Returns
    ///
    /// Returns the maximum width in characters, or `0` if not set.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use lipgloss::Style;
    ///
    /// let style = Style::new().max_width(80);
    /// assert_eq!(style.get_max_width(), 80);
    /// ```
    pub fn get_max_width(&self) -> i32 {
        if self.is_set(MAX_WIDTH_KEY) {
            self.max_width
        } else {
            0
        }
    }

    /// Gets the maximum height constraint.
    ///
    /// Returns the configured maximum height if one has been set, or `0` if no
    /// maximum height constraint has been configured.
    ///
    /// # Returns
    ///
    /// Returns the maximum height in lines, or `0` if not set.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use lipgloss::Style;
    ///
    /// let style = Style::new().max_height(25);
    /// assert_eq!(style.get_max_height(), 25);
    /// ```
    pub fn get_max_height(&self) -> i32 {
        if self.is_set(MAX_HEIGHT_KEY) {
            self.max_height
        } else {
            0
        }
    }

    // ---------- Alignment getters ----------

    /// Gets the horizontal alignment setting.
    ///
    /// Returns the configured horizontal alignment, or [`LEFT`] if no horizontal
    /// alignment has been explicitly set.
    ///
    /// # Returns
    ///
    /// Returns a [`Position`] indicating the horizontal alignment.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use lipgloss::{Style, position::{CENTER, LEFT}};
    ///
    /// let style = Style::new().align_horizontal(CENTER);
    /// assert_eq!(style.get_align_horizontal(), CENTER);
    ///
    /// let style = Style::new(); // Default alignment
    /// assert_eq!(style.get_align_horizontal(), LEFT);
    /// ```
    pub fn get_align_horizontal(&self) -> Position {
        if self.is_set(ALIGN_HORIZONTAL_KEY) {
            self.align_horizontal
        } else {
            LEFT
        }
    }

    /// Gets the vertical alignment setting.
    ///
    /// Returns the configured vertical alignment, or [`TOP`] if no vertical
    /// alignment has been explicitly set.
    ///
    /// # Returns
    ///
    /// Returns a [`Position`] indicating the vertical alignment.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use lipgloss::{Style, position::{CENTER, TOP}};
    ///
    /// let style = Style::new().align_vertical(CENTER);
    /// assert_eq!(style.get_align_vertical(), CENTER);
    ///
    /// let style = Style::new(); // Default alignment
    /// assert_eq!(style.get_align_vertical(), TOP);
    /// ```
    pub fn get_align_vertical(&self) -> Position {
        if self.is_set(ALIGN_VERTICAL_KEY) {
            self.align_vertical
        } else {
            TOP
        }
    }

    /// Gets the horizontal alignment setting (alias for [`get_align_horizontal`]).
    ///
    /// This is a convenience method that returns the same value as
    /// [`get_align_horizontal`](Self::get_align_horizontal).
    ///
    /// # Returns
    ///
    /// Returns a [`Position`] indicating the horizontal alignment.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use lipgloss::{Style, position::CENTER};
    ///
    /// let style = Style::new().align_horizontal(CENTER);
    /// assert_eq!(style.get_align(), CENTER);
    /// assert_eq!(style.get_align(), style.get_align_horizontal());
    /// ```
    pub fn get_align(&self) -> Position {
        self.get_align_horizontal()
    }

    // ---------- Padding getters ----------

    /// Gets all padding values as a tuple.
    ///
    /// Returns the padding values in the order (top, right, bottom, left).
    /// If individual padding values haven't been set, they default to `0`.
    ///
    /// # Returns
    ///
    /// Returns a tuple of `(top, right, bottom, left)` padding values in characters.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use lipgloss::Style;
    ///
    /// let style = Style::new().padding(2, 4, 3, 1);
    /// assert_eq!(style.get_padding(), (2, 4, 3, 1));
    ///
    /// let style = Style::new().padding(2, 2, 2, 2); // Uniform padding
    /// assert_eq!(style.get_padding(), (2, 2, 2, 2));
    /// ```
    pub fn get_padding(&self) -> (i32, i32, i32, i32) {
        (
            self.get_padding_top(),
            self.get_padding_right(),
            self.get_padding_bottom(),
            self.get_padding_left(),
        )
    }

    /// Gets the top padding value.
    ///
    /// Returns the configured top padding, or `0` if not set.
    ///
    /// # Returns
    ///
    /// Returns the top padding in characters.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use lipgloss::Style;
    ///
    /// let style = Style::new().padding_top(3);
    /// assert_eq!(style.get_padding_top(), 3);
    /// ```
    pub fn get_padding_top(&self) -> i32 {
        if self.is_set(PADDING_TOP_KEY) {
            self.padding_top
        } else {
            0
        }
    }

    /// Gets the right padding value.
    ///
    /// Returns the configured right padding, or `0` if not set.
    ///
    /// # Returns
    ///
    /// Returns the right padding in characters.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use lipgloss::Style;
    ///
    /// let style = Style::new().padding_right(2);
    /// assert_eq!(style.get_padding_right(), 2);
    /// ```
    pub fn get_padding_right(&self) -> i32 {
        if self.is_set(PADDING_RIGHT_KEY) {
            self.padding_right
        } else {
            0
        }
    }

    /// Gets the bottom padding value.
    ///
    /// Returns the configured bottom padding, or `0` if not set.
    ///
    /// # Returns
    ///
    /// Returns the bottom padding in characters.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use lipgloss::Style;
    ///
    /// let style = Style::new().padding_bottom(1);
    /// assert_eq!(style.get_padding_bottom(), 1);
    /// ```
    pub fn get_padding_bottom(&self) -> i32 {
        if self.is_set(PADDING_BOTTOM_KEY) {
            self.padding_bottom
        } else {
            0
        }
    }

    /// Gets the left padding value.
    ///
    /// Returns the configured left padding, or `0` if not set.
    ///
    /// # Returns
    ///
    /// Returns the left padding in characters.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use lipgloss::Style;
    ///
    /// let style = Style::new().padding_left(4);
    /// assert_eq!(style.get_padding_left(), 4);
    /// ```
    pub fn get_padding_left(&self) -> i32 {
        if self.is_set(PADDING_LEFT_KEY) {
            self.padding_left
        } else {
            0
        }
    }

    // ---------- Margin getters ----------

    /// Gets all margin values as a tuple.
    ///
    /// Returns the margin values in the order (top, right, bottom, left).
    /// If individual margin values haven't been set, they default to `0`.
    ///
    /// # Returns
    ///
    /// Returns a tuple of `(top, right, bottom, left)` margin values in characters.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use lipgloss::Style;
    ///
    /// let style = Style::new().margin(1, 2, 1, 2);
    /// assert_eq!(style.get_margin(), (1, 2, 1, 2));
    ///
    /// let style = Style::new().margin(3, 3, 3, 3); // Uniform margin
    /// assert_eq!(style.get_margin(), (3, 3, 3, 3));
    /// ```
    pub fn get_margin(&self) -> (i32, i32, i32, i32) {
        (
            self.get_margin_top(),
            self.get_margin_right(),
            self.get_margin_bottom(),
            self.get_margin_left(),
        )
    }

    /// Gets the top margin value.
    ///
    /// Returns the configured top margin, or `0` if not set.
    ///
    /// # Returns
    ///
    /// Returns the top margin in characters.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use lipgloss::Style;
    ///
    /// let style = Style::new().margin_top(2);
    /// assert_eq!(style.get_margin_top(), 2);
    /// ```
    pub fn get_margin_top(&self) -> i32 {
        if self.is_set(MARGIN_TOP_KEY) {
            self.margin_top
        } else {
            0
        }
    }

    /// Gets the right margin value.
    ///
    /// Returns the configured right margin, or `0` if not set.
    ///
    /// # Returns
    ///
    /// Returns the right margin in characters.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use lipgloss::Style;
    ///
    /// let style = Style::new().margin_right(1);
    /// assert_eq!(style.get_margin_right(), 1);
    /// ```
    pub fn get_margin_right(&self) -> i32 {
        if self.is_set(MARGIN_RIGHT_KEY) {
            self.margin_right
        } else {
            0
        }
    }

    /// Gets the bottom margin value.
    ///
    /// Returns the configured bottom margin, or `0` if not set.
    ///
    /// # Returns
    ///
    /// Returns the bottom margin in characters.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use lipgloss::Style;
    ///
    /// let style = Style::new().margin_bottom(3);
    /// assert_eq!(style.get_margin_bottom(), 3);
    /// ```
    pub fn get_margin_bottom(&self) -> i32 {
        if self.is_set(MARGIN_BOTTOM_KEY) {
            self.margin_bottom
        } else {
            0
        }
    }

    /// Gets the left margin value.
    ///
    /// Returns the configured left margin, or `0` if not set.
    ///
    /// # Returns
    ///
    /// Returns the left margin in characters.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use lipgloss::Style;
    ///
    /// let style = Style::new().margin_left(5);
    /// assert_eq!(style.get_margin_left(), 5);
    /// ```
    pub fn get_margin_left(&self) -> i32 {
        if self.is_set(MARGIN_LEFT_KEY) {
            self.margin_left
        } else {
            0
        }
    }

    // ---------- Border getters ----------

    /// Gets the border style and enabled sides.
    ///
    /// Returns a tuple containing the border style and boolean flags indicating
    /// which sides are enabled, in the order (style, top, right, bottom, left).
    ///
    /// # Returns
    ///
    /// Returns `(Border, bool, bool, bool, bool)` where the booleans indicate
    /// whether each border side is enabled.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use lipgloss::{Style, hidden_border, normal_border};
    ///
    /// let style = Style::new().border(normal_border());
    /// let (border, top, right, bottom, left) = style.get_border();
    /// assert_eq!(top, true);
    /// assert_eq!(right, true);
    /// assert_eq!(bottom, true);
    /// assert_eq!(left, true);
    /// ```
    pub fn get_border(&self) -> (Border, bool, bool, bool, bool) {
        let border = if self.is_set(BORDER_STYLE_KEY) {
            self.border_style
        } else {
            hidden_border()
        };
        let top = !self.is_set(BORDER_TOP_KEY) || self.get_attr(ATTR_BORDER_TOP);
        let right = !self.is_set(BORDER_RIGHT_KEY) || self.get_attr(ATTR_BORDER_RIGHT);
        let bottom = !self.is_set(BORDER_BOTTOM_KEY) || self.get_attr(ATTR_BORDER_BOTTOM);
        let left = !self.is_set(BORDER_LEFT_KEY) || self.get_attr(ATTR_BORDER_LEFT);
        (border, top, right, bottom, left)
    }

    /// Gets the border style.
    ///
    /// Returns the configured border style, or a hidden border if no style
    /// has been set.
    ///
    /// # Returns
    ///
    /// Returns the [`Border`] style.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use lipgloss::{Style, normal_border, hidden_border};
    ///
    /// let style = Style::new().border_style(normal_border());
    /// // Border style is set but doesn't directly compare
    ///
    /// let style = Style::new(); // No border set
    /// // Returns hidden_border() by default
    /// ```
    pub fn get_border_style(&self) -> Border {
        if self.is_set(BORDER_STYLE_KEY) {
            self.border_style
        } else {
            hidden_border()
        }
    }

    /// Gets whether the top border is enabled.
    ///
    /// Returns `true` if the top border should be rendered, `false` otherwise.
    /// If the border hasn't been explicitly configured, returns `true` (default enabled).
    ///
    /// # Returns
    ///
    /// Returns `true` if the top border is enabled.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use lipgloss::{Style, normal_border};
    ///
    /// let style = Style::new()
    ///     .border_style(normal_border())
    ///     .border_top(true);
    /// assert_eq!(style.get_border_top(), true);
    ///
    /// let style = Style::new()
    ///     .border_style(normal_border())
    ///     .border_top(false);
    /// assert_eq!(style.get_border_top(), false);
    /// ```
    pub fn get_border_top(&self) -> bool {
        !self.is_set(BORDER_TOP_KEY) || self.get_attr(ATTR_BORDER_TOP)
    }

    /// Gets whether the right border is enabled.
    ///
    /// Returns `true` if the right border should be rendered, `false` otherwise.
    /// If the border hasn't been explicitly configured, returns `true` (default enabled).
    ///
    /// # Returns
    ///
    /// Returns `true` if the right border is enabled.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use lipgloss::{Style, normal_border};
    ///
    /// let style = Style::new()
    ///     .border_style(normal_border())
    ///     .border_right(false);
    /// assert_eq!(style.get_border_right(), false);
    /// ```
    pub fn get_border_right(&self) -> bool {
        !self.is_set(BORDER_RIGHT_KEY) || self.get_attr(ATTR_BORDER_RIGHT)
    }

    /// Gets whether the bottom border is enabled.
    ///
    /// Returns `true` if the bottom border should be rendered, `false` otherwise.
    /// If the border hasn't been explicitly configured, returns `true` (default enabled).
    ///
    /// # Returns
    ///
    /// Returns `true` if the bottom border is enabled.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use lipgloss::{Style, normal_border};
    ///
    /// let style = Style::new()
    ///     .border_style(normal_border())
    ///     .border_bottom(true);
    /// assert_eq!(style.get_border_bottom(), true);
    /// ```
    pub fn get_border_bottom(&self) -> bool {
        !self.is_set(BORDER_BOTTOM_KEY) || self.get_attr(ATTR_BORDER_BOTTOM)
    }

    /// Gets whether the left border is enabled.
    ///
    /// Returns `true` if the left border should be rendered, `false` otherwise.
    /// If the border hasn't been explicitly configured, returns `true` (default enabled).
    ///
    /// # Returns
    ///
    /// Returns `true` if the left border is enabled.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use lipgloss::{Style, normal_border};
    ///
    /// let style = Style::new()
    ///     .border_style(normal_border())
    ///     .border_left(false);
    /// assert_eq!(style.get_border_left(), false);
    /// ```
    pub fn get_border_left(&self) -> bool {
        !self.is_set(BORDER_LEFT_KEY) || self.get_attr(ATTR_BORDER_LEFT)
    }

    // ---------- Other getters ----------

    /// Gets the tab width setting.
    ///
    /// Returns the configured tab width, or the default tab width if none has been set.
    ///
    /// # Returns
    ///
    /// Returns the tab width in characters.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use lipgloss::Style;
    ///
    /// let style = Style::new().tab_width(8);
    /// assert_eq!(style.get_tab_width(), 8);
    ///
    /// let style = Style::new(); // Uses default
    /// // Returns TAB_WIDTH_DEFAULT
    /// ```
    pub fn get_tab_width(&self) -> i32 {
        if self.is_set(TAB_WIDTH_KEY) {
            self.tab_width
        } else {
            TAB_WIDTH_DEFAULT
        }
    }

    /// Gets information about the transform function.
    ///
    /// Since transform functions cannot be cloned or inspected, this method returns
    /// a string indication if a transform function has been set, or `None` if no
    /// transform has been configured.
    ///
    /// # Returns
    ///
    /// Returns `Some("function")` if a transform is set, `None` otherwise.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use lipgloss::Style;
    ///
    /// let style = Style::new().transform(|s| s.to_uppercase());
    /// assert_eq!(style.get_transform(), Some("function".to_string()));
    ///
    /// let style = Style::new(); // No transform
    /// assert_eq!(style.get_transform(), None);
    /// ```
    pub fn get_transform(&self) -> Option<String> {
        // Can't return the actual function, so return indication if it's set
        if self.is_set(TRANSFORM_KEY) && self.transform.is_some() {
            Some("function".to_string())
        } else {
            None
        }
    }

    // ---------- Frame size calculations ----------

    /// Gets the total frame size including padding, borders, and margins.
    ///
    /// Returns the total space consumed by the frame elements (padding + borders + margins)
    /// in both horizontal and vertical directions.
    ///
    /// # Returns
    ///
    /// Returns `(horizontal_size, vertical_size)` in characters and lines respectively.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use lipgloss::{Style, normal_border};
    ///
    /// let style = Style::new()
    ///     .padding(1, 2, 1, 2)  // top, right, bottom, left
    ///     .margin(1, 1, 1, 1)
    ///     .border(normal_border());
    ///
    /// let (horizontal, vertical) = style.get_frame_size();
    /// // horizontal = left_margin + left_border + left_padding + right_padding + right_border + right_margin
    /// // vertical = top_margin + top_border + top_padding + bottom_padding + bottom_border + bottom_margin
    /// ```
    pub fn get_frame_size(&self) -> (i32, i32) {
        (
            self.get_horizontal_frame_size(),
            self.get_vertical_frame_size(),
        )
    }

    /// Gets the horizontal frame size.
    ///
    /// Returns the total horizontal space consumed by padding, borders, and margins.
    /// This is the sum of left and right padding, borders, and margins.
    ///
    /// # Returns
    ///
    /// Returns the horizontal frame size in characters.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use lipgloss::{Style, normal_border};
    ///
    /// let style = Style::new()
    ///     .padding(0, 2, 0, 1)  // right=2, left=1
    ///     .margin(0, 1, 0, 1)   // right=1, left=1
    ///     .border(normal_border()); // right=1, left=1
    ///
    /// assert_eq!(style.get_horizontal_frame_size(), 7); // 2+1 + 1+1 + 1+1
    /// ```
    pub fn get_horizontal_frame_size(&self) -> i32 {
        self.get_horizontal_padding()
            + self.get_horizontal_border_size()
            + self.get_horizontal_margins()
    }

    /// Gets the vertical frame size.
    ///
    /// Returns the total vertical space consumed by padding, borders, and margins.
    /// This is the sum of top and bottom padding, borders, and margins.
    ///
    /// # Returns
    ///
    /// Returns the vertical frame size in lines.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use lipgloss::{Style, normal_border};
    ///
    /// let style = Style::new()
    ///     .padding(2, 0, 1, 0)  // top=2, bottom=1
    ///     .margin(1, 0, 1, 0)   // top=1, bottom=1
    ///     .border(normal_border()); // top=1, bottom=1
    ///
    /// assert_eq!(style.get_vertical_frame_size(), 7); // 2+1 + 1+1 + 1+1
    /// ```
    pub fn get_vertical_frame_size(&self) -> i32 {
        self.get_vertical_padding() + self.get_vertical_border_size() + self.get_vertical_margins()
    }

    /// Gets the total horizontal padding.
    ///
    /// Returns the sum of left and right padding values.
    ///
    /// # Returns
    ///
    /// Returns the total horizontal padding in characters.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use lipgloss::Style;
    ///
    /// let style = Style::new().padding(0, 3, 0, 2); // right=3, left=2
    /// assert_eq!(style.get_horizontal_padding(), 5);
    /// ```
    pub fn get_horizontal_padding(&self) -> i32 {
        self.get_padding_left() + self.get_padding_right()
    }

    /// Gets the total vertical padding.
    ///
    /// Returns the sum of top and bottom padding values.
    ///
    /// # Returns
    ///
    /// Returns the total vertical padding in lines.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use lipgloss::Style;
    ///
    /// let style = Style::new().padding(2, 0, 3, 0); // top=2, bottom=3
    /// assert_eq!(style.get_vertical_padding(), 5);
    /// ```
    pub fn get_vertical_padding(&self) -> i32 {
        self.get_padding_top() + self.get_padding_bottom()
    }

    /// Gets the total horizontal margins.
    ///
    /// Returns the sum of left and right margin values.
    ///
    /// # Returns
    ///
    /// Returns the total horizontal margins in characters.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use lipgloss::Style;
    ///
    /// let style = Style::new().margin(0, 2, 0, 1); // right=2, left=1
    /// assert_eq!(style.get_horizontal_margins(), 3);
    /// ```
    pub fn get_horizontal_margins(&self) -> i32 {
        self.get_margin_left() + self.get_margin_right()
    }

    /// Gets the total vertical margins.
    ///
    /// Returns the sum of top and bottom margin values.
    ///
    /// # Returns
    ///
    /// Returns the total vertical margins in lines.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use lipgloss::Style;
    ///
    /// let style = Style::new().margin(1, 0, 2, 0); // top=1, bottom=2
    /// assert_eq!(style.get_vertical_margins(), 3);
    /// ```
    pub fn get_vertical_margins(&self) -> i32 {
        self.get_margin_top() + self.get_margin_bottom()
    }

    /// Gets the total horizontal border size.
    ///
    /// Returns the sum of left and right border widths. Each enabled border
    /// side contributes 1 character to the total width.
    ///
    /// # Returns
    ///
    /// Returns the total horizontal border size in characters (0-2).
    ///
    /// # Examples
    ///
    /// ```rust
    /// use lipgloss::{Style, normal_border};
    ///
    /// let style = Style::new()
    ///     .border_style(normal_border())
    ///     .border_left(true)
    ///     .border_right(true);
    /// assert_eq!(style.get_horizontal_border_size(), 2);
    ///
    /// let style = Style::new()
    ///     .border_style(normal_border())
    ///     .border_left(true)
    ///     .border_right(false);
    /// assert_eq!(style.get_horizontal_border_size(), 1);
    /// ```
    pub fn get_horizontal_border_size(&self) -> i32 {
        self.get_border_left_size() + self.get_border_right_size()
    }

    /// Gets the total vertical border size.
    ///
    /// Returns the sum of top and bottom border heights. Each enabled border
    /// side contributes 1 line to the total height.
    ///
    /// # Returns
    ///
    /// Returns the total vertical border size in lines (0-2).
    ///
    /// # Examples
    ///
    /// ```rust
    /// use lipgloss::{Style, normal_border};
    ///
    /// let style = Style::new()
    ///     .border_style(normal_border())
    ///     .border_top(true)
    ///     .border_bottom(true);
    /// assert_eq!(style.get_vertical_border_size(), 2);
    ///
    /// let style = Style::new()
    ///     .border_style(normal_border())
    ///     .border_top(false)
    ///     .border_bottom(true);
    /// assert_eq!(style.get_vertical_border_size(), 1);
    /// ```
    pub fn get_vertical_border_size(&self) -> i32 {
        self.get_border_top_size() + self.get_border_bottom_size()
    }

    /// Gets the top border height.
    ///
    /// Returns `1` if the top border is enabled, `0` otherwise.
    ///
    /// # Returns
    ///
    /// Returns the top border height in lines (0 or 1).
    ///
    /// # Examples
    ///
    /// ```rust
    /// use lipgloss::{Style, normal_border};
    ///
    /// let style = Style::new()
    ///     .border_style(normal_border())
    ///     .border_top(true);
    /// assert_eq!(style.get_border_top_size(), 1);
    ///
    /// let style = Style::new()
    ///     .border_style(normal_border())
    ///     .border_top(false);
    /// assert_eq!(style.get_border_top_size(), 0);
    /// ```
    pub fn get_border_top_size(&self) -> i32 {
        if self.get_border_top() {
            1
        } else {
            0
        }
    }

    /// Gets the right border width.
    ///
    /// Returns `1` if the right border is enabled, `0` otherwise.
    ///
    /// # Returns
    ///
    /// Returns the right border width in characters (0 or 1).
    ///
    /// # Examples
    ///
    /// ```rust
    /// use lipgloss::{Style, normal_border};
    ///
    /// let style = Style::new()
    ///     .border_style(normal_border())
    ///     .border_right(true);
    /// assert_eq!(style.get_border_right_size(), 1);
    /// ```
    pub fn get_border_right_size(&self) -> i32 {
        if self.get_border_right() {
            1
        } else {
            0
        }
    }

    /// Gets the bottom border height.
    ///
    /// Returns `1` if the bottom border is enabled, `0` otherwise.
    ///
    /// # Returns
    ///
    /// Returns the bottom border height in lines (0 or 1).
    ///
    /// # Examples
    ///
    /// ```rust
    /// use lipgloss::{Style, normal_border};
    ///
    /// let style = Style::new()
    ///     .border_style(normal_border())
    ///     .border_bottom(true);
    /// assert_eq!(style.get_border_bottom_size(), 1);
    /// ```
    pub fn get_border_bottom_size(&self) -> i32 {
        if self.get_border_bottom() {
            1
        } else {
            0
        }
    }

    /// Gets the left border width.
    ///
    /// Returns `1` if the left border is enabled, `0` otherwise.
    ///
    /// # Returns
    ///
    /// Returns the left border width in characters (0 or 1).
    ///
    /// # Examples
    ///
    /// ```rust
    /// use lipgloss::{Style, normal_border};
    ///
    /// let style = Style::new()
    ///     .border_style(normal_border())
    ///     .border_left(true);
    /// assert_eq!(style.get_border_left_size(), 1);
    /// ```
    pub fn get_border_left_size(&self) -> i32 {
        if self.get_border_left() {
            1
        } else {
            0
        }
    }
}
