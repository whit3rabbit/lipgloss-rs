//! Style inheritance and copying methods.
//!
//! This module provides methods for creating new styles based on existing ones,
//! including selective property inheritance and style composition. These methods
//! are essential for building style hierarchies and creating consistent design
//! systems in terminal applications.
//!
//! # Key Concepts
//!
//! - **Property Inheritance**: Only explicitly set properties are inherited
//! - **Non-Overwriting**: Existing properties on the target style are preserved
//! - **Selective Copying**: Margins, padding, and content strings are not inherited
//! - **Go Compatibility**: Behavior matches the original Go implementation
//!
//! # Examples
//!
//! ```rust
//! use lipgloss::Style;
//! use lipgloss::color::Color;
//!
//! // Create a base style
//! let base = Style::new()
//!     .bold(true)
//!     .foreground(Color("blue".to_string()));
//!
//! // Create a specific style with additional properties
//! let specific = Style::new()
//!     .italic(true)
//!     .padding(1, 2, 1, 2);
//!
//! // Inherit from base - gets bold and blue color, keeps italic and padding
//! let combined = specific.inherit(base);
//! ```

use crate::style::{properties::*, Style};

impl Style {
    /// Inherit properties from another style, creating a new style with combined attributes.
    ///
    /// This method implements selective property inheritance where only explicitly set
    /// properties from the source style are copied to the target style. The inheritance
    /// process is non-destructive - existing properties on the target style are never
    /// overwritten, ensuring that explicit styling choices are preserved.
    ///
    /// # Inheritance Rules
    ///
    /// 1. **Only Explicitly Set Properties**: A property is only inherited if it was
    ///    explicitly set on the source style using a setter method.
    /// 2. **No Overwriting**: If the target style already has a property set, it won't
    ///    be overwritten by the source style's value.
    /// 3. **Excluded Properties**: Margins, padding, and underlying string values are
    ///    never inherited to maintain layout independence.
    /// 4. **Special Background Handling**: Background colors may set margin background
    ///    colors under specific conditions.
    ///
    /// # Properties That Are Inherited
    ///
    /// - **Text Attributes**: bold, italic, underline, strikethrough, reverse, blink, faint
    /// - **Text Behavior**: underline_spaces, strikethrough_spaces, color_whitespace
    /// - **Colors**: foreground, background (with special margin background logic)
    /// - **Dimensions**: width, height, max_width, max_height
    /// - **Alignment**: horizontal and vertical alignment
    /// - **Borders**: style, edge visibility, foreground/background colors
    /// - **Rendering**: inline mode, tab_width, transform functions
    ///
    /// # Properties That Are NOT Inherited
    ///
    /// - **Spacing**: All padding and margin values
    /// - **Content**: Underlying string values
    ///
    /// # Arguments
    ///
    /// * `other` - The source style to inherit properties from
    ///
    /// # Returns
    ///
    /// A new `Style` instance with the combined properties of both styles,
    /// where the target style's properties take precedence.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use lipgloss::Style;
    /// use lipgloss::color::Color;
    ///
    /// // Create a base theme style
    /// let theme = Style::new()
    ///     .bold(true)
    ///     .foreground(Color("#3498db".to_string()))
    ///     .padding(1, 2, 1, 2);  // This won't be inherited
    ///
    /// // Create a component style with specific properties
    /// let component = Style::new()
    ///     .italic(true)
    ///     .border(lipgloss::normal_border())
    ///     .foreground(Color("red".to_string()));  // This takes precedence
    ///
    /// // Inherit theme properties into component
    /// let final_style = component.inherit(theme);
    ///
    /// // Result: italic (from component), red foreground (component wins),
    /// // bold (from theme), border (from component), no padding (not inherited)
    /// ```
    ///
    /// ## Building Style Hierarchies
    ///
    /// ```rust
    /// use lipgloss::Style;
    /// use lipgloss::color::Color;
    ///
    /// // Application-wide defaults
    /// let app_defaults = Style::new()
    ///     .foreground(Color("#ffffff".to_string()))
    ///     .tab_width(4);
    ///
    /// // Section-specific styling
    /// let header_style = Style::new()
    ///     .bold(true)
    ///     .underline(true)
    ///     .inherit(app_defaults.clone());
    ///
    /// // Component-specific styling
    /// let title_style = Style::new()
    ///     .italic(true)
    ///     .foreground(Color("#f39c12".to_string()))  // Override default
    ///     .inherit(header_style);
    ///
    /// // Final style has: italic, bold, underline, orange foreground, tab_width=4
    /// ```
    ///
    /// ## Conditional Inheritance
    ///
    /// ```rust
    /// use lipgloss::Style;
    /// use lipgloss::color::Color;
    ///
    /// let base_style = Style::new().bold(true);
    /// let mut target_style = Style::new();
    ///
    /// // Only inherit if target doesn't have the property
    /// target_style = target_style.inherit(base_style);  // Gets bold=true
    ///
    /// let another_base = Style::new().bold(false);
    /// target_style = target_style.inherit(another_base); // Keeps bold=true (no override)
    /// ```
    pub fn inherit(mut self, other: Style) -> Self {
        // Iterate through all property keys to check what's set in the other style
        let property_keys = [
            BOLD_KEY,
            ITALIC_KEY,
            UNDERLINE_KEY,
            STRIKETHROUGH_KEY,
            REVERSE_KEY,
            BLINK_KEY,
            FAINT_KEY,
            UNDERLINE_SPACES_KEY,
            STRIKETHROUGH_SPACES_KEY,
            COLOR_WHITESPACE_KEY,
            FOREGROUND_KEY,
            BACKGROUND_KEY,
            WIDTH_KEY,
            HEIGHT_KEY,
            ALIGN_HORIZONTAL_KEY,
            ALIGN_VERTICAL_KEY,
            BORDER_STYLE_KEY,
            BORDER_TOP_KEY,
            BORDER_RIGHT_KEY,
            BORDER_BOTTOM_KEY,
            BORDER_LEFT_KEY,
            BORDER_TOP_FOREGROUND_KEY,
            BORDER_RIGHT_FOREGROUND_KEY,
            BORDER_BOTTOM_FOREGROUND_KEY,
            BORDER_LEFT_FOREGROUND_KEY,
            BORDER_TOP_BACKGROUND_KEY,
            BORDER_RIGHT_BACKGROUND_KEY,
            BORDER_BOTTOM_BACKGROUND_KEY,
            BORDER_LEFT_BACKGROUND_KEY,
            INLINE_KEY,
            MAX_WIDTH_KEY,
            MAX_HEIGHT_KEY,
            TAB_WIDTH_KEY,
            TRANSFORM_KEY,
            // Skip padding and margin keys as they are not inherited per Go implementation
        ];

        for &key in &property_keys {
            if !other.is_set(key) {
                continue; // Other style doesn't have this property set
            }

            // Skip margins and padding as per Go implementation
            if matches!(
                key,
                PADDING_TOP_KEY
                    | PADDING_RIGHT_KEY
                    | PADDING_BOTTOM_KEY
                    | PADDING_LEFT_KEY
                    | MARGIN_TOP_KEY
                    | MARGIN_RIGHT_KEY
                    | MARGIN_BOTTOM_KEY
                    | MARGIN_LEFT_KEY
            ) {
                continue;
            }

            // Special handling for background color - inherit margin background if we don't have it
            if key == BACKGROUND_KEY
                && !self.is_set(MARGIN_BACKGROUND_KEY)
                && !other.is_set(MARGIN_BACKGROUND_KEY)
            {
                if let Some(ref bg) = other.bg_color {
                    self.margin_bg_color = Some(bg.clone());
                    self.set_prop(MARGIN_BACKGROUND_KEY);
                }
            }

            if self.is_set(key) {
                continue; // We already have this property set, don't override
            }

            // Copy the property from other to self
            match key {
                BOLD_KEY => {
                    self.set_attr(ATTR_BOLD, other.get_attr(ATTR_BOLD));
                    self.set_prop(key);
                }
                ITALIC_KEY => {
                    self.set_attr(ATTR_ITALIC, other.get_attr(ATTR_ITALIC));
                    self.set_prop(key);
                }
                UNDERLINE_KEY => {
                    self.set_attr(ATTR_UNDERLINE, other.get_attr(ATTR_UNDERLINE));
                    self.set_prop(key);
                }
                STRIKETHROUGH_KEY => {
                    self.set_attr(ATTR_STRIKETHROUGH, other.get_attr(ATTR_STRIKETHROUGH));
                    self.set_prop(key);
                }
                REVERSE_KEY => {
                    self.set_attr(ATTR_REVERSE, other.get_attr(ATTR_REVERSE));
                    self.set_prop(key);
                }
                BLINK_KEY => {
                    self.set_attr(ATTR_BLINK, other.get_attr(ATTR_BLINK));
                    self.set_prop(key);
                }
                FAINT_KEY => {
                    self.set_attr(ATTR_FAINT, other.get_attr(ATTR_FAINT));
                    self.set_prop(key);
                }
                UNDERLINE_SPACES_KEY => {
                    self.set_attr(ATTR_UNDERLINE_SPACES, other.get_attr(ATTR_UNDERLINE_SPACES));
                    self.set_prop(key);
                }
                STRIKETHROUGH_SPACES_KEY => {
                    self.set_attr(
                        ATTR_STRIKETHROUGH_SPACES,
                        other.get_attr(ATTR_STRIKETHROUGH_SPACES),
                    );
                    self.set_prop(key);
                }
                COLOR_WHITESPACE_KEY => {
                    self.set_attr(ATTR_COLOR_WHITESPACE, other.get_attr(ATTR_COLOR_WHITESPACE));
                    self.set_prop(key);
                }
                FOREGROUND_KEY => {
                    self.fg_color = other.fg_color.clone();
                    self.set_prop(key);
                }
                BACKGROUND_KEY => {
                    self.bg_color = other.bg_color.clone();
                    self.set_prop(key);
                }
                WIDTH_KEY => {
                    self.width = other.width;
                    self.set_prop(key);
                }
                HEIGHT_KEY => {
                    self.height = other.height;
                    self.set_prop(key);
                }
                ALIGN_HORIZONTAL_KEY => {
                    self.align_horizontal = other.align_horizontal;
                    self.set_prop(key);
                }
                ALIGN_VERTICAL_KEY => {
                    self.align_vertical = other.align_vertical;
                    self.set_prop(key);
                }
                BORDER_STYLE_KEY => {
                    self.border_style = other.border_style;
                    self.set_prop(key);
                }
                BORDER_TOP_KEY => {
                    self.set_attr(ATTR_BORDER_TOP, other.get_attr(ATTR_BORDER_TOP));
                    self.set_prop(key);
                }
                BORDER_RIGHT_KEY => {
                    self.set_attr(ATTR_BORDER_RIGHT, other.get_attr(ATTR_BORDER_RIGHT));
                    self.set_prop(key);
                }
                BORDER_BOTTOM_KEY => {
                    self.set_attr(ATTR_BORDER_BOTTOM, other.get_attr(ATTR_BORDER_BOTTOM));
                    self.set_prop(key);
                }
                BORDER_LEFT_KEY => {
                    self.set_attr(ATTR_BORDER_LEFT, other.get_attr(ATTR_BORDER_LEFT));
                    self.set_prop(key);
                }
                BORDER_TOP_FOREGROUND_KEY => {
                    self.border_top_fg_color = other.border_top_fg_color.clone();
                    self.set_prop(key);
                }
                BORDER_RIGHT_FOREGROUND_KEY => {
                    self.border_right_fg_color = other.border_right_fg_color.clone();
                    self.set_prop(key);
                }
                BORDER_BOTTOM_FOREGROUND_KEY => {
                    self.border_bottom_fg_color = other.border_bottom_fg_color.clone();
                    self.set_prop(key);
                }
                BORDER_LEFT_FOREGROUND_KEY => {
                    self.border_left_fg_color = other.border_left_fg_color.clone();
                    self.set_prop(key);
                }
                BORDER_TOP_BACKGROUND_KEY => {
                    self.border_top_bg_color = other.border_top_bg_color.clone();
                    self.set_prop(key);
                }
                BORDER_RIGHT_BACKGROUND_KEY => {
                    self.border_right_bg_color = other.border_right_bg_color.clone();
                    self.set_prop(key);
                }
                BORDER_BOTTOM_BACKGROUND_KEY => {
                    self.border_bottom_bg_color = other.border_bottom_bg_color.clone();
                    self.set_prop(key);
                }
                BORDER_LEFT_BACKGROUND_KEY => {
                    self.border_left_bg_color = other.border_left_bg_color.clone();
                    self.set_prop(key);
                }
                INLINE_KEY => {
                    self.set_attr(ATTR_INLINE, other.get_attr(ATTR_INLINE));
                    self.set_prop(key);
                }
                MAX_WIDTH_KEY => {
                    self.max_width = other.max_width;
                    self.set_prop(key);
                }
                MAX_HEIGHT_KEY => {
                    self.max_height = other.max_height;
                    self.set_prop(key);
                }
                TAB_WIDTH_KEY => {
                    self.tab_width = other.tab_width;
                    self.set_prop(key);
                }
                TRANSFORM_KEY => {
                    self.transform = other.transform.clone();
                    self.set_prop(key);
                }
                _ => {} // Unknown key, skip
            }
        }

        self
    }

