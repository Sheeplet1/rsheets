use std::sync::Arc;

use dashmap::DashMap;

use rsheet_lib::command_runner::CellValue;

/// Lazy static pattern for cell names. Using Lazy to avoid multiple
/// regex compilations, which could end up expensive.

#[derive(Debug)]
pub struct Spreadsheet {
    // Dashmap is used for concurrent access to cells.
    cells: DashMap<String, (CellValue, Option<String>)>,
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

    pub fn set_cell(&self, key: &str, value: CellValue, expr: Option<String>) {
        self.cells.insert(key.to_string(), (value, expr));
    }

    pub fn get_cell(&self, key: &str) -> (CellValue, Option<String>) {
        match self.cells.get(key) {
            Some(cell) => cell.value().clone(),
            None => (CellValue::None, None),
        }
    }

    /// Adds a dependency to the key's dependency list. I.e, the value is
    /// dependent on the key, so if the key changes, we need to update the value.
    pub fn add_dependency(self, key: &str, dependency: &str) {
        let dependency = dependency.to_string();
        if let Some(mut deps) = self.dependency_list.get_mut(key) {
            deps.push(dependency);
        } else {
            self.dependency_list
                .insert(key.to_string(), vec![dependency]);
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
