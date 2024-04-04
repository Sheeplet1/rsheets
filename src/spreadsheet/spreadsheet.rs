use std::collections::HashMap;

use rsheet_lib::command_runner::CellValue;

pub struct Spreadsheet {
    cells: HashMap<String, CellValue>,
}

impl Spreadsheet {
    pub fn new() -> Self {
        Self {
            cells: HashMap::new(),
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
