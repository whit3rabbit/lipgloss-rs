//! Unset methods for Style
//!
//! This module provides methods for removing or resetting style properties back to their
//! default values. These methods are essential for style composition and dynamic styling
//! where you need to selectively remove certain styling attributes while preserving others.
//!
//! All unset methods follow a consistent pattern:
//! - They reset the internal property to its default value
//! - They remove the property from the style's property set
//! - They return `Self` for method chaining
//!
//! # Categories of Unset Methods
//!
//! - **Text Attributes**: `unset_bold()`, `unset_italic()`, `unset_underline()`, etc.
//! - **Colors**: `unset_foreground()`, `unset_background()`
//! - **Sizing**: `unset_width()`, `unset_height()`, `unset_max_width()`, etc.
//! - **Alignment**: `unset_align()`, `unset_align_horizontal()`, `unset_align_vertical()`
//! - **Spacing**: `unset_padding()`, `unset_margins()` and individual edge methods
//! - **Borders**: `unset_border_style()` and individual border methods
//! - **Other**: `unset_tab_width()`, `unset_transform()`, `unset_string()`
//!
//! # Examples
//!
//! ```rust
//! use lipgloss::Style;
//!
//! // Create a heavily styled style
//! let mut style = Style::new()
//!     .bold(true)
//!     .italic(true)
//!     .foreground("red")
//!     .background("blue")
//!     .padding(2, 2, 2, 2)
//!     .margin(1, 1, 1, 1);
//!
//! // Selectively remove some attributes
//! style = style
//!     .unset_bold()           // Remove bold
//!     .unset_background()     // Remove background color
//!     .unset_padding();       // Remove all padding
//!
//! // The style now only has italic, red foreground, and margin
//! let result = style.render("Hello World");
//! ```
//!
//! # Style Inheritance and Composition
//!
//! Unset methods are particularly useful when working with style inheritance:
//!
//! ```rust
//! use lipgloss::Style;
//!
//! // Base style for all text
//! let base_style = Style::new()
//!     .bold(true)
//!     .foreground("blue")
//!     .padding(1, 1, 1, 1);
//!
//! // Create a variant that removes certain properties
//! let subtitle_style = base_style
//!     .unset_bold()           // Subtitles shouldn't be bold
//!     .italic(true)           // But they should be italic
//!     .unset_padding();       // And have no padding
//! ```

use crate::border::hidden_border;
use crate::position::{LEFT, TOP};
use crate::style::{properties::*, Style};

impl Style {
    // ---------- Text attribute unset methods ----------

    /// Removes the bold text attribute from this style.
    ///
    /// This resets the bold setting to its default (false) and removes it from the
    /// style's property set. Text rendered with this style will no longer be bold.
    ///
    /// # Returns
    ///
    /// Returns the modified `Style` instance for method chaining.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use lipgloss::Style;
    ///
    /// let style = Style::new()
    ///     .bold(true)
    ///     .italic(true)
    ///     .unset_bold();  // Remove bold, keep italic
    ///
    /// // Text will be italic but not bold
    /// let result = style.render("Hello World");
    /// ```
    pub fn unset_bold(mut self) -> Self {
        self.unset_prop(BOLD_KEY);
        self.set_attr(ATTR_BOLD, false);
        self
    }

    /// Removes the italic text attribute from this style.
    ///
    /// This resets the italic setting to its default (false) and removes it from the
    /// style's property set. Text rendered with this style will no longer be italic.
    ///
    /// # Returns
    ///
    /// Returns the modified `Style` instance for method chaining.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use lipgloss::Style;
    ///
    /// let style = Style::new()
    ///     .italic(true)
    ///     .bold(true)
    ///     .unset_italic();  // Remove italic, keep bold
    ///
    /// // Text will be bold but not italic
    /// let result = style.render("Hello World");
    /// ```
    pub fn unset_italic(mut self) -> Self {
        self.unset_prop(ITALIC_KEY);
        self.set_attr(ATTR_ITALIC, false);
        self
    }

    /// Removes the underline text attribute from this style.
    ///
    /// This resets the underline setting to its default (false) and removes it from the
    /// style's property set. Text rendered with this style will no longer be underlined.
    ///
    /// # Returns
    ///
    /// Returns the modified `Style` instance for method chaining.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use lipgloss::Style;
    ///
    /// let style = Style::new()
    ///     .underline(true)
    ///     .bold(true)
    ///     .unset_underline();  // Remove underline, keep bold
    ///
    /// // Text will be bold but not underlined
    /// let result = style.render("Hello World");
    /// ```
    pub fn unset_underline(mut self) -> Self {
        self.unset_prop(UNDERLINE_KEY);
        self.set_attr(ATTR_UNDERLINE, false);
        self
    }

