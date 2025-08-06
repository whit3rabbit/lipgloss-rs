use lipgloss_tree::Children;

/// Enumerator enumerates a list. Given a list of items and the index of the
/// current enumeration, it returns the prefix that should be displayed for the
/// current item.
///
/// For example, a simple Arabic numeral enumeration would be:
///
/// ```rust
/// use lipgloss_tree::Children;
///
/// fn arabic(_items: &dyn Children, i: usize) -> String {
///     format!("{}.", i + 1)
/// }
/// ```
///
/// There are several predefined enumerators:
/// - alphabet
/// - arabic
/// - bullet
/// - dash
/// - roman
/// - asterisk
///
/// Or, define your own.
pub type Enumerator = fn(&dyn Children, usize) -> String;

/// Indenter indents the children of a tree.
///
/// Indenters allow for displaying nested tree items with connecting borders
/// to sibling nodes.
///
/// For example, the default indenter would be:
///
/// ```rust
/// use lipgloss_tree::Children;
///
/// fn tree_indenter(children: &dyn Children, index: usize) -> String {
///     if children.length() - 1 == index {
///         "│  ".to_string()
///     } else {
///         "   ".to_string()
///     }
/// }
/// ```
pub type Indenter = fn(&dyn Children, usize) -> String;

const ABC_LEN: usize = 26;

/// Alphabet is the enumeration for alphabetical listing.
///
/// Example:
/// ```text
///   a. Foo
///   b. Bar
///   c. Baz
///   d. Qux
/// ```
pub fn alphabet(_items: &dyn Children, i: usize) -> String {
    if i >= ABC_LEN * ABC_LEN + ABC_LEN {
        let c1 = ((i / ABC_LEN / ABC_LEN) - 1) % ABC_LEN;
        let c2 = ((i / ABC_LEN) - 1) % ABC_LEN;
        let c3 = i % ABC_LEN;
        format!(
            "{}{}{}.",
            char::from(b'A' + c1 as u8),
            char::from(b'A' + c2 as u8),
            char::from(b'A' + c3 as u8)
        )
    } else if i >= ABC_LEN {
        let c1 = (i / ABC_LEN) - 1;
        let c2 = i % ABC_LEN;
        format!(
            "{}{}.",
            char::from(b'A' + c1 as u8),
            char::from(b'A' + c2 as u8)
        )
    } else {
        format!("{}.", char::from(b'A' + (i % ABC_LEN) as u8))
    }
}

/// Arabic is the enumeration for arabic numerals listing.
///
/// Example:
/// ```text
///   1. Foo
///   2. Bar
///   3. Baz
///   4. Qux
/// ```
pub fn arabic(_items: &dyn Children, i: usize) -> String {
    format!("{}.", i + 1)
}

/// Roman is the enumeration for roman numerals listing.
///
/// Example:
/// ```text
///   I. Foo
///  II. Bar
/// III. Baz
///  IV. Qux
/// ```
pub fn roman(_items: &dyn Children, mut i: usize) -> String {
    let roman = [
        "M", "CM", "D", "CD", "C", "XC", "L", "XL", "X", "IX", "V", "IV", "I",
    ];
    let arabic = [1000, 900, 500, 400, 100, 90, 50, 40, 10, 9, 5, 4, 1];
    let mut result = String::new();

    i += 1; // Convert to 1-based indexing

    for (idx, &value) in arabic.iter().enumerate() {
        while i >= value {
            i -= value;
            result.push_str(roman[idx]);
        }
    }

    result.push('.');
    result
}

/// Bullet is the enumeration for bullet listing.
///
/// Example:
/// ```text
///   • Foo
///   • Bar
///   • Baz
///   • Qux
/// ```
pub fn bullet(_items: &dyn Children, _i: usize) -> String {
    "•".to_string()
}

/// Asterisk is an enumeration using asterisks.
///
/// Example:
/// ```text
///   * Foo
///   * Bar
///   * Baz
///   * Qux
/// ```
pub fn asterisk(_items: &dyn Children, _i: usize) -> String {
    "*".to_string()
}

/// Dash is an enumeration using dashes.
///
/// Example:
/// ```text
///   - Foo
///   - Bar
///   - Baz
///   - Qux
/// ```
pub fn dash(_items: &dyn Children, _i: usize) -> String {
    "-".to_string()
}
