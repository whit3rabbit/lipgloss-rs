//! Color-related methods for Style
//!
//! This module provides methods for the [`Style`] struct to configure foreground and background
//! colors for text content, borders, and margins in terminal UI elements. Colors can be applied
//! to individual border sides or all sides at once, supporting various color formats including
//! ANSI colors, RGB values, and adaptive colors.
//!
//! # Color Types
//!
//! The methods in this module accept any type that implements the [`TerminalColor`] trait,
//! which includes:
//! - String literals and `String` for named colors
//! - ANSI color codes (0-255)
//! - RGB hex values (e.g., "#FF0000")
//! - [`crate::Color`] types for more advanced color handling
//!
//! # Examples
//!
//! ```rust
//! use lipgloss::Style;
//!
//! // Basic text colors
//! let style = Style::new()
//!     .foreground("red")
//!     .background("#0000FF");
//!
//! // Border-specific colors
//! let style = Style::new()
//!     .border_foreground("green")
//!     .border_background("yellow");
//!
//! // Individual border side colors
//! let style = Style::new()
//!     .border_top_foreground("cyan")
//!     .border_bottom_background("magenta");
//!
//! // Margin background color
//! let style = Style::new().margin_background("white");
//! ```

use crate::color::TerminalColor;
use crate::renderer::default_renderer;
use crate::style::{properties::*, Style};
use crate::utils::which_sides_color;

impl Style {
    /// Sets the foreground (text) color.
    ///
    /// This method sets the color that will be used for rendering text content.
    /// The color applies to all text within the styled element but does not
    /// affect borders or margins.
    ///
    /// # Arguments
    ///
    /// * `color` - Any type implementing [`TerminalColor`], such as color names,
    ///   hex values, ANSI codes, or [`crate::Color`] types
    ///
    /// # Returns
    ///
    /// Returns a new [`Style`] with the foreground color applied.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use lipgloss::Style;
    ///
    /// // Using color names
    /// let style = Style::new().foreground("red");
    ///
    /// // Using hex values
    /// let style = Style::new().foreground("#FF5733");
    ///
    /// // Using ANSI color codes
    /// let style = Style::new().foreground("196"); // Bright red
    /// ```
    pub fn foreground<C: TerminalColor>(mut self, color: C) -> Self {
        self.fg_color = Some(color.token(default_renderer()));
        self.set_prop(FOREGROUND_KEY);
        self
    }

    /// Sets the background color.
    ///
    /// This method sets the background color that will be used behind text content.
    /// The background color fills the entire content area but does not extend to
    /// borders or margins.
    ///
    /// # Arguments
    ///
    /// * `color` - Any type implementing [`TerminalColor`], such as color names,
    ///   hex values, ANSI codes, or [`crate::Color`] types
    ///
    /// # Returns
    ///
    /// Returns a new [`Style`] with the background color applied.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use lipgloss::Style;
    ///
    /// // Dark background with light text
    /// let style = Style::new()
    ///     .background("black")
    ///     .foreground("white");
    ///
    /// // Using RGB hex values
    /// let style = Style::new().background("#2E3440");
    /// ```
    pub fn background<C: TerminalColor>(mut self, color: C) -> Self {
        self.bg_color = Some(color.token(default_renderer()));
        self.set_prop(BACKGROUND_KEY);
        self
    }

    /// Sets the foreground color for the top border.
    ///
    /// This method sets the color used to render the top border characters.
    /// The border must be enabled using [`border_top`] or [`border`] for this
    /// color to be visible.
    ///
    /// # Arguments
    ///
    /// * `c` - Any type implementing [`TerminalColor`] for the border color
    ///
    /// # Returns
    ///
    /// Returns a new [`Style`] with the top border foreground color applied.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use lipgloss::Style;
    /// use lipgloss::normal_border;
    ///
    /// let style = Style::new()
    ///     .border_style(normal_border())
    ///     .border_top(true)
    ///     .border_top_foreground("blue");
    /// ```
    ///
    /// [`border_top`]: Self::border_top
    /// [`border`]: Self::border
    pub fn border_top_foreground<C: TerminalColor>(mut self, c: C) -> Self {
        self.border_top_fg_color = Some(c.token(default_renderer()));
        self.set_prop(BORDER_TOP_FOREGROUND_KEY);
        self
    }

