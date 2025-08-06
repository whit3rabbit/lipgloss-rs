//! Memory safety and security utilities for the lipgloss library.
//!
//! This module provides protection against memory exhaustion attacks and ensures
//! safe allocation patterns when processing untrusted input.

/// Maximum allowed dimension value for width, height, padding, margin, and tab width.
/// This prevents excessive memory allocation from malicious or erroneous input.
///
/// Set to 10,000 which allows for reasonable terminal layouts while preventing
/// multi-gigabyte allocations from dimension values.
pub const MAX_DIMENSION: i32 = 10_000;

/// Maximum allowed string repetition count to prevent memory exhaustion.
/// This is used by safe_repeat and other allocation functions.
pub const MAX_REPEAT_COUNT: usize = MAX_DIMENSION as usize;

/// Maximum total memory budget for a single render operation (in bytes).
/// This provides an additional safety net against cumulative allocations.
pub const MAX_RENDER_MEMORY_BUDGET: usize = 50_000_000; // 50MB

/// Maximum number of bytes to scan when parsing a single ANSI escape sequence.
/// This prevents unbounded scanning in the presence of malformed or unterminated
/// sequences (e.g., an ESC without a terminating byte), mitigating potential DoS.
pub const MAX_ANSI_SEQ_LEN: usize = 64;

/// Validates that a dimension value is within safe bounds.
///
/// # Arguments
///
/// * `value` - The dimension value to validate
/// * `name` - Name of the dimension for error messages
///
/// # Returns
///
/// The validated value, clamped to safe bounds.
///
/// # Examples
///
/// ```rust
/// use lipgloss::security::validate_dimension;
///
/// assert_eq!(validate_dimension(100, "width"), 100);
/// assert_eq!(validate_dimension(20000, "width"), 10000); // Clamped to MAX_DIMENSION
/// assert_eq!(validate_dimension(-5, "padding"), 0); // Negative values become 0
/// ```
pub fn validate_dimension(value: i32, _name: &str) -> i32 {
    value.clamp(0, MAX_DIMENSION)
}

/// Validates tab width allowing the special sentinel -1 (keep tabs as-is).
///
/// Values are clamped as follows:
/// - `-1` or any negative value -> `-1` (keep tabs)
/// - `0` -> remove tabs
/// - `1..=MAX_DIMENSION` -> unchanged (limit enforced)
pub fn validate_tab_width(value: i32) -> i32 {
    if value < 0 {
        -1
    } else {
        value.min(MAX_DIMENSION)
    }
}

/// Safely repeats a character with bounds checking to prevent memory exhaustion.
///
/// This function provides a safe alternative to `String::repeat()` that prevents
/// excessive memory allocation from large repeat counts.
///
/// # Arguments
///
/// * `ch` - The character to repeat
/// * `count` - The number of repetitions (will be clamped to MAX_REPEAT_COUNT)
///
/// # Returns
///
/// A string containing the repeated character, or empty string if count is 0.
///
/// # Examples
///
/// ```rust
/// use lipgloss::security::safe_repeat;
///
/// assert_eq!(safe_repeat(' ', 5), "     ");
/// assert_eq!(safe_repeat('=', 0), "");
///
/// // Large values are safely clamped
/// let result = safe_repeat('-', 50000);
/// assert_eq!(result.len(), 10000); // Clamped to MAX_REPEAT_COUNT
/// ```
pub fn safe_repeat(ch: char, count: usize) -> String {
    let safe_count = count.min(MAX_REPEAT_COUNT);
    ch.to_string().repeat(safe_count)
}

/// Safely repeats a string with bounds checking to prevent memory exhaustion.
///
/// This function provides a safe alternative to `str.repeat()` that prevents
/// excessive memory allocation from large repeat counts.
///
/// # Arguments
///
/// * `s` - The string to repeat
/// * `count` - The number of repetitions (will be clamped based on string length)
///
/// # Returns
///
/// A string containing the repeated content, with total length capped.
///
/// # Examples
///
/// ```rust
/// use lipgloss::security::safe_str_repeat;
///
/// assert_eq!(safe_str_repeat("ab", 3), "ababab");
/// assert_eq!(safe_str_repeat("test", 0), "");
///
/// // Large values are safely clamped based on total output size
/// let result = safe_str_repeat("x", 50000);
/// assert!(result.len() <= 10000);
/// ```
pub fn safe_str_repeat(s: &str, count: usize) -> String {
    if s.is_empty() || count == 0 {
        return String::new();
    }

    // Calculate safe repeat count based on string length to prevent overflow
    let max_safe_count = MAX_REPEAT_COUNT / s.len().max(1);
    let safe_count = count.min(max_safe_count);

    s.repeat(safe_count)
}

/// Checks if a memory allocation of the given size would exceed safe limits.
///
/// # Arguments
///
/// * `size` - The proposed allocation size in bytes
///
/// # Returns
///
/// `true` if the allocation is safe, `false` if it would exceed limits.
pub fn is_safe_allocation(size: usize) -> bool {
    size <= MAX_RENDER_MEMORY_BUDGET
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_dimension() {
        assert_eq!(validate_dimension(100, "width"), 100);
        assert_eq!(validate_dimension(MAX_DIMENSION, "width"), MAX_DIMENSION);
        assert_eq!(
            validate_dimension(MAX_DIMENSION + 1, "width"),
            MAX_DIMENSION
        );
        assert_eq!(validate_dimension(-5, "padding"), 0);
        assert_eq!(validate_dimension(0, "margin"), 0);
    }

    #[test]
    fn test_safe_repeat() {
        assert_eq!(safe_repeat(' ', 5), "     ");
        assert_eq!(safe_repeat('=', 0), "");
        assert_eq!(safe_repeat('x', 1), "x");

        // Test clamping
        let result = safe_repeat('-', MAX_REPEAT_COUNT + 1);
        assert_eq!(result.len(), MAX_REPEAT_COUNT);
    }

    #[test]
    fn test_safe_str_repeat() {
        assert_eq!(safe_str_repeat("ab", 3), "ababab");
        assert_eq!(safe_str_repeat("test", 0), "");
        assert_eq!(safe_str_repeat("", 100), "");

        // Test length-based clamping
        let result = safe_str_repeat("x", MAX_REPEAT_COUNT + 1);
        assert_eq!(result.len(), MAX_REPEAT_COUNT);

        let result = safe_str_repeat("abc", MAX_REPEAT_COUNT);
        assert!(result.len() <= MAX_REPEAT_COUNT);
    }

    #[test]
    fn test_is_safe_allocation() {
        assert!(is_safe_allocation(1000));
        assert!(is_safe_allocation(MAX_RENDER_MEMORY_BUDGET));
        assert!(!is_safe_allocation(MAX_RENDER_MEMORY_BUDGET + 1));
    }
}
