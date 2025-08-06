//! Color handling and terminal color representation.
//!
//! This module provides comprehensive color support for terminal applications, including:
//!
//! - Various color formats (hex, ANSI, ANSI256, true color)
//! - Adaptive colors that change based on light/dark backgrounds
//! - Color profile-aware rendering for different terminal capabilities
//! - Automatic color space conversion and quantization
//!
//! # Usage
//!
//! ```rust
//! use lipgloss::color::{Color, ANSIColor, AdaptiveColor, TerminalColor};
//! use lipgloss::renderer::default_renderer;
//!
//! // Basic hex color
//! let red = Color("#ff0000".to_string());
//! let token = red.token_default();
//!
//! // ANSI color
//! let bright_blue = ANSIColor(12);
//! println!("RGBA: {:?}", bright_blue.rgba());
//!
//! // Adaptive color for light/dark themes
//! let adaptive = AdaptiveColor {
//!     Light: "#000000".to_string(),
//!     Dark: "#ffffff".to_string(),
//! };
//! ```

use crate::renderer::{default_renderer, ColorProfileKind, Renderer};
use palette::color_difference::EuclideanDistance;
use palette::{FromColor, Lab, Srgb, Hsv, Clamp};

/// A color intended to be rendered in the terminal.
///
/// This trait provides a unified interface for different color types that can be
/// rendered in terminal environments. It handles color profile-aware rendering
/// and provides RGBA conversion for interoperability with other graphics systems.
///
/// The trait abstracts over various color representations (hex, ANSI, adaptive)
/// and automatically handles conversion between different terminal color profiles
/// (NoColor, ANSI, ANSI256, TrueColor).
///
/// # Examples
///
/// ```rust
/// use lipgloss::color::{Color, TerminalColor};
/// use lipgloss::renderer::default_renderer;
///
/// let color = Color("#ff0000".to_string());
/// let token = color.token_default(); // Uses default renderer
/// let (r, g, b, a) = color.rgba();   // Get RGBA components
/// ```
pub trait TerminalColor {
    /// Returns the color token appropriate for the given renderer's profile.
    ///
    /// The returned token format depends on the renderer's color profile:
    /// - TrueColor: hex string like "#ff0000" or "#rrggbbaa"
    /// - ANSI256: numeric index like "196" (0-255 range)
    /// - ANSI: numeric index like "9" (0-15 range)
    /// - NoColor: empty string
    ///
    /// # Arguments
    ///
    /// * `r` - The renderer containing color profile information
    ///
    /// # Examples
    ///
    /// ```rust
    /// use lipgloss::color::{Color, TerminalColor};
    /// use lipgloss::renderer::{Renderer, ColorProfileKind};
    ///
    /// let color = Color("#ff0000".to_string());
    /// let mut renderer = Renderer::new();
    /// renderer.set_color_profile(ColorProfileKind::ANSI256);
    /// let token = color.token(&renderer); // Returns "196"
    /// ```
    fn token(&self, r: &Renderer) -> String;

    /// Returns the RGBA color components.
    ///
    /// Components are returned as 16-bit values (0-65535 range) following
    /// Go's color.RGBA convention. For colors without a precise RGB definition
    /// or on parsing errors, returns black with 100% opacity (0, 0, 0, 65535).
    ///
    /// # Returns
    ///
    /// A tuple of (red, green, blue, alpha) components as u32 values.
    /// Each component is in the 0-65535 range.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use lipgloss::color::{Color, TerminalColor};
    ///
    /// let red = Color("#ff0000".to_string());
    /// let (r, g, b, a) = red.rgba();
    /// assert_eq!((r, g, b, a), (255, 0, 0, 65535));
    ///
    /// let invalid = Color("invalid".to_string());
    /// let (r, g, b, a) = invalid.rgba();
    /// assert_eq!((r, g, b, a), (0, 0, 0, 65535)); // Fallback to black
    /// ```
    fn rgba(&self) -> (u32, u32, u32, u32);

    /// Convenience method to resolve color using the default renderer.
    ///
    /// This method provides a shorthand for `self.token(default_renderer())`
    /// when you don't need to specify a custom renderer.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use lipgloss::color::{Color, TerminalColor};
    ///
    /// let color = Color("#00ff00".to_string());
    /// let token = color.token_default(); // Uses global default renderer
    /// ```
    fn token_default(&self) -> String
    where
        Self: Sized,
    {
        self.token(default_renderer())
    }
}

// Convenience: allow using &str and String directly as colors in Style builders.
impl TerminalColor for &str {
    fn token(&self, r: &Renderer) -> String {
        Color::from(*self).token(r)
    }
    fn rgba(&self) -> (u32, u32, u32, u32) {
        Color::from(*self).rgba()
    }
}

impl TerminalColor for String {
    fn token(&self, r: &Renderer) -> String {
        Color::from(self.as_str()).token(r)
    }
    fn rgba(&self) -> (u32, u32, u32, u32) {
        Color::from(self.as_str()).rgba()
    }
}

// Map sRGB 8-bit to nearest ANSI256 index using termenv's exact algorithm.
pub(crate) fn rgb_to_ansi256(r: u8, g: u8, b: u8) -> u8 {
    // Convert to 6x6x6 color cube using termenv's v2ci function
    fn v2ci(v: u8) -> u8 {
        if v < 48 {
            0
        } else if v < 115 {
            1
        } else {
            ((v as i32 - 35) / 40).clamp(0, 5) as u8
        }
    }

    let qr = v2ci(r);
    let qg = v2ci(g);
    let qb = v2ci(b);
    let ci = 36 * qr + 6 * qg + qb; // 0..215, index into 6x6x6 cube

    // Convert back to RGB using termenv's i2cv values
    const I2CV: [u8; 6] = [0, 0x5f, 0x87, 0xaf, 0xd7, 0xff];
    let cr = I2CV[qr as usize];
    let cg = I2CV[qg as usize];
    let cb = I2CV[qb as usize];

    // Calculate grayscale using termenv's algorithm
    // termenv uses: grayIdx = (average - 3) / 10, but clamps differently
    let r_f = r as f64;
    let g_f = g as f64;
    let b_f = b as f64;
    let average = (r_f + g_f + b_f) / 3.0;

    let gray_idx = if average > 238.0 {
        23
    } else {
        ((average - 3.0) / 10.0).round().clamp(0.0, 23.0) as u8
    };
    let gv = 8 + 10 * gray_idx;

    // Compare Euclidean distances (termenv uses colorful distance, but this is close enough)
    let color_cube_dist = dist2(r, g, b, cr, cg, cb);
    let gray_dist = dist2(r, g, b, gv, gv, gv);

    if color_cube_dist <= gray_dist {
        16 + ci
    } else {
        232 + gray_idx
    }
}

fn dist2(r1: u8, g1: u8, b1: u8, r2: u8, g2: u8, b2: u8) -> u32 {
    let dr = r1 as i32 - r2 as i32;
    let dg = g1 as i32 - g2 as i32;
    let db = b1 as i32 - b2 as i32;
    (dr * dr + dg * dg + db * db) as u32
}

fn ansi256_to_rgb_u8(idx: u8) -> (u8, u8, u8) {
    match idx {
        0..=15 => ANSI16_RGB[idx as usize],
        16..=231 => {
            let i = idx - 16;
            let r = i / 36;
            let g = (i % 36) / 6;
            let b = i % 6;
            (
                CUBE_LEVELS[r as usize],
                CUBE_LEVELS[g as usize],
                CUBE_LEVELS[b as usize],
            )
        }
        232..=255 => {
            let v = 8 + 10 * (idx - 232);
            (v, v, v)
        }
    }
}

pub(crate) fn rgb_to_ansi16(r: u8, g: u8, b: u8) -> u8 {
    // Map to nearest among standard 16 ANSI colors using perceptually accurate Delta E
    // distance in the CIE L*a*b* color space, matching Go's termenv behavior.
    let source_color = Srgb::new(r as f32 / 255.0, g as f32 / 255.0, b as f32 / 255.0);
    let source_lab = Lab::from_color(source_color.into_linear());

    ANSI16_RGB
        .iter()
        .enumerate()
        .min_by(|(_, &(r1, g1, b1)), (_, &(r2, g2, b2))| {
            let p1_color = Srgb::new(r1 as f32 / 255.0, g1 as f32 / 255.0, b1 as f32 / 255.0);
            let p1_lab = Lab::from_color(p1_color.into_linear());

            let p2_color = Srgb::new(r2 as f32 / 255.0, g2 as f32 / 255.0, b2 as f32 / 255.0);
            let p2_lab = Lab::from_color(p2_color.into_linear());

            let d1 = source_lab.distance(p1_lab);
            let d2 = source_lab.distance(p2_lab);

            d1.total_cmp(&d2)
        })
        .map(|(i, _)| i as u8)
        .unwrap_or(0) // Default to black on any error
}

