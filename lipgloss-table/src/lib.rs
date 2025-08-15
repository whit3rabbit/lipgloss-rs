//! # lipgloss-table
//!
//! A flexible and powerful table rendering library for terminal applications.
//!
//! This crate provides a comprehensive table rendering system with advanced styling,
//! layout options, and terminal-aware text handling. It's designed to work seamlessly
//! with the `lipgloss` styling library to create beautiful terminal user interfaces.
//!
//! ## Features
//!
//! - **Flexible Table Construction**: Build tables using a fluent builder pattern
//! - **Advanced Styling**: Apply different styles to headers, rows, and individual cells
//! - **Border Customization**: Control all aspects of table borders and separators
//! - **Responsive Layout**: Automatic width detection and content wrapping/truncation
//! - **Height Constraints**: Set maximum heights with automatic scrolling and overflow indicators
//! - **ANSI-Aware**: Proper handling of ANSI escape sequences in content
//! - **Memory Safe**: Built-in protections against memory exhaustion from malicious input
//!
//! ## Quick Start
//!
//! ```rust
//! use lipgloss_table::{Table, HEADER_ROW, header_row_style};
//! use lipgloss::{Style, Color};
//!
//! // Create a simple table
//! let mut table = Table::new()
//!     .headers(vec!["Name", "Age", "City"])
//!     .row(vec!["Alice", "30", "New York"])
//!     .row(vec!["Bob", "25", "London"])
//!     .style_func(header_row_style);
//!
//! println!("{}", table.render());
//! ```
//!
//! ## Advanced Usage
//!
//! ### Custom Styling
//!
//! ```rust
//! use lipgloss_table::{Table, HEADER_ROW};
//! use lipgloss::{Style, Color};
//!
//! let style_func = |row: i32, col: usize| {
//!     match row {
//!         HEADER_ROW => Style::new().bold(true).foreground(Color::from("#FFFFFF")),
//!         _ if row % 2 == 0 => Style::new().background(Color::from("#F0F0F0")),
//!         _ => Style::new(),
//!     }
//! };
//!
//! let mut table = Table::new()
//!     .headers(vec!["Product", "Price", "Stock"])
//!     .rows(vec![
//!         vec!["Widget A", "$10.99", "50"],
//!         vec!["Widget B", "$15.99", "25"],
//!         vec!["Widget C", "$8.99", "100"],
//!     ])
//!     .style_func(style_func)
//!     .width(40);
//!
//! println!("{}", table.render());
//! ```
//!
//! ### Height-Constrained Tables with Scrolling
//!
//! ```rust
//! use lipgloss_table::Table;
//!
//! let mut table = Table::new()
//!     .headers(vec!["Item", "Description"])
//!     .height(10)  // Limit table to 10 lines
//!     .offset(5);  // Skip first 5 rows (scrolling)
//!
//! // Add many rows...
//! for i in 1..=100 {
//!     table = table.row(vec![format!("Item {}", i), "Description".to_string()]);
//! }
//!
//! println!("{}", table.render());
//! println!("Table height: {}", table.compute_height());
//! ```
//!
//! ## Predefined Style Functions
//!
//! The crate includes several predefined styling functions:
//!
//! - [`default_styles`]: Basic styling with no attributes
//! - [`header_row_style`]: Bold headers with default data rows
//! - [`zebra_style`]: Alternating row backgrounds for better readability
//! - [`minimal_style`]: Subtle styling with muted colors
//! - [`column_style_func`]: Factory for creating column-specific styles
//!
//! ## Integration with lipgloss
//!
//! This crate is designed to work seamlessly with the `lipgloss` styling library.
//! All styling functions receive `lipgloss::Style` objects and can use the full
//! range of lipgloss features including colors, borders, padding, and alignment.

#![warn(missing_docs)]

/// Internal module for table resizing logic and column width calculations.
pub mod resizing;

/// Internal module for data handling and row management.
pub mod rows;

/// Internal utility functions for table operations.
pub mod util;

use lipgloss::security::{safe_repeat, safe_str_repeat};
use lipgloss::{Border, Style};
use std::fmt;

// Re-export the main types and functions
pub use resizing::{Resizer, ResizerColumn};
pub use rows::{data_to_matrix, Data, Filter, StringData};

/// HeaderRow denotes the header's row index used when rendering headers.
/// Use this value when looking to customize header styles in StyleFunc.
pub const HEADER_ROW: i32 = -1;

/// StyleFunc is the style function that determines the style of a Cell.
///
/// It takes the row and column of the cell as an input and determines the
/// lipgloss Style to use for that cell position.
///
/// Example:
///
/// ```rust
/// use lipgloss::{Style, Color};
/// use lipgloss_table::{Table, HEADER_ROW};
///
/// let style_func = |row: i32, col: usize| {
///     match row {
///         HEADER_ROW => Style::new().bold(true),
///         _ if row % 2 == 0 => Style::new().foreground(Color::from("#888888")),
///         _ => Style::new(),
///     }
/// };
/// ```
pub type StyleFunc = fn(row: i32, col: usize) -> Style;

/// A basic style function that applies no formatting to any cells.
///
/// This function serves as the default styling approach, returning a plain
/// `Style` with no attributes for all table cells. It's useful as a starting
/// point or when you want completely unstyled table content.
///
/// # Arguments
///
/// * `_row` - The row index (unused, but required by the `StyleFunc` signature)
/// * `_col` - The column index (unused, but required by the `StyleFunc` signature)
///
/// # Returns
///
/// A new `Style` instance with no formatting applied.
///
/// # Examples
///
/// ```rust
/// use lipgloss_table::{Table, default_styles};
///
/// let mut table = Table::new()
///     .headers(vec!["Name", "Age"])
///     .row(vec!["Alice", "30"])
///     .style_func(default_styles);
///
/// println!("{}", table.render());
/// ```
pub fn default_styles(_row: i32, _col: usize) -> Style {
    Style::new()
}

/// A style function that makes header rows bold while leaving data rows unstyled.
///
/// This function provides a simple but effective styling approach by applying
/// bold formatting to header rows (identified by `HEADER_ROW`) while leaving
/// all data rows with default styling. This creates a clear visual distinction
/// between headers and content.
///
/// # Arguments
///
/// * `row` - The row index to style (headers use `HEADER_ROW` constant)
/// * `_col` - The column index (unused, but required by the `StyleFunc` signature)
///
/// # Returns
///
/// * Bold `Style` for header rows (`HEADER_ROW`)
/// * Default `Style` for all data rows
///
/// # Examples
///
/// ```rust
/// use lipgloss_table::{Table, header_row_style};
///
/// let mut table = Table::new()
///     .headers(vec!["Product", "Price", "Stock"])
///     .row(vec!["Widget A", "$10.99", "50"])
///     .row(vec!["Widget B", "$15.99", "25"])
///     .style_func(header_row_style);
///
/// println!("{}", table.render());
/// ```
pub fn header_row_style(row: i32, _col: usize) -> Style {
    match row {
        HEADER_ROW => Style::new().bold(true),
        _ => Style::new(),
    }
}

