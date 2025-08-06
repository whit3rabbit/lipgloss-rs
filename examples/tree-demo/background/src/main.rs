use lipgloss::{Color, Style};
use lipgloss_tree::{Leaf, Node, Tree};

fn main() {
    let enumerator_style = Style::new()
        .background(Color::from("0"))
        .padding(0, 1, 0, 1);
    let header_item_style = Style::new()
        .background(Color::from("#ee6ff8"))
        .foreground(Color::from("#ecfe65"))
        .bold(true)
        .padding(0, 1, 0, 1);
    let item_style = header_item_style.clone().background(Color::from("0"));

    let t = Tree::new()
        .root("# Table of Contents")
        .root_style(item_style.clone())
        .item_style(item_style)
        .enumerator_style(enumerator_style)
        .child(vec![
            Box::new(Tree::new().root("## Chapter 1").child(vec![
                Box::new(Leaf::new("Chapter 1.1", false)) as Box<dyn Node>,
                Box::new(Leaf::new("Chapter 1.2", false)) as Box<dyn Node>,
            ])) as Box<dyn Node>,
            Box::new(Tree::new().root("## Chapter 2").child(vec![
                Box::new(Leaf::new("Chapter 2.1", false)) as Box<dyn Node>,
                Box::new(Leaf::new("Chapter 2.2", false)) as Box<dyn Node>,
            ])) as Box<dyn Node>,
        ]);

    println!("{}", t);
}
