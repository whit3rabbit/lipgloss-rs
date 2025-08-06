/// Data is the interface that wraps the basic methods of a table model.
pub trait Data {
    /// At returns the contents of the cell at the given index.
    fn at(&self, row: usize, cell: usize) -> String;

    /// Rows returns the number of rows in the table.
    fn rows(&self) -> usize;

    /// Columns returns the number of columns in the table.
    fn columns(&self) -> usize;
}

/// StringData is a string-based implementation of the Data interface.
#[derive(Debug, Clone)]
pub struct StringData {
    rows: Vec<Vec<String>>,
    columns: usize,
}

impl StringData {
    /// Creates a new StringData with the given rows.
    pub fn new(rows: Vec<Vec<String>>) -> Self {
        let columns = rows.iter().map(|row| row.len()).max().unwrap_or(0);
        Self { rows, columns }
    }

    /// Creates a new empty StringData.
    pub fn empty() -> Self {
        Self {
            rows: Vec::new(),
            columns: 0,
        }
    }

    /// Appends the given row to the table.
    pub fn append(&mut self, row: Vec<String>) {
        self.columns = self.columns.max(row.len());
        self.rows.push(row);
    }

    /// Item appends the given row to the table (builder pattern).
    pub fn item(mut self, row: Vec<String>) -> Self {
        self.append(row);
        self
    }
}

impl Data for StringData {
    fn at(&self, row: usize, cell: usize) -> String {
        if row >= self.rows.len() || cell >= self.rows[row].len() {
            return String::new();
        }
        self.rows[row][cell].clone()
    }

    fn rows(&self) -> usize {
        self.rows.len()
    }

    fn columns(&self) -> usize {
        self.columns
    }
}

/// Filter applies a filter on some data.
pub struct Filter<D: Data> {
    data: D,
    filter: Option<Box<dyn Fn(usize) -> bool>>,
}

impl<D: Data> Filter<D> {
    /// Creates a new Filter with the given data.
    pub fn new(data: D) -> Self {
        Self { data, filter: None }
    }

    /// Applies the given filter function to the data.
    pub fn filter<F>(mut self, f: F) -> Self
    where
        F: Fn(usize) -> bool + 'static,
    {
        self.filter = Some(Box::new(f));
        self
    }
}

impl<D: Data> Data for Filter<D> {
    fn at(&self, row: usize, cell: usize) -> String {
        if let Some(ref filter) = self.filter {
            let mut j = 0;
            for i in 0..self.data.rows() {
                if filter(i) {
                    if j == row {
                        return self.data.at(i, cell);
                    }
                    j += 1;
                }
            }
            String::new()
        } else {
            self.data.at(row, cell)
        }
    }

    fn rows(&self) -> usize {
        if let Some(ref filter) = self.filter {
            let mut count = 0;
            for i in 0..self.data.rows() {
                if filter(i) {
                    count += 1;
                }
            }
            count
        } else {
            self.data.rows()
        }
    }

    fn columns(&self) -> usize {
        self.data.columns()
    }
}

/// Converts an object that implements the Data interface to a matrix.
pub fn data_to_matrix<D: Data + ?Sized>(data: &D) -> Vec<Vec<String>> {
    let num_rows = data.rows();
    let num_cols = data.columns();
    let mut rows = Vec::with_capacity(num_rows);

    for i in 0..num_rows {
        let mut row = Vec::with_capacity(num_cols);
        for j in 0..num_cols {
            row.push(data.at(i, j));
        }
        rows.push(row);
    }
    rows
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_string_data_basic() {
        let data = StringData::new(vec![
            vec!["A".to_string(), "B".to_string()],
            vec!["C".to_string(), "D".to_string(), "E".to_string()],
        ]);

        assert_eq!(data.rows(), 2);
        assert_eq!(data.columns(), 3); // Max columns
        assert_eq!(data.at(0, 0), "A");
        assert_eq!(data.at(0, 1), "B");
        assert_eq!(data.at(1, 2), "E");
        assert_eq!(data.at(0, 2), ""); // Missing cell
        assert_eq!(data.at(2, 0), ""); // Out of bounds
    }

    #[test]
    fn test_string_data_append() {
        let mut data = StringData::empty();
        assert_eq!(data.rows(), 0);
        assert_eq!(data.columns(), 0);

        data.append(vec!["A".to_string(), "B".to_string()]);
        assert_eq!(data.rows(), 1);
        assert_eq!(data.columns(), 2);

        data.append(vec!["C".to_string()]);
        assert_eq!(data.rows(), 2);
        assert_eq!(data.columns(), 2); // Still 2, not reduced
    }

    #[test]
    fn test_string_data_builder() {
        let data = StringData::empty()
            .item(vec!["Name".to_string(), "Age".to_string()])
            .item(vec!["Alice".to_string(), "30".to_string()])
            .item(vec![
                "Bob".to_string(),
                "25".to_string(),
                "Engineer".to_string(),
            ]);

        assert_eq!(data.rows(), 3);
        assert_eq!(data.columns(), 3);
        assert_eq!(data.at(1, 0), "Alice");
        assert_eq!(data.at(2, 2), "Engineer");
    }

    #[test]
    fn test_filter_basic() {
        let data = StringData::new(vec![
            vec!["A".to_string(), "1".to_string()],
            vec!["B".to_string(), "2".to_string()],
            vec!["C".to_string(), "3".to_string()],
            vec!["D".to_string(), "4".to_string()],
        ]);

        // Filter even rows (0, 2)
        let filtered = Filter::new(data).filter(|row| row % 2 == 0);

        assert_eq!(filtered.rows(), 2);
        assert_eq!(filtered.columns(), 2);
        assert_eq!(filtered.at(0, 0), "A"); // Original row 0
        assert_eq!(filtered.at(1, 0), "C"); // Original row 2
        assert_eq!(filtered.at(0, 1), "1");
        assert_eq!(filtered.at(1, 1), "3");
    }

    #[test]
    fn test_data_to_matrix() {
        let data = StringData::new(vec![
            vec!["A".to_string(), "B".to_string()],
            vec!["C".to_string()],
        ]);

        let matrix = data_to_matrix(&data);
        assert_eq!(matrix.len(), 2);
        assert_eq!(matrix[0], vec!["A", "B"]);
        assert_eq!(matrix[1], vec!["C", ""]); // Padded with empty string
    }
}
