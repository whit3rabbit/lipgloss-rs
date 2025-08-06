//! Size constraint and dimension methods for Style.
//!
//! This module provides methods for controlling the dimensions of styled content,
//! including explicit sizing and maximum size constraints. These methods are
//! essential for creating consistent layouts and ensuring content fits within
//! desired boundaries in terminal applications.
//!
//! # Key Concepts
//!
//! - **Explicit Sizing**: Set exact width and height dimensions
//! - **Maximum Constraints**: Set upper limits for content expansion
//! - **Content Interaction**: How sizing affects text wrapping and truncation
//! - **Layout Impact**: How dimensions affect alignment and positioning
//!
//! # Dimension Types
//!
//! - **Width**: Controls horizontal space (character columns)
//! - **Height**: Controls vertical space (text lines)
//! - **Max Width**: Upper limit for automatic width calculation
//! - **Max Height**: Upper limit for automatic height calculation
//!
//! # Examples
//!
//! ```rust
//! use lipgloss::Style;
//!
//! // Fixed dimensions
//! let fixed_box = Style::new()
//!     .width(20)
//!     .height(10)
//!     .render("Content");
//!
//! // Maximum constraints
//! let constrained = Style::new()
//!     .max_width(50)
//!     .max_height(5)
//!     .render("This text will wrap or truncate if it exceeds the limits");
//!
//! // Combined with other styling
//! let styled_box = Style::new()
//!     .width(30)
//!     .border(lipgloss::normal_border())
//!     .padding(1, 2, 1, 2)
//!     .render("Styled content in a fixed-width box");
//! ```

use crate::security::validate_dimension;
use crate::style::{properties::*, Style};

impl Style {
    /// Set the explicit width for the styled content.
    ///
    /// This method sets a fixed width for the styled content, measured in character
    /// columns. When an explicit width is set, the content will be formatted to fit
    /// within this width, with text wrapping or truncation as necessary. The width
    /// includes padding but excludes borders and margins.
    ///
    /// # Behavior
    ///
    /// - **Text Wrapping**: Long lines will wrap to fit within the specified width
    /// - **Padding Inclusion**: The width includes any horizontal padding
    /// - **Border Exclusion**: Borders add to the total rendered width
    /// - **Alignment**: Content alignment works within the specified width
    /// - **Minimum Size**: Width of 0 or negative values may cause unexpected behavior
    ///
    /// # Arguments
    ///
    /// * `w` - The width in character columns (should be positive)
    ///
    /// # Returns
    ///
    /// The modified `Style` with the width constraint set.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use lipgloss::Style;
    ///
    /// // Basic width setting
    /// let narrow = Style::new()
    ///     .width(10)
    ///     .render("This text will wrap");
    ///
    /// // Width with padding
    /// let padded = Style::new()
    ///     .width(20)
    ///     .padding(0, 2, 0, 2)  // 2 chars padding on each side
    ///     .render("Content");    // Actual content area: 16 chars
    ///
    /// // Width with borders
    /// let bordered = Style::new()
    ///     .width(15)
    ///     .border(lipgloss::normal_border())  // Adds 2 chars to total width
    ///     .render("Text content");
    /// ```
    ///
    /// ## Layout Interaction
    ///
    /// ```rust
    /// use lipgloss::{Style, position::CENTER};
    ///
    /// // Width enables horizontal alignment
    /// let centered = Style::new()
    ///     .width(30)
    ///     .align_horizontal(CENTER)
    ///     .render("Centered text");
    ///
    /// // Width with text wrapping
    /// let wrapped = Style::new()
    ///     .width(12)
    ///     .render("This is a long line that will wrap to multiple lines");
    /// ```
    ///
    /// ## Common Patterns
    ///
    /// ```rust
    /// use lipgloss::Style;
    /// use lipgloss::color::Color;
    ///
    /// // Card-like component with fixed width
    /// let card = Style::new()
    ///     .width(40)
    ///     .border(lipgloss::rounded_border())
    ///     .padding(1, 2, 1, 2)
    ///     .background(Color("#f0f0f0".to_string()))
    ///     .render("Card content with consistent width");
    ///
    /// // Column layout
    /// let column = Style::new()
    ///     .width(25)
    ///     .render("Column 1 content");
    /// ```
    pub fn width(mut self, w: i32) -> Self {
        self.width = validate_dimension(w, "width");
        self.set_prop(WIDTH_KEY);
        self
    }