    /// Removes the strikethrough text attribute from this style.
    ///
    /// This resets the strikethrough setting to its default (false) and removes it from the
    /// style's property set. Text rendered with this style will no longer have strikethrough.
    ///
    /// # Returns
    ///
    /// Returns the modified `Style` instance for method chaining.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use lipgloss::Style;
    ///
    /// let style = Style::new()
    ///     .strikethrough(true)
    ///     .italic(true)
    ///     .unset_strikethrough();  // Remove strikethrough, keep italic
    ///
    /// // Text will be italic but not struck through
    /// let result = style.render("Hello World");
    /// ```
    pub fn unset_strikethrough(mut self) -> Self {
        self.unset_prop(STRIKETHROUGH_KEY);
        self.set_attr(ATTR_STRIKETHROUGH, false);
        self
    }

    /// Removes the reverse video text attribute from this style.
    ///
    /// This resets the reverse setting to its default (false) and removes it from the
    /// style's property set. Text rendered with this style will no longer have reversed
    /// foreground and background colors.
    ///
    /// # Returns
    ///
    /// Returns the modified `Style` instance for method chaining.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use lipgloss::Style;
    ///
    /// let style = Style::new()
    ///     .reverse(true)
    ///     .bold(true)
    ///     .unset_reverse();  // Remove reverse, keep bold
    ///
    /// // Text will be bold but colors won't be reversed
    /// let result = style.render("Hello World");
    /// ```
    pub fn unset_reverse(mut self) -> Self {
        self.unset_prop(REVERSE_KEY);
        self.set_attr(ATTR_REVERSE, false);
        self
    }

    /// Removes the blink text attribute from this style.
    ///
    /// This resets the blink setting to its default (false) and removes it from the
    /// style's property set. Text rendered with this style will no longer blink.
    ///
    /// # Returns
    ///
    /// Returns the modified `Style` instance for method chaining.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use lipgloss::Style;
    ///
    /// let style = Style::new()
    ///     .blink(true)
    ///     .bold(true)
    ///     .unset_blink();  // Remove blink, keep bold
    ///
    /// // Text will be bold but won't blink
    /// let result = style.render("Hello World");
    /// ```
    pub fn unset_blink(mut self) -> Self {
        self.unset_prop(BLINK_KEY);
        self.set_attr(ATTR_BLINK, false);
        self
    }

    /// Removes the faint (dim) text attribute from this style.
    ///
    /// This resets the faint setting to its default (false) and removes it from the
    /// style's property set. Text rendered with this style will no longer be dimmed.
    ///
    /// # Returns
    ///
    /// Returns the modified `Style` instance for method chaining.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use lipgloss::Style;
    ///
    /// let style = Style::new()
    ///     .faint(true)
    ///     .italic(true)
    ///     .unset_faint();  // Remove faint, keep italic
    ///
    /// // Text will be italic but at normal brightness
    /// let result = style.render("Hello World");
    /// ```
    pub fn unset_faint(mut self) -> Self {
        self.unset_prop(FAINT_KEY);
        self.set_attr(ATTR_FAINT, false);
        self
    }

    /// Removes the underline spaces attribute from this style.
    ///
    /// When disabled, spaces within underlined text will no longer be underlined.
    /// This allows for more precise control over underline appearance.
    ///
    /// # Returns
    ///
    /// Returns the modified `Style` instance for method chaining.
    pub fn unset_underline_spaces(mut self) -> Self {
        self.unset_prop(UNDERLINE_SPACES_KEY);
        self.set_attr(ATTR_UNDERLINE_SPACES, false);
        self
    }

    /// Removes the strikethrough spaces attribute from this style.
    ///
    /// When disabled, spaces within strikethrough text will no longer be struck through.
    /// This allows for more precise control over strikethrough appearance.
    ///
    /// # Returns
    ///
    /// Returns the modified `Style` instance for method chaining.
    pub fn unset_strikethrough_spaces(mut self) -> Self {
        self.unset_prop(STRIKETHROUGH_SPACES_KEY);
        self.set_attr(ATTR_STRIKETHROUGH_SPACES, false);
        self
    }

    /// Removes the color whitespace attribute from this style.
    ///
    /// When disabled, whitespace characters (spaces, tabs) will not be colored
    /// with the foreground/background colors of this style.
    ///
    /// # Returns
    ///
    /// Returns the modified `Style` instance for method chaining.
    pub fn unset_color_whitespace(mut self) -> Self {
        self.unset_prop(COLOR_WHITESPACE_KEY);
        self.set_attr(ATTR_COLOR_WHITESPACE, false);
        self
    }

