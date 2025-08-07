use lipgloss::position::{Position, place_horizontal, place_vertical, LEFT, RIGHT, CENTER, TOP, BOTTOM};
use lipgloss::renderer::Renderer;

/// Create a renderer for testing (matches Go's blackhole struct functionality)
fn blackhole_renderer() -> Renderer {
    Renderer::new()
}

#[test]
fn test_place_horizontal() {
    let test_cases = vec![
        // odd spacing
        (10, "Hello", LEFT, "Hello     "),
        (10, "Hello", Position(0.0), "Hello     "),
        (10, "Hello", Position(0.000000001), "Hello     "),
        (10, "Hello", RIGHT, "     Hello"),
        (10, "Hello", Position(1.0), "     Hello"),
        (10, "Hello", Position(0.999999999), "     Hello"),
        (10, "Hello", Position(0.49), "  Hello   "),
        (10, "Hello", CENTER, "  Hello   "),
        (10, "Hello", Position(0.51), "   Hello  "),
    ];

    for (i, (width, text, pos, expected)) in test_cases.iter().enumerate() {
        let renderer = blackhole_renderer();
        let actual = renderer.place_horizontal(*width, *pos, text, &[]);
        assert_eq!(actual, *expected, "Test {}: expected {:?}, got {:?}", i, expected, actual);
    }
}

#[test]
fn test_place_vertical() {
    let test_cases = vec![
        (3, "Hello", TOP, "Hello\n     \n     "),
        (3, "Hello", Position(0.0), "Hello\n     \n     "),
        (3, "Hello", Position(0.000000001), "Hello\n     \n     "),
        (3, "Hello", BOTTOM, "     \n     \nHello"),
        (3, "Hello", Position(1.0), "     \n     \nHello"),
        (3, "Hello", Position(0.999999999), "     \n     \nHello"),
        (4, "Hello", Position(0.49), "     \nHello\n     \n     "),
        (4, "Hello", CENTER, "     \nHello\n     \n     "),
        (4, "Hello", Position(0.51), "     \n     \nHello\n     "),
    ];

    for (i, (height, content, position, expected)) in test_cases.iter().enumerate() {
        let renderer = blackhole_renderer();
        let actual = renderer.place_vertical(*height, *position, content, &[]);
        assert_eq!(actual, *expected, "Test {}: expected {:?}, got {:?}", i, expected, actual);
    }
}

#[test]
fn test_place_horizontal_global_functions() {
    // Test the global functions work the same as renderer methods
    let test_cases = vec![
        (10, "Hello", LEFT, "Hello     "),
        (10, "Hello", CENTER, "  Hello   "),
        (10, "Hello", RIGHT, "     Hello"),
    ];

    for (width, text, pos, expected) in test_cases {
        let actual = place_horizontal(width, pos, text, &[]);
        assert_eq!(actual, expected);
    }
}

#[test]
fn test_place_vertical_global_functions() {
    // Test the global functions work the same as renderer methods  
    let test_cases = vec![
        (3, "Hello", TOP, "Hello\n     \n     "),
        (4, "Hello", CENTER, "     \nHello\n     \n     "),
        (3, "Hello", BOTTOM, "     \n     \nHello"),
    ];

    for (height, content, position, expected) in test_cases {
        let actual = place_vertical(height, position, content, &[]);
        assert_eq!(actual, expected);
    }
}

#[test]
fn test_position_value_clamping() {
    // Test that position values are properly clamped
    assert_eq!(Position(-0.5).value(), 0.0);
    assert_eq!(Position(0.5).value(), 0.5);
    assert_eq!(Position(1.5).value(), 1.0);
}

#[test]
fn test_symmetric_center_positioning() {
    // Test that center positioning is symmetric for different widths
    let test_cases = vec![
        (6, "Hi", "  Hi  "),    // even width
        (7, "Hi", "  Hi   "),   // odd width  
        (8, "Hi", "   Hi   "),  // even width
        (9, "Hi", "   Hi    "), // odd width
    ];

    for (width, text, expected) in test_cases {
        let actual = place_horizontal(width, CENTER, text, &[]);
        assert_eq!(actual, expected, "Width {}: expected {:?}, got {:?}", width, expected, actual);
    }
}

#[test]  
fn test_symmetric_center_positioning_vertical() {
    // Test that center positioning is symmetric for different heights
    let test_cases = vec![
        (4, "Hi", "  \nHi\n  \n  "),   // even height
        (5, "Hi", "  \n  \nHi\n  \n  "), // odd height
    ];

    for (height, text, expected) in test_cases {
        let actual = place_vertical(height, CENTER, text, &[]);
        assert_eq!(actual, expected, "Height {}: expected {:?}, got {:?}", height, expected, actual);
    }
}

#[test]
fn test_multiline_horizontal_placement() {
    // Test placing multiline text
    let text = "Hello\nWorld";
    let actual = place_horizontal(10, CENTER, text, &[]);
    let expected = "  Hello   \n  World   ";
    assert_eq!(actual, expected);
}

#[test]
fn test_zero_gap_handling() {
    // Test when content is same size or larger than target
    assert_eq!(place_horizontal(5, CENTER, "Hello", &[]), "Hello");
    assert_eq!(place_horizontal(3, CENTER, "Hello", &[]), "Hello");
    assert_eq!(place_vertical(1, CENTER, "Hello", &[]), "Hello");
}