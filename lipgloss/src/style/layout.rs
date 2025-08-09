//! Layout-related methods for Style (padding, margin, alignment)

use crate::position::Position;
use crate::security::validate_dimension;
use crate::style::{properties::*, Style};
use crate::utils::which_sides_int;

impl Style {
    /// Set horizontal and vertical alignment for content within the styled box.
    ///
    /// This method sets both horizontal and vertical alignment in a single call,
    /// determining how content will be positioned within the available space when
    /// the content is smaller than the box dimensions.
    ///
    /// # Arguments
    ///
    /// * `horizontal` - Horizontal position (0.0 = left, 0.5 = center, 1.0 = right)
    /// * `vertical` - Vertical position (0.0 = top, 0.5 = center, 1.0 = bottom)
    ///
    /// # Returns
    ///
    /// The modified `Style` with both alignment properties set.
    ///
    /// # Examples
    ///
    /// ```
    /// use lipgloss::{Style, position::{CENTER, LEFT, TOP}};
    ///
    /// // Center content both horizontally and vertically
    /// let style = Style::new().align(CENTER, CENTER);
    ///
    /// // Align to top-left corner
    /// let style = Style::new().align(LEFT, TOP);
    ///
    /// // Custom alignment (75% right, 25% down)
    /// use lipgloss::position::Position;
    /// let style = Style::new().align(Position(0.75), Position(0.25));
    /// ```
    pub fn align(mut self, horizontal: Position, vertical: Position) -> Self {
        self.align_horizontal = horizontal;
        self.align_vertical = vertical;
        self.set_prop(ALIGN_HORIZONTAL_KEY);
        self.set_prop(ALIGN_VERTICAL_KEY);
        self
    }

    /// Set horizontal alignment for content within the styled box.
    ///
    /// This method controls how content is horizontally positioned when it's
    /// narrower than the available width. The position is specified as a value
    /// from 0.0 (left) to 1.0 (right), with 0.5 being center.
    ///
    /// # Arguments
    ///
    /// * `p` - Horizontal position (0.0 = left, 0.5 = center, 1.0 = right)
    ///
    /// # Returns
    ///
    /// The modified `Style` with horizontal alignment set.
    ///
    /// # Examples
    ///
    /// ```
    /// use lipgloss::{Style, position::{LEFT, CENTER, RIGHT, Position}};
    ///
    /// // Left-align content
    /// let left_style = Style::new().align_horizontal(LEFT);
    ///
    /// // Center content horizontally
    /// let center_style = Style::new().align_horizontal(CENTER);
    ///
    /// // Right-align content
    /// let right_style = Style::new().align_horizontal(RIGHT);
    ///
    /// // Custom horizontal position (25% from left)
    /// let custom_style = Style::new().align_horizontal(Position(0.25));
    /// ```
    pub fn align_horizontal(mut self, p: Position) -> Self {
        self.align_horizontal = p;
        self.set_prop(ALIGN_HORIZONTAL_KEY);
        self
    }

    /// Set vertical alignment for content within the styled box.
    ///
    /// This method controls how content is vertically positioned when it's
    /// shorter than the available height. The position is specified as a value
    /// from 0.0 (top) to 1.0 (bottom), with 0.5 being center.
    ///
    /// # Arguments
    ///
    /// * `p` - Vertical position (0.0 = top, 0.5 = center, 1.0 = bottom)
    ///
    /// # Returns
    ///
    /// The modified `Style` with vertical alignment set.
    ///
    /// # Examples
    ///
    /// ```
    /// use lipgloss::{Style, position::{TOP, CENTER, BOTTOM, Position}};
    ///
    /// // Top-align content
    /// let top_style = Style::new().align_vertical(TOP);
    ///
    /// // Center content vertically
    /// let center_style = Style::new().align_vertical(CENTER);
    ///
    /// // Bottom-align content
    /// let bottom_style = Style::new().align_vertical(BOTTOM);
    ///
    /// // Custom vertical position (75% from top)
    /// let custom_style = Style::new().align_vertical(Position(0.75));
    /// ```
    pub fn align_vertical(mut self, p: Position) -> Self {
        self.align_vertical = p;
        self.set_prop(ALIGN_VERTICAL_KEY);
        self
    }