    /// Removes the inline rendering attribute from this style.
    ///
    /// When disabled, the style will revert to block-level rendering behavior,
    /// which may include margins, padding, and borders affecting layout.
    ///
    /// # Returns
    ///
    /// Returns the modified `Style` instance for method chaining.
    pub fn unset_inline(mut self) -> Self {
        self.unset_prop(INLINE_KEY);
        self.set_attr(ATTR_INLINE, false);
        self
    }

    // ---------- Color unset methods ----------

    /// Removes the foreground color from this style.
    ///
    /// This resets the foreground color to `None` and removes it from the style's property set.
    /// Text rendered with this style will use the terminal's default foreground color.
    ///
    /// # Returns
    ///
    /// Returns the modified `Style` instance for method chaining.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use lipgloss::Style;
    ///
    /// let style = Style::new()
    ///     .foreground("red")
    ///     .background("blue")
    ///     .unset_foreground();  // Remove red foreground, keep blue background
    ///
    /// // Text will have blue background but default foreground color
    /// let result = style.render("Hello World");
    /// ```
    pub fn unset_foreground(mut self) -> Self {
        self.unset_prop(FOREGROUND_KEY);
        self.fg_color = None;
        self
    }

    /// Removes the background color from this style.
    ///
    /// This resets the background color to `None` and removes it from the style's property set.
    /// Text rendered with this style will use the terminal's default background color.
    ///
    /// # Returns
    ///
    /// Returns the modified `Style` instance for method chaining.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use lipgloss::{Style, Color};
    ///
    /// let style = Style::new()
    ///     .foreground("red")
    ///     .background("blue")
    ///     .unset_background();  // Remove blue background, keep red foreground
    ///
    /// // Text will have red foreground but default background color
    /// let result = style.render("Hello World");
    /// ```
    pub fn unset_background(mut self) -> Self {
        self.unset_prop(BACKGROUND_KEY);
        self.bg_color = None;
        self
    }

    // ---------- Size unset methods ----------

    /// Removes the width constraint from this style.
    ///
    /// This resets the width to 0 (unconstrained) and removes it from the style's property set.
    /// Text rendered with this style will use its natural width.
    ///
    /// # Returns
    ///
    /// Returns the modified `Style` instance for method chaining.
    pub fn unset_width(mut self) -> Self {
        self.unset_prop(WIDTH_KEY);
        self.width = 0;
        self
    }

    /// Removes the height constraint from this style.
    ///
    /// This resets the height to 0 (unconstrained) and removes it from the style's property set.
    /// Text rendered with this style will use its natural height.
    ///
    /// # Returns
    ///
    /// Returns the modified `Style` instance for method chaining.
    pub fn unset_height(mut self) -> Self {
        self.unset_prop(HEIGHT_KEY);
        self.height = 0;
        self
    }

    /// Removes the maximum width constraint from this style.
    ///
    /// This resets the max width to 0 (unconstrained) and removes it from the style's property set.
    /// Text rendered with this style will not be limited by a maximum width.
    ///
    /// # Returns
    ///
    /// Returns the modified `Style` instance for method chaining.
    pub fn unset_max_width(mut self) -> Self {
        self.unset_prop(MAX_WIDTH_KEY);
        self.max_width = 0;
        self
    }

    /// Removes the maximum height constraint from this style.
    ///
    /// This resets the max height to 0 (unconstrained) and removes it from the style's property set.
    /// Text rendered with this style will not be limited by a maximum height.
    ///
    /// # Returns
    ///
    /// Returns the modified `Style` instance for method chaining.
    pub fn unset_max_height(mut self) -> Self {
        self.unset_prop(MAX_HEIGHT_KEY);
        self.max_height = 0;
        self
    }

    // ---------- Alignment unset methods ----------

    /// Removes both horizontal and vertical alignment settings from this style.
    ///
    /// This resets both alignments to their defaults (LEFT and TOP) and removes them
    /// from the style's property set. Text will be aligned to the top-left.
    ///
    /// # Returns
    ///
    /// Returns the modified `Style` instance for method chaining.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use lipgloss::Style;
    /// use lipgloss::position::CENTER;
    ///
    /// let style = Style::new()
    ///     .align_horizontal(CENTER)
    ///     .align_vertical(CENTER)
    ///     .unset_align();  // Reset both to defaults
    ///
    /// // Text will now be aligned to top-left
    /// let result = style.render("Hello World");
    /// ```
    pub fn unset_align(mut self) -> Self {
        self.unset_prop(ALIGN_HORIZONTAL_KEY);
        self.unset_prop(ALIGN_VERTICAL_KEY);
        self.align_horizontal = LEFT;
        self.align_vertical = TOP;
        self
    }

