//! Core Style struct and basic implementation
//!
//! This module provides the primary [`Style`] struct, which is the main entry point
//! for styling terminal text. A Style defines all the visual properties that can be
//! applied to text, including colors, borders, padding, margins, alignment, and more.
//!
//! The [`Style`] struct uses a builder pattern for configuration and efficiently tracks
//! which properties have been explicitly set using internal bitfields. This allows for
//! optimal memory usage and performance when combining styles.
//!
//! # Key Concepts
//!
//! - **Builder Pattern**: All setter methods return `Self` for method chaining
//! - **Property Tracking**: Internal bitfields track which properties are explicitly set
//! - **Immutable Operations**: Methods create new instances rather than modifying existing ones
//! - **Flexible Rendering**: Support for custom renderers and text transforms

use crate::border::{hidden_border, Border};
use crate::position::{Position, LEFT, TOP};
use crate::renderer::Renderer;
use crate::style::properties::*;
use std::sync::Arc;

/// A comprehensive style definition for terminal text rendering.
///
/// `Style` contains all the rules and properties needed to render styled text in the terminal.
/// It uses a builder pattern for configuration and efficiently tracks which properties have
/// been explicitly set using internal bitfields.
///
/// # Features
///
/// - **Text Attributes**: Bold, italic, underline, strikethrough, blink, faint, reverse
/// - **Colors**: Foreground and background colors with full terminal color profile support
/// - **Layout**: Width, height, padding, margins, and alignment
/// - **Borders**: Configurable border styles with per-side colors
/// - **Advanced**: Text transforms and custom renderers
///
/// # Examples
///
/// ## Basic Usage
///
/// ```rust,no_run
/// use lipgloss::Style;
///
/// let style = Style::new()
///     .bold(true)
///     .foreground(lipgloss::Color::from("212"))
///     .background(lipgloss::Color::from("#FF5733"))
///     .padding(0, 2, 0, 2)
///     .margin(1, 0, 1, 0);
///
/// let styled_text = style.render("Hello, World!");
/// println!("{}", styled_text);
/// ```
///
/// ## Complex Styling
///
/// ```rust,no_run
/// use lipgloss::{Style, normal_border};
///
/// let card_style = Style::new()
///     .border(normal_border())
///     .border_foreground(lipgloss::Color::from("63"))
///     .padding(2, 4, 2, 4)
///     .width(40)
///     .align_horizontal(lipgloss::position::CENTER); // Center alignment
///
/// let content = "This is a centered card with a border";
/// println!("{}", card_style.render(content));
/// ```
///
/// ## Method Chaining
///
/// All setter methods return `Self`, enabling fluent method chaining:
///
/// ```rust
/// use lipgloss::Style;
///
/// let style = Style::new()
///     .bold(true)
///     .italic(true)
///     .underline(true)
///     .foreground(lipgloss::Color::from("9"))
///     .background(lipgloss::Color::from("21"))
///     .border_top(true);
/// ```
#[derive(Clone)]
pub struct Style {
    // Optional renderer
    pub(crate) r: Option<Renderer>,

    // Bitfield tracking which properties are set
    pub(crate) props: u64,

    // Underlying string value (for String()/SetString parity)
    pub(crate) value: String,

    // Store bool values as bitfield
    pub(crate) attrs: u32,

    // Colors - stored as strings for simplicity and cloning
    pub(crate) fg_color: Option<String>,
    pub(crate) bg_color: Option<String>,

    // Size constraints
    pub(crate) width: i32,
    pub(crate) height: i32,
    pub(crate) max_width: i32,
    pub(crate) max_height: i32,

    // Alignment
    pub(crate) align_horizontal: Position,
    pub(crate) align_vertical: Position,

    // Padding
    pub(crate) padding_top: i32,
    pub(crate) padding_right: i32,
    pub(crate) padding_bottom: i32,
    pub(crate) padding_left: i32,

    // Margins
    pub(crate) margin_top: i32,
    pub(crate) margin_right: i32,
    pub(crate) margin_bottom: i32,
    pub(crate) margin_left: i32,
    pub(crate) margin_bg_color: Option<String>,

    // Borders
    pub(crate) border_style: Border,
    pub(crate) border_top_fg_color: Option<String>,
    pub(crate) border_right_fg_color: Option<String>,
    pub(crate) border_bottom_fg_color: Option<String>,
    pub(crate) border_left_fg_color: Option<String>,
    pub(crate) border_top_bg_color: Option<String>,
    pub(crate) border_right_bg_color: Option<String>,
    pub(crate) border_bottom_bg_color: Option<String>,
    pub(crate) border_left_bg_color: Option<String>,

    // Misc
    pub(crate) tab_width: i32,
    pub(crate) transform: Option<Arc<dyn Fn(String) -> String + Send + Sync>>,
}

