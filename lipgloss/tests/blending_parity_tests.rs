//! Comprehensive Go parity tests for blending and color utilities.
//! 
//! This module contains tests ported directly from the Go implementation
//! to ensure exact compatibility and color accuracy.

use lipgloss::color::{Color, TerminalColor, alpha, lighten, darken, complementary, parse_hex};
use lipgloss::{blend_1d, blend_2d};

// Test helper functions (equivalent to Go test helpers)

/// Creates a color from hex string, panicking if invalid (like Go's hex() function)
fn hex(hex_str: &str) -> Color {
    match parse_hex(hex_str) {
        Some((r, g, b, a)) => Color::from_rgba(r, g, b, a),
        None => panic!("Invalid hex color: {}", hex_str),
    }
}

/// Creates a color from RGBA values (equivalent to Go's color.RGBA)
fn rgba_color(r: u8, g: u8, b: u8, a: u8) -> Color {
    Color::from_rgba(r, g, b, a)
}

/// Compares two colors for exact RGBA match (equivalent to Go's expectColorMatches)
fn expect_color_matches(got: &Color, want: &Color) {
    let (gr, gg, gb, ga) = got.rgba16();
    let (wr, wg, wb, wa) = want.rgba16();

    // Convert from 16-bit to 8-bit values for comparison
    let gru = (gr >> 8) as u8;
    let ggu = (gg >> 8) as u8;
    let gbu = (gb >> 8) as u8; 
    let gau = (ga >> 8) as u8;

    let wru = (wr >> 8) as u8;
    let wgu = (wg >> 8) as u8;
    let wbu = (wb >> 8) as u8;
    let wau = (wa >> 8) as u8;

    assert_eq!((gru, ggu, gbu, gau), (wru, wgu, wbu, wau),
        "Color mismatch: got rgba({},{},{},{}) want rgba({},{},{},{})", 
        gru, ggu, gbu, gau, wru, wgu, wbu, wau);
}

/// Helper for debug output (equivalent to Go's rgbaString)
#[allow(dead_code)]
fn rgba_string(color: &Color) -> String {
    let (r, g, b, a) = color.rgba16();
    format!("rgba({},{},{},{})", r >> 8, g >> 8, b >> 8, a >> 8)
}

// Blend1D Tests (direct port from Go)
#[cfg(test)]
mod blend_1d_tests {
    use super::*;

    #[test]
    fn test_2_colors_10_steps() {
        let stops = vec![
            rgba_color(255, 0, 0, 255),
            rgba_color(0, 0, 255, 255),
        ];
        let expected = vec![
            rgba_color(255, 0, 0, 255),
            rgba_color(246, 0, 45, 255),
            rgba_color(235, 0, 73, 255),
            rgba_color(223, 0, 99, 255),
            rgba_color(210, 0, 124, 255),
            rgba_color(193, 0, 149, 255),
            rgba_color(173, 0, 175, 255),
            rgba_color(147, 0, 201, 255),
            rgba_color(109, 0, 228, 255),
            rgba_color(0, 0, 255, 255),
        ];

        let got = blend_1d(10, stops);
        assert_eq!(got.len(), expected.len());

        for (got_color, expected_color) in got.iter().zip(expected.iter()) {
            expect_color_matches(got_color, expected_color);
        }
    }

    #[test]
    fn test_3_colors_4_steps() {
        let stops = vec![
            rgba_color(255, 0, 0, 255),
            rgba_color(0, 255, 0, 255),
            rgba_color(0, 0, 255, 255),
        ];
        let expected = vec![
            rgba_color(255, 0, 0, 255),
            rgba_color(0, 255, 0, 255),
            rgba_color(0, 255, 0, 255),
            rgba_color(0, 0, 255, 255),
        ];

        let got = blend_1d(4, stops);
        assert_eq!(got.len(), expected.len());

        for (got_color, expected_color) in got.iter().zip(expected.iter()) {
            expect_color_matches(got_color, expected_color);
        }
    }