    /// Removes the horizontal alignment setting from this style.
    ///
    /// This resets horizontal alignment to its default (LEFT) and removes it
    /// from the style's property set.
    ///
    /// # Returns
    ///
    /// Returns the modified `Style` instance for method chaining.
    pub fn unset_align_horizontal(mut self) -> Self {
        self.unset_prop(ALIGN_HORIZONTAL_KEY);
        self.align_horizontal = LEFT;
        self
    }

    /// Removes the vertical alignment setting from this style.
    ///
    /// This resets vertical alignment to its default (TOP) and removes it
    /// from the style's property set.
    ///
    /// # Returns
    ///
    /// Returns the modified `Style` instance for method chaining.
    pub fn unset_align_vertical(mut self) -> Self {
        self.unset_prop(ALIGN_VERTICAL_KEY);
        self.align_vertical = TOP;
        self
    }

    // ---------- Padding unset methods ----------

    /// Removes all padding from this style.
    ///
    /// This resets all padding values (top, right, bottom, left) to 0 and removes
    /// them from the style's property set.
    ///
    /// # Returns
    ///
    /// Returns the modified `Style` instance for method chaining.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use lipgloss::Style;
    ///
    /// let style = Style::new()
    ///     .padding(2, 2, 2, 2)
    ///     .margin(1, 1, 1, 1)
    ///     .unset_padding();  // Remove all padding, keep margin
    ///
    /// let result = style.render("Hello World");
    /// ```
    pub fn unset_padding(mut self) -> Self {
        self.unset_prop(PADDING_TOP_KEY);
        self.unset_prop(PADDING_RIGHT_KEY);
        self.unset_prop(PADDING_BOTTOM_KEY);
        self.unset_prop(PADDING_LEFT_KEY);
        self.padding_top = 0;
        self.padding_right = 0;
        self.padding_bottom = 0;
        self.padding_left = 0;
        self
    }

    /// Removes the top padding from this style.
    ///
    /// # Returns
    ///
    /// Returns the modified `Style` instance for method chaining.
    pub fn unset_padding_top(mut self) -> Self {
        self.unset_prop(PADDING_TOP_KEY);
        self.padding_top = 0;
        self
    }

    /// Removes the right padding from this style.
    ///
    /// # Returns
    ///
    /// Returns the modified `Style` instance for method chaining.
    pub fn unset_padding_right(mut self) -> Self {
        self.unset_prop(PADDING_RIGHT_KEY);
        self.padding_right = 0;
        self
    }

    /// Removes the bottom padding from this style.
    ///
    /// # Returns
    ///
    /// Returns the modified `Style` instance for method chaining.
    pub fn unset_padding_bottom(mut self) -> Self {
        self.unset_prop(PADDING_BOTTOM_KEY);
        self.padding_bottom = 0;
        self
    }

    /// Removes the left padding from this style.
    ///
    /// # Returns
    ///
    /// Returns the modified `Style` instance for method chaining.
    pub fn unset_padding_left(mut self) -> Self {
        self.unset_prop(PADDING_LEFT_KEY);
        self.padding_left = 0;
        self
    }

    // ---------- Margin unset methods ----------

    /// Removes all margins from this style.
    ///
    /// This resets all margin values (top, right, bottom, left) to 0, removes the
    /// margin background color, and removes them from the style's property set.
    ///
    /// # Returns
    ///
    /// Returns the modified `Style` instance for method chaining.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use lipgloss::Style;
    ///
    /// let style = Style::new()
    ///     .margin(2, 2, 2, 2)
    ///     .margin_background("blue")
    ///     .padding(1, 1, 1, 1)
    ///     .unset_margins();  // Remove all margins, keep padding
    ///
    /// let result = style.render("Hello World");
    /// ```
    pub fn unset_margins(mut self) -> Self {
        self.unset_prop(MARGIN_TOP_KEY);
        self.unset_prop(MARGIN_RIGHT_KEY);
        self.unset_prop(MARGIN_BOTTOM_KEY);
        self.unset_prop(MARGIN_LEFT_KEY);
        self.unset_prop(MARGIN_BACKGROUND_KEY);
        self.margin_top = 0;
        self.margin_right = 0;
        self.margin_bottom = 0;
        self.margin_left = 0;
        self.margin_bg_color = None;
        self
    }

