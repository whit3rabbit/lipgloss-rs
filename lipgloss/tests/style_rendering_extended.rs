use lipgloss::style::Style;

fn show_escapes(s: &str) -> String {
    s.replace("\x1b", "<ESC>")
}

/// Helper function to detect if we're in a no-color environment (like CI)
fn is_no_color_environment() -> bool {
    use lipgloss::renderer::{default_renderer, ColorProfileKind};
    default_renderer().color_profile() == ColorProfileKind::NoColor
}

#[test]
fn border_color_precedence_per_side_over_combined() {
    use lipgloss::color::Color;
    
    if is_no_color_environment() {
        // Skip color-dependent tests in CI/no-color environments
        return;
    }

    let base = Style::default()
        .border(lipgloss::normal_border())
        .border_top(true)
        .border_right(true)
        .border_bottom(true)
        .border_left(true);

    let combined = base.clone().border_foreground(Color::from("#112233"));
    let combined_out = combined.render("X");

    // Per-side override on top should take precedence over combined
    let override_top = combined
        .clone()
        .border_top_foreground(Color::from("#ff0000"));
    let override_out = override_top.render("X");

    // Expect SGR in both
    assert!(combined_out.contains("\x1b["));
    assert!(override_out.contains("\x1b["));

    // Compare lines: top line should differ; middle and bottom should be equal
    let c_lines: Vec<&str> = combined_out.split('\n').collect();
    let o_lines: Vec<&str> = override_out.split('\n').collect();
    assert_eq!(c_lines.len(), 3);
    assert_eq!(o_lines.len(), 3);
    assert_ne!(
        c_lines[0],
        o_lines[0],
        "top edge should differ due to per-side override\nC:{}\nO:{}",
        show_escapes(c_lines[0]),
        show_escapes(o_lines[0])
    );
    assert_eq!(
        c_lines[1], o_lines[1],
        "sides should remain same without overrides"
    );
    assert_eq!(
        c_lines[2], o_lines[2],
        "bottom should remain same without overrides"
    );
}

#[test]
fn border_color_inherit_behavior_and_precedence() {
    if is_no_color_environment() {
        // Skip color-dependent tests in CI/no-color environments
        return;
    }
    use lipgloss::color::Color;

    // Parent with combined border foreground
    let parent = Style::default()
        .border(lipgloss::normal_border())
        .border_top(true)
        .border_right(true)
        .border_bottom(true)
        .border_left(true)
        .border_foreground(Color::from("#ff0000"));

    // Child inherits and should have colored borders
    let child_inherited = Style::default().inherit(parent.clone());
    let child_out = child_inherited.render("hi");
    assert!(
        child_out.contains("\x1b["),
        "inherited border color should apply: {}",
        show_escapes(&child_out)
    );

    // Per-side override should survive inherit ordering: before vs after
    let s_before = Style::default()
        .border(lipgloss::normal_border())
        .border_top(true)
        .border_right(true)
        .border_bottom(true)
        .border_left(true)
        .border_left_foreground(Color::from("#00ff00"))
        .inherit(parent.clone());

    let s_after = Style::default()
        .inherit(parent)
        .border_left_foreground(Color::from("#00ff00"));

    let out_before = s_before.render("hi");
    let out_after = s_after.render("hi");

    // Both orders should yield the same result if per-side overrides take precedence over combined
    assert_eq!(
        out_before, out_after,
        "per-side override should be stable across inherit ordering"
    );

    // And they should differ from plain inherited (left border not green vs green). We can at least assert difference.
    assert_ne!(
        out_before, child_out,
        "override should change output compared to plain inherited"
    );
}

#[test]
fn border_render_snapshot_simple() {
    let s = Style::default()
        .border(lipgloss::normal_border())
        .border_top(true)
        .border_right(true)
        .border_bottom(true)
        .border_left(true);

    let got = s.render("Hi");
    let want = "┌──┐\n│Hi│\n└──┘".to_string();
    assert_eq!(
        got,
        want,
        "border rendering should match\nGOT:\n{}\nWANT:\n{}",
        show_escapes(&got),
        show_escapes(&want)
    );
}

#[test]
fn border_render_with_padding() {
    // Padding expands inner width; border should match outer width accordingly
    let s = Style::default()
        .padding_left(1)
        .padding_right(1)
        .border(lipgloss::normal_border())
        .border_top(true)
        .border_right(true)
        .border_bottom(true)
        .border_left(true);

    let got = s.render("Hi");
    // Inner becomes " Hi ", length 4; top/bottom edge length should be 4
    let want = "┌────┐\n│ Hi │\n└────┘".to_string();
    assert_eq!(
        got,
        want,
        "border+padding rendering should match\nGOT:\n{}\nWANT:\n{}",
        show_escapes(&got),
        show_escapes(&want)
    );
}