    /// Set padding on all sides of the styled content.
    ///
    /// Padding adds space inside the styled box, between the content and any
    /// borders. This method sets all four sides at once using CSS-like ordering
    /// (top, right, bottom, left). Negative values are allowed and will reduce
    /// the available content area.
    ///
    /// # Arguments
    ///
    /// * `top` - Top padding in characters/lines
    /// * `right` - Right padding in characters
    /// * `bottom` - Bottom padding in characters/lines
    /// * `left` - Left padding in characters
    ///
    /// # Returns
    ///
    /// The modified `Style` with all padding properties set.
    ///
    /// # Examples
    ///
    /// ```
    /// use lipgloss::Style;
    ///
    /// // Uniform padding of 2 on all sides
    /// let padded = Style::new().padding(2, 2, 2, 2);
    ///
    /// // Different padding for each side
    /// let asymmetric = Style::new().padding(1, 4, 2, 3);
    ///
    /// // No padding
    /// let no_padding = Style::new().padding(0, 0, 0, 0);
    ///
    /// // Can be combined with other styles
    /// use lipgloss::color::Color;
    /// let styled = Style::new()
    ///     .padding(2, 4, 2, 4)
    ///     .background(Color("blue".to_string()))
    ///     .render("Content");
    /// ```
    pub fn padding(mut self, top: i32, right: i32, bottom: i32, left: i32) -> Self {
        self.padding_top = validate_dimension(top, "padding_top");
        self.padding_right = validate_dimension(right, "padding_right");
        self.padding_bottom = validate_dimension(bottom, "padding_bottom");
        self.padding_left = validate_dimension(left, "padding_left");
        self.set_prop(PADDING_TOP_KEY);
        self.set_prop(PADDING_RIGHT_KEY);
        self.set_prop(PADDING_BOTTOM_KEY);
        self.set_prop(PADDING_LEFT_KEY);
        self
    }

    /// Set padding with two values: vertical and horizontal.
    ///
    /// This is a convenient shorthand for setting top/bottom and left/right
    /// padding with the same values. Equivalent to calling
    /// `padding(vertical, horizontal, vertical, horizontal)`.
    ///
    /// # Arguments
    ///
    /// * `vertical` - Padding for top and bottom sides
    /// * `horizontal` - Padding for left and right sides
    ///
    /// # Examples
    ///
    /// ```
    /// use lipgloss::Style;
    ///
    /// // 2 characters top/bottom, 4 characters left/right
    /// let padded = Style::new().padding_2(2, 4);
    ///
    /// // Equivalent to:
    /// let equivalent = Style::new().padding(2, 4, 2, 4);
    /// ```
    pub fn padding_2(self, vertical: i32, horizontal: i32) -> Self {
        self.padding(vertical, horizontal, vertical, horizontal)
    }

    /// Set padding with three values: top, horizontal, and bottom.
    ///
    /// This is a convenient shorthand where the horizontal value applies to
    /// both left and right sides. Equivalent to calling
    /// `padding(top, horizontal, bottom, horizontal)`.
    ///
    /// # Arguments
    ///
    /// * `top` - Top padding
    /// * `horizontal` - Padding for left and right sides
    /// * `bottom` - Bottom padding
    ///
    /// # Examples
    ///
    /// ```
    /// use lipgloss::Style;
    ///
    /// // 1 char top, 3 chars left/right, 2 chars bottom
    /// let padded = Style::new().padding_3(1, 3, 2);
    ///
    /// // Equivalent to:
    /// let equivalent = Style::new().padding(1, 3, 2, 3);
    /// ```
    pub fn padding_3(self, top: i32, horizontal: i32, bottom: i32) -> Self {
        self.padding(top, horizontal, bottom, horizontal)
    }

