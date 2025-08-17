## lipgloss-tree

A tree component for Terminal UIs, styled with Lip Gloss. This crate is part of the lipgloss-rs ecosystem and targets 1:1 API and rendering parity with Charm's Go implementation.

- Render hierarchical trees using box-drawing characters (├──, └──, │)
- Style roots, branch prefixes, and items independently
- Support for custom enumerators and indenters
- Multiline content with correct indentation and alignment

### Installation

Use the batteries-included facade (recommended):

```toml
[dependencies]
lipgloss-extras = { version = "0.1.0", features = ["trees"] }
```

Or depend directly on the component (add `lipgloss` if you want styling):

```toml
[dependencies]
lipgloss-tree = "0.1.0"
lipgloss = "0.1.0"
```

### Quick start

```rust
use lipgloss_tree::{Leaf, Node, Tree};

let t = Tree::new().root(".")
    .child(vec![
        Box::new(Leaf::new("macOS", false)) as Box<dyn Node>,
        Box::new(
            Tree::new().root("Linux")
                .child(vec![
                    Box::new(Leaf::new("NixOS", false)) as Box<dyn Node>,
                    Box::new(Leaf::new("Arch Linux (btw)", false)) as Box<dyn Node>,
                ])
        ) as Box<dyn Node>,
    ]);

println!("{}", t);
```

Output:

```
.
├── macOS
└── Linux
    ├── NixOS
    └── Arch Linux (btw)
```

### Styling

You can style the root, enumerators (branch prefixes), and items. Styles are from the `lipgloss` crate.

```rust
use lipgloss::{Color, Style};
use lipgloss_tree::{Leaf, Node, Tree};

let t = Tree::new()
    .root("Project")
    .root_style(Style::new().bold(true).foreground(Color::from("63")))
    .enumerator_style(Style::new().foreground(Color::from("238")).padding_right(1))
    .item_style(Style::new().foreground(Color::from("201")))
    .child(vec![
        Box::new(Leaf::new("README.md", false)) as Box<dyn Node>,
        Box::new(Leaf::new("src/", false)) as Box<dyn Node>,
    ]);

println!("{}", t);
```

Dynamic styling per item is also supported via functions:

```rust
use lipgloss::{Color, Style};
use lipgloss_tree::Tree;

let t = Tree::new()
    .root("Roman List")
    .child(vec!["First".into(), "Second".into(), "Third".into()])
    .item_style_func(|_, i| {
        if i % 2 == 0 { Style::new().foreground(Color::from("86")) }
        else { Style::new().foreground(Color::from("219")) }
    });

println!("{}", t);
```

### Custom enumerators and indenters

Use built-ins or supply your own.

```rust
use lipgloss_tree::{default_indenter, rounded_enumerator, Tree};

let t = Tree::new()
    .root("Cute Rounds")
    .enumerator(rounded_enumerator)
    .indenter(default_indenter)
    .child(vec!["Foo".into(), "Bar".into(), "Baz".into()]);

println!("{}", t);
```

Custom functions receive the visible children and the current index.

```rust
use lipgloss_tree::{Children, Tree};

let t = Tree::new()
    .root("Custom")
    .enumerator(|children: &dyn Children, i| {
        if i == children.length() - 1 { "╰──".to_string() } else { "├──".to_string() }
    })
    .indenter(|children: &dyn Children, i| {
        if i == children.length() - 1 { "    ".to_string() } else { "│   ".to_string() }
    })
    .child(vec!["A".into(), "B".into(), "C".into()]);

println!("{}", t);
```

### Advanced: Renderer API

For full control, render with `Renderer` and `TreeStyle`.

```rust
use lipgloss::{Color, Style};
use lipgloss_tree::{renderer::{TreeStyle, new_renderer}, Tree};

let style = TreeStyle {
    root: Style::new().bold(true).foreground(Color::from("63")),
    enumerator_func: |_, _| Style::new().padding_right(1),
    item_func: |_, _| Style::new(),
    enumerator_base: None,
    item_base: None,
};

let tree = Tree::new().root("Styled Tree").child(vec!["Item 1".into(), "Item 2".into()]);
let renderer = new_renderer().style(style);
println!("{}", renderer.render(&tree, true, ""));
```

### Multiline content

Items can contain newlines. Indentation is extended to match height.

```rust
use lipgloss_tree::Tree;

let tree = Tree::new().root("Logs").child(vec![
    "Build:\nOK".into(),
    "Test:\nFAIL\nflake".into(),
]);

println!("{}", tree);
```

### Demos

Run the included demos from the repository root:

```bash
cargo run --package tree-demo-simple
cargo run --package tree-demo-files
cargo run --package tree-demo-styles
```

### Documentation

- API docs: `https://docs.rs/lipgloss-tree`
- Project: `https://github.com/whit3rabbit/lipgloss-rs`

### License

MIT