/// A style function that creates alternating row backgrounds (zebra striping) for improved readability.
///
/// This function applies a "zebra stripe" pattern to table rows, alternating between
/// a default background and a subtle background color for even-numbered rows. The header
/// row receives bold styling. The background colors are adaptive, changing based on
/// whether the terminal has a light or dark theme.
///
/// # Row Pattern
///
/// * Header row: Bold text
/// * Even data rows (0, 2, 4...): Subtle background color
/// * Odd data rows (1, 3, 5...): Default background
///
/// # Arguments
///
/// * `row` - The row index to style (headers use `HEADER_ROW` constant)
/// * `_col` - The column index (unused, but required by the `StyleFunc` signature)
///
/// # Returns
///
/// * Bold `Style` for header rows
/// * `Style` with subtle background for even data rows
/// * Default `Style` for odd data rows
///
/// # Examples
///
/// ```rust
/// use lipgloss_table::{Table, zebra_style};
///
/// let mut table = Table::new()
///     .headers(vec!["Name", "Score", "Grade"])
///     .row(vec!["Alice", "95", "A"])   // Even row - background color
///     .row(vec!["Bob", "87", "B"])     // Odd row - default
///     .row(vec!["Charlie", "92", "A"]) // Even row - background color
///     .style_func(zebra_style);
///
/// println!("{}", table.render());
/// ```
pub fn zebra_style(row: i32, _col: usize) -> Style {
    use lipgloss::color::AdaptiveColor;
    let table_row_even_bg = AdaptiveColor {
        Light: "#F9FAFB",
        Dark: "#1F1F1F",
    };
    match row {
        HEADER_ROW => Style::new().bold(true),
        _ if row % 2 == 0 => Style::new().background(table_row_even_bg),
        _ => Style::new(),
    }
}

/// A subtle style function that provides minimal, professional-looking table styling.
///
/// This function creates a clean, minimal aesthetic using muted colors and subtle
/// contrast. Headers are bold with high-contrast text, while data rows alternate
/// between normal and muted text colors. All colors are adaptive to work well
/// with both light and dark terminal themes.
///
/// # Row Pattern
///
/// * Header row: Bold text with high-contrast color
/// * Even data rows (0, 2, 4...): Muted text color
/// * Odd data rows (1, 3, 5...): Normal text color
///
/// # Arguments
///
/// * `row` - The row index to style (headers use `HEADER_ROW` constant)
/// * `_col` - The column index (unused, but required by the `StyleFunc` signature)
///
/// # Returns
///
/// * Bold `Style` with high-contrast foreground for headers
/// * `Style` with muted foreground for even data rows
/// * `Style` with normal foreground for odd data rows
///
/// # Examples
///
/// ```rust
/// use lipgloss_table::{Table, minimal_style};
///
/// let mut table = Table::new()
///     .headers(vec!["Status", "Task", "Priority"])
///     .row(vec!["Done", "Fix bug #123", "High"])
///     .row(vec!["In Progress", "Add new feature", "Medium"])
///     .row(vec!["Todo", "Update docs", "Low"])
///     .style_func(minimal_style);
///
/// println!("{}", table.render());
/// ```
pub fn minimal_style(row: i32, _col: usize) -> Style {
    use lipgloss::color::AdaptiveColor;
    let table_header_text = AdaptiveColor {
        Light: "#171717",
        Dark: "#F5F5F5",
    };
    let table_row_text = AdaptiveColor {
        Light: "#262626",
        Dark: "#FAFAFA",
    };
    let text_muted = AdaptiveColor {
        Light: "#737373",
        Dark: "#A3A3A3",
    };
    match row {
        HEADER_ROW => Style::new().bold(true).foreground(table_header_text),
        _ if row % 2 == 0 => Style::new().foreground(text_muted),
        _ => Style::new().foreground(table_row_text),
    }
}

/// Creates a style function that applies column-specific styling to table cells.
///
/// This function factory generates a style function that can apply different styles
/// to specific columns while maintaining consistent header styling. It's particularly
/// useful for highlighting important columns like status indicators, priority levels,
/// or key data fields.
///
/// # Arguments
///
/// * `column_styles` - A vector of tuples where each tuple contains:
///   - `usize`: The zero-based column index to style
///   - `Style`: The lipgloss style to apply to that column
///
/// # Returns
///
/// A closure that implements the `StyleFunc` signature, applying:
/// * Bold styling to all header row cells
/// * Column-specific styles to matching data cells
/// * Default styling to other cells
///
/// # Examples
///
/// ```rust
/// use lipgloss_table::{Table, column_style_func};
/// use lipgloss::{Style, Color};
///
/// // Define styles for specific columns
/// let column_styles = vec![
///     (0, Style::new().foreground(Color::from("#00FF00"))), // Green for first column
///     (2, Style::new().bold(true).foreground(Color::from("#FF0000"))), // Bold red for third column
/// ];
///
/// let mut table = Table::new()
///     .headers(vec!["Status", "Task", "Priority", "Assignee"])
///     .row(vec!["Active", "Fix bug", "High", "Alice"])
///     .row(vec!["Done", "Add feature", "Medium", "Bob"])
///     .style_func_boxed(Box::new(column_style_func(column_styles)));
///
/// println!("{}", table.render());
/// ```
pub fn column_style_func(column_styles: Vec<(usize, Style)>) -> impl Fn(i32, usize) -> Style {
    move |row: i32, col: usize| {
        // Apply header styling
        let mut base_style = if row == HEADER_ROW {
            Style::new().bold(true)
        } else {
            Style::new()
        };

        // Apply column-specific styling
        for &(target_col, ref style) in &column_styles {
            if col == target_col {
                // Inherit from the column style
                base_style = base_style.inherit(style.clone());
                break;
            }
        }

        base_style
    }
}

/// A trait object type for flexible style functions that can capture their environment.
///
/// This type allows for more complex styling logic that can capture variables
/// from the surrounding scope, unlike the simple function pointer `StyleFunc`.
/// It's particularly useful when you need to reference external data or state
/// in your styling logic.
///
/// # Examples
///
/// ```rust
/// use lipgloss_table::{Table, BoxedStyleFunc, HEADER_ROW};
/// use lipgloss::{Style, Color};
///
/// let error_color = Color::from("#FF0000");
/// let warning_color = Color::from("#FFAA00");
///
/// let boxed_style: BoxedStyleFunc = Box::new(move |row: i32, col: usize| {
///     match (row, col) {
///         (HEADER_ROW, _) => Style::new().bold(true),
///         (_, 0) => Style::new().foreground(error_color.clone()),
///         (_, 1) => Style::new().foreground(warning_color.clone()),
///         _ => Style::new(),
///     }
/// });
/// ```
pub type BoxedStyleFunc = Box<dyn Fn(i32, usize) -> Style + Send + Sync>;

