use once_cell::sync::Lazy;
use regex::Regex;

/// Checks if a given cell is valid.
///
/// # Example
///
/// ```rust
/// use rsheets::utils::is_valid_cell;
///
/// assert_eq!(is_valid_cell("A1"), true);
/// assert_eq!(is_valid_cell("A1_B2"), true);
/// assert_eq!(is_valid_cell("A1_B2_C3"), false);
/// ```
pub fn is_valid_cell(cell_name: &str) -> bool {
    // NOTE: I took this Regex from the rsheets codebase/library. Not sure how
    // to attribute the source.
    static CELL_PATTERN: Lazy<Regex> =
        Lazy::new(|| Regex::new(r"^[A-Z]+[0-9]+(_[A-Z]+[0-9]+)?$").unwrap());
    CELL_PATTERN.is_match(cell_name)
}