    /// Set the explicit height for the styled content.
    ///
    /// This method sets a fixed height for the styled content, measured in text lines.
    /// When an explicit height is set, the content will be formatted to fit within
    /// this height, with vertical alignment and truncation as necessary. The height
    /// includes padding but excludes borders and margins.
    ///
    /// # Behavior
    ///
    /// - **Content Truncation**: Content exceeding the height will be truncated
    /// - **Vertical Alignment**: Content can be aligned within the specified height
    /// - **Padding Inclusion**: The height includes any vertical padding
    /// - **Border Exclusion**: Borders add to the total rendered height
    /// - **Empty Space**: Heights larger than content create whitespace
    ///
    /// # Arguments
    ///
    /// * `h` - The height in text lines (should be positive)
    ///
    /// # Returns
    ///
    /// The modified `Style` with the height constraint set.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use lipgloss::Style;
    ///
    /// // Basic height setting
    /// let tall_box = Style::new()
    ///     .height(5)
    ///     .render("Content\nLine 2\nLine 3");
    ///
    /// // Height with padding
    /// let padded = Style::new()
    ///     .height(8)
    ///     .padding(1, 0, 1, 0)  // 1 line padding top/bottom
    ///     .render("Content");    // Actual content area: 6 lines
    ///
    /// // Height with borders
    /// let bordered = Style::new()
    ///     .height(6)
    ///     .border(lipgloss::normal_border())  // Adds 2 lines to total height
    ///     .render("Text\ncontent");
    /// ```
    ///
    /// ## Vertical Alignment
    ///
    /// ```rust
    /// use lipgloss::{Style, position::{TOP, CENTER, BOTTOM}};
    ///
    /// // Top-aligned content in tall box
    /// let top_aligned = Style::new()
    ///     .height(10)
    ///     .align_vertical(TOP)
    ///     .render("Top content");
    ///
    /// // Centered content
    /// let centered = Style::new()
    ///     .height(8)
    ///     .align_vertical(CENTER)
    ///     .render("Centered\ncontent");
    ///
    /// // Bottom-aligned content
    /// let bottom_aligned = Style::new()
    ///     .height(6)
    ///     .align_vertical(BOTTOM)
    ///     .render("Bottom content");
    /// ```
    ///
    /// ## Content Management
    ///
    /// ```rust
    /// use lipgloss::Style;
    ///
    /// // Truncation behavior
    /// let truncated = Style::new()
    ///     .height(3)
    ///     .render("Line 1\nLine 2\nLine 3\nLine 4\nLine 5");  // Lines 4-5 truncated
    ///
    /// // Creating consistent height panels
    /// let panel = Style::new()
    ///     .width(30)
    ///     .height(12)
    ///     .border(lipgloss::rounded_border())
    ///     .padding(1, 2, 1, 2)
    ///     .render("Panel content with fixed dimensions");
    /// ```
    pub fn height(mut self, h: i32) -> Self {
        self.height = validate_dimension(h, "height");
        self.set_prop(HEIGHT_KEY);
        self
    }

    /// Set the maximum width constraint for the styled content.
    ///
    /// This method sets an upper limit for the width of styled content, measured in
    /// character columns. Unlike `width()`, this doesn't force a specific width but
    /// instead prevents the content from exceeding the specified maximum. Content
    /// narrower than the maximum will retain its natural width.
    ///
    /// # Behavior
    ///
    /// - **Constraint Only**: Content narrower than max_width keeps its natural width
    /// - **Text Wrapping**: Long lines wrap when they would exceed the maximum
    /// - **Dynamic Sizing**: Final width depends on content and constraints
    /// - **Padding Interaction**: Maximum width includes horizontal padding
    /// - **Border Independence**: Borders are added outside the maximum width
    ///
    /// # Arguments
    ///
    /// * `w` - The maximum width in character columns (should be positive)
    ///
    /// # Returns
    ///
    /// The modified `Style` with the maximum width constraint set.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use lipgloss::Style;
    ///
    /// // Basic maximum width
    /// let constrained = Style::new()
    ///     .max_width(25)
    ///     .render("This long text will wrap when it exceeds 25 characters per line");
    ///
    /// // Short content keeps natural width
    /// let short = Style::new()
    ///     .max_width(50)
    ///     .render("Short");  // Will be narrower than 50 chars
    ///
    /// // Maximum width with padding
    /// let padded = Style::new()
    ///     .max_width(30)
    ///     .padding(0, 3, 0, 3)  // 6 chars total horizontal padding
    ///     .render("Content");   // Effective content area: up to 24 chars
    /// ```
    ///
    /// ## Responsive Design
    ///
    /// ```rust
    /// use lipgloss::Style;
    ///
    /// // Responsive text blocks
    /// let responsive = Style::new()
    ///     .max_width(60)
    ///     .border(lipgloss::normal_border())
    ///     .padding(1, 2, 1, 2)
    ///     .render("This content will wrap nicely within 60 characters, but shorter content won't be forced to that width");
    ///
    /// // Flexible containers
    /// let flexible = Style::new()
    ///     .max_width(40)
    ///     .render("Adapts to content size up to 40 chars");
    /// ```
    ///
    /// ## Comparison with Fixed Width
    ///
    /// ```rust
    /// use lipgloss::Style;
    ///
    /// // Fixed width - always 20 characters
    /// let fixed = Style::new()
    ///     .width(20)
    ///     .render("Hi");  // Padded to 20 chars
    ///
    /// // Maximum width - only as wide as needed
    /// let flexible = Style::new()
    ///     .max_width(20)
    ///     .render("Hi");  // Only 2 chars wide
    ///
    /// // Maximum width with long content
    /// let wrapped = Style::new()
    ///     .max_width(20)
    ///     .render("This is a very long line that will wrap");
    /// ```
    ///
    /// ## Layout Applications
    ///
    /// ```rust
    /// use lipgloss::Style;
    /// use lipgloss::color::Color;
    ///
    /// // Flexible card components
    /// let card = Style::new()
    ///     .max_width(50)
    ///     .border(lipgloss::rounded_border())
    ///     .padding(1, 2, 1, 2)
    ///     .background(Color("#f8f9fa".to_string()))
    ///     .render("Dynamic card content that adapts to content length");
    ///
    /// // Constrained text areas
    /// let text_area = Style::new()
    ///     .max_width(80)
    ///     .render("Long form text content that should wrap at reasonable line lengths for readability");
    /// ```
    pub fn max_width(mut self, w: i32) -> Self {
        self.max_width = validate_dimension(w, "max_width");
        self.set_prop(MAX_WIDTH_KEY);
        self
    }

