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
    pub parent_of: DashMap<String, Vec<String>>,
    pub child_of: DashMap<String, Vec<String>>,
}

impl Spreadsheet {
    pub fn new() -> Self {
        Self {
            cells: DashMap::new(),
            parent_of: DashMap::new(),
            child_of: DashMap::new(),
        }
    }

    pub fn set_cell(&self, key: &str, value: CellValue, expr: Option<String>) {
        self.cells.insert(key.to_string(), (value, expr));
    }

    pub fn get_cell_val(&self, key: &str) -> CellValue {
        match self.cells.get(key) {
            Some(cell) => cell.0.clone(),
            None => CellValue::None,
        }
    }

    pub fn get_cell_expr(&self, key: &str) -> Option<String> {
        match self.cells.get(key) {
            Some(cell) => cell.value().1.clone(),
            None => None,
        }
    }

    pub fn get_children(&self, parent: &str) -> Option<Vec<String>> {
        self.parent_of.get(parent).map(|deps| deps.value().clone())
    }

    /// Adds a dependency to the key's dependency list. I.e, the value is
    /// dependent on the key, so if the key changes, we need to update the value.
    pub fn add_dependency(&self, parent: &str, child: &str) {
        if !self.parent_of.contains_key(parent) {
            self.parent_of
                .insert(parent.to_string(), vec![child.to_string()]);
        } else {
            self.parent_of
                .get_mut(parent)
                .unwrap()
                .push(child.to_string());
        }

        if !self.child_of.contains_key(child) {
            self.child_of
                .insert(child.to_string(), vec![parent.to_string()]);
        } else {
            self.child_of
                .get_mut(child)
                .unwrap()
                .push(parent.to_string());
        }
    }

    pub fn remove_dependency(&self, parent: &str, child: &str) {
        if let Some(mut parent_deps) = self.parent_of.get_mut(parent) {
            parent_deps.retain(|dep| dep != child);
        }

        if let Some(mut child_deps) = self.child_of.get_mut(child) {
            child_deps.retain(|dep| dep != parent);
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