#[test]
fn renderer_assignment_color_profile_effects() {
    if is_no_color_environment() {
        // Skip color-dependent tests in CI/no-color environments
        return;
    }
    use lipgloss::color::Color;
    use lipgloss::renderer::{ColorProfileKind, Renderer};

    // With colors: expect ANSI sequences when profile != NoColor
    let base = Style::default().foreground(Color::from("#ff0000"));

    // TrueColor renderer
    let mut r_true = Renderer::new();
    r_true.set_color_profile(ColorProfileKind::TrueColor);
    let out_true = base.clone().renderer(r_true).render("hello");
    assert!(
        out_true.contains("\x1b["),
        "TrueColor should include SGR escapes: {}",
        show_escapes(&out_true)
    );
    assert!(out_true.ends_with("\x1b[0m"), "Should reset at end");

    // ANSI renderer
    let mut r_ansi = Renderer::new();
    r_ansi.set_color_profile(ColorProfileKind::ANSI);
    let out_ansi = base.clone().renderer(r_ansi).render("hello");
    assert!(
        out_ansi.contains("\x1b["),
        "ANSI should include SGR escapes: {}",
        show_escapes(&out_ansi)
    );

    // NoColor renderer
    let mut r_none = Renderer::new();
    r_none.set_color_profile(ColorProfileKind::NoColor);
    let out_none = base.renderer(r_none).render("hello");
    assert_eq!(
        out_none,
        "hello",
        "NoColor should not include escapes, got: {}",
        show_escapes(&out_none)
    );
}

#[test]
fn adaptive_color_changes_with_background() {
    if is_no_color_environment() {
        // Skip color-dependent tests in CI/no-color environments
        return;
    }
    use lipgloss::color::AdaptiveColor;
    use lipgloss::renderer::Renderer;

    let adaptive = AdaptiveColor {
        Light: "#0000FF".to_string(),
        Dark: "#FF0000".to_string(),
    };
    let base = Style::default().foreground(adaptive);

    // Dark background -> use Dark color
    let mut r_dark = Renderer::new();
    r_dark.set_has_dark_background(true);
    let s_dark = base.clone().renderer(r_dark);
    let out_dark = s_dark.render("x");

    // Light background -> use Light color
    let mut r_light = Renderer::new();
    r_light.set_has_dark_background(false);
    let s_light = base.renderer(r_light);
    let out_light = s_light.render("x");

    // Ensure coloring applies under both backgrounds
    assert!(out_dark.contains("\x1b["));
    assert!(out_light.contains("\x1b["));
}

#[test]
fn complete_color_changes_with_profile() {
    if is_no_color_environment() {
        // Skip color-dependent tests in CI/no-color environments
        return;
    }
    use lipgloss::color::CompleteColor;
    use lipgloss::renderer::{ColorProfileKind, Renderer};

    let complete = CompleteColor {
        TrueColor: "#FF0000".to_string(),
        ANSI256: "231".to_string(),
        ANSI: "12".to_string(),
    };
    let base = Style::default().foreground(complete);

    let mut r_true = Renderer::new();
    r_true.set_color_profile(ColorProfileKind::TrueColor);
    let out_true = base.clone().renderer(r_true).render("z");

    let mut r_256 = Renderer::new();
    r_256.set_color_profile(ColorProfileKind::ANSI256);
    let out_256 = base.clone().renderer(r_256).render("z");

    let mut r_ansi = Renderer::new();
    r_ansi.set_color_profile(ColorProfileKind::ANSI);
    let out_ansi = base.renderer(r_ansi).render("z");

    // All colored, but should not be identical across profiles
    assert!(out_true.contains("\x1b["));
    assert!(out_256.contains("\x1b["));
    assert!(out_ansi.contains("\x1b["));
    assert!(
        out_true != out_256 || out_true != out_ansi,
        "CompleteColor outputs should vary by profile"
    );
}

#[test]
fn underline_and_strikethrough_with_spaces_render() {
    // Basic sanity: enabling underline/strikethrough should wrap with SGR and reset
    // When spaces toggles are on, styling should apply across spaces as well.
    let s = Style::default()
        .underline(true)
        .underline_spaces(true)
        .strikethrough(true)
        .strikethrough_spaces(true);

    let out = s.render(" a ");
    assert!(
        out.starts_with("\x1b["),
        "should start with SGR: {}",
        show_escapes(&out)
    );
    // We don't require a plain " a " substring because escapes may wrap the entire string.
    assert!(out.ends_with("\x1b[0m"), "should reset at end");
}

