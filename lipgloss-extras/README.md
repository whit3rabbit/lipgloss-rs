# lipgloss-extras

Facade crate for the `lipgloss` Rust ecosystem.

- Always re-exports `lipgloss` as `lipgloss_extras::lipgloss`.
- Optional feature-gated re-exports:
  - `lists`  → `lipgloss_extras::list`
  - `trees`  → `lipgloss_extras::tree`
  - `tables` → `lipgloss_extras::table`
- `full` enables all of the above.

## Usage

```toml
[dependencies]
# Everything:
lipgloss-extras = { version = "0.0.7", features = ["full"] }

# or cherry-pick:
# lipgloss-extras = { version = "0.0.7", features = ["lists", "tables"] }
```

```rust
use lipgloss_extras::lipgloss::{Style, Color};

#[cfg(feature = "lists")]
use lipgloss_extras::list::List;

fn main() {
    let s = Style::new().foreground(Color::from("#ff6b6b"));
    println!("{}", s.render("hello"));
}
```