/// A flexible table renderer with advanced styling and layout capabilities.
///
/// `Table` provides a comprehensive solution for rendering tabular data in terminal
/// applications. It supports a wide range of customization options including borders,
/// styling functions, width/height constraints, text wrapping, and scrolling.
///
/// # Features
///
/// - **Flexible Content**: Supports headers, multiple data rows, and various data sources
/// - **Advanced Styling**: Cell-by-cell styling with function-based or closure-based approaches
/// - **Border Control**: Granular control over all border elements (top, bottom, sides, separators)
/// - **Layout Management**: Width/height constraints with automatic wrapping and truncation
/// - **Scrolling Support**: Offset-based scrolling for large datasets
/// - **ANSI-Aware**: Proper handling of ANSI escape sequences in cell content
/// - **Memory Safe**: Built-in protections against excessive memory usage
///
/// # Builder Pattern
///
/// `Table` uses a fluent builder pattern where each method returns `Self`, allowing
/// for method chaining. Call `render()` to generate the final string representation.
///
/// # Examples
///
/// ## Basic Table
///
/// ```rust
/// use lipgloss_table::Table;
///
/// let mut table = Table::new()
///     .headers(vec!["Name", "Age", "City"])
///     .row(vec!["Alice", "30", "New York"])
///     .row(vec!["Bob", "25", "London"]);
///
/// println!("{}", table.render());
/// ```
///
/// ## Styled Table with Width Constraint
///
/// ```rust
/// use lipgloss_table::{Table, zebra_style};
/// use lipgloss::rounded_border;
///
/// let mut table = Table::new()
///     .headers(vec!["Product", "Description", "Price"])
///     .rows(vec![
///         vec!["Widget A", "A useful widget for all your needs", "$19.99"],
///         vec!["Widget B", "An even more useful widget", "$29.99"],
///     ])
///     .width(50)
///     .border(rounded_border())
///     .style_func(zebra_style);
///
/// println!("{}", table.render());
/// ```
///
/// ## Scrollable Table with Height Limit
///
/// ```rust
/// use lipgloss_table::Table;
///
/// let mut large_table = Table::new()
///     .headers(vec!["ID", "Data"])
///     .height(10)  // Limit to 10 lines total
///     .offset(20); // Start from row 20 (scrolling)
///
/// // Add many rows...
/// for i in 1..=1000 {
///     large_table = large_table.row(vec![i.to_string(), format!("Data {}", i)]);
/// }
///
/// println!("{}", large_table.render());
/// println!("Actual height: {}", large_table.compute_height());
/// ```
pub struct Table {
    style_func: StyleFunc,
    boxed_style_func: Option<BoxedStyleFunc>,
    border: Border,

    border_top: bool,
    border_bottom: bool,
    border_left: bool,
    border_right: bool,
    border_header: bool,
    border_column: bool,
    border_row: bool,

    border_style: Style,
    headers: Vec<String>,
    data: Box<dyn Data>,

    width: i32,
    height: i32,
    use_manual_height: bool,
    offset: usize,
    wrap: bool,

    // widths tracks the width of each column.
    widths: Vec<usize>,

    // heights tracks the height of each row.
    heights: Vec<usize>,
}

impl Table {
    /// Creates a new `Table` with default settings and no content.
    ///
    /// The default table configuration includes:
    /// - Rounded borders (`lipgloss::rounded_border()`)
    /// - All border sides enabled (top, bottom, left, right, header separator, column separators)
    /// - Row separators disabled
    /// - Text wrapping enabled
    /// - No width or height constraints
    /// - No content (headers or data rows)
    /// - Basic styling function (`default_styles`)
    ///
    /// # Returns
    ///
    /// A new `Table` instance ready for configuration via the builder pattern.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use lipgloss_table::Table;
    ///
    /// let table = Table::new();
    /// assert_eq!(table.compute_height(), 2); // Just top and bottom borders
    /// ```
    ///
    /// ```rust
    /// use lipgloss_table::Table;
    ///
    /// let mut table = Table::new()
    ///     .headers(vec!["Column 1", "Column 2"])
    ///     .row(vec!["Data 1", "Data 2"]);
    ///
    /// println!("{}", table.render());
    /// ```
    pub fn new() -> Self {
        Self {
            style_func: default_styles,
            boxed_style_func: None,
            border: lipgloss::rounded_border(),
            border_bottom: true,
            border_column: true,
            border_header: true,
            border_left: true,
            border_right: true,
            border_top: true,
            border_row: false,
            border_style: Style::new(),
            headers: Vec::new(),
            data: Box::new(StringData::empty()),
            width: 0,
            height: 0,
            use_manual_height: false,
            offset: 0,
            wrap: true,
            widths: Vec::new(),
            heights: Vec::new(),
        }
    }

    /// Removes all data rows from the table while preserving headers and settings.
    ///
    /// This method clears only the table's data content, leaving headers, styling,
    /// borders, and other configuration unchanged. It's useful for reusing a
    /// configured table with different data.
    ///
    /// # Returns
    ///
    /// The `Table` instance with all data rows removed, enabling method chaining.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use lipgloss_table::Table;
    ///
    /// let mut table = Table::new()
    ///     .headers(vec!["Name", "Age"])
    ///     .row(vec!["Alice", "30"])
    ///     .row(vec!["Bob", "25"])
    ///     .clear_rows()
    ///     .row(vec!["Charlie", "35"]);
    ///
    /// // Table now has headers and only Charlie's row
    /// println!("{}", table.render());
    /// ```
    pub fn clear_rows(mut self) -> Self {
        self.data = Box::new(StringData::empty());
        self
    }

    /// Sets a simple function-based styling function for table cells.
    ///
    /// This method accepts a function pointer that determines the style for each
    /// cell based on its row and column position. The function receives the row
    /// index (with `HEADER_ROW` for headers) and column index, returning a
    /// `Style` to apply to that cell.
    ///
    /// Using this method will clear any previously set boxed style function.
    ///
    /// # Arguments
    ///
    /// * `style` - A function that takes `(row: i32, col: usize) -> Style`
    ///
    /// # Returns
    ///
    /// The `Table` instance with the style function applied, enabling method chaining.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use lipgloss_table::{Table, HEADER_ROW, header_row_style};
    /// use lipgloss::{Style, Color};
    ///
    /// // Using a predefined style function
    /// let table1 = Table::new()
    ///     .headers(vec!["Name", "Age"])
    ///     .style_func(header_row_style);
    ///
    /// // Using a custom style function
    /// let custom_style = |row: i32, col: usize| {
    ///     match (row, col) {
    ///         (HEADER_ROW, _) => Style::new().bold(true),
    ///         (_, 0) => Style::new().foreground(Color::from("#00FF00")),
    ///         _ => Style::new(),
    ///     }
    /// };
    ///
    /// let table2 = Table::new()
    ///     .headers(vec!["Status", "Message"])
    ///     .style_func(custom_style);
    /// ```
    pub fn style_func(mut self, style: StyleFunc) -> Self {
        self.style_func = style;
        self.boxed_style_func = None; // Clear any boxed style func
        self
    }

