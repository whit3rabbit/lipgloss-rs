use lipgloss::{Color, Style};
use lipgloss_tree::{Leaf, Node, Tree};

fn main() {
    let purple = Style::new().foreground(Color::from("99")).margin_right(1);
    let pink = Style::new().foreground(Color::from("212")).margin_right(1);

    let t = Tree::new()
        .child(vec![
            Box::new(Leaf::new("Glossier", false)) as Box<dyn Node>,
            Box::new(Leaf::new("Claire’s Boutique", false)) as Box<dyn Node>,
            Box::new(
                Tree::new()
                    .root("Nyx")
                    .child(vec![
                        Box::new(Leaf::new("Lip Gloss", false)) as Box<dyn Node>,
                        Box::new(Leaf::new("Foundation", false)) as Box<dyn Node>,
                    ])
                    .enumerator_style(pink),
            ) as Box<dyn Node>,
            Box::new(Leaf::new("Mac", false)) as Box<dyn Node>,
            Box::new(Leaf::new("Milk", false)) as Box<dyn Node>,
        ])
        .enumerator_style(purple);

    println!("{}", t);
}