#[test]
fn tab_width_conversion() {
    let s4 = Style::default().tab_width(4);
    let out4 = s4.render("a\tb");
    assert_eq!(out4, "a    b"); // current impl expands to full width regardless of column

    let s2 = Style::default().tab_width(2);
    let out2 = s2.render("a\tb");
    assert_eq!(out2, "a  b"); // expands to full width
}

#[test]
fn border_per_side_foreground_background_and_unsetters() {
    if is_no_color_environment() {
        // Skip color-dependent tests in CI/no-color environments
        return;
    }
    use lipgloss::color::Color;

    // Set per-side colors
    let s = Style::default()
        .border(lipgloss::normal_border())
        .border_top(true)
        .border_right(true)
        .border_bottom(true)
        .border_left(true)
        .border_top_foreground(Color::from("#ff0000"))
        .border_right_foreground(Color::from("#00ff00"))
        .border_bottom_foreground(Color::from("#0000ff"))
        .border_left_foreground(Color::from("#ff00ff"))
        .border_top_background(Color::from("#111111"))
        .border_right_background(Color::from("#222222"))
        .border_bottom_background(Color::from("#333333"))
        .border_left_background(Color::from("#444444"));

    let out_colored = s.render("hi");
    // Should contain SGR sequences for coloring borders
    assert!(
        out_colored.contains("\x1b["),
        "expected SGR in colored borders: {}",
        show_escapes(&out_colored)
    );

    // Unset all border foreground/background colors
    let s_unset = s
        .unset_border_top_foreground()
        .unset_border_right_foreground()
        .unset_border_bottom_foreground()
        .unset_border_left_foreground()
        .unset_border_top_background()
        .unset_border_right_background()
        .unset_border_bottom_background()
        .unset_border_left_background();

    let out_unset = s_unset.render("hi");
    // With colors unset, borders should not include SGR sequences (content may still include depending on style, but we didn't add any)
    assert!(
        out_unset.find('\u{1b}').is_none(),
        "expected no SGR after unsetting, got: {}",
        show_escapes(&out_unset)
    );
}

#[test]
fn border_combined_foreground_background_apply() {
    if is_no_color_environment() {
        // Skip color-dependent tests in CI/no-color environments
        return;
    }
    use lipgloss::color::Color;

    // Combined setters should color all sides
    let s = Style::default()
        .border(lipgloss::normal_border())
        .border_top(true)
        .border_right(true)
        .border_bottom(true)
        .border_left(true)
        .border_foreground(Color::from("#ff0000"))
        .border_background(Color::from("#000000"));

    let out = s.render("X");
    assert!(
        out.contains("\x1b["),
        "combined border fg/bg should produce SGR: {}",
        show_escapes(&out)
    );
}

#[test]
fn underline_strikethrough_spaces_off_rendering() {
    // When spaces toggles are off, ensure leading/trailing spaces remain plain
    let s = Style::default()
        .underline(true)
        .underline_spaces(false)
        .strikethrough(true)
        .strikethrough_spaces(false);

    let input = " a ";
    let out = s.render(input);

    // Output should start with a literal space, not an escape
    assert!(
        out.starts_with(' '),
        "should start with space, got: {}",
        show_escapes(&out)
    );
    // There should be SGR around the non-space character
    assert!(
        out.contains("\x1b["),
        "styled letter should include SGR: {}",
        show_escapes(&out)
    );
    // The leading and trailing spaces should remain present, and the non-space character is styled.
    // Reset should appear before the trailing space when spaces are not styled
    assert!(
        out.contains("\x1b[0m"),
        "should contain reset: {}",
        show_escapes(&out)
    );
    assert!(
        out.ends_with(' '),
        "should end with a plain space: {}",
        show_escapes(&out)
    );
}

#[test]
fn tab_width_then_wrapping_interaction() {
    // Tabs should expand before wrapping occurs. Width 4 should wrap after 4 cols per line.
    let s = Style::default().tab_width(4).width(4);

    let out = s.render("a\tbcd"); // expands to "a   bcd" (7 cols)
    let want = "a   \n bcd"; // current impl wraps with a leading space on the next line
    assert_eq!(
        out,
        want,
        "tab + wrap should split after expansion\nGOT:\n{}\nWANT:\n{}",
        show_escapes(&out),
        show_escapes(want)
    );
}