    /// Set the maximum height constraint for the styled content.
    ///
    /// This method sets an upper limit for the height of styled content, measured in
    /// text lines. Unlike `height()`, this doesn't force a specific height but instead
    /// prevents the content from exceeding the specified maximum. Content shorter than
    /// the maximum will retain its natural height.
    ///
    /// # Behavior
    ///
    /// - **Constraint Only**: Content shorter than max_height keeps its natural height
    /// - **Content Truncation**: Excess lines are truncated when maximum is exceeded
    /// - **Dynamic Sizing**: Final height depends on content and constraints
    /// - **Padding Interaction**: Maximum height includes vertical padding
    /// - **Border Independence**: Borders are added outside the maximum height
    ///
    /// # Arguments
    ///
    /// * `h` - The maximum height in text lines (should be positive)
    ///
    /// # Returns
    ///
    /// The modified `Style` with the maximum height constraint set.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use lipgloss::Style;
    ///
    /// // Basic maximum height
    /// let constrained = Style::new()
    ///     .max_height(5)
    ///     .render("Line 1\nLine 2\nLine 3\nLine 4\nLine 5\nLine 6\nLine 7");
    /// // Only first 5 lines will be shown
    ///
    /// // Short content keeps natural height
    /// let short = Style::new()
    ///     .max_height(10)
    ///     .render("Short\ncontent");  // Will be only 2 lines tall
    ///
    /// // Maximum height with padding
    /// let padded = Style::new()
    ///     .max_height(8)
    ///     .padding(1, 0, 1, 0)  // 2 lines total vertical padding
    ///     .render("Content\nLine 2\nLine 3\nLine 4\nLine 5\nLine 6");
    /// // Effective content area: up to 6 lines
    /// ```
    ///
    /// ## Content Management
    ///
    /// ```rust
    /// use lipgloss::Style;
    ///
    /// // Scrollable content areas
    /// let scrollable = Style::new()
    ///     .max_height(6)
    ///     .border(lipgloss::normal_border())
    ///     .render("Line 1\nLine 2\nLine 3\nLine 4\nLine 5\nLine 6\nLine 7\nLine 8");
    /// // Shows first 6 lines, indicates more content available
    ///
    /// // Flexible containers
    /// let flexible = Style::new()
    ///     .max_height(12)
    ///     .render("Adapts to content height up to 12 lines");
    /// ```
    ///
    /// ## Comparison with Fixed Height
    ///
    /// ```rust
    /// use lipgloss::Style;
    ///
    /// // Fixed height - always 5 lines
    /// let fixed = Style::new()
    ///     .height(5)
    ///     .render("Line 1\nLine 2");  // Padded to 5 lines with whitespace
    ///
    /// // Maximum height - only as tall as needed
    /// let flexible = Style::new()
    ///     .max_height(5)
    ///     .render("Line 1\nLine 2");  // Only 2 lines tall
    ///
    /// // Maximum height with long content
    /// let truncated = Style::new()
    ///     .max_height(3)
    ///     .render("Line 1\nLine 2\nLine 3\nLine 4\nLine 5");  // Truncated to 3 lines
    /// ```
    ///
    /// ## Layout Applications
    ///
    /// ```rust
    /// use lipgloss::Style;
    /// use lipgloss::color::Color;
    ///
    /// // Preview panels with content limits
    /// let preview = Style::new()
    ///     .max_width(40)
    ///     .max_height(8)
    ///     .border(lipgloss::rounded_border())
    ///     .padding(1, 2, 1, 2)
    ///     .background(Color("#f0f0f0".to_string()))
    ///     .render("Preview content that may be truncated if too long...");
    ///
    /// // Constrained output areas
    /// let output = Style::new()
    ///     .max_height(15)
    ///     .render("Command output or log content with height limits");
    /// ```
    pub fn max_height(mut self, h: i32) -> Self {
        self.max_height = validate_dimension(h, "max_height");
        self.set_prop(MAX_HEIGHT_KEY);
        self
    }
}
