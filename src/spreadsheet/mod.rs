use std::sync::Arc;

use dashmap::DashMap;

use rsheet_lib::command_runner::CellValue;

/// Lazy static pattern for cell names. Using Lazy to avoid multiple
/// regex compilations, which could end up expensive.

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

    pub fn set(&self, key: String, value: CellValue) {
        self.cells.insert(key, value);
    }

    pub fn get(&self, key: String) -> CellValue {
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

pub fn new_shared_spreadsheet() -> Arc<Spreadsheet> {
    Arc::new(Spreadsheet::new())
}