    /// Removes the top margin from this style.
    ///
    /// This resets the top margin to 0 and removes it from the style's property set.
    /// The content will no longer have spacing above it.
    ///
    /// # Returns
    ///
    /// Returns the modified `Style` instance for method chaining.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use lipgloss::Style;
    ///
    /// let style = Style::new()
    ///     .margin(2, 1, 2, 1)  // top, right, bottom, left
    ///     .unset_margin_top();  // Remove only top margin
    ///
    /// // Content will have 0 top margin, but keep right(1), bottom(2), left(1)
    /// let result = style.render("Hello World");
    /// ```
    pub fn unset_margin_top(mut self) -> Self {
        self.unset_prop(MARGIN_TOP_KEY);
        self.margin_top = 0;
        self
    }

    /// Removes the right margin from this style.
    ///
    /// This resets the right margin to 0 and removes it from the style's property set.
    /// The content will no longer have spacing to its right.
    ///
    /// # Returns
    ///
    /// Returns the modified `Style` instance for method chaining.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use lipgloss::Style;
    ///
    /// let style = Style::new()
    ///     .margin(2, 3, 2, 1)  // top, right, bottom, left
    ///     .unset_margin_right();  // Remove only right margin
    ///
    /// // Content will have 0 right margin, but keep top(2), bottom(2), left(1)
    /// let result = style.render("Hello World");
    /// ```
    pub fn unset_margin_right(mut self) -> Self {
        self.unset_prop(MARGIN_RIGHT_KEY);
        self.margin_right = 0;
        self
    }

    /// Removes the bottom margin from this style.
    ///
    /// This resets the bottom margin to 0 and removes it from the style's property set.
    /// The content will no longer have spacing below it.
    ///
    /// # Returns
    ///
    /// Returns the modified `Style` instance for method chaining.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use lipgloss::Style;
    ///
    /// let style = Style::new()
    ///     .margin(2, 1, 3, 1)  // top, right, bottom, left
    ///     .unset_margin_bottom();  // Remove only bottom margin
    ///
    /// // Content will have 0 bottom margin, but keep top(2), right(1), left(1)
    /// let result = style.render("Hello World");
    /// ```
    pub fn unset_margin_bottom(mut self) -> Self {
        self.unset_prop(MARGIN_BOTTOM_KEY);
        self.margin_bottom = 0;
        self
    }

    /// Removes the left margin from this style.
    ///
    /// This resets the left margin to 0 and removes it from the style's property set.
    /// The content will no longer have spacing to its left.
    ///
    /// # Returns
    ///
    /// Returns the modified `Style` instance for method chaining.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use lipgloss::Style;
    ///
    /// let style = Style::new()
    ///     .margin(2, 1, 2, 3)  // top, right, bottom, left
    ///     .unset_margin_left();  // Remove only left margin
    ///
    /// // Content will have 0 left margin, but keep top(2), right(1), bottom(2)
    /// let result = style.render("Hello World");
    /// ```
    pub fn unset_margin_left(mut self) -> Self {
        self.unset_prop(MARGIN_LEFT_KEY);
        self.margin_left = 0;
        self
    }

    /// Removes the margin background color from this style.
    ///
    /// This resets the margin background color to `None` and removes it from the style's
    /// property set. Margin areas will use the terminal's default background color.
    ///
    /// # Returns
    ///
    /// Returns the modified `Style` instance for method chaining.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use lipgloss::Style;
    ///
    /// let style = Style::new()
    ///     .margin(2, 2, 2, 2)
    ///     .margin_background("blue")
    ///     .background("red")
    ///     .unset_margin_background();  // Remove margin color, keep content color
    ///
    /// // Content will have red background, but margin areas use default color
    /// let result = style.render("Hello World");
    /// ```
    pub fn unset_margin_background(mut self) -> Self {
        self.unset_prop(MARGIN_BACKGROUND_KEY);
        self.margin_bg_color = None;
        self
    }

    // ---------- Border unset methods ----------

    /// Removes the border style and disables all border edges.
    ///
    /// This resets the border style to hidden and disables all border edges
    /// (top, right, bottom, left). It effectively removes all border rendering.
    ///
    /// # Returns
    ///
    /// Returns the modified `Style` instance for method chaining.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use lipgloss::{Style, normal_border};
    ///
    /// let style = Style::new()
    ///     .border_style(normal_border())
    ///     .border_foreground("red")
    ///     .unset_border_style();  // Remove all borders
    ///
    /// // Text will have no borders
    /// let result = style.render("Hello World");
    /// ```
    pub fn unset_border_style(mut self) -> Self {
        self.unset_prop(BORDER_STYLE_KEY);
        self.unset_prop(BORDER_TOP_KEY);
        self.unset_prop(BORDER_RIGHT_KEY);
        self.unset_prop(BORDER_BOTTOM_KEY);
        self.unset_prop(BORDER_LEFT_KEY);
        self.border_style = hidden_border();
        self.set_attr(ATTR_BORDER_TOP, false);
        self.set_attr(ATTR_BORDER_RIGHT, false);
        self.set_attr(ATTR_BORDER_BOTTOM, false);
        self.set_attr(ATTR_BORDER_LEFT, false);
        self
    }

