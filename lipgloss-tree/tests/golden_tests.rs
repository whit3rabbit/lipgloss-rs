use std::fs;
use std::path::PathBuf;

use lipgloss::renderer::{set_color_profile, ColorProfileKind};
use lipgloss_tree as tree;
use lipgloss_tree::{new_string_data, root, Children, Filter, Tree};

// Helper macro for easier child creation
macro_rules! child {
    ($($item:expr),* $(,)?) => {
        vec![$($item.into()),*]
    };
}

fn golden_dir() -> PathBuf {
    // lipgloss-tree is at .../lipgloss-rs/lipgloss-tree
    // golden testdata lives at .../lipgloss-rs/lipgloss-tree/tests/testdata
    let here = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    here.join("tests/testdata")
}

fn read_golden(rel: &str) -> String {
    let p = golden_dir().join(rel);
    fs::read_to_string(&p).unwrap_or_else(|e| panic!("failed to read {:?}: {}", p, e))
}

fn assert_matches_golden(output: &str, rel: &str) {
    let want = read_golden(rel);
    assert_eq!(output, want, "mismatch for golden {:?}", rel);
}

#[test]
fn test_tree_before_after() {
    let tr = Tree::new().child(child![
        "Foo",
        root("Bar").child(child![
            "Qux",
            root("Quux").child(child!["Foo", "Bar"]),
            "Quuux"
        ]),
        "Baz"
    ]);

    // before
    assert_matches_golden(&format!("{}", tr), "TestTree/before.golden");

    // after (rounded)
    let tr2 = tr.clone().enumerator(tree::rounded_enumerator);
    assert_matches_golden(&format!("{}", tr2), "TestTree/after.golden");
}

#[test]
fn test_tree_hidden() {
    let tr = Tree::new().child(child![
        "Foo",
        root("Bar").child(child![
            "Qux",
            root("Quux").child(child!["Foo", "Bar"]).hide(true),
            "Quuux"
        ]),
        "Baz"
    ]);
    assert_matches_golden(&format!("{}", tr), "TestTreeHidden.golden");
}

#[test]
fn test_tree_all_hidden() {
    let tr = Tree::new()
        .child(child![
            "Foo",
            root("Bar").child(child![
                "Qux",
                root("Quux").child(child!["Foo", "Bar"]),
                "Quuux"
            ]),
            "Baz"
        ])
        .hide(true);
    assert_matches_golden(&format!("{}", tr), "TestTreeAllHidden.golden");
}

#[test]
fn test_tree_root() {
    let tr = Tree::new().root("Root").child(child![
        "Foo",
        root("Bar").child(child!["Qux", "Quuux"]),
        "Baz"
    ]);
    assert_matches_golden(&format!("{}", tr), "TestTreeRoot.golden");
}

#[test]
fn test_tree_starts_with_subtree() {
    let tr = Tree::new().child(child![
        Tree::new().root("Bar").child(child!["Qux", "Quuux"]),
        "Baz"
    ]);
    assert_matches_golden(&format!("{}", tr), "TestTreeStartsWithSubtree.golden");
}

#[test]
fn test_tree_add_two_subtrees_without_name() {
    let tr = Tree::new().child(child![
        "Bar",
        "Foo",
        Tree::new().child(child!["Qux", "Qux", "Qux", "Qux", "Qux"]),
        Tree::new().child(child!["Quux", "Quux", "Quux", "Quux", "Quux"]),
        "Baz"
    ]);
    assert_matches_golden(
        &format!("{}", tr),
        "TestTreeAddTwoSubTreesWithoutName.golden",
    );
}

#[test]
fn test_tree_last_node_is_subtree() {
    let tr = Tree::new().child(child![
        "Foo",
        root("Bar").child(child![
            "Qux",
            root("Quux").child(child!["Foo", "Bar",]),
            "Quuux",
        ]),
    ]);
    assert_matches_golden(&format!("{}", tr), "TestTreeLastNodeIsSubTree.golden");
}

#[test]
fn test_tree_nil() {
    // Go test uses nil; here simulate by adding an empty-value container
    let tr = Tree::new().child(child![
        Tree::new(), // Empty tree simulates nil
        root("Bar").child(child!["Qux", root("Quux").child(child!["Bar"]), "Quuux",]),
        "Baz",
    ]);
    assert_matches_golden(&format!("{}", tr), "TestTreeNil.golden");
}

