//! Facade crate for the lipgloss Rust ecosystem.
//!
//! - Always re-exports `lipgloss` as `lipgloss_extras::lipgloss`.
//! - Optional feature-gated re-exports:
//!   - `lists`  → `lipgloss_extras::list`
//!   - `trees`  → `lipgloss_extras::tree`
//!   - `tables` → `lipgloss_extras::table`
//! - `full` enables all of the above.
//!
//! Example:
//! ```toml
//! [dependencies]
//! lipgloss-extras = { version = "0.0.7", features = ["full"] }
//! ```
//!
//! ```rust
//! use lipgloss_extras::lipgloss::{Style, Color};
//! #[cfg(feature = "lists")]
//! use lipgloss_extras::list::List;
//! ```

pub use lipgloss;

#[cfg(feature = "lists")]
pub use lipgloss_list as list;

#[cfg(feature = "trees")]
pub use lipgloss_tree as tree;

#[cfg(feature = "tables")]
pub use lipgloss_table as table;

/// Commonly-used items re-exported for convenience.
#[allow(ambiguous_glob_reexports)]
pub mod prelude {
    pub use crate::lipgloss::{self, *};

    #[cfg(feature = "lists")]
    pub use crate::list::*;

    #[cfg(feature = "trees")]
    pub use crate::tree::*;

    #[cfg(feature = "tables")]
    pub use crate::table::*;
}

#[cfg(test)]
mod tests {
    use super::lipgloss::Style;

    #[test]
    fn core_exists() {
        let _ = Style::new();
    }

    #[cfg(feature = "lists")]
    #[test]
    fn lists_exists() {
        let _ = crate::list::List::new();
    }

    #[cfg(feature = "trees")]
    #[test]
    fn trees_exists() {
        let _ = crate::tree::Tree::new();
    }

    #[cfg(feature = "tables")]
    #[test]
    fn tables_exists() {
        let _ = crate::table::Table::new();
    }
}
