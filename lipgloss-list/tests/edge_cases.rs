use lipgloss_list::{alphabet, arabic, List};
use lipgloss_tree::{Leaf, Node, Tree};

// Hidden nodes between visible siblings should not increment the parent list's
// enumeration index.
#[test]
fn hidden_nodes_do_not_increment_enumeration() {
    let mut l = List::new().enumerator(alphabet).item("first");

    // Insert a hidden node between first and second
    let hidden_leaf: Box<dyn Node> = Box::new(Leaf::new("should not show", true));
    l = l.item_node(hidden_leaf);

    // Now add the second visible sibling
    l = l.item("second");

    let out = format!("{}", l);
    let lines: Vec<&str> = out.lines().collect();

    // We expect exactly A. first and B. second
    assert!(
        lines.iter().any(|s| s.trim_start().starts_with("A. first")),
        "missing 'A. first' in output\n{}",
        out
    );
    assert!(
        lines
            .iter()
            .any(|s| s.trim_start().starts_with("B. second")),
        "missing 'B. second' in output\n{}",
        out
    );
    // Ensure there's no 'C.'
    assert!(
        lines.iter().all(|s| !s.trim_start().starts_with("C.")),
        "unexpected third enumerated item found\n{}",
        out
    );
}

// Multiple consecutive container (empty-value) nodes with nested content should not
// advance the enumeration; their nested content should attach under the previous item.
#[test]
fn consecutive_container_nodes_preserve_enumeration_and_indent() {
    let mut top = List::new().enumerator(alphabet).items(vec!["one", "two"]);

    // Create two container nodes (empty-root trees) and attach nested lists under them
    let nested1 = List::new().enumerator(arabic).items(vec!["a", "b"]);
    let nested2 = List::new().enumerator(arabic).items(vec!["c", "d"]);

    let container1: Box<dyn Node> = Box::new(Tree::new());
    let container2: Box<dyn Node> = Box::new(Tree::new());

    top = top.item_node(container1).item_list(nested1);
    top = top.item_node(container2).item_list(nested2);
    top = top.item("three");

    let out = format!("{}", top);
    let lines: Vec<&str> = out.lines().collect();

    // A. one, B. two, C. three should exist
    assert!(
        lines.iter().any(|s| s.trim_start().starts_with("A. one")),
        "missing 'A. one'\n{}",
        out
    );
    assert!(
        lines.iter().any(|s| s.trim_start().starts_with("B. two")),
        "missing 'B. two'\n{}",
        out
    );
    assert!(
        lines.iter().any(|s| s.trim_start().starts_with("C. three")),
        "missing 'C. three'\n{}",
        out
    );
}

// Multi-line list items should render continuation lines without a new enumerator
// and with correct indentation.
#[test]
fn multiline_items_do_not_repeat_enumerator() {
    let l = List::new()
        .enumerator(alphabet)
        .item("hello\nworld")
        .item("goodbye");

    let out = format!("{}", l);
    let lines: Vec<&str> = out.lines().collect();

    // First line has A. hello
    assert!(
        lines.get(0).is_some() && lines[0].contains("A. hello"),
        "first line should contain 'A. hello'\n{}",
        out
    );
    // Second line should be a continuation, not starting with an enumerator token like 'B.'
    assert!(
        lines.get(1).is_some() && !lines[1].trim_start().starts_with("B."),
        "continuation line should not have a new enumerator\n{}",
        out
    );
}

// Empty children list should render nothing extra and not panic.
#[test]
fn empty_nested_list_is_noop() {
    let empty_child = List::new().enumerator(arabic); // no items
    let top = List::new()
        .enumerator(alphabet)
        .item("only")
        .item_list(empty_child);

    let out = format!("{}", top);
    let lines: Vec<&str> = out.lines().collect();
    assert_eq!(
        lines.len(),
        1,
        "expected a single top-level item, got {}\n{}",
        lines.len(),
        out
    );
    assert!(lines[0].contains("A. only"), "expected 'A. only'\n{}", out);
}
