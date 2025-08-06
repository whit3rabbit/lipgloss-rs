//! Property keys and constants for tracking which style properties are set.
//!
//! This module defines a bitfield-based system for efficiently tracking which
//! style properties have been explicitly set on a `Style` instance. Each property
//! is represented by a unique bit position, allowing for fast set/get operations
//! and compact storage.
//!
//! The system uses two main approaches:
//! - **Property Keys (`PropKey`)**: 64-bit values where each bit represents a specific property
//! - **Attribute Flags**: 32-bit values for boolean attributes that are stored as a bitfield
//!
//! # Design Rationale
//!
//! This approach allows the Style system to:
//! - Distinguish between "not set" and "set to default value"
//! - Efficiently check if a property has been explicitly configured
//! - Support style inheritance and composition
//! - Minimize memory usage through compact bitfield storage
//!
//! # Examples
//!
//! ```rust,ignore
//! // Internal usage - checking if a property is set
//! if self.props & BOLD_KEY != 0 {
//!     // Bold property has been explicitly set
//! }
//!
//! // Setting a property
//! self.props |= FOREGROUND_KEY;
//! ```

/// Type alias for property keys used in the Style bitfield system.
///
/// Each `PropKey` represents a unique bit position in a 64-bit integer,
/// allowing up to 64 different style properties to be tracked. The keys
/// are used with bitwise operations to efficiently set, check, and clear
/// property flags.
///
/// # Usage
///
/// ```rust,ignore
/// // Check if a property is set
/// let is_set = (style.props & SOME_PROPERTY_KEY) != 0;
///
/// // Set a property
/// style.props |= SOME_PROPERTY_KEY;
///
/// // Clear a property  
/// style.props &= !SOME_PROPERTY_KEY;
/// ```
pub(crate) type PropKey = u64;

// Text attribute properties - These track styling attributes that affect text rendering

/// Property key for bold text attribute.
///
/// When set, indicates that the bold styling has been explicitly configured,
/// regardless of whether it's enabled or disabled.
pub(crate) const BOLD_KEY: PropKey = 1 << 0;

/// Property key for italic text attribute.
///
/// When set, indicates that the italic styling has been explicitly configured,
/// regardless of whether it's enabled or disabled.
pub(crate) const ITALIC_KEY: PropKey = 1 << 1;

/// Property key for underline text attribute.
///
/// When set, indicates that the underline styling has been explicitly configured,
/// regardless of whether it's enabled or disabled.
pub(crate) const UNDERLINE_KEY: PropKey = 1 << 2;

/// Property key for strikethrough text attribute.
///
/// When set, indicates that the strikethrough styling has been explicitly configured,
/// regardless of whether it's enabled or disabled.
pub(crate) const STRIKETHROUGH_KEY: PropKey = 1 << 3;

/// Property key for reverse video text attribute.
///
/// When set, indicates that the reverse video styling (swapped foreground and background)
/// has been explicitly configured, regardless of whether it's enabled or disabled.
pub(crate) const REVERSE_KEY: PropKey = 1 << 4;

/// Property key for blinking text attribute.
///
/// When set, indicates that the blinking text styling has been explicitly configured,
/// regardless of whether it's enabled or disabled. Note that many modern terminals
/// don't support blinking text.
pub(crate) const BLINK_KEY: PropKey = 1 << 5;

/// Property key for faint (dim) text attribute.
///
/// When set, indicates that the faint/dim text styling has been explicitly configured,
/// regardless of whether it's enabled or disabled.
pub(crate) const FAINT_KEY: PropKey = 1 << 6;

/// Property key for underline spaces attribute.
///
/// When set, indicates that the "underline spaces" option has been explicitly configured.
/// This controls whether whitespace characters are also underlined when underline is enabled.
pub(crate) const UNDERLINE_SPACES_KEY: PropKey = 1 << 7;

/// Property key for strikethrough spaces attribute.
///
/// When set, indicates that the "strikethrough spaces" option has been explicitly configured.
/// This controls whether whitespace characters are also struck through when strikethrough is enabled.
pub(crate) const STRIKETHROUGH_SPACES_KEY: PropKey = 1 << 8;