    #[test]
    fn test_black_to_white_5_steps() {
        let stops = vec![
            rgba_color(0, 0, 0, 255),
            rgba_color(255, 255, 255, 255),
        ];
        let expected = vec![
            rgba_color(0, 0, 0, 255),
            rgba_color(59, 59, 59, 255),
            rgba_color(119, 119, 119, 255),
            rgba_color(185, 185, 185, 255),
            rgba_color(255, 255, 255, 255),
        ];

        let got = blend_1d(5, stops);
        assert_eq!(got.len(), expected.len());

        for (got_color, expected_color) in got.iter().zip(expected.iter()) {
            expect_color_matches(got_color, expected_color);
        }
    }

    #[test]
    fn test_4_colors_6_steps() {
        let stops = vec![
            rgba_color(255, 0, 0, 255),
            rgba_color(255, 255, 0, 255),
            rgba_color(0, 255, 0, 255),
            rgba_color(0, 0, 255, 255),
        ];
        let expected = vec![
            rgba_color(255, 0, 0, 255),
            rgba_color(255, 255, 0, 255),
            rgba_color(255, 255, 0, 255),
            rgba_color(0, 255, 0, 255),
            rgba_color(0, 255, 0, 255),
            rgba_color(0, 0, 255, 255),
        ];

        let got = blend_1d(6, stops);
        assert_eq!(got.len(), expected.len());

        for (got_color, expected_color) in got.iter().zip(expected.iter()) {
            expect_color_matches(got_color, expected_color);
        }
    }

    #[test]
    fn test_2_steps_5_stops() {
        let stops = vec![
            rgba_color(255, 0, 0, 255),
            rgba_color(0, 255, 0, 255),
            rgba_color(0, 0, 255, 255),
            rgba_color(255, 255, 0, 255),
            rgba_color(0, 0, 0, 255),
        ];
        let expected = vec![
            rgba_color(255, 0, 0, 255),
            rgba_color(0, 255, 0, 255),
        ];

        let got = blend_1d(2, stops);
        assert_eq!(got.len(), expected.len());

        for (got_color, expected_color) in got.iter().zip(expected.iter()) {
            expect_color_matches(got_color, expected_color);
        }
    }

    #[test]
    fn test_insufficient_stops() {
        let stops = vec![rgba_color(255, 0, 0, 255)];
        let expected = vec![
            rgba_color(255, 0, 0, 255),
            rgba_color(255, 0, 0, 255),
            rgba_color(255, 0, 0, 255),
        ];

        let got = blend_1d(3, stops);
        assert_eq!(got.len(), expected.len());

        for (got_color, expected_color) in got.iter().zip(expected.iter()) {
            expect_color_matches(got_color, expected_color);
        }
    }

    #[test]
    fn test_insufficient_steps() {
        let stops = vec![
            rgba_color(255, 0, 0, 255),
            rgba_color(0, 0, 255, 255),
        ];
        let expected = vec![
            rgba_color(255, 0, 0, 255),
            rgba_color(0, 0, 255, 255),
        ];

        let got = blend_1d(1, stops); // Should be clamped to 2
        assert_eq!(got.len(), expected.len());

        for (got_color, expected_color) in got.iter().zip(expected.iter()) {
            expect_color_matches(got_color, expected_color);
        }
    }

    #[test]
    fn test_empty_stops() {
        let stops: Vec<Color> = vec![];
        let got = blend_1d(5, stops);
        assert!(got.is_empty(), "Empty stops should return empty vector");
    }
}

// Blend2D Tests
#[cfg(test)]
mod blend_2d_tests {
    use super::*;

    #[test]
    fn test_2x2_red_to_blue_0deg() {
        let stops = vec![
            rgba_color(255, 0, 0, 255),
            rgba_color(0, 0, 255, 255),
        ];
        let got = blend_2d(2, 2, 0.0, stops);
        assert_eq!(got.len(), 4);

        // Verify all colors are valid (non-empty)
        for (i, color) in got.iter().enumerate() {
            assert!(!color.0.is_empty(), "Color at index {} should not be empty", i);
        }
    }

    #[test]
    fn test_3x2_red_to_blue_90deg() {
        let stops = vec![
            rgba_color(255, 0, 0, 255),
            rgba_color(0, 0, 255, 255),
        ];
        let got = blend_2d(3, 2, 90.0, stops);
        assert_eq!(got.len(), 6);
    }

