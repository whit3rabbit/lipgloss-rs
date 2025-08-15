/// A column representation in the table resizer with width analysis and content.
///
/// `ResizerColumn` stores statistical information about a table column including
/// width measurements, content, and padding requirements. This data is used by
/// the resizing algorithm to determine optimal column widths while respecting
/// content constraints and table width limits.
#[derive(Debug, Clone)]
pub struct ResizerColumn {
    /// The zero-based column index within the table.
    pub index: usize,
    
    /// The minimum content width found in this column across all rows.
    pub min: usize,
    
    /// The maximum content width found in this column across all rows.
    pub max: usize,
    
    /// The median content width for this column, used for balanced sizing.
    pub median: usize,
    
    /// All cell content for this column, organized by row.
    pub rows: Vec<Vec<String>>,
    
    /// Horizontal padding requirement (margins + padding from styles).
    pub x_padding: usize,
    
    /// Fixed width constraint from style specifications, if any.
    pub fixed_width: usize,
}

impl ResizerColumn {
    /// Creates a new resizer column with the specified index.
    ///
    /// This constructor initializes a new `ResizerColumn` with default values for all fields
    /// except the index. The column will start with zero dimensions and empty content,
    /// which will be populated later during the resizing process.
    ///
    /// # Arguments
    ///
    /// * `index` - The zero-based column index within the table
    ///
    /// # Returns
    ///
    /// Returns a new `ResizerColumn` instance with:
    /// - `index` set to the provided value
    /// - `min`, `max`, `median`, `x_padding`, `fixed_width` set to 0
    /// - `rows` initialized as an empty vector
    ///
    /// # Examples
    ///
    /// ```
    /// use lipgloss_table::resizing::ResizerColumn;
    ///
    /// let column = ResizerColumn::new(2);
    /// assert_eq!(column.index, 2);
    /// assert_eq!(column.min, 0);
    /// assert_eq!(column.max, 0);
    /// assert!(column.rows.is_empty());
    /// ```
    pub fn new(index: usize) -> Self {
        Self {
            index,
            min: 0,
            max: 0,
            median: 0,
            rows: Vec::new(),
            x_padding: 0,
            fixed_width: 0,
        }
    }
}

/// A comprehensive table resizing engine that calculates optimal layouts.
///
/// The `Resizer` analyzes table content, applies constraints, and computes
/// the best possible layout for tables within specified dimensions. It handles
/// content analysis, width distribution, padding calculations, and height
/// management to create well-formatted tables.
#[derive(Debug)]
pub struct Resizer {
    /// Target width for the entire table in characters.
    pub table_width: i32,
    
    /// Target height for the table (used for height constraints).
    pub table_height: i32,
    
    /// Column headers as strings (empty if no headers).
    pub headers: Vec<String>,
    
    /// All table rows including headers, organized as vectors of cell content.
    pub all_rows: Vec<Vec<String>>,
    
    /// Calculated height for each row in the table.
    pub row_heights: Vec<usize>,
    
    /// Column analysis data with width statistics and content.
    pub columns: Vec<ResizerColumn>,

    /// Whether text wrapping is enabled for content that exceeds column width.
    pub wrap: bool,
    
    /// Whether column separators/borders are enabled between columns.
    pub border_column: bool,
    
    /// Vertical padding requirements for each cell, organized by [row][column].
    pub y_paddings: Vec<Vec<usize>>,
}

