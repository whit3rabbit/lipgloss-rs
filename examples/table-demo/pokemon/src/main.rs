use std::collections::HashMap;
use lipgloss::{normal_border, style::Style, Color};
use lipgloss_table::{Table, HEADER_ROW};

fn main() {
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
        vec!["1", "Bulbasaur", "Grass", "Poison", "フシギダネ", "Fushigidane"],
        vec!["2", "Ivysaur", "Grass", "Poison", "フシギソウ", "Fushigisou"],
        vec!["25", "Pikachu", "Electric", "", "ピカチュウ", "Pikachu"],
    ];

    let capitalize_headers = |h: Vec<&str>| h.iter().map(|s| s.to_uppercase()).collect::<Vec<_>>();

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
            "#", "Name", "Type 1", "Type 2", "Japanese", "Official Rom.",
        ]))
        .width(80)
        .rows(data)
        .style_func_boxed(style_func);

    println!("{}", t);
}