    /// Set left padding for the styled content.
    ///
    /// Left padding adds space on the left side of the content, between the
    /// content and any borders. This affects text positioning and the overall
    /// width of the rendered output.
    ///
    /// # Arguments
    ///
    /// * `n` - Left padding in characters (can be negative to reduce space)
    ///
    /// # Returns
    ///
    /// The modified `Style` with left padding set.
    ///
    /// # Examples
    ///
    /// ```
    /// use lipgloss::Style;
    ///
    /// // Add 4 characters of left padding
    /// let indented = Style::new().padding_left(4);
    ///
    /// // No left padding
    /// let no_indent = Style::new().padding_left(0);
    ///
    /// // Combine with other padding
    /// let mixed_padding = Style::new()
    ///     .padding_left(4)
    ///     .padding_right(2)
    ///     .padding_top(1);
    /// ```
    pub fn padding_left(mut self, n: i32) -> Self {
        self.padding_left = validate_dimension(n, "padding_left");
        self.set_prop(PADDING_LEFT_KEY);
        self
    }

    /// Set right padding for the styled content.
    ///
    /// Right padding adds space on the right side of the content, between the
    /// content and any borders. This affects the overall width of the rendered
    /// output and can be useful for creating consistent layouts.
    ///
    /// # Arguments
    ///
    /// * `n` - Right padding in characters (can be negative to reduce space)
    ///
    /// # Returns
    ///
    /// The modified `Style` with right padding set.
    ///
    /// # Examples
    ///
    /// ```
    /// use lipgloss::Style;
    ///
    /// // Add 3 characters of right padding
    /// let padded = Style::new().padding_right(3);
    ///
    /// // No right padding
    /// let no_padding = Style::new().padding_right(0);
    ///
    /// // Create symmetrical horizontal padding
    /// let symmetric = Style::new()
    ///     .padding_left(2)
    ///     .padding_right(2);
    /// ```
    pub fn padding_right(mut self, n: i32) -> Self {
        self.padding_right = validate_dimension(n, "padding_right");
        self.set_prop(PADDING_RIGHT_KEY);
        self
    }

    /// Set top padding for the styled content.
    ///
    /// Top padding adds empty lines above the content, between the content and
    /// any borders. This increases the vertical space and can be used to create
    /// visual separation or centering effects.
    ///
    /// # Arguments
    ///
    /// * `n` - Top padding in lines (can be negative to reduce space)
    ///
    /// # Returns
    ///
    /// The modified `Style` with top padding set.
    ///
    /// # Examples
    ///
    /// ```
    /// use lipgloss::Style;
    ///
    /// // Add 2 lines of top padding
    /// let spaced = Style::new().padding_top(2);
    ///
    /// // No top padding
    /// let compact = Style::new().padding_top(0);
    ///
    /// // Create vertical spacing with other padding
    /// let box_style = Style::new()
    ///     .padding_top(1)
    ///     .padding_bottom(1)
    ///     .padding_left(3)
    ///     .padding_right(3);
    /// ```
    pub fn padding_top(mut self, n: i32) -> Self {
        self.padding_top = validate_dimension(n, "padding_top");
        self.set_prop(PADDING_TOP_KEY);
        self
    }

    /// Set bottom padding for the styled content.
    ///
    /// Bottom padding adds empty lines below the content, between the content and
    /// any borders. This increases the vertical space and helps create balanced
    /// layouts or visual separation.
    ///
    /// # Arguments
    ///
    /// * `n` - Bottom padding in lines (can be negative to reduce space)
    ///
    /// # Returns
    ///
    /// The modified `Style` with bottom padding set.
    ///
    /// # Examples
    ///
    /// ```
    /// use lipgloss::Style;
    ///
    /// // Add 1 line of bottom padding
    /// let spaced = Style::new().padding_bottom(1);
    ///
    /// // No bottom padding
    /// let compact = Style::new().padding_bottom(0);
    ///
    /// // Create symmetrical vertical padding
    /// let balanced = Style::new()
    ///     .padding_top(2)
    ///     .padding_bottom(2);
    /// ```
    pub fn padding_bottom(mut self, n: i32) -> Self {
        self.padding_bottom = validate_dimension(n, "padding_bottom");
        self.set_prop(PADDING_BOTTOM_KEY);
        self
    }

