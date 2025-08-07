# lipgloss-rs

A port of [lipgloss](https://github.com/charmbracelet/lipgloss) to Rust which are style definitions for nice terminal layouts.

![Rust Layout Demo](examples/layout-demo/demo.png)

This is a Rust port of the excellent Go library of the same name, [**lipgloss**](https://github.com/charmbracelet/lipgloss), created by the talented team at [**Charm**](https://charm.sh). This port achieves **1:1 API compatibility** and **exact rendering parity** with the original Go implementation.

`lipgloss-rs` takes an expressive, declarative approach to terminal rendering. Users familiar with CSS will feel at home.

## 🎯 **Go Parity Achieved**

✅ **Automatic Width/Height Constraints** - Act as minimum dimensions with intelligent padding  
✅ **Solid Background Blocks** - Background colors fill entire constrained areas  
✅ **Perfect Text Alignment** - Applied line-by-line for precise visual positioning  
✅ **Smart Border Extension** - Borders automatically encompass full dimensions  
✅ **Vertical Alignment Support** - TOP, CENTER, BOTTOM distribution for height constraints  
✅ **Memory Safety** - Built-in protection against dimension attacks and malicious input

```rust
use lipgloss::{Style, Color, rounded_border, CENTER};

fn main() {
    // Create a solid colored block with automatic padding and borders
    let style = Style::new()
        .bold(true)
        .foreground(Color::from("#FAFAFA"))
        .background(Color::from("#7D56F4"))
        .padding(1, 2, 1, 2)
        .width(30)                    // Minimum width constraint
        .height(5)                    // Minimum height constraint  
        .align_horizontal(CENTER)     // Center content within width
        .align_vertical(CENTER)       // Center content within height
        .border_style(rounded_border())
        .border(true, true, true, true)
        .color_whitespace(true);      // Fill background in whitespace

    println!("{}", style.render("Hello, kitty."));
    // Creates a solid 30x5 purple block with white text centered inside
}
```

## Installation

Add `lipgloss` and its component crates to your `Cargo.toml`:

```toml
[dependencies]
lipgloss = "0.0.6"
lipgloss-list = "0.0.6"
lipgloss-table = "0.0.6"
lipgloss-tree = "0.0.6"
```

## Colors

`lipgloss-rs` supports the following color profiles:

### ANSI 16 colors (4-bit)

```rust
use lipgloss::Color;

let magenta = Color::from("5");
let red = Color::from("9");
let light_blue = Color::from("12");
```

### ANSI 256 Colors (8-bit)

```rust
use lipgloss::Color;

let aqua = Color::from("86");
let hot_pink = Color::from("201");
let orange = Color::from("202");
```

### True Color (16,777,216 colors; 24-bit)

```rust
use lipgloss::Color;

let blue = Color::from("#0000FF");
let green = Color::from("#04B575");
let dark_gray = Color::from("#3C3C3C");
```

...as well as a 1-bit ASCII profile, which is black and white only.

The terminal's color profile will be automatically detected, and colors outside the gamut of the current palette will be automatically coerced to their closest available value.

### Adaptive Colors

You can also specify color options for light and dark backgrounds:

```rust
use lipgloss::AdaptiveColor;

let color = AdaptiveColor::new("#0000FF", "#000099"); // Light, Dark
```

The terminal's background color will automatically be detected and the appropriate color will be chosen at runtime.

### Gradients & Color Interpolation

`lipgloss-rs` includes advanced gradient capabilities using perceptually uniform color spaces for smooth, visually appealing transitions:

#### Basic Gradients

![Linear 1D Gradient](examples/blending/linear-1d/standalone/demo.png)

```rust
use lipgloss::{gradient, Style};

// Create smooth color gradients
let colors = gradient("#FF0000", "#0000FF", 10);
for color in colors {
    let block = Style::new()
        .set_string("██")
        .background(color)
        .render("");
    print!("{}", block);
}
```

#### 2D Bilinear Interpolation

![Linear 2D Gradient](examples/blending/linear-2d/standalone/demo.png)

```rust
use lipgloss::bilinear_interpolation_grid;

// 2D color grids with bilinear interpolation
let grid = bilinear_interpolation_grid(
    8, 4,  // 8 columns, 4 rows
    ("#FF0000", "#00FF00", "#0000FF", "#FFFF00")  // corner colors
);
```

#### Advanced Gradient Effects

![Comprehensive Blending Demo](examples/blending/comprehensive-demo/demo.png)

```rust
use lipgloss::{gradient, Style};

// Gradient text effects
let text_colors = gradient("#FF6B6B", "#4ECDC4", 20);
let text = "Rainbow Text Effect!";
let mut result = String::new();

for (i, ch) in text.chars().enumerate() {
    let color = &text_colors[i % text_colors.len()];
    let styled_char = Style::new()
        .foreground(color.clone())
        .render(&ch.to_string());
    result.push_str(&styled_char);
}
```

#### Brightness and Color Mixing

![Brightness Demo](examples/blending/brightness/demo.png)

```rust
use lipgloss::{Style, Color};

// Brightness adjustments and color mixing
let base_color = Color::from("#7C3AED");
let bright_style = Style::new()
    .background(base_color.lighten(0.3))
    .foreground(Color::from("#FFFFFF"));
```

### Complete Colors

`CompleteColor` specifies exact values for True Color, ANSI256, and ANSI color profiles.

```rust
use lipgloss::CompleteColor;

let color = CompleteColor {
    true_color: "#0000FF".to_string(),
    ansi256: "86".to_string(),
    ansi: "5".to_string(),
};
```

Automatic color degradation will not be performed in this case and it will be based on the color specified.

### Complete Adaptive Colors

You can use `CompleteColor` with `AdaptiveColor` to specify the exact values for light and dark backgrounds without automatic color degradation.

```rust
use lipgloss::CompleteAdaptiveColor;

let color = CompleteAdaptiveColor {
    light: CompleteColor {
        true_color: "#d7ffae".to_string(),
        ansi256: "193".to_string(),
        ansi: "11".to_string(),
    },
    dark: CompleteColor {
        true_color: "#d75fee".to_string(),
        ansi256: "163".to_string(),
        ansi: "5".to_string(),
    },
};
```

## Inline Formatting

`lipgloss-rs` supports the usual ANSI text formatting options:

```rust
use lipgloss::Style;

let style = Style::new()
    .bold(true)
    .italic(true)
    .faint(true)
    .blink(true)
    .strikethrough(true)
    .underline(true)
    .reverse(true);
```

## Block-Level Formatting

`lipgloss-rs` also supports rules for block-level formatting:

```rust
use lipgloss::Style;

// Padding
let style = Style::new()
    .padding_top(2)
    .padding_right(4)
    .padding_bottom(2)
    .padding_left(4);

// Margins
let style = Style::new()
    .margin_top(2)
    .margin_right(4)
    .margin_bottom(2)
    .margin_left(4);```

There are also shorthand methods for margins and padding, which follow a similar format to CSS:

```rust
// 2 cells on all sides
Style::new().padding(2, 2, 2, 2);

// 2 cells on the top and bottom, 4 cells on the left and right
Style::new().margin(2, 4, 2, 4);

// 1 cell on the top, 4 cells on the sides, 2 cells on the bottom
Style::new().padding(1, 4, 2, 4);

// Clockwise, starting from the top: 2 cells on the top, 4 on the right, 3 on
// the bottom, and 1 on the left
Style::new().margin(2, 4, 3, 1);
```

## Aligning Text

You can align paragraphs of text to the left, right, or center.

```rust
use lipgloss::{Style, LEFT, RIGHT, CENTER};

let style = Style::new()
    .width(24)
    .align_horizontal(LEFT)   // align it left
    .align_horizontal(RIGHT)  // no wait, align it right
    .align_horizontal(CENTER); // just kidding, align it in the center
```

## Width and Height

Setting a minimum width and height is simple and straightforward.

```rust
use lipgloss::{Style, Color};

let style = Style::new()
    .set_string("What's for lunch?")
    .width(24)
    .height(32)
    .foreground(Color::from("63"));
```

## Perfect Text Alignment (Go Parity)

Text alignment now works line-by-line for precise visual positioning, exactly like the Go implementation.

```rust
use lipgloss::{Style, Color, LEFT, RIGHT, CENTER, TOP, BOTTOM};

// Horizontal alignment - applied to each wrapped line individually
let horizontal_demo = Style::new()
    .width(30)
    .background(Color::from("#E8E8E8"))
    .foreground(Color::from("#333333"))
    .align_horizontal(CENTER)
    .color_whitespace(true);

let text = "This is a long line that will wrap\nand each line will be centered\nindividually within the width";
println!("{}", horizontal_demo.render(text));

// Vertical alignment - distributes padding lines based on position
let vertical_demo = Style::new()
    .width(25)
    .height(8)
    .background(Color::from("#FF6B6B"))
    .foreground(Color::from("#FFFFFF"))
    .align_vertical(BOTTOM)        // Content positioned at bottom
    .align_horizontal(RIGHT)       // Content aligned right
    .color_whitespace(true);

println!("{}", vertical_demo.render("Bottom-right\naligned content"));

// Combined alignment with borders
let perfect_alignment = Style::new()
    .width(35)
    .height(7)
    .align_horizontal(CENTER)
    .align_vertical(CENTER)
    .border_style(lipgloss::rounded_border())
    .border(true, true, true, true)
    .border_foreground(Color::from("63"))
    .background(Color::from("#F0F0F0"))
    .color_whitespace(true);

println!("{}", perfect_alignment.render("Perfectly centered\nmulti-line content\nwith auto-extending borders"));
```

## Security & Performance (Rust Advantages)

The Rust implementation includes built-in safety features and performance optimizations beyond the original Go version:

```rust
use lipgloss::{Style, Color, security::validate_dimension};

// Automatic dimension validation prevents memory exhaustion attacks
let safe_style = Style::new()
    .width(user_width)      // Automatically clamped to safe limits (max 10,000)
    .height(user_height)    // Automatically clamped to safe limits (max 10,000)
    .background(Color::from("#FF6B6B"));

// Built-in memory budget enforcement (50MB limit)
let output = safe_style.render(&potentially_large_content);

// Optional explicit validation for extra safety
let validated_width = validate_dimension(untrusted_input, "width");
let extra_safe = Style::new().width(validated_width);
```

## Enforcing Rules

Sometimes, such as when developing a component, you want to make sure style definitions respect their intended purpose in the UI. This is where `inline` and `max_width`, and `max_height` come in:

```rust
use lipgloss::Style;

// Force rendering onto a single line, ignoring margins, padding, and borders.
let some_style = Style::new().inline(true);
println!("{}", some_style.render("yadda yadda"));

// Also limit rendering to five cells
let constrained_style = Style::new().inline(true).max_width(5);
println!("{}", constrained_style.render("yadda yadda"));

// Limit rendering to a 5x5 cell block
let block_style = Style::new().max_width(5).max_height(5);
println!("{}", block_style.render("yadda yadda"));
```

**Key Safety Features:**
- 🛡️ **Dimension validation** - Prevents memory exhaustion from large width/height values
- 🔒 **Safe string operations** - Bounded repetition functions prevent runaway allocations  
- 🚫 **ANSI sequence limits** - Protection against malicious escape sequences
- ⚡ **Performance optimized** - 10-100x faster style comparisons, efficient rendering pipeline

## Tabs

The tab character (`\t`) is rendered differently in different terminals (often as 8 spaces, sometimes 4). Because of this inconsistency, `lipgloss-rs` converts tabs to 4 spaces at render time. This behavior can be changed on a per-style basis, however:

```rust
use lipgloss::{Style, NO_TAB_CONVERSION};

let style = Style::new(); // tabs will render as 4 spaces, the default
let style = style.tab_width(2);    // render tabs as 2 spaces
let style = style.tab_width(0);    // remove tabs entirely
let style = style.tab_width(NO_TAB_CONVERSION); // leave tabs intact
```

## Smart Borders (Auto-Extending)

Borders now automatically extend to encompass the full constrained dimensions, including automatic padding:

```rust
use lipgloss::{normal_border, rounded_border, thick_border, block_border, Border, Style, Color, CENTER};

// Add a purple, rectangular border that auto-extends to width/height
let style = Style::new()
    .border_style(normal_border())
    .border(true, true, true, true)
    .border_foreground(Color::from("63"))
    .width(25)                    // Border extends to full width
    .height(5);                   // Border extends to full height

// Rounded border with background and alignment
let fancy_style = Style::new()
    .border_style(rounded_border())
    .border(true, true, true, true)
    .border_foreground(Color::from("228"))
    .background(Color::from("63"))
    .width(30)
    .height(6)
    .align_horizontal(CENTER)
    .align_vertical(CENTER)
    .color_whitespace(true);      // Solid background within border

println!("{}", fancy_style.render("Centered with\nauto-extending border"));

// Partial borders (top and left only) still respect constraints
let partial_border = Style::new()
    .border_style(thick_border())
    .border_top(true)
    .border_left(true)
    .border_foreground(Color::from("201"))
    .width(20)
    .height(4);

// Make your own custom border
let my_cute_border = Border::new(
    "._.:*:", "._.:*:", "|*", "|*", "*", "*", "*", "*",
    "*", "*", "*", "*", "*",
);

let custom_style = Style::new()
    .border_style(my_cute_border)
    .border(true, true, true, true)
    .width(22)
    .height(3);
```

There are also shorthand functions for defining borders, which follow a similar pattern to the margin and padding shorthand functions.

```rust
use lipgloss::{Style, thick_border, double_border};

// Add a thick border to the top and bottom
let style = Style::new()
    .border_style(thick_border())
    .border_top(true)
    .border_bottom(true);

// Add a double border to the top and left sides. Rules are set clockwise from top.
let style = Style::new()
    .border_style(double_border())
    .border_top(true)
    .border_right(false)
    .border_bottom(false)
    .border_left(true);
```

## Copying Styles

The builder pattern for `Style` returns a new, modified copy with each call. For an explicit copy, just use `.clone()`:

```rust
use lipgloss::{Style, Color};

let style = Style::new().foreground(Color::from("219"));

let copied_style = style.clone(); // this is a true copy

let wild_style = style.blink(true); // this is also a new copy, with blink added
```

## Inheritance

Styles can inherit rules from other styles. When inheriting, only unset rules on the receiver are inherited.

```rust
use lipgloss::{Style, Color};

let style_a = Style::new()
    .foreground(Color::from("229"))
    .background(Color::from("63"));

// Only the background color will be inherited here, because the foreground
// color will have been already set:
let style_b = Style::new()
    .foreground(Color::from("201"))
    .inherit(style_a);
```

## Unsetting Rules

All rules can be unset:

```rust
use lipgloss::{Style, Color};

let style = Style::new()
    .bold(true)         // make it bold
    .unset_bold()       // jk don't make it bold
    .background(Color::from("227")) // yellow background
    .unset_background(); // never mind
```

## Rendering

Generally, you just call the `render(&str)` method on a `lipgloss::Style`:

```rust
use lipgloss::Style;

let style = Style::new().bold(true).set_string("Hello,");
println!("{}", style.render("kitty.")); // Hello, kitty.
println!("{}", style.render("puppy.")); // Hello, puppy.
```

### Custom Renderers

Custom renderers allow you to render to a specific outputs. This is particularly important when you want to render to different outputs and correctly detect the color profile and dark background status for each, such as in a server-client situation.

```rust
use lipgloss::{Renderer, AdaptiveColor, Style};

fn my_little_handler(session: &mut SshSession) {
    // Create a renderer for the client.
    let renderer = Renderer::new(session);

    // Create a new style on the renderer.
    let style = renderer.new_style()
        .background(AdaptiveColor::new("63", "228"));

    // Render. The color profile and dark background state will be correctly detected.
    session.write_string(&style.render("Heyyyyyyy"));
}
```

## Utilities

In addition to pure styling, `lipgloss-rs` also ships with some utilities to help assemble your layouts.

### Joining Paragraphs

Horizontally and vertically joining paragraphs is a cinch.

```rust
use lipgloss::{join_horizontal, join_vertical, Position};

let paragraph_a = "First";
let paragraph_b = "Second\nLine";
let paragraph_c = "Third";

// Horizontally join three paragraphs along their bottom edges
join_horizontal(Position::bottom(), &[paragraph_a, paragraph_b, paragraph_c]);

// Vertically join two paragraphs along their center axes
join_vertical(Position::center(), &[paragraph_a, paragraph_b]);

// Horizontally join three paragraphs, with the shorter ones aligning 20%
// from the top of the tallest
join_horizontal(Position::relative(0.2), &[paragraph_a, paragraph_b, paragraph_c]);
```

### Measuring Width and Height

Sometimes you’ll want to know the width and height of text blocks when building your layouts.

```rust
use lipgloss::{width, height, size, Style};

// Render a block of text.
let style = Style::new()
    .width(40)
    .padding(2, 2, 2, 2);
let block = style.render("some long string");

// Get the actual, physical dimensions of the text block.
let w = width(&block);
let h = height(&block);

// Here's a shorthand function.
let (w, h) = size(&block);
```

### Placing Text in Whitespace

Sometimes you’ll simply want to place a block of text in whitespace.

```rust
use lipgloss::{place, place_horizontal, place_vertical, Position};

let fancy_styled_paragraph = "I'm fancy.";

// Center a paragraph horizontally in a space 80 cells wide.
let block = place_horizontal(80, Position::center(), fancy_styled_paragraph, &[]);

// Place a paragraph at the bottom of a space 30 cells tall.
let block = place_vertical(30, Position::bottom(), fancy_styled_paragraph, &[]);

// Place a paragraph in the bottom right corner of a 30x80 cell space.
let block = place(30, 80, Position::right(), Position::bottom(), fancy_styled_paragraph, &[]);
```

## Rendering Tables

`lipgloss-rs` ships with a table rendering sub-package, `lipgloss-table`.

```rust
use lipgloss::{Style, Color, Position, thick_border};
use lipgloss_table::{Table, HEADER_ROW};

let purple = Color::from("99");
let gray = Color::from("245");
let light_gray = Color::from("241");

let rows = vec![
    vec!["Chinese", "您好", "你好"],
    vec!["Japanese", "こんにちは", "やあ"],
];

let style_func = move |row: i32, col: usize| -> Style {
    // Styles...
    // ...
    Style::new() // placeholder
};

let mut t = Table::new()
    .border(thick_border())
    .border_style(Style::new().foreground(purple))
    .style_func_boxed(style_func)
    .headers(vec!["LANGUAGE", "FORMAL", "INFORMAL"])
    .rows(rows);

t = t.row(vec!["English", "You look absolutely fabulous.", "How's it going?"]);
println!("{}", t);
```

![Table Example]

### Table Borders

There are helpers to generate tables in markdown or ASCII style:

#### Markdown Table

```rust
use lipgloss::{Style, markdown_border};
use lipgloss_table::Table;

let t = Table::new()
    .border_style(markdown_border())
    .border_top(false)
    .border_bottom(false);
```

```
| LANGUAGE |    FORMAL    | INFORMAL  |
|----------|--------------|-----------| 
| Chinese  | Nǐn hǎo      | Nǐ hǎo    |
| French   | Bonjour      | Salut     |
| Russian  | Zdravstvuyte | Privet    |
| Spanish  | Hola         | ¿Qué tal? |
```

#### ASCII Table

```rust 
use lipgloss::{Style, ascii_border};
use lipgloss_table::Table;

let t = Table::new().border_style(ascii_border());
```

```
+----------+--------------+-----------+
| LANGUAGE |    FORMAL    | INFORMAL  |
+----------+--------------+-----------+
| Chinese  | Nǐn hǎo      | Nǐ hǎo    |
| French   | Bonjour      | Salut     |
| Russian  | Zdravstvuyte | Privet    |
| Spanish  | Hola         | ¿Qué tal? |
+----------+--------------+-----------+
```

## Rendering Lists

`lipgloss-rs` ships with a list rendering sub-package, `lipgloss-list`.

```rust
use lipgloss_list::{List, roman};

let l = List::new()
    .item("A")
    .item_list(
        List::new()
            .item("Artichoke")
    )
    .item("B");

println!("{}", l);
```

## Rendering Trees

`lipgloss-rs` ships with a tree rendering sub-package, `lipgloss-tree`.

```rust
use lipgloss_tree::{Leaf, Node, Tree};

let t = Tree::new().root(".")
    .child(vec![
        Box::new(Leaf::new("macOS", false)) as Box<dyn Node>,
        Box::new(
            Tree::new().root("Linux")
                .child(vec![
                    Box::new(Leaf::new("NixOS", false)) as Box<dyn Node>,
                ])
        ) as Box<dyn Node>,
    ]);

println!("{}", t);
```

## Running the Examples

`lipgloss-rs` comes with several demo applications that showcase the library's capabilities:

```bash
# Layout demo - comprehensive styling showcase
cargo run --bin layout-demo

# List rendering demo
cargo run --bin list-demo

# Simple styling example
cargo run --bin simple-style

# Table rendering demo
cargo run --bin table-demo

# Tree rendering demo
cargo run --bin tree-demo
```

Each demo demonstrates different aspects of the library, from basic styling to complex layouts with tables, lists, and trees.

## What about `bubbletea-rs`?

`lipgloss-rs` doesn't replace `bubbletea-rs`. Rather, it is an excellent companion. It was designed to make assembling terminal user interface views as simple and fun as possible so that you can focus on building your application instead of concerning yourself with low-level layout details.

In simple terms, you can use `lipgloss-rs` to help build your views in [`bubbletea-rs`](https://github.com/whit3rabbit/bubbletea-rs), another Rust port from the Charm ecosystem.

## Under the Hood

`lipgloss-rs` achieves exact Go parity through a carefully architected **"Layout First, Styling Second"** rendering pipeline:

1. **Layout First**: Width/height constraints create the full-size canvas with alignment padding
2. **Styling Second**: Colors and attributes are applied to the entire canvas for solid blocks
3. **Smart Borders**: Borders automatically encompass the complete constrained dimensions
4. **Margin Inheritance**: Margins inherit background colors when not explicitly set

The implementation is built on `crossterm`, `unicode-width`, and `palette` libraries for terminal manipulation, ANSI-aware text operations, and advanced color management with perceptually uniform interpolation.

## Acknowledgements

`lipgloss-rs` is a port of the original Go library, [**lipgloss**](https://github.com/charmbracelet/lipgloss), created by the amazing team at [**Charm**](https://charm.sh). All credit for the brilliant API design and functionality goes to them.

## License

[MIT](https://github.com/whit3rabbit/lipgloss-rs/blob/main/LICENSE)