use lipgloss::align::{
    align, align_horizontal, align_vertical, HorizontalAlignment as H, VerticalAlignment as V,
};

#[test]
fn align_horizontal_left_plain() {
    let out = align_horizontal(8, H::Left, "abc", &[]);
    assert_eq!(out, "abc     ");
}

#[test]
fn align_horizontal_center_plain_even_remainder_right() {
    // width 9, content width 3 => short 6 => left 3, right 3
    let out = align_horizontal(9, H::Center, "abc", &[]);
    assert_eq!(out, "   abc   ");
}

#[test]
fn align_horizontal_right_styled_visible_width() {
    let s = "\x1b[31mok\x1b[0m"; // visible width 2
    let out = align_horizontal(5, H::Right, s, &[]);
    assert_eq!(out, format!("{}{}", " ".repeat(3), s));
}

#[test]
fn align_vertical_top_basic() {
    let out = align_vertical(4, V::Top, "a\nb", &[]);
    // With Go parity, vertical padding lines are whitespace of max line width
    assert_eq!(out, "a\nb\n \n ");
}

#[test]
fn align_vertical_center_remainder_bottom() {
    // height 5, strHeight 2 => pad 3 => top 1, bottom 2
    let out = align_vertical(5, V::Center, "x\ny", &[]);
    // One whitespace line on top, then content, then two whitespace lines
    assert_eq!(out, " \nx\ny\n \n ");
}

#[test]
fn align_both_center_styled_multiline() {
    let s = "\x1b[32mgo\x1b[0m\nxy"; // lines visible widths: 2 and 2
    let out = align(6, 3, H::Center, V::Center, s, &[]);
    // Horizontal: width 6, line width 2 => short 4 => left 2, right 2
    // Vertical: height 3, strHeight 2 => top 0 or 0? (height-strHeight=1) => top 0, bottom 1
    let expected = format!(
        "{}{}{}\n{}{}{}\n{}",
        " ".repeat(2),
        s.split('\n').next().unwrap(),
        " ".repeat(2),
        " ".repeat(2),
        {
            let mut it = s.split('\n');
            it.next();
            it.next().unwrap()
        },
        " ".repeat(2),
        " ".repeat(6),
    );
    assert_eq!(out, expected);
}