    /// Set margin on all sides of the styled box.
    ///
    /// Margin adds space outside the styled box, creating separation from other
    /// elements. Unlike padding, margin is outside any borders and backgrounds.
    /// This method sets all four sides at once using CSS-like ordering
    /// (top, right, bottom, left). Negative values are allowed.
    ///
    /// # Arguments
    ///
    /// * `top` - Top margin in characters/lines
    /// * `right` - Right margin in characters
    /// * `bottom` - Bottom margin in characters/lines
    /// * `left` - Left margin in characters
    ///
    /// # Returns
    ///
    /// The modified `Style` with all margin properties set.
    ///
    /// # Examples
    ///
    /// ```
    /// use lipgloss::Style;
    ///
    /// // Uniform margin of 1 on all sides
    /// let spaced = Style::new().margin(1, 1, 1, 1);
    ///
    /// // Different margins for each side
    /// let asymmetric = Style::new().margin(2, 4, 1, 3);
    ///
    /// // No margin
    /// let no_margin = Style::new().margin(0, 0, 0, 0);
    ///
    /// // Combine with other styles for card-like appearance
    /// use lipgloss::color::Color;
    /// let card = Style::new()
    ///     .margin(1, 2, 1, 2)
    ///     .padding(1, 2, 1, 2)
    ///     .border(lipgloss::rounded_border())
    ///     .border_foreground(Color("gray".to_string()));
    /// ```
    pub fn margin(mut self, top: i32, right: i32, bottom: i32, left: i32) -> Self {
        self.margin_top = validate_dimension(top, "margin_top");
        self.margin_right = validate_dimension(right, "margin_right");
        self.margin_bottom = validate_dimension(bottom, "margin_bottom");
        self.margin_left = validate_dimension(left, "margin_left");
        self.set_prop(MARGIN_TOP_KEY);
        self.set_prop(MARGIN_RIGHT_KEY);
        self.set_prop(MARGIN_BOTTOM_KEY);
        self.set_prop(MARGIN_LEFT_KEY);
        self
    }

    /// Set margin with two values: vertical and horizontal.
    ///
    /// This is a convenient shorthand for setting top/bottom and left/right
    /// margin with the same values. Equivalent to calling
    /// `margin(vertical, horizontal, vertical, horizontal)`.
    ///
    /// # Arguments
    ///
    /// * `vertical` - Margin for top and bottom sides
    /// * `horizontal` - Margin for left and right sides
    ///
    /// # Examples
    ///
    /// ```
    /// use lipgloss::Style;
    ///
    /// // 1 character top/bottom, 2 characters left/right
    /// let spaced = Style::new().margin_2(1, 2);
    ///
    /// // Equivalent to:
    /// let equivalent = Style::new().margin(1, 2, 1, 2);
    /// ```
    pub fn margin_2(self, vertical: i32, horizontal: i32) -> Self {
        self.margin(vertical, horizontal, vertical, horizontal)
    }

    /// Set margin with three values: top, horizontal, and bottom.
    ///
    /// This is a convenient shorthand where the horizontal value applies to
    /// both left and right sides. Equivalent to calling
    /// `margin(top, horizontal, bottom, horizontal)`.
    ///
    /// # Arguments
    ///
    /// * `top` - Top margin
    /// * `horizontal` - Margin for left and right sides
    /// * `bottom` - Bottom margin
    ///
    /// # Examples
    ///
    /// ```
    /// use lipgloss::Style;
    ///
    /// // 2 chars top, 1 char left/right, 1 char bottom
    /// let spaced = Style::new().margin_3(2, 1, 1);
    ///
    /// // Equivalent to:
    /// let equivalent = Style::new().margin(2, 1, 1, 1);
    /// ```
    pub fn margin_3(self, top: i32, horizontal: i32, bottom: i32) -> Self {
        self.margin(top, horizontal, bottom, horizontal)
    }

