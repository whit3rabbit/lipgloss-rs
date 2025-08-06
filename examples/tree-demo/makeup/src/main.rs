use lipgloss::{Color, Style};
use lipgloss_tree::{rounded_enumerator, Leaf, Node, Tree};

fn main() {
    let enumerator_style = Style::new().foreground(Color::from("63")).margin_right(1);
    let root_style = Style::new().foreground(Color::from("35"));
    let item_style = Style::new().foreground(Color::from("212"));

    let t = Tree::new()
        .root("⁜ Makeup")
        .child(vec![
            Box::new(Leaf::new("Glossier", false)) as Box<dyn Node>,
            Box::new(Leaf::new("Fenty Beauty", false)) as Box<dyn Node>,
            Box::new(Tree::new().child(vec![
                Box::new(Leaf::new("Gloss Bomb Universal Lip Luminizer", false)) as Box<dyn Node>,
                Box::new(Leaf::new("Hot Cheeks Velour Blushlighter", false)) as Box<dyn Node>,
            ])) as Box<dyn Node>,
            Box::new(Leaf::new("Nyx", false)) as Box<dyn Node>,
            Box::new(Leaf::new("Mac", false)) as Box<dyn Node>,
            Box::new(Leaf::new("Milk", false)) as Box<dyn Node>,
        ])
        .enumerator(rounded_enumerator)
        .enumerator_style(enumerator_style)
        .root_style(root_style)
        .item_style(item_style);
    println!("{}", t);
}