/// Property key for color whitespace attribute.
///
/// When set, indicates that the "color whitespace" option has been explicitly configured.
/// This controls whether whitespace characters inherit foreground/background colors.
pub(crate) const COLOR_WHITESPACE_KEY: PropKey = 1 << 9;

// Color properties - These track whether colors have been explicitly set

/// Property key for foreground color.
///
/// When set, indicates that a foreground color has been explicitly configured,
/// even if it's set to a "no color" value.
pub(crate) const FOREGROUND_KEY: PropKey = 1 << 10;

/// Property key for background color.
///
/// When set, indicates that a background color has been explicitly configured,
/// even if it's set to a "no color" value.
pub(crate) const BACKGROUND_KEY: PropKey = 1 << 11;

// Size and alignment properties - These track dimensions and positioning

/// Property key for explicit width setting.
///
/// When set, indicates that a specific width has been explicitly configured,
/// overriding automatic width calculation.
pub(crate) const WIDTH_KEY: PropKey = 1 << 12;

/// Property key for explicit height setting.
///
/// When set, indicates that a specific height has been explicitly configured,
/// overriding automatic height calculation.
pub(crate) const HEIGHT_KEY: PropKey = 1 << 13;

/// Property key for horizontal alignment.
///
/// When set, indicates that horizontal alignment (left, center, right, or custom position)
/// has been explicitly configured.
pub(crate) const ALIGN_HORIZONTAL_KEY: PropKey = 1 << 14;

/// Property key for vertical alignment.
///
/// When set, indicates that vertical alignment (top, center, bottom, or custom position)
/// has been explicitly configured.
pub(crate) const ALIGN_VERTICAL_KEY: PropKey = 1 << 15;

// Padding properties - These track internal spacing configuration

/// Property key for top padding.
///
/// When set, indicates that top padding (space above content) has been
/// explicitly configured, even if set to zero.
pub(crate) const PADDING_TOP_KEY: PropKey = 1 << 16;

/// Property key for right padding.
///
/// When set, indicates that right padding (space to the right of content) has been
/// explicitly configured, even if set to zero.
pub(crate) const PADDING_RIGHT_KEY: PropKey = 1 << 17;

/// Property key for bottom padding.
///
/// When set, indicates that bottom padding (space below content) has been
/// explicitly configured, even if set to zero.
pub(crate) const PADDING_BOTTOM_KEY: PropKey = 1 << 18;

/// Property key for left padding.
///
/// When set, indicates that left padding (space to the left of content) has been
/// explicitly configured, even if set to zero.
pub(crate) const PADDING_LEFT_KEY: PropKey = 1 << 19;

// Margin properties - These track external spacing configuration

/// Property key for top margin.
///
/// When set, indicates that top margin (space above the styled box) has been
/// explicitly configured, even if set to zero.
pub(crate) const MARGIN_TOP_KEY: PropKey = 1 << 20;

/// Property key for right margin.
///
/// When set, indicates that right margin (space to the right of the styled box) has been
/// explicitly configured, even if set to zero.
pub(crate) const MARGIN_RIGHT_KEY: PropKey = 1 << 21;

/// Property key for bottom margin.
///
/// When set, indicates that bottom margin (space below the styled box) has been
/// explicitly configured, even if set to zero.
pub(crate) const MARGIN_BOTTOM_KEY: PropKey = 1 << 22;

/// Property key for left margin.
///
/// When set, indicates that left margin (space to the left of the styled box) has been
/// explicitly configured, even if set to zero.
pub(crate) const MARGIN_LEFT_KEY: PropKey = 1 << 23;

/// Property key for margin background color.
///
/// When set, indicates that a background color for margin areas has been
/// explicitly configured.
pub(crate) const MARGIN_BACKGROUND_KEY: PropKey = 1 << 24;

// Border properties - These track border configuration

/// Property key for border style.
///
/// When set, indicates that a border style (normal, rounded, thick, etc.) has been
/// explicitly configured.
pub(crate) const BORDER_STYLE_KEY: PropKey = 1 << 25;

