mod cell;
mod spreadsheet;

use regex::Regex;

pub fn new() -> spreadsheet::Spreadsheet {
    spreadsheet::Spreadsheet::new()
}

pub fn is_valid_cell(cell_name: &str) -> bool {
    // TODO:
    let regex = Regex::new(r"^[A-Z]+[0-9]+$").unwrap();
    regex.is_match(cell_name)
}