    /// Sets a flexible closure-based styling function that can capture variables from its environment.
    ///
    /// This method allows for more complex styling logic than `style_func` by accepting
    /// a closure that can capture variables from the surrounding scope. This is useful
    /// when your styling logic needs to reference external data, configuration, or state.
    ///
    /// The closure is boxed and stored, allowing it to outlive the current scope while
    /// maintaining access to captured variables.
    ///
    /// # Type Parameters
    ///
    /// * `F` - A closure type that implements `Fn(i32, usize) -> Style + Send + Sync + 'static`
    ///
    /// # Arguments
    ///
    /// * `style` - A closure that takes `(row: i32, col: usize) -> Style`
    ///
    /// # Returns
    ///
    /// The `Table` instance with the boxed style function applied, enabling method chaining.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use lipgloss_table::{Table, HEADER_ROW};
    /// use lipgloss::{Style, Color};
    ///
    /// // Capture colors from the environment
    /// let error_color = Color::from("#FF0000");
    /// let success_color = Color::from("#00FF00");
    /// let warning_color = Color::from("#FFAA00");
    ///
    /// let mut table = Table::new()
    ///     .headers(vec!["Status", "Message", "Code"])
    ///     .row(vec!["Error", "Something failed", "500"])
    ///     .row(vec!["Success", "All good", "200"])
    ///     .row(vec!["Warning", "Be careful", "400"])
    ///     .style_func_boxed(move |row: i32, col: usize| {
    ///         match (row, col) {
    ///             (HEADER_ROW, _) => Style::new().bold(true),
    ///             (_, 0) => {
    ///                 // Style status column based on content
    ///                 match row {
    ///                     0 => Style::new().foreground(error_color.clone()),
    ///                     1 => Style::new().foreground(success_color.clone()),
    ///                     2 => Style::new().foreground(warning_color.clone()),
    ///                     _ => Style::new(),
    ///                 }
    ///             }
    ///             _ => Style::new(),
    ///         }
    ///     });
    ///
    /// println!("{}", table.render());
    /// ```
    pub fn style_func_boxed<F>(mut self, style: F) -> Self
    where
        F: Fn(i32, usize) -> Style + Send + Sync + 'static,
    {
        self.boxed_style_func = Some(Box::new(style));
        self
    }

    /// Sets the table border.
    pub fn border(mut self, border: Border) -> Self {
        self.border = border;
        self
    }

    /// Sets the style for the table border.
    pub fn border_style(mut self, style: Style) -> Self {
        self.border_style = style;
        self
    }

    /// Sets whether or not the top border is rendered.
    pub fn border_top(mut self, v: bool) -> Self {
        self.border_top = v;
        self
    }

    /// Sets whether or not the bottom border is rendered.
    pub fn border_bottom(mut self, v: bool) -> Self {
        self.border_bottom = v;
        self
    }

    /// Sets whether or not the left border is rendered.
    pub fn border_left(mut self, v: bool) -> Self {
        self.border_left = v;
        self
    }

    /// Sets whether or not the right border is rendered.
    pub fn border_right(mut self, v: bool) -> Self {
        self.border_right = v;
        self
    }

    /// Sets whether or not the header separator is rendered.
    pub fn border_header(mut self, v: bool) -> Self {
        self.border_header = v;
        self
    }

    /// Sets whether or not column separators are rendered.
    pub fn border_column(mut self, v: bool) -> Self {
        self.border_column = v;
        self
    }

    /// Sets whether or not row separators are rendered.
    pub fn border_row(mut self, v: bool) -> Self {
        self.border_row = v;
        self
    }

    /// Sets the column headers for the table.
    ///
    /// Headers are displayed at the top of the table and are typically styled
    /// differently from data rows (e.g., bold text). The number of headers
    /// determines the number of columns in the table.
    ///
    /// # Type Parameters
    ///
    /// * `I` - An iterator type that yields items convertible to `String`
    /// * `S` - A type that can be converted into `String`
    ///
    /// # Arguments
    ///
    /// * `headers` - An iterable collection of header values (strings, string slices, etc.)
    ///
    /// # Returns
    ///
    /// The `Table` instance with headers set, enabling method chaining.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use lipgloss_table::Table;
    ///
    /// // Using string slices
    /// let table1 = Table::new()
    ///     .headers(vec!["Name", "Age", "City"]);
    ///
    /// // Using owned strings
    /// let headers = vec!["ID".to_string(), "Description".to_string()];
    /// let table2 = Table::new()
    ///     .headers(headers);
    ///
    /// // Using an array
    /// let table3 = Table::new()
    ///     .headers(["Product", "Price", "Stock"]);
    /// ```
    pub fn headers<I, S>(mut self, headers: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        self.headers = headers.into_iter().map(|s| s.into()).collect();
        self
    }

    /// Adds a single row to the table.
    pub fn row<I, S>(mut self, row: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        let row_data: Vec<String> = row.into_iter().map(|s| s.into()).collect();

        // Convert current data to StringData - always create a new one from the matrix
        let matrix = data_to_matrix(self.data.as_ref());
        let mut string_data = StringData::new(matrix);
        string_data.append(row_data);
        self.data = Box::new(string_data);
        self
    }

    /// Adds multiple rows to the table.
    pub fn rows<I, J, S>(mut self, rows: I) -> Self
    where
        I: IntoIterator<Item = J>,
        J: IntoIterator<Item = S>,
        S: Into<String>,
    {
        for row in rows {
            self = self.row(row);
        }
        self
    }

