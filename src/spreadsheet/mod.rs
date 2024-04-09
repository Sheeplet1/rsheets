use std::sync::Arc;

use dashmap::DashMap;

use rsheet_lib::command_runner::CellValue;

/// Lazy static pattern for cell names. Using Lazy to avoid multiple
/// regex compilations, which could end up expensive.

#[derive(Debug)]
pub struct Spreadsheet {
    // Dashmap is used for concurrent access to cells.
    cells: DashMap<String, CellValue>,
    /// Dependency list is used to keep track of which cells are dependent on
    /// which other cells. The values are dependent on the key cell.
    dependency_list: DashMap<String, Vec<String>>,
}

impl Spreadsheet {
    pub fn new() -> Self {
        Self {
            cells: DashMap::new(),
            dependency_list: DashMap::new(),
        }
    }

    pub fn set_cell(&self, key: String, value: CellValue) {
        self.cells.insert(key, value);
    }

    pub fn get_cell(&self, key: String) -> CellValue {
        match self.cells.get(&key) {
            Some(cell) => cell.clone(),
            None => CellValue::None,
        }
    }

    pub fn add_dependency(&self, key: String, dependency: String) {
        if !self.dependency_list.contains_key(&key) {
            self.dependency_list.insert(key.clone(), vec![dependency]);
        } else {
            self.dependency_list.get_mut(&key).unwrap().push(dependency);
        }
    }

    pub fn clear_dependencies(&self, key: String) {
        self.dependency_list.remove(&key);
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
