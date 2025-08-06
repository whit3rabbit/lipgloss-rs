use lipgloss::{Color, Style};
use lipgloss_list::List;
use lipgloss_tree::Children;
use std::fmt;

#[derive(Clone)]
struct Document {
    name: String,
    time: String,
}

impl fmt::Display for Document {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let faint = Style::new().faint(true);
        write!(f, "{}\n{}", self.name, faint.render(&self.time))
    }
}

const SELECTED: usize = 1;

fn custom_enumerator(_items: &dyn Children, i: usize) -> String {
    if i == SELECTED {
        "│\n│".to_string()
    } else {
        " ".to_string()
    }
}

fn item_style_func(_items: &dyn Children, i: usize) -> Style {
    let base_style = Style::new().margin_bottom(1).margin_left(1);
    let dim_color = Color::from("250");
    let highlight_color = Color::from("#EE6FF8");

    if SELECTED == i {
        base_style.foreground(highlight_color)
    } else {
        base_style.foreground(dim_color)
    }
}

fn enumerator_style_func(_items: &dyn Children, i: usize) -> Style {
    let dim_color = Color::from("250");
    let highlight_color = Color::from("#EE6FF8");

    if SELECTED == i {
        Style::new().foreground(highlight_color)
    } else {
        Style::new().foreground(dim_color)
    }
}

fn main() {
    let docs = vec![
        Document {
            name: "README.md".to_string(),
            time: "2 minutes ago".to_string(),
        },
        Document {
            name: "Example.md".to_string(),
            time: "1 hour ago".to_string(),
        },
        Document {
            name: "secrets.md".to_string(),
            time: "1 week ago".to_string(),
        },
    ];

    let mut l = List::new()
        .enumerator(custom_enumerator)
        .item_style_func(item_style_func)
        .enumerator_style_func(enumerator_style_func);

    for d in &docs {
        l = l.item(&d.to_string());
    }

    println!();
    println!("{}", l);
}