    /// Sets the foreground color for the right border.
    ///
    /// This method sets the color used to render the right border characters.
    /// The border must be enabled using [`border_right`] or [`border`] for this
    /// color to be visible.
    ///
    /// # Arguments
    ///
    /// * `c` - Any type implementing [`TerminalColor`] for the border color
    ///
    /// # Returns
    ///
    /// Returns a new [`Style`] with the right border foreground color applied.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use lipgloss::Style;
    ///
    /// let style = Style::new()
    ///     .border_style(lipgloss::normal_border())
    ///     .border_right(true)
    ///     .border_right_foreground("green");
    /// ```
    ///
    /// [`border_right`]: Self::border_right
    /// [`border`]: Self::border
    pub fn border_right_foreground<C: TerminalColor>(mut self, c: C) -> Self {
        self.border_right_fg_color = Some(c.token(default_renderer()));
        self.set_prop(BORDER_RIGHT_FOREGROUND_KEY);
        self
    }

    /// Sets the foreground color for the bottom border.
    ///
    /// This method sets the color used to render the bottom border characters.
    /// The border must be enabled using [`border_bottom`] or [`border`] for this
    /// color to be visible.
    ///
    /// # Arguments
    ///
    /// * `c` - Any type implementing [`TerminalColor`] for the border color
    ///
    /// # Returns
    ///
    /// Returns a new [`Style`] with the bottom border foreground color applied.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use lipgloss::Style;
    ///
    /// let style = Style::new()
    ///     .border_style(lipgloss::normal_border())
    ///     .border_bottom(true)
    ///     .border_bottom_foreground("red");
    /// ```
    ///
    /// [`border_bottom`]: Self::border_bottom
    /// [`border`]: Self::border
    pub fn border_bottom_foreground<C: TerminalColor>(mut self, c: C) -> Self {
        self.border_bottom_fg_color = Some(c.token(default_renderer()));
        self.set_prop(BORDER_BOTTOM_FOREGROUND_KEY);
        self
    }

    /// Sets the foreground color for the left border.
    ///
    /// This method sets the color used to render the left border characters.
    /// The border must be enabled using [`border_left`] or [`border`] for this
    /// color to be visible.
    ///
    /// # Arguments
    ///
    /// * `c` - Any type implementing [`TerminalColor`] for the border color
    ///
    /// # Returns
    ///
    /// Returns a new [`Style`] with the left border foreground color applied.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use lipgloss::Style;
    ///
    /// let style = Style::new()
    ///     .border_style(lipgloss::normal_border())
    ///     .border_left(true)
    ///     .border_left_foreground("yellow");
    /// ```
    ///
    /// [`border_left`]: Self::border_left
    /// [`border`]: Self::border
    pub fn border_left_foreground<C: TerminalColor>(mut self, c: C) -> Self {
        self.border_left_fg_color = Some(c.token(default_renderer()));
        self.set_prop(BORDER_LEFT_FOREGROUND_KEY);
        self
    }

    /// Sets the background color for the top border.
    ///
    /// This method sets the background color behind the top border characters.
    /// The border must be enabled using [`border_top`] or [`border`] for this
    /// color to be visible.
    ///
    /// # Arguments
    ///
    /// * `c` - Any type implementing [`TerminalColor`] for the border background color
    ///
    /// # Returns
    ///
    /// Returns a new [`Style`] with the top border background color applied.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use lipgloss::Style;
    ///
    /// let style = Style::new()
    ///     .border_style(lipgloss::normal_border())
    ///     .border_top(true)
    ///     .border_top_background("lightblue");
    /// ```
    ///
    /// [`border_top`]: Self::border_top
    /// [`border`]: Self::border
    pub fn border_top_background<C: TerminalColor>(mut self, c: C) -> Self {
        self.border_top_bg_color = Some(c.token(default_renderer()));
        self.set_prop(BORDER_TOP_BACKGROUND_KEY);
        self
    }

    /// Sets the background color for the right border.
    ///
    /// This method sets the background color behind the right border characters.
    /// The border must be enabled using [`border_right`] or [`border`] for this
    /// color to be visible.
    ///
    /// # Arguments
    ///
    /// * `c` - Any type implementing [`TerminalColor`] for the border background color
    ///
    /// # Returns
    ///
    /// Returns a new [`Style`] with the right border background color applied.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use lipgloss::Style;
    ///
    /// let style = Style::new()
    ///     .border_style(lipgloss::normal_border())
    ///     .border_right(true)
    ///     .border_right_background("lightgreen");
    /// ```
    ///
    /// [`border_right`]: Self::border_right
    /// [`border`]: Self::border
    pub fn border_right_background<C: TerminalColor>(mut self, c: C) -> Self {
        self.border_right_bg_color = Some(c.token(default_renderer()));
        self.set_prop(BORDER_RIGHT_BACKGROUND_KEY);
        self
    }