#[test]
fn test_tree_custom_enumerators_and_styles() {
    // Use ANSI profile to match Go golden output
    set_color_profile(ColorProfileKind::ANSI);
    // Mimic TestTreeCustom: customize enumerator/indenter and styles (blue arrows, red items)
    // Use bright blue (94) and bright red (91) to match golden output
    let enum_style = lipgloss::Style::new()
        .foreground(lipgloss::Color::from("94"))
        .padding_right(1);
    let item_style = lipgloss::Style::new().foreground(lipgloss::Color::from("91"));

    let tr = Tree::new()
        .child(child![
            "Foo",
            Tree::new().root("Bar").child(child![
                "Qux",
                Tree::new().root("Quux").child(child!["Foo", "Bar",]),
                "Quuux",
            ]),
            "Baz",
        ])
        .item_style(item_style)
        .enumerator_style(enum_style)
        .enumerator(|_, _| "->".to_string())
        .indenter(|_, _| "->".to_string());

    assert_matches_golden(&format!("{}", tr), "TestTreeCustom.golden");
}

#[test]
fn test_tree_multiline_node() {
    let tr = Tree::new().root("Big\nRoot\nNode").child(child![
        "Foo",
        Tree::new().root("Bar").child(child![
            "Line 1\nLine 2\nLine 3\nLine 4",
            Tree::new().root("Quux").child(child!["Foo", "Bar",]),
            "Quuux",
        ]),
        "Baz\nLine 2",
    ]);
    assert_matches_golden(&format!("{}", tr), "TestTreeMultilineNode.golden");
}

#[test]
fn test_tree_subtree_with_custom_enumerator() {
    let tr = Tree::new().root("The Root Node™").child(child![
        Tree::new()
            .root("Parent")
            .child(child!["child 1", "child 2",])
            .item_style_func(|_, _| lipgloss::Style::new().set_string("*"))
            .enumerator_style_func(|_, _| {
                lipgloss::Style::new().set_string("+").padding_right(1)
            }),
        "Baz",
    ]);
    assert_matches_golden(
        &format!("{}", tr),
        "TestTreeSubTreeWithCustomEnumerator.golden",
    );
}

#[test]
fn test_tree_mixed_enumerator_size() {
    let tr = Tree::new()
        .root("The Root Node™")
        .child(child!["Foo", "Foo", "Foo", "Foo", "Foo",])
        .enumerator(|_, i| match i + 1 {
            1 => "I".into(),
            2 => "II".into(),
            3 => "III".into(),
            4 => "IV".into(),
            5 => "V".into(),
            6 => "VI".into(),
            _ => unreachable!(),
        });
    assert_matches_golden(&format!("{}", tr), "TestTreeMixedEnumeratorSize.golden");
}

#[test]
fn test_tree_style_nil_funcs() {
    let tr = Tree::new()
        .root("Silly")
        .child(child!["Willy ", "Nilly",])
        // Use no-op closures to simulate nil funcs
        .item_style_func(|_, _| lipgloss::Style::new())
        .enumerator_style_func(|_, _| lipgloss::Style::new());
    assert_matches_golden(&format!("{}", tr), "TestTreeStyleNilFuncs.golden");
}

#[test]
fn test_tree_style_at() {
    let tr = Tree::new()
        .root("Root")
        .child(child!["Foo", "Baz",])
        .enumerator(|data, i| {
            if data.at(i).map(|n| n.value()) == Some("Foo".into()) {
                ">".into()
            } else {
                "-".into()
            }
        });
    assert_matches_golden(&format!("{}", tr), "TestTreeStyleAt.golden");
}

#[test]
fn test_at() {
    let data = new_string_data(&["Foo", "Bar"]);
    assert_eq!(data.at(0).unwrap().value(), "Foo");
    assert!(data.at(10).is_none());
}

#[test]
fn test_filter_and_node_data_len() {
    let data = Filter::new(Box::new(new_string_data(&["Foo", "Bar", "Baz", "Nope"])))
        .filter(|index| index != 3);
    // Expand filtered data into explicit children under Root
    let mut children = Vec::new();
    for i in 0..data.length() {
        if let Some(n) = data.at(i) {
            children.push(n.value().into());
        }
    }
    let tr = Tree::new().root("Root").child(children);
    assert_matches_golden(&format!("{}", tr), "TestFilter.golden");
}

// The following tests require other crates (lipgloss-list, lipgloss-table). Mirror now, enable when crates are ready.
#[test]
fn test_root_style() {
    set_color_profile(ColorProfileKind::TrueColor);
    let tr = Tree::new()
        .root("Root")
        .child(child!["Foo", "Baz",])
        .root_style(lipgloss::Style::new().background(lipgloss::Color::from("#5A56E0")))
        .item_style(lipgloss::Style::new().background(lipgloss::Color::from("#04B575")));
    assert_matches_golden(&format!("{}", tr), "TestRootStyle.golden");
}

#[test]
fn test_embed_list_within_tree() {
    use lipgloss_list::{self as list, List};
    let list1 = List::new()
        .items(vec!["A", "B", "C"])
        .enumerator(list::arabic);
    let list2 = List::new()
        .items(vec!["1", "2", "3"])
        .enumerator(list::alphabet);
    let t1 = Tree::new()
        .child(child![format!("{}", list1)])
        .child(child![format!("{}", list2)]);
    assert_matches_golden(&format!("{}", t1), "TestEmbedListWithinTree.golden");
}

