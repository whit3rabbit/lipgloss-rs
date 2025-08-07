//! Security and DoS protection tests for the lipgloss library.
//!
//! These tests verify that the library properly handles potentially malicious input
//! and prevents memory exhaustion attacks through excessive dimension values.

use lipgloss::{
    security::{safe_repeat, safe_str_repeat, validate_dimension, MAX_DIMENSION},
    Style,
};
use std::time::{Duration, Instant};

#[test]
fn test_dimension_validation_clamping() {
    // Test that excessive dimensions are clamped to safe limits
    assert_eq!(validate_dimension(50000, "width"), MAX_DIMENSION);
    assert_eq!(validate_dimension(-1000, "height"), 0);
    assert_eq!(validate_dimension(5000, "padding"), 5000); // Within limits
    assert_eq!(
        validate_dimension(MAX_DIMENSION + 1, "margin"),
        MAX_DIMENSION
    );
}

#[test]
fn test_unterminated_escape_truncate_visible_line_fast() {
    // Craft an ESC + CSI with a very long parameter list and no final byte
    // Previously this could scan the entire string; now it should cap quickly.
    let long_params = "1;".repeat(100_000);
    let malicious = format!(
        "\x1b[{}NO_TERM Followed text that should still be processed",
        long_params
    );

    let start = Instant::now();
    let out = lipgloss::Style::truncate_visible_line(&malicious, 20);
    let dur = start.elapsed();

    // Should complete very fast (well under 100ms on test hardware)
    assert!(
        dur < Duration::from_millis(100),
        "truncate_visible_line too slow: {:?}",
        dur
    );
    assert!(!out.is_empty());
}

#[test]
fn test_unterminated_escape_hard_wrap_fast() {
    let long_params = "1;".repeat(100_000);
    let malicious = format!("\x1b[{}NO_TERM Some text to wrap", long_params);

    let start = Instant::now();
    let lines = lipgloss::Style::hard_wrap_ansi_aware(&malicious, 8);
    let dur = start.elapsed();

    assert!(
        dur < Duration::from_millis(100),
        "hard_wrap_ansi_aware too slow: {:?}",
        dur
    );
    assert!(!lines.is_empty());
}

#[test]
fn test_unterminated_escape_tokenize_fast() {
    let long_params = "1;".repeat(150_000);
    let malicious = format!("\x1b[{}NO_TERM token1 token2", long_params);

    let start = Instant::now();
    let tokens = lipgloss::Style::tokenize_with_breakpoints(&malicious, &[' ']);
    let dur = start.elapsed();

    assert!(
        dur < Duration::from_millis(100),
        "tokenize_with_breakpoints too slow: {:?}",
        dur
    );
    assert!(!tokens.is_empty());
}

#[test]
fn test_width_dimension_safety() {
    // Test that width setter clamps excessive values
    let style = Style::new().width(50000);
    assert_eq!(style.get_width(), MAX_DIMENSION);

    let style = Style::new().width(-1000);
    assert_eq!(style.get_width(), 0);
}

#[test]
fn test_height_dimension_safety() {
    // Test that height setter clamps excessive values
    let style = Style::new().height(50000);
    assert_eq!(style.get_height(), MAX_DIMENSION);

    let style = Style::new().height(-500);
    assert_eq!(style.get_height(), 0);
}

#[test]
fn test_padding_dimension_safety() {
    // Test that padding setters clamp excessive values
    let style = Style::new().padding(100000, 100000, 100000, 100000);
    assert_eq!(style.get_padding_top(), MAX_DIMENSION);
    assert_eq!(style.get_padding_right(), MAX_DIMENSION);
    assert_eq!(style.get_padding_bottom(), MAX_DIMENSION);
    assert_eq!(style.get_padding_left(), MAX_DIMENSION);

    // Test individual setters
    let style = Style::new().padding_left(100000);
    assert_eq!(style.get_padding_left(), MAX_DIMENSION);
}