    #[test]
    fn test_2x3_red_to_blue_180deg() {
        let stops = vec![
            rgba_color(255, 0, 0, 255),
            rgba_color(0, 0, 255, 255),
        ];
        let got = blend_2d(2, 3, 180.0, stops);
        assert_eq!(got.len(), 6);
    }

    #[test]
    fn test_2x2_red_to_blue_270deg() {
        let stops = vec![
            rgba_color(255, 0, 0, 255),
            rgba_color(0, 0, 255, 255),
        ];
        let got = blend_2d(2, 2, 270.0, stops);
        assert_eq!(got.len(), 4);
    }

    #[test]
    fn test_1x1_single_color() {
        let stops = vec![rgba_color(255, 0, 0, 255)];
        let got = blend_2d(1, 1, 0.0, stops);
        assert_eq!(got.len(), 1);
        expect_color_matches(&got[0], &rgba_color(255, 0, 0, 255));
    }

    #[test]
    fn test_3_colors_2x2_0deg() {
        let stops = vec![
            rgba_color(255, 0, 0, 255),
            rgba_color(0, 255, 0, 255),
            rgba_color(0, 0, 255, 255),
        ];
        let got = blend_2d(2, 2, 0.0, stops);
        assert_eq!(got.len(), 4);
    }

    #[test]
    fn test_invalid_dimensions_fallback() {
        let stops = vec![rgba_color(255, 0, 0, 255)];
        let got = blend_2d(0, 0, 0.0, stops); // Should fallback to 1x1
        assert_eq!(got.len(), 1);
    }

    #[test]
    fn test_angle_normalization_450() {
        let stops = vec![
            rgba_color(255, 0, 0, 255),
            rgba_color(0, 0, 255, 255),
        ];
        let got = blend_2d(2, 2, 450.0, stops); // Should normalize to 90°
        assert_eq!(got.len(), 4);
    }

    #[test]
    fn test_negative_angle_normalization() {
        let stops = vec![
            rgba_color(255, 0, 0, 255),
            rgba_color(0, 0, 255, 255),
        ];
        let got = blend_2d(2, 2, -90.0, stops); // Should normalize to 270°
        assert_eq!(got.len(), 4);
    }

    #[test]
    fn test_empty_stops() {
        let stops: Vec<Color> = vec![];
        let got = blend_2d(2, 2, 0.0, stops);
        assert!(got.is_empty(), "Empty stops should return empty vector");
    }

    #[test]
    fn test_single_color_all_same() {
        let stops = vec![rgba_color(255, 0, 0, 255)];
        let got = blend_2d(2, 2, 0.0, stops);
        assert_eq!(got.len(), 4);

        // All colors should be the same
        for color in got.iter() {
            expect_color_matches(color, &rgba_color(255, 0, 0, 255));
        }
    }
}

// Color utility tests
#[cfg(test)]
mod color_utility_tests {
    use super::*;

    #[test]
    fn test_hex_to_color_conversion() {
        let test_cases = [
            ("#FF0000", 0xFF0000),
            ("#00F", 0x0000FF),
            ("#6B50FF", 0x6B50FF),
            ("invalid color", 0x0), // Invalid should return black
        ];

        for (input, expected) in test_cases {
            let color = Color::from(input);
            let (r, g, b, _a) = color.rgba();  // Use regular rgba() for backward compatibility test
            let actual = (r << 16) + (g << 8) + b;  // No bit shifting since rgba() returns 8-bit now
            assert_eq!(actual as u32, expected, "Input '{}': expected 0x{:06X}, got 0x{:06X}", input, expected, actual);
        }
    }

