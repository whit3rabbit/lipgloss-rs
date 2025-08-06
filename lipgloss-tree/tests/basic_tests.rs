use lipgloss_tree::{rounded_enumerator, Leaf, Tree};

#[test]
fn test_basic_tree() {
    let mut tree = Tree::new().root("Root");
    tree = tree.add_child(Box::new(Leaf::new("Child 1", false)) as Box<dyn lipgloss_tree::Node>);
    tree = tree.add_child(Box::new(Leaf::new("Child 2", false)) as Box<dyn lipgloss_tree::Node>);
    tree = tree.add_child(Box::new(Leaf::new("Child 3", false)) as Box<dyn lipgloss_tree::Node>);

    let output = format!("{}", tree);
    assert!(output.contains("Root"));
    assert!(output.contains("├── Child 1"));
    assert!(output.contains("├── Child 2"));
    assert!(output.contains("└── Child 3"));
}

#[test]
fn test_rounded_tree() {
    let mut tree = Tree::new().root("Files").enumerator(rounded_enumerator);
    tree = tree.add_child(Box::new(Leaf::new("file1.txt", false)) as Box<dyn lipgloss_tree::Node>);
    tree = tree.add_child(Box::new(Leaf::new("file2.txt", false)) as Box<dyn lipgloss_tree::Node>);
    tree = tree.add_child(Box::new(Leaf::new("file3.txt", false)) as Box<dyn lipgloss_tree::Node>);

    let output = format!("{}", tree);
    assert!(output.contains("Files"));
    assert!(output.contains("├── file1.txt"));
    assert!(output.contains("├── file2.txt"));
    assert!(output.contains("╰── file3.txt")); // Rounded last item
}

#[test]
fn test_tree_without_root() {
    let mut tree = Tree::new();
    tree = tree.add_child(Box::new(Leaf::new("Item 1", false)) as Box<dyn lipgloss_tree::Node>);
    tree = tree.add_child(Box::new(Leaf::new("Item 2", false)) as Box<dyn lipgloss_tree::Node>);

    let output = format!("{}", tree);
    assert!(!output.contains("Root")); // No root value
    assert!(output.contains("├── Item 1"));
    assert!(output.contains("└── Item 2"));
}

#[test]
fn test_hidden_nodes() {
    let mut tree = Tree::new().root("Visible Root");
    tree = tree.add_child(Box::new(Leaf::new("Visible", false)) as Box<dyn lipgloss_tree::Node>);
    tree = tree.add_child(Box::new(Leaf::new("Hidden", true)) as Box<dyn lipgloss_tree::Node>); // Hidden
    tree =
        tree.add_child(Box::new(Leaf::new("Also Visible", false)) as Box<dyn lipgloss_tree::Node>);

    let output = format!("{}", tree);
    assert!(output.contains("Visible Root"));
    assert!(output.contains("├── Visible"));
    assert!(!output.contains("Hidden")); // Hidden node should not appear
    assert!(output.contains("└── Also Visible"));
}

#[test]
fn test_tree_offset() {
    let mut tree = Tree::new().root("Root");
    for i in 1..=5 {
        tree = tree.add_child(
            Box::new(Leaf::new(format!("Child {}", i), false)) as Box<dyn lipgloss_tree::Node>
        );
    }
    tree = tree.offset(1, 1); // Skip first and last child

    let output = format!("{}", tree);
    assert!(output.contains("Root"));
    assert!(!output.contains("Child 1")); // Skipped by offset
    assert!(output.contains("├── Child 2"));
    assert!(output.contains("├── Child 3"));
    assert!(output.contains("└── Child 4"));
    assert!(!output.contains("Child 5")); // Skipped by offset
}