// Border edge properties - These track which border edges are enabled

/// Property key for top border edge.
///
/// When set, indicates that the top border edge visibility has been
/// explicitly configured (enabled or disabled).
pub(crate) const BORDER_TOP_KEY: PropKey = 1 << 26;

/// Property key for right border edge.
///
/// When set, indicates that the right border edge visibility has been
/// explicitly configured (enabled or disabled).
pub(crate) const BORDER_RIGHT_KEY: PropKey = 1 << 27;

/// Property key for bottom border edge.
///
/// When set, indicates that the bottom border edge visibility has been
/// explicitly configured (enabled or disabled).
pub(crate) const BORDER_BOTTOM_KEY: PropKey = 1 << 28;

/// Property key for left border edge.
///
/// When set, indicates that the left border edge visibility has been
/// explicitly configured (enabled or disabled).
pub(crate) const BORDER_LEFT_KEY: PropKey = 1 << 29;

// Border foreground color properties - These track border text colors

/// Property key for top border foreground color.
///
/// When set, indicates that the foreground color for the top border edge
/// has been explicitly configured.
pub(crate) const BORDER_TOP_FOREGROUND_KEY: PropKey = 1 << 30;

/// Property key for right border foreground color.
///
/// When set, indicates that the foreground color for the right border edge
/// has been explicitly configured.
pub(crate) const BORDER_RIGHT_FOREGROUND_KEY: PropKey = 1 << 31;

/// Property key for bottom border foreground color.
///
/// When set, indicates that the foreground color for the bottom border edge
/// has been explicitly configured.
pub(crate) const BORDER_BOTTOM_FOREGROUND_KEY: PropKey = 1 << 32;

/// Property key for left border foreground color.
///
/// When set, indicates that the foreground color for the left border edge
/// has been explicitly configured.
pub(crate) const BORDER_LEFT_FOREGROUND_KEY: PropKey = 1 << 33;

// Border background color properties - These track border background colors

/// Property key for top border background color.
///
/// When set, indicates that the background color for the top border edge
/// has been explicitly configured.
pub(crate) const BORDER_TOP_BACKGROUND_KEY: PropKey = 1 << 34;

/// Property key for right border background color.
///
/// When set, indicates that the background color for the right border edge
/// has been explicitly configured.
pub(crate) const BORDER_RIGHT_BACKGROUND_KEY: PropKey = 1 << 35;

/// Property key for bottom border background color.
///
/// When set, indicates that the background color for the bottom border edge
/// has been explicitly configured.
pub(crate) const BORDER_BOTTOM_BACKGROUND_KEY: PropKey = 1 << 36;

/// Property key for left border background color.
///
/// When set, indicates that the background color for the left border edge
/// has been explicitly configured.
pub(crate) const BORDER_LEFT_BACKGROUND_KEY: PropKey = 1 << 37;

// Other properties - Miscellaneous style configuration flags

/// Property key for inline rendering mode.
///
/// When set, indicates that inline rendering mode has been explicitly configured.
/// Inline mode affects how the styled content is rendered in relation to surrounding text.
pub(crate) const INLINE_KEY: PropKey = 1 << 38;

/// Property key for maximum width constraint.
///
/// When set, indicates that a maximum width limit has been explicitly configured.
/// Content will be wrapped or truncated to fit within this width.
pub(crate) const MAX_WIDTH_KEY: PropKey = 1 << 39;

/// Property key for maximum height constraint.
///
/// When set, indicates that a maximum height limit has been explicitly configured.
/// Content will be truncated to fit within this height.
pub(crate) const MAX_HEIGHT_KEY: PropKey = 1 << 40;

/// Property key for tab width setting.
///
/// When set, indicates that a custom tab width has been explicitly configured,
/// overriding the default tab width for tab character rendering.
pub(crate) const TAB_WIDTH_KEY: PropKey = 1 << 41;

/// Property key for text transform function.
///
/// When set, indicates that a text transformation function has been explicitly configured.
/// This function will be applied to the content during rendering.
pub(crate) const TRANSFORM_KEY: PropKey = 1 << 42;

