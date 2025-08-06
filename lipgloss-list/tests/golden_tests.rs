use std::fs;
use std::path::PathBuf;

use lipgloss_list::{alphabet, arabic, asterisk, bullet, dash, roman, List};

fn testdata_path(rel: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../lipgloss-master/list/testdata")
        .join(rel)
}

fn read_golden(rel: &str) -> String {
    let path = testdata_path(rel);
    fs::read_to_string(&path)
        .unwrap_or_else(|e| panic!("failed to read golden {}: {}", path.display(), e))
}

fn normalize(s: &str) -> String {
    // Trim leading/trailing empty lines and trailing whitespace per line
    let mut lines: Vec<&str> = s.lines().collect();
    while matches!(lines.first(), Some(l) if l.trim().is_empty()) {
        lines.remove(0);
    }
    while matches!(lines.last(), Some(l) if l.trim().is_empty()) {
        lines.pop();
    }
    let lines: Vec<String> = lines
        .into_iter()
        .map(|l| l.trim_end().to_string())
        .collect();
    lines.join("\n")
}

fn assert_golden(rel: &str, got: &str) {
    let expected = read_golden(rel);
    let exp_n = normalize(&expected);
    let got_n = normalize(got);
    assert_eq!(
        got_n, exp_n,
        "mismatch for golden {}\nexpected:\n{}\n\ngot:\n{}\n",
        rel, exp_n, got_n
    );
}

#[test]
fn golden_list_basic() {
    let l = List::new().items(vec!["Foo", "Bar", "Baz"]);
    assert_golden("TestList.golden", &format!("{}", l));
}

#[test]
fn golden_enumerators_bullet() {
    let l = List::new()
        .items(vec!["Foo", "Bar", "Baz"])
        .enumerator(bullet);
    assert_golden("TestEnumerators/bullet.golden", &format!("{}", l));
}

#[test]
fn golden_enumerators_asterisk() {
    let l = List::new()
        .items(vec!["Foo", "Bar", "Baz"])
        .enumerator(asterisk);
    assert_golden("TestEnumerators/asterisk.golden", &format!("{}", l));
}

#[test]
fn golden_enumerators_dash() {
    let l = List::new()
        .items(vec!["Foo", "Bar", "Baz"])
        .enumerator(dash);
    assert_golden("TestEnumerators/dash.golden", &format!("{}", l));
}

#[test]
fn golden_enumerators_arabic() {
    let l = List::new()
        .items(vec!["Foo", "Bar", "Baz"])
        .enumerator(arabic);
    assert_golden("TestEnumerators/arabic.golden", &format!("{}", l));
}

#[test]
fn golden_enumerators_alphabet() {
    let l = List::new()
        .items(vec!["Foo", "Bar", "Baz"])
        .enumerator(alphabet);
    assert_golden("TestEnumerators/alphabet.golden", &format!("{}", l));
}

#[test]
fn golden_enumerators_roman() {
    let l = List::new()
        .items(vec!["Foo", "Bar", "Baz"])
        .enumerator(roman);
    assert_golden("TestEnumerators/roman.golden", &format!("{}", l));
}

// --- Additional golden coverage matching Go testdata ---

// TestListItems: items are arbitrary strings
#[test]
fn golden_list_items() {
    let l = List::new().items(vec!["Foo", "Bar", "Baz"]);
    assert_golden("TestListItems.golden", &format!("{}", l));
}

// TestListIntegers: items are numeric strings
#[test]
fn golden_list_integers() {
    let l = List::new().items(vec!["1", "2", "3"]);
    assert_golden("TestListIntegers.golden", &format!("{}", l));
}

// TestMultiline: items with multiple lines should align under the enumerator
#[test]
fn golden_multiline_items() {
    let l = List::new().items(vec![
        "Item1 \nline 2\nline 3",
        "Item2 \nline 2\nline 3",
        "3",
    ]);
    assert_golden("TestMultiline.golden", &format!("{}", l));
}

// TestSublist: simple nested list using roman numerals for the sublist
#[test]
fn golden_sublist() {
    // Outer: bullet
    let mut outer = List::new().items(vec!["Foo", "Bar"]);
    // Sublist under Bar with roman enumerator and three items
    let sub = List::new()
        .enumerator(roman)
        .items(vec!["Hi", "Hello", "Halo"]);
    outer = outer.item_list(sub).item("Qux");
    assert_golden("TestSublist.golden", &format!("{}", outer));
}

