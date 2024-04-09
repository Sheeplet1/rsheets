use once_cell::sync::Lazy;
use regex::Regex;

pub fn is_valid_cell(cell_name: &str) -> bool {
    // NOTE: I took this Regex from the rsheets codebase/library. Not sure how
    // to attribute the source.
    static CELL_PATTERN: Lazy<Regex> =
        Lazy::new(|| Regex::new(r"^[A-Z]+[0-9]+(_[A-Z]+[0-9]+)?$").unwrap());
    CELL_PATTERN.is_match(cell_name)
}