    /// Set left margin for the styled box.
    ///
    /// Left margin adds space on the left side of the entire styled box,
    /// creating separation from other elements or indenting the box from
    /// the left edge. This is outside any borders or padding.
    ///
    /// # Arguments
    ///
    /// * `n` - Left margin in characters (can be negative)
    ///
    /// # Returns
    ///
    /// The modified `Style` with left margin set.
    ///
    /// # Examples
    ///
    /// ```
    /// use lipgloss::Style;
    ///
    /// // Indent the box by 6 characters
    /// let indented = Style::new().margin_left(6);
    ///
    /// // No left margin
    /// let flush = Style::new().margin_left(0);
    ///
    /// // Create nested indentation
    /// let nested = Style::new()
    ///     .margin_left(4)
    ///     .padding_left(2)
    ///     .border(lipgloss::normal_border());
    /// ```
    pub fn margin_left(mut self, n: i32) -> Self {
        self.margin_left = validate_dimension(n, "margin_left");
        self.set_prop(MARGIN_LEFT_KEY);
        self
    }

    /// Set right margin for the styled box.
    ///
    /// Right margin adds space on the right side of the entire styled box,
    /// creating separation from other elements or ensuring the box doesn't
    /// extend to the right edge. This is outside any borders or padding.
    ///
    /// # Arguments
    ///
    /// * `n` - Right margin in characters (can be negative)
    ///
    /// # Returns
    ///
    /// The modified `Style` with right margin set.
    ///
    /// # Examples
    ///
    /// ```
    /// use lipgloss::Style;
    ///
    /// // Add 4 characters of right margin
    /// let spaced = Style::new().margin_right(4);
    ///
    /// // No right margin
    /// let flush = Style::new().margin_right(0);
    ///
    /// // Create symmetrical horizontal margins
    /// let centered = Style::new()
    ///     .margin_left(5)
    ///     .margin_right(5);
    /// ```
    pub fn margin_right(mut self, n: i32) -> Self {
        self.margin_right = validate_dimension(n, "margin_right");
        self.set_prop(MARGIN_RIGHT_KEY);
        self
    }

    /// Set top margin for the styled box.
    ///
    /// Top margin adds empty lines above the entire styled box, creating
    /// vertical separation from other elements. This is outside any borders
    /// or padding and affects the overall layout positioning.
    ///
    /// # Arguments
    ///
    /// * `n` - Top margin in lines (can be negative)
    ///
    /// # Returns
    ///
    /// The modified `Style` with top margin set.
    ///
    /// # Examples
    ///
    /// ```
    /// use lipgloss::Style;
    ///
    /// // Add 3 lines of top margin
    /// let spaced = Style::new().margin_top(3);
    ///
    /// // No top margin
    /// let compact = Style::new().margin_top(0);
    ///
    /// // Create section spacing
    /// let section = Style::new()
    ///     .margin_top(2)
    ///     .margin_bottom(1)
    ///     .bold(true);
    /// ```
    pub fn margin_top(mut self, n: i32) -> Self {
        self.margin_top = validate_dimension(n, "margin_top");
        self.set_prop(MARGIN_TOP_KEY);
        self
    }

    /// Set bottom margin for the styled box.
    ///
    /// Bottom margin adds empty lines below the entire styled box, creating
    /// vertical separation from following elements. This is outside any borders
    /// or padding and helps establish visual hierarchy in layouts.
    ///
    /// # Arguments
    ///
    /// * `n` - Bottom margin in lines (can be negative)
    ///
    /// # Returns
    ///
    /// The modified `Style` with bottom margin set.
    ///
    /// # Examples
    ///
    /// ```
    /// use lipgloss::Style;
    ///
    /// // Add 2 lines of bottom margin
    /// let spaced = Style::new().margin_bottom(2);
    ///
    /// // No bottom margin
    /// let compact = Style::new().margin_bottom(0);
    ///
    /// // Create paragraph-like spacing
    /// let paragraph = Style::new()
    ///     .margin_bottom(1)
    ///     .padding(0, 2, 0, 2);
    /// ```
    pub fn margin_bottom(mut self, n: i32) -> Self {
        self.margin_bottom = validate_dimension(n, "margin_bottom");
        self.set_prop(MARGIN_BOTTOM_KEY);
        self
    }