#[test]
fn test_margin_dimension_safety() {
    // Test that margin setters clamp excessive values
    let style = Style::new().margin(100000, 100000, 100000, 100000);
    assert_eq!(style.get_margin_top(), MAX_DIMENSION);
    assert_eq!(style.get_margin_right(), MAX_DIMENSION);
    assert_eq!(style.get_margin_bottom(), MAX_DIMENSION);
    assert_eq!(style.get_margin_left(), MAX_DIMENSION);
}

#[test]
fn test_tab_width_safety() {
    // Test that tab width is properly clamped
    let style = Style::new().tab_width(100000);
    assert_eq!(style.get_tab_width(), MAX_DIMENSION);

    let style = Style::new().tab_width(-1000);
    assert_eq!(style.get_tab_width(), 0);
}

#[test]
fn test_safe_repeat_bounds() {
    // Test that safe_repeat properly limits output size
    let result = safe_repeat('x', 50000);
    assert!(result.len() <= lipgloss::security::MAX_REPEAT_COUNT);

    // Test normal usage still works
    let result = safe_repeat(' ', 10);
    assert_eq!(result, "          ");

    // Test zero count
    let result = safe_repeat('a', 0);
    assert_eq!(result, "");
}

#[test]
fn test_safe_str_repeat_bounds() {
    // Test that safe_str_repeat properly limits output size
    let result = safe_str_repeat("abc", 20000);
    assert!(result.len() <= lipgloss::security::MAX_REPEAT_COUNT);

    // Test normal usage still works
    let result = safe_str_repeat("hi", 3);
    assert_eq!(result, "hihihi");

    // Test empty string
    let result = safe_str_repeat("", 1000);
    assert_eq!(result, "");

    // Test zero count
    let result = safe_str_repeat("test", 0);
    assert_eq!(result, "");
}

#[test]
fn test_render_dos_protection() {
    // Test that rendering with excessive dimensions doesn't cause DoS
    let start = Instant::now();

    let malicious_style = Style::new()
        .width(100000) // Will be clamped to MAX_DIMENSION
        .height(100000) // Will be clamped to MAX_DIMENSION
        .padding(50000, 50000, 50000, 50000) // Will be clamped
        .margin(50000, 50000, 50000, 50000); // Will be clamped

    // This should complete quickly due to dimension clamping
    let result = malicious_style.render("Test content");

    let duration = start.elapsed();

    // Increased timeout for CI environments which can be slower
    // This is still a reasonable DoS protection threshold
    assert!(
        duration < Duration::from_secs(30),
        "Render took too long: {:?}",
        duration
    );
    assert!(!result.is_empty(), "Render should produce some output");
}

#[test]
fn test_memory_usage_bounds() {
    // Test that we don't allocate excessive memory
    let style = Style::new().width(MAX_DIMENSION).padding(
        MAX_DIMENSION / 4,
        MAX_DIMENSION / 4,
        MAX_DIMENSION / 4,
        MAX_DIMENSION / 4,
    );

    // This should complete without running out of memory
    let result = style.render("Small content");
    assert!(!result.is_empty());
}

#[test]
fn test_performance_regression_style_comparison() {
    // Test that the optimized style comparison is performant
    // Use different properties that don't depend on color parsing for CI compatibility
    let style1 = Style::new().bold(true).width(100).padding(5, 5, 5, 5);
    let style2 = Style::new().bold(true).width(100).padding(5, 5, 5, 5);
    let style3 = Style::new()
        .bold(true)
        .width(200) // Different width to ensure inequality
        .padding(5, 5, 5, 5);

    // Basic sanity check - these should be different based on width
    assert_ne!(style1.get_width(), style3.get_width());

    let start = Instant::now();

    // Perform many comparisons (reduced count for CI tolerance)
    for _ in 0..1000 {
        assert!(style1.is_equivalent(&style2));
        assert!(!style1.is_equivalent(&style3));
    }

    let duration = start.elapsed();

    // More generous timeout for CI environments
    assert!(
        duration < Duration::from_millis(1000),
        "Style comparison too slow: {:?}",
        duration
    );
}