    /// Sets the background color for the bottom border.
    ///
    /// This method sets the background color behind the bottom border characters.
    /// The border must be enabled using [`border_bottom`] or [`border`] for this
    /// color to be visible.
    ///
    /// # Arguments
    ///
    /// * `c` - Any type implementing [`TerminalColor`] for the border background color
    ///
    /// # Returns
    ///
    /// Returns a new [`Style`] with the bottom border background color applied.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use lipgloss::Style;
    ///
    /// let style = Style::new()
    ///     .border_style(lipgloss::normal_border())
    ///     .border_bottom(true)
    ///     .border_bottom_background("lightyellow");
    /// ```
    ///
    /// [`border_bottom`]: Self::border_bottom
    /// [`border`]: Self::border
    pub fn border_bottom_background<C: TerminalColor>(mut self, c: C) -> Self {
        self.border_bottom_bg_color = Some(c.token(default_renderer()));
        self.set_prop(BORDER_BOTTOM_BACKGROUND_KEY);
        self
    }

    /// Sets the background color for the left border.
    ///
    /// This method sets the background color behind the left border characters.
    /// The border must be enabled using [`border_left`] or [`border`] for this
    /// color to be visible.
    ///
    /// # Arguments
    ///
    /// * `c` - Any type implementing [`TerminalColor`] for the border background color
    ///
    /// # Returns
    ///
    /// Returns a new [`Style`] with the left border background color applied.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use lipgloss::Style;
    ///
    /// let style = Style::new()
    ///     .border_style(lipgloss::normal_border())
    ///     .border_left(true)
    ///     .border_left_background("lightcyan");
    /// ```
    ///
    /// [`border_left`]: Self::border_left
    /// [`border`]: Self::border
    pub fn border_left_background<C: TerminalColor>(mut self, c: C) -> Self {
        self.border_left_bg_color = Some(c.token(default_renderer()));
        self.set_prop(BORDER_LEFT_BACKGROUND_KEY);
        self
    }

    /// Sets the foreground color for all border edges.
    ///
    /// This is a convenience method that applies the same foreground color to all
    /// four border sides (top, right, bottom, left) in a single operation. This is
    /// more efficient than calling each individual border foreground method separately.
    ///
    /// # Arguments
    ///
    /// * `c` - Any type implementing [`TerminalColor`] + [`Clone`] for the border color.
    ///   The `Clone` trait is required because the color is applied to multiple sides.
    ///
    /// # Returns
    ///
    /// Returns a new [`Style`] with the foreground color applied to all border sides.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use lipgloss::Style;
    ///
    /// // Apply red foreground to all border sides
    /// let style = Style::new()
    ///     .border(lipgloss::normal_border())
    ///     .border_foreground("red");
    ///
    /// // Equivalent to calling each side individually:
    /// let style = Style::new()
    ///     .border(lipgloss::normal_border())
    ///     .border_top_foreground("red")
    ///     .border_right_foreground("red")
    ///     .border_bottom_foreground("red")
    ///     .border_left_foreground("red");
    /// ```
    pub fn border_foreground<C: TerminalColor + Clone>(self, c: C) -> Self {
        self.border_top_foreground(c.clone())
            .border_right_foreground(c.clone())
            .border_bottom_foreground(c.clone())
            .border_left_foreground(c)
    }

    /// Sets the background color for all border edges.
    ///
    /// This is a convenience method that applies the same background color to all
    /// four border sides (top, right, bottom, left) in a single operation. This is
    /// more efficient than calling each individual border background method separately.
    ///
    /// # Arguments
    ///
    /// * `c` - Any type implementing [`TerminalColor`] + [`Clone`] for the border background color.
    ///   The `Clone` trait is required because the color is applied to multiple sides.
    ///
    /// # Returns
    ///
    /// Returns a new [`Style`] with the background color applied to all border sides.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use lipgloss::{Style, Border};
    ///
    /// // Apply blue background to all border sides
    /// let style = Style::new()
    ///     .border(lipgloss::normal_border())
    ///     .border_background("blue");
    ///
    /// // Equivalent to calling each side individually:
    /// let style = Style::new()
    ///     .border(lipgloss::normal_border())
    ///     .border_top_background("blue")
    ///     .border_right_background("blue")
    ///     .border_bottom_background("blue")
    ///     .border_left_background("blue");
    /// ```
    pub fn border_background<C: TerminalColor + Clone>(self, c: C) -> Self {
        self.border_top_background(c.clone())
            .border_right_background(c.clone())
            .border_bottom_background(c.clone())
            .border_left_background(c)
    }

