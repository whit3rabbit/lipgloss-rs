use lipgloss::*;

fn render_tiny_table_1x2(b: &Border) -> (String, String, String) {
    // top:  tl + top + mt + top + tr
    // mid:  l  +  ' ' + m  + ' ' + r
    // bot:  bl + bot + mb + bot + br
    let top = format!(
        "{}{}{}{}{}",
        b.top_left, b.top, b.middle_top, b.top, b.top_right
    );
    let mid = format!("{} {} {}", b.left, b.middle, b.right);
    let bot = format!(
        "{}{}{}{}{}",
        b.bottom_left, b.bottom, b.middle_bottom, b.bottom, b.bottom_right
    );
    (top, mid, bot)
}

#[test]
fn snapshot_table_3x3_rounded() {
    let lines = render_grid(&rounded_border(), 3, 3);
    assert_eq!(
        lines,
        vec![
            "╭─┬─┬─╮",
            "│ ┼ ┼ │",
            "├─┼─┼─┤",
            "│ ┼ ┼ │",
            "├─┼─┼─┤",
            "│ ┼ ┼ │",
            "╰─┴─┴─╯",
        ]
    );
}

#[test]
fn snapshot_table_3x3_thick() {
    let lines = render_grid(&thick_border(), 3, 3);
    assert_eq!(
        lines,
        vec![
            "┏━┳━┳━┓",
            "┃ ╋ ╋ ┃",
            "┣━╋━╋━┫",
            "┃ ╋ ╋ ┃",
            "┣━╋━╋━┫",
            "┃ ╋ ╋ ┃",
            "┗━┻━┻━┛",
        ]
    );
}

#[test]
fn snapshot_table_3x3_normal_mixed_width_content() {
    let contents = vec![
        &["中", ".", "字"][..],
        &["A", "汉", "B"][..],
        &["🙂", "C", "国"][..],
    ];
    let lines = render_grid_with(&normal_border(), &contents);
    assert_eq!(
        lines,
        vec![
            "┌─┬─┬─┐",
            "│中┼.┼字│",
            "├─┼─┼─┤",
            "│A┼汉┼B│",
            "├─┼─┼─┤",
            "│🙂┼C┼国│",
            "└─┴─┴─┘",
        ]
    );
}

fn render_grid(b: &Border, rows: usize, cols: usize) -> Vec<String> {
    // Single-line cell height, single-space content
    let mut out = Vec::new();
    // top
    let mut top = String::new();
    top.push_str(b.top_left);
    top.push_str(b.top);
    for _ in 1..cols {
        top.push_str(b.middle_top);
        top.push_str(b.top);
    }
    top.push_str(b.top_right);
    out.push(top);

    // each row
    for r in 0..rows {
        let mut row = String::new();
        row.push_str(b.left);
        row.push(' ');
        for _ in 1..cols {
            row.push_str(b.middle);
            row.push(' ');
        }
        row.push_str(b.right);
        out.push(row);

        if r + 1 < rows {
            let mut mid = String::new();
            mid.push_str(b.middle_left);
            mid.push_str(b.top);
            for _ in 1..cols {
                mid.push_str(b.middle);
                mid.push_str(b.top);
            }
            mid.push_str(b.middle_right);
            out.push(mid);
        }
    }

    // bottom
    let mut bot = String::new();
    bot.push_str(b.bottom_left);
    bot.push_str(b.bottom);
    for _ in 1..cols {
        bot.push_str(b.middle_bottom);
        bot.push_str(b.bottom);
    }
    bot.push_str(b.bottom_right);
    out.push(bot);

    out
}

fn render_grid_with(b: &Border, contents: &[&[&str]]) -> Vec<String> {
    let rows = contents.len();
    let cols = if rows > 0 { contents[0].len() } else { 0 };
    let mut out = Vec::new();

    // top
    let mut top = String::new();
    top.push_str(b.top_left);
    top.push_str(b.top);
    for _ in 1..cols {
        top.push_str(b.middle_top);
        top.push_str(b.top);
    }
    top.push_str(b.top_right);
    out.push(top);

    for (r, row_contents) in contents.iter().enumerate() {
        let mut row = String::new();
        row.push_str(b.left);
        row.push_str(row_contents[0]);
        for c in 1..cols {
            row.push_str(b.middle);
            row.push_str(row_contents[c]);
        }
        row.push_str(b.right);
        out.push(row);

        if r + 1 < rows {
            let mut mid = String::new();
            mid.push_str(b.middle_left);
            mid.push_str(b.top);
            for _ in 1..cols {
                mid.push_str(b.middle);
                mid.push_str(b.top);
            }
            mid.push_str(b.middle_right);
            out.push(mid);
        }
    }

    // bottom
    let mut bot = String::new();
    bot.push_str(b.bottom_left);
    bot.push_str(b.bottom);
    for _ in 1..cols {
        bot.push_str(b.middle_bottom);
        bot.push_str(b.bottom);
    }
    bot.push_str(b.bottom_right);
    out.push(bot);

    out
}