impl Resizer {
    /// Creates a new table resizer with the specified dimensions and content.
    ///
    /// This constructor initializes a complete resizing system that analyzes the provided
    /// table content to determine optimal column widths and row heights. It processes
    /// both headers and data rows, calculating statistical information (min, max, median)
    /// for each column to enable intelligent resizing decisions.
    ///
    /// The resizer automatically:
    /// - Combines headers and data rows into a unified structure
    /// - Creates `ResizerColumn` instances for each column in the content
    /// - Calculates width statistics (min, max, median) for each column
    /// - Prepares the foundation for width optimization algorithms
    ///
    /// # Arguments
    ///
    /// * `table_width` - Target width for the entire table in characters
    /// * `table_height` - Target height for the table (used for future height constraints)
    /// * `headers` - Column headers as strings (can be empty if no headers)
    /// * `rows` - Data rows, where each row is a vector of cell content strings
    ///
    /// # Returns
    ///
    /// Returns a new `Resizer` instance ready for width optimization, with:
    /// - All content processed and stored in `all_rows`
    /// - Column analysis completed with width statistics
    /// - Default settings for wrapping and borders enabled
    ///
    /// # Examples
    ///
    /// ```
    /// use lipgloss_table::resizing::Resizer;
    ///
    /// let headers = vec!["Name".to_string(), "Age".to_string()];
    /// let rows = vec![
    ///     vec!["Alice".to_string(), "30".to_string()],
    ///     vec!["Bob".to_string(), "25".to_string()],
    /// ];
    ///
    /// let resizer = Resizer::new(80, 24, headers, rows);
    /// assert_eq!(resizer.table_width, 80);
    /// assert_eq!(resizer.columns.len(), 2);
    /// assert_eq!(resizer.all_rows.len(), 3); // headers + 2 data rows
    /// ```
    ///
    /// # Notes
    ///
    /// - If `headers` is empty, only data rows will be processed
    /// - The number of columns is determined by the row with the most cells
    /// - Column width calculations use Unicode-aware width detection
    /// - The resizer defaults to enabling text wrapping and column borders
    pub fn new(
        table_width: i32,
        table_height: i32,
        headers: Vec<String>,
        rows: Vec<Vec<String>>,
    ) -> Self {
        let mut resizer = Self {
            table_width,
            table_height,
            headers: headers.clone(),
            all_rows: Vec::new(),
            row_heights: Vec::new(),
            columns: Vec::new(),
            wrap: true,
            border_column: true,
            y_paddings: Vec::new(),
        };

        // Build all_rows (headers + data rows)
        if !headers.is_empty() {
            resizer.all_rows.push(headers);
        }
        resizer.all_rows.extend(rows);

        // Initialize columns based on the content
        let max_cols = resizer
            .all_rows
            .iter()
            .map(|row| row.len())
            .max()
            .unwrap_or(0);

        for col_idx in 0..max_cols {
            let mut column = ResizerColumn::new(col_idx);
            column.rows = resizer.all_rows.clone();

            // Calculate min/max widths for this column
            let mut widths = Vec::new();
            for row in &resizer.all_rows {
                let cell_content = row.get(col_idx).map(|s| s.as_str()).unwrap_or("");
                let width = display_width(cell_content);
                widths.push(width);
                column.min = column.min.max(width);
                column.max = column.max.max(width);
            }

            // Calculate median width
            column.median = calculate_median(&widths);
            resizer.columns.push(column);
        }

        resizer
    }

    /// Returns the default row heights (all 1).
    pub fn default_row_heights(&self) -> Vec<usize> {
        vec![1; self.all_rows.len()]
    }

    /// Detects the table width automatically based on content.
    pub fn detect_table_width(&self) -> i32 {
        let content_width: usize = self.columns.iter().map(|col| col.max + col.x_padding).sum();
        let border_width = self.total_horizontal_border();
        (content_width + border_width) as i32
    }

    /// Returns the total width with maximum column widths.
    pub fn max_total(&self) -> usize {
        let content_width: usize = self.columns.iter().map(|col| col.max + col.x_padding).sum();
        content_width + self.total_horizontal_border()
    }

    /// Returns the maximum column widths.
    pub fn max_column_widths(&self) -> Vec<usize> {
        self.columns
            .iter()
            .map(|col| {
                if col.fixed_width > 0 {
                    col.fixed_width
                } else {
                    col.max + col.x_padding
                }
            })
            .collect()
    }

    /// Calculates total horizontal border width.
    pub fn total_horizontal_border(&self) -> usize {
        if self.border_column && !self.columns.is_empty() {
            self.columns.len() + 1 // One border between each column, plus edges
        } else {
            0
        }
    }

    /// Main method to get optimized column widths and row heights.
    pub fn optimized_widths(&mut self) -> (Vec<usize>, Vec<usize>) {
        if self.max_total() <= self.table_width as usize {
            self.expand_table_width()
        } else {
            self.shrink_table_width()
        }
    }

    /// Expands table width to fit the target by growing columns evenly.
    fn expand_table_width(&mut self) -> (Vec<usize>, Vec<usize>) {
        let mut col_widths = self.max_column_widths();

        // Iteratively expand the shortest columns until we reach target width
        loop {
            let total_width = col_widths.iter().sum::<usize>() + self.total_horizontal_border();
            if total_width >= self.table_width as usize {
                break;
            }

            // Find the shortest non-fixed column
            let mut shortest_idx = 0;
            let mut shortest_width = usize::MAX;

            for (j, &width) in col_widths.iter().enumerate() {
                if self.columns[j].fixed_width > 0 {
                    continue; // Skip fixed-width columns
                }
                if width < shortest_width {
                    shortest_width = width;
                    shortest_idx = j;
                }
            }

            // Expand the shortest column by 1
            col_widths[shortest_idx] += 1;
        }

        let row_heights = self.expand_row_heights(&col_widths);
        (col_widths, row_heights)
    }

