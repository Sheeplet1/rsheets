use std::collections::HashMap;

use rsheet_lib::command_runner::CellValue;

use super::cell::Cell;

pub struct Spreadsheet {
    cells: HashMap<String, Cell>,
}

impl Spreadsheet {
    pub fn new() -> Self {
        Self {
            cells: HashMap::new(),
        }
    }

    pub fn set(&mut self, key: String, value: CellValue) {
        self.cells.insert(key, Cell::new(value));
    }

    pub fn get(&mut self, key: String) -> Option<&Cell> {
        match self.cells.get(&key) {
            Some(cell) => Some(cell),
            None => None,
        }
    }
}
