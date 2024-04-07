use dashmap::DashMap;
use once_cell::sync::Lazy;
use regex::Regex;

use rsheet_lib::command_runner::CellValue;

/// Lazy static pattern for cell names. Using Lazy to avoid multiple
/// regex compilations, which could end up expensive.
static CELL_PATTERN: Lazy<Regex> = Lazy::new(|| Regex::new(r"^[A-Z]+[0-9]+$").unwrap());

pub struct Spreadsheet {
    // Dashmap is used for concurrent access to cells.
    cells: DashMap<String, CellValue>,
}

impl Spreadsheet {
    pub fn new() -> Self {
        Self {
            cells: DashMap::new(),
        }
    }

    pub fn set(&mut self, key: String, value: CellValue) {
        self.cells.insert(key, value);
    }

    pub fn get(&mut self, key: String) -> CellValue {
        match self.cells.get(&key) {
            Some(cell) => cell.clone(),
            None => CellValue::None,
        }
    }
}

impl Default for Spreadsheet {
    fn default() -> Self {
        Self::new()
    }
}

pub fn is_valid_cell(cell_name: &str) -> bool {
    CELL_PATTERN.is_match(cell_name)
}