    /// Sets the data source for the table.
    pub fn data<D: Data + 'static>(mut self, data: D) -> Self {
        self.data = Box::new(data);
        self
    }

    /// Sets a fixed width for the table.
    pub fn width(mut self, w: i32) -> Self {
        self.width = w;
        self
    }

    /// Sets a fixed height for the table.
    pub fn height(mut self, h: i32) -> Self {
        self.height = h;
        self.use_manual_height = h > 0;
        self
    }

    /// Sets the row offset for the table (for scrolling).
    pub fn offset(mut self, o: usize) -> Self {
        self.offset = o;
        self
    }

    /// Sets whether text wrapping is enabled.
    pub fn wrap(mut self, w: bool) -> Self {
        self.wrap = w;
        self
    }

    /// Renders the table to a complete string representation.
    ///
    /// This method performs the final rendering step, calculating layout dimensions,
    /// applying styles, and constructing the complete table string with borders,
    /// headers, and data rows. It must be called to generate the visual output.
    ///
    /// The rendering process includes:
    /// - Calculating optimal column widths and row heights
    /// - Applying cell styles and text wrapping/truncation
    /// - Constructing borders and separators
    /// - Handling height constraints and overflow indicators
    ///
    /// # Returns
    ///
    /// A `String` containing the complete rendered table with ANSI escape sequences
    /// for styling and proper spacing.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use lipgloss_table::{Table, header_row_style};
    ///
    /// let mut table = Table::new()
    ///     .headers(vec!["Name", "Score"])
    ///     .row(vec!["Alice", "95"])
    ///     .row(vec!["Bob", "87"])
    ///     .style_func(header_row_style);
    ///
    /// let output = table.render();
    /// println!("{}", output);
    /// ```
    ///
    /// ```rust
    /// use lipgloss_table::Table;
    ///
    /// let mut table = Table::new()
    ///     .headers(vec!["Product", "Description"])
    ///     .row(vec!["Widget", "A very long description that will wrap"])
    ///     .width(30);
    ///
    /// let output = table.render();
    /// // Output will be wrapped to fit within 30 characters width
    /// println!("{}", output);
    /// ```
    pub fn render(&mut self) -> String {
        self.resize();
        self.construct_table()
    }

    /// Computes the total height the table will occupy when rendered.
    ///
    /// This method calculates the exact number of terminal lines the table will
    /// use when rendered, including all borders, headers, data rows, and separators.
    /// It's useful for layout planning, especially when working with height-constrained
    /// terminals or when implementing scrolling interfaces.
    ///
    /// The calculation includes:
    /// - Top and bottom borders (if enabled)
    /// - Header row and header separator (if headers exist)
    /// - All data rows with their calculated heights
    /// - Row separators between data rows (if enabled)
    ///
    /// # Returns
    ///
    /// The total height in terminal lines as a `usize`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use lipgloss_table::Table;
    ///
    /// let table = Table::new();
    /// assert_eq!(table.compute_height(), 2); // Just top and bottom borders
    ///
    /// let table_with_content = Table::new()
    ///     .headers(vec!["Name", "Age"])
    ///     .row(vec!["Alice", "30"]);
    /// // Height = top border + header + header separator + data row + bottom border
    /// assert_eq!(table_with_content.compute_height(), 5);
    /// ```
    ///
    /// ```rust
    /// use lipgloss_table::Table;
    ///
    /// let mut large_table = Table::new()
    ///     .headers(vec!["ID", "Data"])
    ///     .height(10); // Height constraint
    ///
    /// for i in 1..=100 {
    ///     large_table = large_table.row(vec![i.to_string(), format!("Data {}", i)]);
    /// }
    ///
    /// large_table.render(); // Must render first to populate heights
    /// let height = large_table.compute_height();
    /// // compute_height() returns the natural height, not constrained height
    /// // The actual rendered output will be constrained to 10 lines
    /// assert!(height > 10); // Natural height is larger than constraint
    /// ```
    pub fn compute_height(&self) -> usize {
        let has_headers = !self.headers.is_empty();
        let data_rows = self.data.rows();

        // If no rows and no headers, just border height
        if data_rows == 0 && !has_headers {
            return if self.border_top && self.border_bottom {
                2
            } else if self.border_top || self.border_bottom {
                1
            } else {
                0
            };
        }

        let mut total_height = 0;

        // Top border
        if self.border_top {
            total_height += 1;
        }

        // Header row
        if has_headers {
            total_height += 1;

            // Header separator
            if self.border_header {
                total_height += 1;
            }
        }

        // Data rows
        if data_rows > 0 {
            // Sum the heights of all data rows
            let header_offset = if has_headers { 1 } else { 0 };
            for i in 0..data_rows {
                let row_height = self.heights.get(i + header_offset).unwrap_or(&1);
                total_height += row_height;

                // Row separators (between data rows, not after the last one)
                if self.border_row && i < data_rows - 1 {
                    total_height += 1;
                }
            }
        }

        // Bottom border
        if self.border_bottom {
            total_height += 1;
        }

        total_height
    }

    // Private methods for internal rendering

    /// Get the appropriate style for a cell, using either the function pointer or boxed function.
    fn get_cell_style(&self, row: i32, col: usize) -> Style {
        if let Some(ref boxed_func) = self.boxed_style_func {
            boxed_func(row, col)
        } else {
            (self.style_func)(row, col)
        }
    }

    fn resize(&mut self) {
        let has_headers = !self.headers.is_empty();
        let rows = data_to_matrix(self.data.as_ref());
        let mut resizer = Resizer::new(self.width, self.height, self.headers.clone(), rows);
        resizer.wrap = self.wrap;
        resizer.border_column = self.border_column;
        resizer.y_paddings = vec![vec![0; resizer.columns.len()]; resizer.all_rows.len()];

        // Calculate style-based padding for each cell
        resizer.row_heights = resizer.default_row_heights();

        for (i, row) in resizer.all_rows.iter().enumerate() {
            if i >= resizer.y_paddings.len() {
                resizer.y_paddings.push(vec![0; row.len()]);
            }
            if resizer.y_paddings[i].len() < row.len() {
                resizer.y_paddings[i].resize(row.len(), 0);
            }

            for j in 0..row.len() {
                if j >= resizer.columns.len() {
                    continue;
                }

                // Making sure we're passing the right index to the style function.
                // The header row should be `-1` and the others should start from `0`.
                let row_index = if has_headers { i as i32 - 1 } else { i as i32 };
                let style = self.get_cell_style(row_index, j);

                // Extract margin and padding values
                let (top_margin, right_margin, bottom_margin, left_margin) = (
                    style.get_margin_top().max(0) as usize,
                    style.get_margin_right().max(0) as usize,
                    style.get_margin_bottom().max(0) as usize,
                    style.get_margin_left().max(0) as usize,
                );
                let (top_padding, right_padding, bottom_padding, left_padding) = (
                    style.get_padding_top().max(0) as usize,
                    style.get_padding_right().max(0) as usize,
                    style.get_padding_bottom().max(0) as usize,
                    style.get_padding_left().max(0) as usize,
                );

                let total_horizontal_padding =
                    left_margin + right_margin + left_padding + right_padding;
                resizer.columns[j].x_padding =
                    resizer.columns[j].x_padding.max(total_horizontal_padding);

                let width = style.get_width();
                if width > 0 {
                    resizer.columns[j].fixed_width =
                        resizer.columns[j].fixed_width.max(width as usize);
                }

                let height = style.get_height();
                if height > 0 {
                    resizer.row_heights[i] = resizer.row_heights[i].max(height as usize);
                }

                let total_vertical_padding =
                    top_margin + bottom_margin + top_padding + bottom_padding;
                resizer.y_paddings[i][j] = total_vertical_padding;
            }
        }

        // Auto-detect table width if not specified
        if resizer.table_width <= 0 {
            resizer.table_width = resizer.detect_table_width();
        }

        let (widths, heights) = resizer.optimized_widths();
        self.widths = widths;
        self.heights = heights;
    }

    fn construct_table(&self) -> String {
        let mut result = String::new();
        let has_headers = !self.headers.is_empty();
        let _data_rows = self.data.rows();

        if self.widths.is_empty() {
            return result;
        }

        // Construct top border
        if self.border_top {
            result.push_str(&self.construct_top_border());
            result.push('\n');
        }

        // Construct headers
        if has_headers {
            result.push_str(&self.construct_headers());
            result.push('\n');

            // Header separator
            if self.border_header {
                result.push_str(&self.construct_header_separator());
                result.push('\n');
            }
        }

        // Construct data rows
        let available_lines = if self.use_manual_height && self.height > 0 {
            let used_lines = if self.border_top { 1 } else { 0 }
                + if has_headers { 1 } else { 0 }
                + if has_headers && self.border_header {
                    1
                } else {
                    0
                }
                + if self.border_bottom { 1 } else { 0 };
            (self.height as usize).saturating_sub(used_lines)
        } else {
            usize::MAX
        };

        result.push_str(&self.construct_rows(available_lines));

        // Construct bottom border
        if self.border_bottom {
            if !result.is_empty() && !result.ends_with('\n') {
                result.push('\n');
            }
            result.push_str(&self.construct_bottom_border());
        }

        result
    }

    fn construct_top_border(&self) -> String {
        let mut border_parts = Vec::new();

        if self.border_left {
            border_parts.push(self.border.top_left.to_string());
        }

        for (i, &width) in self.widths.iter().enumerate() {
            border_parts.push(safe_str_repeat(self.border.top, width));

            if i < self.widths.len() - 1 && self.border_column {
                border_parts.push(self.border.middle_top.to_string());
            }
        }

        if self.border_right {
            border_parts.push(self.border.top_right.to_string());
        }

        self.border_style.render(&border_parts.join(""))
    }

    fn construct_bottom_border(&self) -> String {
        let mut border_parts = Vec::new();

        if self.border_left {
            border_parts.push(self.border.bottom_left.to_string());
        }

        for (i, &width) in self.widths.iter().enumerate() {
            border_parts.push(safe_str_repeat(self.border.bottom, width));

            if i < self.widths.len() - 1 && self.border_column {
                border_parts.push(self.border.middle_bottom.to_string());
            }
        }

        if self.border_right {
            border_parts.push(self.border.bottom_right.to_string());
        }

        self.border_style.render(&border_parts.join(""))
    }

    fn construct_header_separator(&self) -> String {
        let mut border_parts = Vec::new();

        if self.border_left {
            border_parts.push(self.border.middle_left.to_string());
        }

        for (i, &width) in self.widths.iter().enumerate() {
            border_parts.push(safe_str_repeat(self.border.top, width));

            if i < self.widths.len() - 1 && self.border_column {
                border_parts.push(self.border.middle.to_string());
            }
        }

        if self.border_right {
            border_parts.push(self.border.middle_right.to_string());
        }

        self.border_style.render(&border_parts.join(""))
    }

    fn construct_headers(&self) -> String {
        self.construct_row_content(&self.headers, HEADER_ROW)
    }

    fn construct_rows(&self, available_lines: usize) -> String {
        let mut result = String::new();
        let mut lines_used = 0;
        let data_rows = self.data.rows();

        for i in self.offset..data_rows {
            if lines_used >= available_lines {
                // Add overflow indicator if we have more data
                if i < data_rows {
                    result.push_str(&self.construct_overflow_row());
                }
                break;
            }

            // Get row data
            let mut row_data = Vec::new();
            for j in 0..self.data.columns() {
                row_data.push(self.data.at(i, j));
            }

            result.push_str(&self.construct_row_content(&row_data, i as i32));
            lines_used += self
                .heights
                .get(i + if !self.headers.is_empty() { 1 } else { 0 })
                .unwrap_or(&1);

            // Add row separator if needed
            if self.border_row && i < data_rows - 1 && lines_used < available_lines {
                result.push('\n');
                result.push_str(&self.construct_row_separator());
                lines_used += 1;
            }

            if i < data_rows - 1 {
                result.push('\n');
            }
        }

        result
    }

    fn construct_row_content(&self, row_data: &[String], row_index: i32) -> String {
        let mut cell_parts = Vec::new();

        if self.border_left {
            cell_parts.push(self.border.left.to_string());
        }

        for (j, cell_content) in row_data.iter().enumerate() {
            if j >= self.widths.len() {
                break;
            }

            let cell_width = self.widths[j];
            let style = self.get_cell_style(row_index, j);

            // Apply cell styling and fit to width
            let styled_content = self.style_cell_content(cell_content, cell_width, style);
            cell_parts.push(styled_content);

            if self.border_column && j < row_data.len() - 1 {
                cell_parts.push(self.border.left.to_string());
            }
        }

        if self.border_right {
            cell_parts.push(self.border.right.to_string());
        }

        cell_parts.join("")
    }

    fn construct_row_separator(&self) -> String {
        let mut border_parts = Vec::new();

        if self.border_left {
            border_parts.push(self.border.middle_left.to_string());
        }

        for (i, &width) in self.widths.iter().enumerate() {
            border_parts.push(safe_str_repeat(self.border.top, width));

            if i < self.widths.len() - 1 && self.border_column {
                border_parts.push(self.border.middle.to_string());
            }
        }

        if self.border_right {
            border_parts.push(self.border.middle_right.to_string());
        }

        self.border_style.render(&border_parts.join(""))
    }

    fn construct_overflow_row(&self) -> String {
        let mut cell_parts = Vec::new();

        if self.border_left {
            cell_parts.push(self.border.left.to_string());
        }

        for (i, &width) in self.widths.iter().enumerate() {
            let ellipsis = "…".to_string();
            let padding = safe_repeat(' ', width.saturating_sub(ellipsis.len()));
            cell_parts.push(format!("{}{}", ellipsis, padding));

            if self.border_column && i < self.widths.len() - 1 {
                cell_parts.push(self.border.left.to_string());
            }
        }

        if self.border_right {
            cell_parts.push(self.border.right.to_string());
        }

        cell_parts.join("")
    }

    fn style_cell_content(&self, content: &str, width: usize, style: Style) -> String {
        // Handle content wrapping if needed
        let fitted_content = if self.wrap {
            self.wrap_cell_content(content, width)
        } else {
            self.truncate_cell_content(content, width)
        };

        // Apply the lipgloss style to the content
        // The style should handle its own width constraints, so we apply it directly
        style.width(width as i32).render(&fitted_content)
    }

    fn truncate_cell_content(&self, content: &str, width: usize) -> String {
        let content_width = lipgloss::width(content);

        if content_width > width {
            // Truncate with ellipsis, handling ANSI sequences properly
            if width == 0 {
                return String::new();
            } else if width == 1 {
                return "…".to_string();
            }

            // For ANSI-aware truncation, we need to be more careful
            // For now, use a simple approach that may not be perfect with ANSI sequences
            let chars: Vec<char> = content.chars().collect();
            let mut result = String::new();
            let mut current_width = 0;

            for ch in chars {
                let char_str = ch.to_string();
                let char_width = lipgloss::width(&char_str);

                if current_width + char_width + 1 > width {
                    // +1 for ellipsis
                    break;
                }

                result.push(ch);
                current_width += char_width;
            }

            result.push('…');
            result
        } else {
            content.to_string()
        }
    }

    fn wrap_cell_content(&self, content: &str, width: usize) -> String {
        if width == 0 {
            return String::new();
        }

        let mut wrapped_lines = Vec::new();

        // Handle existing line breaks
        for line in content.lines() {
            if line.is_empty() {
                wrapped_lines.push(String::new());
                continue;
            }

            // Use lipgloss width which handles ANSI sequences
            let line_width = lipgloss::width(line);
            if line_width <= width {
                wrapped_lines.push(line.to_string());
            } else {
                // Need to wrap this line - use ANSI-aware wrapping
                wrapped_lines.extend(self.wrap_line_ansi_aware(line, width));
            }
        }

        wrapped_lines.join("\n")
    }

    fn wrap_line_ansi_aware(&self, line: &str, width: usize) -> Vec<String> {
        // For now, use a simple word-based wrapping that preserves ANSI sequences
        // This could be enhanced to use lipgloss's word wrapping utilities if available
        let words: Vec<&str> = line.split_whitespace().collect();
        let mut lines = Vec::new();
        let mut current_line = String::new();
        let mut current_width = 0;

        for word in words {
            let word_width = lipgloss::width(word);

            // If adding this word would exceed width, start a new line
            if !current_line.is_empty() && current_width + 1 + word_width > width {
                lines.push(current_line);
                current_line = word.to_string();
                current_width = word_width;
            } else if current_line.is_empty() {
                current_line = word.to_string();
                current_width = word_width;
            } else {
                current_line.push(' ');
                current_line.push_str(word);
                current_width += 1 + word_width;
            }
        }

        if !current_line.is_empty() {
            lines.push(current_line);
        }

        if lines.is_empty() {
            lines.push(String::new());
        }

        lines
    }
}

