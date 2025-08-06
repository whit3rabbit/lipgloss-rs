use lipgloss::{border::normal_border, position, style::Style};

#[test]
fn place_horizontal_ansi_visible_width_left() {
    let s = "\x1b[31mred\x1b[0m"; // visible width 3
    let out = position::place_horizontal(10, position::LEFT, s, &[]);
    let expected = format!("{}{}", s, " ".repeat(7));
    assert_eq!(out, expected);
}

#[test]
fn style_border_with_ansi_content_uses_visible_width() {
    let content = "\x1b[32mgo\x1b[0m"; // visible width 2
    let styled = Style::default().border(normal_border());
    let out = styled.render(content);
    let expected = format!(
        "{}{}{}\n{}{}{}\n{}{}{}",
        "┌",
        "─".repeat(2),
        "┐",
        "│",
        content,
        "│",
        "└",
        "─".repeat(2),
        "┘",
    );
    assert_eq!(out, expected);
}

#[test]
fn join_vertical_right_alignment_with_ansi() {
    let a = "\x1b[31mab\x1b[0m"; // visible width 2
    let b = "wxyz"; // width 4
    let out = lipgloss::join::join_vertical(position::RIGHT, &[a, b]);
    let expected = format!("{}{}\n{}", " ".repeat(2), a, b);
    assert_eq!(out, expected);
}
