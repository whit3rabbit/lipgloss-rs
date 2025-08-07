use lipgloss::{style::Style, thick_border, Color, CENTER};
use lipgloss_table::{Table, HEADER_ROW};

fn main() {
    // Data with long strings and different scripts to test wrapping and alignment
    let data: Vec<Vec<&str>> = vec![
        vec!["Chinese", "您好", "你好"],
        vec!["Japanese", "こんにちは", "やあ"],
        vec!["Arabic", "أهلين", "أهلا"],
        vec!["Russian", "Здравствуйте", "Привет"],
        vec!["Spanish", "Hola", "¿Qué tal?"],
        vec![
            "English",
            "You look absolutely fabulous.",
            "How's it going?",
        ],
    ];

    let base = Style::new().padding(0, 1, 0, 1);
    let header = base.clone().bold(true).align_horizontal(CENTER);
    let formal = base.clone();
    let informal = base.clone();

    // Style function to apply per-cell styles
    let style_func = move |row: i32, col: usize| -> Style {
        match row {
            r if r == HEADER_ROW => header.clone(),
            _ => match col {
                0 => base.clone(),
                1 => formal.clone(),
                2 => informal.clone(),
                _ => base.clone(),
            },
        }
    };

    let t = Table::new()
        .border(thick_border())
        .border_style(Style::new().foreground(Color::from("238")))
        .wrap(true)
        .headers(vec!["LANGUAGE", "FORMAL", "INFORMAL"])
        .width(46)
        .rows(data)
        .style_func_boxed(style_func);

    println!("{}", t);
}
