use lipgloss::color::{Color, TerminalColor};
use lipgloss::renderer::{ColorProfileKind, Renderer};

#[test]
fn numeric_string_passthrough_for_ansi_profiles() {
    // Numeric ANSI index should pass through for ANSI profile
    let c = Color("9".to_string());
    let mut r = Renderer::new();
    r.set_color_profile(ColorProfileKind::ANSI);
    assert_eq!(c.token(&r), "9");

    // Numeric ANSI256 index should pass through for ANSI256 profile (clamped)
    let c = Color("300".to_string());
    r.set_color_profile(ColorProfileKind::ANSI256);
    // 300 % 256 = 44
    assert_eq!(c.token(&r), "44");
}

#[test]
fn numeric_string_converts_to_hex_for_truecolor() {
    // For TrueColor, numeric strings are interpreted as ANSI256 and converted to hex
    let c = Color("196".to_string()); // red
    let mut r = Renderer::new();
    r.set_color_profile(ColorProfileKind::TrueColor);
    let token = c.token(&r);
    assert_eq!(token, "#ff0000");
}

#[test]
fn hex_converts_to_indices_for_limited_profiles() {
    // Common primaries map to well-known indices
    let c = Color("#ff0000".to_string());
    let mut r = Renderer::new();

    r.set_color_profile(ColorProfileKind::ANSI256);
    assert_eq!(c.token(&r), "196");

    r.set_color_profile(ColorProfileKind::ANSI);
    assert_eq!(c.token(&r), "9");
}
