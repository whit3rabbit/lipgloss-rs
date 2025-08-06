use lipgloss::{
    join,
    position::Position,
    position::{BOTTOM, CENTER, LEFT, RIGHT, TOP},
};

#[test]
fn join_horizontal_top_bottom_center_heights() {
    let a = "A\nA\nA\nA\nA"; // height 5, width 1
    let b = "b1\nb2"; // height 2, width 2

    // Top
    let out_top = join::join_horizontal(TOP, &[a, b]);
    let expected_top =
        "A".to_string() + "b1\n" + "A" + "b2\n" + "A" + "  \n" + "A" + "  \n" + "A" + "  ";
    assert_eq!(out_top, expected_top);

    // Bottom
    let out_bottom = join::join_horizontal(BOTTOM, &[a, b]);
    let expected_bottom =
        "A".to_string() + "  \n" + "A" + "  \n" + "A" + "  \n" + "A" + "b1\n" + "A" + "b2";
    assert_eq!(out_bottom, expected_bottom);

    // Center (pos=0.5): n=3 -> split=round(3*0.5)=2, top=1, bottom=2
    let out_center = join::join_horizontal(CENTER, &[a, b]);
    // Go algorithm prepends extraLines[top:] (2 lines) and appends extraLines[bottom:] (1 line)
    let expected_center =
        "A".to_string() + "  \n" + "A" + "  \n" + "A" + "b1\n" + "A" + "b2\n" + "A" + "  ";
    assert_eq!(out_center, expected_center);
}

#[test]
fn join_vertical_fractional_positions() {
    let a = "x\ny\nz"; // width 1, height 3
    let b = "ww"; // width 2, height 1

    // pos=0.2 => w=max(visible widths) = 2; for lines from `a`, w=1 so pad 1:
    let out_left = join::join_vertical(Position(0.2), &[a, b]);
    let expected_left = "x \ny \nz \nww";
    assert_eq!(out_left, expected_left);

    // pos=0.8 => still width padding of 1 goes to left side
    let out_rightish = join::join_vertical(Position(0.8), &[a, b]);
    let expected_rightish = " x\n y\n z\nww";
    assert_eq!(out_rightish, expected_rightish);
}

#[test]
fn join_horizontal_multi_blocks_cjk_emoji_and_ansi() {
    // CJK chars (full width) and emoji; ANSI-styled segment
    let cjk = "漢字\n仮名"; // widths 2 per line
    let emoji = "😀\n🎉"; // many terminals treat as width 2
    let ansi = "\x1b[34mblue\x1b[0m\n\x1b[32mgrn\x1b[0m"; // widths 4 and 3

    // We don’t assert full layout string (can vary by terminal width rules for emoji),
    // but we verify length equality per merged line against computed max widths.
    let out = join::join_horizontal(CENTER, &[cjk, emoji, ansi]);
    let lines: Vec<&str> = out.split('\n').collect();
    assert_eq!(lines.len(), 2);

    // Visible widths
    fn w(s: &str) -> usize {
        lipgloss::utils::width_visible(s)
    }
    let cjk_lines: Vec<&str> = cjk.split('\n').collect();
    let emoji_lines: Vec<&str> = emoji.split('\n').collect();
    let ansi_lines: Vec<&str> = ansi.split('\n').collect();

    let cjk_max = cjk_lines.iter().map(|l| w(l)).max().unwrap();
    let emoji_max = emoji_lines.iter().map(|l| w(l)).max().unwrap();
    let ansi_max = ansi_lines.iter().map(|l| w(l)).max().unwrap();

    // For each merged line, its visible width should equal sum of block max widths
    let total = cjk_max + emoji_max + ansi_max;
    for ln in lines {
        assert_eq!(w(ln), total);
    }
}

#[test]
fn join_horizontal_fractional_position() {
    let a = "A\nA\nA\nA\nA"; // height 5
    let b = "b1\nb2"; // height 2
                      // pos=0.2: n=3, split=round(3*0.2)=1, top=2, bottom=1
    let pos = Position(0.2);
    let out = join::join_horizontal(pos, &[a, b]);
    let expected =
        "A".to_string() + "  \n" + "A" + "b1\n" + "A" + "b2\n" + "A" + "  \n" + "A" + "  ";
    assert_eq!(out, expected);
}

#[test]
fn join_vertical_left_right_and_center_with_ansi() {
    let a = "X\nY"; // widths 1
    let b = "\x1b[31mab\x1b[0m"; // visible width 2

    // Left
    let out_left = join::join_vertical(LEFT, &[a, b]);
    let expected_left = format!("X \nY \n{}", b);
    assert_eq!(out_left, expected_left);

    // Right
    let out_right = join::join_vertical(RIGHT, &[a, b]);
    let expected_right = format!(" X\n Y\n{}", b);
    assert_eq!(out_right, expected_right);

    // Center with fractional pos=0.3 on width w=2-1=1 -> left 0, right 1
    let pos = Position(0.3);
    let out_center = join::join_vertical(pos, &[a, b]);
    let expected_center = format!("X \nY \n{}", b);
    assert_eq!(out_center, expected_center);
}