    /// Shrinks table width using intelligent median-based algorithm.
    fn shrink_table_width(&mut self) -> (Vec<usize>, Vec<usize>) {
        let mut col_widths = self.max_column_widths();

        // Phase 1: Shrink very big columns (>= tableWidth/2)
        self.shrink_biggest_columns(&mut col_widths, true);

        // Phase 2: Shrink to median (intelligent shrinking)
        self.shrink_to_median(&mut col_widths);

        // Phase 3: Shrink any remaining big columns
        self.shrink_biggest_columns(&mut col_widths, false);

        let row_heights = self.expand_row_heights(&col_widths);
        (col_widths, row_heights)
    }

    /// Shrinks the biggest columns. If very_big_only is true, only shrink columns >= tableWidth/2.
    fn shrink_biggest_columns(&mut self, col_widths: &mut [usize], very_big_only: bool) {
        loop {
            let total_width = col_widths.iter().sum::<usize>() + self.total_horizontal_border();
            if total_width <= self.table_width as usize {
                break;
            }

            let mut biggest_idx = None;
            let mut biggest_width = 0;

            for (j, &width) in col_widths.iter().enumerate() {
                if self.columns[j].fixed_width > 0 {
                    continue; // Skip fixed-width columns
                }

                if very_big_only && width < (self.table_width as usize / 2) {
                    continue; // Only consider very big columns in phase 1
                }

                if width > biggest_width {
                    biggest_width = width;
                    biggest_idx = Some(j);
                }
            }

            if let Some(idx) = biggest_idx {
                if col_widths[idx] > 0 {
                    col_widths[idx] -= 1;
                } else {
                    break; // Can't shrink further
                }
            } else {
                break; // No suitable columns to shrink
            }
        }
    }

    /// Shrinks columns based on their difference from median content width.
    fn shrink_to_median(&mut self, col_widths: &mut [usize]) {
        loop {
            let total_width = col_widths.iter().sum::<usize>() + self.total_horizontal_border();
            if total_width <= self.table_width as usize {
                break;
            }

            let mut target_idx = None;
            let mut max_diff = 0;

            for (j, &width) in col_widths.iter().enumerate() {
                if self.columns[j].fixed_width > 0 {
                    continue; // Skip fixed-width columns
                }

                let median_width = self.columns[j].median + self.columns[j].x_padding;
                if width > median_width {
                    let diff = width - median_width;
                    if diff > max_diff {
                        max_diff = diff;
                        target_idx = Some(j);
                    }
                }
            }

            if let Some(idx) = target_idx {
                if col_widths[idx] > 0 {
                    col_widths[idx] -= 1;
                } else {
                    break;
                }
            } else {
                // No columns are wider than their median, fall back to shrinking biggest
                let mut biggest_idx = None;
                let mut biggest_width = 0;

                for (j, &width) in col_widths.iter().enumerate() {
                    if self.columns[j].fixed_width > 0 {
                        continue;
                    }
                    if width > biggest_width {
                        biggest_width = width;
                        biggest_idx = Some(j);
                    }
                }

                if let Some(idx) = biggest_idx {
                    if col_widths[idx] > 0 {
                        col_widths[idx] -= 1;
                    } else {
                        break;
                    }
                } else {
                    break;
                }
            }
        }
    }

    /// Calculates row heights considering text wrapping.
    fn expand_row_heights(&self, col_widths: &[usize]) -> Vec<usize> {
        let mut row_heights = self.default_row_heights();
        let has_headers = !self.headers.is_empty();

        for (i, row) in self.all_rows.iter().enumerate() {
            for (j, cell) in row.iter().enumerate() {
                // Headers never wrap (always height 1)
                if has_headers && i == 0 {
                    continue;
                }

                if j >= col_widths.len() {
                    continue;
                }

                // Calculate content width minus padding
                let content_width = col_widths[j].saturating_sub(self.x_padding_for_col(j));
                let cell_height =
                    self.detect_content_height(cell, content_width) + self.y_padding_for_cell(i, j);

                row_heights[i] = row_heights[i].max(cell_height);
            }
        }

        row_heights
    }

    /// Detects the height of content considering text wrapping.
    fn detect_content_height(&self, content: &str, width: usize) -> usize {
        if width == 0 {
            return 1;
        }

        let content = content.replace("\r\n", "\n");
        let mut height = 0;

        for line in content.lines() {
            if line.is_empty() {
                height += 1;
            } else {
                height += self.calculate_wrapped_line_height(line, width);
            }
        }

        height.max(1)
    }

