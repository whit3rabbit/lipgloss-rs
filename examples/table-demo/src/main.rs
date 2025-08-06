use lipgloss::{join_horizontal, join_vertical, normal_border, style::Style, Color, CENTER, RIGHT};
use lipgloss_table::{Table, HEADER_ROW};
use std::collections::HashMap;

fn main() {
    println!("--- ANSI Table ---");
    ansi_table();

    println!("\n--- Chess Board ---");
    chess_board();

    println!("\n--- Color Palette ---");
    mindy_palette();

    println!("\n--- Pokémon Table ---");
    pokemon_table();
}

fn ansi_table() {
    let s = Style::new().foreground(Color::from("240"));
    let mut t = Table::new();
    t = t.row(vec!["Bubble Tea", &s.render("Milky")]);
    t = t.row(vec!["Milk Tea", &s.render("Also milky")]);
    t = t.row(vec!["Actual milk", &s.render("Milky as well")]);
    println!("{}", t);
}

fn chess_board() {
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

    let ranks = label_style.render(&vec![" A", "B", "C", "D", "E", "F", "G", "H  "].join("   "));
    let files = label_style.render(&vec![" 1", "2", "3", "4", "5", "6", "7", "8 "].join("\n\n "));

    let table_render = t.render();
    let board_with_files = join_horizontal(CENTER, &[&files, &table_render]);
    println!("{}\n", join_vertical(RIGHT, &[&board_with_files, &ranks]));
}

fn mindy_palette() {
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
        if col % 2 == 0 {
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
            row.push("".to_string());
        }
    }
    while row.len() < ROW_LENGTH {
        row.push("".to_string());
    }
    row
}

fn make_empty_row() -> Vec<String> {
    make_row(0, 0)[..0].to_vec()
}

fn pokemon_table() {
    let base_style = Style::new().padding(0, 1, 0, 1);
    let header_style = base_style.clone().foreground(Color::from("252")).bold(true);
    let selected_style = base_style
        .clone()
        .foreground(Color::from("#01BE85"))
        .background(Color::from("#00432F"));

    let mut type_colors = HashMap::new();
    type_colors.insert("Bug", Color::from("#D7FF87"));
    type_colors.insert("Electric", Color::from("#FDFF90"));
    type_colors.insert("Fire", Color::from("#FF7698"));
    type_colors.insert("Flying", Color::from("#FF87D7"));
    type_colors.insert("Grass", Color::from("#75FBAB"));
    type_colors.insert("Ground", Color::from("#FF875F"));
    type_colors.insert("Normal", Color::from("#929292"));
    type_colors.insert("Poison", Color::from("#7D5AFC"));
    type_colors.insert("Water", Color::from("#00E2C7"));

    let data = vec![
        vec![
            "1",
            "Bulbasaur",
            "Grass",
            "Poison",
            "フシギダネ",
            "Fushigidane",
        ],
        vec![
            "2",
            "Ivysaur",
            "Grass",
            "Poison",
            "フシギソウ",
            "Fushigisou",
        ],
        vec!["25", "Pikachu", "Electric", "", "ピカチュウ", "Pikachu"],
    ];

    let capitalize_headers = |h: Vec<&str>| h.iter().map(|s| s.to_uppercase()).collect::<Vec<_>>();

    // Clone data for use in style_func
    let data_clone = data.clone();
    let style_func = move |row: i32, col: usize| {
        if row == HEADER_ROW {
            return header_style.clone();
        }
        if data_clone[row as usize][1] == "Pikachu" {
            return selected_style.clone();
        }
        if col == 2 || col == 3 {
            if let Some(color) = type_colors.get(data_clone[row as usize][col]) {
                return base_style.clone().foreground(color.clone());
            }
        }
        base_style.clone()
    };

    let t = Table::new()
        .border(normal_border())
        .border_style(Style::new().foreground(Color::from("238")))
        .headers(capitalize_headers(vec![
            "#",
            "Name",
            "Type 1",
            "Type 2",
            "Japanese",
            "Official Rom.",
        ]))
        .width(80)
        .rows(data)
        .style_func_boxed(style_func);

    println!("{}", t);
}