    /// Set padding using CSS-style shorthand notation.
    ///
    /// This method accepts 1-4 values and applies them using CSS shorthand rules:
    /// - 1 value: applies to all sides
    /// - 2 values: first is top/bottom, second is left/right  
    /// - 3 values: first is top, second is left/right, third is bottom
    /// - 4 values: top, right, bottom, left (clockwise from top)
    ///
    /// Invalid input (0 or 5+ values) will be ignored and return the style unchanged.
    ///
    /// # Arguments
    ///
    /// * `values` - Slice of 1-4 padding values in CSS shorthand order
    ///
    /// # Returns
    ///
    /// The modified `Style` with padding properties set according to CSS shorthand rules.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use lipgloss::Style;
    ///
    /// // Uniform padding of 2 on all sides
    /// let uniform = Style::new().padding_shorthand(&[2]);
    ///
    /// // Vertical padding of 1, horizontal padding of 3
    /// let vh = Style::new().padding_shorthand(&[1, 3]);
    ///
    /// // Top 1, horizontal 2, bottom 3
    /// let asymmetric = Style::new().padding_shorthand(&[1, 2, 3]);
    ///
    /// // Clockwise from top: 1, 2, 3, 4
    /// let clockwise = Style::new().padding_shorthand(&[1, 2, 3, 4]);
    ///
    /// // Invalid (ignored): too many values
    /// let unchanged = Style::new().padding_shorthand(&[1, 2, 3, 4, 5]);
    /// ```
    ///
    /// # See Also
    ///
    /// - [`Self::padding`] - For explicit four-side padding
    /// - [`Self::padding_top`], [`Self::padding_right`], [`Self::padding_bottom`], [`Self::padding_left`] - Individual sides
    pub fn padding_shorthand(self, values: &[i32]) -> Self {
        let (top, right, bottom, left, ok) = which_sides_int(values);
        if !ok {
            return self;
        }
        self.padding(top, right, bottom, left)
    }

    /// Set margin using CSS-style shorthand notation.
    ///
    /// This method accepts 1-4 values and applies them using CSS shorthand rules:
    /// - 1 value: applies to all sides
    /// - 2 values: first is top/bottom, second is left/right
    /// - 3 values: first is top, second is left/right, third is bottom
    /// - 4 values: top, right, bottom, left (clockwise from top)
    ///
    /// Invalid input (0 or 5+ values) will be ignored and return the style unchanged.
    ///
    /// # Arguments
    ///
    /// * `values` - Slice of 1-4 margin values in CSS shorthand order
    ///
    /// # Returns
    ///
    /// The modified `Style` with margin properties set according to CSS shorthand rules.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use lipgloss::Style;
    ///
    /// // Uniform margin of 1 on all sides
    /// let uniform = Style::new().margin_shorthand(&[1]);
    ///
    /// // Vertical margin of 2, horizontal margin of 4
    /// let vh = Style::new().margin_shorthand(&[2, 4]);
    ///
    /// // Top 1, horizontal 0, bottom 2
    /// let asymmetric = Style::new().margin_shorthand(&[1, 0, 2]);
    ///
    /// // Clockwise from top: 2, 1, 3, 0
    /// let clockwise = Style::new().margin_shorthand(&[2, 1, 3, 0]);
    ///
    /// // Invalid (ignored): empty input
    /// let unchanged = Style::new().margin_shorthand(&[]);
    /// ```
    ///
    /// # See Also
    ///
    /// - [`Self::margin`] - For explicit four-side margin
    /// - [`Self::margin_top`], [`Self::margin_right`], [`Self::margin_bottom`], [`Self::margin_left`] - Individual sides
    pub fn margin_shorthand(self, values: &[i32]) -> Self {
        let (top, right, bottom, left, ok) = which_sides_int(values);
        if !ok {
            return self;
        }
        self.margin(top, right, bottom, left)
    }