    /// Calculates how many lines a single line of text will take when wrapped.
    fn calculate_wrapped_line_height(&self, line: &str, width: usize) -> usize {
        let line_width = display_width(line);
        if line_width <= width {
            return 1;
        }

        // Word-based wrapping calculation
        let words: Vec<&str> = line.split_whitespace().collect();
        if words.is_empty() {
            return 1;
        }

        let mut lines = 1;
        let mut current_width = 0;

        for word in words {
            let word_width = display_width(word);

            // If this word alone exceeds width, it will need its own line(s)
            if word_width > width {
                if current_width > 0 {
                    lines += 1; // Wrap before this word
                }
                // Add lines for this long word (character wrapping)
                lines += word_width.div_ceil(width);
                current_width = word_width % width;
            } else {
                // Check if adding this word would exceed width
                let needed_width = if current_width > 0 {
                    current_width + 1 + word_width // +1 for space
                } else {
                    word_width
                };

                if needed_width > width {
                    lines += 1; // Start new line
                    current_width = word_width;
                } else {
                    current_width = needed_width;
                }
            }
        }

        lines
    }

    /// Returns horizontal padding for a column.
    fn x_padding_for_col(&self, col: usize) -> usize {
        if col < self.columns.len() {
            self.columns[col].x_padding
        } else {
            0
        }
    }

    /// Returns vertical padding for a specific cell.
    fn y_padding_for_cell(&self, row: usize, col: usize) -> usize {
        if row < self.y_paddings.len() && col < self.y_paddings[row].len() {
            self.y_paddings[row][col]
        } else {
            0
        }
    }
}

/// Calculates the median of a slice of numbers.
fn calculate_median(numbers: &[usize]) -> usize {
    if numbers.is_empty() {
        return 0;
    }

    let mut sorted = numbers.to_vec();
    sorted.sort_unstable();

    let len = sorted.len();
    if len % 2 == 0 {
        // Even length: average of two middle values
        let h = len / 2;
        (sorted[h - 1] + sorted[h]) / 2
    } else {
        // Odd length: middle value
        sorted[len / 2]
    }
}

/// Gets the display width of a string, accounting for Unicode width and ANSI sequences.
/// This is equivalent to Go's lipgloss.Width().
fn display_width(s: &str) -> usize {
    // Use lipgloss's ANSI-aware width function
    lipgloss::width(s)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_resizer_new() {
        let headers = vec!["Name".to_string(), "Age".to_string()];
        let rows = vec![
            vec!["Alice".to_string(), "30".to_string()],
            vec!["Bob".to_string(), "25".to_string()],
        ];

        let resizer = Resizer::new(80, 0, headers, rows);
        assert_eq!(resizer.columns.len(), 2);
        assert_eq!(resizer.all_rows.len(), 3); // headers + 2 data rows
    }

    #[test]
    fn test_calculate_median() {
        assert_eq!(calculate_median(&[]), 0);
        assert_eq!(calculate_median(&[5]), 5);
        assert_eq!(calculate_median(&[1, 3, 5]), 3);
        assert_eq!(calculate_median(&[1, 2, 3, 4]), 2); // (2 + 3) / 2 = 2
        assert_eq!(calculate_median(&[4, 1, 3, 2]), 2); // Sorts to [1,2,3,4]
    }

    #[test]
    fn test_display_width() {
        assert_eq!(display_width("hello"), 5);
        assert_eq!(display_width(""), 0);
        assert_eq!(display_width("测试"), 4); // CJK characters are width 2
    }

    #[test]
    fn test_detect_content_height() {
        let resizer = Resizer::new(80, 0, vec![], vec![]);

        assert_eq!(resizer.detect_content_height("hello", 10), 1);
        assert_eq!(resizer.detect_content_height("hello\nworld", 10), 2);
        assert_eq!(resizer.detect_content_height("", 10), 1);

        // Test wrapping
        assert_eq!(resizer.detect_content_height("hello world", 5), 2); // "hello" + "world"
    }

    #[test]
    fn test_max_column_widths() {
        let headers = vec!["Name".to_string(), "Age".to_string()];
        let rows = vec![
            vec!["Alice".to_string(), "30".to_string()],
            vec!["Bob".to_string(), "25".to_string()],
        ];

        let resizer = Resizer::new(80, 0, headers, rows);
        let widths = resizer.max_column_widths();

        assert_eq!(widths.len(), 2);
        assert_eq!(widths[0], 5); // "Alice" is 5 chars
        assert_eq!(widths[1], 3); // "Age" is 3 chars (header is longest)
    }
}