    /// Removes the top border from this style.
    ///
    /// This disables the top border edge and removes it from the style's property set.
    /// The top edge of the content will not have a border.
    ///
    /// # Returns
    ///
    /// Returns the modified `Style` instance for method chaining.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use lipgloss::{Style, normal_border};
    ///
    /// let style = Style::new()
    ///     .border_style(normal_border())
    ///     .border_top(true)
    ///     .border_bottom(true)
    ///     .unset_border_top();  // Remove top border, keep bottom
    ///
    /// // Content will have bottom border but no top border
    /// let result = style.render("Hello World");
    /// ```
    pub fn unset_border_top(mut self) -> Self {
        self.unset_prop(BORDER_TOP_KEY);
        self.set_attr(ATTR_BORDER_TOP, false);
        self
    }

    /// Removes the right border from this style.
    ///
    /// This disables the right border edge and removes it from the style's property set.
    /// The right edge of the content will not have a border.
    ///
    /// # Returns
    ///
    /// Returns the modified `Style` instance for method chaining.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use lipgloss::{Style, normal_border};
    ///
    /// let style = Style::new()
    ///     .border_style(normal_border())
    ///     .border_right(true)
    ///     .border_left(true)
    ///     .unset_border_right();  // Remove right border, keep left
    ///
    /// // Content will have left border but no right border
    /// let result = style.render("Hello World");
    /// ```
    pub fn unset_border_right(mut self) -> Self {
        self.unset_prop(BORDER_RIGHT_KEY);
        self.set_attr(ATTR_BORDER_RIGHT, false);
        self
    }

    /// Removes the bottom border from this style.
    ///
    /// This disables the bottom border edge and removes it from the style's property set.
    /// The bottom edge of the content will not have a border.
    ///
    /// # Returns
    ///
    /// Returns the modified `Style` instance for method chaining.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use lipgloss::{Style, normal_border};
    ///
    /// let style = Style::new()
    ///     .border_style(normal_border())
    ///     .border_top(true)
    ///     .border_bottom(true)
    ///     .unset_border_bottom();  // Remove bottom border, keep top
    ///
    /// // Content will have top border but no bottom border
    /// let result = style.render("Hello World");
    /// ```
    pub fn unset_border_bottom(mut self) -> Self {
        self.unset_prop(BORDER_BOTTOM_KEY);
        self.set_attr(ATTR_BORDER_BOTTOM, false);
        self
    }

    /// Removes the left border from this style.
    ///
    /// This disables the left border edge and removes it from the style's property set.
    /// The left edge of the content will not have a border.
    ///
    /// # Returns
    ///
    /// Returns the modified `Style` instance for method chaining.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use lipgloss::{Style, normal_border};
    ///
    /// let style = Style::new()
    ///     .border_style(normal_border())
    ///     .border_left(true)
    ///     .border_right(true)
    ///     .unset_border_left();  // Remove left border, keep right
    ///
    /// // Content will have right border but no left border
    /// let result = style.render("Hello World");
    /// ```
    pub fn unset_border_left(mut self) -> Self {
        self.unset_prop(BORDER_LEFT_KEY);
        self.set_attr(ATTR_BORDER_LEFT, false);
        self
    }

    /// Removes all border foreground colors from this style.
    ///
    /// This resets all border edge foreground colors (top, right, bottom, left) to `None`
    /// and removes them from the style's property set. Borders will use default colors.
    ///
    /// # Returns
    ///
    /// Returns the modified `Style` instance for method chaining.
    pub fn unset_border_foreground(mut self) -> Self {
        self.unset_prop(BORDER_TOP_FOREGROUND_KEY);
        self.unset_prop(BORDER_RIGHT_FOREGROUND_KEY);
        self.unset_prop(BORDER_BOTTOM_FOREGROUND_KEY);
        self.unset_prop(BORDER_LEFT_FOREGROUND_KEY);
        self.border_top_fg_color = None;
        self.border_right_fg_color = None;
        self.border_bottom_fg_color = None;
        self.border_left_fg_color = None;
        self
    }

    /// Removes all border background colors from this style.
    ///
    /// This resets all border edge background colors (top, right, bottom, left) to `None`
    /// and removes them from the style's property set. Borders will use default colors.
    ///
    /// # Returns
    ///
    /// Returns the modified `Style` instance for method chaining.
    pub fn unset_border_background(mut self) -> Self {
        self.unset_prop(BORDER_TOP_BACKGROUND_KEY);
        self.unset_prop(BORDER_RIGHT_BACKGROUND_KEY);
        self.unset_prop(BORDER_BOTTOM_BACKGROUND_KEY);
        self.unset_prop(BORDER_LEFT_BACKGROUND_KEY);
        self.border_top_bg_color = None;
        self.border_right_bg_color = None;
        self.border_bottom_bg_color = None;
        self.border_left_bg_color = None;
        self
    }