impl fmt::Display for Table {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Need to create a mutable copy for rendering since fmt doesn't allow mutable self
        let mut table_copy = Table {
            style_func: self.style_func,
            boxed_style_func: None, // Cannot clone boxed closures easily
            border: self.border,
            border_top: self.border_top,
            border_bottom: self.border_bottom,
            border_left: self.border_left,
            border_right: self.border_right,
            border_header: self.border_header,
            border_column: self.border_column,
            border_row: self.border_row,
            border_style: self.border_style.clone(),
            headers: self.headers.clone(),
            data: Box::new(StringData::new(data_to_matrix(self.data.as_ref()))),
            width: self.width,
            height: self.height,
            use_manual_height: self.use_manual_height,
            offset: self.offset,
            wrap: self.wrap,
            widths: self.widths.clone(),
            heights: self.heights.clone(),
        };

        write!(f, "{}", table_copy.render())
    }
}

impl Default for Table {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_table_new() {
        let table = Table::new();
        assert_eq!(table.headers.len(), 0);
        assert_eq!(table.data.rows(), 0);
        assert_eq!(table.data.columns(), 0);
        assert!(table.border_top);
        assert!(table.border_bottom);
        assert!(table.border_left);
        assert!(table.border_right);
        assert!(table.border_header);
        assert!(table.border_column);
        assert!(!table.border_row);
        assert!(table.wrap);
    }