#[test]
fn test_multiline_prefix() {
    let padding_style = lipgloss::Style::new().padding_left(1).padding_bottom(1);
    let tr = Tree::new()
        .enumerator(|_, i| {
            if i == 1 {
                "│\n│".to_string()
            } else {
                " ".to_string()
            }
        })
        .indenter(|_, _| " ".to_string())
        .item_style(padding_style)
        .enumerator_style_func(|_, _| lipgloss::Style::new())
        .child(child![
            "Foo Document\nThe Foo Files",
            "Bar Document\nThe Bar Files",
            "Baz Document\nThe Baz Files",
        ]);
    assert_matches_golden(&format!("{}", tr), "TestMultilinePrefix.golden");
}

#[test]
fn test_multiline_prefix_subtree() {
    let padding_style = lipgloss::Style::new().padding(0, 0, 1, 1);
    let tr = Tree::new().child(child![
        "Foo",
        "Bar",
        Tree::new()
            .root("Baz")
            .enumerator(|_, i| {
                if i == 1 {
                    "│\n│".to_string()
                } else {
                    " ".to_string()
                }
            })
            .indenter(|_, _| " ".to_string())
            .item_style(padding_style.clone())
            .enumerator_style_func(|_, _| lipgloss::Style::new())
            .child(child![
                "Foo Document\nThe Foo Files",
                "Bar Document\nThe Bar Files",
                "Baz Document\nThe Baz Files",
            ]),
        "Qux",
    ]);
    assert_matches_golden(&format!("{}", tr), "TestMultilinePrefixSubtree.golden");
}

#[test]
fn test_multiline_prefix_inception() {
    let glow_enum = |_: &dyn tree::Children, i: usize| {
        if i == 1 {
            "│\n│".to_string()
        } else {
            " ".to_string()
        }
    };
    let glow_indenter = |_: &dyn tree::Children, _: usize| "  ".to_string();
    let padding_style = lipgloss::Style::new().padding_left(1).padding_bottom(1);

    let tr = Tree::new()
        .enumerator(glow_enum)
        .indenter(glow_indenter)
        .item_style(padding_style.clone())
        .enumerator_style_func(|_, _| lipgloss::Style::new())
        .child(child![
            "Foo Document\nThe Foo Files",
            "Bar Document\nThe Bar Files",
            Tree::new()
                .enumerator(glow_enum)
                .indenter(glow_indenter)
                .item_style(padding_style.clone())
                .enumerator_style_func(|_, _| lipgloss::Style::new())
                .child(child![
                    "Qux Document\nThe Qux Files",
                    "Quux Document\nThe Quux Files",
                    "Quuux Document\nThe Quuux Files",
                ]),
            "Baz Document\nThe Baz Files",
        ]);
    assert_matches_golden(&format!("{}", tr), "TestMultilinePrefixInception.golden");
}

#[test]
fn test_types() {
    let tr = Tree::new().child(child![
        "0",    // simulating fmt.Sprintf("%v", 0)
        "true", // simulating fmt.Sprintf("%v", true)
        "Foo",  // []any{"Foo", "Bar"} expanded
        "Bar", "Qux", // []string{"Qux", "Quux", "Quuux"} expanded
        "Quux", "Quuux",
    ]);
    assert_matches_golden(&format!("{}", tr), "TestTypes.golden");
}

#[test]
fn test_add_item_with_and_without_root() {
    // Test "with root"
    let t1 = Tree::new().child(child![
        "Foo",
        "Bar",
        Tree::new().child(child!["Baz"]),
        "Qux",
    ]);
    assert_matches_golden(
        &format!("{}", t1),
        "TestAddItemWithAndWithoutRoot/with_root.golden",
    );

    // Test "without root"
    let t2 = Tree::new().child(child![
        "Foo",
        Tree::new().root("Bar").child(child!["Baz"]),
        "Qux",
    ]);
    assert_matches_golden(
        &format!("{}", t2),
        "TestAddItemWithAndWithoutRoot/without_root.golden",
    );
}

#[test]
fn test_tree_table() {
    use lipgloss_table::Table;
    let tbl = Table::new()
        .width(20)
        .style_func(|_, _| lipgloss::Style::new().padding(0, 1, 0, 1))
        .headers(vec!["Foo", "Bar"])
        .row(vec!["Qux", "Baz"])
        .row(vec!["Qux", "Baz"])
        .render();

    let tr = Tree::new().child(child![
        "Foo",
        root("Bar").child(child!["Baz", "Baz", tbl, "Baz"]),
        "Qux"
    ]);
    assert_matches_golden(&format!("{}", tr), "TestTreeTable.golden");
}