    /// Removes the top border foreground color from this style.
    ///
    /// This resets the top border foreground color to `None` and removes it from the
    /// style's property set. The top border will use the default foreground color.
    ///
    /// # Returns
    ///
    /// Returns the modified `Style` instance for method chaining.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use lipgloss::{Style, normal_border};
    ///
    /// let style = Style::new()
    ///     .border_style(normal_border())
    ///     .border_top_foreground("red")
    ///     .border_bottom_foreground("blue")
    ///     .unset_border_top_foreground();  // Remove top color, keep bottom
    ///
    /// // Top border uses default color, bottom border is blue
    /// let result = style.render("Hello World");
    /// ```
    pub fn unset_border_top_foreground(mut self) -> Self {
        self.unset_prop(BORDER_TOP_FOREGROUND_KEY);
        self.border_top_fg_color = None;
        self
    }

    /// Removes the right border foreground color from this style.
    ///
    /// This resets the right border foreground color to `None` and removes it from the
    /// style's property set. The right border will use the default foreground color.
    ///
    /// # Returns
    ///
    /// Returns the modified `Style` instance for method chaining.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use lipgloss::{Style, normal_border};
    ///
    /// let style = Style::new()
    ///     .border_style(normal_border())
    ///     .border_right_foreground("red")
    ///     .border_left_foreground("blue")
    ///     .unset_border_right_foreground();  // Remove right color, keep left
    ///
    /// // Right border uses default color, left border is blue
    /// let result = style.render("Hello World");
    /// ```
    pub fn unset_border_right_foreground(mut self) -> Self {
        self.unset_prop(BORDER_RIGHT_FOREGROUND_KEY);
        self.border_right_fg_color = None;
        self
    }

    /// Removes the bottom border foreground color from this style.
    ///
    /// This resets the bottom border foreground color to `None` and removes it from the
    /// style's property set. The bottom border will use the default foreground color.
    ///
    /// # Returns
    ///
    /// Returns the modified `Style` instance for method chaining.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use lipgloss::{Style, normal_border};
    ///
    /// let style = Style::new()
    ///     .border_style(normal_border())
    ///     .border_bottom_foreground("red")
    ///     .border_top_foreground("blue")
    ///     .unset_border_bottom_foreground();  // Remove bottom color, keep top
    ///
    /// // Bottom border uses default color, top border is blue
    /// let result = style.render("Hello World");
    /// ```
    pub fn unset_border_bottom_foreground(mut self) -> Self {
        self.unset_prop(BORDER_BOTTOM_FOREGROUND_KEY);
        self.border_bottom_fg_color = None;
        self
    }

    /// Removes the left border foreground color from this style.
    ///
    /// This resets the left border foreground color to `None` and removes it from the
    /// style's property set. The left border will use the default foreground color.
    ///
    /// # Returns
    ///
    /// Returns the modified `Style` instance for method chaining.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use lipgloss::{Style, normal_border};
    ///
    /// let style = Style::new()
    ///     .border_style(normal_border())
    ///     .border_left_foreground("red")
    ///     .border_right_foreground("blue")
    ///     .unset_border_left_foreground();  // Remove left color, keep right
    ///
    /// // Left border uses default color, right border is blue
    /// let result = style.render("Hello World");
    /// ```
    pub fn unset_border_left_foreground(mut self) -> Self {
        self.unset_prop(BORDER_LEFT_FOREGROUND_KEY);
        self.border_left_fg_color = None;
        self
    }

    /// Removes the top border background color from this style.
    ///
    /// This resets the top border background color to `None` and removes it from the
    /// style's property set. The top border will use the default background color.
    ///
    /// # Returns
    ///
    /// Returns the modified `Style` instance for method chaining.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use lipgloss::{Style, normal_border};
    ///
    /// let style = Style::new()
    ///     .border_style(normal_border())
    ///     .border_top_background("red")
    ///     .border_bottom_background("blue")
    ///     .unset_border_top_background();  // Remove top color, keep bottom
    ///
    /// // Top border uses default background, bottom border has blue background
    /// let result = style.render("Hello World");
    /// ```
    pub fn unset_border_top_background(mut self) -> Self {
        self.unset_prop(BORDER_TOP_BACKGROUND_KEY);
        self.border_top_bg_color = None;
        self
    }