    /// Create a copy of this style (deprecated - use `clone()` instead).
    ///
    /// This method creates an identical copy of the current style, including all
    /// properties, attributes, and values. It's a thin wrapper around Rust's
    /// `Clone` trait implementation.
    ///
    /// # Deprecation Notice
    ///
    /// This method is deprecated to match the Go implementation and because Rust's
    /// `Clone` trait provides the same functionality more idiomatically. Use
    /// `.clone()` or simple assignment for copying styles.
    ///
    /// # Returns
    ///
    /// An identical copy of this `Style` instance.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use lipgloss::Style;
    /// use lipgloss::color::Color;
    ///
    /// let original = Style::new()
    ///     .bold(true)
    ///     .foreground(Color("blue".to_string()))
    ///     .padding(1, 2, 1, 2);
    ///
    /// // Deprecated way (avoid this)
    /// #[allow(deprecated)]
    /// let copy1 = original.copy();
    ///
    /// // Preferred ways
    /// let copy2 = original.clone();
    /// let copy3 = original.clone();
    ///
    /// // original is still usable here for more operations
    /// let final_copy = original.clone();
    /// // original is still usable here
    /// ```
    ///
    /// # Migration Guide
    ///
    /// Replace `style.copy()` with `style.clone()` in your code:
    ///
    /// ```rust
    /// use lipgloss::Style;
    ///
    /// let style = Style::new().bold(true);
    ///
    /// // Old (deprecated)
    /// #[allow(deprecated)]
    /// let old_way = style.copy();
    ///
    /// // New (recommended)
    /// let new_way = style.clone();
    /// ```
    #[deprecated(note = "Use clone() or assignment instead")]
    pub fn copy(&self) -> Self {
        self.clone()
    }
}