// TestComplexSublist: deep nesting, mixed enumerators, and trees
#[test]
fn golden_complex_sublist() {
    use lipgloss_tree::{Leaf, Node, Tree};

    // • Foo
    // • Bar
    //   • foo2
    //   • bar2
    // • Qux
    //    I. aaa
    //   II. bbb
    // • Deep
    //   A. foo
    //   B. Deeper
    //     1. a
    //     2. b
    //     3. Even Deeper, inherit parent renderer
    //       * sus
    //       * d minor
    //       * f#
    //       * One ore level, with another renderer
    //         - a      \n  multine\n  string
    //         - hoccus poccus
    //         - abra kadabra
    //         - And finally, a tree within all this
    //           <tree structure>
    //         - this is a tree\n  and other obvious statements
    //   C. bar
    // • Baz

    // Build: Foo, Bar(with sublist), Qux(with roman sublist), Deep(with alpha sublist and more), Baz
    let mut root = List::new().item("Foo");

    // Bar with two children (bullet)
    let bar_children = List::new().items(vec!["foo2", "bar2"]);
    root = root.item("Bar").item_list(bar_children);

    // Qux with roman children
    let qux_children = List::new().enumerator(roman).items(vec!["aaa", "bbb"]);
    root = root.item("Qux").item_list(qux_children);

    // Deep branch will be constructed below

    // Under "Deeper": arabic list with three items, then asterisk list nested, then dash list nested, then trees

    // Arabic list under Deeper
    let arabic_list = List::new().enumerator(arabic).items(vec![
        "a",
        "b",
        "Even Deeper, inherit parent renderer",
    ]);

    // Asterisk list under the third arabic item
    let mut star_list = List::new().enumerator(asterisk).items(vec![
        "sus",
        "d minor",
        "f#",
        "One ore level, with another renderer",
    ]);

    // Dash list under the fourth star item
    let dash_list = List::new().enumerator(dash).items(vec![
        "a      \nmultine\nstring ",
        "hoccus poccus",
        "abra kadabra",
        "And finally, a tree within all this",
    ]);

    // A simple tree to embed: empty root with children, where the first child is a
    // single node with a multiline value so continuation lines are indented without
    // additional branch connectors, matching Go golden output.
    let tree = Tree::new().child(vec![
        Box::new(Tree::new().root("another\nmultine\nstring")) as Box<dyn Node>,
        Box::new(Leaf::new("something", false)) as Box<dyn Node>,
        Box::new(Tree::new().root("a subtree").child(vec![
            Box::new(Leaf::new("yup", false)) as Box<dyn Node>,
            Box::new(Leaf::new("many itens", false)) as Box<dyn Node>,
            Box::new(Leaf::new("another", false)) as Box<dyn Node>,
        ])) as Box<dyn Node>,
        Box::new(Leaf::new("hallo", false)) as Box<dyn Node>,
        Box::new(Leaf::new("wunderbar!", false)) as Box<dyn Node>,
    ]);

    // Another tree from free text
    let free_text_tree =
        Tree::new().root("this is a tree              \nand other obvious statements");

    // Assemble nesting
    let dash_list = dash_list
        .item_node(Box::new(tree))
        .item_node(Box::new(free_text_tree));
    star_list = star_list.item_list(dash_list);

    // Attach star_list under arabic_list's third item by just appending as another nested list
    let arabic_list = arabic_list.item_list(star_list);

    // Now attach arabic_list under Deeper
    // Build a new list where after "Deeper" we append the arabic_list and then add trailing C. bar
    let deep_children = List::new()
        .enumerator(alphabet)
        .item("foo")
        .item("Deeper")
        .item_list(arabic_list)
        .item("bar");

    // Attach Deep branch to root and add Baz
    root = root.item("Deep").item_list(deep_children).item("Baz");

    assert_golden("TestComplexSublist.golden", &format!("{}", root));
}

// TestSublistItems and TestSubListItems2
#[test]
fn golden_sublist_items() {
    let mut root = List::new().item("A").item("B").item("C");
    let roman_sub = List::new().enumerator(roman).items(vec!["D", "E", "F"]);
    root = root.item_list(roman_sub).item("G");
    assert_golden("TestSublistItems.golden", &format!("{}", root));
}

#[test]
fn golden_sublist_items2() {
    let mut root = List::new().item("S");
    root = root.item_list(List::new().items(vec!["neovim", "vscode"]));
    root = root.item("HI");
    root = root.item_list(List::new().items(vec!["vim", "doom emacs"]));
    root = root.item("Parent 2");
    root = root.item_list(List::new().items(vec!["I like fuzzy socks"]));
    assert_golden("TestSubListItems2.golden", &format!("{}", root));
}

// TestEnumeratorsAlign: roman numerals alignment
#[test]
fn golden_enumerators_align() {
    let mut l = List::new().enumerator(roman);
    for _ in 0..100 {
        l = l.item("Foo");
    }
    assert_golden("TestEnumeratorsAlign.golden", &format!("{}", l));
}

// TestEnumeratorsTransform: define custom enumerators to match transformations
fn alphabet_lower(_items: &dyn lipgloss_tree::Children, i: usize) -> String {
    let s = alphabet(_items, i);
    s.trim_end_matches('.').to_lowercase() + "."
}

fn arabic_paren(_items: &dyn lipgloss_tree::Children, i: usize) -> String {
    format!("{})", i + 1)
}

fn bullet_is_dash(_items: &dyn lipgloss_tree::Children, _i: usize) -> String {
    "-".to_string()
}

fn roman_within_parens(_items: &dyn lipgloss_tree::Children, i: usize) -> String {
    // lowercase roman without trailing dot, within parentheses
    let r = roman(_items, i).trim_end_matches('.').to_lowercase();
    format!("({})", r)
}

#[test]
fn golden_enumerators_transform_alphabet_lower() {
    let l = List::new()
        .items(vec!["Foo", "Bar", "Baz"])
        .enumerator(alphabet_lower);
    assert_golden(
        "TestEnumeratorsTransform/alphabet_lower.golden",
        &format!("{}", l),
    );
}

#[test]
fn golden_enumerators_transform_arabic_paren() {
    let l = List::new()
        .items(vec!["Foo", "Bar", "Baz"])
        .enumerator(arabic_paren);
    assert_golden("TestEnumeratorsTransform/arabic).golden", &format!("{}", l));
}

#[test]
fn golden_enumerators_transform_bullet_is_dash() {
    let l = List::new()
        .items(vec!["Foo", "Bar", "Baz"])
        .enumerator(bullet_is_dash);
    assert_golden(
        "TestEnumeratorsTransform/bullet_is_dash.golden",
        &format!("{}", l),
    );
}

#[test]
fn golden_enumerators_transform_roman_within_parens() {
    let l = List::new()
        .items(vec!["Foo", "Bar", "Baz"])
        .enumerator(roman_within_parens);
    assert_golden(
        "TestEnumeratorsTransform/roman_within_().golden",
        &format!("{}", l),
    );
}
