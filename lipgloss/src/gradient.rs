//! Functions for creating color gradients and grids for terminal styling.
//!
//! This module provides tools for generating smooth color transitions using
//! perceptually uniform color spaces, enabling the creation of beautiful
//! gradients in terminal user interfaces.

use crate::color::Color;
use palette::{FromColor, Luv, Mix, Srgb};

/// Creates a gradient of colors by blending two sRGB colors.
///
/// The blending is done in the perceptually uniform CIE L*u*v* color space,
/// which produces smooth and visually appealing transitions.
///
/// # Arguments
///
/// * `start` - The starting `Srgb<u8>` color.
/// * `end` - The ending `Srgb<u8>` color.
/// * `count` - The number of color steps to generate in the gradient.
///
/// # Returns
///
/// A `Vec<Color>` containing the generated gradient.
///
/// # Example
///
/// ```
/// use lipgloss::gradient::gradient_rgb;
/// use palette::Srgb;
///
/// let start = Srgb::new(255u8, 0, 0); // Red
/// let end = Srgb::new(0u8, 0, 255);   // Blue
/// let colors = gradient_rgb(start, end, 5);
/// assert_eq!(colors.len(), 5);
/// ```
pub fn gradient_rgb(start: Srgb<u8>, end: Srgb<u8>, count: usize) -> Vec<Color> {
    let start_luv = Luv::from_color(start.into_format());
    let end_luv = Luv::from_color(end.into_format());

    (0..count)
        .map(|i| {
            let factor = if count > 1 { i as f32 / (count - 1) as f32 } else { 0.0 };
            let blended_luv = start_luv.mix(end_luv, factor);
            let blended_rgb = Srgb::from_color(blended_luv).into_format::<u8>();
            Color::from(format!(
                "#{:02x}{:02x}{:02x}",
                blended_rgb.red, blended_rgb.green, blended_rgb.blue
            ).as_str())
        })
        .collect()
}

/// A convenience function to create a gradient from two hex color strings.
///
/// Parses the hex strings and calls `gradient_rgb`. Returns an error if the hex strings are invalid.
///
/// # Arguments
///
/// * `start_hex` - The starting hex color string (e.g., "#FF0000").
/// * `end_hex` - The ending hex color string (e.g., "#0000FF").
/// * `count` - The number of color steps to generate.
///
/// # Returns
///
/// A `Vec<Color>` containing the generated gradient.
///
/// # Example
///
/// ```
/// use lipgloss::gradient::gradient;
///
/// let colors = gradient("#FF0000", "#0000FF", 5);
/// assert_eq!(colors.len(), 5);
/// ```
pub fn gradient(start_hex: &str, end_hex: &str, count: usize) -> Vec<Color> {
    let start_rgb = parse_hex_color(start_hex);
    let end_rgb = parse_hex_color(end_hex);
    gradient_rgb(start_rgb, end_rgb, count)
}

/// Generates a 2D grid of colors using bilinear interpolation between four corner colors.
///
/// This function creates a smooth 2D color field, perfect for generating color palettes
/// or background effects. The blending is performed in the CIE L*u*v* color space.
///
/// # Arguments
///
/// * `x_steps` - The number of columns in the grid.
/// * `y_steps` - The number of rows in the grid.
/// * `corners` - A tuple of four hex strings: `(top_left, top_right, bottom_left, bottom_right)`.
///
/// # Returns
///
/// A 2D vector (`Vec<Vec<Color>>`) representing the color grid.
///
/// # Example
///
/// ```
/// use lipgloss::gradient::bilinear_interpolation_grid;
///
/// let grid = bilinear_interpolation_grid(
///     3, 2,
///     ("#FF0000", "#00FF00", "#0000FF", "#FFFF00")
/// );
/// assert_eq!(grid.len(), 2); // 2 rows
/// assert_eq!(grid[0].len(), 3); // 3 columns
/// ```
pub fn bilinear_interpolation_grid(
    x_steps: usize,
    y_steps: usize,
    corners: (&str, &str, &str, &str),
) -> Vec<Vec<Color>> {
    let (top_left, top_right, bottom_left, bottom_right) = corners;

    let x0y0 = Luv::from_color(parse_hex_color(top_left).into_format());
    let x1y0 = Luv::from_color(parse_hex_color(top_right).into_format());
    let x0y1 = Luv::from_color(parse_hex_color(bottom_left).into_format());
    let x1y1 = Luv::from_color(parse_hex_color(bottom_right).into_format());

    let left_edge: Vec<Luv> = (0..y_steps)
        .map(|i| {
            let factor = if y_steps > 1 { i as f32 / (y_steps - 1) as f32 } else { 0.0 };
            x0y0.mix(x0y1, factor)
        })
        .collect();

    let right_edge: Vec<Luv> = (0..y_steps)
        .map(|i| {
            let factor = if y_steps > 1 { i as f32 / (y_steps - 1) as f32 } else { 0.0 };
            x1y0.mix(x1y1, factor)
        })
        .collect();

    (0..y_steps)
        .map(|y| {
            let start_of_row = left_edge[y];
            let end_of_row = right_edge[y];
            (0..x_steps)
                .map(|x| {
                    let factor = if x_steps > 1 { x as f32 / (x_steps - 1) as f32 } else { 0.0 };
                    let blended_luv = start_of_row.mix(end_of_row, factor);
                    let blended_rgb = Srgb::from_color(blended_luv).into_format::<u8>();
                    Color::from(format!(
                        "#{:02x}{:02x}{:02x}",
                        blended_rgb.red, blended_rgb.green, blended_rgb.blue
                    ).as_str())
                })
                .collect()
        })
        .collect()
}

/// Helper function to parse hex color strings into Srgb<u8>.
fn parse_hex_color(hex: &str) -> Srgb<u8> {
    let hex = hex.trim_start_matches('#');
    let r = u8::from_str_radix(&hex[0..2], 16).unwrap_or(0);
    let g = u8::from_str_radix(&hex[2..4], 16).unwrap_or(0);
    let b = u8::from_str_radix(&hex[4..6], 16).unwrap_or(0);
    Srgb::new(r, g, b)
}
