//! Border-related methods for Style
//!
//! This module provides methods for the [`Style`] struct to configure and manage borders
//! in terminal UI elements. Borders can be applied to individual sides (top, right, bottom, left)
//! or all sides at once, using various predefined or custom border styles.
//!
//! # Examples
//!
//! ```rust,no_run
//! use lipgloss::Style;
//! use lipgloss::{normal_border, rounded_border, thick_border};
//!
//! // Apply a border to all sides
//! let style = Style::new().border(normal_border());
//!
//! // Apply border style without enabling sides
//! let style = Style::new()
//!     .border_style(rounded_border())
//!     .border_top(true)
//!     .border_bottom(true);
//!
//! // Selectively enable border sides
//! let style = Style::new()
//!     .border_style(thick_border())
//!     .border_left(true)
//!     .border_right(false);
//! ```

use crate::border::Border;
use crate::style::{properties::*, Style};
use crate::utils::which_sides_bool;

impl Style {
    /// Sets a border style and enables all border sides.
    ///
    /// This is a convenience method that both sets the border style and enables
    /// all four border sides (top, right, bottom, left) in one operation.
    /// Use this when you want a complete border around your content.
    ///
    /// # Arguments
    ///
    /// * `b` - The [`Border`] style to apply
    ///
    /// # Returns
    ///
    /// Returns a new [`Style`] with the border style set and all sides enabled.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use lipgloss::Style;
    /// use lipgloss::normal_border;
    ///
    /// let style = Style::new().border(normal_border());
    /// let text = style.render("Hello, World!");
    /// ```
    ///
    /// For more control over individual border sides, use [`border_style`] followed
    /// by individual side methods like [`border_top`], [`border_right`], etc.
    ///
    /// [`border_style`]: Self::border_style
    /// [`border_top`]: Self::border_top
    /// [`border_right`]: Self::border_right
    pub fn border(mut self, b: Border) -> Self {
        self.border_style = b;
        self.set_prop(BORDER_STYLE_KEY);

        // Enable all border sides
        self.set_attr(ATTR_BORDER_TOP, true);
        self.set_attr(ATTR_BORDER_RIGHT, true);
        self.set_attr(ATTR_BORDER_BOTTOM, true);
        self.set_attr(ATTR_BORDER_LEFT, true);
        self.set_prop(BORDER_TOP_KEY);
        self.set_prop(BORDER_RIGHT_KEY);
        self.set_prop(BORDER_BOTTOM_KEY);
        self.set_prop(BORDER_LEFT_KEY);

        self
    }

    /// Sets a border style without enabling any border sides.
    ///
    /// This method only sets the border style that will be used when borders are
    /// enabled, but doesn't actually enable any border sides. Use this when you
    /// want to define the border appearance first, then selectively enable specific
    /// sides using methods like [`border_top`], [`border_right`], etc.
    ///
    /// # Arguments
    ///
    /// * `b` - The [`Border`] style to use for rendering borders
    ///
    /// # Returns
    ///
    /// Returns a new [`Style`] with the border style configured.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use lipgloss::Style;
    /// use lipgloss::rounded_border;
    ///
    /// // Set border style, then enable only top and bottom
    /// let style = Style::new()
    ///     .border_style(rounded_border())
    ///     .border_top(true)
    ///     .border_bottom(true);
    /// ```
    ///
    /// [`border_top`]: Self::border_top
    /// [`border_right`]: Self::border_right
    pub fn border_style(self, b: Border) -> Self {
        let mut s = self;
        s.border_style = b;
        s.set_prop(BORDER_STYLE_KEY);
        s
    }

    /// Enables or disables the top border.
    ///
    /// Controls whether the top border is rendered when the style is applied.
    /// The border style must be set using [`border`] or [`border_style`] for
    /// this to have a visual effect.
    ///
    /// # Arguments
    ///
    /// * `v` - `true` to enable the top border, `false` to disable it
    ///
    /// # Returns
    ///
    /// Returns a new [`Style`] with the top border setting applied.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use lipgloss::Style;
    /// use lipgloss::normal_border;
    ///
    /// // Enable top border only
    /// let style = Style::new()
    ///     .border_style(normal_border())
    ///     .border_top(true);
    ///
    /// // Disable top border (useful when modifying existing styles)
    /// let existing_style = Style::new().border_style(normal_border());
    /// let style = existing_style.border_top(false);
    /// ```
    ///
    /// [`border`]: Self::border
    /// [`border_style`]: Self::border_style
    pub fn border_top(mut self, v: bool) -> Self {
        self.set_attr(ATTR_BORDER_TOP, v);
        self.set_prop(BORDER_TOP_KEY);
        self
    }

    /// Enables or disables the right border.
    ///
    /// Controls whether the right border is rendered when the style is applied.
    /// The border style must be set using [`border`] or [`border_style`] for
    /// this to have a visual effect.
    ///
    /// # Arguments
    ///
    /// * `v` - `true` to enable the right border, `false` to disable it
    ///
    /// # Returns
    ///
    /// Returns a new [`Style`] with the right border setting applied.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use lipgloss::Style;
    /// use lipgloss::normal_border;
    ///
    /// // Enable right border only
    /// let style = Style::new()
    ///     .border_style(normal_border())
    ///     .border_right(true);
    ///
    /// // Disable right border
    /// let existing_style = Style::new().border_style(normal_border());
    /// let style = existing_style.border_right(false);
    /// ```
    ///
    /// [`border`]: Self::border
    /// [`border_style`]: Self::border_style
    pub fn border_right(mut self, v: bool) -> Self {
        self.set_attr(ATTR_BORDER_RIGHT, v);
        self.set_prop(BORDER_RIGHT_KEY);
        self
    }

