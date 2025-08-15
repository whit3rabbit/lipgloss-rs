## lipgloss-list

A list component for Terminal UIs, styled with Lip Gloss. This crate is part of the lipgloss-rs ecosystem and aims for 1:1 API and rendering parity with Charm's Go implementation.

- Simple and nested lists (sublists)
- Predefined enumerators: `alphabet`, `arabic`, `roman`, `bullet`, `dash`, `asterisk`
- Custom enumerators and custom indenter support
- Per-item styling and enumerator styling via style functions
- Multiline items with correct alignment

### Installation

Use the batteries-included facade (recommended):

```toml
[dependencies]
lipgloss-extras = { version = "0.0.9", features = ["lists"] }
```

Or depend directly on the component (add `lipgloss` for styling):

```toml
[dependencies]
lipgloss-list = "0.0.9"
lipgloss = "0.0.9"
```

### Quick start

```rust
use lipgloss_list::List;

let l = List::new()
    .items(vec!["A", "B", "C"]);

println!("{}", l);
```

Output:

```
• A
• B
• C
```

### Enumerators

Pick from built-ins or define your own.

```rust
use lipgloss_list::{List, roman, arabic, alphabet, bullet, dash, asterisk};

let a = List::new().items(vec!["Foo", "Bar", "Baz"]).enumerator(arabic);
let r = List::new().items(vec!["Foo", "Bar", "Baz"]).enumerator(roman);
let ab = List::new().items(vec!["Foo", "Bar", "Baz"]).enumerator(alphabet);
let b = List::new().items(vec!["Foo", "Bar", "Baz"]).enumerator(bullet);
let d = List::new().items(vec!["Foo", "Bar", "Baz"]).enumerator(dash);
let s = List::new().items(vec!["Foo", "Bar", "Baz"]).enumerator(asterisk);

println!("{}\n\n{}\n\n{}\n\n{}\n\n{}\n\n{}", a, r, ab, b, d, s);
```

Define a custom enumerator:

```rust
use lipgloss_tree::Children;
use lipgloss_list::List;

fn bracketed(_items: &dyn Children, i: usize) -> String {
    format!("[{}]", i + 1)
}

let l = List::new().items(vec!["X", "Y", "Z"]).enumerator(bracketed);
println!("{}", l);
```

### Sublists (nested lists)

Sublists are just lists as items.

```rust
use lipgloss_list::{List, roman};

let l = List::new()
    .item("A")
    .item("B")
    .item_list(List::new().items(vec!["C1", "C2", "C3"]).enumerator(roman))
    .item("D");

println!("{}", l);
```

Which renders like:

```
• A
• B
    I. C1
   II. C2
  III. C3
• D
```

### Styling items and enumerators

Use style functions to style items and/or enumerators based on index or context.

```rust
use lipgloss::{Color, Style};
use lipgloss_list::List;
use lipgloss_tree::Children;

let l = List::new()
    .items(vec!["First", "Second", "Third", "Fourth"]) 
    .item_style_func(|_items: &dyn Children, i| {
        if i % 2 == 0 { Style::new().foreground(Color::from("86")) }
        else { Style::new().foreground(Color::from("219")) }
    })
    .enumerator_style_func(|items: &dyn Children, i| {
        if i == items.length() - 1 { Style::new().bold(true) } else { Style::new() }
    });

println!("{}", l);
```

Apply a base style to all items or all enumerators:

```rust
use lipgloss::{Color, Style};
use lipgloss_list::List;

let l = List::new()
    .items(vec!["Alpha", "Beta", "Gamma"]) 
    .item_style(Style::new().foreground(Color::from("201")))
    .enumerator_style(Style::new().foreground(Color::from("238")).padding_right(1));

println!("{}", l);
```

### Custom indentation

Change indentation for nested content via `indenter`.

```rust
use lipgloss_list::List;
use lipgloss_tree::Children;

fn arrow_indenter(_items: &dyn Children, _i: usize) -> String { "→ ".into() }

let l = List::new()
    .items(vec!["Foo", "Bar", "Baz"]) 
    .indenter(arrow_indenter);

println!("{}", l);
```

Note: lists default to a two-space indenter for sublists, matching golden output.

### Offsets and hiding

- `offset(start, end)` shows only a subset of items, useful for paging.
- `hide(true)` hides the entire list.

```rust
use lipgloss_list::List;

let visible_slice = List::new().items(vec!["A", "B", "C", "D"]).offset(1, 1);
let hidden = List::new().items(vec!["X", "Y"]).hide(true);

println!("{}", visible_slice);
assert_eq!(format!("{}", hidden), "");
```

### Multiline items

Items can contain newlines; continuation lines are indented correctly.

```rust
use lipgloss_list::List;

let l = List::new().items(vec![
    "Item1 \nline 2\nline 3",
    "Item2 \nline 2\nline 3",
    "3",
]);

println!("{}", l);
```

### Demos

Run from the repository root:

```bash
cargo run --package simple-list-demo
cargo run --package list-demo-duckduckgoose
cargo run --package list-demo-roman
cargo run --package list-demo-glow
cargo run --package list-demo-grocery
```

### Notes

- Lists are built on top of `lipgloss-tree` and benefit from its Unicode-aware layout rules.
- Roman numerals alignment is handled to match Go’s output.
- Nesting trees and tables inside lists is supported; deep complex nestings match golden coverage (with one known spacing discrepancy documented in `docs/TREE_IN_LIST_SPACING_ISSUE.md`).

### Documentation

- API docs: `https://docs.rs/lipgloss-list`
- Project: `https://github.com/whit3rabbit/lipgloss-rs`

### License

MIT