    /// Set alignment using shorthand notation.
    ///
    /// This method accepts 1-2 position values:
    /// - 1 value: sets horizontal alignment only
    /// - 2 values: first is horizontal, second is vertical
    ///
    /// Invalid input (0 or 3+ values) will be ignored and return the style unchanged.
    ///
    /// # Arguments
    ///
    /// * `positions` - Slice of 1-2 position values
    ///
    /// # Returns
    ///
    /// The modified `Style` with alignment properties set.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use lipgloss::{Style, position::{LEFT, CENTER, RIGHT, TOP, BOTTOM}};
    ///
    /// // Center horizontally only
    /// let h_center = Style::new().align_shorthand(&[CENTER]);
    ///
    /// // Left horizontal, top vertical
    /// let left_top = Style::new().align_shorthand(&[LEFT, TOP]);
    ///
    /// // Right horizontal, bottom vertical
    /// let right_bottom = Style::new().align_shorthand(&[RIGHT, BOTTOM]);
    ///
    /// // Invalid (ignored): too many values
    /// let unchanged = Style::new().align_shorthand(&[LEFT, CENTER, RIGHT]);
    /// ```
    ///
    /// # See Also
    ///
    /// - [`Self::align`] - For explicit horizontal and vertical alignment
    /// - [`Self::align_horizontal`], [`Self::align_vertical`] - Individual alignment axes
    pub fn align_shorthand(self, positions: &[Position]) -> Self {
        match positions.len() {
            1 => self.align_horizontal(positions[0]),
            2 => self.align(positions[0], positions[1]),
            _ => self, // Invalid input, return unchanged
        }
    }

    // CSS-style aliases for Go compatibility

    /// CSS-style alias for [`padding_shorthand`].
    ///
    /// This method provides the exact same functionality as [`padding_shorthand`]
    /// but with a name that matches CSS padding syntax more closely. Both methods
    /// accept 1-4 values and apply CSS shorthand rules.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use lipgloss::Style;
    ///
    /// // These are equivalent:
    /// let style1 = Style::new().padding_css(&[2, 4]);
    /// let style2 = Style::new().padding_shorthand(&[2, 4]);
    /// ```
    ///
    /// [`padding_shorthand`]: Self::padding_shorthand
    pub fn padding_css(self, values: &[i32]) -> Self {
        self.padding_shorthand(values)
    }

    /// CSS-style alias for [`margin_shorthand`].
    ///
    /// This method provides the exact same functionality as [`margin_shorthand`]
    /// but with a name that matches CSS margin syntax more closely. Both methods
    /// accept 1-4 values and apply CSS shorthand rules.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use lipgloss::Style;
    ///
    /// // These are equivalent:
    /// let style1 = Style::new().margin_css(&[1, 2, 3, 4]);
    /// let style2 = Style::new().margin_shorthand(&[1, 2, 3, 4]);
    /// ```
    ///
    /// [`margin_shorthand`]: Self::margin_shorthand
    pub fn margin_css(self, values: &[i32]) -> Self {
        self.margin_shorthand(values)
    }

    /// Positional alias for [`align_shorthand`].
    ///
    /// This method provides the exact same functionality as [`align_shorthand`]
    /// but with a name that emphasizes the positional aspect. Both methods
    /// accept 1-2 position values.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use lipgloss::{Style, position::{CENTER, TOP}};
    ///
    /// // These are equivalent:
    /// let style1 = Style::new().align_pos(&[CENTER, TOP]);
    /// let style2 = Style::new().align_shorthand(&[CENTER, TOP]);
    /// ```
    ///
    /// [`align_shorthand`]: Self::align_shorthand
    pub fn align_pos(self, positions: &[Position]) -> Self {
        self.align_shorthand(positions)
    }
}
