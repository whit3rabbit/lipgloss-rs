//! Transformation and tab-related methods for Style
//!
//! This module provides methods for configuring text transformation, tab width handling,
//! and custom rendering for styles. These methods allow you to:
//!
//! - Set custom tab width for consistent spacing
//! - Apply transformation functions to modify text during rendering
//! - Use custom renderers for specialized output contexts
//!
//! # Examples
//!
//! ```rust
//! use lipgloss::Style;
//!
//! // Create a style with custom tab width
//! let style = Style::new().tab_width(4);
//!
//! // Add text transformation
//! let uppercase_style = Style::new()
//!     .transform(|text| text.to_uppercase());
//! ```

use crate::renderer::Renderer;
use crate::security::validate_tab_width;
use crate::style::{properties::*, Style};
use std::sync::Arc;

impl Style {
    /// Sets the tab width for this style.
    ///
    /// This determines how many spaces a tab character (`\t`) should be expanded to
    /// when the text is rendered. This is particularly useful for maintaining consistent
    /// indentation and alignment in terminal output.
    ///
    /// # Arguments
    ///
    /// * `n` - The number of spaces to use for each tab character. Must be non-negative.
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
    /// // Set tab width to 4 spaces (common programming convention)
    /// let style = Style::new().tab_width(4);
    ///
    /// // Render text with tabs
    /// let text = "Line 1\n\tIndented line";
    /// let rendered = style.render(text);
    /// // The tab will be rendered as 4 spaces
    /// ```
    ///
    /// # Note
    ///
    /// The default tab width varies by terminal and context. Setting an explicit
    /// tab width ensures consistent rendering across different environments.
    pub fn tab_width(mut self, n: i32) -> Self {
        self.tab_width = validate_tab_width(n);
        self.set_prop(TAB_WIDTH_KEY);
        self
    }

    /// Sets a transformation function that will be applied to text during rendering.
    ///
    /// The transformation function allows you to modify the text content before it's
    /// styled and rendered. This is useful for text processing like case conversion,
    /// formatting, filtering, or any custom text manipulation.
    ///
    /// # Arguments
    ///
    /// * `f` - A function that takes a `String` and returns a transformed `String`.
    ///   The function must be `Send + Sync + 'static` to support concurrent
    ///   rendering and storage in the style.
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
    /// // Convert text to uppercase
    /// let uppercase_style = Style::new()
    ///     .transform(|text| text.to_uppercase());
    ///
    /// // Add prefix and suffix
    /// let wrapped_style = Style::new()
    ///     .transform(|text| format!("[{}]", text));
    ///
    /// // Remove whitespace and convert to lowercase
    /// let clean_style = Style::new()
    ///     .transform(|text| text.trim().to_lowercase());
    ///
    /// // Chain with other styling
    /// let fancy_style = Style::new()
    ///     .bold(true)
    ///     .foreground("blue")
    ///     .transform(|text| format!("✨ {} ✨", text));
    ///
    /// let result = fancy_style.render("Hello World");
    /// // Result will be bold, blue, and transformed: "✨ HELLO WORLD ✨"
    /// ```
    ///
    /// # Note
    ///
    /// The transformation is applied before any styling (colors, borders, etc.).
    /// Multiple transformations can be layered by calling `transform` multiple times,
    /// though only the last transformation will be used.
    pub fn transform<F>(mut self, f: F) -> Self
    where
        F: Fn(String) -> String + Send + Sync + 'static,
    {
        self.transform = Some(Arc::new(f));
        self.set_prop(TRANSFORM_KEY);
        self
    }

    /// Sets a custom renderer for this style.
    ///
    /// This allows you to override the default renderer with a custom one that may have
    /// different color profiles, output capabilities, or rendering behavior. This is
    /// useful when you need to render styles for different terminal environments or
    /// when building styles for specific output contexts.
    ///
    /// # Arguments
    ///
    /// * `r` - A `Renderer` instance configured for your specific output context.
    ///   This could be a renderer with different color profiles, output streams,
    ///   or terminal capabilities.
    ///
    /// # Returns
    ///
    /// Returns the modified `Style` instance for method chaining.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use lipgloss::Style;
    /// use lipgloss::renderer::{Renderer, ColorProfileKind};
    ///
    /// // Create a renderer for 256-color output
    /// let mut color_renderer = Renderer::new();
    /// color_renderer.set_color_profile(ColorProfileKind::ANSI256);
    ///
    /// let style = Style::new()
    ///     .foreground("red")
    ///     .renderer(color_renderer.clone());
    ///
    /// // This style will ignore colors when rendering
    /// let result = style.render("Hello World");
    ///
    /// // Reuse the renderer on another style
    /// let colorful_style = Style::new()
    ///     .background("#2d7dc8")
    ///     .renderer(color_renderer);
    /// ```
    ///
    /// # Note
    ///
    /// If no custom renderer is set, the style will use the default global renderer.
    /// Setting a custom renderer only affects this specific style instance and doesn't
    /// change the global rendering behavior.
    pub fn renderer(mut self, r: Renderer) -> Self {
        self.r = Some(r);
        self
    }
}
