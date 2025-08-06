/// Converts a boolean to an integer: 1 if true, 0 if false.
pub fn btoi(b: bool) -> usize {
    if b {
        1
    } else {
        0
    }
}

/// Returns the sum of all integers in a slice.
pub fn sum(numbers: &[usize]) -> usize {
    numbers.iter().sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_btoi() {
        assert_eq!(btoi(true), 1);
        assert_eq!(btoi(false), 0);
    }

    #[test]
    fn test_sum() {
        assert_eq!(sum(&[]), 0);
        assert_eq!(sum(&[1, 2, 3, 4, 5]), 15);
        assert_eq!(sum(&[10]), 10);
    }
}