#[test]
fn snapshot_table_3x3_normal() {
    let lines = render_grid(&normal_border(), 3, 3);
    assert_eq!(
        lines,
        vec![
            "┌─┬─┬─┐",
            "│ ┼ ┼ │",
            "├─┼─┼─┤",
            "│ ┼ ┼ │",
            "├─┼─┼─┤",
            "│ ┼ ┼ │",
            "└─┴─┴─┘",
        ]
    );
}

#[test]
fn snapshot_table_2x3_rounded() {
    let lines = render_grid(&rounded_border(), 2, 3);
    assert_eq!(
        lines,
        vec!["╭─┬─┬─╮", "│ ┼ ┼ │", "├─┼─┼─┤", "│ ┼ ┼ │", "╰─┴─┴─╯",]
    );
}

#[test]
fn snapshot_table_2x3_double() {
    let lines = render_grid(&double_border(), 2, 3);
    assert_eq!(
        lines,
        vec!["╔═╦═╦═╗", "║ ╬ ╬ ║", "╠═╬═╬═╣", "║ ╬ ╬ ║", "╚═╩═╩═╝",]
    );
}

#[test]
fn snapshot_table_2x2_normal_mixed_width_content() {
    // Mixed-width content: CJK wide runes next to ASCII
    let contents = vec![&["中", "."][..], &["A", "汉"][..]];
    let lines = render_grid_with(&normal_border(), &contents);
    assert_eq!(lines, vec!["┌─┬─┐", "│中┼.│", "├─┼─┤", "│A┼汉│", "└─┴─┘",]);
}

fn render_table_2x2(b: &Border) -> Vec<String> {
    let top = format!(
        "{}{}{}{}{}",
        b.top_left, b.top, b.middle_top, b.top, b.top_right
    );
    let row = format!("{} {} {}", b.left, b.middle, b.right);
    let mid = format!(
        "{}{}{}{}{}",
        b.middle_left, b.top, b.middle, b.top, b.middle_right
    );
    let bot = format!(
        "{}{}{}{}{}",
        b.bottom_left, b.bottom, b.middle_bottom, b.bottom, b.bottom_right
    );
    vec![top, row.clone(), mid, row, bot]
}

#[test]
fn snapshot_tiny_table_renders_expected_glyphs() {
    let (t, m, b) = render_tiny_table_1x2(&normal_border());
    assert_eq!(t, "┌─┬─┐");
    assert_eq!(m, "│ ┼ │");
    assert_eq!(b, "└─┴─┘");

    let (t, m, b) = render_tiny_table_1x2(&double_border());
    assert_eq!(t, "╔═╦═╗");
    assert_eq!(m, "║ ╬ ║");
    assert_eq!(b, "╚═╩═╝");

    let (t, m, b) = render_tiny_table_1x2(&ascii_border());
    assert_eq!(t, "+-+-+");
    assert_eq!(m, "| + |");
    assert_eq!(b, "+-+-+");

    let (t, m, b) = render_tiny_table_1x2(&markdown_border());
    assert_eq!(t, "|-|-|");
    assert_eq!(m, "| | |");
    assert_eq!(b, "|-|-|");
}

#[test]
fn snapshot_table_2x2_renders_expected_glyphs() {
    // Normal
    let lines = render_table_2x2(&normal_border());
    assert_eq!(lines, vec!["┌─┬─┐", "│ ┼ │", "├─┼─┤", "│ ┼ │", "└─┴─┘",]);

    // Double
    let lines = render_table_2x2(&double_border());
    assert_eq!(lines, vec!["╔═╦═╗", "║ ╬ ║", "╠═╬═╣", "║ ╬ ║", "╚═╩═╝",]);

    // ASCII
    let lines = render_table_2x2(&ascii_border());
    assert_eq!(lines, vec!["+-+-+", "| + |", "+-+-+", "| + |", "+-+-+",]);

    // Markdown
    let lines = render_table_2x2(&markdown_border());
    assert_eq!(lines, vec!["|-|-|", "| | |", "|-|-|", "| | |", "|-|-|",]);

    // Rounded
    let lines = render_table_2x2(&rounded_border());
    assert_eq!(lines, vec!["╭─┬─╮", "│ ┼ │", "├─┼─┤", "│ ┼ │", "╰─┴─╯",]);

    // Thick
    let lines = render_table_2x2(&thick_border());
    assert_eq!(lines, vec!["┏━┳━┓", "┃ ╋ ┃", "┣━╋━┫", "┃ ╋ ┃", "┗━┻━┛",]);

    // Block
    let lines = render_table_2x2(&block_border());
    assert_eq!(lines, vec!["█████", "█ █ █", "█████", "█ █ █", "█████",]);
}

#[test]
fn snapshot_table_2x2_renders_expected_glyphs_half_blocks() {
    // Outer half-block border
    let lines = render_table_2x2(&outer_half_block_border());
    assert_eq!(lines, vec!["▛▀▀▜", "▌  ▐", "▀▀", "▌  ▐", "▙▄▄▟",]);

    // Inner half-block border
    let lines = render_table_2x2(&inner_half_block_border());
    assert_eq!(lines, vec!["▗▄▄▖", "▐  ▌", "▄▄", "▐  ▌", "▝▀▀▘",]);
}
