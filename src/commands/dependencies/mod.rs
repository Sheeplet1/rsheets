use std::sync::Arc;

use rsheet_lib::{
    cells::{column_name_to_number, column_number_to_name},
    command_runner::{CellValue, CommandRunner},
    replies::Reply,
};

use crate::{commands::variables::variable_map_for_runner, spreadsheet::Spreadsheet};

/// Add the current cell as a dependency to all cells in the given range. Acts
/// as a wrapper around `spreadsheet`'s `add_dependency` method for multiple
/// cells.
///
/// # Example
///
/// ```rust
/// use std::sync::Arc;
/// use rsheet_lib::spreadsheet::Spreadsheet;
/// use rsheet_server::commands::dependencies::add_as_dependent;
///
/// let spreadsheet = spreadsheet::new_shared_spreadsheet();
/// add_dependencies(&spreadsheet, "A1", "A", "2", "A", "4");
///
/// let dependencies = spreadsheet.get_dependencies("A1").unwrap();
/// assert_eq!(dependencies, vec!["A2".to_string(), "A3".to_string(), "A4".to_string()]);
/// ```
pub fn add_dependencies(
    spreadsheet: &Arc<Spreadsheet>,
    cell: &str,
    start_col: &str,
    start_row: &str,
    end_col: &str,
    end_row: &str,
) {
    let start_row: u32 = start_row.parse().unwrap();
    let end_row: u32 = end_row.parse().unwrap();

    let start_col = column_name_to_number(start_col);
    let end_col = column_name_to_number(end_col);

    for row in start_row..=end_row {
        for col in start_col..=end_col {
            let col = column_number_to_name(col);
            let parent = format!("{}{}", col, row).to_string();
            spreadsheet.add_dependency(&parent, cell);
        }
    }
}

/// Removing all dependencies associated with the old expression. There are
/// preliminary checks to ensure that the old expression is not the same as the
/// new expression.
///
/// # Example
///
/// ```rust
/// use std::sync::Arc;
/// use rsheet_lib::spreadsheet::Spreadsheet;
/// use rsheet_server::commands::dependencies::remove_all_dependencies;
///
/// let spreadsheet = spreadsheet::new_shared_spreadsheet();
/// spreadsheet.set_cell("A1", CellValue::Int(10), Some("A2 + 10".to_string()));
/// assert_eq!(spreadsheet.get_dependencies("A2"), Some(vec!["A1".to_string()]));
///
/// remove_all_dependencies(&spreadsheet, "A1", &"A2 + 10".to_string());
/// assert_eq!(spreadsheet.get_dependencies("A2"), None);
/// ```
pub fn remove_all_dependencies(spreadsheet: &Arc<Spreadsheet>, cell: &str, new_expr: &String) {
    let old_expr = spreadsheet.get_cell_expr(cell);

    if let Some(old_expr) = old_expr {
        if old_expr != *new_expr {
            let old_vars = CommandRunner::new(&old_expr).find_variables();
            old_vars.into_iter().for_each(|var| {
                // This isn't actually comparing the old variables and the
                // new variables, its just removing all variables from the old
                // expression.
                spreadsheet.remove_dependency(&var, cell);
            });
        }
    }
}

/// Updates the dependencies of the cell.
///
/// # Example
///
/// ```rust
/// use std::sync::Arc;
/// use rsheet_lib::spreadsheet::Spreadsheet;
/// use rsheet_server::commands::dependencies::update_dependency;
/// use rsheet_lib::cells::CellValue;
///
/// let spreadsheet = spreadsheet::new_shared_spreadsheet();
/// spreadsheet.set_cell("A1", CellValue::Int(10), None);
/// spreadsheet.set_cell("A2", CellValue::Int(20), Some("A1 + 10".to_string()));
/// spreadsheet.set_cell("A3", CellValue::Int(30), Some("A2 + 10".to_string()));
///
/// update_dependency(&spreadsheet, "A1", 0);
///
/// assert_eq!(spreadsheet.get_cell_val("A1"), CellValue::Int(0));
/// assert_eq!(spreadsheet.get_cell_val("A2"), CellValue::Int(10));
/// assert_eq!(spreadsheet.get_cell_val("A3"), CellValue::Int(20));
/// ```
pub fn update_dependency(
    spreadsheet: &Arc<Spreadsheet>,
    parent: &str,
    path: &mut Vec<String>,
    timestamp: u64,
) -> Result<(), Reply> {
    if path.contains(&parent.to_string()) {
        // If the parent is in the path, then we have found a circular
        // dependency and we return early to avoid infinite recursion.
        handle_circular_dependency(spreadsheet, parent, timestamp);
        return Ok(());
    }

    // Add the parent to the path to keep track of the cells that has been
    // visited in this call.
    path.push(parent.to_string());

    // Get the dependencies of the parent cell. If there are no dependencies,
    // then it defaults to an empty vec.
    let dependencies = spreadsheet.get_dependencies(parent).unwrap_or_default();

    // Update these dependencies with the new values.
    for dep in dependencies {
        let expr = match spreadsheet.get_cell_expr(&dep) {
            Some(expr) => expr,
            None => {
                // If there is no expression, then skip the cell. Realistically,
                // this shouldn't happen since if there is a dependency, there
                // should be an expression.
                continue;
            }
        };
        let runner = CommandRunner::new(&expr);
        let vars = runner.find_variables();
        let var_map = variable_map_for_runner(spreadsheet, &vars);
        let cell_val = runner.run(&var_map);

        // If there aren't any variables, then its a scalar value and we set
        // the cell value directly. Otherwise, we need to store the expression.
        match vars.is_empty() {
            true => spreadsheet.set_cell(&dep, cell_val, None, timestamp),
            false => spreadsheet.set_cell(&dep, cell_val, Some(expr), timestamp),
        }

        // Each dependency could have its own set of dependencies, so we need
        // to update those as well.
        let mut new_path = path.clone();
        update_dependency(spreadsheet, &dep, &mut new_path, timestamp)?;
    }

    path.pop();

    Ok(())
}

/// Handles updating dependencies with the circular dependency error.
///
/// # Example
///
/// ```rust
/// use std::sync::Arc;
/// use rsheet_lib::spreadsheet::Spreadsheet;
/// use rsheet_server::commands::dependencies::handle_circular_dependency;
/// use rsheet_lib::cells::CellValue;
///
/// let spreadsheet = spreadsheet::new_shared_spreadsheet();
/// handle_circular_dependency(&spreadsheet, "A1", 0);
/// assert_eq!(spreadsheet.get_cell_val("A1"), CellValue::Error("Cell A1 is self-referential".to_string()));
/// ```
fn handle_circular_dependency(spreadsheet: &Arc<Spreadsheet>, parent: &str, timestamp: u64) {
    spreadsheet.set_cell(
        parent,
        CellValue::Error(format!("Cell {} is self-referential", parent)),
        Some("Circular Dependency".to_string()),
        timestamp,
    );

    // Each dependency could have its own dependencies, so we need to also
    // update them to contain the error message. `unwrap_or_default` is
    // used here to handle the case where the parent cell has no
    // dependencies.
    let dependencies = spreadsheet.get_dependencies(parent).unwrap_or_default();
    for dep in dependencies {
        spreadsheet.set_cell(
            &dep,
            CellValue::Error(format!("Cell {} is involved in a circular dependency", dep)),
            Some("Circular Dependency".to_string()),
            timestamp,
        );
    }
}