impl Default for Style {
    /// Creates a new [`Style`] with default values.
    ///
    /// This implementation provides sensible defaults for all style properties:
    /// - No colors set (inherit terminal defaults)
    /// - Zero padding and margins
    /// - Left-top alignment
    /// - Hidden border style
    /// - Default tab width
    /// - No text transforms
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use lipgloss::Style;
    ///
    /// let style = Style::default();
    /// let style2 = Style::new(); // Equivalent to default()
    /// ```
    fn default() -> Self {
        Self {
            r: None,
            props: 0,
            value: String::new(),
            attrs: 0,
            fg_color: None,
            bg_color: None,
            width: 0,
            height: 0,
            max_width: 0,
            max_height: 0,
            align_horizontal: LEFT,
            align_vertical: TOP,
            padding_top: 0,
            padding_right: 0,
            padding_bottom: 0,
            padding_left: 0,
            margin_top: 0,
            margin_right: 0,
            margin_bottom: 0,
            margin_left: 0,
            margin_bg_color: None,
            border_style: hidden_border(),
            border_top_fg_color: None,
            border_right_fg_color: None,
            border_bottom_fg_color: None,
            border_left_fg_color: None,
            border_top_bg_color: None,
            border_right_bg_color: None,
            border_bottom_bg_color: None,
            border_left_bg_color: None,
            tab_width: TAB_WIDTH_DEFAULT,
            transform: None,
        }
    }
}

impl Style {
    /// Checks if a specific property has been explicitly set on this style.
    ///
    /// This method uses bitfield operations to efficiently determine whether
    /// a property has been configured, which is useful for style inheritance
    /// and optimization.
    ///
    /// # Arguments
    ///
    /// * `k` - The property key to check
    ///
    /// # Returns
    ///
    /// Returns `true` if the property is set, `false` otherwise.
    pub(crate) fn is_set(&self, k: PropKey) -> bool {
        self.props & k != 0
    }

    /// Marks a property as explicitly set using bitfield operations.
    ///
    /// This method is used internally when style properties are configured
    /// to track which properties have been explicitly set vs. using defaults.
    ///
    /// # Arguments
    ///
    /// * `k` - The property key to mark as set
    pub(crate) fn set_prop(&mut self, k: PropKey) {
        self.props |= k;
    }

    /// Unmarks a property as set, effectively removing it from the style.
    ///
    /// This method clears the bit for the specified property, causing it to
    /// revert to default behavior or be ignored during style inheritance.
    ///
    /// # Arguments
    ///
    /// * `k` - The property key to unmark
    pub(crate) fn unset_prop(&mut self, k: PropKey) {
        self.props &= !k;
    }

    /// Retrieves the value of a boolean attribute using bitfield operations.
    ///
    /// Attributes represent boolean style properties like bold, italic, underline, etc.
    /// This method efficiently checks if a specific attribute bit is set.
    ///
    /// # Arguments
    ///
    /// * `bit` - The attribute bit to check
    ///
    /// # Returns
    ///
    /// Returns `true` if the attribute is enabled, `false` otherwise.
    pub(crate) fn get_attr(&self, bit: u32) -> bool {
        self.attrs & bit != 0
    }

    /// Sets or clears a boolean attribute using bitfield operations.
    ///
    /// This method efficiently updates attribute bits for boolean style properties
    /// like bold, italic, underline, etc. without affecting other attributes.
    ///
    /// # Arguments
    ///
    /// * `bit` - The attribute bit to modify
    /// * `value` - `true` to set the bit, `false` to clear it
    pub(crate) fn set_attr(&mut self, bit: u32, value: bool) {
        if value {
            self.attrs |= bit;
        } else {
            self.attrs &= !bit;
        }
    }

    /// Creates a new style with default settings.
    ///
    /// This creates a clean style with no properties set, ready for customization
    /// using the builder pattern methods. This is equivalent to [`Style::default()`].
    ///
    /// # Returns
    ///
    /// Returns a new [`Style`] instance with default values.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use lipgloss::Style;
    ///
    /// let style = Style::new();
    /// let styled = style.bold(true).foreground("red").render("Hello");
    /// ```
    ///
    /// Method chaining for building complex styles:
    ///
    /// ```rust
    /// use lipgloss::Style;
    ///
    /// let card_style = Style::new()
    ///     .padding(2, 2, 2, 2)
    ///     .margin(1, 1, 1, 1)
    ///     .background("blue")
    ///     .foreground("white")
    ///     .bold(true);
    /// ```
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the text content that this style will be applied to.
    ///
    /// This method allows you to associate specific text content with the style,
    /// which can then be rendered using the [`render`](Self::render) method or
    /// by converting the style to a string. The content is stored internally and
    /// can be accessed later using [`value`](Self::value) or [`string`](Self::string).
    ///
    /// # Arguments
    ///
    /// * `s` - The text content to associate with this style
    ///
    /// # Returns
    ///
    /// Returns `self` to enable method chaining.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use lipgloss::Style;
    ///
    /// // Set content and apply styling
    /// let styled_text = Style::new()
    ///     .bold(true)
    ///     .foreground("red")
    ///     .set_string("Important message")
    ///     .to_string();
    ///
    /// // Alternative approach using render
    /// let style = Style::new().bold(true).foreground("red");
    /// let styled_text = style.render("Important message");
    /// ```
    pub fn set_string(mut self, s: &str) -> Self {
        self.value = s.to_string();
        self
    }