// Default values - These define standard default values for properties

/// Default tab width in characters.
///
/// This is the standard width used for tab characters when no custom tab width
/// has been configured. The value of 4 is commonly used in many text editors
/// and terminals.
pub(crate) const TAB_WIDTH_DEFAULT: i32 = 4;

// Attribute bitfield constants for Style struct - These store the actual boolean values
//
// Unlike the PropKey constants above (which track whether a property has been SET),
// these ATTR constants store the actual boolean values of attributes in a compact bitfield.
// They are used in the Style struct's `attrs` field for efficient storage and fast access.

/// Attribute flag for bold text.
///
/// When this bit is set in the Style's `attrs` field, bold text rendering is enabled.
pub(crate) const ATTR_BOLD: u32 = 1 << 0;

/// Attribute flag for italic text.
///
/// When this bit is set in the Style's `attrs` field, italic text rendering is enabled.
pub(crate) const ATTR_ITALIC: u32 = 1 << 1;

/// Attribute flag for underlined text.
///
/// When this bit is set in the Style's `attrs` field, underlined text rendering is enabled.
pub(crate) const ATTR_UNDERLINE: u32 = 1 << 2;

/// Attribute flag for strikethrough text.
///
/// When this bit is set in the Style's `attrs` field, strikethrough text rendering is enabled.
pub(crate) const ATTR_STRIKETHROUGH: u32 = 1 << 3;

/// Attribute flag for reverse video text.
///
/// When this bit is set in the Style's `attrs` field, reverse video (inverted colors) is enabled.
pub(crate) const ATTR_REVERSE: u32 = 1 << 4;

/// Attribute flag for blinking text.
///
/// When this bit is set in the Style's `attrs` field, blinking text rendering is enabled.
/// Note that many modern terminals don't support or ignore blinking text.
pub(crate) const ATTR_BLINK: u32 = 1 << 5;

/// Attribute flag for faint (dim) text.
///
/// When this bit is set in the Style's `attrs` field, faint/dim text rendering is enabled.
pub(crate) const ATTR_FAINT: u32 = 1 << 6;

/// Attribute flag for underlining whitespace characters.
///
/// When this bit is set in the Style's `attrs` field, whitespace characters (spaces, tabs)
/// will also be underlined when underline is enabled.
pub(crate) const ATTR_UNDERLINE_SPACES: u32 = 1 << 7;

/// Attribute flag for striking through whitespace characters.
///
/// When this bit is set in the Style's `attrs` field, whitespace characters (spaces, tabs)
/// will also be struck through when strikethrough is enabled.
pub(crate) const ATTR_STRIKETHROUGH_SPACES: u32 = 1 << 8;

/// Attribute flag for coloring whitespace characters.
///
/// When this bit is set in the Style's `attrs` field, whitespace characters will inherit
/// the foreground and background colors of the style.
pub(crate) const ATTR_COLOR_WHITESPACE: u32 = 1 << 9;

/// Attribute flag for inline rendering mode.
///
/// When this bit is set in the Style's `attrs` field, the content will be rendered
/// inline without adding extra whitespace or line breaks.
pub(crate) const ATTR_INLINE: u32 = 1 << 10;

/// Attribute flag for top border visibility.
///
/// When this bit is set in the Style's `attrs` field, the top border edge is visible.
pub(crate) const ATTR_BORDER_TOP: u32 = 1 << 11;

/// Attribute flag for right border visibility.
///
/// When this bit is set in the Style's `attrs` field, the right border edge is visible.
pub(crate) const ATTR_BORDER_RIGHT: u32 = 1 << 12;

/// Attribute flag for bottom border visibility.
///
/// When this bit is set in the Style's `attrs` field, the bottom border edge is visible.
pub(crate) const ATTR_BORDER_BOTTOM: u32 = 1 << 13;

/// Attribute flag for left border visibility.
///
/// When this bit is set in the Style's `attrs` field, the left border edge is visible.
pub(crate) const ATTR_BORDER_LEFT: u32 = 1 << 14;