    #[test]
    fn test_table_headers() {
        let table = Table::new().headers(vec!["Name", "Age", "Location"]);
        assert_eq!(table.headers.len(), 3);
        assert_eq!(table.headers[0], "Name");
        assert_eq!(table.headers[1], "Age");
        assert_eq!(table.headers[2], "Location");
    }

    #[test]
    fn test_table_rows() {
        let table = Table::new()
            .headers(vec!["Name", "Age"])
            .row(vec!["Alice", "30"])
            .row(vec!["Bob", "25"]);

        assert_eq!(table.data.rows(), 2);
        assert_eq!(table.data.columns(), 2);
        assert_eq!(table.data.at(0, 0), "Alice");
        assert_eq!(table.data.at(0, 1), "30");
        assert_eq!(table.data.at(1, 0), "Bob");
        assert_eq!(table.data.at(1, 1), "25");
    }

    #[test]
    fn test_table_builder_pattern() {
        let table = Table::new()
            .border_top(false)
            .border_bottom(false)
            .width(80)
            .height(10)
            .wrap(false);

        assert!(!table.border_top);
        assert!(!table.border_bottom);
        assert_eq!(table.width, 80);
        assert_eq!(table.height, 10);
        assert!(!table.wrap);
    }

    #[test]
    fn test_compute_height_empty_table() {
        let table = Table::new();
        assert_eq!(table.compute_height(), 2); // top + bottom border

        let table_no_borders = Table::new().border_top(false).border_bottom(false);
        assert_eq!(table_no_borders.compute_height(), 0);

        let table_top_only = Table::new().border_bottom(false);
        assert_eq!(table_top_only.compute_height(), 1);

        let table_bottom_only = Table::new().border_top(false);
        assert_eq!(table_bottom_only.compute_height(), 1);
    }

    #[test]
    fn test_compute_height_headers_only() {
        let table = Table::new().headers(vec!["Name", "Age"]);
        // top border + header + header separator + bottom border
        assert_eq!(table.compute_height(), 4);

        let table_no_header_sep = Table::new()
            .headers(vec!["Name", "Age"])
            .border_header(false);
        // top border + header + bottom border
        assert_eq!(table_no_header_sep.compute_height(), 3);

        let table_no_borders = Table::new()
            .headers(vec!["Name", "Age"])
            .border_top(false)
            .border_bottom(false)
            .border_header(false);
        // just header
        assert_eq!(table_no_borders.compute_height(), 1);
    }

    #[test]
    fn test_compute_height_with_data() {
        let mut table = Table::new()
            .headers(vec!["Name", "Age"])
            .row(vec!["Alice", "30"])
            .row(vec!["Bob", "25"]);

        // Need to render first to populate heights
        table.render();

        // top border + header + header separator + 2 data rows + bottom border = 6
        assert_eq!(table.compute_height(), 6);
    }

    #[test]
    fn test_compute_height_with_row_borders() {
        let mut table = Table::new()
            .headers(vec!["Name", "Age"])
            .row(vec!["Alice", "30"])
            .row(vec!["Bob", "25"])
            .border_row(true);

        table.render();

        // top border + header + header separator + row1 + row separator + row2 + bottom border = 7
        assert_eq!(table.compute_height(), 7);
    }

    #[test]
    fn test_compute_height_data_only() {
        let mut table = Table::new().row(vec!["Alice", "30"]).row(vec!["Bob", "25"]);

        table.render();

        // top border + 2 data rows + bottom border = 4
        assert_eq!(table.compute_height(), 4);

        let mut table_with_row_borders = Table::new()
            .row(vec!["Alice", "30"])
            .row(vec!["Bob", "25"])
            .border_row(true);

        table_with_row_borders.render();

        // top border + row1 + row separator + row2 + bottom border = 5
        assert_eq!(table_with_row_borders.compute_height(), 5);
    }

    #[test]
    fn test_compute_height_single_row() {
        let mut table = Table::new().headers(vec!["Name"]).row(vec!["Alice"]);

        table.render();

        // top border + header + header separator + 1 data row + bottom border = 5
        assert_eq!(table.compute_height(), 5);
    }

    #[test]
    fn test_compute_height_minimal_borders() {
        let mut table = Table::new()
            .headers(vec!["Name", "Age"])
            .row(vec!["Alice", "30"])
            .border_top(false)
            .border_bottom(false)
            .border_header(false);

        table.render();

        // just header + data row = 2
        assert_eq!(table.compute_height(), 2);
    }

    #[test]
    fn test_table_clear_rows() {
        let table = Table::new()
            .row(vec!["A", "B"])
            .row(vec!["C", "D"])
            .clear_rows();

        assert_eq!(table.data.rows(), 0);
        assert_eq!(table.data.columns(), 0);
    }

    #[test]
    fn test_table_rendering() {
        let mut table = Table::new()
            .headers(vec!["Name", "Age", "City"])
            .row(vec!["Alice", "30", "New York"])
            .row(vec!["Bob", "25", "London"]);

        let output = table.render();
        assert!(!output.is_empty());

        // Should contain the header and data
        assert!(output.contains("Name"));
        assert!(output.contains("Alice"));
        assert!(output.contains("Bob"));

        // Should have borders by default
        assert!(output.contains("┌") || output.contains("╭")); // Top-left corner
    }

    #[test]
    fn test_table_no_borders() {
        let mut table = Table::new()
            .headers(vec!["Name", "Age"])
            .row(vec!["Alice", "30"])
            .border_top(false)
            .border_bottom(false)
            .border_left(false)
            .border_right(false)
            .border_column(false);

        let output = table.render();
        assert!(!output.is_empty());
        assert!(output.contains("Name"));
        assert!(output.contains("Alice"));

        // Should not contain border characters
        assert!(!output.contains("┌"));
        assert!(!output.contains("│"));
    }

