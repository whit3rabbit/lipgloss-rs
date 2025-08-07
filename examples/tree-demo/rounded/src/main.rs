use lipgloss::{Color, Style};
use lipgloss_tree::{rounded_enumerator, Leaf, Node, Tree};

fn main() {
    let item_style = Style::new().margin_right(1);
    let enumerator_style = Style::new().foreground(Color::from("8")).margin_right(1);

    let t = Tree::new()
        .root("Groceries")
        .child(vec![
            Box::new(Tree::new().root("Fruits").child(vec![
                Box::new(Leaf::new("Blood Orange", false)) as Box<dyn Node>,
                Box::new(Leaf::new("Papaya", false)) as Box<dyn Node>,
                Box::new(Leaf::new("Dragonfruit", false)) as Box<dyn Node>,
                Box::new(Leaf::new("Yuzu", false)) as Box<dyn Node>,
            ])) as Box<dyn Node>,
            Box::new(Tree::new().root("Items").child(vec![
                Box::new(Leaf::new("Cat Food", false)) as Box<dyn Node>,
                Box::new(Leaf::new("Nutella", false)) as Box<dyn Node>,
                Box::new(Leaf::new("Powdered Sugar", false)) as Box<dyn Node>,
            ])) as Box<dyn Node>,
            Box::new(Tree::new().root("Veggies").child(vec![
                Box::new(Leaf::new("Leek", false)) as Box<dyn Node>,
                Box::new(Leaf::new("Artichoke", false)) as Box<dyn Node>,
            ])) as Box<dyn Node>,
        ])
        .item_style(item_style)
        .enumerator_style(enumerator_style)
        .enumerator(rounded_enumerator);

    println!("{}", t);
}
