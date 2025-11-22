use lipgloss::{style::Style, Color, RIGHT};
use lipgloss_table::Table;

fn main() {
    let label_style = Style::new().width(3).align_horizontal(RIGHT);
    let swatch_style = Style::new().width(6);

    let mut data: Vec<Vec<String>> = Vec::new();
    for i in (0..13).step_by(8) {
        data.push(make_row(i, i + 5));
    }
    data.push(make_empty_row());
    for i in (6..15).step_by(8) {
        data.push(make_row(i, i + 1));
    }
    data.push(make_empty_row());
    for i in (16..231).step_by(6) {
        data.push(make_row(i, i + 5));
    }
    data.push(make_empty_row());
    for i in (232..256).step_by(6) {
        data.push(make_row(i, i + 5));
    }

    let data_clone = data.clone();
    let style_func = move |row: i32, col: usize| -> Style {
        let color = Color::from(data_clone[row as usize][col - col % 2].as_str());
        if col.is_multiple_of(2) {
            label_style.clone().foreground(color)
        } else {
            swatch_style.clone().background(color)
        }
    };

    let t = Table::new()
        .border(lipgloss::hidden_border())
        .rows(data)
        .style_func_boxed(style_func);

    println!("{}", t);
}

fn make_row(start: usize, end: usize) -> Vec<String> {
    const ROW_LENGTH: usize = 12;
    let mut row = Vec::new();
    if start <= end {
        for i in start..=end {
            row.push(i.to_string());
            row.push(String::new());
        }
    }
    while row.len() < ROW_LENGTH {
        row.push(String::new());
    }
    row
}

fn make_empty_row() -> Vec<String> {
    make_row(0, 0)[..0].to_vec()
}