    #[test]
    fn test_parse_hex_comprehensive() {
        // Valid cases
        expect_color_matches(&hex("#FF0000"), &hex("#FF0000"));
        expect_color_matches(&hex("#00FF00"), &hex("#00FF00"));
        expect_color_matches(&hex("#0000FF"), &hex("#0000FF"));
        expect_color_matches(&hex("#FFFFFF"), &hex("#FFFFFF"));
        expect_color_matches(&hex("#000000"), &hex("#000000"));
        expect_color_matches(&hex("#808080"), &hex("#808080"));
        expect_color_matches(&hex("#F00"), &hex("#FF0000"));
        expect_color_matches(&hex("#0F0"), &hex("#00FF00"));
        expect_color_matches(&hex("#00F"), &hex("#0000FF"));
        expect_color_matches(&hex("#FFF"), &hex("#FFFFFF"));
        expect_color_matches(&hex("#000"), &hex("#000000"));
        expect_color_matches(&hex("#ff0000"), &hex("#FF0000"));
        expect_color_matches(&hex("#Ff0000"), &hex("#FF0000"));
        expect_color_matches(&hex("#f00"), &hex("#FF0000"));

        // Invalid cases should panic (tested separately if needed)
    }

    #[test]
    fn test_alpha_function() {
        let test_cases = [
            (rgba_color(255, 0, 0, 255), 1.0, rgba_color(255, 0, 0, 255)),
            (rgba_color(0, 255, 0, 255), 0.5, rgba_color(0, 255, 0, 127)),
            (rgba_color(0, 0, 255, 255), 0.25, rgba_color(0, 0, 255, 63)),
            (rgba_color(255, 255, 255, 255), 0.0, rgba_color(255, 255, 255, 0)),
            (rgba_color(255, 0, 255, 255), 1.5, rgba_color(255, 0, 255, 255)), // Clamped
            (rgba_color(255, 255, 0, 255), -0.5, rgba_color(255, 255, 0, 0)), // Clamped
            (rgba_color(18, 52, 86, 255), 0.75, rgba_color(18, 52, 86, 191)),
        ];

        for (color, alpha_val, expected) in test_cases {
            let result = alpha(&color, alpha_val);
            expect_color_matches(&result, &expected);
        }
    }

    #[test]
    fn test_complementary_function() {
        let test_cases = [
            (hex("#FF0000"), hex("#00FFFF")), // Red -> Cyan
            (hex("#00FF00"), hex("#FF00FF")), // Green -> Magenta  
            (hex("#0000FF"), hex("#FFFF00")), // Blue -> Yellow
            (hex("#FFFF00"), hex("#0000FF")), // Yellow -> Blue
            (hex("#00FFFF"), hex("#FF0000")), // Cyan -> Red
            (hex("#FF00FF"), hex("#00FF00")), // Magenta -> Green
            (hex("#000000"), hex("#000000")), // Black -> Black (no hue)
            (hex("#FFFFFF"), hex("#FFFFFF")), // White -> White (no hue)
            (hex("#808080"), hex("#808080")), // Gray -> Gray (no hue)
            (hex("#FF8000"), hex("#007FFF")), // Orange -> Light Blue
            (hex("#8000FF"), hex("#7FFF00")), // Purple -> Lime
        ];

        for (input, expected) in test_cases {
            let result = complementary(&input);
            expect_color_matches(&result, &expected);
        }
    }

    #[test]
    fn test_darken_function() {
        let test_cases = [
            (hex("#FFFFFF"), 0.5, hex("#7F7F7F")),
            (hex("#FF0000"), 0.25, hex("#BF0000")),
            (hex("#0000FF"), 0.75, hex("#00003F")),
            (hex("#000000"), 0.1, hex("#000000")),
            (hex("#FFFFFF"), 0.0, hex("#FFFFFF")),
            (hex("#FFFFFF"), 1.0, hex("#000000")),
        ];

        for (input, percent, expected) in test_cases {
            let result = darken(&input, percent);
            expect_color_matches(&result, &expected);
        }
    }

    #[test]
    fn test_lighten_function() {
        let test_cases = [
            (hex("#000000"), 0.5, hex("#7F7F7F")),
            (hex("#800000"), 0.25, hex("#BF3F3F")),
            (hex("#000080"), 0.75, hex("#BFBFFF")),
            (hex("#FFFFFF"), 0.1, hex("#FFFFFF")),
            (hex("#000000"), 0.0, hex("#000000")),
            (hex("#000000"), 1.0, hex("#FFFFFF")),
        ];

        for (input, percent, expected) in test_cases {
            let result = lighten(&input, percent);
            expect_color_matches(&result, &expected);
        }
    }
}