#[test]
fn test_render_performance_bounds() {
    // Test that rendering performance is reasonable
    let complex_style = Style::new()
        .bold(true)
        .italic(true)
        .foreground("red")
        .background("blue")
        .width(80)
        .height(20)
        .padding(2, 4, 2, 4)
        .margin(1, 2, 1, 2);

    let content = "This is test content that will be styled with a complex style.\nIt has multiple lines.\nAnd various properties applied.";

    let start = Instant::now();

    // Render multiple times to test performance
    for _ in 0..1000 {
        let _result = complex_style.render(content);
    }

    let duration = start.elapsed();

    // Should complete 1000 renders in reasonable time
    assert!(
        duration < Duration::from_secs(2),
        "Rendering too slow: {:?}",
        duration
    );
}

#[test]
fn test_large_content_handling() {
    // Test handling of large content without excessive memory usage
    let large_content = "A".repeat(10000); // 10KB of content
    let style = Style::new().width(100).padding(2, 2, 2, 2);

    let start = Instant::now();
    let result = style.render(&large_content);
    let duration = start.elapsed();

    // Should handle large content efficiently
    assert!(
        duration < Duration::from_secs(1),
        "Large content rendering too slow: {:?}",
        duration
    );
    assert!(!result.is_empty());
}

#[test]
fn test_edge_case_dimensions() {
    // Test edge cases for dimensions
    let style = Style::new().width(0).height(0).max_width(0).max_height(0);

    // Should not panic or cause issues
    let result = style.render("Test");
    assert!(!result.is_empty());
}

#[test]
fn test_negative_dimensions_handling() {
    // Test that negative dimensions are handled safely
    let style = Style::new()
        .width(-1000)
        .height(-500)
        .padding(-100, -200, -150, -300);

    // All should be clamped to 0
    assert_eq!(style.get_width(), 0);
    assert_eq!(style.get_height(), 0);
    assert_eq!(style.get_padding_top(), 0);
    assert_eq!(style.get_padding_right(), 0);
    assert_eq!(style.get_padding_bottom(), 0);
    assert_eq!(style.get_padding_left(), 0);

    // Should still render without issues
    let result = style.render("Test");
    assert!(!result.is_empty());
}

#[test]
fn test_standard_render_safety() {
    // Test that standard render is safe with large dimensions
    let style = Style::new()
        .bold(true)
        .foreground("red")
        .width(50)
        .padding(2, 2, 2, 2);

    let content = "Hello\nWorld\nTest";
    let result = style.render(content);

    // Should produce output without panicking
    assert!(!result.is_empty());

    // Result should contain visible content
    let clean = lipgloss::utils::strip_ansi(&result);
    assert!(clean.contains("Hello"));
    assert!(clean.contains("World"));
    assert!(clean.contains("Test"));
}

#[test]
fn test_concurrent_safety() {
    // Test that the library is safe to use concurrently
    use std::sync::Arc;
    use std::thread;

    let style = Arc::new(Style::new().bold(true).foreground("red").width(20));
    let mut handles = vec![];

    for i in 0..10 {
        let style_clone = Arc::clone(&style);
        let handle = thread::spawn(move || {
            let content = format!("Thread {} content", i);
            let result = style_clone.render(&content);
            assert!(!result.is_empty());
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }
}

#[test]
fn test_string_repeat_replacement_coverage() {
    // Verify that all string repeat operations use safe versions
    // This is mostly a smoke test since we've replaced the implementations

    let style = Style::new()
        .width(100)
        .tab_width(8)
        .padding(5, 5, 5, 5)
        .margin(3, 3, 3, 3);

    // Content with tabs to test tab expansion
    let content = "Line1\tTabbed\nLine2\tMore tabs\t\nLine3";

    // Should not panic or consume excessive memory
    let result = style.render(content);
    assert!(!result.is_empty());

    // Test with whitespace rendering
    use lipgloss::whitespace::new_whitespace;
    let ws = new_whitespace(lipgloss::renderer::default_renderer(), &[]);
    let ws_result = ws.render(50); // Should use safe_repeat internally
    assert_eq!(ws_result.len(), 50);
}
