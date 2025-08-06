use lipgloss::{join_horizontal, join_vertical, normal_border, style::Style, Color, CENTER, RIGHT};
use lipgloss_table::Table;

fn main() {
    let label_style = Style::new().foreground(Color::from("241"));

    let board = vec![
        vec!["♜", "♞", "♝", "♛", "♚", "♝", "♞", "♜"],
        vec!["♟", "♟", "♟", "♟", "♟", "♟", "♟", "♟"],
        vec![" ", " ", " ", " ", " ", " ", " ", " "],
        vec![" ", " ", " ", " ", " ", " ", " ", " "],
        vec![" ", " ", " ", " ", " ", " ", " ", " "],
        vec![" ", " ", " ", " ", " ", " ", " ", " "],
        vec!["♙", "♙", "♙", "♙", "♙", "♙", "♙", "♙"],
        vec!["♖", "♘", "♗", "♕", "♔", "♗", "♘", "♖"],
    ];

    let style_func = move |_row: i32, _col: usize| -> Style { Style::new().padding(0, 1, 0, 1) };

    let mut t = Table::new()
        .border(normal_border())
        .border_row(true)
        .border_column(true)
        .rows(board)
        .style_func_boxed(style_func);

    let ranks = label_style
        .render(&[" A", "B", "C", "D", "E", "F", "G", "H  "].join("   "));
    let files = label_style.render(&[" 1", "2", "3", "4", "5", "6", "7", "8 "].join("\n\n "));

    let table_render = t.render();
    let board_with_files = join_horizontal(CENTER, &[&files, &table_render]);
    println!("{}\n", join_vertical(RIGHT, &[&board_with_files, &ranks]));
}