    /// Removes the right border background color from this style.
    ///
    /// This resets the right border background color to `None` and removes it from the
    /// style's property set. The right border will use the default background color.
    ///
    /// # Returns
    ///
    /// Returns the modified `Style` instance for method chaining.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use lipgloss::{Style, normal_border};
    ///
    /// let style = Style::new()
    ///     .border_style(normal_border())
    ///     .border_right_background("red")
    ///     .border_left_background("blue")
    ///     .unset_border_right_background();  // Remove right color, keep left
    ///
    /// // Right border uses default background, left border has blue background
    /// let result = style.render("Hello World");
    /// ```
    pub fn unset_border_right_background(mut self) -> Self {
        self.unset_prop(BORDER_RIGHT_BACKGROUND_KEY);
        self.border_right_bg_color = None;
        self
    }

    /// Removes the bottom border background color from this style.
    ///
    /// This resets the bottom border background color to `None` and removes it from the
    /// style's property set. The bottom border will use the default background color.
    ///
    /// # Returns
    ///
    /// Returns the modified `Style` instance for method chaining.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use lipgloss::{Style, normal_border};
    ///
    /// let style = Style::new()
    ///     .border_style(normal_border())
    ///     .border_bottom_background("red")
    ///     .border_top_background("blue")
    ///     .unset_border_bottom_background();  // Remove bottom color, keep top
    ///
    /// // Bottom border uses default background, top border has blue background
    /// let result = style.render("Hello World");
    /// ```
    pub fn unset_border_bottom_background(mut self) -> Self {
        self.unset_prop(BORDER_BOTTOM_BACKGROUND_KEY);
        self.border_bottom_bg_color = None;
        self
    }

    /// Removes the left border background color from this style.
    ///
    /// This resets the left border background color to `None` and removes it from the
    /// style's property set. The left border will use the default background color.
    ///
    /// # Returns
    ///
    /// Returns the modified `Style` instance for method chaining.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use lipgloss::{Style, normal_border};
    ///
    /// let style = Style::new()
    ///     .border_style(normal_border())
    ///     .border_left_background("red")
    ///     .border_right_background("blue")
    ///     .unset_border_left_background();  // Remove left color, keep right
    ///
    /// // Left border uses default background, right border has blue background
    /// let result = style.render("Hello World");
    /// ```
    pub fn unset_border_left_background(mut self) -> Self {
        self.unset_prop(BORDER_LEFT_BACKGROUND_KEY);
        self.border_left_bg_color = None;
        self
    }

    // ---------- Other unset methods ----------

    /// Removes the custom tab width setting from this style.
    ///
    /// This resets the tab width to its default value and removes it from
    /// the style's property set. Tabs will be rendered using the default width.
    ///
    /// # Returns
    ///
    /// Returns the modified `Style` instance for method chaining.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use lipgloss::Style;
    ///
    /// let style = Style::new()
    ///     .tab_width(8)
    ///     .bold(true)
    ///     .unset_tab_width();  // Reset to default tab width, keep bold
    ///
    /// let result = style.render("Text\twith\ttabs");
    /// ```
    pub fn unset_tab_width(mut self) -> Self {
        self.unset_prop(TAB_WIDTH_KEY);
        self.tab_width = TAB_WIDTH_DEFAULT;
        self
    }

    /// Removes the text transformation function from this style.
    ///
    /// This resets the transform function to `None` and removes it from
    /// the style's property set. Text will be rendered without transformation.
    ///
    /// # Returns
    ///
    /// Returns the modified `Style` instance for method chaining.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use lipgloss::Style;
    ///
    /// let style = Style::new()
    ///     .transform(|text| text.to_uppercase())
    ///     .bold(true)
    ///     .unset_transform();  // Remove transformation, keep bold
    ///
    /// // Text will not be transformed to uppercase
    /// let result = style.render("Hello World");
    /// ```
    pub fn unset_transform(mut self) -> Self {
        self.unset_prop(TRANSFORM_KEY);
        self.transform = None;
        self
    }

    /// Clears the underlying string value from this style.
    ///
    /// This resets the internal string value to an empty string. This is typically
    /// used when you want to reuse a style with different content.
    ///
    /// # Returns
    ///
    /// Returns the modified `Style` instance for method chaining.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use lipgloss::Style;
    ///
    /// let mut style = Style::new().bold(true);
    /// // First render with initial content
    /// let _first = style.render("First message");
    ///
    /// // Clear the content but keep styling
    /// style = style.unset_string();
    ///
    /// // Reuse the style with new content
    /// let result = style.render("Second message");
    /// ```
    pub fn unset_string(mut self) -> Self {
        self.value = String::new();
        self
    }
}