    /// Enables or disables the bottom border.
    ///
    /// Controls whether the bottom border is rendered when the style is applied.
    /// The border style must be set using [`border`] or [`border_style`] for
    /// this to have a visual effect.
    ///
    /// # Arguments
    ///
    /// * `v` - `true` to enable the bottom border, `false` to disable it
    ///
    /// # Returns
    ///
    /// Returns a new [`Style`] with the bottom border setting applied.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use lipgloss::Style;
    /// use lipgloss::normal_border;
    ///
    /// // Enable bottom border only
    /// let style = Style::new()
    ///     .border_style(normal_border())
    ///     .border_bottom(true);
    ///
    /// // Disable bottom border
    /// let existing_style = Style::new().border_style(normal_border());
    /// let style = existing_style.border_bottom(false);
    /// ```
    ///
    /// [`border`]: Self::border
    /// [`border_style`]: Self::border_style
    pub fn border_bottom(mut self, v: bool) -> Self {
        self.set_attr(ATTR_BORDER_BOTTOM, v);
        self.set_prop(BORDER_BOTTOM_KEY);
        self
    }

    /// Enables or disables the left border.
    ///
    /// Controls whether the left border is rendered when the style is applied.
    /// The border style must be set using [`border`] or [`border_style`] for
    /// this to have a visual effect.
    ///
    /// # Arguments
    ///
    /// * `v` - `true` to enable the left border, `false` to disable it
    ///
    /// # Returns
    ///
    /// Returns a new [`Style`] with the left border setting applied.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use lipgloss::Style;
    /// use lipgloss::normal_border;
    ///
    /// // Enable left border only
    /// let style = Style::new()
    ///     .border_style(normal_border())
    ///     .border_left(true);
    ///
    /// // Disable left border
    /// let existing_style = Style::new().border_style(normal_border());
    /// let style = existing_style.border_left(false);
    /// ```
    ///
    /// [`border`]: Self::border
    /// [`border_style`]: Self::border_style
    pub fn border_left(mut self, v: bool) -> Self {
        self.set_attr(ATTR_BORDER_LEFT, v);
        self.set_prop(BORDER_LEFT_KEY);
        self
    }

    /// Sets a border style with selective side control using CSS-style shorthand.
    ///
    /// This method sets the border style and allows selective enabling of border sides
    /// using CSS-style shorthand notation:
    /// - 1 value: applies to all sides
    /// - 2 values: first is top/bottom, second is left/right
    /// - 3 values: first is top, second is left/right, third is bottom  
    /// - 4 values: top, right, bottom, left (clockwise from top)
    ///
    /// Invalid input (0 or 5+ values) will enable all sides as a fallback.
    ///
    /// # Arguments
    ///
    /// * `border` - The [`Border`] style to apply
    /// * `sides` - Slice of 1-4 boolean values indicating which sides to enable
    ///
    /// # Returns
    ///
    /// Returns a new [`Style`] with the border style and selective sides configured.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use lipgloss::{Style, normal_border};
    ///
    /// // Enable all sides (equivalent to border())
    /// let all_sides = Style::new().border_with_sides(normal_border(), &[true]);
    ///
    /// // Enable top/bottom only
    /// let vertical = Style::new().border_with_sides(normal_border(), &[true, false]);
    ///
    /// // Top enabled, left/right disabled, bottom enabled
    /// let top_bottom = Style::new().border_with_sides(normal_border(), &[true, false, true]);
    ///
    /// // Clockwise from top: top=true, right=false, bottom=true, left=false
    /// let selective = Style::new().border_with_sides(normal_border(), &[true, false, true, false]);
    ///
    /// // Invalid input defaults to all sides enabled
    /// let fallback = Style::new().border_with_sides(normal_border(), &[]);
    /// ```
    ///
    /// # See Also
    ///
    /// - [`Self::border`] - Sets border style and enables all sides
    /// - [`Self::border_style`] - Sets border style without enabling sides
    /// - [`Self::border_top`], [`Self::border_right`], [`Self::border_bottom`], [`Self::border_left`] - Individual side control
    pub fn border_with_sides(mut self, border: Border, sides: &[bool]) -> Self {
        self.border_style = border;
        self.set_prop(BORDER_STYLE_KEY);

        let (top, right, bottom, left, ok) = which_sides_bool(sides);

        // If invalid input, default to all sides enabled (Go behavior)
        let (top, right, bottom, left) = if ok {
            (top, right, bottom, left)
        } else {
            (true, true, true, true)
        };

        self.set_attr(ATTR_BORDER_TOP, top);
        self.set_attr(ATTR_BORDER_RIGHT, right);
        self.set_attr(ATTR_BORDER_BOTTOM, bottom);
        self.set_attr(ATTR_BORDER_LEFT, left);
        self.set_prop(BORDER_TOP_KEY);
        self.set_prop(BORDER_RIGHT_KEY);
        self.set_prop(BORDER_BOTTOM_KEY);
        self.set_prop(BORDER_LEFT_KEY);

        self
    }
}
