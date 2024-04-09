use once_cell::sync::Lazy;
use regex::Regex;

pub fn is_valid_cell(cell_name: &str) -> bool {
    static CELL_PATTERN: Lazy<Regex> = Lazy::new(|| Regex::new(r"^[A-Z]+[0-9]+$").unwrap());
    CELL_PATTERN.is_match(cell_name)
}