    /// Sets the background color for margins.
    ///
    /// This method sets the background color that will be used in the margin areas
    /// around the styled content. Margins are the outermost space around an element,
    /// outside of any borders and padding. The margin background color is only visible
    /// when margins are applied using methods like [`margin`] or [`margin_top`].
    ///
    /// # Arguments
    ///
    /// * `c` - Any type implementing [`TerminalColor`] for the margin background color
    ///
    /// # Returns
    ///
    /// Returns a new [`Style`] with the margin background color applied.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use lipgloss::Style;
    ///
    /// // Create a style with margins and margin background color
    /// let style = Style::new()
    ///     .margin_background("#444444")
    ///     .margin(2, 0, 2, 0);
    /// // The result will have a light gray margin area around white content
    /// let rendered = style.render("Content");
    /// ```
    ///
    /// [`margin`]: Self::margin
    /// [`margin_top`]: Self::margin_top
    pub fn margin_background<C: TerminalColor>(mut self, c: C) -> Self {
        self.margin_bg_color = Some(c.token(default_renderer()));
        self.set_prop(MARGIN_BACKGROUND_KEY);
        self
    }

    /// Sets border foreground colors using CSS-style shorthand notation.
    ///
    /// This method accepts 1-4 color values and applies them using CSS shorthand rules:
    /// - 1 value: applies to all sides
    /// - 2 values: first is top/bottom, second is left/right
    /// - 3 values: first is top, second is left/right, third is bottom
    /// - 4 values: top, right, bottom, left (clockwise from top)
    ///
    /// Invalid input (0 or 5+ values) will be ignored and return the style unchanged.
    ///
    /// # Arguments
    ///
    /// * `colors` - Slice of 1-4 color values following CSS shorthand rules
    ///
    /// # Returns
    ///
    /// Returns a new [`Style`] with border foreground colors applied.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use lipgloss::{Style, normal_border};
    ///
    /// // All sides red
    /// let uniform = Style::new()
    ///     .border_style(normal_border())
    ///     .border_foreground_shorthand(&["red"]);
    ///
    /// // Top/bottom blue, left/right green
    /// let vertical_horizontal = Style::new()
    ///     .border_style(normal_border())
    ///     .border_foreground_shorthand(&["blue", "green"]);
    ///
    /// // Top red, left/right yellow, bottom cyan
    /// let three_colors = Style::new()
    ///     .border_style(normal_border())
    ///     .border_foreground_shorthand(&["red", "yellow", "cyan"]);
    ///
    /// // Clockwise from top: red, green, blue, yellow
    /// let clockwise = Style::new()
    ///     .border_style(normal_border())
    ///     .border_foreground_shorthand(&["red", "green", "blue", "yellow"]);
    /// ```
    ///
    /// # See Also
    ///
    /// - [`border_foreground`] - Sets all border sides to the same color
    /// - [`border_top_foreground`], [`border_right_foreground`], etc. - Individual side colors
    pub fn border_foreground_shorthand<C: TerminalColor + Clone>(self, colors: &[C]) -> Self {
        let result = which_sides_color(colors);
        let (top, right, bottom, left, ok) = result;

        if !ok {
            return self;
        }

        self.border_top_foreground(top)
            .border_right_foreground(right)
            .border_bottom_foreground(bottom)
            .border_left_foreground(left)
    }

    /// Sets border background colors using CSS-style shorthand notation.
    ///
    /// This method accepts 1-4 color values and applies them using CSS shorthand rules:
    /// - 1 value: applies to all sides
    /// - 2 values: first is top/bottom, second is left/right
    /// - 3 values: first is top, second is left/right, third is bottom
    /// - 4 values: top, right, bottom, left (clockwise from top)
    ///
    /// Invalid input (0 or 5+ values) will be ignored and return the style unchanged.
    ///
    /// # Arguments
    ///
    /// * `colors` - Slice of 1-4 color values following CSS shorthand rules
    ///
    /// # Returns
    ///
    /// Returns a new [`Style`] with border background colors applied.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use lipgloss::{Style, normal_border};
    ///
    /// // All sides light gray background
    /// let uniform = Style::new()
    ///     .border_style(normal_border())
    ///     .border_background_shorthand(&["lightgray"]);
    ///
    /// // Top/bottom white, left/right black
    /// let contrast = Style::new()
    ///     .border_style(normal_border())
    ///     .border_background_shorthand(&["white", "black"]);
    ///
    /// // Individual colors for each side
    /// let rainbow = Style::new()
    ///     .border_style(normal_border())
    ///     .border_background_shorthand(&["#FF0000", "#00FF00", "#0000FF", "#FFFF00"]);
    /// ```
    ///
    /// # See Also
    ///
    /// - [`border_background`] - Sets all border sides to the same background color
    /// - [`border_top_background`], [`border_right_background`], etc. - Individual side colors
    pub fn border_background_shorthand<C: TerminalColor + Clone>(self, colors: &[C]) -> Self {
        let result = which_sides_color(colors);
        let (top, right, bottom, left, ok) = result;

        if !ok {
            return self;
        }

        self.border_top_background(top)
            .border_right_background(right)
            .border_bottom_background(bottom)
            .border_left_background(left)
    }
}
