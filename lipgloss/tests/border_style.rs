use lipgloss::{
    border::{normal_border, rounded_border},
    style::Style,
};

#[test]
fn render_normal_border_single_line() {
    let s = Style::default().border(normal_border());
    let out = s.render("hi");
    let expected = format!(
        "{}{}{}\n{}hi{}\n{}{}{}",
        "┌",
        "─".repeat(2),
        "┐",
        "│",
        "│",
        "└",
        "─".repeat(2),
        "┘",
    );
    assert_eq!(out, expected);
}

#[test]
fn render_rounded_border_single_line() {
    let s = Style::default().border(rounded_border());
    let out = s.render("ok");
    let expected = format!(
        "{}{}{}\n{}ok{}\n{}{}{}",
        "╭",
        "─".repeat(2),
        "╮",
        "│",
        "│",
        "╰",
        "─".repeat(2),
        "╯",
    );
    assert_eq!(out, expected);
}
