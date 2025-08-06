use lipgloss_list::{alphabet, arabic, asterisk, dash, roman, List};

#[test]
fn test_bullet_list() {
    let list = List::new().items(vec!["Item 1", "Item 2", "Item 3"]);
    let output = format!("{}", list);
    assert!(output.contains("• Item 1"));
    assert!(output.contains("• Item 2"));
    assert!(output.contains("• Item 3"));
}

#[test]
fn test_arabic_list() {
    let list = List::new()
        .items(vec!["First", "Second", "Third"])
        .enumerator(arabic);
    let output = format!("{}", list);
    assert!(output.contains("1. First"));
    assert!(output.contains("2. Second"));
    assert!(output.contains("3. Third"));
}

#[test]
fn test_roman_list() {
    let list = List::new()
        .items(vec!["Alpha", "Beta", "Gamma", "Delta"])
        .enumerator(roman);
    let output = format!("{}", list);
    assert!(output.contains("I. Alpha"));
    assert!(output.contains("II. Beta"));
    assert!(output.contains("III. Gamma"));
    assert!(output.contains("IV. Delta"));
}

#[test]
fn test_alphabet_list() {
    let list = List::new()
        .items(vec!["Apple", "Banana", "Cherry"])
        .enumerator(alphabet);
    let output = format!("{}", list);
    assert!(output.contains("A. Apple"));
    assert!(output.contains("B. Banana"));
    assert!(output.contains("C. Cherry"));
}

#[test]
fn test_dash_list() {
    let list = List::new()
        .items(vec!["Option A", "Option B"])
        .enumerator(dash);
    let output = format!("{}", list);
    assert!(output.contains("- Option A"));
    assert!(output.contains("- Option B"));
}

#[test]
fn test_asterisk_list() {
    let list = List::new()
        .items(vec!["Note 1", "Note 2"])
        .enumerator(asterisk);
    let output = format!("{}", list);
    assert!(output.contains("* Note 1"));
    assert!(output.contains("* Note 2"));
}

#[test]
fn test_list_offset() {
    let list = List::new()
        .items(vec!["A", "B", "C", "D", "E"])
        .offset(1, 1); // Skip first item and last item
    let output = format!("{}", list);
    assert!(!output.contains("• A"));
    assert!(output.contains("• B"));
    assert!(output.contains("• C"));
    assert!(output.contains("• D"));
    assert!(!output.contains("• E"));
}

#[test]
fn test_hidden_list() {
    let list = List::new()
        .items(vec!["Visible", "Also visible"])
        .hide(true);
    let output = format!("{}", list);
    assert_eq!(output, ""); // Hidden list should produce empty output
}