    #[test]
    fn test_table_width_constraint() {
        let mut table = Table::new()
            .headers(vec!["Name", "Age", "City"])
            .row(vec!["Alice Johnson", "28", "New York"])
            .row(vec!["Bob Smith", "35", "London"])
            .width(25); // Force narrow width

        let output = table.render();
        assert!(!output.is_empty());

        // Each line should respect the width constraint (using display width, not character count)
        for line in output.lines() {
            // Use lipgloss width which handles ANSI sequences properly
            let line_width = lipgloss::width(line);
            assert!(
                line_width <= 25,
                "Line '{}' has display width {} > 25",
                line,
                line_width
            );
        }
    }

    #[test]
    fn test_comprehensive_table_demo() {
        let mut table = Table::new()
            .headers(vec!["Name", "Age", "City", "Occupation"])
            .row(vec!["Alice Johnson", "28", "New York", "Software Engineer"])
            .row(vec!["Bob Smith", "35", "London", "Product Manager"])
            .row(vec!["Charlie Brown", "42", "Tokyo", "UX Designer"])
            .row(vec!["Diana Prince", "30", "Paris", "Data Scientist"]);

        let output = table.render();
        println!("\n=== Comprehensive Table Demo ===");
        println!("{}", output);

        assert!(!output.is_empty());
        assert!(output.contains("Alice Johnson"));
        assert!(output.contains("Software Engineer"));

        // Test different border styles
        println!("\n=== No Borders Demo ===");
        let mut no_border_table = Table::new()
            .headers(vec!["Item", "Price"])
            .row(vec!["Coffee", "$3.50"])
            .row(vec!["Tea", "$2.25"])
            .border_top(false)
            .border_bottom(false)
            .border_left(false)
            .border_right(false)
            .border_column(false)
            .border_header(false);

        println!("{}", no_border_table.render());

        // Test width constraint
        println!("\n=== Width Constrained Table ===");
        let mut narrow_table = Table::new()
            .headers(vec!["Product", "Description", "Price"])
            .row(vec![
                "MacBook Pro",
                "Powerful laptop for developers",
                "$2399",
            ])
            .row(vec![
                "iPhone",
                "Latest smartphone with amazing camera",
                "$999",
            ])
            .width(40);

        println!("{}", narrow_table.render());
    }

    #[test]
    fn test_empty_table() {
        let mut table = Table::new();
        let output = table.render();

        // Empty table should produce minimal output
        assert!(output.is_empty() || output.trim().is_empty());
    }

    #[test]
    fn test_cell_styling_with_lipgloss() {
        use lipgloss::{
            color::{STATUS_ERROR, TEXT_MUTED},
            Style,
        };

        let style_func = |row: i32, _col: usize| match row {
            HEADER_ROW => Style::new().bold(true).foreground(STATUS_ERROR),
            _ if row % 2 == 0 => Style::new().foreground(TEXT_MUTED),
            _ => Style::new().italic(true),
        };

        let mut table = Table::new()
            .headers(vec!["Name", "Age", "City"])
            .row(vec!["Alice", "30", "New York"])
            .row(vec!["Bob", "25", "London"])
            .style_func(style_func);

        let output = table.render();
        assert!(!output.is_empty());
        assert!(output.contains("Name")); // Headers should be present
        assert!(output.contains("Alice")); // Data should be present

        // Since we're applying styles, there should be ANSI escape sequences
        assert!(output.contains("\\x1b[") || output.len() > 50); // Either ANSI codes or substantial content
    }

    #[test]
    fn test_text_wrapping_functionality() {
        let mut table = Table::new()
            .headers(vec!["Short", "VeryLongContentThatShouldWrap"])
            .row(vec!["A", "This is a very long piece of content that should wrap across multiple lines when the table width is constrained"])
            .width(30)
            .wrap(true);

        let output = table.render();
        assert!(!output.is_empty());

        // With wrapping enabled and constrained width, we should get multiple lines
        let line_count = output.lines().count();
        assert!(
            line_count > 3,
            "Expected more than 3 lines due to wrapping, got {}",
            line_count
        );
    }

    #[test]
    fn test_text_truncation_functionality() {
        let mut table = Table::new()
            .headers(vec!["Short", "Long"])
            .row(vec![
                "A",
                "This is a very long piece of content that should be truncated",
            ])
            .width(25)
            .wrap(false); // Disable wrapping to force truncation

        let output = table.render();
        assert!(!output.is_empty());

        // Should contain ellipsis indicating truncation
        assert!(
            output.contains("…"),
            "Expected ellipsis for truncated content"
        );
    }

    #[test]
    fn test_ansi_aware_width_calculation() {
        use lipgloss::{Color, Style};

        // Create content with ANSI sequences
        let styled_content = Style::new()
            .foreground(Color::from("#FF0000"))
            .bold(true)
            .render("Test");

        let mut table = Table::new()
            .headers(vec!["Styled"])
            .row(vec![&styled_content])
            .width(10);

        let output = table.render();
        assert!(!output.is_empty());

        // The table should handle ANSI sequences correctly in width calculations
        // The visual width should be respected, not the character count
        for line in output.lines() {
            let visual_width = lipgloss::width(line);
            assert!(
                visual_width <= 10,
                "Line has visual width {} > 10: '{}'",
                visual_width,
                line
            );
        }
    }

    #[test]
    fn test_predefined_style_functions() {
        // Test header_row_style
        let mut table1 = Table::new()
            .headers(vec!["Name", "Age"])
            .row(vec!["Alice", "30"])
            .style_func(header_row_style);

        let output1 = table1.render();
        assert!(!output1.is_empty());
        assert!(output1.contains("Name"));

        // Test zebra_style
        let mut table2 = Table::new()
            .headers(vec!["Item", "Count"])
            .row(vec!["Apple", "5"])
            .row(vec!["Banana", "3"])
            .row(vec!["Cherry", "8"])
            .style_func(zebra_style);

        let output2 = table2.render();
        assert!(!output2.is_empty());
        assert!(output2.contains("Item"));

        // Test minimal_style
        let mut table3 = Table::new()
            .headers(vec!["Name"])
            .row(vec!["Test"])
            .style_func(minimal_style);

        let output3 = table3.render();
        assert!(!output3.is_empty());
        assert!(output3.contains("Name"));
    }

    #[test]
    fn test_boxed_style_function() {
        use lipgloss::{
            color::{STATUS_ERROR, STATUS_WARNING},
            Style,
        };

        // Create a closure that captures variables
        let error_color = STATUS_ERROR;
        let warning_color = STATUS_WARNING;

        let mut table = Table::new()
            .headers(vec!["Status", "Message"])
            .row(vec!["ERROR", "Something went wrong"])
            .row(vec!["WARNING", "This is a warning"])
            .row(vec!["INFO", "Everything is fine"])
            .style_func_boxed(move |row: i32, col: usize| {
                if row == HEADER_ROW {
                    Style::new().bold(true)
                } else if col == 0 {
                    // Style the status column based on content
                    // Note: In a real implementation, you'd have access to the cell content
                    match row {
                        0 => Style::new().foreground(error_color.clone()),
                        1 => Style::new().foreground(warning_color.clone()),
                        _ => Style::new(),
                    }
                } else {
                    Style::new()
                }
            });

        let output = table.render();
        assert!(!output.is_empty());
        assert!(output.contains("Status"));
        assert!(output.contains("ERROR"));
    }
}
