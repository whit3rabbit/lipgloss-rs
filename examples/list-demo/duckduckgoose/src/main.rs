use lipgloss::{Color, Style};
use lipgloss_list::List;
use lipgloss_tree::Children;

fn duck_duck_goose_enumerator(items: &dyn Children, i: usize) -> String {
    if let Some(item) = items.at(i) {
        if item.value() == "Goose" {
            return "Honk →".to_string();
        }
    }
    " ".to_string()
}

fn main() {
    let enum_style = Style::new()
        .foreground(Color::from("#00d787"))
        .margin_right(1);
    let item_style = Style::new().foreground(Color::from("255"));

    let l = List::new()
        .items(vec!["Duck", "Duck", "Duck", "Goose", "Duck"])
        .item_style(item_style)
        .enumerator_style(enum_style)
        .enumerator(duck_duck_goose_enumerator);

    println!("{}", l);
}
