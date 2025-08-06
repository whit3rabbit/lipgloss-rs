use lipgloss_list::{alphabet, arabic, List};

// This targeted test verifies that nested content added between items does not
// increment the parent list's enumeration index. Specifically, after adding
// nested lists under "Deeper", the next sibling should still be enumerated as
// "C. bar" (not "D. bar").
#[test]
fn alphabet_sublist_enumeration_does_not_count_nested_content() {
    // Parent alphabetical list with two items
    let mut alpha = List::new()
        .enumerator(alphabet)
        .items(vec!["foo", "Deeper"]);

    // A nested arabic list to attach under "Deeper"
    let nested = List::new().enumerator(arabic).items(vec!["a", "b", "c"]);

    // Attach the nested list (container node) and then add the final sibling
    alpha = alpha.item_list(nested);
    alpha = alpha.item("bar");

    let out = format!("{}", alpha);
    let lines: Vec<&str> = out.lines().collect();

    // Collect only lines that look like top-level alphabet enumerations
    let alpha_lines: Vec<&str> = lines
        .iter()
        .copied()
        .filter(|l| {
            l.trim_start().starts_with("A. ")
                || l.trim_start().starts_with("B. ")
                || l.trim_start().starts_with("C. ")
        })
        .collect();

    // We expect exactly A. foo, B. Deeper, C. bar in order
    assert_eq!(
        alpha_lines.len(),
        3,
        "expected exactly three top-level alphabet items, got: {}\n{}",
        alpha_lines.len(),
        out
    );
    assert!(
        alpha_lines[0].contains("A. foo"),
        "first item should be 'A. foo' but was: '{}'\n{}",
        alpha_lines[0],
        out
    );
    assert!(
        alpha_lines[1].contains("B. Deeper"),
        "second item should be 'B. Deeper' but was: '{}'\n{}",
        alpha_lines[1],
        out
    );
    assert!(
        alpha_lines[2].contains("C. bar"),
        "third item should be 'C. bar' but was: '{}'\n{}",
        alpha_lines[2],
        out
    );
}