    /// Returns a reference to the text content associated with this style.
    ///
    /// This method provides access to the underlying text content without
    /// consuming the style or creating a copy. If no content has been set
    /// using [`set_string`](Self::set_string), this returns an empty string.
    ///
    /// # Returns
    ///
    /// Returns a string slice reference to the stored text content.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use lipgloss::Style;
    ///
    /// let style = Style::new().set_string("Hello, World!");
    /// assert_eq!(style.value(), "Hello, World!");
    ///
    /// // Empty by default
    /// let empty_style = Style::new();
    /// assert_eq!(empty_style.value(), "");
    /// ```
    pub fn value(&self) -> &str {
        &self.value
    }

    /// Returns the text content as an owned string.
    ///
    /// This method returns a copy of the text content associated with the style.
    /// For accessing the content without copying, use [`value`](Self::value) instead.
    /// If no content has been set, this returns an empty string.
    ///
    /// # Returns
    ///
    /// Returns an owned [`String`] containing the stored text content.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use lipgloss::Style;
    ///
    /// let style = Style::new().set_string("Content");
    /// let content: String = style.string();
    /// assert_eq!(content, "Content");
    ///
    /// // Can be used for further processing
    /// let processed = content.to_uppercase();
    /// ```
    pub fn string(&self) -> String {
        self.value.clone()
    }
}

impl std::fmt::Debug for Style {
    /// Formats the [`Style`] for debugging purposes.
    ///
    /// This implementation provides a detailed view of all style properties,
    /// including internal state like property bitfields and attribute flags.
    /// The `transform` field is excluded as closures cannot be easily debugged.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use lipgloss::Style;
    ///
    /// let style = Style::new().bold(true).foreground("red");
    /// println!("{:?}", style); // Shows detailed style information
    /// ```
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Style")
            .field("props", &self.props)
            .field("value", &self.value)
            .field("attrs", &self.attrs)
            .field("fg_color", &self.fg_color)
            .field("bg_color", &self.bg_color)
            .field("width", &self.width)
            .field("height", &self.height)
            .field("max_width", &self.max_width)
            .field("max_height", &self.max_height)
            .field("align_horizontal", &self.align_horizontal)
            .field("align_vertical", &self.align_vertical)
            .field("padding_top", &self.padding_top)
            .field("padding_right", &self.padding_right)
            .field("padding_bottom", &self.padding_bottom)
            .field("padding_left", &self.padding_left)
            .field("margin_top", &self.margin_top)
            .field("margin_right", &self.margin_right)
            .field("margin_bottom", &self.margin_bottom)
            .field("margin_left", &self.margin_left)
            .field("margin_bg_color", &self.margin_bg_color)
            .field("border_style", &self.border_style)
            .field("border_top_fg_color", &self.border_top_fg_color)
            .field("border_right_fg_color", &self.border_right_fg_color)
            .field("border_bottom_fg_color", &self.border_bottom_fg_color)
            .field("border_left_fg_color", &self.border_left_fg_color)
            .field("border_top_bg_color", &self.border_top_bg_color)
            .field("border_right_bg_color", &self.border_right_bg_color)
            .field("border_bottom_bg_color", &self.border_bottom_bg_color)
            .field("border_left_bg_color", &self.border_left_bg_color)
            .field("tab_width", &self.tab_width)
            // skip transform
            .finish()
    }
}

impl std::fmt::Display for Style {
    /// Formats the [`Style`] for display by returning its text content.
    ///
    /// This implementation allows [`Style`] instances to be used directly in
    /// formatting contexts like `println!()` and `format!()`. It returns the
    /// text content associated with the style via [`set_string`](Self::set_string).
    ///
    /// Note: This only returns the raw text content without any styling applied.
    /// To get styled output, use the [`render`](Self::render) method instead.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use lipgloss::Style;
    ///
    /// let style = Style::new()
    ///     .bold(true)
    ///     .set_string("Hello, World!");
    ///
    /// println!("{}", style); // Prints: Hello, World!
    /// // Use render() for styled output:
    /// println!("{}", style.render(""));
    /// ```
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.string())
    }
}
