use std::sync::Arc;

use dashmap::DashMap;

use rsheet_lib::command_runner::CellValue;

#[derive(Debug)]
struct Cell {
    value: CellValue,
    expression: Option<String>,
    timestamp: usize,
}

impl Default for Cell {
    fn default() -> Self {
        Self {
            value: CellValue::None,
            expression: None,
            timestamp: 0,
        }
    }
}

#[derive(Debug)]
pub struct Spreadsheet {
    /// Cells is the main data structure for the spreadsheet. It uses a
    /// DashMap for concurrent access and modification. The key is the
    /// cell name (e.g, A1, B1 ...) and the value is a tuple with the
    /// cell's value and the expression.
    cells: DashMap<String, Cell>,

    /// dependencies: a map where the key cell has a vector of cells that
    /// are dependent on it. If the key cell's value changes, then we need
    /// to update all the cells' values in the vector.
    ///
    /// Example: dependencies[A1] = [B1, C1] means that B1 and C1 are
    /// dependent on A1. If A1 changes, B1 and C1 need to be updated.
    ///
    /// Another way of expressing this relationship is the parent-child model.
    /// A1 is the parent of B1 and C1. If the parent changes, the children
    /// will also change.
    pub dependencies: DashMap<String, Vec<String>>,
}

impl Spreadsheet {
    pub fn new() -> Self {
        Self {
            cells: DashMap::new(),
            dependencies: DashMap::new(),
        }
    }

    /// Set the cell's value, expression and timestamp. If the incoming
    /// timestamp is not more recent, then we don't update the cell.
    pub fn set_cell(
        &self,
        key: &str,
        value: CellValue,
        expr: Option<String>,
        inc_timestamp: usize,
    ) {
        // Get the cell entry, otherwise default to the default Cell struct.
        let mut cell_entry = self.cells.entry(key.to_string()).or_default();

        // If the incoming timestamp is more recent than the cell's timestamp,
        // then we update the cell.
        let curr_timestamp = cell_entry.timestamp;
        if inc_timestamp >= curr_timestamp {
            cell_entry.value = value;
            cell_entry.expression = expr;
            cell_entry.timestamp = inc_timestamp;
        }
    }

    /// Gets the cell's value from the `cells` map, as the value is a tuple.
    ///
    /// # Example
    ///
    /// ```
    /// let spreadsheet = Spreadsheet::new();
    /// spreadsheet.set_cell("A1", CellValue::Int(10.0), None);
    /// assert_eq!(spreadsheet.get_cell_val("A1"), CellValue::Int(10.0));
    /// ```
    pub fn get_cell_val(&self, key: &str) -> CellValue {
        match self.cells.get(key) {
            Some(cell) => cell.value.clone(),
            None => CellValue::None,
        }
    }

    /// Gets the cell's expression from the `cells` map, as the value is a tuple.
    ///
    /// # Example
    ///
    /// ```
    /// let spreadsheet = Spreadsheet::new();
    /// spreadsheet.set_cell("A1", CellValue::Int(10.0), Some("A2 + 10".to_string()));
    /// assert_eq!(spreadsheet.get_cell_expr("A1"), Some("A2 + 10".to_string()));
    /// ```
    pub fn get_cell_expr(&self, key: &str) -> Option<String> {
        match self.cells.get(key) {
            Some(cell) => cell.expression.clone(),
            None => None,
        }
    }

    // TODO: We might not even need this function if we have checks beforehand.
    pub fn get_cell_timestamp(&self, key: &str) -> usize {
        match self.cells.get(key) {
            Some(cell) => cell.timestamp,
            None => 0,
        }
    }

    /// Get the parent's dependencies.
    ///
    /// # Example
    ///
    /// ```
    /// let spreadsheet = Spreadsheet::new();
    /// spreadsheet.add_dependency("A1", "B1");
    /// assert_eq!(spreadsheet.get_dependencies("A1"), Some(vec!["B1".to_string()]));
    /// ```
    pub fn get_dependencies(&self, parent: &str) -> Option<Vec<String>> {
        self.dependencies
            .get(parent)
            .map(|deps| deps.value().clone())
    }

    /// Adds a dependency to the key's dependency list. I.e, the value is
    /// dependent on the key, so if the key changes, we need to update the value.
    ///
    /// # Example
    ///
    /// ```
    /// let spreadsheet = Spreadsheet::new();
    /// spreadsheet.add_dependency("A1", "B1");
    /// assert_eq!(spreadsheet.get_dependencies("A1"), Some(vec!["B1".to_string()]));
    /// ```
    pub fn add_dependency(&self, parent: &str, child: &str) {
        if !self.dependencies.contains_key(parent) {
            self.dependencies
                .insert(parent.to_string(), vec![child.to_string()]);
        } else {
            self.dependencies
                .get_mut(parent)
                .unwrap()
                .push(child.to_string());
        }
    }

    /// Removes a dependency from the parent's list. I.e, the value is no longer
    /// influenced by the parent.
    ///
    /// # Example
    ///
    /// ```
    /// let spreadsheet = Spreadsheet::new();
    ///
    /// spreadsheet.add_dependency("A1", "B1");
    /// assert_eq!(spreadsheet.get_dependencies("A1"), Some(vec!["B1".to_string()]));
    ///
    /// spreadsheet.remove_dependency("A1", "B1");
    /// assert_eq!(spreadsheet.get_dependencies("A1"), None);
    /// ```
    pub fn remove_dependency(&self, parent: &str, child: &str) {
        if let Some(mut parent_deps) = self.dependencies.get_mut(parent) {
            parent_deps.retain(|dep| dep != child);
        }
    }
}

impl Default for Spreadsheet {
    fn default() -> Self {
        Self::new()
    }
}

/// Creates an `Arc` instance of `Spreadsheet` for concurrency.
pub fn new_shared_spreadsheet() -> Arc<Spreadsheet> {
    Arc::new(Spreadsheet::new())
}