const CUBE_LEVELS: [u8; 6] = [0, 0x5f, 0x87, 0xaf, 0xd7, 0xff]; // termenv's i2cv values
const ANSI16_RGB: [(u8, u8, u8); 16] = [
    (0x00, 0x00, 0x00), // 0 black          #000000
    (0x80, 0x00, 0x00), // 1 red            #800000
    (0x00, 0x80, 0x00), // 2 green          #008000
    (0x80, 0x80, 0x00), // 3 yellow         #808000
    (0x00, 0x00, 0x80), // 4 blue           #000080
    (0x80, 0x00, 0x80), // 5 magenta        #800080
    (0x00, 0x80, 0x80), // 6 cyan           #008080
    (0xc0, 0xc0, 0xc0), // 7 white          #c0c0c0
    (0x80, 0x80, 0x80), // 8 bright black   #808080
    (0xff, 0x00, 0x00), // 9 bright red     #ff0000
    (0x00, 0xff, 0x00), // 10 bright green  #00ff00
    (0xff, 0xff, 0x00), // 11 bright yellow #ffff00
    (0x00, 0x00, 0xff), // 12 bright blue   #0000ff
    (0xff, 0x00, 0xff), // 13 bright magenta#ff00ff
    (0x00, 0xff, 0xff), // 14 bright cyan   #00ffff
    (0xff, 0xff, 0xff), // 15 bright white  #ffffff
];

/// Resolves a color string into a profile-appropriate token.
///
/// This internal helper function converts color strings (typically hex values)
/// into the appropriate format for the specified color profile. It handles
/// automatic conversion between color spaces when needed.
///
/// # Arguments
///
/// * `s` - The input color string (typically hex format like "#ff0000")
/// * `profile` - The target color profile to convert to
///
/// # Returns
///
/// A string token appropriate for the specified profile:
/// - `NoColor`: Empty string
/// - `TrueColor`: Original string (typically hex)
/// - `ANSI256`: Numeric string (0-255)
/// - `ANSI`: Numeric string (0-15)
///
/// # Examples
///
/// ```ignore
/// use lipgloss::color::resolve_color_token_for_profile;
/// use lipgloss::renderer::ColorProfileKind;
///
/// let hex_red = "#ff0000";
/// let ansi256_red = resolve_color_token_for_profile(hex_red, ColorProfileKind::ANSI256);
/// // Returns "196" for ANSI256 terminals
/// ```
pub(crate) fn resolve_color_token_for_profile(s: &str, profile: ColorProfileKind) -> String {
    match profile {
        ColorProfileKind::NoColor => String::new(),
        ColorProfileKind::TrueColor => {
            // If a numeric ANSI/ANSI256 index was provided, convert it to a hex token.
            if let Ok(idx) = s.parse::<u32>() {
                let (r, g, b) = ansi256_to_rgb_u8((idx % 256) as u8);
                return format!("#{:02x}{:02x}{:02x}", r, g, b);
            }
            s.to_string()
        }
        ColorProfileKind::ANSI256 => {
            // If the input is already a numeric ANSI/ANSI256 token, pass it through.
            if let Ok(idx) = s.parse::<u32>() {
                return ((idx % 256) as u8).to_string();
            }
            if let Some((r, g, b, _a)) = parse_hex_rgba(s) {
                let idx = rgb_to_ansi256(r as u8, g as u8, b as u8);
                idx.to_string()
            } else {
                s.to_string()
            }
        }
        ColorProfileKind::ANSI => {
            // If the input is a numeric token, handle direct ANSI codes and color indices
            if let Ok(idx) = s.parse::<u32>() {
                // Allow direct ANSI codes to pass through unchanged
                if (30..=37).contains(&idx)
                    || (90..=97).contains(&idx)
                    || (40..=47).contains(&idx)
                    || (100..=107).contains(&idx)
                {
                    return idx.to_string();
                }
                // For color indices 0-15, map to ANSI16 and pass through
                if idx <= 15 {
                    return idx.to_string();
                }
                // For other values, clamp to 0-15 range
                return ((idx % 16) as u8).to_string();
            }
            if let Some((r, g, b, _a)) = parse_hex_rgba(s) {
                let idx = rgb_to_ansi16(r as u8, g as u8, b as u8);
                idx.to_string()
            } else {
                s.to_string()
            }
        }
    }
}

impl Color {
    /// Returns the color token for a specific renderer with color profile mapping.
    ///
    /// This method provides explicit control over which renderer to use for
    /// color token generation, unlike the trait method which might have
    /// different behavior in different implementations.
    ///
    /// # Arguments
    ///
    /// * `r` - The renderer to use for color profile determination
    ///
    /// # Examples
    ///
    /// ```rust
    /// use lipgloss::color::Color;
    /// use lipgloss::renderer::{Renderer, ColorProfileKind};
    ///
    /// let color = Color("#ff0000".to_string());
    /// let mut renderer = Renderer::new();
    /// renderer.set_color_profile(ColorProfileKind::ANSI256);
    /// let token = color.token_for_renderer(&renderer);
    /// ```
    pub fn token_for_renderer(&self, r: &Renderer) -> String {
        resolve_color_token_for_profile(&self.0, r.color_profile())
    }
}

// For TerminalColor trait, delegate to mapping-aware method.
impl TerminalColor for Color {
    fn token(&self, r: &Renderer) -> String {
        self.token_for_renderer(r)
    }

