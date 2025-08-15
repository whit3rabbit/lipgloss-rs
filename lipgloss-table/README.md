## lipgloss-table

A table component for Terminal UIs, styled with Lip Gloss. This crate is part of the lipgloss-rs ecosystem and aims for 1:1 API and rendering parity with Charm's Go implementation.

- Headers, rows, borders, and separators
- Per-cell styling with `lipgloss::Style`
- Intelligent column sizing (expand/shrink) with Unicode-aware width
- Optional wrapping or truncation with ellipsis
- Fixed column widths via style, padding/margins support

### Installation

Use the batteries-included facade (recommended):

```toml
[dependencies]
lipgloss-extras = { version = "0.0.9", features = ["tables"] }
```

Or depend directly on the component (add `lipgloss` for styling):

```toml
[dependencies]
lipgloss-table = "0.0.9"
lipgloss = "0.0.9"
```

### Quick start

```rust
use lipgloss_table::Table;

let mut t = Table::new()
    .headers(vec!["Name", "Age", "City"])
    .row(vec!["Alice", "30", "New York"])
    .row(vec!["Bob", "25", "London"]);

println!("{}", t.render());
```

### Styling cells

Apply styles per cell with a function. Use `HEADER_ROW` for header styling.

```rust
use lipgloss::{Color, Style};
use lipgloss_table::{Table, HEADER_ROW};

let style_fn = |row: i32, _col: usize| {
    match row {
        HEADER_ROW => Style::new().bold(true).foreground(Color::from("63")),
        r if r % 2 == 0 => Style::new().foreground(Color::from("238")),
        _ => Style::new(),
    }
};

let mut t = Table::new()
    .headers(vec!["Language", "Formal", "Informal"])
    .row(vec!["Japanese", "こんにちは", "やあ"])
    .row(vec!["Russian", "Здравствуйте", "Привет"])
    .style_func(style_fn);

println!("{}", t.render());
```

For complex logic, capture state with a boxed closure:

```rust
use lipgloss::{Color, Style};
use lipgloss_table::{Table, HEADER_ROW};

let highlight = Color::from("201");
let mut t = Table::new()
    .headers(vec!["Status", "Message"])
    .row(vec!["ERROR", "Something went wrong"])
    .row(vec!["OK", "All good"])
    .style_func_boxed(move |row, col| {
        if row == HEADER_ROW { return Style::new().bold(true); }
        if col == 0 {
            match row {
                0 => Style::new().foreground(highlight.clone()),
                _ => Style::new(),
            }
        } else {
            Style::new()
        }
    });

println!("{}", t.render());
```

Predefined helpers are available:

```rust
use lipgloss_table::{Table, header_row_style, zebra_style, minimal_style};

let mut t = Table::new()
    .headers(vec!["Item", "Qty"]) 
    .row(vec!["Apples", "3"]) 
    .row(vec!["Bananas", "5"]) 
    .style_func(zebra_style);

println!("{}", t.render());
```

### Borders and separators

Enable/disable borders and separators:

```rust
use lipgloss::{rounded_border, thick_border, Style, Color};
use lipgloss_table::Table;

let mut bordered = Table::new()
    .headers(vec!["A", "B"]) 
    .row(vec!["1", "2"]) 
    .border(rounded_border())
    .border_style(Style::new().foreground(Color::from("238")))
    .border_row(true);

let mut minimal = Table::new()
    .headers(vec!["A", "B"]) 
    .row(vec!["1", "2"]) 
    .border_top(false)
    .border_bottom(false)
    .border_left(false)
    .border_right(false)
    .border_header(false)
    .border_column(false);

println!("{}\n\n{}", bordered.render(), minimal.render());
```

### Width, wrapping, and truncation

- Set `width(i32)` to constrain total table width. Columns are resized with an intelligent expand/shrink algorithm.
- Enable `wrap(true)` to wrap cell content; use `wrap(false)` to truncate with ellipsis.
- Set `height(i32)` and `offset(usize)` for paging/scrolling.

```rust
use lipgloss_table::Table;

let mut wrap_demo = Table::new()
    .headers(vec!["Product", "Description"]) 
    .row(vec![
        "MacBook Pro",
        "Powerful laptop for developers with long descriptions that wrap",
    ])
    .width(40)
    .wrap(true);

let mut trunc_demo = Table::new()
    .headers(vec!["Product", "Description"]) 
    .row(vec![
        "MacBook Pro",
        "This description will be truncated rather than wrapped",
    ])
    .width(30)
    .wrap(false);

println!("{}\n\n{}", wrap_demo.render(), trunc_demo.render());
```

### Fixed column widths and padding

Column widths are inferred from content and style. You can set fixed widths and spacing via per-cell styles in your style function. The widest explicit width per column is respected.

```rust
use lipgloss::Style;
use lipgloss_table::{Table, HEADER_ROW};

let style_fn = |row: i32, col: usize| {
    let mut s = Style::new();
    if row == HEADER_ROW { s = s.bold(true); }
    // Fix the first column to width 12, add padding to all cells
    if col == 0 { s = s.width(12); }
    s.padding(0, 1, 0, 1)
};

let mut t = Table::new()
    .headers(vec!["Name", "Role"]) 
    .row(vec!["Alice Johnson", "Engineer"]) 
    .row(vec!["Bob", "PM"]) 
    .style_func(style_fn)
    .width(32);

println!("{}", t.render());
```

### Data sources and filtering

You can supply your own data source by implementing `Data`, or use the built-in `StringData`. Filter rows without copying using `Filter`:

```rust
use lipgloss_table::{rows::{StringData, Filter}, Table};

let data = StringData::new(vec![
    vec!["A".into(), "1".into()],
    vec!["B".into(), "2".into()],
    vec!["C".into(), "3".into()],
]);

let filtered = Filter::new(data).filter(|row| row % 2 == 0); // keep even rows

let mut t = Table::new()
    .headers(vec!["Key", "Val"]) 
    .data(filtered)
    .width(18);

println!("{}", t.render());
```

### Demos

Run demos from the repository root:

```bash
cargo run --package table-demo-languages
cargo run --package table-demo-chess
cargo run --package table-demo-mindy
cargo run --package table-demo-pokemon
cargo run --package table-demo-ansi
```

### Notes

- Width/height calculations are Unicode- and ANSI-aware via `lipgloss::width`.
- Borders are rendered using `lipgloss::Border` presets (rounded, thick, block, etc.).
- Height-constrained rendering uses an overflow indicator row when data exceeds available space.

### Documentation

- API docs: `https://docs.rs/lipgloss-table`
- Project: `https://github.com/whit3rabbit/lipgloss-rs`

### License

MIT
