use lipgloss_tree::{Leaf, Node, Tree};

fn main() {
    let t = Tree::new().root(".").child(vec![
        Box::new(Leaf::new("macOS", false)) as Box<dyn Node>,
        Box::new(Tree::new().root("Linux").child(vec![
            Box::new(Leaf::new("NixOS", false)) as Box<dyn Node>,
            Box::new(Leaf::new("Arch Linux (btw)", false)) as Box<dyn Node>,
            Box::new(Leaf::new("Void Linux", false)) as Box<dyn Node>,
        ])) as Box<dyn Node>,
        Box::new(Tree::new().root("BSD").child(vec![
            Box::new(Leaf::new("FreeBSD", false)) as Box<dyn Node>,
            Box::new(Leaf::new("OpenBSD", false)) as Box<dyn Node>,
        ])) as Box<dyn Node>,
    ]);

    println!("{}", t);
}
