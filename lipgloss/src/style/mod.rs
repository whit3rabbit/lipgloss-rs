//! Terminal text styling with comprehensive formatting capabilities.
//!
//! This module provides the [`Style`] struct and all related functionality for creating
//! beautiful terminal user interfaces. The style system supports a wide range of visual
//! properties including colors, text attributes, borders, spacing, and layout.
//!
//! # Core Concepts
//!
//! - **Builder Pattern**: All style methods return `Self` for easy chaining
//! - **Property Tracking**: Efficiently tracks which properties have been set
//! - **Rendering**: Converts styled content to ANSI escape sequences
//! - **Inheritance**: Styles can inherit from other styles with proper precedence
//!
//! # Quick Start
//!
//! ```rust,no_run
//! use lipgloss::Style;
//!
//! // Create a basic styled text
//! let hello = Style::new()
//!     .bold(true)
//!     .foreground("bright-blue")
//!     .render("Hello, World!");
//!
//! println!("{}", hello);
//! ```
//!
//! # Features
//!
//! ## Text Attributes
//! - Bold, italic, underline, strikethrough
//! - Blink, faint, reverse video
//!
//! ## Colors
//! - Full color profile support (truecolor, 256-color, 16-color)
//! - Adaptive colors for light/dark backgrounds
//! - Named colors and hex values
//!
//! ## Layout & Spacing
//! - Width and height constraints
//! - Padding and margins
//! - Text alignment (horizontal and vertical)
//!
//! ## Borders
//! - Multiple border styles (normal, rounded, thick, double, etc.)
//! - Per-side border control
//! - Border colors (foreground and background)
//!
//! ## Advanced Features
//! - Text transformations
//! - Custom renderers
//!
//! Style module containing all style-related functionality

// Property definitions
pub(crate) mod properties;

// Core Style struct and basic implementation
mod core;
pub use core::Style;

// Specialized functionality modules
mod attributes;
mod borders;
mod colors;
mod equality;
mod getters;
mod layout;
mod render;
mod render_utils;
mod rules;
mod sizing;
mod transform;
mod unset;
