use lipgloss::{style::Style, Color};
use lipgloss_table::Table;

fn main() {
    let s = Style::new().foreground(Color::from("240"));
    let mut t = Table::new();
    t = t.row(vec!["Bubble Tea", &s.render("Milky")]);
    t = t.row(vec!["Milk Tea", &s.render("Also milky")]);
    t = t.row(vec!["Actual milk", &s.render("Milky as well")]);
    println!("{}", t);
}
