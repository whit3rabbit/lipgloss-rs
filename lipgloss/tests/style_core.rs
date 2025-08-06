use lipgloss::{position, style::Style};

#[test]
fn inline_removes_newlines() {
    let s = Style::default().inline(true);
    let out = s.render("a\nb\n\nc");
    assert_eq!(out, "abc");
}

#[test]
fn tab_width_default_replaces_with_4_spaces() {
    // Default tab width is 4
    let out = Style::default().render("a\tb");
    assert_eq!(out, format!("a{}b", " ".repeat(4)));
}

#[test]
fn tab_width_zero_removes_tabs() {
    let out = Style::default().tab_width(0).render("a\tb\t");
    assert_eq!(out, "ab");
}

#[test]
fn tab_width_two_replaces_with_2_spaces() {
    let out = Style::default().tab_width(2).render("x\ty");
    assert_eq!(out, "x  y");
}

#[test]
fn tab_width_minus_one_keeps_tabs() {
    let out = Style::default().tab_width(-1).render("x\ty");
    assert_eq!(out, "x\ty");
}

#[test]
fn max_height_truncates_lines() {
    let s = Style::default().max_height(1);
    let out = s.render("one\ntwo\nthree");
    assert_eq!(out, "one");
}

#[test]
fn max_width_truncates_visible_width_with_ansi_preserved() {
    // "\x1b[31mb\x1b[0m" has visible width 1 but contains ESC sequences
    let input = "a\x1b[31mb\x1b[0mc";
    let out = Style::default().max_width(2).render(input);
    // Expect first two visible chars: "a" and colored "b"; reset should be preserved; "c" dropped
    let expected = "a\x1b[31mb\x1b[0m";
    assert_eq!(out, expected);
}

#[test]
fn align_horizontal_only_effective_with_width_or_multiline() {
    let base = Style::default().align_horizontal(position::RIGHT);

    // No width and single line: no effect
    assert_eq!(base.render("ab"), "ab");

    // With width: aligns right within 4
    let out = base.clone().width(4).render("ab");
    assert_eq!(out, "  ab");

    // Multiline without width: alignment applies via horizontal placement
    let out2 = base.render("ab\ncd");
    // Since lines are equal width, right-align is effectively a no-op
    assert_eq!(out2, "ab\ncd");
}

#[test]
fn padding_is_applied_around_content_per_line() {
    let s = Style::default().padding_left(2).padding_right(1);
    let out = s.render("x");
    assert_eq!(out, "  x ");

    let out_multi = s.render("a\nb");
    assert_eq!(out_multi, "  a \n  b ");
}

#[test]
fn unset_boolean_attrs_clear_getters() {
    let s = Style::default()
        .bold(true)
        .italic(true)
        .underline(true)
        .strikethrough(true)
        .reverse(true)
        .blink(true)
        .faint(true)
        .unset_bold()
        .unset_italic()
        .unset_underline()
        .unset_strikethrough()
        .unset_reverse()
        .unset_blink()
        .unset_faint();

    assert!(!s.get_bold());
    assert!(!s.get_italic());
    assert!(!s.get_underline());
    assert!(!s.get_strikethrough());
    assert!(!s.get_reverse());
    assert!(!s.get_blink());
    assert!(!s.get_faint());
}
