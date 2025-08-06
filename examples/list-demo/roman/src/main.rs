use lipgloss::{Color, Style};
use lipgloss_list::{roman, List};

fn main() {
    let enumerator_style = Style::new().foreground(Color::from("99")).margin_right(1);
    let item_style = Style::new().foreground(Color::from("255")).margin_right(1);

    let l = List::new()
        .items(vec!["Glossier", "Claire's Boutique", "Nyx", "Mac", "Milk"])
        .enumerator(roman)
        .enumerator_style(enumerator_style)
        .item_style(item_style);

    println!("{}", l);
}