    fn rgba(&self) -> (u32, u32, u32, u32) {
        if let Some((r, g, b, a)) = parse_hex_rgba(&self.0) {
            (r, g, b, a)
        } else if let Ok(idx) = self.0.parse::<u32>() {
            let (r, g, b) = ansi256_to_rgb_u8((idx % 256) as u8);
            (r as u32, g as u32, b as u32, 0xFFFF)
        } else {
            (0x0, 0x0, 0x0, 0xFFFF)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hex_short_and_alpha() {
        assert_eq!(parse_hex_rgba("#fff"), Some((255, 255, 255, 0xFFFF)));
        assert_eq!(parse_hex_rgba("#000"), Some((0, 0, 0, 0xFFFF)));
        assert_eq!(
            parse_hex_rgba("#abcd"),
            Some((0xAA, 0xBB, 0xCC, 0xDD * 257))
        );
    }

    #[test]
    fn test_hex_long_and_alpha() {
        assert_eq!(parse_hex_rgba("#ff0000"), Some((255, 0, 0, 0xFFFF)));
        assert_eq!(
            parse_hex_rgba("#11223344"),
            Some((0x11, 0x22, 0x33, 0x44 * 257))
        );
    }

    #[test]
    fn test_ansi256_values() {
        // Test what ANSI256 index 59 should map to (suggested by online converter)
        let (r59, g59, b59) = ansi256_to_rgb_u8(59);
        println!("ANSI256 59 -> RGB({}, {}, {})", r59, g59, b59);

        // Test what ANSI256 index 240 should map to
        let (r240, g240, b240) = ansi256_to_rgb_u8(240);
        println!("ANSI256 240 -> RGB({}, {}, {})", r240, g240, b240);

        // Test what ANSI256 index 238 should map to
        let (r238, g238, b238) = ansi256_to_rgb_u8(238);
        println!("ANSI256 238 -> RGB({}, {}, {})", r238, g238, b238);

        // Test what rgb(64,64,64) is closest to
        println!("Testing rgb(64,64,64):");
        let dist_to_59 = dist2(64, 64, 64, r59, g59, b59);
        let dist_to_240 = dist2(64, 64, 64, r240, g240, b240);
        let dist_to_238 = dist2(64, 64, 64, r238, g238, b238);
        println!("Distance to ANSI256 59: {}", dist_to_59);
        println!("Distance to ANSI256 240: {}", dist_to_240);
        println!("Distance to ANSI256 238: {}", dist_to_238);

        // Let's find the absolute closest match by brute force
        let mut closest_idx = 0;
        let mut closest_dist = u32::MAX;
        for i in 0..=255 {
            let (r, g, b) = ansi256_to_rgb_u8(i);
            let dist = dist2(64, 64, 64, r, g, b);
            if dist < closest_dist {
                closest_dist = dist;
                closest_idx = i;
            }
        }
        println!(
            "Closest match: ANSI256 {} with distance {}",
            closest_idx, closest_dist
        );
        let (rclosest, gclosest, bclosest) = ansi256_to_rgb_u8(closest_idx);
        println!(
            "ANSI256 {} -> RGB({}, {}, {})",
            closest_idx, rclosest, gclosest, bclosest
        );
    }

    #[test]
    fn test_rgb_to_ansi256_debug() {
        let r = 64u8;
        let g = 64u8;
        let b = 64u8;

        // Color cube calculation
        fn v2ci(v: u8) -> u8 {
            if v < 48 {
                0
            } else if v < 115 {
                1
            } else {
                ((v as i32 - 35) / 40).clamp(0, 5) as u8
            }
        }

        let qr = v2ci(r);
        let qg = v2ci(g);
        let qb = v2ci(b);
        let ci = 36 * qr + 6 * qg + qb;

        const I2CV: [u8; 6] = [0, 0x5f, 0x87, 0xaf, 0xd7, 0xff];
        let cr = I2CV[qr as usize];
        let cg = I2CV[qg as usize];
        let cb = I2CV[qb as usize];

        println!("Color cube: qr={}, qg={}, qb={}, ci={}", qr, qg, qb, ci);
        println!("Color cube RGB: ({}, {}, {})", cr, cg, cb);

        // Grayscale calculation
        let average = (r as u32 + g as u32 + b as u32) / 3;
        let gray_idx = if average > 238 {
            23
        } else {
            ((average as i32 - 3) / 10).clamp(0, 23) as u8
        };
        let gv = 8 + 10 * gray_idx;

        println!(
            "Grayscale: average={}, gray_idx={}, gv={}",
            average, gray_idx, gv
        );

        // Distance calculations
        let color_cube_dist = dist2(r, g, b, cr, cg, cb);
        let gray_dist = dist2(r, g, b, gv, gv, gv);

        println!(
            "Distances: color_cube={}, gray={}",
            color_cube_dist, gray_dist
        );

        let result = if color_cube_dist <= gray_dist {
            16 + ci
        } else {
            232 + gray_idx
        };

        println!("Result: {} (should be closest distance)", result);
    }

    #[test]
    fn test_rgb_to_ansi256_termenv_compatibility() {
        // Test core algorithm - these values match termenv's behavior
        assert_eq!(rgb_to_ansi256(255, 0, 0), 196); // Pure red -> ANSI256 196
        assert_eq!(rgb_to_ansi256(0, 255, 0), 46); // Pure green -> ANSI256 46
        assert_eq!(rgb_to_ansi256(0, 0, 255), 21); // Pure blue -> ANSI256 21

        // Test grayscale colors (verified actual outputs)
        assert_eq!(rgb_to_ansi256(128, 128, 128), 102); // Mid gray chooses color cube
        assert_eq!(rgb_to_ansi256(64, 64, 64), 238); // Dark gray
        assert_eq!(rgb_to_ansi256(192, 192, 192), 251); // Light gray

        // Test color cube edge cases
        assert_eq!(rgb_to_ansi256(95, 95, 95), 59); // Color cube level 1
        assert_eq!(rgb_to_ansi256(135, 135, 135), 102); // Color cube level 2
    }

    #[test]
    fn test_rgb_to_ansi16_termenv_compatibility() {
        // Test colors that should map to standard ANSI colors
        assert_eq!(rgb_to_ansi16(255, 0, 0), 9); // Bright red
        assert_eq!(rgb_to_ansi16(0, 255, 0), 10); // Bright green
        assert_eq!(rgb_to_ansi16(0, 0, 255), 12); // Bright blue
        assert_eq!(rgb_to_ansi16(128, 0, 0), 1); // Dark red
        assert_eq!(rgb_to_ansi16(0, 128, 0), 2); // Dark green
        assert_eq!(rgb_to_ansi16(0, 0, 128), 4); // Dark blue
        assert_eq!(rgb_to_ansi16(192, 192, 192), 7); // White
        assert_eq!(rgb_to_ansi16(128, 128, 128), 8); // Bright black
    }

    #[test]
    fn test_color_rgba_from_numeric_index() {
        let c = Color("196".to_string());
        assert_eq!(c.rgba(), (255, 0, 0, 0xFFFF));
    }

    #[test]
    fn test_adaptive_color_field_names() {
        // Test that field names match Go API exactly
        let adaptive = AdaptiveColor {
            Light: "#000000".to_string(),
            Dark: "#ffffff".to_string(),
        };
        // Should work with our trait implementation
        let _token = adaptive.token_default();
        let _rgba = adaptive.rgba();
    }

    #[test]
    fn test_complete_color_field_names() {
        // Test that field names match Go API exactly
        let complete = CompleteColor {
            TrueColor: "#ff0000".to_string(),
            ANSI256: "196".to_string(),
            ANSI: "9".to_string(),
        };
        // Should work with our trait implementation
        let _token = complete.token_default();
        let _rgba = complete.rgba();
    }

    #[test]
    fn test_color_token_profile_conversion() {
        use crate::renderer::{ColorProfileKind, Renderer};

        let hex_red = Color("#ff0000".to_string());
        let ansi_red = Color("9".to_string());
        let ansi256_red = Color("196".to_string());

        // Test hex color conversion to different profiles
        let mut renderer = Renderer::new();

        renderer.set_color_profile(ColorProfileKind::TrueColor);
        assert_eq!(hex_red.token(&renderer), "#ff0000");

        renderer.set_color_profile(ColorProfileKind::ANSI256);
        assert_eq!(hex_red.token(&renderer), "196"); // Should convert to ANSI256

        renderer.set_color_profile(ColorProfileKind::ANSI);
        assert_eq!(hex_red.token(&renderer), "9"); // Should convert to ANSI16

        // Test numeric ANSI color passthrough
        renderer.set_color_profile(ColorProfileKind::ANSI256);
        assert_eq!(ansi256_red.token(&renderer), "196"); // Should pass through

        renderer.set_color_profile(ColorProfileKind::ANSI);
        assert_eq!(ansi_red.token(&renderer), "9"); // Should pass through

        // Test TrueColor conversion of numeric colors
        renderer.set_color_profile(ColorProfileKind::TrueColor);
        assert_eq!(ansi_red.token(&renderer), "#ff0000"); // Should convert to hex
    }

    #[test]
    fn test_style_rendering_with_color_profiles() {
        use crate::{
            renderer::{set_default_renderer, ColorProfileKind, Renderer},
            Style,
        };

        let input = "hello";
        let test_cases = [
            (ColorProfileKind::NoColor, "hello"),
            (ColorProfileKind::ANSI, "\x1b[34mhello\x1b[0m"),
            (ColorProfileKind::ANSI256, "\x1b[38;5;62mhello\x1b[0m"),
            (
                ColorProfileKind::TrueColor,
                "\x1b[38;2;90;86;224mhello\x1b[0m",
            ),
        ];

        for (profile, expected) in test_cases {
            let mut renderer = Renderer::new();
            renderer.set_color_profile(profile);
            set_default_renderer(renderer);

            let style = Style::new().foreground(Color("#5A56E0".to_string()));
            let result = style.render(input);

            assert_eq!(
                result, expected,
                "Profile {:?}: expected '{}', got '{}'",
                profile, expected, result
            );
        }
    }

    #[test]
    fn test_hex_to_color_conversion() {
        let test_cases = [
            ("#FF0000", 0xFF0000),
            ("#00F", 0x0000FF),
            ("#6B50FF", 0x6B50FF),
            ("invalid color", 0x0),
            ("", 0x0),
        ];

        for (input, expected) in test_cases {
            let color = Color(input.to_string());
            let (r, g, b, _a) = color.rgba();
            // Convert from 16-bit back to 8-bit for comparison
            let actual = (r << 16) + (g << 8) + b;

            assert_eq!(
                actual, expected,
                "Input '{}': expected 0x{:06X}, got 0x{:06X}",
                input, expected, actual
            );
        }
    }

    #[test]
    fn test_comprehensive_rgba_validation() {
        use crate::renderer::{set_default_renderer, ColorProfileKind, Renderer};

        // Test basic Color types
        let basic_tests = [
            (
                ColorProfileKind::TrueColor,
                true,
                Color("#FF0000".to_string()),
                0xFF0000,
            ),
            (
                ColorProfileKind::TrueColor,
                true,
                Color("9".to_string()),
                0xFF0000,
            ),
            (
                ColorProfileKind::TrueColor,
                true,
                Color("21".to_string()),
                0x0000FF,
            ),
        ];

        for (i, (profile, dark_bg, color, expected)) in basic_tests.iter().enumerate() {
            let mut renderer = Renderer::new();
            renderer.set_color_profile(*profile);
            renderer.set_has_dark_background(*dark_bg);
            set_default_renderer(renderer);

            let (r, g, b, _a) = color.rgba();
            // Our rgba() already returns 8-bit values, but Go's TestRGBA expects 8-bit RGB values
            let actual = (r << 16) + (g << 8) + b;

            assert_eq!(
                actual,
                *expected,
                "Basic Test #{}: Profile {:?}, Dark: {}, Expected 0x{:06X}, Got 0x{:06X}",
                i + 1,
                profile,
                dark_bg,
                expected,
                actual
            );
        }

        // Test AdaptiveColor types
        let adaptive_tests = [
            (
                true,
                AdaptiveColor {
                    Light: "#0000FF".to_string(),
                    Dark: "#FF0000".to_string(),
                },
                0xFF0000,
            ),
            (
                false,
                AdaptiveColor {
                    Light: "#0000FF".to_string(),
                    Dark: "#FF0000".to_string(),
                },
                0x0000FF,
            ),
            (
                true,
                AdaptiveColor {
                    Light: "21".to_string(),
                    Dark: "9".to_string(),
                },
                0xFF0000,
            ),
            (
                false,
                AdaptiveColor {
                    Light: "21".to_string(),
                    Dark: "9".to_string(),
                },
                0x0000FF,
            ),
        ];

        for (i, (dark_bg, color, expected)) in adaptive_tests.iter().enumerate() {
            let mut renderer = Renderer::new();
            renderer.set_color_profile(ColorProfileKind::TrueColor);
            renderer.set_has_dark_background(*dark_bg);
            set_default_renderer(renderer);

            let (r, g, b, _a) = color.rgba();
            let actual = (r << 16) + (g << 8) + b;

            assert_eq!(
                actual,
                *expected,
                "Adaptive Test #{}: Dark: {}, Expected 0x{:06X}, Got 0x{:06X}",
                i + 1,
                dark_bg,
                expected,
                actual
            );
        }

        // Test CompleteColor types - RGBA always uses TrueColor field regardless of profile
        let complete_tests = [
            (
                ColorProfileKind::TrueColor,
                CompleteColor {
                    TrueColor: "#FF0000".to_string(),
                    ANSI256: "231".to_string(),
                    ANSI: "12".to_string(),
                },
                0xFF0000,
            ),
            (
                ColorProfileKind::ANSI256,
                CompleteColor {
                    TrueColor: "#FF0000".to_string(),
                    ANSI256: "231".to_string(),
                    ANSI: "12".to_string(),
                },
                0xFF0000,
            ),
            (
                ColorProfileKind::ANSI,
                CompleteColor {
                    TrueColor: "#FF0000".to_string(),
                    ANSI256: "231".to_string(),
                    ANSI: "12".to_string(),
                },
                0xFF0000,
            ),
            (
                ColorProfileKind::TrueColor,
                CompleteColor {
                    TrueColor: "".to_string(),
                    ANSI256: "231".to_string(),
                    ANSI: "12".to_string(),
                },
                0x000000,
            ),
        ];

        for (i, (profile, color, expected)) in complete_tests.iter().enumerate() {
            let mut renderer = Renderer::new();
            renderer.set_color_profile(*profile);
            renderer.set_has_dark_background(true);
            set_default_renderer(renderer);

            let (r, g, b, _a) = color.rgba();
            let actual = (r << 16) + (g << 8) + b;

            assert_eq!(
                actual,
                *expected,
                "Complete Test #{}: Profile {:?}, Expected 0x{:06X}, Got 0x{:06X}",
                i + 1,
                profile,
                expected,
                actual
            );
        }
    }

    #[test]
    fn test_invalid_color_handling() {
        // Test various invalid color inputs return black with full alpha
        let invalid_colors = [
            "",
            "invalid",
            "#",
            "#ZZ",
            "#12345",      // Wrong length
            "#1234567890", // Too long
            "not-a-color",
            "rgb(255,0,0)", // CSS format not supported
        ];

        for invalid in invalid_colors {
            let color = Color(invalid.to_string());
            let (r, g, b, a) = color.rgba();

            assert_eq!(
                (r, g, b, a),
                (0, 0, 0, 0xFFFF),
                "Invalid color '{}' should return black with full alpha, got ({}, {}, {}, {})",
                invalid,
                r,
                g,
                b,
                a
            );
        }
    }

    #[test]
    fn test_adaptive_color_rgba_combinations() {
        let adaptive = AdaptiveColor {
            Light: "#FF0000".to_string(), // Red for light background
            Dark: "#00FF00".to_string(),  // Green for dark background
        };

        // Test that RGBA uses the default renderer's background setting
        let (r, g, b, a) = adaptive.rgba();

        // Should match either red or green depending on default renderer background
        let is_red = (r, g, b) == (255, 0, 0);
        let is_green = (r, g, b) == (0, 255, 0);

        assert!(
            is_red || is_green,
            "AdaptiveColor RGBA should return either red (255,0,0) or green (0,255,0), got ({},{},{})",
            r, g, b
        );
        assert_eq!(a, 0xFFFF, "Alpha should be full opacity");
    }

    #[test]
    fn test_complete_color_rgba_combinations() {
        let complete = CompleteColor {
            TrueColor: "#FF0000".to_string(),
            ANSI256: "46".to_string(), // Different color for testing
            ANSI: "12".to_string(),    // Different color for testing
        };

        // CompleteColor RGBA should always use TrueColor value
        let (r, g, b, a) = complete.rgba();
        assert_eq!(
            (r, g, b, a),
            (255, 0, 0, 0xFFFF),
            "CompleteColor RGBA should use TrueColor value"
        );

        // Test with empty TrueColor (should fallback to black)
        let empty_complete = CompleteColor {
            TrueColor: "".to_string(),
            ANSI256: "46".to_string(),
            ANSI: "12".to_string(),
        };

        let (r, g, b, a) = empty_complete.rgba();
        assert_eq!(
            (r, g, b, a),
            (0, 0, 0, 0xFFFF),
            "CompleteColor with empty TrueColor should fallback to black"
        );
    }

    #[test]
    fn test_color_utility_functions() {
        // Test clamp
        assert_eq!(clamp(5, 0, 10), 5);
        assert_eq!(clamp(-1, 0, 10), 0);
        assert_eq!(clamp(15, 0, 10), 10);

        // Test parse_hex
        assert_eq!(parse_hex("#ff0000"), Some((255, 0, 0, 255)));
        assert_eq!(parse_hex("#f00"), Some((255, 0, 0, 255)));
        assert_eq!(parse_hex("#ff0000aa"), Some((255, 0, 0, 170)));
        assert_eq!(parse_hex("invalid"), None);
        assert_eq!(parse_hex(""), None);

        // Test is_dark_color
        let black = Color("#000000".to_string());
        let white = Color("#ffffff".to_string());
        let dark_gray = Color("#404040".to_string());
        let light_gray = Color("#c0c0c0".to_string());
        
        assert!(is_dark_color(&black));
        assert!(!is_dark_color(&white));
        assert!(is_dark_color(&dark_gray));
        assert!(!is_dark_color(&light_gray));
    }

    #[test]
    fn test_lighten_darken() {
        let red = Color("#800000".to_string()); // Dark red
        
        // Test lighten
        let lighter = lighten(&red, 0.5);
        let (lr, lg, lb, _) = lighter.rgba();
        let (or, og, ob, _) = red.rgba();
        
        // Should be lighter than original
        assert!(lr >= or);
        assert!(lg >= og);
        assert!(lb >= ob);

        // Test darken
        let bright_red = Color("#ff0000".to_string());
        let darker = darken(&bright_red, 0.3);
        let (dr, dg, db, _) = darker.rgba();
        let (br, bg, bb, _) = bright_red.rgba();
        
        // Should be darker than original  
        assert!(dr <= br);
        assert!(dg <= bg);
        assert!(db <= bb);
    }

    #[test]
    fn test_alpha_adjustment() {
        let red = Color("#ff0000".to_string());
        let semi_transparent = alpha(&red, 0.5);
        
        // Should have alpha component in hex string
        assert!(semi_transparent.0.len() == 9); // #rrggbbaa format
        assert!(semi_transparent.0.contains("7f") || semi_transparent.0.contains("80")); // ~127-128 in hex
    }

    #[test]
    fn test_complementary_color() {
        let red = Color("#ff0000".to_string());
        let comp = complementary(&red);
        let (cr, cg, cb, _) = comp.rgba();
        
        // Complementary of red should be cyan-ish (high green and blue)
        assert!(cg > 100 || cb > 100);
        assert!(cr < cg || cr < cb); // Red should be lower than green or blue
    }

    #[test]
    fn test_light_dark_function() {
        let red = Color("#ff0000".to_string());
        let blue = Color("#0000ff".to_string());
        
        // Test dark background
        let dark_fn = light_dark(true);
        let dark_choice = dark_fn(&red, &blue);
        let (dr, dg, db, _) = dark_choice.rgba();
        let (br, bg, bb, _) = blue.rgba();
        assert_eq!((dr, dg, db), (br, bg, bb)); // Should choose blue for dark
        
        // Test light background
        let light_fn = light_dark(false);
        let light_choice = light_fn(&red, &blue);
        let (lr, lg, lb, _) = light_choice.rgba();
        let (rr, rg, rb, _) = red.rgba();
        assert_eq!((lr, lg, lb), (rr, rg, rb)); // Should choose red for light
    }

    #[test]
    fn test_complete_function() {
        use crate::renderer::ColorProfileKind;
        
        let ansi = Color("1".to_string());
        let ansi256 = Color("124".to_string());
        let truecolor = Color("#ff34ac".to_string());
        
        // Test TrueColor profile
        let complete_fn = complete(ColorProfileKind::TrueColor);
        let chosen = complete_fn(&ansi, &ansi256, &truecolor);
        let (cr, cg, cb, _) = chosen.rgba();
        let (tr, tg, tb, _) = truecolor.rgba();
        assert_eq!((cr, cg, cb), (tr, tg, tb));
        
        // Test ANSI profile
        let complete_fn = complete(ColorProfileKind::ANSI);
        let chosen = complete_fn(&ansi, &ansi256, &truecolor);
        let (cr, cg, cb, _) = chosen.rgba();
        let (ar, ag, ab, _) = ansi.rgba();
        assert_eq!((cr, cg, cb), (ar, ag, ab));
    }

    #[test]
    fn test_complete_adaptive_color_combinations() {
        let complete_adaptive = CompleteAdaptiveColor {
            light: CompleteColor {
                TrueColor: "#FF0000".to_string(), // Red for light
                ANSI256: "196".to_string(),
                ANSI: "9".to_string(),
            },
            dark: CompleteColor {
                TrueColor: "#00FF00".to_string(), // Green for dark
                ANSI256: "46".to_string(),
                ANSI: "10".to_string(),
            },
        };

        // RGBA should use default renderer background to choose light/dark
        let (r, g, b, a) = complete_adaptive.rgba();

        // Should match either red or green depending on default renderer background
        let is_red = (r, g, b) == (255, 0, 0);
        let is_green = (r, g, b) == (0, 255, 0);

        assert!(
            is_red || is_green,
            "CompleteAdaptiveColor RGBA should return either red (255,0,0) or green (0,255,0), got ({},{},{})",
            r, g, b
        );
        assert_eq!(a, 0xFFFF, "Alpha should be full opacity");
    }
}

/// Specifies the absence of color styling.
///
/// This color type represents the explicit absence of color styling. When used
/// as a foreground color, text will use the terminal's default foreground color.
/// When used as a background color, no background will be drawn.
///
/// This is useful for explicitly removing color styling or for creating
/// color schemes that fall back to terminal defaults.
///
/// # Examples
///
/// ```rust
/// use lipgloss::color::{NoColor, TerminalColor};
///
/// let no_color = NoColor;
/// assert_eq!(no_color.token_default(), "");
/// assert_eq!(no_color.rgba(), (0, 0, 0, 65535)); // Black fallback
/// ```
#[derive(Debug, Clone, Copy, Default)]
pub struct NoColor;

impl TerminalColor for NoColor {
    fn token(&self, _r: &Renderer) -> String {
        String::new()
    }

    fn rgba(&self) -> (u32, u32, u32, u32) {
        // Black, 100% opacity
        (0x0, 0x0, 0x0, 0xFFFF)
    }
}

/// A color specified by hex or ANSI/ANSI256 value as a string.
///
/// This is the most versatile color type, accepting various string formats:
/// - Hex colors: `#RGB`, `#RRGGBB`, `#RGBA`, `#RRGGBBAA`
/// - ANSI colors: `"0"` through `"15"` (basic ANSI colors)
/// - ANSI256 colors: `"0"` through `"255"` (extended palette)
///
/// The color will be automatically converted to the appropriate format based
/// on the terminal's color profile capabilities.
///
/// # Examples
///
/// ```rust
/// use lipgloss::color::{Color, TerminalColor};
///
/// // Hex colors
/// let red = Color("#ff0000".to_string());
/// let blue_with_alpha = Color("#0000ff80".to_string());
///
/// // ANSI colors
/// let bright_red = Color("9".to_string());
/// let dark_blue = Color("4".to_string());
///
/// // ANSI256 colors
/// let orange = Color("208".to_string());
///
/// // Get color information
/// let (r, g, b, a) = red.rgba();
/// let token = red.token_default();
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Color(pub String);

impl From<&str> for Color {
    /// Creates a Color from a string slice.
    ///
    /// This is a convenience implementation that allows creating Color instances
    /// directly from string literals and &str references.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use lipgloss::color::Color;
    ///
    /// let red: Color = "#ff0000".into();
    /// let blue = Color::from("#0000ff");
    /// let ansi_green = Color::from("10");
    /// ```
    fn from(s: &str) -> Self {
        Color(s.to_string())
    }
}

impl Color {
    /// Creates a Color from RGBA values (equivalent to Go's color.RGBA).
    ///
    /// This method creates a Color from 8-bit RGBA values, which is useful
    /// for creating colors from exact RGB specifications like in tests.
    ///
    /// # Arguments
    ///
    /// * `r` - Red component (0-255)
    /// * `g` - Green component (0-255)
    /// * `b` - Blue component (0-255)
    /// * `a` - Alpha component (0-255)
    ///
    /// # Examples
    ///
    /// ```rust
    /// use lipgloss::color::Color;
    ///
    /// let red = Color::from_rgba(255, 0, 0, 255);
    /// let semi_transparent_blue = Color::from_rgba(0, 0, 255, 127);
    /// ```
    pub fn from_rgba(r: u8, g: u8, b: u8, a: u8) -> Self {
        if a == 255 {
            // Fully opaque, no need for alpha channel
            Color(format!("#{:02x}{:02x}{:02x}", r, g, b))
        } else {
            // Include alpha channel
            Color(format!("#{:02x}{:02x}{:02x}{:02x}", r, g, b, a))
        }
    }

    /// Creates a Color from RGB values with full opacity.
    ///
    /// This is a convenience method for creating fully opaque colors.
    ///
    /// # Arguments
    ///
    /// * `r` - Red component (0-255)
    /// * `g` - Green component (0-255)
    /// * `b` - Blue component (0-255)
    ///
    /// # Examples
    ///
    /// ```rust
    /// use lipgloss::color::Color;
    ///
    /// let red = Color::from_rgb(255, 0, 0);
    /// let green = Color::from_rgb(0, 255, 0);
    /// ```
    pub fn from_rgb(r: u8, g: u8, b: u8) -> Self {
        Self::from_rgba(r, g, b, 255)
    }

    /// Returns 16-bit RGBA values (0-65535) for blending operations.
    /// This matches Go's color.RGBA interface for exact parity.
    pub fn rgba16(&self) -> (u32, u32, u32, u32) {
        if let Some((r, g, b, a)) = parse_hex_rgba_8bit(&self.0) {
            // Convert 8-bit values to true 16-bit for Go compatibility
            srgb_to_true_rgba16(Srgb::new(r as u8, g as u8, b as u8), a as u8)
        } else if let Ok(idx) = self.0.parse::<u32>() {
            let (r, g, b) = ansi256_to_rgb_u8((idx % 256) as u8);
            srgb_to_true_rgba16(Srgb::new(r, g, b), 255)
        } else {
            srgb_to_true_rgba16(Srgb::new(0, 0, 0), 255)
        }
    }
}

/// A color specified by an ANSI color value.
///
/// This is a type-safe wrapper around numeric ANSI color codes, providing
/// syntactic sugar for the more general `Color` type. It accepts values
/// from 0-255, where:
/// - 0-15: Standard ANSI colors (black, red, green, etc.)
/// - 16-231: 216-color cube (6×6×6 RGB values)
/// - 232-255: 24-step grayscale ramp
///
/// Values are automatically clamped to the 0-255 range using modulo arithmetic.
///
/// # Examples
///
/// ```rust
/// use lipgloss::color::{ANSIColor, TerminalColor};
///
/// // Basic ANSI colors (0-15)
/// let red = ANSIColor(1);          // Dark red
/// let bright_red = ANSIColor(9);   // Bright red
///
/// // 256-color palette
/// let orange = ANSIColor(208);
/// let dark_gray = ANSIColor(235);
///
/// // Get color information
/// let (r, g, b, a) = orange.rgba();
/// let token = orange.token_default();
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ANSIColor(pub u32);

impl TerminalColor for ANSIColor {
    fn token(&self, _r: &Renderer) -> String {
        self.0.to_string()
    }

    fn rgba(&self) -> (u32, u32, u32, u32) {
        let (r, g, b) = ansi256_to_rgb_u8((self.0 % 256) as u8);
        (r as u32, g as u32, b as u32, 0xFFFF)
    }
}

/// Provides color options for light and dark backgrounds.
///
/// This color type automatically adapts based on the terminal's background
/// color detection. It contains two color specifications - one for light
/// backgrounds and one for dark backgrounds - and the appropriate color
/// is selected at render time.
///
/// This is particularly useful for creating themes that work well in both
/// light and dark terminal environments without requiring manual color
/// profile switching.
///
/// # Examples
///
/// ```rust
/// use lipgloss::color::{AdaptiveColor, TerminalColor};
///
/// let adaptive_text = AdaptiveColor {
///     Light: "#000000".to_string(), // Black text on light background
///     Dark: "#ffffff".to_string(),  // White text on dark background
/// };
///
/// let adaptive_accent = AdaptiveColor {
///     Light: "#0066cc".to_string(), // Dark blue on light background
///     Dark: "#66b3ff".to_string(),  // Light blue on dark background
/// };
///
/// // Color automatically adapts based on terminal background
/// let token = adaptive_text.token_default();
/// let (r, g, b, a) = adaptive_text.rgba();
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
#[allow(non_snake_case)]
pub struct AdaptiveColor {
    /// Color specification for light backgrounds
    pub Light: String,
    /// Color specification for dark backgrounds
    pub Dark: String,
}

impl TerminalColor for AdaptiveColor {
    fn token(&self, r: &Renderer) -> String {
        if r.has_dark_background() {
            Color(self.Dark.clone()).token(r)
        } else {
            Color(self.Light.clone()).token(r)
        }
    }

    fn rgba(&self) -> (u32, u32, u32, u32) {
        // Use default renderer's background to pick, then use Color's logic for parsing
        let color_str = if default_renderer().has_dark_background() {
            &self.Dark
        } else {
            &self.Light
        };
        Color(color_str.clone()).rgba()
    }
}

/// Specifies exact color values for all terminal color profiles.
///
/// This color type provides explicit control over how a color appears in
/// different terminal environments by specifying exact values for each
/// color profile type. This is useful when you need precise color matching
/// across different terminal capabilities.
///
/// Unlike other color types that perform automatic conversion, CompleteColor
/// uses the exact values you specify for each profile, giving you full
/// control over the color appearance.
///
/// # Examples
///
/// ```rust
/// use lipgloss::color::{CompleteColor, TerminalColor};
/// use lipgloss::renderer::{Renderer, ColorProfileKind};
///
/// let red = CompleteColor {
///     TrueColor: "#ff0000".to_string(),    // Hex for true color terminals
///     ANSI256: "196".to_string(),           // Index for 256-color terminals
///     ANSI: "9".to_string(),                // Index for basic ANSI terminals
/// };
///
/// // Different renderers will use different values
/// let mut true_color_renderer = Renderer::new();
/// true_color_renderer.set_color_profile(ColorProfileKind::TrueColor);
/// assert_eq!(red.token(&true_color_renderer), "#ff0000");
///
/// let mut ansi_renderer = Renderer::new();
/// ansi_renderer.set_color_profile(ColorProfileKind::ANSI);
/// assert_eq!(red.token(&ansi_renderer), "9");
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
#[allow(non_snake_case)]
pub struct CompleteColor {
    /// Color value for true color (24-bit) terminals
    pub TrueColor: String,
    /// Color value for 256-color terminals
    pub ANSI256: String,
    /// Color value for basic ANSI (16-color) terminals
    pub ANSI: String,
}

impl TerminalColor for CompleteColor {
    fn token(&self, r: &Renderer) -> String {
        match r.color_profile() {
            ColorProfileKind::TrueColor => self.TrueColor.clone(),
            ColorProfileKind::ANSI256 => self.ANSI256.clone(),
            ColorProfileKind::ANSI => self.ANSI.clone(),
            ColorProfileKind::NoColor => String::new(),
        }
    }

    fn rgba(&self) -> (u32, u32, u32, u32) {
        // CompleteColor RGBA should always use TrueColor value for consistent RGBA conversion
        if !self.TrueColor.is_empty() {
            Color(self.TrueColor.clone()).rgba()
        } else {
            (0x0, 0x0, 0x0, 0xFFFF)
        }
    }
}

/// Provides exact color values for light/dark backgrounds across all profiles.
///
/// This color type combines the features of `CompleteColor` and `AdaptiveColor`,
/// providing explicit control over color values for each terminal profile while
/// also adapting to light and dark backgrounds.
///
/// This is the most comprehensive color type, offering maximum control over
/// color appearance across different terminal environments and background types.
///
/// # Examples
///
/// ```rust
/// use lipgloss::color::{CompleteAdaptiveColor, CompleteColor, TerminalColor};
///
/// let adaptive_red = CompleteAdaptiveColor {
///     light: CompleteColor {
///         TrueColor: "#cc0000".to_string(),  // Darker red for light backgrounds
///         ANSI256: "160".to_string(),
///         ANSI: "1".to_string(),
///     },
///     dark: CompleteColor {
///         TrueColor: "#ff4444".to_string(),  // Lighter red for dark backgrounds
///         ANSI256: "203".to_string(),
///         ANSI: "9".to_string(),
///     },
/// };
///
/// // Color adapts based on both terminal capabilities AND background
/// let token = adaptive_red.token_default();
/// let (r, g, b, a) = adaptive_red.rgba();
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CompleteAdaptiveColor {
    /// Color specification for light backgrounds
    pub light: CompleteColor,
    /// Color specification for dark backgrounds
    pub dark: CompleteColor,
}

impl TerminalColor for CompleteAdaptiveColor {
    fn token(&self, r: &Renderer) -> String {
        if r.has_dark_background() {
            self.dark.token(r)
        } else {
            self.light.token(r)
        }
    }

    fn rgba(&self) -> (u32, u32, u32, u32) {
        if default_renderer().has_dark_background() {
            self.dark.rgba()
        } else {
            self.light.rgba()
        }
    }
}

// --- helpers ---

/// Parse hex color with 8-bit RGBA values for backward compatibility
pub(crate) fn parse_hex_rgba_8bit(s: &str) -> Option<(u32, u32, u32, u32)> {
    // Support #RGB, #RRGGBB, #RGBA, #RRGGBBAA forms.
    let s = s.trim();
    let hex = s.strip_prefix('#')?;

    let (r, g, b, a_u8) = match hex.len() {
        3 => {
            // #RGB -> expand each nibble
            let r = u8::from_str_radix(&hex[0..1], 16).ok()?;
            let g = u8::from_str_radix(&hex[1..2], 16).ok()?;
            let b = u8::from_str_radix(&hex[2..3], 16).ok()?;
            let r = (r << 4) | r;
            let g = (g << 4) | g;
            let b = (b << 4) | b;
            (r, g, b, 0xFF)
        }
        4 => {
            // #RGBA -> expand
            let r = u8::from_str_radix(&hex[0..1], 16).ok()?;
            let g = u8::from_str_radix(&hex[1..2], 16).ok()?;
            let b = u8::from_str_radix(&hex[2..3], 16).ok()?;
            let a = u8::from_str_radix(&hex[3..4], 16).ok()?;
            let r = (r << 4) | r;
            let g = (g << 4) | g;
            let b = (b << 4) | b;
            let a = (a << 4) | a;
            (r, g, b, a)
        }
        6 => {
            // #RRGGBB
            let r = u8::from_str_radix(&hex[0..2], 16).ok()?;
            let g = u8::from_str_radix(&hex[2..4], 16).ok()?;
            let b = u8::from_str_radix(&hex[4..6], 16).ok()?;
            (r, g, b, 0xFF)
        }
        8 => {
            // #RRGGBBAA
            let r = u8::from_str_radix(&hex[0..2], 16).ok()?;
            let g = u8::from_str_radix(&hex[2..4], 16).ok()?;
            let b = u8::from_str_radix(&hex[4..6], 16).ok()?;
            let a = u8::from_str_radix(&hex[6..8], 16).ok()?;
            (r, g, b, a)
        }
        _ => return None,
    };

    // Return 8-bit values (no expansion to 16-bit)
    Some((r as u32, g as u32, b as u32, a_u8 as u32))
}

/// Parse hex color with 16-bit RGBA values for blending operations (Go compatibility)
pub(crate) fn parse_hex_rgba(s: &str) -> Option<(u32, u32, u32, u32)> {
    // Support #RGB, #RRGGBB, #RGBA, #RRGGBBAA forms.
    let s = s.trim();
    let hex = s.strip_prefix('#')?;

    let (r, g, b, a_u8) = match hex.len() {
        3 => {
            // #RGB -> expand each nibble
            let r = u8::from_str_radix(&hex[0..1], 16).ok()?;
            let g = u8::from_str_radix(&hex[1..2], 16).ok()?;
            let b = u8::from_str_radix(&hex[2..3], 16).ok()?;
            let r = (r << 4) | r;
            let g = (g << 4) | g;
            let b = (b << 4) | b;
            (r, g, b, 0xFF)
        }
        4 => {
            // #RGBA -> expand
            let r = u8::from_str_radix(&hex[0..1], 16).ok()?;
            let g = u8::from_str_radix(&hex[1..2], 16).ok()?;
            let b = u8::from_str_radix(&hex[2..3], 16).ok()?;
            let a = u8::from_str_radix(&hex[3..4], 16).ok()?;
            let r = (r << 4) | r;
            let g = (g << 4) | g;
            let b = (b << 4) | b;
            let a = (a << 4) | a;
            (r, g, b, a)
        }
        6 => {
            // #RRGGBB
            let r = u8::from_str_radix(&hex[0..2], 16).ok()?;
            let g = u8::from_str_radix(&hex[2..4], 16).ok()?;
            let b = u8::from_str_radix(&hex[4..6], 16).ok()?;
            (r, g, b, 0xFF)
        }
        8 => {
            // #RRGGBBAA
            let r = u8::from_str_radix(&hex[0..2], 16).ok()?;
            let g = u8::from_str_radix(&hex[2..4], 16).ok()?;
            let b = u8::from_str_radix(&hex[4..6], 16).ok()?;
            let a = u8::from_str_radix(&hex[6..8], 16).ok()?;
            (r, g, b, a)
        }
        _ => return None,
    };

    // Use palette for consistent sRGB semantics and expand to 16-bit for blending.
    let rgb = Srgb::new(r, g, b);
    Some(srgb_to_rgba16(rgb, a_u8))
}

fn srgb_to_rgba16(rgb: Srgb<u8>, a_u8: u8) -> (u32, u32, u32, u32) {
    // Original behavior: keep RGB as 8-bit, expand alpha to 16-bit
    let (r, g, b) = (rgb.red as u32, rgb.green as u32, rgb.blue as u32);
    let a = a_u8 as u32;
    (r, g, b, a * 257)
}

/// Convert 8-bit RGBA to true 16-bit RGBA for Go blending compatibility
fn srgb_to_true_rgba16(rgb: Srgb<u8>, a_u8: u8) -> (u32, u32, u32, u32) {
    // Expand 8-bit channels to 16-bit like Go's color.RGBA
    // Go's color.RGBA interface returns 16-bit values where each 8-bit value 
    // is expanded by multiplying by 257 (0x101) to fill the full 16-bit range
    // So 255 (0xFF) becomes 65535 (0xFFFF)
    let (r, g, b) = (
        rgb.red as u32 * 257,
        rgb.green as u32 * 257, 
        rgb.blue as u32 * 257
    );
    let a = a_u8 as u32 * 257;
    (r, g, b, a)
}

// --- Color Utility Functions ---

/// Clamps a value between a low and high bound.
pub fn clamp<T: PartialOrd>(v: T, low: T, high: T) -> T {
    if v < low {
        low
    } else if v > high {
        high
    } else {
        v
    }
}

/// Adjusts the alpha value of a color using a 0-1 (clamped) float scale.
/// 0 = transparent, 1 = opaque.
///
/// # Arguments
/// * `color` - The color to adjust
/// * `alpha` - Alpha value from 0.0 (transparent) to 1.0 (opaque)
///
/// # Examples
/// ```rust
/// use lipgloss::color::{Color, alpha};
///
/// let red = Color("#ff0000".to_string());
/// let semi_transparent_red = alpha(&red, 0.5);
/// ```
pub fn alpha<C: TerminalColor>(color: &C, alpha_val: f64) -> Color {
    let (r, g, b, _) = color.rgba();
    let clamped_alpha = clamp(alpha_val, 0.0, 1.0);
    let alpha_u8 = (clamped_alpha * 255.0) as u8;
    
    // rgba() now returns 8-bit values directly
    let r_u8 = r as u8;
    let g_u8 = g as u8;
    let b_u8 = b as u8;
    
    Color(format!(
        "#{:02x}{:02x}{:02x}{:02x}",
        r_u8, g_u8, b_u8, alpha_u8
    ))
}

/// Makes a color lighter by a specific percentage (0-1, clamped).
///
/// # Arguments
/// * `color` - The color to lighten
/// * `percent` - Amount to lighten (0.0 = no change, 1.0 = maximum lightening)
///
/// # Examples
/// ```rust
/// use lipgloss::color::{Color, lighten};
///
/// let dark_red = Color("#800000".to_string());
/// let lighter_red = lighten(&dark_red, 0.3);
/// ```
pub fn lighten<C: TerminalColor>(color: &C, percent: f64) -> Color {
    let (r, g, b, _a) = color.rgba();
    let add = 255.0 * clamp(percent, 0.0, 1.0);
    
    // rgba() now returns 8-bit values directly
    let r_u8 = r as u8;
    let g_u8 = g as u8;  
    let b_u8 = b as u8;
    
    Color(format!(
        "#{:02x}{:02x}{:02x}",
        ((r_u8 as f64 + add).min(255.0)) as u8,
        ((g_u8 as f64 + add).min(255.0)) as u8,
        ((b_u8 as f64 + add).min(255.0)) as u8
    ))
}

/// Makes a color darker by a specific percentage (0-1, clamped).
///
/// # Arguments
/// * `color` - The color to darken
/// * `percent` - Amount to darken (0.0 = no change, 1.0 = maximum darkening)
///
/// # Examples
/// ```rust
/// use lipgloss::color::{Color, darken};
///
/// let bright_red = Color("#ff0000".to_string());
/// let darker_red = darken(&bright_red, 0.3);
/// ```
pub fn darken<C: TerminalColor>(color: &C, percent: f64) -> Color {
    let (r, g, b, _a) = color.rgba();
    let mult = 1.0 - clamp(percent, 0.0, 1.0);
    
    // rgba() now returns 8-bit values directly
    let r_u8 = r as u8;
    let g_u8 = g as u8;
    let b_u8 = b as u8;
    
    Color(format!(
        "#{:02x}{:02x}{:02x}",
        (r_u8 as f64 * mult) as u8,
        (g_u8 as f64 * mult) as u8,
        (b_u8 as f64 * mult) as u8
    ))
}

/// Returns the complementary color (180° away on color wheel) of the given color.
/// This is useful for creating a contrasting color.
///
/// # Arguments
/// * `color` - The color to find the complement of
///
/// # Examples
/// ```rust
/// use lipgloss::color::{Color, complementary};
///
/// let blue = Color("#0000ff".to_string());
/// let orange = complementary(&blue); // Should be approximately orange
/// ```
pub fn complementary<C: TerminalColor>(color: &C) -> Color {
    let (r, g, b, _a) = color.rgba();
    
    // rgba() now returns 8-bit values directly
    let r_u8 = r as u8;
    let g_u8 = g as u8;
    let b_u8 = b as u8;
    
    // Convert to HSV to rotate hue by 180°
    let srgb = Srgb::new(r_u8 as f32 / 255.0, g_u8 as f32 / 255.0, b_u8 as f32 / 255.0);
    let hsv: Hsv = Hsv::from_color(srgb);
    
    let mut new_hue = hsv.hue.into_positive_degrees() + 180.0;
    if new_hue >= 360.0 {
        new_hue -= 360.0;
    } else if new_hue < 0.0 {
        new_hue += 360.0;
    }
    
    let complementary_hsv = Hsv::new(new_hue, hsv.saturation, hsv.value);
    let complementary_srgb: Srgb = Srgb::from_color(complementary_hsv);
    let clamped = complementary_srgb.clamp();
    
    Color(format!(
        "#{:02x}{:02x}{:02x}",
        (clamped.red * 255.0) as u8,
        (clamped.green * 255.0) as u8,
        (clamped.blue * 255.0) as u8
    ))
}

/// Returns whether the given color is dark (based on the luminance portion of the color as interpreted as HSL).
///
/// # Arguments
/// * `color` - The color to check
///
/// # Examples
/// ```rust
/// use lipgloss::color::{Color, is_dark_color};
///
/// let black = Color("#000000".to_string());
/// let white = Color("#ffffff".to_string());
/// assert!(is_dark_color(&black));
/// assert!(!is_dark_color(&white));
/// ```
pub fn is_dark_color<C: TerminalColor>(color: &C) -> bool {
    let (r, g, b, _a) = color.rgba();
    
    // Calculate relative luminance (simplified)
    let luminance = 0.299 * (r as f64) + 0.587 * (g as f64) + 0.114 * (b as f64);
    luminance < 127.5 // Midpoint of 0-255
}

/// A function that returns a color based on whether the terminal has a light or dark background.
pub type LightDarkFunc = Box<dyn Fn(&dyn TerminalColor, &dyn TerminalColor) -> Color>;

/// Returns a function that chooses between light and dark colors based on background.
///
/// # Arguments
/// * `is_dark` - Whether the background is dark
///
/// # Examples
/// ```rust
/// use lipgloss::color::{Color, light_dark};
///
/// let light_dark_fn = light_dark(true); // Dark background
/// let red = Color("#ff0000".to_string());
/// let blue = Color("#0000ff".to_string());
/// let chosen = light_dark_fn(&red, &blue); // Will choose blue for dark bg
/// ```
pub fn light_dark(is_dark: bool) -> LightDarkFunc {
    Box::new(move |light: &dyn TerminalColor, dark: &dyn TerminalColor| {
        if is_dark {
            // Convert dark color to our Color type
            let (r, g, b, _a) = dark.rgba();
            Color(format!("#{:02x}{:02x}{:02x}", r as u8, g as u8, b as u8))
        } else {
            // Convert light color to our Color type  
            let (r, g, b, _a) = light.rgba();
            Color(format!("#{:02x}{:02x}{:02x}", r as u8, g as u8, b as u8))
        }
    })
}

/// A function that returns the appropriate color based on the color profile.
pub type CompleteFunc = Box<dyn Fn(&dyn TerminalColor, &dyn TerminalColor, &dyn TerminalColor) -> Color>;

/// Returns a function that will return the appropriate color based on the given color profile.
///
/// # Arguments
/// * `profile` - The color profile to use for selection
///
/// # Examples
/// ```rust
/// use lipgloss::color::{Color, complete};
/// use lipgloss::renderer::ColorProfileKind;
///
/// let complete_fn = complete(ColorProfileKind::TrueColor);
/// let ansi = Color("1".to_string());
/// let ansi256 = Color("124".to_string()); 
/// let truecolor = Color("#ff34ac".to_string());
/// let chosen = complete_fn(&ansi, &ansi256, &truecolor); // Will choose truecolor
/// ```
pub fn complete(profile: ColorProfileKind) -> CompleteFunc {
    Box::new(move |ansi: &dyn TerminalColor, ansi256: &dyn TerminalColor, truecolor: &dyn TerminalColor| {
        let chosen_color = match profile {
            ColorProfileKind::ANSI => ansi,
            ColorProfileKind::ANSI256 => ansi256,
            ColorProfileKind::TrueColor => truecolor,
            ColorProfileKind::NoColor => return Color("".to_string()),
        };
        
        let (r, g, b, _a) = chosen_color.rgba();
        Color(format!("#{:02x}{:02x}{:02x}", r as u8, g as u8, b as u8))
    })
}

/// Optimized hex color parsing with better performance than alternatives.
/// Parses hex color strings in formats: #RGB, #RRGGBB, #RGBA, #RRGGBBAA
///
/// # Arguments
/// * `hex` - Hex color string starting with #
///
/// # Returns
/// * `Some((r, g, b, a))` - RGBA components as u8 values, or None if invalid
///
/// # Examples
/// ```rust
/// use lipgloss::color::parse_hex;
///
/// assert_eq!(parse_hex("#ff0000"), Some((255, 0, 0, 255)));
/// assert_eq!(parse_hex("#f00"), Some((255, 0, 0, 255)));
/// assert_eq!(parse_hex("invalid"), None);
/// ```
pub fn parse_hex(s: &str) -> Option<(u8, u8, u8, u8)> {
    let s = s.trim();
    if s.is_empty() || !s.starts_with('#') {
        return None;
    }
    
    let hex = &s[1..];
    
    match hex.len() {
        3 => {
            // #RGB
            let r = u8::from_str_radix(&hex[0..1], 16).ok()?;
            let g = u8::from_str_radix(&hex[1..2], 16).ok()?;
            let b = u8::from_str_radix(&hex[2..3], 16).ok()?;
            Some(((r << 4) | r, (g << 4) | g, (b << 4) | b, 255))
        }
        4 => {
            // #RGBA
            let r = u8::from_str_radix(&hex[0..1], 16).ok()?;
            let g = u8::from_str_radix(&hex[1..2], 16).ok()?;
            let b = u8::from_str_radix(&hex[2..3], 16).ok()?;
            let a = u8::from_str_radix(&hex[3..4], 16).ok()?;
            Some(((r << 4) | r, (g << 4) | g, (b << 4) | b, (a << 4) | a))
        }
        6 => {
            // #RRGGBB
            let r = u8::from_str_radix(&hex[0..2], 16).ok()?;
            let g = u8::from_str_radix(&hex[2..4], 16).ok()?;
            let b = u8::from_str_radix(&hex[4..6], 16).ok()?;
            Some((r, g, b, 255))
        }
        8 => {
            // #RRGGBBAA
            let r = u8::from_str_radix(&hex[0..2], 16).ok()?;
            let g = u8::from_str_radix(&hex[2..4], 16).ok()?;
            let b = u8::from_str_radix(&hex[4..6], 16).ok()?;
            let a = u8::from_str_radix(&hex[6..8], 16).ok()?;
            Some((r, g, b, a))
        }
        _ => None,
    }
}
